
extern crate app_dirs;
#[macro_use]
extern crate error_chain;
#[cfg(target_os = "macos")]
extern crate libflate;
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

use app_dirs::AppDataType;
use util::*;

#[cfg(target_os = "macos")]
mod air;
mod cg;
mod errors;
mod util;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

quick_main!(run);

fn run() -> Result<()> {
    println!("LoLUpdater for macOS {}", VERSION);
    println!("Report errors, feature requests or any issues at \
              https://github.com/LoLUpdater/LoLUpdater-macOS/issues.");

    let mode = env::args().nth(1).unwrap_or("install".to_string());
    let lol_dir = env::args()
        .nth(2)
        .unwrap_or("/Applications/League of Legends.app".to_string());
    env::set_current_dir(lol_dir)
        .chain_err(|| "Failed to set CWD to LoL location")?;

    let backups = {
        let mut t = app_dirs::app_root(AppDataType::UserData, &APP_INFO)
            .chain_err(|| "Create data root")?;
        t.push("Backups");
        t
    };

    if Path::new("Backups").exists() {
        fs::rename("Backups", backups)
            .chain_err(|| "Move backups to new location")?;
    } else if !backups.exists() {
        fs::create_dir(backups)
            .chain_err(|| "Create backup dir")?;
    }


    match mode.as_ref() {
        "install" => install(),
        "uninstall" => uninstall(),
        _ => panic!("Unkown mode!"),
    }
}

#[cfg(target_os = "macos")]
fn install() -> Result<()> {
    if Path::new("Contents/LoL/RADS/projects/lol_air_client").exists() {
        air::install()?;
    } else {
        println!("Skipping Adobe Air update because it's missing in the modern client!");
    }

    cg::install()?;

    println!("Done installing!");
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn install() -> Result<()> {
    cg::install()?;
    println!("Done installing!");
    Ok(())
}

#[cfg(target_os = "macos")]
fn uninstall() -> Result<()> {
    air::remove()
        .chain_err(|| "Failed to uninstall Adobe Air")?;

    cg::remove().chain_err(|| "Failed to uninstall Cg")?;

    println!("Done uninstalling!");
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn uninstall() -> Result<()> {
    cg::remove().chain_err(|| "Failed to uninstall Cg")?;

    println!("Done uninstalling!");
    Ok(())
}
