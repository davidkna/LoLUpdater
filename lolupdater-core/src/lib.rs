extern crate app_dirs;
#[macro_use]
extern crate error_chain;
#[cfg(target_os = "macos")]
extern crate flate2;
#[cfg(target_os = "macos")]
extern crate plist;
extern crate regex;
extern crate reqwest;
extern crate ring;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[cfg(target_os = "macos")]
extern crate tar;
extern crate tempdir;
#[cfg(target_os = "macos")]
extern crate walkdir;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[cfg(target_os = "macos")]
pub mod cg;
pub mod errors;
pub mod util;

use std::env;
use app_dirs::AppDataType;
use plist::serde::deserialize;
use std::fs;
use util::*;

pub const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));

pub fn init_backups() -> Result<()> {
    let backups = {
        let mut t = app_dirs::app_root(AppDataType::UserData, &APP_INFO)
            .chain_err(|| "Create data root")?;
        t.push("Backups");
        t
    };

    if !backups.exists() {
        fs::create_dir(backups).chain_err(|| "Create backup dir")?;
    }
    Ok(())
}

pub fn install(lol_dir: &str) -> Result<()> {
    set_lol_dir(lol_dir)?;
    init_backups()?;
    cg::install()?;

    info!("Done installing!");
    Ok(())
}

pub fn uninstall(lol_dir: &str) -> Result<()> {
    set_lol_dir(lol_dir)?;
    cg::remove().chain_err(|| "Failed to uninstall Cg")?;

    info!("Done uninstalling!");
    Ok(())
}

#[cfg(target_os = "macos")]
#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Info {
    CFBundleIdentifier: String,
}

fn set_lol_dir(lol_dir: &str) -> Result<()> {
    env::set_current_dir(lol_dir).chain_err(
        || "Failed to set CWD to LoL location. Did you set the correct path for LoL?",
    )?;
    if cfg!(target_os = "macos") {
        let info_plist = std::fs::File::open("Contents/Info.plist").chain_err(
            || "Failed to find Info.plist. Is this an app bundle?",
        )?;
        let info: Info = deserialize(info_plist).chain_err(
            || "Could not parse Info.plist",
        )?;
        if info.CFBundleIdentifier != "com.riotgames.MacContainer" {
            return Err(
                "The chosen app bundle is not LoL. Please check again!".into(),
            );
        }
    }
    Ok(())
}

#[derive(Deserialize, Debug)]
struct GithubRelease {
    tag_name: String,
}


pub fn update_available() -> Result<bool> {
    info!("Checking for updates…");
    let release_dl = reqwest::get(
        "https://api.github.com/repos/LoLUpdater/LoLUpdater-macOS/releases/latest",
    )?;

    let git_release: GithubRelease = serde_json::from_reader(release_dl)?;

    Ok(git_release.tag_name != VERSION)
}
