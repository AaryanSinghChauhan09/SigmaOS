#![allow(unused_imports, unused_variables, dead_code, unused_mut, clippy::all)]
// SigmaOS Drivers Main Entry Point
#![cfg_attr(not(any(target_os = "linux", target_os = "windows", target_os = "macos", test)), no_std)]
#![cfg_attr(not(any(target_os = "linux", target_os = "windows", target_os = "macos", test)), no_main)]

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos", test)))]
use core::panic::PanicInfo;

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos", test)))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Drivers entry point
    loop {}
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos", test)))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
fn main() {
    // Host stub
}
