
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

use std::env;
use std::fs;
use std::path::Path;
use std::thread;

#[cfg(target_os = "macos")]
mod air;
mod cg;
mod errors;
mod util;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("LoLUpdater for macOS {}", VERSION);
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

#[cfg(target_os = "macos")]
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

#[cfg(not(target_os = "macos"))]
fn install() {
    if !Path::new("Backups").exists() {
        fs::create_dir("Backups").expect("Create Backup dir");
    }

    let cg_update = thread::Builder::new()
        .name("cg_thread".to_string())
        .spawn(|| {
            cg::install();
        })
        .unwrap();

    let cg_result = cg_update.join();
    if cg_result.is_ok() {
        println!("Cg was updated!");
    } else {
        println!("Failed to update Cg!");
    }
    println!("Done installing!");
}

#[cfg(target_os = "macos")]
fn uninstall() {
    air::remove().expect("Failed to uninstall Adobe Air");

    cg::remove().expect("Failed to uninstall Cg");

    println!("Done uninstalling!");
}

#[cfg(not(target_os = "macos"))]
fn uninstall() {
    cg::remove().expect("Failed to uninstall Cg");

    println!("Done uninstalling!");
}
