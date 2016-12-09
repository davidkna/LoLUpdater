use std::io::prelude::*;
use std::io::{self, ErrorKind};
use std::fs::File;
use std::fs;
use std::mem;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::result;

use hyper::Client;
use hyper::header::Connection;
use ring::{digest, test};
use regex::Regex;
use tempdir::{self, TempDir};
use walkdir::WalkDir;

use errors::LoLUpdaterError;

pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;

pub type Result<T> = result::Result<T, LoLUpdaterError>;

#[cfg(macos)]
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
    let mut writer = fs::OpenOptions::new().write(true).create(true).truncate(true).open(to)?;

    io::copy(&mut reader, &mut writer)?;
    Ok(())
}

#[cfg(macos)]
pub fn mount(image_path: &Path) -> Result<tempdir::TempDir> {
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

#[cfg(macos)]
pub fn unmount(mountpoint: &Path) -> io::Result<()> {
    Command::new("/usr/bin/hdiutil").arg("detach")
        .arg("-quiet")
        .arg(mountpoint.as_os_str())
        .output()?;
    Ok(())
}

pub fn download(target_path: &Path, url: &str, expected_hash: Option<&str>) -> Result<()> {
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
        let number = r"[0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5]";

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

pub fn join_version(head: &Path, tail: &Path) -> Result<PathBuf> {
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
        .expect("Failed to get max");
    Ok(head.join(version).join(tail))
}

pub fn copy_digest<R: ?Sized, W: ?Sized>(reader: &mut R,
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
