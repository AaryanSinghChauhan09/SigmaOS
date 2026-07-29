<<<<<<< HEAD
// SigmaOS Userspace Main Entry Point
=======
#![allow(warnings)]
#![allow(clippy::all)]
>>>>>>> origin/fix/mem-leak-custom-vec-drop-7188808108065826003
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]
#![allow(clippy::all, unused)]

<<<<<<< HEAD
=======
// SigmaOS Userspace Main Entry Point

>>>>>>> origin/fix/mem-leak-custom-vec-drop-7188808108065826003
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
