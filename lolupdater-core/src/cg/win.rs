use std::fs;
use std::path::{Path, PathBuf};

use app_dirs::{self, AppDataType};

use util::*;

const LOL_CL_PATH: [&str; 2] = ["RADS/solutions/lol_game_client_sln/releases", "deploy"];

const LOL_SLN_PATH: [&str; 2] = ["RADS/projects/lol_game_client/releases", "deploy"];


const CG_MN_DLL_DL: &str = "https://lolupdater.com/downloads/DLLs/cg.dll";
const CG_GL_DLL_DL: &str = "https://lolupdater.com/downloads/DLLs/cgGL.dll";
const CG_D9_DLL_DL: &str = "https://lolupdater.com/downloads/DLLs/cgD3D9.dll";

const CG_MN_DLL_DL_HASH: &str = "546c4d9220056a181e3914ba14aec0d2bb0c9464918481e5628df32625956bb5d9bc2cb77506cc26d1abb044f6fa2d65";
const CG_GL_DLL_DL_HASH: &str = "536f96f25f0f6edee7d7c3d85d99d7e796434b320df06428e45b56c860a44ea3a276cba3b8f674ac9fd81a4c46e19036";
const CG_D9_DLL_DL_HASH: &str = "0b0831ecd7bb0b21bad3d27278afee227daf15ab5a96d51277e6f663e9019c63fc7f3bc0696664cae570ff7fa32b5cb1";


pub fn install() -> Result<()> {
    info!("Backing up Nvidia Cg…");
    backup_cg().chain_err(|| "Failed to backup Cg")?;

    let cg_dir = app_dirs::get_app_dir(AppDataType::UserCache, &APP_INFO, "Cg")?;
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
    let cg_backup_path = app_dirs::get_app_dir(AppDataType::UserData, &APP_INFO, "Backups/Cg")?;
    if !cg_backup_path.exists() {
        return Err("No Cg backup found!".into());
    }
    info!("Restoring Nvidia Cg…");
    update_cg(&cg_backup_path)?;
    fs::remove_dir_all(&cg_backup_path)?;
    info!("Removing Nvidia Cg backup…");
    let cg_cache_path = app_dirs::get_app_dir(AppDataType::UserCache, &APP_INFO, "Cg")?;

    if cg_cache_path.exists() {
        info!("Removing Nvidia Cg download cache…");
        fs::remove_dir_all(cg_cache_path)?;
    }
    Ok(())
}

fn download_cg(cg_dir: &Path) -> Result<()> {
    fs::create_dir(&cg_dir)?;
    download(
        &cg_dir.join("Cg.dll"),
        CG_MN_DLL_DL,
        Some(CG_MN_DLL_DL_HASH),
    )?;
    download(
        &cg_dir.join("CgGL.dll"),
        CG_GL_DLL_DL,
        Some(CG_GL_DLL_DL_HASH),
    )?;
    download(
        &cg_dir.join("cgD3D9.dll"),
        CG_D9_DLL_DL,
        Some(CG_D9_DLL_DL_HASH),
    )?;
    Ok(())
}

fn backup_cg() -> Result<()> {
    let lol_cl_path = join_version(
        &PathBuf::from(LOL_CL_PATH[0]),
        &PathBuf::from(LOL_CL_PATH[1]),
    )?;

    let cg_backup = app_dirs::get_app_dir(AppDataType::UserData, &APP_INFO, "Backups/Cg")?;
    if cg_backup.exists() {
        info!("Skipping NVIDIA Cg backup! (Already exists)");
    } else {
        fs::create_dir(&cg_backup)?;
        update_file(&lol_cl_path.join("Cg.dll"), &cg_backup.join("Cg.dll"))?;
        update_file(&lol_cl_path.join("CgGL.dll"), &cg_backup.join("CgGL.dll"))?;
        update_file(
            &lol_cl_path.join("cgD3D9.dll"),
            &cg_backup.join("CgD3D9.dll"),
        )?;
    }
    Ok(())
}

fn update_cg(cg_dir: &Path) -> Result<()> {
    let lol_cl_path = join_version(
        &PathBuf::from(LOL_CL_PATH[0]),
        &PathBuf::from(LOL_CL_PATH[1]),
    )?;
    update_file(&cg_dir.join("Cg.dll"), &lol_cl_path.join("Cg.dll"))?;
    update_file(&cg_dir.join("CgGL.dll"), &lol_cl_path.join("CgGL.dll"))?;
    update_file(&cg_dir.join("cgD3D9.dll"), &lol_cl_path.join("cgD3D9.dll"))?;

    let lol_sln_path = join_version(
        &PathBuf::from(LOL_SLN_PATH[0]),
        &PathBuf::from(LOL_SLN_PATH[1]),
    )?;
    update_file(&cg_dir.join("Cg.dll"), &lol_sln_path.join("Cg.dll"))?;
    update_file(&cg_dir.join("CgGL.dll"), &lol_sln_path.join("CgGL.dll"))?;
    update_file(&cg_dir.join("cgD3D9.dll"), &lol_sln_path.join("cgD3D9.dll"))?;
    Ok(())
}
