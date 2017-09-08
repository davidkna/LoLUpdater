use std::fs::File;
use std::fs;
use std::path::{Path, PathBuf};

use app_dirs::{self, AppDataType};
use xz2::read::XzDecoder;
use serde_json;
use tar::Archive;

use util::*;

#[derive(Deserialize)]
struct Manifest {
    cef_version: String,
}

const LOL_LC_PATH: [&str; 2] = [
    "Contents/LoL/RADS/projects/league_client/releases",
    "deploy/League of Legends.app/Contents/Frameworks",
];

const LOL_LP_PATH: [&str; 2] = [
    "Contents/LoL/RADS/projects/lol_patcher/releases",
    "deploy/LoLPatcherUx.app/Contents/Frameworks",
];

const DOWNLOAD_URL: &str = "https://lolupdater.com/downloads/lol4mac/lolupdater-mac-cef.tar.xz";
const MANIFEST_URL: &str = "https://lolupdater.com/downloads/lol4mac/Manifest.JSON";

pub fn install() -> Result<()> {
    info!("Backing up CEF…");
    backup_cef().chain_err(|| "Failed to backup CEF")?;

    let cef_dir = app_dirs::get_app_dir(
        AppDataType::UserCache,
        &APP_INFO,
        "Chromium Embedded Framework.framework",
    )?;
    let manifest_file_path =
        app_dirs::get_app_dir(AppDataType::UserConfig, &APP_INFO, "Manifest.JSON")?;
    if !cef_dir.exists() {
        info!("CEF not cached.");
        download(&manifest_file_path, MANIFEST_URL, None)?;
        download_cef()?;
    } else {
        info!("CEF cached. Checking manifest…");
        let cf_file = File::open(&manifest_file_path)?;
        let current_manifest: Manifest = serde_json::from_reader(cf_file)?;
        download(&manifest_file_path, MANIFEST_URL, None)?;
        let of_file = File::open(&manifest_file_path)?;
        let online_manifest: Manifest = serde_json::from_reader(of_file)?;
        if online_manifest.cef_version != current_manifest.cef_version {
            info!("CEF is already cached but needs updating!");
            fs::remove_dir_all(&cef_dir)?;
            download_cef()?;
        } else {
            info!("CEF cache is up to date!");
        }

    }

    info!("Updating CEF…\n");
    update_cef(&cef_dir).chain_err(|| "Failed to update CEF")?;
    Ok(())
}

pub fn remove() -> Result<()> {
    let cef_backup_path = app_dirs::get_app_dir(
        AppDataType::UserData,
        &APP_INFO,
        "Backups/Chromium Embedded Framework.framework",
    )?;
    if !cef_backup_path.exists() {
        return Err("No CEF backup found!".into());
    }
    info!("Restoring CEF…");
    update_cef(&cef_backup_path)?;
    fs::remove_dir_all(&cef_backup_path)?;
    info!("Removing CEF backup…");
    let cef_cache_path = app_dirs::get_app_dir(
        AppDataType::UserCache,
        &APP_INFO,
        "Chromium Embedded Framework.framework",
    )?;

    if cef_cache_path.exists() {
        info!("Removing CEF download cache…");
        fs::remove_dir_all(cef_cache_path)?;
    }

    info!("Removing CEF download cache manifest…");
    let manifest_file_path =
        app_dirs::get_app_dir(AppDataType::UserConfig, &APP_INFO, "Manifest.JSON")?;
    if manifest_file_path.exists() {
        fs::remove_file(manifest_file_path)?;
    }
    Ok(())
}

fn download_cef() -> Result<()> {
    info!("Downloading CEF…");
    let cef_dl = new_request(DOWNLOAD_URL, false)?;
    let decompressed = XzDecoder::new(cef_dl);
    let mut archive = Archive::new(decompressed);

    let target_dir = app_dirs::get_app_dir(
        AppDataType::UserCache,
        &APP_INFO,
        "Chromium Embedded Framework.framework",
    )?;
    for file in archive.entries()? {
        let mut file = file?;
        let path = file.path()?.into_owned();

        let target = target_dir.join(path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        file.unpack(target)?;

    }
    Ok(())
}

fn backup_cef() -> Result<()> {
    let lol_lc_path = join_version(
        &PathBuf::from(LOL_LC_PATH[0]),
        &PathBuf::from(LOL_LC_PATH[1]),
    )?
        .join("Chromium Embedded Framework.framework");

    let cef_backup = app_dirs::get_app_dir(
        AppDataType::UserData,
        &APP_INFO,
        "Backups/Chromium Embedded Framework.framework",
    )?;
    if cef_backup.exists() {
        info!("Skipping CEF backup! (Already exists)");
    } else {
        update_dir(&lol_lc_path, &cef_backup)?;
    }
    Ok(())
}

fn update_cef(cef_dir: &Path) -> Result<()> {
    let lol_lc_path = join_version(
        &PathBuf::from(LOL_LC_PATH[0]),
        &PathBuf::from(LOL_LC_PATH[1]),
    )?
        .join("Chromium Embedded Framework.framework");
    update_dir(cef_dir, &lol_lc_path)?;

    let lol_lp_path = join_version(
        &PathBuf::from(LOL_LP_PATH[0]),
        &PathBuf::from(LOL_LP_PATH[1]),
    )?
        .join("Chromium Embedded Framework.framework");
    update_dir(cef_dir, &lol_lp_path)?;
    Ok(())
}
