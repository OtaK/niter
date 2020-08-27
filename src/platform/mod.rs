#[cfg(target_os = "linux")]
mod bluez;
#[cfg(target_os = "linux")]
pub use self::bluez::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use self::macos::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;
