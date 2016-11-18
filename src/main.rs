#![feature(plugin)]

#![plugin(clippy)]

extern crate flate2;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate ring;
extern crate tar;
extern crate tempdir;
extern crate walkdir;


use std::io::prelude::*;
use std::io::{self, ErrorKind};
use std::fs::File;
use std::fs;
use std::path::{Path, PathBuf};
use std::vec;
use regex::Regex;
use std::process::Command;

use std::env;
use hyper::Client;
use hyper::header::Connection;

use flate2::read::GzDecoder;

use tempdir::TempDir;
use tar::Archive;
use tar::Entry;

use std::fmt;
use std::thread;
use ring::{digest, test};

const DEFAULT_BUF_SIZE: usize = 8 * 1024;
use walkdir::WalkDir;

const LOL_AIR_PATH: [&'static str; 2] = [
    "Contents/LoL/RADS/projects/lol_air_client/releases",
    "deploy/Frameworks"
];

const LOL_CL_PATH: [&'static str; 2] = [
    "Contents/LoL/RADS/solutions/lol_game_client_sln/releases",
    "deploy/LeagueOfLegends.app/Contents/Frameworks"
];

fn main() {
    let lol_dir = env::args().nth(1).unwrap_or("/Applications/League of Legends.app".to_string());
    env::set_current_dir(lol_dir)
        .expect("Failed to set CWD to LoL location");

    let result = join_version(&PathBuf::from(LOL_AIR_PATH[0]), &PathBuf::from(LOL_AIR_PATH[1]));
    print!("{:?}", result.unwrap());
    /*
    let cg_update = thread::spawn(|| {
        cg_main();
    });

    let cg_result = cg_update.join();
    if cg_result.is_err() {
        println!("Failed to update Cg!");
    } else {
        println!("Cg was updated!");
    }
    */
}

fn cg_main() {
    let download_dir = TempDir::new("lolupdater-cg-dl")
        .expect("Failed to create temp dir for Cg download");
    let url: &str = "http://developer.download.nvidia.com/cg/Cg_3.1/Cg-3.1_April2012.dmg";
    let image_file = download_dir.path().join("cg.dmg");
    println!("Downloading Cg…");
    download(&image_file, url, Some("56abcc26d2774b1a33adf286c09e83b6f878c270d4dd5bff5952b83c21af8fa69e3d37060f08b6869a9a40a0907be3dacc2ee2ef1c28916069400ed867b83925"))
    .expect("Downloading Nvidia Cg failed!");

    println!("Mounting Cg…");
    let mount_dir = mount(&image_file).expect("Failed to mount Cg image");

    println!("Extracting Cg…");
    let cg_dir = extract_cg(&mount_dir.path()).expect("Failed to extract Cg!");

    println!("Unmounting Cg…");
    unmount(&mount_dir.path()).expect("Failed to unmount Cg");

    println!("Updating Cg…");
    update_cg(/*&cg_dir.path()*/).expect("Failed to update Cg");

}

fn update_cg() -> io::Result<()> {
    // Stub
    Ok(())
}

fn extract_cg(mount_dir: &Path) -> io::Result<tempdir::TempDir> {
    let cg_dir = TempDir::new("lolupdater-cg")?;
    let a_file = File::open(mount_dir.join("Cg-3.1.0013.app/Contents/Resources/Installer Items/NVIDIA_Cg.tgz"))?;
    let decompressed = GzDecoder::new(a_file)?;
    let mut archive = Archive::new(decompressed);

    for file in archive.entries()? {
        let mut file = file?;
        let p = file.path()?.into_owned();
        if let Ok(path) = p.strip_prefix("Library/Frameworks/Cg.Framework") {
            let target = cg_dir.path().join(path);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            file.unpack(target)?;
        }
    }
    Ok(cg_dir)
}

fn mount(image_path: &Path) -> io::Result<tempdir::TempDir> {
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

fn download(target_path: &Path,
            url: &str,
            expected_hash: Option<&str>)
            -> Result<(), hyper::Error> {
    let client = Client::new();

    let mut res = client.get(url)
        .header(Connection::close())
        .send()?;
    assert_eq!(res.status, hyper::Ok);

    let mut target_image_file = File::create(target_path)?;
    match expected_hash {
        Some(h) => copy_digest(&mut res, &mut target_image_file, h),
        None => io::copy(&mut res, &mut target_image_file),
    };
    Ok(())
}


lazy_static! {
    static ref REGEX: Regex = {
        let number = r"0|[1-9][0-9]*";

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

fn to_version(input: &str) -> Option<(u64, u64, u64, u64)> {
    let captures = match REGEX.captures(input) {
        Some(captures) => captures,
        None => return None,
    };
    // Unwrapping should always work here
    let a = captures.name("a").unwrap().parse().unwrap();
    let b = captures.name("b").unwrap().parse().unwrap();
    let c = captures.name("c").unwrap().parse().unwrap();
    let d = captures.name("d").unwrap().parse().unwrap();
    Some((a, b, c, d))
}

fn join_version(head: &Path, tail: &Path) -> io::Result<PathBuf>  {
    let dir_iter = head.read_dir()?;
    let version = dir_iter.filter_map(|s| {
        let name = s.unwrap().file_name();
        let name_str = name.into_string().unwrap();
        if REGEX.is_match(&name_str) {
            return Some(name_str)
        }
        None
    }).max_by_key(|k| to_version(k)).unwrap();
    Ok(head.join(version).join(tail))
}

fn copy_digest<R: ?Sized, W: ?Sized>(reader: &mut R,
                                     writer: &mut W,
                                     expected_hex: &str)
                                     -> Result<u64, io::Error>
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
