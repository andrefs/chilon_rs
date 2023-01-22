use chilon_rs::parse::parse;
use rio_api::parser::TriplesParser;
use rio_turtle::TurtleError;

use clap::Parser;
use log::{debug, error, info};
use simplelog::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = None
    )]
pub struct Cli {
    #[arg(required = true, value_name = "RDF_FILE")]
    pub files: Vec<PathBuf>,
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

    for path in cli.files {
        info!("Checking file {}", path.to_string_lossy());
        let mut graph = parse(&path);

        let mut i = 0;
        while !graph.is_end() {
            i += 1;
            if i % 1_000_000 == 0 {
                debug!("Read {} triples so far", i);
            }
            graph
                .parse_step(&mut |t| {
                    println!("{}", t.to_string());
                    Ok(())
                })
                .unwrap_or_else(|err: TurtleError| {
                    error!("Error testing file {}: {}", path.to_string_lossy(), err);
                });
        }

        info!("File {} seems ok.", path.to_string_lossy());
    }
}
