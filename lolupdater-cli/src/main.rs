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


quick_main!(run);

fn run() -> Result<()> {
    println!("LoLUpdater for macOS {}", VERSION);
    println!(
        "Report errors, feature requests or any issues at \
              https://github.com/LoLUpdater/LoLUpdater-macOS/issues.\n"
    );

    init_log();

    if update_available()? {
        println!(
            "A new update is available.\nPlease download it from https://github.com/LoLUpdater/LoLUpdater-macOS/releases/latest to use LoLUpdater."
        );
        ::std::process::exit(1);
    }

    let mode = env::args().nth(1).unwrap_or_else(|| "install".to_string());
    let lol_dir = env::args().nth(2).unwrap_or_else(|| DEFAULT_LOL_DIR.into());

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
