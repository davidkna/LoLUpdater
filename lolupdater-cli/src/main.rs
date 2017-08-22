#![cfg_attr(feature="clippy", feature(plugin))]

#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate app_dirs;
#[macro_use]
extern crate error_chain;

extern crate lolupdater_core;

use std::env;

use lolupdater_core::*;
use errors::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

quick_main!(run);

fn run() -> Result<()> {
    println!("LoLUpdater for macOS v{}", VERSION);
    println!(
        "Report errors, feature requests or any issues at \
              https://github.com/LoLUpdater/LoLUpdater-macOS/issues."
    );
    println!("");

    let mode = env::args().nth(1).unwrap_or_else(|| "install".to_string());
    let lol_dir = env::args().nth(2).unwrap_or_else(|| {
        "/Applications/League of Legends.app".to_string()
    });

    match mode.as_ref() {
        "install" => install(&lol_dir),
        "uninstall" => uninstall(&lol_dir),
        _ => panic!("Unkown mode!"),
    }
}
