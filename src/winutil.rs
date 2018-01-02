extern crate winapi;

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;
use std::slice;
use util::*;

use self::winapi::shared::winerror::S_OK;
use self::winapi::um::combaseapi::CoTaskMemFree;
use self::winapi::um::knownfolders::FOLDERID_SystemX86;
use self::winapi::um::shlobj::SHGetKnownFolderPath;
use self::winapi::shared::ntdef::PWSTR;

thread_local! {
    pub static SYSTEMX86: PathBuf = {
        get_dir(&FOLDERID_SystemX86).unwrap()
    };
}

fn get_dir(id: &winapi::um::shtypes::KNOWNFOLDERID) -> Result<PathBuf> {
    let mut result: PWSTR = ptr::null_mut();
    let error;
    unsafe {
        error = SHGetKnownFolderPath(id, 0, ptr::null_mut(), &mut result);
    }
    if error != S_OK {
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
        CoTaskMemFree(result as *mut _);
        Ok(PathBuf::from(os_string))
    }
}

#[test]
fn test_get_dir() {
    get_dir(&FOLDERID_SystemX86).unwrap();
}
