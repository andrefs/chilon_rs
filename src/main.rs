#![feature(btree_drain_filter)]

mod args;
mod counter;
mod extract;
mod iri_trie;
mod meta_info;
mod normalize;
mod ns_trie;
mod parse;
mod prefixes;
mod seg_tree;
mod trie;
mod util;
mod visualization;

use crate::iri_trie::IriTrieExt;
use crate::meta_info::{MetaInfo, MetaInfoNormalization, MetaInfoVisualization, StageTask};
use crate::normalize::save_normalized_triples;
use crate::prefixes::build_iri_trie;
use crate::seg_tree::SegTree;
use args::Cli;
use chilon_rs::util::gen_file_name;
use chilon_rs::visualization::{build_data, dump_json, render_vis, vis_dev_server};
use chrono::Utc;
use clap::Parser;
use log::info;
use normalize::normalize_triples;
use ns_trie::{InferredNamespaces, NamespaceTrie, SaveTrie};
use prefixes::community;
use std::fs::{self, File};
use std::path::Path;
use std::process::Command;
use std::str;

use simplelog::*;

fn main() {
    /**********************
     * Initializing stuff *
     **********************/

    // output folder path
    let out = gen_file_name(
        format!("results/{}", Utc::now().format("%Y%m%d")),
        "".to_string(),
    );
    let outf = out.as_str();

    // create output folder
    println!("Creating folder {outf} to store results");
    fs::create_dir(outf).unwrap();

    // start timers
    let tasks_path = Path::new(".").join(outf).join("tasks.json");
    let mut meta = MetaInfo::new(tasks_path);

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

    let allow_subns = false;

    info!("Loading community namespaces");
    let mut ns_trie: NamespaceTrie = community::load(allow_subns);

    if cli.infer_ns {
        info!("Getting namespaces");
        // TODO: add more mappings to ns_map  from user supplied rdf file with flag -p
        let mut infer_t = meta_info::MetaInfoInference::new();
        let (mut iri_trie, tasks, hk) =
            build_iri_trie(cli.files.clone(), &mut ns_trie, allow_subns);
        infer_t.add_tasks(tasks);
        infer_t.housekeeping = hk;

        info!("Inferring namespaces from IRIs left");
        let seg_tree = SegTree::from(&iri_trie);
        let (inferred, gbg_collected) = seg_tree.infer_namespaces();

        info!("Adding inferred namespaces");
        let added = ns_trie.add_namespaces(&inferred, allow_subns);

        info!("Removing IRIs with inferred namespaces");
        iri_trie.remove_prefixes(&added);

        info!("Removing IRIs with garbage collected namespaces");
        iri_trie.remove_prefixes(&gbg_collected);

        //warn!(
        //    "IRIs without namespace: {:?}",
        //    iri_trie.iter().map(|x| x.0).collect::<Vec<_>>()
        //);

        info!("Saving namespaces");
        ns_trie.save(outf);

        infer_t.finish("Finished namespace inference");
        meta.inference = Some(infer_t);
    }

    /*********************
     * Normalize triples *
     *********************/

    let mut norm_t = MetaInfoNormalization::new();

    info!("Normalizing triples");
    let (nts, used_groups, tasks) =
        normalize_triples(cli.files.clone(), &mut ns_trie, cli.ignore_unknown, outf);

    norm_t.add_tasks(tasks);
    norm_t.namespaces = used_groups.namespaces.len();

    info!("Saving normalized triples");
    save_normalized_triples(&nts, used_groups, Some(10), outf); // min_occurs = 10

    norm_t.finish("Finished summarizing graph");
    meta.normalization = Some(norm_t);

    /*****************
     * Visualization *
     *****************/

    //let mut summ = load_summary(path);

    let mut vis_t = MetaInfoVisualization::new();

    let vis_data = build_data(outf);
    dump_json(&vis_data, outf);

    let render_dir = render_vis(&vis_data, outf);

    vis_t.finish("Finished generating visualization");
    meta.visualization = Some(vis_t);
    meta.save();
    //vis_dev_server(render_dir);
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
