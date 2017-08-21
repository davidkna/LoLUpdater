#![cfg_attr(feature="clippy", feature(plugin))]

#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate app_dirs;
#[macro_use]
extern crate error_chain;
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

#[cfg(target_os = "macos")]
pub mod cg;
pub mod errors;
pub mod util;

use std::path::Path;
use app_dirs::AppDataType;
use std::fs;
use util::*;

pub fn init_backups() -> Result<()> {
    let air_backup_path = app_dirs::get_app_dir(
        AppDataType::UserData,
        &APP_INFO,
        "Backups/Adobe AIR.framework",
    )?;
    if air_backup_path.exists() {
        println!("Removing obsolete Air backup!");
        fs::remove_dir_all(air_backup_path)?;
    }

    let backups = {
        let mut t = app_dirs::app_root(AppDataType::UserData, &APP_INFO)
            .chain_err(|| "Create data rootg")?;
        t.push("Backups");
        t
    };

    if Path::new("Backups").exists() {
        fs::rename("Backups", backups).chain_err(
            || "Move backups to new location",
        )?;
    } else if !backups.exists() {
        fs::create_dir(backups).chain_err(|| "Create backup dir")?;
    }
    Ok(())
}
