use rio_api::parser::TriplesParser;
use rio_turtle::{NTriplesParser, TurtleError, TurtleParser};

use crate::extract::{extract, ReaderWrapper};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub struct NTWrapper {
    prefixes: HashMap<String, String>,
    parser: NTriplesParser<ReaderWrapper>,
}
pub enum ParserWrapper {
    Turtle(TurtleParser<ReaderWrapper>),
    NTriples(NTWrapper),
}

impl TriplesParser for ParserWrapper {
    type Error = TurtleError;
    fn is_end(&self) -> bool {
        match &self {
            &ParserWrapper::NTriples(w) => w.parser.is_end(),
            &ParserWrapper::Turtle(p) => p.is_end(),
        }
    }

    fn parse_step<E: From<Self::Error>>(
        &mut self,
        on_triple: &mut impl FnMut(rio_api::model::Triple<'_>) -> Result<(), E>,
    ) -> Result<(), E> {
        match self {
            ParserWrapper::NTriples(p) => p.parser.parse_step(on_triple),
            ParserWrapper::Turtle(p) => p.parse_step(on_triple),
        }
    }
}

impl ParserWrapper {
    pub fn prefixes(&self) -> &HashMap<String, String> {
        match self {
            ParserWrapper::Turtle(p) => p.prefixes(),
            ParserWrapper::NTriples(w) => &w.prefixes,
        }
    }
}

pub fn parse(path: &PathBuf) -> ParserWrapper {
    let (stream, file_stem) = extract(&path);
    let path_stem = Path::new(file_stem);
    let ext = path_stem.extension();

    if ext.is_some() && ext.unwrap() == "nt" {
        let parser = NTriplesParser::new(stream);
        return ParserWrapper::NTriples(NTWrapper {
            prefixes: Default::default(),
            parser,
        });
    } else {
        let parser = TurtleParser::new(stream, None);
        return ParserWrapper::Turtle(parser);
    }
}
