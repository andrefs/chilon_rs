use crate::trie::Node;
use log::{debug, info};
use url::Url;

pub type NamespaceTrie = Node<String>;

pub trait SaveTrie {
    fn save(&self);
}

impl SaveTrie for NamespaceTrie {
    fn save(&self) {
        todo!()
    }
}

pub trait InferredNamespaces {
    fn add_inferred_namespaces(&mut self, inferred: Vec<String>);
}

impl InferredNamespaces for NamespaceTrie {
    fn add_inferred_namespaces(&mut self, inferred: Vec<String>) {
        for ns in inferred.iter() {
            let url_obj = Url::parse(ns.as_str()).unwrap();
            if url_obj.has_host() {
                let domains = url_obj.host_str().unwrap().split('.').collect::<Vec<_>>();
                let mut rev_domains = domains.iter().rev();
                let (tld, alias_cand) =
                    (*rev_domains.next().unwrap(), *rev_domains.next().unwrap());

                let mut alias = alias_cand.to_string();

                if self.contains_key(alias.as_str()) {
                    let alias_tld = format!("{}{}", alias, tld);
                    alias = alias_tld.clone();
                    if self.contains_key(alias.as_str()) {
                        let mut count = 2;
                        while self.contains_key(alias.as_str()) {
                            alias = format!("{}{}", alias_tld, count);
                            count += 1;
                        }
                    }
                }
                debug!("Adding new namespace {} -> {} to namespace trie", alias, ns);
                self.insert(ns, alias);
            }
        }
    }
}
