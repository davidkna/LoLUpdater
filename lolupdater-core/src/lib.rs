
extern crate app_dirs;
#[macro_use]
extern crate error_chain;
#[cfg(target_os = "macos")]
extern crate flate2;
extern crate regex;
extern crate reqwest;
extern crate ring;
#[cfg(target_os = "macos")]
extern crate tar;
extern crate tempdir;
#[cfg(target_os = "macos")]
extern crate walkdir;

#[macro_use]
extern crate lazy_static;

#[cfg(target_os = "macos")]
pub mod air;
pub mod cg;
pub mod errors;
pub mod util;