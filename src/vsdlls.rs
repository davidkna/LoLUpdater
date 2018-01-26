use std::fs;
use std::path::Path;

use app_dirs::{self, AppDataType};

use util::*;
use winutil::SYSTEMX86;

const VSDLLS: [&str; 4] = [
    "concrt140.dll",
    "msvcp140.dll",
    "ucrtbase.dll",
    "vcruntime140.dll",
];

pub fn install() -> Result<()> {
    info!("Checking if VSDLL update supported…");
    let vsdll_supported = VSDLLS
        .into_iter()
        .all(|dll| SYSTEMX86.with(|k| k.clone()).join(dll).exists());
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
    let base = if LOL_KIND.with(|k| k == &InstallKind::Garena) {
        Path::new("LeagueClient")
    } else {
        Path::new("")
    };
    for dll in VSDLLS.into_iter() {
        update_file(&vsdll_backup.join(dll), &base.join(dll))?;
        update_file(
            &vsdll_backup.join(dll),
            &LOLP_GC_PATH.with(|k| k.clone()).join(dll),
        )?;
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
    let base = if LOL_KIND.with(|k| k == &InstallKind::Garena) {
        Path::new("LeagueClient")
    } else {
        Path::new("")
    };
    for dll in VSDLLS.into_iter() {
        update_file(&SYSTEMX86.with(|k| k.clone()).join(dll), &base.join(dll))?;
        update_file(
            &SYSTEMX86.with(|k| k.clone()).join(dll),
            &LOLP_GC_PATH.with(|k| k.clone()).join(dll),
        )?;
    }
    Ok(())
}
