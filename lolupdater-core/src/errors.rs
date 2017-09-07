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

error_chain! {
    foreign_links {
        AppDirs(app_dirs::AppDirsError);
        EnvVar(env::VarError) #[cfg(not(target_os = "macos"))];
        Io(io::Error);
        Parse(num::ParseIntError);
        Prefix(path::StripPrefixError);
        Reqwest(reqwest::Error);
        SerdeJSON(serde_json::Error);
        WalkDir(walkdir::Error) #[cfg(target_os = "macos")];
    }
}
