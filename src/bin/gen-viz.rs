use chilon_rs::{util::gen_file_name, visualization::render_vis};
use chrono::Utc;
use log::info;
use simple_logger::SimpleLogger;

use clap::Parser;
use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

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
    SimpleLogger::new().init().unwrap();

    let out = gen_file_name(
        format!("tmp/{}", Utc::now().format("%Y%m%d")),
        "".to_string(),
    );
    let outf = out.as_str();

    info!("Creating folder {outf} to store results");
    fs::create_dir(outf).unwrap();

    let data_path = cli.file;

    let file = File::open(data_path).unwrap();
    let buf_reader = BufReader::new(file);

    let vis_data = serde_json::from_reader(buf_reader).unwrap();

    render_vis(&vis_data, outf);
}
