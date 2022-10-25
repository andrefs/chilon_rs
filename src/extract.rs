use bzip2::bufread::BzDecoder;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn extract(path: &PathBuf) -> impl BufRead {
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let dec = BzDecoder::new(buf_reader);
    let stream = BufReader::new(dec);
    return stream;
}
