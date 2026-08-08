<<<<<<< HEAD
#![allow(warnings)]
#![allow(clippy::all)]
||||||| 23ef22a4a
// SigmaOS Userspace Main Entry Point
=======
#![allow(warnings)]
#![allow(clippy::all)]
// SigmaOS Userspace Main Entry Point
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]
#![allow(clippy::all, unused)]

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

// SigmaOS Userspace Main Entry Point

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

#[cfg(target_os = "none")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Userspace entry point
    loop {}
}

#[cfg(not(target_os = "none"))]
fn main() {}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
