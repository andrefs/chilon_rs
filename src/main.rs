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

use crate::iri_trie::NodeStats;
use crate::iri_trie::Stats;
use crate::iri_trie::{IriTrie, IriTrieExt};
use crate::normalize::save_normalized_triples;
use crate::prefixes::build_iri_trie;
use crate::prefixes::infer_namespaces;
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
    info!("Inferring namespaces");

    debug!("Building IRI trie");

    let mut iri_trie: IriTrie = build_iri_trie(cli.files.clone(), &mut ns_trie);

    // /// TESTING STUFF
    // debug_unknown_namespaces(&mut iri_trie);

    debug!("Removing IRI trie leaves");
    //iri_trie.remove_leaves();
    debug!("Inferring namespaces");

    let seg_tree = SegTree::from(iri_trie);
    let inferred = seg_tree.infer_namespaces();

    debug!("Adding inferred namespaces");
    ns_trie.add_inferred_namespaces(inferred);

    info!("Saving namespaces");
    ns_trie.save();

    /*********************
     * Normalize triples *
     *********************/

    info!("Normalizing triples");
    let nts = normalize_triples(cli.files.clone(), &mut ns_trie); // TODO improve

    debug!("saving normalized triples");
    save_normalized_triples(&nts);
}

fn debug_unknown_namespaces(iri_trie: &mut IriTrie) {
    let unmatched = iri_trie.count();

    let mut rng = thread_rng();

    let mut urls = Vec::from([
        //"http://wordnet-rdf.princeton.edu/ontology#partOfSpeech".to_string(),
        //"http://wordnet-rdf.princeton.edu/id/00001740-n".to_string(),
        //"http://ili.globalwordnet.org/ili/i35545".to_string(),
        //"http://wikipedia.org/wiki/Synchronized_swimming".to_string(),
        //"https://www.w3.org/2009/08/skos-reference/skos-owl1-dl.rdf".to_string(),
        //"http://wordnet-rdf.princeton.edu/rdf/lemma/pair_of_trousers#pair_of_trousers-04496264-n",

        // //"http://github.com/andrefs.com/project1/somefolter/ontology/a-resource",
        // //"http://github.com/andrefs.com/project2/ontology#a-resource",
    ]);

    let mut next_rand = 0;
    for (i, (url, node)) in iri_trie.iter().enumerate() {
        if (i - next_rand) % 1000 == 0 {
            urls.push(url);
            next_rand = rng.gen_range(1..25);
        }
    }

    for u in urls {
        let mut v: Vec<(String, String)> = Vec::new();
        iri_trie.value_along_path(u.to_string(), "".to_string(), &mut v);

        for (url, stats) in v.iter() {
            println!("{:indent$} {:?} ", url, stats, indent = u.len());
        }
    }

    panic!("FIM.");
}
