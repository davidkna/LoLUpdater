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
            Arg::with_name("uninstall")
                .help("Uninstall patches instead of installing them")
                .long("uninstall")
                .short("u"),
        )
        .arg(
            Arg::with_name("PATH")
                .help("Target League of Legends patch")
                .index(1),
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

    let lol_dir = matches.value_of("PATH").unwrap_or(DEFAULT_LOL_DIR);

    match matches.is_present("uninstall") {
        false => install(&lol_dir),
        true => uninstall(&lol_dir),
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
