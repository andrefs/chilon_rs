use bzip2::bufread::BzDecoder;
use flate2::bufread::GzDecoder;
use log::debug;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, Error};
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub enum ReaderWrapper {
    Plain(BufReader<File>),
    Gz(BufReader<GzDecoder<BufReader<File>>>),
    Bz2(BufReader<BzDecoder<BufReader<File>>>),
}

impl Read for ReaderWrapper {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        match self {
            ReaderWrapper::Plain(f) => f.read(buf),
            ReaderWrapper::Gz(f) => f.read(buf),
            ReaderWrapper::Bz2(f) => f.read(buf),
        }
    }
}

impl BufRead for ReaderWrapper {
    fn consume(&mut self, amt: usize) {
        match self {
            ReaderWrapper::Plain(f) => f.consume(amt),
            ReaderWrapper::Gz(f) => f.consume(amt),
            ReaderWrapper::Bz2(f) => f.consume(amt),
        }
    }

    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        match self {
            ReaderWrapper::Plain(f) => f.fill_buf(),
            ReaderWrapper::Gz(f) => f.fill_buf(),
            ReaderWrapper::Bz2(f) => f.fill_buf(),
        }
    }
}

pub fn extract(path: &PathBuf) -> (ReaderWrapper, &OsStr) {
    let file = File::open(path)
        .unwrap_or_else(|e| panic!("Could not open file {}: {e}", path.to_string_lossy()));
    let buf_reader = BufReader::new(file);

    let extension = path.extension();
    let file_stem = path.file_stem();

    if extension.is_some() && extension.unwrap() == "bz2" {
        debug!("extracting bz2 file {:?}", path);
        let stream = ReaderWrapper::Bz2(BufReader::new(BzDecoder::new(buf_reader)));
        return (stream, file_stem.unwrap_or(path.as_os_str()));
    }
    if extension.is_some() && extension.unwrap() == "gz" {
        debug!("extracting gz file {:?}", path);
        let stream = ReaderWrapper::Gz(BufReader::new(GzDecoder::new(buf_reader)));
        return (stream, file_stem.unwrap_or(path.as_os_str()));
    } else {
        debug!("extracting plain file {:?}", path);
        let stream = ReaderWrapper::Plain(buf_reader);
        return (stream, path.as_os_str());
    }
}
