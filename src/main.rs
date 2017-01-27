
extern crate app_dirs;
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

use app_dirs::AppDataType;
use util::*;

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

    let backups = {
        let mut t = app_dirs::app_root(AppDataType::UserData, &APP_INFO).expect("Create data root");
        t.push("Backups");
        t
    };

    if Path::new("Backups").exists() {
        fs::rename("Backups", backups).expect("Move backups to new location");
    } else if !backups.exists() {
        fs::create_dir(backups).expect("Create backup dir");
    }


    match mode.as_ref() {
        "install" => install(),
        "uninstall" => uninstall(),
        _ => panic!("Unkown mode!"),
    }
}

#[cfg(target_os = "macos")]
fn install() {
    let air_handle = {
        if Path::new("Contents/LoL/RADS/projects/lol_air_client").exists() {
            let handle = thread::Builder::new()
                .name("air_thread".to_string())
                .spawn(|| { air::install(); })
                .unwrap();
            Some(handle)
        } else {
            println!("Skipping Adobe Air update because missing in new client!");
            None
        }
    };

    let cg_handle = thread::Builder::new()
        .name("cg_thread".to_string())
        .spawn(|| { cg::install(); })
        .unwrap();

    if let Some(handle) = air_handle {
        let air_result = handle.join();
        if air_result.is_ok() {
            println!("Adobe Air was updated!");
        } else {
            println!("Failed to update Adobe Air!");
        }
    }

    let cg_result = cg_handle.join();
    if cg_result.is_ok() {
        println!("Cg was updated!");
    } else {
        println!("Failed to update Cg!");
    }
    println!("Done installing!");
}

#[cfg(not(target_os = "macos"))]
fn install() {
    let cg_update = thread::Builder::new()
        .name("cg_thread".to_string())
        .spawn(|| { cg::install(); })
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
