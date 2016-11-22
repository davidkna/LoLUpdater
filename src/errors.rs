use std::io;
use std::num;
use std::path;

use hyper;
use walkdir;

#[derive(Debug)]
pub enum LoLUpdaterError {
    Hyper(hyper::Error),
    Io(io::Error),
    Parse(num::ParseIntError),
    Prefix(path::StripPrefixError),
    WalkDir(walkdir::Error),
}

impl From<hyper::Error> for LoLUpdaterError {
    fn from(err: hyper::Error) -> LoLUpdaterError {
        LoLUpdaterError::Hyper(err)
    }
}

impl From<io::Error> for LoLUpdaterError {
    fn from(err: io::Error) -> LoLUpdaterError {
        LoLUpdaterError::Io(err)
    }
}

impl From<num::ParseIntError> for LoLUpdaterError {
    fn from(err: num::ParseIntError) -> LoLUpdaterError {
        LoLUpdaterError::Parse(err)
    }
}

impl From<path::StripPrefixError> for LoLUpdaterError {
    fn from(err: path::StripPrefixError) -> LoLUpdaterError {
        LoLUpdaterError::Prefix(err)
    }
}

impl From<walkdir::Error> for LoLUpdaterError {
    fn from(err: walkdir::Error) -> LoLUpdaterError {
        LoLUpdaterError::WalkDir(err)
    }
}
