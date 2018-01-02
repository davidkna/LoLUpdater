extern crate clap;
extern crate env_logger;
extern crate log;

extern crate lolupdater_core;

#[macro_use]
extern crate error_chain;

use std::env;
use std::io::Write;
use log::LevelFilter;
use env_logger::Builder;

use lolupdater_core::*;
use errors::*;
use clap::{App, Arg};

quick_main!(run);

fn run() -> Result<()> {
    init_log();
    let matches = App::new("LoLUpdater")
        .version(VERSION)
        .arg(
            Arg::with_name("uninstall")
                .help("Uninstall patches instead of installing them")
                .long("uninstall")
                .short("u"),
        )
        .arg(
            Arg::with_name("PATH")
                .help(&format!(
                    "Target League of Legends patch. Default is \"{}\".",
                    DEFAULT_LOL_DIR
                ))
                .index(1),
        )
        .get_matches();

    println!("LoLUpdater {}", VERSION);
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
    let mut builder = Builder::new();
 
    builder.format(|buf, record| writeln!(buf, "[{}] {}", record.level(), record.args()))
           .filter(None, LevelFilter::Info);

    if let Ok(rust_log) = env::var("RUST_LOG") {
       builder.parse(&rust_log);
    }

    builder.init();
}
