use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::fs;
use std::mem;
use std::path::{Path, PathBuf};
#[cfg(target_os = "macos")]
use std::process::Command;

use app_dirs::AppInfo;
use ring::{digest, test};
use regex::Regex;
use reqwest;
#[cfg(target_os = "macos")]
use tempdir::{self, TempDir};
#[cfg(target_os = "macos")]
use walkdir::WalkDir;

pub use errors::*;

pub const APP_INFO: AppInfo = AppInfo {
    name: "LoLUpdater-rs",
    author: "LoLUpdater",
};

pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;


#[cfg(target_os = "macos")]
pub fn update_dir(from: &Path, to: &Path) -> Result<()> {
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

pub fn update_file(from: &Path, to: &Path) -> Result<()> {
    let mut reader = File::open(from)?;
    let mut writer = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(to)?;

    io::copy(&mut reader, &mut writer)?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn mount(image_path: &Path) -> Result<tempdir::TempDir> {
    let mountpoint = TempDir::new("lolupdater-mount")?;
    let exit_status = Command::new("/usr/bin/hdiutil")
        .arg("attach")
        .arg("-nobrowse")
        .arg("-quiet")
        .arg("-mountpoint")
        .arg(mountpoint.path().as_os_str())
        .arg(image_path.as_os_str())
        .spawn()?
        .wait()?;
    if !exit_status.success() {
        return Err("Mounting failed!".into());
    }
    Ok(mountpoint)
}

#[cfg(target_os = "macos")]
pub fn unmount(mountpoint: &Path) -> Result<()> {
    let exit_status = Command::new("/usr/bin/hdiutil")
        .arg("detach")
        .arg("-quiet")
        .arg(mountpoint.as_os_str())
        .spawn()?
        .wait()?;
    if !exit_status.success() {
        return Err("Unmounting failed!".into());
    }
    Ok(())
}

pub fn new_request(url: &str, gzip: bool) -> Result<reqwest::Response> {
    let r = reqwest::Client::builder()?
        .gzip(gzip)
        .build()?
        .get(url)?
        .header(reqwest::header::UserAgent::new(
            "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0)",
        ))
        .send()?;
    Ok(r)
}

pub fn download(target_path: &Path, url: &str, expected_hash: Option<&str>) -> Result<()> {
    let mut res = new_request(url, true)?;

    let mut target_image_file = File::create(target_path)?;
    match expected_hash {
        Some(h) => copy_digest(&mut res, &mut target_image_file, h),
        None => io::copy(&mut res, &mut target_image_file).map_err(::errors::Error::from),
    }?;
    Ok(())
}


lazy_static! {
    static ref VERSION_REGEX: Regex = {
        // 0 to 255
        let number = r"(?:[1-9]?[0-9])|(?:1[0-9][0-9])|(?:2[0-4][0-9])|(?:25[0-5])";

        // Parses version a.b.c.d
        let regex = format!(r"(?x) # Comments!
            ^({0})            # a
            (?:\.)({0})       # .b
            (?:\.)({0})       # .c
            (?:\.)({0})$      # .d
            ",
            number);
        Regex::new(&regex).unwrap()
    };
}

fn to_version(input: &str) -> u32 {
    let captures = VERSION_REGEX.captures(input).unwrap();
    // Unwrapping should always work here
    let a: u8 = captures.get(1).unwrap().as_str().parse().unwrap();
    let b: u8 = captures.get(2).unwrap().as_str().parse().unwrap();
    let c: u8 = captures.get(3).unwrap().as_str().parse().unwrap();
    let d: u8 = captures.get(4).unwrap().as_str().parse().unwrap();

    // Do scary stuff to make it an u32
    unsafe {
        let num = if cfg!(target_endian = "big") {
            [a, b, c, d]
        } else {
            [d, c, b, a]
        };
        mem::transmute::<[u8; 4], u32>(num)
    }
}
#[test]
fn to_version_works() {
    for i in 0..255 {
        let test_version = format!("0.0.0.{}", i);
        assert_eq!(to_version(&test_version), i);
    }
}

pub fn join_version(head: &Path, tail: &Path) -> Result<PathBuf> {
    let dir_iter = head.read_dir()?;
    let version = dir_iter
        .filter_map(|s| {
            let name = s.expect("Failed to unwrap DirEntry!").file_name();
            let name_str = name.into_string().expect("Failed to filename as Unicode!");
            if VERSION_REGEX.is_match(&name_str) {
                return Some(name_str);
            }
            None
        })
        .max_by_key(|k| to_version(k))
        .expect("Failed to get max");
    Ok(head.join(version).join(tail))
}

#[test]
fn regex_works() {
    for i in 0..255 {
        let test_version = format!("{0}.{0}.{0}.{0}", i);
        assert!(VERSION_REGEX.is_match(&test_version));
    }
    let i = 256;
    let test_version = format!("{0}.{0}.{0}.{0}", i);
    assert!(!VERSION_REGEX.is_match(&test_version));
}

pub fn copy_digest<R: ?Sized, W: ?Sized>(
    reader: &mut R,
    writer: &mut W,
    expected_hex: &str,
) -> Result<u64>
where
    R: Read,
    W: Write,
{
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let mut ctx = digest::Context::new(&digest::SHA384);
    let mut written = 0;
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => {
                let actual = ctx.finish();
                let expected: Vec<u8> = test::from_hex(expected_hex)?;
                if expected != actual.as_ref() {
                    return Err("Checksum validation Failed!".into());
                }
                return Ok(written);
            }
            Ok(len) => len,
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e).map_err(Error::from),
        };
        writer.write_all(&buf[..len])?;
        ctx.update(&buf[..len]);
        written += len as u64;
    }
}
