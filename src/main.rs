#![feature(btree_drain_filter)]

mod args;
mod extract;
mod iri_trie;
mod normalize;
mod ns_trie;
mod parse;
mod prefixes;
mod seg_tree;
mod trie;
mod util;

use crate::iri_trie::{IriTrie, IriTrieExt};
use crate::normalize::save_normalized_triples;
use crate::prefixes::build_iri_trie;
use crate::seg_tree::SegTree;
use args::Cli;
use clap::Parser;
use log::{debug, info};
use normalize::normalize_triples;
use ns_trie::{InferredNamespaces, NamespaceTrie, SaveTrie};
use prefixes::prefixcc;
use rand::{thread_rng, Rng};
use simple_logger::SimpleLogger;

fn main() {
    let cli = Cli::parse();
    SimpleLogger::new().init().unwrap();

    /**********************
     * Prepare namespaces *
     **********************/

    info!("Loading namespaces from Prefix.cc");
    let mut ns_trie: NamespaceTrie = prefixcc::load();

    // // TODO: add more mappings to ns_map  from user supplied rdf file with flag -p
    info!("Getting namespaces");
    let mut iri_trie: IriTrie = build_iri_trie(cli.files.clone(), &mut ns_trie);

    // /// TESTING STUFF
    // debug_unknown_namespaces(&mut iri_trie);

    //debug!("Removing IRI trie leaves");
    //iri_trie.remove_leaves();

    info!("Inferring namespaces from IRIs left");
    let seg_tree = SegTree::from(&iri_trie);
    let inferred = seg_tree.infer_namespaces();

    debug!("Adding inferred namespaces");
    let added = ns_trie.add_inferred_namespaces(&inferred);

    debug!("Removing IRIs with inferred namespaces");
    iri_trie.remove_known_prefixes(&added);

    //warn!(
    //    "IRIs without namespace: {:?}",
    //    iri_trie.iter().map(|x| x.0).collect::<Vec<_>>()
    //);

    ns_trie.save();

    /*********************
     * Normalize triples *
     *********************/

    info!("Normalizing triples");
    let nts = normalize_triples(cli.files.clone(), &mut ns_trie); // TODO improve

    debug!("saving normalized triples");
    save_normalized_triples(&nts);
}
