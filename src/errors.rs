#[cfg(not(macos))]
use std::env;
use std::io;
use std::num;
use std::path;

use reqwest;
#[cfg(macos)]
use walkdir;

#[derive(Debug)]
#[cfg(macos)]
pub enum LoLUpdaterError {
    Io(io::Error),
    Parse(num::ParseIntError),
    Prefix(path::StripPrefixError),
    Reqwest(reqwest::Error),
    WalkDir(walkdir::Error),
}

#[derive(Debug)]
#[cfg(not(macos))]
pub enum LoLUpdaterError {
    EnvVar(env::VarError),
    Io(io::Error),
    Parse(num::ParseIntError),
    Prefix(path::StripPrefixError),
    Reqwest(reqwest::Error),
}

#[cfg(not(macos))]
impl From<env::VarError> for LoLUpdaterError {
    fn from(err: env::VarError) -> LoLUpdaterError {
        LoLUpdaterError::EnvVar(err)
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

impl From<reqwest::Error> for LoLUpdaterError {
    fn from(err: reqwest::Error) -> LoLUpdaterError {
        LoLUpdaterError::Reqwest(err)
    }
}

#[cfg(macos)]
impl From<walkdir::Error> for LoLUpdaterError {
    fn from(err: walkdir::Error) -> LoLUpdaterError {
        LoLUpdaterError::WalkDir(err)
    }
}
