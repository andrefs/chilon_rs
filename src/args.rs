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
    fn test_no_infer_ns_flag() {
        let args = vec!["chilon_rs", "--no-infer-ns", "file.ttl"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(!cli.infer_ns);
    }

    #[test]
    fn test_ignore_unknown_flag() {
        let args = vec!["chilon_rs", "--ignore-unknown", "file.ttl"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(cli.ignore_unknown);
    }

    #[test]
    fn test_subcommand_test() {
        let args = vec!["chilon_rs", "test", "--list"];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.commands {
            Some(Commands::Test { list }) => assert!(list),
            _ => panic!("Expected Test subcommand"),
        }
    }
}
