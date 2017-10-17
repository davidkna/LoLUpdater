extern crate clap;
extern crate env_logger;
extern crate log;

extern crate lolupdater_core;

#[macro_use]
extern crate error_chain;

use std::env;

use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;

use lolupdater_core::*;
use errors::*;
use clap::{Arg, App};

quick_main!(run);

fn run() -> Result<()> {
    init_log();
    let matches = App::new("LoLUpdater for macOS")
        .version(VERSION)
        .arg(
            Arg::with_name("MODE")
                .help("Whether to install or remove LoLUpdater patches")
                .index(1)
                .possible_values(&["install", "uninstall"]),
        )
        .arg(
            Arg::with_name("PATH")
                .help("Target League of Legends patch")
                .index(2),
        )
        .get_matches();

    println!("LoLUpdater for macOS {}", VERSION);
    println!(
        "Report errors, feature requests or any issues at \
              https://github.com/MOBASuite/LoLUpdater-macOS/issues.\n"
    );

    if update_available()? {
        return Err(
            "A new update is available.\nPlease download it from https://github.com/MOBASuite/LoLUpdater-macOS/releases/latest to use LoLUpdater.".into()
        );
    }

    let lol_dir = matches.value_of("INPUT").unwrap_or(DEFAULT_LOL_DIR);
    let mode = matches.value_of("PATH").unwrap_or("install");

    match mode.as_ref() {
        "install" => install(&lol_dir),
        "uninstall" => uninstall(&lol_dir),
        _ => panic!("Unknown mode!"),
    }
}

fn init_log() {
    let format = |record: &LogRecord| format!("[{}] {}", record.level(), record.args());

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, LogLevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.init().unwrap();
}
