use std::fs::File;
use std::fs;
use std::path::{Path, PathBuf};

use app_dirs::{self, AppDataType};
use libflate::gzip::Decoder;
use tempdir::TempDir;
use tar::Archive;

use util::*;


const LOL_CL_PATH: [&'static str; 2] = ["Contents/LoL/RADS/solutions/lol_game_client_sln/releases",
                                        "deploy/LeagueOfLegends.app/Contents/Frameworks"];

const LOL_SLN_PATH: [&'static str; 2] = ["Contents/LoL/RADS/projects/lol_game_client/releases",
                                         "deploy/LeagueOfLegends.app/Contents/Frameworks"];

pub fn install() -> Result<()> {
    println!("Backing up Nvidia Cg…");
    backup_cg().chain_err(|| "Failed to backup Cg")?;

    let download_dir = TempDir::new("lolupdater-cg-dl")
        .chain_err(|| "Failed to create temp dir for Nvidia Cg download")?;
    let url: &str = "http://developer.download.nvidia.com/cg/Cg_3.1/Cg-3.1_April2012.dmg";
    let image_file = download_dir.path().join("cg.dmg");
    println!("Downloading Nvidia Cg…");
    let cg_hash = "56abcc26d2774b1a33adf286c09e83b6f878c270d4dd5bff5952b83c21af8fa69e3d37060f08b6869a9a40a0907be3dacc2ee2ef1c28916069400ed867b83925";
    download(&image_file, url, Some(cg_hash))
        .chain_err(|| "Downloading Nvidia Cg failed!")?;

    println!("Mounting Nvidia Cg…");
    let mount_dir = mount(&image_file)
        .chain_err(|| "Failed to mount Cg image")?;

    println!("Extracting Nvidia Cg…");
    let cg_dir = extract_cg(mount_dir.path())
        .chain_err(|| "Failed to extract Cg!")?;

    println!("Unmounting Nvidia Cg…");
    unmount(mount_dir.path())
        .chain_err(|| "Failed to unmount Cg")?;

    println!("Updating Nvidia Cg…");
    update_cg(cg_dir.path())
        .chain_err(|| "Failed to update Cg")?;
    Ok(())
}

pub fn remove() -> Result<()> {
    let cg_backup_path =
        app_dirs::get_app_dir(AppDataType::UserData, &APP_INFO, "Backups/Cg.framework")?;
    update_cg(&cg_backup_path)
}

fn backup_cg() -> Result<()> {
    let lol_cl_path = join_version(&PathBuf::from(LOL_CL_PATH[0]),
                                   &PathBuf::from(LOL_CL_PATH[1]))?
            .join("Cg.framework");

    let cg_backup =
        app_dirs::get_app_dir(AppDataType::UserData, &APP_INFO, "Backups/Cg.framework")?;
    if cg_backup.exists() {
        println!("Skipping NVIDIA Cg backup! (Already exists)");
    } else {
        update_dir(&lol_cl_path, &cg_backup)?;
    }
    Ok(())
}

fn update_cg(cg_dir: &Path) -> Result<()> {
    let lol_cl_path = join_version(&PathBuf::from(LOL_CL_PATH[0]),
                                   &PathBuf::from(LOL_CL_PATH[1]))?
            .join("Cg.framework");
    update_dir(cg_dir, &lol_cl_path)?;

    let lol_sln_path = join_version(&PathBuf::from(LOL_SLN_PATH[0]),
                                    &PathBuf::from(LOL_SLN_PATH[1]))?
            .join("Cg.framework");
    update_dir(cg_dir, &lol_sln_path)?;
    Ok(())
}

fn extract_cg(mount_dir: &Path) -> Result<TempDir> {
    let cg_dir = TempDir::new("lolupdater-cg")?;
    let a_path = mount_dir.join("Cg-3.1.0013.app/Contents/Resources/Installer Items/NVIDIA_Cg.tgz");
    let a_file = File::open(a_path)?;
    let decompressed = Decoder::new(a_file)?;
    let mut archive = Archive::new(decompressed);

    for file in archive.entries()? {
        let mut file = file?;
        let p = file.path()?.into_owned();
        if let Ok(path) = p.strip_prefix("Library/Frameworks/Cg.framework") {
            let target = cg_dir.path().join(path);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            file.unpack(target)?;
        }
    }
    Ok(cg_dir)
}
