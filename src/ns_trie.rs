use std::{
    collections::{BTreeMap, HashMap},
    fmt,
    fs::write,
    path::Path,
};

use crate::trie::Node;
use log::{debug, info, warn};
use url::Url;

#[derive(Debug, Clone, Copy)]
pub enum NamespaceSource {
    User,
    Community,
    GraphFile,
    Inference,
}

impl fmt::Display for NamespaceSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NamespaceSource::User => write!(f, "user"),
            NamespaceSource::Community => write!(f, "community"),
            NamespaceSource::GraphFile => write!(f, "graph_file"),
            NamespaceSource::Inference => write!(f, "inference"),
        }
    }
}

pub type NamespaceTrie = Node<(String, NamespaceSource)>;
pub type NamespaceMap = BTreeMap<String, (String, String)>;

pub trait SaveTrie {
    fn save(&self, outf: &str);
}

impl SaveTrie for NamespaceTrie {
    fn save(&self, outf: &str) {
        let file_path = Path::new(".").join(outf).join("all-prefixes.json");
        info!("Saving namespaces in {}", file_path.to_string_lossy());

        let mut ns_map = NamespaceMap::new();

        for (ns, node) in self.iter_leaves() {
            let (alias, source) = node.value.as_ref().unwrap().clone();
            ns_map.insert(alias, (ns, source.to_string()));
        }

        write(file_path, serde_json::to_string_pretty(&ns_map).unwrap()).unwrap();
    }
}

pub trait InferredNamespaces {
    fn add_namespaces(
        &mut self,
        inferred: &Vec<(String, usize, NamespaceSource)>,
        allow_subns: bool,
    ) -> Vec<String>;

    fn to_map(&self) -> NamespaceMap;
}

impl InferredNamespaces for NamespaceTrie {
    fn to_map(&self) -> NamespaceMap {
        let mut trie = NamespaceMap::new();
        for (ns, node) in self.iter() {
            if let Some((alias, source)) = node.value.clone() {
                trie.insert(alias, (ns, source.to_string()));
            }
        }
        return trie;
    }
    fn add_namespaces(
        &mut self,
        inferred: &Vec<(String, usize, NamespaceSource)>,
        allow_subns: bool,
    ) -> Vec<String> {
        let mut aliases = self.to_map();

        let mut added = Node::<String>::new();

        for (ns, size, source) in inferred.iter() {
            match Url::parse(ns.as_str()) {
                Err(err) => warn!("Could not parse IRI {ns}: {err}"),
                Ok(url_obj) => {
                    if !url_obj.has_host() {
                        warn!("IRI {ns} does not have host");
                        continue;
                    }
                    if let Some((node, exists_ns)) =
                        self.longest_prefix(url_obj.to_string().as_str(), true)
                    {
                        if node.value.is_none() {
                            println!("{ns} {exists_ns} {:#?} {}", node.value, node.is_terminal);
                        }
                        let (alias, _) = node.value.as_ref().unwrap();

                        if *ns == exists_ns {
                            debug!("Inferred namespace {ns} already in trie with alias {alias}",);
                            continue;
                        }
                        if !allow_subns {
                            debug!("Inferred namespace {ns} is contained in existing namespace {exists_ns} ({alias})",);
                            continue;
                        }
                    }
                    let alias_opt = gen_alias(url_obj, &aliases);
                    if let Some(alias) = alias_opt {
                        debug!(
                            "Adding new namespace {} -> {} to namespace trie (size: {size})",
                            alias, ns
                        );
                        self.insert(ns, (alias.clone(), *source));
                        aliases.insert(alias.clone(), (ns.clone(), source.to_string()));
                        added.insert(&alias, ns.clone());
                    } else {
                        warn!("gen_alias() returned None for {}", ns);
                    }
                }
            }
        }

        let res = added
            .iter_leaves()
            .filter_map(|(_, node)| {
                if node.value.is_some() {
                    Some(node.value.clone().unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();
        return res;
    }
}

pub fn gen_alias(url_obj: Url, aliases: &NamespaceMap) -> Option<String> {
    let mut domains = url_obj
        .host_str()
        .unwrap_or_else(|| panic!("Url {} has no host str", url_obj.to_string()))
        .split('.');

    let alias_cand = domains.next().unwrap_or_else(|| {
        panic!(
            "domains is empty! {:#?} {:#?} {:#?}",
            url_obj.to_string(),
            url_obj,
            domains
        )
    });
    let tld = domains.last();

    let mut alias = alias_cand.to_string();
    let alias_abbrv = alias.chars().take(5).collect::<String>();

    // check if already exists
    let conflict = aliases.get(&alias);
    if let None = conflict {
        return Some(alias);
    }

    // check if tlds are different
    let confl_url = conflict.unwrap().clone().0;
    let confl_url_obj = Url::parse(&confl_url).unwrap();
    if confl_url_obj.to_string() == url_obj.to_string() {
        return None;
    }

    let confl_domains = confl_url_obj
        .host_str()
        .unwrap()
        .split('.')
        .collect::<Vec<_>>();
    let mut rev_confl_domains = confl_domains.iter().rev();
    let confl_tld = *rev_confl_domains.next().unwrap();
    if tld.is_some() && tld.unwrap() != confl_tld {
        let alias_tld = format!("{}{}", alias_abbrv, confl_tld);

        if !aliases.contains_key(&alias_tld) {
            return Some(alias_tld);
        }
    }

    // check if last segment is different
    let segs = url_obj.path_segments();
    let confl_segs = confl_url_obj.path_segments();
    if segs.is_some() && confl_segs.is_some() {
        let last_seg = segs.unwrap().last();
        let confl_last_seg = confl_segs.unwrap().last();
        if last_seg.is_some() && confl_last_seg.is_some() && last_seg != confl_last_seg {
            let alias_seg = format!("{}{}", alias_abbrv, last_seg.unwrap());
            if !aliases.contains_key(&alias_seg) {
                return Some(alias_seg);
            }
        }
    }

    // generate new number to add to alias
    let mut count = 2;
    while aliases.contains_key(alias.as_str()) {
        alias = format!("{}{}", alias_abbrv, count);
        count += 1;
    }

    return Some(alias);
}
