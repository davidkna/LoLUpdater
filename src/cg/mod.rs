pub use self::imp::*;

#[cfg(target_os = "macos")]
#[path = "mac.rs"]
mod imp;

#[cfg(not(target_os = "macos"))]
#[path = "win.rs"]
mod imp;
