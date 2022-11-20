use regex::Regex;
use std::{
    collections::BTreeMap,
    fs::{create_dir_all, write, File},
    io::{BufReader, Read},
    path::Path,
};
use ureq;

use crate::ns_trie::NamespaceTrie;

const PCC_URL: &str = "https://prefix.cc/popular/all.file.json";
const PCC_DIR: &str = "cache";
const PCC_PATH: &str = "cache/prefix.cc.json";

pub type PrefixMap = BTreeMap<String, String>;

pub fn download() {
    let res = ureq::get(&PCC_URL).call().unwrap();
    let json = res.into_string().unwrap();
    let map = parse(json);
    let fixed = fix_pcc(map);

    create_dir_all(PCC_DIR).unwrap();
    write(PCC_PATH, serde_json::to_string_pretty(&fixed).unwrap()).unwrap();
}

pub fn parse<'a>(json: String) -> PrefixMap {
    let m: PrefixMap = serde_json::from_str(json.as_str()).unwrap();
    m
}

fn map_to_trie<'a>(map: PrefixMap) -> NamespaceTrie {
    let mut t = NamespaceTrie::new();
    for (k, v) in map.iter() {
        t.insert(v, k.clone());
    }
    return t;
}

pub fn load() -> NamespaceTrie {
    if !Path::new(PCC_PATH).exists() {
        download();
    }
    let file = File::open(PCC_PATH).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut s = String::new();
    buf_reader.read_to_string(&mut s).unwrap();

    let map: PrefixMap = serde_json::from_str(s.as_str()).unwrap();
    return map_to_trie(map);
}

fn fix_pcc(ns_map: PrefixMap) -> PrefixMap {
    let fixed: PrefixMap = ns_map
        .iter()
        .filter(|(alias, namespace)| {
            // TODO improve
            if alias.contains("walmart") && namespace.contains("amazon.es") {
                return false;
            }
            if alias.contains("movie") && namespace.contains("data.linkedmdb.org/resource/movie") {
                return false;
            }

            if alias.contains("linkedmdb") && namespace.contains("data.linkedmdb.org") {
                return false;
            }

            // https://www.w3.org/2006/vcard/ns#latitude#"
            if Regex::new("#.*#").unwrap().is_match(namespace) {
                return false;
            }

            return true;
        })
        .map(|(k, v)| ((k.to_owned(), v.to_owned())))
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
