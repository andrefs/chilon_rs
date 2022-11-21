mod args;
mod extract;
mod iri_trie;
mod normalize;
mod ns_trie;
mod parse;
mod prefixes;
mod trie;
mod util;

use crate::iri_trie::{IriTrie, IriTrieExt};
use crate::normalize::save_normalized_triples;
use crate::prefixes::build_iri_trie;
use crate::prefixes::infer_namespaces;
use args::Cli;
use clap::Parser;
use log::info;
use normalize::normalize_triples;
use ns_trie::{InferredNamespaces, NamespaceTrie, SaveTrie};
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
    let mut iri_trie: IriTrie = build_iri_trie(cli.files.clone(), &mut ns_trie);
    iri_trie.remove_leaves();
    let inferred = infer_namespaces(&iri_trie);
    ns_trie.add_inferred_namespaces(inferred);

    info!("Saving namespaces");
    ns_trie.save(); // TODO

    /*********************
     * normalize triples *
     *********************/
    info!("Normalizing triples");
    let nts = normalize_triples(cli.files.clone(), &mut ns_trie); // TODO
    save_normalized_triples(&nts);

    println!("{:#?}", nts)

    /*******************
     * summarize graph *
     *******************/

    //println!("{:#?}", iri_trie);
}
