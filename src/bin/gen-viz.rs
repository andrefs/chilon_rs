use chilon_rs::{
    util::gen_file_name,
    visualization::{render_vis, vis_dev_server},
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
    #[arg(required = true, value_name = "DATA.JSON")]
    pub file: PathBuf,
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

    //let out = gen_file_name(
    //    format!("tmp/{}", Utc::now().format("%Y%m%d")),
    //    "".to_string(),
    //);
    //let outf = out.as_str();
    let file = cli.file.clone();
    let outf = file.parent().unwrap();
    let outf_str = outf.to_str().unwrap();

    let data_path = cli.file;

    let file = File::open(data_path).unwrap();
    let buf_reader = BufReader::new(file);

    let vis_data = serde_json::from_reader(buf_reader).unwrap();

    let render_dir = render_vis(&vis_data, outf_str);
    vis_dev_server(render_dir);
}
