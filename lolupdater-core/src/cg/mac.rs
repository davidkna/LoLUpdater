use std::fs::File;
use std::fs;
use std::path::{Path, PathBuf};

use app_dirs::{self, AppDataType};
use flate2::read::GzDecoder;
use tempdir::TempDir;
use tar::Archive;

use util::*;

const DOWNLOAD_URL: &str = "http://developer.download.nvidia.com/cg/Cg_3.1/Cg-3.1_April2012.dmg";

#[cfg_attr(rustfmt, rustfmt_skip)]
const DOWNLOAD_HASH: [u8; 48] = [
    0x96, 0xc8, 0x6a, 0xb6, 0x0a, 0xbc, 0xf0, 0x22, 0x55, 0x40, 0x17, 0xb7, 0x22, 0x23, 0x6a, 0x0f,
    0x16, 0x73, 0x61, 0x8f, 0x37, 0x96, 0x30, 0x5e, 0xbc, 0x8f, 0x5d, 0x58, 0x54, 0x55, 0x2c, 0xcc,
    0x57, 0x80, 0xaa, 0xfd, 0xbd, 0x44, 0x73, 0xab, 0xd6, 0x53, 0x49, 0x99, 0x5e, 0x9c, 0x57, 0x3b,
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
    let image_file = download_dir.path().join("cg.dmg");

    info!("Downloading Nvidia Cg…");
    download(&image_file, DOWNLOAD_URL, Some(&DOWNLOAD_HASH))
        .chain_err(|| "Downloading Nvidia Cg failed!")?;

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

#[test]
fn download_cg_works() {
    let target = TempDir::new("lolupdater-cg-target").unwrap();
    download_cg(&target.path()).unwrap();
}

fn backup_cg() -> Result<()> {
    let cg_backup =
        app_dirs::get_app_dir(AppDataType::UserData, &APP_INFO, "Backups/Cg.framework")?;
    if cg_backup.exists() {
        info!("Skipping NVIDIA Cg backup! (Already exists)");
    } else {
        update_dir(
            &LOLP_GC_PATH.with(|k| k.clone()).join("Cg.framework"),
            &cg_backup,
        )?;
    }
    Ok(())
}

fn update_cg(cg_dir: &Path) -> Result<()> {
    update_dir(
        cg_dir,
        &LOLP_GC_PATH.with(|k| k.clone()).join("Cg.framework"),
    )?;
    update_dir(
        cg_dir,
        &LOLSLN_GC_PATH.with(|k| k.clone()).join("Cg.framework"),
    )?;
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
