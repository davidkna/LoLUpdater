#![feature(plugin)]

#![plugin(clippy)]

extern crate flate2;
extern crate hyper;
extern crate regex;
extern crate ring;
extern crate tar;
extern crate tempdir;
extern crate walkdir;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::io::prelude::*;
use std::io::{self, ErrorKind};
use std::fs::File;
use std::fs;
use std::mem;
use std::num;
use std::path::{self, Path, PathBuf};
use std::process::Command;
use std::result;
use std::thread;

use hyper::Client;
use hyper::header::Connection;
use flate2::read::GzDecoder;
use ring::{digest, test};
use regex::Regex;
use tempdir::TempDir;
use tar::Archive;
use walkdir::WalkDir;

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

const LOL_AIR_PATH: [&'static str; 2] = ["Contents/LoL/RADS/projects/lol_air_client/releases",
                                         "deploy/Frameworks"];

const LOL_CL_PATH: [&'static str; 2] = ["Contents/LoL/RADS/solutions/lol_game_client_sln/releases",
                                        "deploy/LeagueOfLegends.app/Contents/Frameworks"];

const LOL_SLN_PATH: [&'static str; 2] = ["Contents/LoL/RADS/projects/lol_game_client/releases",
                                         "deploy/LeagueOfLegends.app/Contents/Frameworks"];

#[derive(Debug)]
enum LoLUpdaterError {
    Hyper(hyper::Error),
    Io(io::Error),
    Parse(num::ParseIntError),
    Prefix(path::StripPrefixError),
    WalkDir(walkdir::Error),
}

impl From<hyper::Error> for LoLUpdaterError {
    fn from(err: hyper::Error) -> LoLUpdaterError {
        LoLUpdaterError::Hyper(err)
    }
}

impl From<io::Error> for LoLUpdaterError {
    fn from(err: io::Error) -> LoLUpdaterError {
        LoLUpdaterError::Io(err)
    }
}

impl From<num::ParseIntError> for LoLUpdaterError {
    fn from(err: num::ParseIntError) -> LoLUpdaterError {
        LoLUpdaterError::Parse(err)
    }
}

impl From<path::StripPrefixError> for LoLUpdaterError {
    fn from(err: path::StripPrefixError) -> LoLUpdaterError {
        LoLUpdaterError::Prefix(err)
    }
}

impl From<walkdir::Error> for LoLUpdaterError {
    fn from(err: walkdir::Error) -> LoLUpdaterError {
        LoLUpdaterError::WalkDir(err)
    }
}

type Result<T> = result::Result<T, LoLUpdaterError>;

fn main() {
    println!("LoLUpdater for macOS 3.0.0");
    println!("Report errors, feature requests or any issues at \
              https://github.com/LoLUpdater/LoLUpdater-macOS/issues.");

    let mode = env::args().nth(1).unwrap_or("install".to_string());
    let lol_dir = env::args().nth(2).unwrap_or("/Applications/League of Legends.app".to_string());
    env::set_current_dir(lol_dir).expect("Failed to set CWD to LoL location");

    match mode.as_ref() {
        "install" => install(),
        "uninstall" => uninstall(),
        _ => panic!("Unkown mode!"),
    }
}


fn install() {
    if !Path::new("Backups").exists() {
        fs::create_dir("Backups").expect("Create Backup dir");
    }

    let air_update = thread::Builder::new().name("air_thread".to_string()).spawn(|| {
        air_main();
    }).unwrap();

    let cg_update = thread::Builder::new().name("cg_thread".to_string()).spawn(|| {
        cg_main();
    }).unwrap();;

    let air_result = air_update.join();
    if air_result.is_ok() {
        println!("Adobe Air was updated!");
    } else {
        println!("Failed to update Adobe Air!");
    }

    let cg_result = cg_update.join();
    if cg_result.is_ok() {
        println!("Cg was updated!");
    } else {
        println!("Failed to update Cg!");
    }
    println!("Done installing!");
}

