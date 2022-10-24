use rio_api::parser::TriplesParser;
use rio_turtle::{TurtleError, TurtleParser};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};
use trie_generic::Trie;
use ureq;

pub fn download() {
    let pcc_url = "https://prefix.cc/popular/all.file.ttl";
    let res = ureq::get(pcc_url)
        .set("Content-Type", "text/turtle")
        .call()
        .unwrap();
    let ttl = res.into_string().unwrap();
    parse(ttl);
}

pub fn parse(ttl: String) -> Trie<String> {
    let mut parser = TurtleParser::new(ttl.as_ref(), None);
    parser
        .parse_all(&mut |_| Ok(()) as Result<(), TurtleError>)
        .unwrap();
    let pfs = parser.prefixes();
    let pref_hash = pfs.to_owned();

    return build_namespace_trie(pref_hash);
}

pub fn load() -> Trie<String> {
    let pcc_path = "cache/prefix.cc.ttl";
    let file = File::open(pcc_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut s = String::new();
    buf_reader.read_to_string(&mut s).unwrap();
    return parse(s);
}

pub fn build_namespace_trie(pref_hash: HashMap<String, String>) -> Trie<String> {
    let mut t = Trie::new(None);
    for (alias, namespace) in pref_hash.iter() {
        t.add(namespace, Some(alias.to_owned()));
    }
    return t;
}
