#![allow(warnings)]
#![allow(clippy::all)]
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]
#![allow(clippy::all, unused)]

// SigmaOS Kernel Main Entry Point

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

#[cfg(target_os = "none")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Kernel entry point
    loop {}
}

#[cfg(not(target_os = "none"))]
fn main() {}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}