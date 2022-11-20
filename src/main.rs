mod args;
mod extract;
mod iri_trie;
mod ns_trie;
mod parse;
mod prefixes;
mod trie;

use args::Cli;
use clap::Parser;
use prefixes::prefixcc;

use crate::{iri_trie::IriTrieExt, prefixes::build_iri_trie};

fn main() {
    let cli = Cli::parse();

    /*******************
     * prepare_prefixes *
     *******************/

    let mut ns_trie = prefixcc::load();
    println!("ns_map_trie: {:#?}", ns_trie);
    // // TODO: add more mappings to ns_map  from user supplied rdf file with flag -p
    let mut iri_trie = build_iri_trie(cli.files, &mut ns_trie);
    iri_trie.remove_known_prefixes(&ns_trie);
    //iri_trie.remove_leaves();
    println!("{:#?}", iri_trie);

    /*********************
     * normalize triples *
     *********************/

    /*******************
     * summarize graph *
     *******************/
}
