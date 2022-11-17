mod args;
mod extract;
mod iri_trie;
mod parse;
mod prefixes;
mod trie;

use args::Cli;
use clap::Parser;
use prefixes::prefixcc;

use crate::{iri_trie::IriTrieExt, prefixes::build_iri_trie};

fn main() {
    let cli = Cli::parse();

    // prepare_prefixes
    // normalize triples
    // summarize graph

    let mut ns_map = prefixcc::load();
    // // TODO: add more mappings to ns_map  from user supplied rdf file with flag -p
    let mut iri_trie = build_iri_trie(cli.files, &mut ns_map);
    //println!("Total IRIs: {}", iri_trie.count_terminals());
    println!("{:#?}", iri_trie);
    iri_trie.remove_known_prefixes(&ns_map);
    println!("{:#?}", iri_trie);
    //println!("Unmatched IRIs: {}", iri_trie.count_terminals());

    //println!("\n\n\ndown");
    //iri_trie.traverse(&|key, value| println!("{key}"));

    //println!("Remove leaves 1: {}", remove_leaves(&mut iri_trie));
    ////println!("Remove leaves 2: {}", remove_leaves(&mut iri_trie));
    ////println!("Remove leaves 3: {}", remove_leaves(&mut iri_trie));

    //println!("\n\n\nup");
    //iri_trie.traverse_up(&|key, value| println!("{key}"));
    //println!("{:#?}", iri_trie);
    //println!("{}", iri_trie.pp(true));
}
