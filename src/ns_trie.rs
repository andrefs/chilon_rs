use std::{
    collections::{BTreeSet, HashMap},
    fs::write,
};

use crate::{trie::Node, util::gen_file_name};
use log::{debug, info};
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
    fn add_inferred_namespaces(&mut self, inferred: &Vec<String>);
}

impl InferredNamespaces for NamespaceTrie {
    fn add_inferred_namespaces(&mut self, inferred: &Vec<String>) {
        //let mut used_alias = self
        //    .iter()
        //    .filter_map(|(_, node)| node.value.clone())
        //    .collect::<BTreeSet<_>>();
        let mut alias_trie = Node::<String>::new();
        for (ns, node) in self.iter() {
            if let Some(alias) = node.value.clone() {
                alias_trie.insert(&alias, ns);
            }
        }
        for ns in inferred.iter() {
            let url_obj = Url::parse(ns.as_str()).unwrap();
            if url_obj.has_host() {
                let domains = url_obj.host_str().unwrap().split('.').collect::<Vec<_>>();
                let mut rev_domains = domains.iter().rev();
                let (tld, alias_cand) =
                    (*rev_domains.next().unwrap(), *rev_domains.next().unwrap());

                let mut alias = alias_cand.to_string();

                if alias_trie.contains_key(alias.as_str()) {
                    let alias_tld = format!("{}{}", alias, tld);
                    alias = alias_tld.clone();
                    if alias_trie.contains_key(alias.as_str()) {
                        let mut count = 2;
                        while alias_trie.contains_key(alias.as_str()) {
                            alias = format!("{}{}", alias_tld, count);
                            count += 1;
                        }
                    }
                }
                debug!("Adding new namespace {} -> {} to namespace trie", alias, ns);
                self.insert(ns, alias.clone());
                alias_trie.insert(&alias, ns.clone());
            }
        }
    }
}

//fn noConflictAlias(url1: String, url2: String, alias1: String) -> String {}
