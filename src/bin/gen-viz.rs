use chilon_rs::{
    util::gen_file_name,
    visualization::{build_data, dump_json, render_vis, vis_dev_server},
};
use chrono::Utc;
use log::{info, warn};

use clap::Parser;
use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

use simplelog::*;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = None
    )]
pub struct Cli {
    #[arg(required = true, value_name = "FOLDER")]
    pub folder: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let log_config = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .set_target_level(LevelFilter::Error)
        .build();
    TermLogger::init(
        LevelFilter::Trace,
        log_config.clone(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let folder_str = cli.folder.to_str().unwrap();

    let vis_data = build_data(folder_str);
    dump_json(&vis_data, folder_str);

    let render_dir = render_vis(&vis_data, folder_str);
    vis_dev_server(render_dir);
}
