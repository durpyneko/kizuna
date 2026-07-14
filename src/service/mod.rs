#[cfg(target_os = "linux")]
mod linux;
pub use linux::{install, status, uninstall};

#[cfg(target_os = "windows")]
fn install() {
    // Windows Service Manager
}

#[cfg(target_os = "macos")]
fn install() {
    // launchd
}
