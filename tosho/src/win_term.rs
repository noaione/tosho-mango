#[cfg(windows)]
use windows_sys::Win32::System::Console::GetConsoleMode;
#[cfg(windows)]
use windows_sys::Win32::System::Console::{GetStdHandle, STD_OUTPUT_HANDLE};

/// Check if the terminal supports ANSI/VT escape codes
///
/// Use `unsafe` + [`windows-sys`](https://crates.io/crates/windows-sys) crate to check for VT support.
///
/// Reference implementation from [`rich`](https://github.com/Textualize/rich) library.
#[cfg(windows)]
pub fn check_windows_vt_support() -> bool {
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut console_mode: u32 = 0;
        let raw = &mut console_mode as *mut u32;

        let success = GetConsoleMode(handle, raw);

        if success > 0 {
            console_mode & 0x0004 > 0
        } else {
            // fail
            false
        }
    }
}

/// Check if the terminal supports ANSI/VT escape codes
///
/// Use `unsafe` + [`windows-sys`](https://crates.io/crates/windows-sys) crate to check for VT support.
///
/// The following is a stub implementation for non-Windows platforms.
///
/// Reference implementation from [`rich`](https://github.com/Textualize/rich) library.
#[cfg(not(windows))]
pub fn check_windows_vt_support() -> bool {
    false
}
