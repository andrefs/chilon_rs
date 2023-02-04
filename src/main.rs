#![feature(btree_drain_filter)]

mod args;
mod counter;
mod extract;
mod iri_trie;
mod normalize;
mod ns_trie;
mod parse;
mod prefixes;
mod seg_tree;
mod timed_task;
mod trie;
mod util;
mod visualization;

use crate::iri_trie::IriTrieExt;
use crate::normalize::save_normalized_triples;
use crate::prefixes::build_iri_trie;
use crate::seg_tree::SegTree;
use crate::timed_task::{Task, TaskType};
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
    let tasks_path = Path::new(".").join(outf).join("tasks.jsonl");
    let mut full_t = Task::new("full".to_string(), TaskType::Execution, tasks_path.clone());

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
        let mut infer_t = Task::new(
            "infer".to_string(),
            TaskType::InferNamespaces,
            tasks_path.clone(),
        );

        // TODO: add more mappings to ns_map  from user supplied rdf file with flag -p
        info!("Getting namespaces");
        let (mut iri_trie, trip_c) =
            build_iri_trie(cli.files.clone(), &mut ns_trie, &mut infer_t, allow_subns);
        infer_t.triples = trip_c;

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
    }

    /*********************
     * Normalize triples *
     *********************/

    let mut norm_t = Task::new(
        "normalize".to_string(),
        TaskType::Normalize,
        tasks_path.clone(),
    );

    info!("Normalizing triples");
    let (nts, used_groups, trip_c) = normalize_triples(
        cli.files.clone(),
        &mut ns_trie,
        cli.ignore_unknown,
        outf,
        &mut norm_t,
    );

    norm_t.triples = trip_c as usize;
    full_t.triples = trip_c as usize;

    info!("Saving normalized triples");
    save_normalized_triples(&nts, used_groups, Some(10), outf); // min_occurs = 10

    norm_t.finish("Finished summarizing graph");

    /*****************
     * Visualization *
     *****************/

    //let mut summ = load_summary(path);

    let mut vis_t = Task::new(
        "visualization".to_string(),
        TaskType::Visualization,
        tasks_path,
    );

    let vis_data = build_data(outf);
    dump_json(&vis_data, outf);

    full_t.finish("Finished summarizing graph");
    let render_dir = render_vis(&vis_data, outf);

    vis_t.finish("Finished generating visualization");
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
