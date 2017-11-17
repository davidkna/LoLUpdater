use std::fs::{self, remove_file};
use std::path::Path;

use app_dirs::{self, AppDataType};

use util::*;
use winutil::SYSTEM32;

const VSDLLS: [&str; 4] = [
    "concrt140.dll",
    "msvcp140.dll",
    "ucrtbase.dll",
    "vcruntime140.dll",
];

#[test]
fn test_get_dir() {
    install().unwrap();
}


pub fn install() -> Result<()> {
    info!("Checking if VSDLL update supported…");
    let vsdll_supported = VSDLLS.into_iter().all(|dll| SYSTEM32.join(dll).exists());
    if !vsdll_supported {
        info!("VSDLL update not supported!");
        return Ok(());
    } else {
        info!("VSDLL update supported, moving on!");
    }
    info!("Backing up the VSDLLs…");
    backup_vsdlls().chain_err(|| "Failed to backup VSDLLs")?;

    info!("Updating VSDLLs…\n");
    update_vsdlls().chain_err(|| "Failed to update VSDLLs")?;
    Ok(())
}

pub fn remove() -> Result<()> {
    let vsdll_backup = app_dirs::get_app_dir(AppDataType::UserData, &APP_INFO, "Backups/vsdlls")?;
    if !vsdll_backup.exists() {
        return Err("No VSDLL backup found!".into());
    }
    for dll in VSDLLS.into_iter() {
        update_file(&vsdll_backup.join(dll), Path::new(dll))?;
        update_file(&vsdll_backup.join(dll), &LOL_CL_PATH.join(dll))?;
    }
    fs::remove_dir_all(&vsdll_backup)?;
    Ok(())
}

fn backup_vsdlls() -> Result<()> {
    let vsdll_backup = app_dirs::get_app_dir(AppDataType::UserData, &APP_INFO, "Backups/vsdlls")?;
    if vsdll_backup.exists() {
        info!("Skipping VSDLL backup! (Already exists)");
    } else {
        fs::create_dir(&vsdll_backup)?;
        for dll in VSDLLS.into_iter() {
            update_file(Path::new(dll), &vsdll_backup.join(dll))?
        }
    }
    Ok(())
}

fn update_vsdlls() -> Result<()> {
    for dll in VSDLLS.into_iter() {
        if Path::new(dll).exists() {
            remove_file(&dll)?;
        }
        let cl_path = LOL_CL_PATH.join(dll);
        if cl_path.exists() {
            remove_file(&cl_path)?;
        }
    }
    Ok(())
}
