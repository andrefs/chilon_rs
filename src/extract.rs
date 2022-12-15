use bzip2::bufread::BzDecoder;
use log::debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn extract(path: &PathBuf) -> impl BufRead {
    let file = File::open(path)
        .unwrap_or_else(|e| panic!("Could not open file {}: {e}", path.to_string_lossy()));
    let buf_reader = BufReader::new(file);
    debug!("extracting {:?}", path);
    let dec = BzDecoder::new(buf_reader);
    let stream = BufReader::new(dec);
    return stream;
}

// pub fn compress(stream, path) {}
