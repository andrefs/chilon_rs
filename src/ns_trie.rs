use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs::write,
};

use crate::{ns_trie, trie::Node, util::gen_file_name};
use log::{debug, info, warn};
use url::Url;

pub type NamespaceTrie = Node<String>;

pub trait SaveTrie {
    fn save(&self);
}

impl SaveTrie for NamespaceTrie {
    fn save(&self) {
        let file_path = gen_file_name("results/all-prefixes".to_string(), "json".to_string());
        info!("Saving namespaces in {}", file_path);
        let mut ns_map = HashMap::<String, String>::new();
        for (ns, node) in self.iter_leaves() {
            ns_map.insert(node.value.as_ref().unwrap().clone(), ns);
        }

        write(file_path, serde_json::to_string_pretty(&ns_map).unwrap()).unwrap();
    }
}

pub trait InferredNamespaces {
    fn add_inferred_namespaces(&mut self, inferred: &Vec<(String, usize)>) -> Vec<String>;
}

impl InferredNamespaces for NamespaceTrie {
    fn add_inferred_namespaces(&mut self, inferred: &Vec<(String, usize)>) -> Vec<String> {
        let mut added = Node::<String>::new();
        let mut aliases = Node::<String>::new();
        for (ns, node) in self.iter() {
            if let Some(alias) = node.value.clone() {
                aliases.insert(&alias, ns);
            }
        }

        for (ns, size) in inferred.iter() {
            match Url::parse(ns.as_str()) {
                Err(err) => warn!("Could not parse IRI {ns}: {err}"),
                Ok(url_obj) => {
                    if !url_obj.has_host() {
                        warn!("IRI {ns} does not have host");
                        continue;
                    }
                    let alias = gen_alias(url_obj, &aliases);
                    debug!(
                        "Adding new namespace {} -> {} to namespace trie (size: {size})",
                        alias, ns
                    );
                    self.insert(ns, alias.clone());
                    aliases.insert(&alias.clone(), ns.clone());
                    added.insert(&alias.clone(), ns.clone());
                }
            }
        }

        return added
            .iter_leaves()
            .filter_map(|(_, node)| {
                if node.value.is_some() {
                    Some(node.value.clone().unwrap())
                } else {
                    None
                }
            })
            .collect();
    }
}

fn gen_alias(url_obj: Url, aliases: &Node<String>) -> String {
    let domains = url_obj.host_str().unwrap().split('.').collect::<Vec<_>>();
    let mut rev_domains = domains.iter().rev();
    let (tld, alias_cand) = (*rev_domains.next().unwrap(), *rev_domains.next().unwrap());

    let mut alias = alias_cand.to_string();
    let alias_abbrv = alias.chars().take(5).collect::<String>();

    // check if already exists
    let conflict = aliases.find(&alias, true);
    if let None = conflict {
        return alias;
    }

    // check if tlds are different
    let confl_url = conflict.unwrap().value.clone().unwrap();
    let confl_url_obj = Url::parse(&confl_url).unwrap();

    let confl_domains = confl_url_obj
        .host_str()
        .unwrap()
        .split('.')
        .collect::<Vec<_>>();
    let mut rev_confl_domains = confl_domains.iter().rev();
    let confl_tld = *rev_confl_domains.next().unwrap();
    if tld != confl_tld {
        let alias_tld = format!("{}{}", alias_abbrv, confl_tld);

        if !aliases.contains_key(&alias_tld) {
            return alias_tld;
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
                return alias_seg;
            }
        }
    }

    // generate new number to add to alias
    let mut count = 2;
    while aliases.contains_key(alias.as_str()) {
        alias = format!("{}{}", alias_abbrv, count);
        count += 1;
    }

    return alias;
}
