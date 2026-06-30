#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// The main entry point for the kernel.
/// `_start` is the default entry point name for most linkers.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Basic VGA buffer logic for "Hello World"
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in b"Welcome to SigmaOS Phase 1!".iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // Light cyan color
        }
    }

    loop {}
}
