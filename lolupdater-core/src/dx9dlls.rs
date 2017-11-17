use std::fs::{self, remove_file};

use app_dirs::{self, AppDataType};

use util::*;
use winutil::SYSTEM32;

pub fn install() -> Result<()> {
    info!("Checking if DX9DLL update supported…");
    let dx9dll_supported = SYSTEM32.join("D3DCompiler_43.dll").exists();
    if !dx9dll_supported {
        info!("DX9DLL update not supported!");
        return Ok(());
    } else {
        info!("DX9DLL update supported, moving on!");
    }
    info!("Backing up the DX9DLLs…");
    backup_dx9dlls().chain_err(|| "Failed to backup DX9DLLs")?;

    info!("Updating DX9DLLs…\n");
    update_dx9dlls().chain_err(|| "Failed to update DX9DLLs")?;
    Ok(())
}

pub fn remove() -> Result<()> {
    let dx9dll_backup = app_dirs::get_app_dir(AppDataType::UserData, &APP_INFO, "Backups/dx9dlls")?;
    if !dx9dll_backup.exists() {
        return Err("No DX9DLL backup found!".into());
    }
    update_file(
        &dx9dll_backup.join("D3DCompiler_43.dll"),
        &LOL_CL_PATH.join("D3DCompiler_43.dll"),
    )?;
    update_file(
        &dx9dll_backup.join("D3DCompiler_43.dll"),
        &LOL_SLN_PATH.join("D3DCompiler_43.dll"),
    )?;
    fs::remove_dir_all(&dx9dll_backup)?;
    Ok(())
}

fn backup_dx9dlls() -> Result<()> {
    let dx9dll_backup = app_dirs::get_app_dir(AppDataType::UserData, &APP_INFO, "Backups/dx9dlls")?;
    if dx9dll_backup.exists() {
        info!("Skipping DX9DLL backup! (Already exists)");
    } else {
        fs::create_dir(&dx9dll_backup)?;
        update_file(
            &LOL_CL_PATH.join("D3DCompiler_43.dll"),
            &dx9dll_backup.join("D3DCompiler_43.dll"),
        )?;
    }
    Ok(())
}

fn update_dx9dlls() -> Result<()> {
    let cl_path = LOL_CL_PATH.join("D3DCompiler_43.dll");
    if cl_path.exists() {
        remove_file(&cl_path)?;
    }
    let sln_path = LOL_SLN_PATH.join("D3DCompiler_43.dll");
    if sln_path.exists() {
        remove_file(&sln_path)?;
    }

    Ok(())
}
