#![feature(plugin)]

#![plugin(clippy)]

extern crate flate2;
extern crate hyper;
extern crate regex;
extern crate ring;
extern crate tar;
extern crate tempdir;
extern crate walkdir;

#[macro_use]
extern crate lazy_static;


use std::env;
use std::fs;
use std::path::Path;
use std::thread;

mod air;
mod cg;
mod errors;
mod help;

fn main() {
    println!("LoLUpdater for macOS 3.0.0");
    println!("Report errors, feature requests or any issues at \
              https://github.com/LoLUpdater/LoLUpdater-macOS/issues.");

    let mode = env::args().nth(1).unwrap_or("install".to_string());
    let lol_dir = env::args().nth(2).unwrap_or("/Applications/League of Legends.app".to_string());
    env::set_current_dir(lol_dir).expect("Failed to set CWD to LoL location");

    match mode.as_ref() {
        "install" => install(),
        "uninstall" => uninstall(),
        _ => panic!("Unkown mode!"),
    }
}


fn install() {
    if !Path::new("Backups").exists() {
        fs::create_dir("Backups").expect("Create Backup dir");
    }

    let air_update = thread::Builder::new()
        .name("air_thread".to_string())
        .spawn(|| {
            air::install();
        })
        .unwrap();

    let cg_update = thread::Builder::new()
        .name("cg_thread".to_string())
        .spawn(|| {
            cg::install();
        })
        .unwrap();

    let air_result = air_update.join();
    if air_result.is_ok() {
        println!("Adobe Air was updated!");
    } else {
        println!("Failed to update Adobe Air!");
    }

    let cg_result = cg_update.join();
    if cg_result.is_ok() {
        println!("Cg was updated!");
    } else {
        println!("Failed to update Cg!");
    }
    println!("Done installing!");
}

fn uninstall() {
    air::remove().expect("Failed to uninstall Adobe Air");

    cg::remove().expect("Failed to uninstall Cg");

    println!("Done uninstalling!");
}
