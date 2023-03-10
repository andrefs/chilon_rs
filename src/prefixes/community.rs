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
    context: String,
    prefix: String,
    namespace: String,
    status: String,
}

pub fn download() {
    let res = ureq::get(&PV_URL).call().unwrap();
    let reader = res.into_reader();
    let v = parse(reader);
    let fixed = fix_pv(v);

    create_dir_all(PV_DIR).unwrap();
    write(PV_PATH, serde_json::to_string_pretty(&fixed).unwrap()).unwrap();
}

fn parse<'a>(reader: impl Read) -> Vec<Record> {
    csv::Reader::from_reader(reader)
        .into_deserialize()
        .filter_map(|res| res.unwrap())
        .collect()
}

fn vec_to_trie<'a>(v: PrefixVec, allow_subns: bool) -> NamespaceTrie {
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
            if node.value.is_some() {
                let (existing_alias, _) = node.value.as_ref().unwrap().clone();

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
    return t;
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
    return vec_to_trie(map, allow_subns);
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

            return true;
        })
        .map(|r| ((r.prefix.to_owned(), r.namespace.to_owned())))
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
