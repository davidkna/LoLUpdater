extern crate shell32;
extern crate winapi;
extern crate ole32;

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;
use std::slice;
use util::*;

lazy_static! {
    pub static ref SYSTEMX86: PathBuf = {
        get_dir(&SYSTEMX86_ID).unwrap()
    };
}

// {D65231B0-B2F1-4857-A4CE-A8E7C6EA7D27}
static SYSTEMX86_ID: winapi::shtypes::KNOWNFOLDERID = winapi::shtypes::KNOWNFOLDERID {
    Data1: 0xD65231B0,
    Data2: 0xB2F1,
    Data3: 0x4857,
    Data4: [0xA4, 0xCE, 0xA8, 0xE7, 0xC6, 0xEA, 0x7D, 0x27],
};

fn get_dir(id: &winapi::shtypes::KNOWNFOLDERID) -> Result<PathBuf> {
    let mut result: winapi::PWSTR = ptr::null_mut();
    let error;
    unsafe {
        error = shell32::SHGetKnownFolderPath(id, 0, ptr::null_mut(), &mut result);
    }
    if error != winapi::S_OK {
        return Err("Failed to get Path from WINAPI".into());
    }
    unsafe {
        let mut len = 0;
        let mut cur = result;
        while *cur != 0 {
            len += 1;
            cur = cur.offset(1);
        }
        let os_string: OsString = OsStringExt::from_wide(slice::from_raw_parts(result, len));
        ole32::CoTaskMemFree(result as *mut _);
        Ok(PathBuf::from(os_string))
    }
}

#[test]
fn test_get_dir() {
    get_dir(&SYSTEMX86_ID).unwrap();
}
