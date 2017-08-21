pub use self::imp::*;

#[cfg(target_os = "macos")]
#[path = "cg_mac.rs"]
mod imp;

#[cfg(not(target_os = "macos"))]
#[path = "cg_win.rs"]
mod imp;
