use qp_trie::{wrapper::BString, Trie};
use regex::Regex;
use rio_api::parser::TriplesParser;
use rio_turtle::{TurtleError, TurtleParser};
use std::{
    collections::HashMap,
    fs::{create_dir_all, write, File},
    io::{BufReader, Read},
    path::Path,
};
use ureq;

const PCC_URL: &str = "https://prefix.cc/popular/all.file.ttl";
const PCC_DIR: &str = "cache";
const PCC_PATH: &str = "cache/prefix.cc.ttl";

pub fn download() {
    let res = ureq::get(&PCC_URL)
        .set("Content-Type", "text/turtle")
        .call()
        .unwrap();
    let ttl = res.into_string().unwrap();
    let fixed = fix_pcc(ttl);

    create_dir_all(PCC_DIR).unwrap();
    write(PCC_PATH, &fixed).unwrap();
    parse(fixed);
}

pub fn parse<'a>(ttl: String) -> PrefixTrie {
    let mut parser = TurtleParser::new(ttl.as_ref(), None);
    parser
        .parse_all(&mut |_| Ok(()) as Result<(), TurtleError>)
        .unwrap();
    let pfs = parser.prefixes();
    let pref_hash: HashMap<String, String> = pfs
        .iter()
        .map(|(alias, namespace)| (alias.to_owned(), namespace.to_owned()))
        .collect();

    let trie = build_namespace_trie(pref_hash);
    return trie;
}

pub fn load<'a>() -> PrefixTrie {
    if !Path::new(PCC_PATH).exists() {
        download();
    }
    let file = File::open(PCC_PATH).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut s = String::new();
    buf_reader.read_to_string(&mut s).unwrap();
    return parse(s);
}

pub type PrefixTrie = Box<Trie<BString, Box<Option<String>>>>;

pub fn build_namespace_trie<'a>(pref_hash: HashMap<String, String>) -> PrefixTrie {
    let mut t = Box::new(Trie::new());
    for (alias, namespace) in pref_hash.iter() {
        t.insert_str(namespace, Box::new(Some(alias.to_owned())));
    }
    return t;
}

fn fix_pcc(ttl: String) -> String {
    let lines = ttl.lines();
    let line_count = lines.clone().count();
    let fixed = lines.filter(|line| {
        let re = Regex::new(r"^@prefix\s+(\w+):\s+<(.*)>\s*.$").unwrap();
        let caps = re.captures(line).unwrap();
        let alias = caps.get(1).unwrap().as_str();
        let namespace = caps.get(2).unwrap().as_str();

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
    });
    let fixed_count = fixed.clone().count();
    println!("fix_pcc {} {}", line_count, fixed_count);
    return fixed.collect::<Vec<_>>().join("");
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
