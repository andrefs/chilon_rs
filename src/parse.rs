use rio_turtle::TurtleParser;

use crate::extract::extract;
use std::{io::BufRead, path::PathBuf};

pub fn parse(path: &PathBuf) -> TurtleParser<impl BufRead> {
    let stream = extract(&path);
    let parser = TurtleParser::new(stream, None);
    return parser;
}
