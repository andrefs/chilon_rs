use csv;
use itertools::Itertools;
use log::warn;
use regex::Regex;
use serde::Deserialize;
use std::{
    cmp::Ordering,
    fs::{create_dir_all, write, File},
    io::{BufReader, Read},
    path::Path,
};
use ureq;

use crate::ns_trie::{NamespaceSource, NamespaceTrie};

const PV_URL: &str =
    "https://raw.githubusercontent.com/linkml/prefixmaps/main/src/prefixmaps/data/merged.csv";
const PV_DIR: &str = "cache";
const PV_PATH: &str = "cache/prefixmap.json";

pub type PrefixVec = Vec<(String, String)>;

#[derive(Deserialize)]
struct Record {
    prefix: String,
    namespace: String,
    status: String,
}

pub fn download() {
    let res = ureq::get(PV_URL).call().unwrap();
    let reader = res.into_reader();
    let v = parse(reader);
    let fixed = fix_pv(v);

    create_dir_all(PV_DIR).unwrap();
    write(PV_PATH, serde_json::to_string_pretty(&fixed).unwrap()).unwrap();
}

fn parse(reader: impl Read) -> Vec<Record> {
    csv::Reader::from_reader(reader)
        .into_deserialize()
        .filter_map(|res| res.unwrap())
        .collect()
}

fn vec_to_trie(v: PrefixVec, allow_subns: bool) -> NamespaceTrie {
    let mut t = NamespaceTrie::new();
    for (alias, namespace) in v.into_iter().sorted_by(|(_, ns1), (_, ns2)| {
        let len1 = ns1.len();
        let len2 = ns2.len();

        if len1 < len2 {
            Ordering::Less
        } else if len1 > len2 {
            Ordering::Greater
        } else {
            ns1.cmp(ns2)
        }
    }) {
        let res = t.longest_prefix(namespace.as_str(), true);
        if let Some((node, ns)) = res {
            if let Some((existing_alias, _)) = &node.value {
                if namespace.eq(&ns) {
                    warn!("Namespace {namespace} (alias {alias}) is already in trie with alias {existing_alias}");
                    continue;
                }
                if !allow_subns {
                    //warn!(
                    //    "Won't insert namespace {namespace} (alias {alias}) because shorter namespace {} (alias {}) already exists",
                    //    existing_alias,
                    //    ns
                    //);
                    continue;
                }
            }
        }
        t.insert(&namespace, (alias.clone(), NamespaceSource::Community));
    }
    t
}

pub fn load(allow_subns: bool) -> NamespaceTrie {
    if !Path::new(PV_PATH).exists() {
        download();
    }
    let file = File::open(PV_PATH).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut s = String::new();
    buf_reader.read_to_string(&mut s).unwrap();

    let map: PrefixVec = serde_json::from_str(s.as_str()).unwrap();
    vec_to_trie(map, allow_subns)
}

fn fix_pv(pv: Vec<Record>) -> PrefixVec {
    let fixed: PrefixVec = pv
        .iter()
        .filter(|r| r.status == "canonical")
        .filter(|r| {
            // TODO improve
            if r.prefix.contains("walmart") && r.namespace.contains("amazon") {
                return false;
            }
            if r.prefix.contains("movie")
                && r.namespace.contains("data.linkedmdb.org/resource/movie")
            {
                return false;
            }

            // https://www.w3.org/2006/vcard/ns#latitude#" -> invalid URL
            if Regex::new("#.*#").unwrap().is_match(r.namespace.as_str()) {
                return false;
            }

            true
        })
        .map(|r| (r.prefix.to_owned(), r.namespace.to_owned()))
        .collect();
    fixed
}

//  @prefix walmart:    <https://www.amazon.de/>.
//  @prefix movie:      <http://data.linkedmdb.org/resource/movie/>.
//  @prefix linkedmdb:  <http://data.linkedmdb.org/>.
//  remove namespaces supersets of other namespaces

//fn remove_pair(line: String) -> bool {
//    let re = Regex::new;
//
//    let line_re = re(r"^")
//
//
//
//    let blacklisted_pairs = HashMap::<&str, &str>::from([
//        ("^walmart$", "^https://www.amazon.de/$"),
//        ("^linkedmdb$", "http://data.linkedmdb.org/$"),
//    ]);
//
//    return false;
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_pv_filters_non_canonical() {
        let pv = vec![
            Record {
                prefix: "good".into(),
                namespace: "http://good.org/".into(),
                status: "canonical".into(),
            },
            Record {
                prefix: "bad".into(),
                namespace: "http://bad.org/".into(),
                status: "deprecated".into(),
            },
        ];
        let fixed = fix_pv(pv);
        assert_eq!(fixed.len(), 1);
        assert_eq!(fixed[0].0, "good");
    }

    #[test]
    fn test_fix_pv_filters_walmart_amazon() {
        let pv = vec![Record {
            prefix: "walmart".into(),
            namespace: "https://www.amazon.de/".into(),
            status: "canonical".into(),
        }];
        let fixed = fix_pv(pv);
        assert!(fixed.is_empty());
    }

    #[test]
    fn test_fix_pv_filters_double_hash() {
        let pv = vec![Record {
            prefix: "bad".into(),
            namespace: "http://example.org/#foo#bar".into(),
            status: "canonical".into(),
        }];
        let fixed = fix_pv(pv);
        assert!(fixed.is_empty());
    }

    #[test]
    fn test_vec_to_trie_basic() {
        let v = vec![("ex".into(), "http://example.org/".into())];
        let trie = vec_to_trie(v, false);
        let res = trie.longest_prefix("http://example.org/foo", true);
        assert!(res.is_some());
        if let Some((node, _)) = res {
            assert_eq!(node.value.as_ref().unwrap().0, "ex");
        }
    }

    #[test]
    fn test_vec_to_trie_sort_by_length() {
        let v = vec![
            ("long".into(), "http://long.org/very/long/path/".into()),
            ("short".into(), "http://s.org/".into()),
        ];
        let trie = vec_to_trie(v, false);
        // Shorter namespace inserted first, but both are non-overlapping
        let res = trie.longest_prefix("http://s.org/x", true);
        assert!(res.is_some());
        if let Some((node, _)) = res {
            assert_eq!(node.value.as_ref().unwrap().0, "short");
        }
        let res2 = trie.longest_prefix("http://long.org/very/long/path/y", true);
        assert!(res2.is_some());
        if let Some((node, _)) = res2 {
            assert_eq!(node.value.as_ref().unwrap().0, "long");
        }
    }
}
