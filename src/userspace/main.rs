#![allow(warnings)]
#![allow(clippy::all)]
#![cfg_attr(target_os = "none", no_std)]
||||||| 43be3a7e8
||||||| 43be3a7e8
#![allow(warnings)]
#![allow(clippy::all)]
// SigmaOS Userspace Main Entry Point
#![no_std]
// SigmaOS Userspace Main Entry Point
#![cfg_attr(target_os = "none", no_std)]
||||||| 43be3a7e8
#![no_std]
#![cfg_attr(target_os = "none", no_std)]
||||||| 43be3a7e8
#![no_std]
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

// SigmaOS Userspace Main Entry Point

#[cfg(target_os = "none")]
||||||| 43be3a7e8
#[cfg(target_os = "none")]
use core::panic::PanicInfo;

||||||| 43be3a7e8
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
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
