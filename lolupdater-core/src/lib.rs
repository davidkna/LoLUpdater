extern crate app_dirs;
#[macro_use]
extern crate error_chain;
#[cfg(target_os = "macos")]
extern crate flate2;
#[cfg(target_os = "macos")]
extern crate plist;
extern crate regex;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;
#[cfg(target_os = "macos")]
extern crate tar;
#[cfg(target_os = "macos")]
extern crate tempdir;
#[cfg(target_os = "macos")]
extern crate walkdir;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

mod cg;

pub mod errors;
pub mod util;

use std::env;
#[cfg(not(target_os = "macos"))]
use std::path::Path;
use app_dirs::AppDataType;
#[cfg(target_os = "macos")]
use plist::serde::deserialize;
use std::fs;
use util::*;

pub const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));

#[cfg(target_os = "macos")]
pub const DEFAULT_LOL_DIR: &str = "/Applications/League of Legends.app";

#[cfg(not(target_os = "macos"))]
pub const DEFAULT_LOL_DIR: &str = "C:/Riot Games/League of Legends";

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
    cg::install().chain_err(|| "Failed to update Cg")?;

    info!("Done installing!");
    Ok(())
}


pub fn uninstall(lol_dir: &str) -> Result<()> {
    set_lol_dir(lol_dir)?;
    cg::remove().chain_err(|| "Failed to restore Cg")?;

    info!("Done uninstalling!");
    Ok(())
}

fn set_lol_dir(lol_dir: &str) -> Result<()> {
    env::set_current_dir(lol_dir).chain_err(
        || "Failed to set CWD to LoL location. Did you set the correct path for LoL?",
    )?;
    lol_dir_ok()
}

#[cfg(target_os = "macos")]
#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Info {
    CFBundleIdentifier: String,
}

#[cfg(target_os = "macos")]
fn lol_dir_ok() -> Result<()> {
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
    Ok(())
}
#[cfg(not(target_os = "macos"))]
fn lol_dir_ok() -> Result<()> {
    if !Path::new("LeagueClient.exe").exists() {
        return Err(
            "The chosen app folder is not LoL. Please check again!".into(),
        );
    }
    Ok(())
}

#[derive(Deserialize, Debug)]
struct GithubRelease {
    tag_name: String,
}


pub fn update_available() -> Result<bool> {
    info!("Checking for updates…");
    if cfg!(debug_assertions) {
        return Ok(false);
    }
    let release_dl = reqwest::get(
        "https://api.github.com/repos/MOBASuite/LoLUpdater-macOS/releases/latest",
    )?;

    let git_release: GithubRelease = serde_json::from_reader(release_dl)?;

    Ok(git_release.tag_name != VERSION)
}
