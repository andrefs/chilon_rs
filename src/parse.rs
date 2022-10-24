use rio_turtle::TurtleParser;

use crate::extract::extract;
use bzip2::bufread::BzDecoder;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn parse(path: &PathBuf) -> TurtleParser<BufReader<BzDecoder<BufReader<File>>>> {
    let stream = extract(&path);
    let parser = TurtleParser::new(stream, None);
    return parser;
}
