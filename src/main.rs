mod args;
mod extract;
mod iri_trie;
mod normalize;
mod ns_trie;
mod parse;
mod prefixes;
mod trie;

use crate::prefixes::build_iri_trie;
use args::Cli;
use clap::Parser;
use log::{debug, error, info, log_enabled, Level};
use normalize::normalize_triples;
use ns_trie::{NamespaceTrie, SaveTrie};
use prefixes::prefixcc;
use simple_logger::SimpleLogger;

fn main() {
    let cli = Cli::parse();
    SimpleLogger::new().init().unwrap();

    /*******************
     * prepare_prefixes *
     *******************/

    info!("Loading namespaces from Prefix.cc");
    let mut ns_trie: NamespaceTrie = prefixcc::load();

    // // TODO: add more mappings to ns_map  from user supplied rdf file with flag -p
    info!("Infering namespaces");
    let mut iri_trie = build_iri_trie(cli.files.clone(), &mut ns_trie);

    info!("Saving namespaces");
    //ns_trie.save(); // TODO

    //iri_trie.remove_leaves();
    //ns_trie.add_infered_namespaces(iri_trie);

    /*********************
     * normalize triples *
     *********************/
    info!("Normalizing triples");
    normalize_triples(cli.files.clone(), &mut ns_trie); // TODO

    /*******************
     * summarize graph *
     *******************/

    //println!("{:#?}", iri_trie);
}
