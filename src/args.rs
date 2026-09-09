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

    /// Infer namespaces
    #[arg(long = "no-infer-ns", action = clap::ArgAction::SetFalse, default_value_t = true)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_basic_parsing() {
        let args = vec!["chilon_rs", "file1.ttl", "file2.ttl"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.files.len(), 2);
        assert_eq!(cli.files[0], PathBuf::from("file1.ttl"));
        assert_eq!(cli.files[1], PathBuf::from("file2.ttl"));
        assert!(cli.infer_ns); // default true
        assert!(!cli.ignore_unknown); // default false
    }

    #[test]
    fn test_flags() {
        let args = vec!["chilon_rs", "--no-infer-ns", "--ignore-unknown", "file.ttl"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(!cli.infer_ns);
        assert!(cli.ignore_unknown);
    }

    #[test]
    fn test_flags_combined() {
        let args = vec!["chilon_rs", "--no-infer-ns", "file1.ttl", "file2.ttl"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(!cli.infer_ns);
        assert!(!cli.ignore_unknown);
        assert_eq!(cli.files.len(), 2);
    }

    #[test]
    fn test_defaults_when_no_flags() {
        let args = vec!["chilon_rs", "single.ttl"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(cli.infer_ns);
        assert!(!cli.ignore_unknown);
        assert_eq!(cli.files.len(), 1);
        assert_eq!(cli.files[0], PathBuf::from("single.ttl"));
    }
}
