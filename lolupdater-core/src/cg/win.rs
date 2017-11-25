use std::fs;
use std::path::Path;

use app_dirs::{self, AppDataType};

use util::*;

const CG_MN_DLL_DL: &str = "https://mobasuite.com/downloads/dlls/cg.dll";
const CG_GL_DLL_DL: &str = "https://mobasuite.com/downloads/dlls/cgGL.dll";
const CG_D9_DLL_DL: &str = "https://mobasuite.com/downloads/dlls/cgD3D9.dll";

#[cfg_attr(rustfmt, rustfmt_skip)]
const CG_MN_DLL_DL_HASH: [u8; 48] = [
    0x2d, 0x27, 0x17, 0x03, 0x95, 0x97, 0xde, 0x0b, 0xf4, 0x88, 0x14, 0xad, 0xee, 0x90, 0xa2, 0xb8,
    0xac, 0xfd, 0x9d, 0xab, 0x29, 0xf3, 0x7a, 0x64, 0xbf, 0x94, 0x8f, 0xb5, 0x5f, 0xcf, 0x9c, 0xa7,
    0x8f, 0xb0, 0x5f, 0x92, 0x22, 0x27, 0x31, 0x65, 0xe2, 0x3c, 0x5c, 0xa2, 0xab, 0x87, 0x4d, 0x21,
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const CG_GL_DLL_DL_HASH: [u8; 48] = [
    0xbc, 0x81, 0x45, 0xc4, 0x7d, 0x3c, 0xa6, 0x96, 0x5c, 0xe5, 0x19, 0x2e, 0x2a, 0xd7, 0xe6, 0xe7,
    0x26, 0x26, 0xdd, 0x8c, 0x3b, 0xe9, 0x6a, 0xa9, 0x30, 0x75, 0x69, 0x36, 0x1f, 0x30, 0x34, 0x5b,
    0x7b, 0x11, 0x24, 0xfb, 0x1d, 0x09, 0x2c, 0x0a, 0xdd, 0xb3, 0x82, 0x0b, 0x53, 0xa3, 0x8a, 0x78,
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const CG_D9_DLL_DL_HASH: [u8; 48] = [
    0xeb, 0x58, 0x44, 0x85, 0x9a, 0x39, 0xd6, 0x85, 0x3c, 0x1f, 0x14, 0x9c, 0xe0, 0x51, 0x16, 0x79,
    0x1d, 0x2a, 0x45, 0x7a, 0x7f, 0x98, 0x41, 0xed, 0x07, 0xec, 0xdc, 0x1a, 0xc7, 0xc5, 0xad, 0xcb,
    0x34, 0xd6, 0x30, 0x50, 0xbe, 0xe5, 0xad, 0xa5, 0x8e, 0xbd, 0x25, 0xb5, 0x02, 0xe7, 0x28, 0x24,
];

pub fn install() -> Result<()> {
    info!("Backing up Nvidia Cg…");
    backup_cg().chain_err(|| "Failed to backup Cg")?;

    let cg_dir = app_dirs::get_app_dir(AppDataType::UserCache, &APP_INFO, "Cg")?;
    if !cg_dir.exists() {
        info!("Downloading Nvidia Cg…");
        let result = download_cg(&cg_dir);
        if result.is_err() {
            fs::remove_dir_all(&cg_dir)?;
        }
        result?;
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
        Some(&CG_MN_DLL_DL_HASH),
    )?;
    download(
        &cg_dir.join("CgGL.dll"),
        CG_GL_DLL_DL,
        Some(&CG_GL_DLL_DL_HASH),
    )?;
    download(
        &cg_dir.join("cgD3D9.dll"),
        CG_D9_DLL_DL,
        Some(&CG_D9_DLL_DL_HASH),
    )?;
    Ok(())
}

#[test]
fn download_cg_works() {
    use tempdir::TempDir;
    let target = TempDir::new("lolupdater-cg-target").unwrap();
    download_cg(&target.path().join("cg")).unwrap();
}

fn backup_cg() -> Result<()> {
    let cg_backup = app_dirs::get_app_dir(AppDataType::UserData, &APP_INFO, "Backups/Cg")?;
    if cg_backup.exists() {
        info!("Skipping NVIDIA Cg backup! (Already exists)");
    } else {
        fs::create_dir(&cg_backup)?;
        update_file(
            &LOLP_GC_PATH.with(|k| k.clone()).join("Cg.dll"),
            &cg_backup.join("Cg.dll"),
        )?;
        update_file(
            &LOLP_GC_PATH.with(|k| k.clone()).join("CgGL.dll"),
            &cg_backup.join("CgGL.dll"),
        )?;
        update_file(
            &LOLP_GC_PATH.with(|k| k.clone()).join("cgD3D9.dll"),
            &cg_backup.join("CgD3D9.dll"),
        )?;
    }
    Ok(())
}

fn update_cg(cg_dir: &Path) -> Result<()> {
    update_file(
        &cg_dir.join("Cg.dll"),
        &LOLP_GC_PATH.with(|k| k.clone()).join("Cg.dll"),
    )?;
    update_file(
        &cg_dir.join("CgGL.dll"),
        &LOLP_GC_PATH.with(|k| k.clone()).join("CgGL.dll"),
    )?;
    update_file(
        &cg_dir.join("cgD3D9.dll"),
        &LOLP_GC_PATH.with(|k| k.clone()).join("cgD3D9.dll"),
    )?;

    update_file(
        &cg_dir.join("Cg.dll"),
        &LOLSLN_GC_PATH.with(|k| k.clone()).join("Cg.dll"),
    )?;
    update_file(
        &cg_dir.join("CgGL.dll"),
        &LOLSLN_GC_PATH.with(|k| k.clone()).join("CgGL.dll"),
    )?;
    update_file(
        &cg_dir.join("cgD3D9.dll"),
        &LOLSLN_GC_PATH.with(|k| k.clone()).join("cgD3D9.dll"),
    )?;
    Ok(())
}
