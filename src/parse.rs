use rio_api::parser::TriplesParser;
use rio_turtle::TurtleParser;

use crate::extract::extract;
use std::path::PathBuf;

pub fn parse(path: &PathBuf) -> impl TriplesParser {
    let stream = extract(&path);
    let parser = TurtleParser::new(stream, None);
    return parser;
}
