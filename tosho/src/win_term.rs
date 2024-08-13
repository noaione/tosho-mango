#[cfg(windows)]
use std::sync::OnceLock;

/// The static flag to check for VT support
#[cfg(windows)]
static IS_WIN_VT_SUPPORTED: OnceLock<bool> = OnceLock::new();

/// The flag to check for VT support
///
/// According to <https://learn.microsoft.com/en-us/windows/console/getconsolemode#parameters>
///
/// The flag for output mode is defined: ENABLE_VIRTUAL_TERMINAL_PROCESSING (`0x0004`)
#[cfg(windows)]
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

/// Check if the terminal supports ANSI/VT escape codes
///
/// Use `unsafe` + [`windows-sys`](https://crates.io/crates/windows-sys) crate to check for VT support.
///
/// Reference implementation from [`rich`](https://github.com/Textualize/rich) library.
#[cfg(windows)]
pub fn check_windows_vt_support() -> bool {
    use windows_sys::Win32::System::Console::GetConsoleMode;
    use windows_sys::Win32::System::Console::{GetStdHandle, STD_OUTPUT_HANDLE};

    *IS_WIN_VT_SUPPORTED.get_or_init(|| unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut console_mode: u32 = 0;

        if GetConsoleMode(handle, &mut console_mode) == 0 {
            false
        } else {
            console_mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING > 0
        }
    })
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
