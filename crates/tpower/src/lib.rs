#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
compile_error!("Powerflow requires Apple Silicon macOS 27 or later");

pub mod battery_watcher;
pub mod de;
pub mod ffi;
pub mod macros;
pub mod provider;
pub mod util;
