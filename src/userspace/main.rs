#![allow(unused_imports, unused_variables, dead_code, unused_mut, clippy::all)]
// SigmaOS Userspace Main Entry Point
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]
#![allow(clippy::all, unused)]

use core::panic::PanicInfo;

#[cfg(target_os = "none")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Userspace entry point
    loop {}
}

#[cfg(not(target_os = "none"))]
fn main() {}

#[cfg(all(not(test), target_os = "none"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
