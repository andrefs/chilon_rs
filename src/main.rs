mod args;
mod extract;
mod iri_trie;
//mod normalize;
mod ns_trie;
mod parse;
mod prefixes;
mod trie;

use args::Cli;
use clap::Parser;
//use normalize::normalize_triples;
use ns_trie::{NamespaceTrie, SaveTrie};
use prefixes::prefixcc;

use crate::prefixes::build_iri_trie;

fn main() {
    let cli = Cli::parse();

    /*******************
     * prepare_prefixes *
     *******************/

    let mut ns_trie: NamespaceTrie = prefixcc::load();
    // // TODO: add more mappings to ns_map  from user supplied rdf file with flag -p
    let mut iri_trie = build_iri_trie(cli.files.clone(), &mut ns_trie);
    //ns_trie.save(); // TODO

    //iri_trie.remove_leaves();
    //ns_trie.add_infered_namespaces(iri_trie);

    /*********************
     * normalize triples *
     *********************/
    //normalize_triples(cli.files.clone(), &mut ns_trie); // TODO

    /*******************
     * summarize graph *
     *******************/

    //println!("{:#?}", iri_trie);
}
