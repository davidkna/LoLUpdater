pub use self::imp::*;

#[cfg(macos)]
#[path = "cg_mac.rs"]
mod imp;

#[cfg(not(macos))]
#[path = "cg_win.rs"]
mod imp;
