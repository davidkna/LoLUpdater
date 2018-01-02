#[cfg(not(target_os = "macos"))]
use std::env;
use std::io;
use std::num;
use std::path;

use app_dirs;
use reqwest;
use serde_json;
#[cfg(target_os = "macos")]
use walkdir;

#[derive(Debug, ErrorChain)]
pub enum ErrorKind {
    Msg(String),

    #[error_chain(foreign)] AppDirs(app_dirs::AppDirsError),

    #[cfg(not(target_os = "macos"))]
    #[error_chain(foreign)]
    EnvVar(env::VarError),

    #[error_chain(foreign)] Io(io::Error),

    #[error_chain(foreign)] Parse(num::ParseIntError),

    #[error_chain(foreign)] Prefix(path::StripPrefixError),

    #[error_chain(foreign)] Reqwest(reqwest::Error),

    #[error_chain(foreign)] SerdeJSON(serde_json::Error),

    #[cfg(target_os = "macos")]
    #[error_chain(foreign)]
    WalkDir(walkdir::Error),
}
