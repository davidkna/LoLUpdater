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
    env::set_current_dir(lol_dir).chain_err(
        || "Failed to set CWD to LoL location",
    )?;

    init_backups()?;


    match mode.as_ref() {
        "install" => install(),
        "uninstall" => uninstall(),
        _ => panic!("Unkown mode!"),
    }
}

fn install() -> Result<()> {
    cg::install()?;
    println!("Done installing!");
    Ok(())
}

fn uninstall() -> Result<()> {
    cg::remove().chain_err(|| "Failed to uninstall Cg")?;

    println!("Done uninstalling!");
    Ok(())
}
