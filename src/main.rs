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
mod visualization;

use crate::iri_trie::{IriTrie, IriTrieExt};
use crate::normalize::save_normalized_triples;
use crate::prefixes::build_iri_trie;
use crate::seg_tree::SegTree;
use args::Cli;
use chilon_rs::util::gen_file_name;
use chilon_rs::visualization::{build_data, dump_json, load_summary, render_vis};
use chrono::Utc;
use clap::Parser;
use log::{debug, info};
use normalize::normalize_triples;
use ns_trie::{InferredNamespaces, NamespaceTrie, SaveTrie};
use prefixes::prefixcc;
use simple_logger::SimpleLogger;
use std::fs;

fn main() {
    /**********************
     * Initializing stuff *
     **********************/

    let cli = Cli::parse();
    SimpleLogger::new().init().unwrap();

    let out = gen_file_name(
        format!("results/{}", Utc::now().format("%Y%m%d")),
        "".to_string(),
    );
    let outf = out.as_str();
    info!("Creating folder {outf} to store results");
    fs::create_dir(outf).unwrap();

    /**********************
     * Prepare namespaces *
     **********************/

    info!("Loading namespaces from Prefix.cc");
    let mut ns_trie: NamespaceTrie = prefixcc::load();

    if cli.infer_ns {
        // TODO: add more mappings to ns_map  from user supplied rdf file with flag -p
        info!("Getting namespaces");
        let mut iri_trie: IriTrie = build_iri_trie(cli.files.clone(), &mut ns_trie);

        info!("Inferring namespaces from IRIs left");
        let seg_tree = SegTree::from(&iri_trie);
        let (inferred, gbg_collected) = seg_tree.infer_namespaces();

        debug!("Adding inferred namespaces");
        let added = ns_trie.add_inferred_namespaces(&inferred);

        debug!("Removing IRIs with inferred namespaces");
        iri_trie.remove_prefixes(&added);

        debug!("Removing IRIs with garbage collected namespaces");
        iri_trie.remove_prefixes(&gbg_collected);

        //warn!(
        //    "IRIs without namespace: {:?}",
        //    iri_trie.iter().map(|x| x.0).collect::<Vec<_>>()
        //);

        ns_trie.save(outf);
    }

    /*********************
     * Normalize triples *
     *********************/

    info!("Normalizing triples");
    let (nts, used_groups) =
        normalize_triples(cli.files.clone(), &mut ns_trie, cli.ignore_unknown, outf);

    debug!("saving normalized triples");
    save_normalized_triples(&nts, used_groups, Some(10), outf); // min_occurs = 10

    /*****************
     * Visualization *
     *****************/

    //let mut summ = load_summary(path);

    let vis_data = build_data(outf);
    dump_json(&vis_data, outf);
    render_vis(&vis_data, outf);
}
