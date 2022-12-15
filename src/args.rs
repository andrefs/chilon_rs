use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = None
    )]
pub struct Cli {
    #[arg(required = true, value_name = "RDF_FILE(S)")]
    pub files: Vec<PathBuf>,

    ///// Sets a custom config file
    //#[arg(short, long, value_name = "FILE")]
    //pub config: Option<PathBuf>,
    //
    /// Infer namespaces
    #[arg(short, long, default_value_t = true, action = clap::ArgAction::Set)]
    pub infer_ns: bool,

    /// Ignore triples with unknown namespaces
    #[arg(short, long, default_value_t = false)]
    pub ignore_unknown: bool,
    //
    ///// Turn debugging information on
    //#[arg(short, long, action = clap::ArgAction::Count)]
    //pub debug: u8,
}

#[derive(Subcommand)]
pub enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}
