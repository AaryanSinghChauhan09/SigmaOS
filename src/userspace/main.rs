#![allow(unused_imports, unused_variables, dead_code, unused_mut, clippy::all)]
// SigmaOS Userspace Main Entry Point
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]
#![allow(clippy::all, unused)]

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos", test)))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Userspace entry point
    loop {}
}

<<<<<<< HEAD
=======
#[cfg(not(target_os = "none"))]
fn main() {}

>>>>>>> origin/fix/mem-leak-custom-vec-drop-7188808108065826003
#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(target_os = "none"))]
fn main() {
    // Host stub
}
