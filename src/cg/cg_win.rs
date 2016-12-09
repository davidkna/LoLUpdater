
use util::*;

const LOL_CL_PATH: [&'static str; 2] = ["Contents/LoL/RADS/solutions/lol_game_client_sln/releases",
                                        "deploy/LeagueOfLegends.app/Contents/Frameworks"];

const LOL_SLN_PATH: [&'static str; 2] = ["Contents/LoL/RADS/projects/lol_game_client/releases",
                                         "deploy/LeagueOfLegends.app/Contents/Frameworks"];

pub fn install() {
    println!("Backing up Nvidia Cg…");
    backup_cg().expect("Failed to backup Cg");

    let download_dir = TempDir::new("lolupdater-cg-dl")
        .expect("Failed to create temp dir for Nvidia Cg download");
    let url: &str = "http://developer.download.nvidia.com/cg/Cg_3.1/Cg-3.1_April2012_Setup.exe";
    let cg_exe = download_dir.path().join("cg.exe");
    println!("Downloading Nvidia Cg…");
    let cg_hash = "066792a95eaa99a3dde3a10877a4bcd201834223eeee2b05b274f04112e55123df50478680984c5882a27eb2137e4833ed4f3468127d81bc8451f033bba75114";
    download(&cg_exe, url, Some(cg_hash)).expect("Downloading Nvidia Cg failed!");

    println!("Installing Nvidia Cg…");
    install(&cg_exe).expect("Failed to mount Cg image");

    println!("Updating Nvidia Cg…");
    update_cg().expect("Failed to update Cg");

}

pub fn remove() -> Result<()> {
    let cg_backup_path = Path::new("Backups/Cg.dll");
    update_cg(cg_backup_path)
}

fn backup_cg() -> Result<()> {
    let lol_cl_path = join_version(&PathBuf::from(LOL_CL_PATH[0]),
                                   &PathBuf::from(LOL_CL_PATH[1]))
        ?
        .join("Cg.framework");

    let cg_backup = Path::new("Backups/Cg.framework");
    if cg_backup.exists() {
        println!("Skipping NVIDIA Cg backup! (Already exists)");
    } else {
        update_dir(&lol_cl_path, cg_backup)?;
    }
    Ok(())
}

fn update_cg() -> Result<()> {
    let cg_dir = env::var("CG_BIN_PATH");
    let lol_cl_path = join_version(&PathBuf::from(LOL_CL_PATH[0]),
                                   &PathBuf::from(LOL_CL_PATH[1]))
        ?
        .join("Cg.framework");
    update_dir(cg_dir, &lol_cl_path)?;

    let lol_sln_path = join_version(&PathBuf::from(LOL_SLN_PATH[0]),
                                    &PathBuf::from(LOL_SLN_PATH[1]))
        ?
        .join("Cg.framework");
    update_dir(cg_dir, &lol_sln_path)?;
    Ok(())
}

fn install_cg(cg_exe: &Path) -> Result<TempDir> {
    Command::new("cg_exe").arg("/verysilent")
        .output()?;
}
