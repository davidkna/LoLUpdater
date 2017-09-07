use std::fs::File;
use std::fs;
use std::path::{Path, PathBuf};

use app_dirs::{self, AppDataType};
use flate2::read::GzDecoder;
use tempdir::TempDir;
use tar::Archive;

use util::*;


const LOL_CL_PATH: [&str; 2] = [
    "Contents/LoL/RADS/solutions/lol_game_client_sln/releases",
    "deploy/LeagueOfLegends.app/Contents/Frameworks",
];

const LOL_SLN_PATH: [&str; 2] = [
    "Contents/LoL/RADS/projects/lol_game_client/releases",
    "deploy/LeagueOfLegends.app/Contents/Frameworks",
];

pub fn install() -> Result<()> {
    info!("Backing up Nvidia Cg…");
    backup_cg().chain_err(|| "Failed to backup Cg")?;

    let cg_dir = app_dirs::get_app_dir(AppDataType::UserCache, &APP_INFO, "Cg.Framework")?;
    if !cg_dir.exists() {
        download_cg(&cg_dir)?;
    } else {
        info!("Nvidia Cg is already cached!")
    }

    info!("Updating Nvidia Cg…\n");
    update_cg(&cg_dir).chain_err(|| "Failed to update Cg")?;
    Ok(())
}

pub fn remove() -> Result<()> {
    let cg_backup_path =
        app_dirs::get_app_dir(AppDataType::UserData, &APP_INFO, "Backups/Cg.framework")?;
    if !cg_backup_path.exists() {
        return Err("No Cg backup found!".into());
    }
    info!("Restoring Nvidia Cg…");
    update_cg(&cg_backup_path)?;
    fs::remove_dir_all(&cg_backup_path)?;
    info!("Removing Nvidia Cg backup…");
    let cg_cache_path = app_dirs::get_app_dir(AppDataType::UserCache, &APP_INFO, "Cg.Framework")?;

    if cg_cache_path.exists() {
        info!("Removing Nvidia Cg download cache…");
        fs::remove_dir_all(cg_cache_path)?;
    }
    Ok(())
}

fn download_cg(cg_dir: &Path) -> Result<()> {
    let download_dir = TempDir::new("lolupdater-cg-dl").chain_err(
        || "Failed to create temp dir for Nvidia Cg download",
    )?;
    let url: &str = "http://developer.download.nvidia.com/cg/Cg_3.1/Cg-3.1_April2012.dmg";
    let image_file = download_dir.path().join("cg.dmg");

    info!("Downloading Nvidia Cg…");
    let cg_hash = "96c86ab60abcf022554017b722236a0f1673618f3796305ebc8f5d5854552ccc5780aafdbd4473abd65349995e9c573b";
    download(&image_file, url, Some(cg_hash)).chain_err(
        || "Downloading Nvidia Cg failed!",
    )?;

    info!("Mounting Nvidia Cg…");
    let mount_dir = mount(&image_file).chain_err(|| "Failed to mount Cg image")?;

    info!("Extracting Nvidia Cg…");
    extract_cg(mount_dir.path(), cg_dir)?;

    info!("Unmounting Nvidia Cg…");
    unmount(mount_dir.path()).chain_err(
        || "Failed to unmount Cg",
    )?;
    Ok(())
}

fn backup_cg() -> Result<()> {
    let lol_cl_path = join_version(
        &PathBuf::from(LOL_CL_PATH[0]),
        &PathBuf::from(LOL_CL_PATH[1]),
    )?
        .join("Cg.framework");

    let cg_backup =
        app_dirs::get_app_dir(AppDataType::UserData, &APP_INFO, "Backups/Cg.framework")?;
    if cg_backup.exists() {
        info!("Skipping NVIDIA Cg backup! (Already exists)");
    } else {
        update_dir(&lol_cl_path, &cg_backup)?;
    }
    Ok(())
}

fn update_cg(cg_dir: &Path) -> Result<()> {
    let lol_cl_path = join_version(
        &PathBuf::from(LOL_CL_PATH[0]),
        &PathBuf::from(LOL_CL_PATH[1]),
    )?
        .join("Cg.framework");
    update_dir(cg_dir, &lol_cl_path)?;

    let lol_sln_path = join_version(
        &PathBuf::from(LOL_SLN_PATH[0]),
        &PathBuf::from(LOL_SLN_PATH[1]),
    )?
        .join("Cg.framework");
    update_dir(cg_dir, &lol_sln_path)?;
    Ok(())
}

fn extract_cg(mount_dir: &Path, target_dir: &Path) -> Result<()> {
    let a_path = mount_dir.join(
        "Cg-3.1.0013.app/Contents/Resources/Installer Items/NVIDIA_Cg.tgz",
    );
    let a_file = File::open(a_path)?;
    let decompressed = GzDecoder::new(a_file)?;
    let mut archive = Archive::new(decompressed);

    for file in archive.entries()? {
        let mut file = file?;
        let p = file.path()?.into_owned();
        if let Ok(path) = p.strip_prefix("Library/Frameworks/Cg.framework") {
            let target = target_dir.join(path);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            file.unpack(target)?;
        }
    }
    Ok(())
}
