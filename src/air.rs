use std::path::{Path, PathBuf};
use app_dirs::{self, AppDataType};
use tempdir::TempDir;

use util::*;

const LOL_AIR_PATH: [&'static str; 2] = ["Contents/LoL/RADS/projects/lol_air_client/releases",
                                         "deploy/Frameworks"];

pub fn install() {
    println!("Backing up Adobe Air…");
    backup_air().expect("Failed to back up Adobe Air");

    let download_dir = TempDir::new("lolupdater-air-dl")
        .expect("Failed to create temp dir for Adobe Air download");
    let url: &str = "https://airdownload.adobe.com/air/mac/download/24.0/AdobeAIR.dmg";
    let image_file = download_dir.path().join("air.dmg");
    println!("Downloading Adobe Air…");
    download(&image_file, url, None).expect("Downloading Adobe Air failed!");

    println!("Mounting Adobe Air…");
    let mount_dir = mount(&image_file).expect("Failed to mount Adobe Air image");

    println!("Updating Adobe Air…");
    let air_framework = mount_dir.path()
        .join("Adobe Air Installer.app/Contents/Frameworks/Adobe AIR.framework");
    update_air(&air_framework).expect("Failed to update Adobe Air");

    println!("Unmounting Adobe Air…");
    unmount(mount_dir.path()).expect("Failed to unmount Adobe Air");
}

pub fn remove() -> Result<()> {
    let air_backup_path = app_dirs::get_app_dir(AppDataType::UserData,
                                                &APP_INFO,
                                                "Backups/Adobe AIR.framework")?;
    update_air(&air_backup_path)
}

fn backup_air() -> Result<()> {
    let lol_air_path = join_version(&PathBuf::from(LOL_AIR_PATH[0]),
                                    &PathBuf::from(LOL_AIR_PATH[1]))
        ?
        .join("Adobe AIR.framework");

    let air_backup = app_dirs::get_app_dir(AppDataType::UserData,
                                           &APP_INFO,
                                           "Backups/Adobe AIR.framework")?;
    if air_backup.exists() {
        println!("Skipping Adobe Air backup! (Already exists)");
    } else {
        update_dir(&lol_air_path, &air_backup)?;
    }
    Ok(())
}

fn update_air(air_dir: &Path) -> Result<()> {
    let lol_air_path = join_version(&PathBuf::from(LOL_AIR_PATH[0]),
                                    &PathBuf::from(LOL_AIR_PATH[1]))
        ?
        .join("Adobe AIR.framework");
    update_dir(air_dir, &lol_air_path)
}
