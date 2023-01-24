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

use crate::iri_trie::IriTrieExt;
use crate::normalize::save_normalized_triples;
use crate::prefixes::build_iri_trie;
use crate::seg_tree::SegTree;
use args::Cli;
use chilon_rs::util::gen_file_name;
use chilon_rs::visualization::{build_data, dump_json, render_vis};
use chrono::Utc;
use clap::Parser;
use log::{debug, info};
use normalize::normalize_triples;
use ns_trie::{InferredNamespaces, NamespaceTrie, SaveTrie};
use prefixes::community;
use std::fs::{self, File};
use std::path::Path;
use std::process::Command;
use std::str;

use simplelog::*;
use std::time::Instant;

fn main() {
    /**********************
     * Initializing stuff *
     **********************/

    // start timers
    let start = Instant::now();
    let mut task_start = start;

    // create output folder
    let out = gen_file_name(
        format!("results/{}", Utc::now().format("%Y%m%d")),
        "".to_string(),
    );
    let outf = out.as_str();
    println!("Creating folder {outf} to store results");
    fs::create_dir(outf).unwrap();

    // start logging
    init_log(outf);
    info!("Created folder {outf} to store results");

    // log git commit
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .unwrap();
    info!(
        "Running from commit {}",
        str::from_utf8(&output.stdout).unwrap().trim()
    );

    let cli = Cli::parse();

    /**********************
     * Prepare namespaces *
     **********************/

    info!("Loading community namespaces");
    let mut ns_trie: NamespaceTrie = community::load();

    if cli.infer_ns {
        task_start = Instant::now();

        // TODO: add more mappings to ns_map  from user supplied rdf file with flag -p
        info!("Getting namespaces");
        let mut iri_trie = build_iri_trie(cli.files.clone(), &mut ns_trie);

        info!("Inferring namespaces from IRIs left");
        let seg_tree = SegTree::from(&iri_trie);
        let (inferred, gbg_collected) = seg_tree.infer_namespaces();

        debug!("Adding inferred namespaces");
        let added = ns_trie.add_namespaces(&inferred);

        debug!("Removing IRIs with inferred namespaces");
        iri_trie.remove_prefixes(&added);

        debug!("Removing IRIs with garbage collected namespaces");
        iri_trie.remove_prefixes(&gbg_collected);

        //warn!(
        //    "IRIs without namespace: {:?}",
        //    iri_trie.iter().map(|x| x.0).collect::<Vec<_>>()
        //);

        debug!("Saving namespaces");
        ns_trie.save(outf);

        restart_task_timer(&mut task_start, "Finished namespace inferrence");
    }

    /*********************
     * Normalize triples *
     *********************/

    info!("Normalizing triples");
    let (nts, used_groups) =
        normalize_triples(cli.files.clone(), &mut ns_trie, cli.ignore_unknown, outf);

    debug!("saving normalized triples");
    save_normalized_triples(&nts, used_groups, Some(10), outf); // min_occurs = 10

    restart_task_timer(&mut task_start, "Finished summarizing graph");

    /*****************
     * Visualization *
     *****************/

    //let mut summ = load_summary(path);

    let vis_data = build_data(outf);
    dump_json(&vis_data, outf);
    render_vis(&vis_data, outf);

    restart_task_timer(&mut task_start, "Finished generating visualization");
}

fn restart_task_timer(timer: &mut Instant, msg: &str) {
    info!("{msg} ({:?})", timer.elapsed());
    *timer = Instant::now();
}

fn init_log(outf: &str) {
    let file_path = Path::new(".").join(outf).join("chilon.log");
    let term_log_config = ConfigBuilder::new()
        .set_location_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Error)
        .set_thread_mode(ThreadLogMode::Both)
        .build();
    let file_log_config = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .set_target_level(LevelFilter::Error)
        .build();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Trace,
            term_log_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            file_log_config,
            File::create(file_path).unwrap(),
        ),
    ])
    .unwrap();
}