fn uninstall() {
    let air_backup_path = Path::new("Backups/Adobe AIR.framework");
    update_air(air_backup_path).expect("Failed to uninstall Adobe Air");

    let cg_backup_path = Path::new("Backups/Adobe Cg.framework");
    update_cg(cg_backup_path).expect("Failed to uninstall Cg");

    println!("Done uninstalling!");
}



fn air_main() {
    println!("Backing up Adobe Air…");
    backup_air().expect("Failed to back up Adobe Air");

    let download_dir = TempDir::new("lolupdater-air-dl")
        .expect("Failed to create temp dir for Adobe Air download");
    let url: &str = "https://airdownload.adobe.com/air/mac/download/23.0/AdobeAIR.dmg";
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

fn backup_air() -> Result<()> {
    let lol_air_path = join_version(&PathBuf::from(LOL_AIR_PATH[0]),
                                    &PathBuf::from(LOL_AIR_PATH[1]))
        ?
        .join("Adobe AIR.framework");

    let air_backup = Path::new("Backups/Adobe AIR.framework");
    if air_backup.exists() {
        println!("Skipping Adobe Air backup! (Already exists)");
    } else {
        update_dir(&lol_air_path, air_backup)?;
    }
    Ok(())
}

fn update_air(air_dir: &Path) -> Result<()> {
    let lol_air_path = join_version(&PathBuf::from(LOL_AIR_PATH[0]),
                                    &PathBuf::from(LOL_AIR_PATH[1]))
        ?
        .join("Adobe AIR.framework");
    update_dir(air_dir, &lol_air_path)?;
    Ok(())
}

fn cg_main() {
    println!("Backing up Nvidia Cg…");
    backup_cg().expect("Failed to backup Cg");

    let download_dir = TempDir::new("lolupdater-cg-dl")
        .expect("Failed to create temp dir for Nvidia Cg download");
    let url: &str = "http://developer.download.nvidia.com/cg/Cg_3.1/Cg-3.1_April2012.dmg";
    let image_file = download_dir.path().join("cg.dmg");
    println!("Downloading Nvidia Cg…");
    let cg_hash = "56abcc26d2774b1a33adf286c09e83b6f878c270d4dd5bff5952b83c21af8fa69e3d37060f08b6869a9a40a0907be3dacc2ee2ef1c28916069400ed867b83925";
    download(&image_file, url, Some(cg_hash)).expect("Downloading Nvidia Cg failed!");

    println!("Mounting Nvidia Cg…");
    let mount_dir = mount(&image_file).expect("Failed to mount Cg image");

    println!("Extracting Nvidia Cg…");
    let cg_dir = extract_cg(mount_dir.path()).expect("Failed to extract Cg!");

    println!("Unmounting Nvidia Cg…");
    unmount(mount_dir.path()).expect("Failed to unmount Cg");

    println!("Updating Nvidia Cg…");
    update_cg(cg_dir.path()).expect("Failed to update Cg");

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

fn update_cg(cg_dir: &Path) -> Result<()> {
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

fn update_dir(from: &Path, to: &Path) -> Result<()> {
    let walker = WalkDir::new(from);
    for entry in walker {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let stripped_entry = entry.path().strip_prefix(from)?;
        let target = to.join(stripped_entry);
        if metadata.is_file() {
            if target.is_dir() {
                fs::remove_dir_all(&target)?;
            }
            update_file(entry.path(), &target)?;
        } else if metadata.is_dir() && !target.is_dir() {
            fs::create_dir(target)?;
        }
    }
    Ok(())
}

fn update_file(from: &Path, to: &Path) -> Result<()> {
    let mut reader = File::open(from)?;
    let mut writer = fs::OpenOptions::new().write(true).create(true).truncate(true).open(to)?;

    io::copy(&mut reader, &mut writer)?;
    Ok(())
}

fn extract_cg(mount_dir: &Path) -> Result<tempdir::TempDir> {
    let cg_dir = TempDir::new("lolupdater-cg")?;
    let a_path = mount_dir.join("Cg-3.1.0013.app/Contents/Resources/Installer Items/NVIDIA_Cg.tgz");
    let a_file = File::open(a_path)?;
    let decompressed = GzDecoder::new(a_file)?;
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

fn mount(image_path: &Path) -> Result<tempdir::TempDir> {
    let mountpoint = TempDir::new("lolupdater-mount")?;
    Command::new("/usr/bin/hdiutil").arg("attach")
        .arg("-nobrowse")
        .arg("-quiet")
        .arg("-mountpoint")
        .arg(mountpoint.path().as_os_str())
        .arg(image_path.as_os_str())
        .output()?;
    Ok(mountpoint)
}

fn unmount(mountpoint: &Path) -> io::Result<()> {
    Command::new("/usr/bin/hdiutil").arg("detach")
        .arg("-quiet")
        .arg(mountpoint.as_os_str())
        .output()?;
    Ok(())
}

fn download(target_path: &Path, url: &str, expected_hash: Option<&str>) -> Result<()> {
    let client = Client::new();

    let mut res = client.get(url)
        .header(Connection::close())
        .send()?;

    let mut target_image_file = File::create(target_path)?;
    match expected_hash {
        Some(h) => copy_digest(&mut res, &mut target_image_file, h),
        None => io::copy(&mut res, &mut target_image_file),
    }?;
    Ok(())
}


lazy_static! {
    static ref VERSION_REGEX: Regex = {
        // 0 to 255
        let number = r"0|[1-9][0-9]|[1-2][0-5][0-9]";

        // Parses version a.b.c.d
        let regex = format!(r"(?x) # Comments!
            ^(?P<a>{0})            # a
            (?:\.(?P<b>{0}))       # b
            (?:\.(?P<c>{0}))       # c
            (?:\.(?P<d>{0}))$      # d
            ",
            number);
        Regex::new(&regex).unwrap()
    };
}

fn to_version(input: &str) -> u32 {
    let captures = VERSION_REGEX.captures(input).unwrap();
    // Unwrapping should always work here
    let a: u8 = captures.name("a").unwrap().parse().unwrap();
    let b: u8 = captures.name("b").unwrap().parse().unwrap();
    let c: u8 = captures.name("c").unwrap().parse().unwrap();
    let d: u8 = captures.name("d").unwrap().parse().unwrap();

    // Do scary stuff to make it an u32
    unsafe {
        let num = [a, b, c, d];
        mem::transmute::<[u8; 4], u32>(num)
    }
}

fn join_version(head: &Path, tail: &Path) -> Result<PathBuf> {
    let dir_iter = head.read_dir()?;
    let version = dir_iter.filter_map(|s| {
            let name = s.expect("Failed to unwrap DirEntry!").file_name();
            let name_str = name.into_string().expect("Failed to filename as Unicode!");
            if VERSION_REGEX.is_match(&name_str) {
                return Some(name_str);
            }
            None
        })
        .max_by_key(|k| to_version(k))
        .unwrap();
    Ok(head.join(version).join(tail))
}

fn copy_digest<R: ?Sized, W: ?Sized>(reader: &mut R,
                                     writer: &mut W,
                                     expected_hex: &str)
                                     -> io::Result<u64>
    where R: Read,
          W: Write
{
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let mut ctx = digest::Context::new(&digest::SHA512);
    let mut written = 0;
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => {
                let actual = ctx.finish();
                let expected: Vec<u8> = test::from_hex(expected_hex).unwrap();
                if &expected != &actual.as_ref() {
                    return Err(io::Error::new(io::ErrorKind::Other, "Checksum validation Failed!"));
                }
                return Ok(written);
            }
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(&buf[..len])?;
        ctx.update(&buf[..len]);
        written += len as u64;
    }
}
