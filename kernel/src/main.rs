#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod drivers;

use core::panic::PanicInfo;
use drivers::DriverRegistry;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// The main entry point for the kernel.
/// `_start` is the default entry point name for most linkers.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize the Driver Registry (modular, OOP-based framework)
    let mut registry = DriverRegistry::new();
    
    // Initialize all hardware drivers (currently just VGA)
    registry.init_all();

    // Use the safe VGA driver to print to the screen
    registry.vga.print_str("Welcome to SigmaOS Phase 2!\n");
    registry.vga.print_str("Modular Driver Framework Initialized successfully.\n");
    registry.vga.print_str("Booting core systems...");

    loop {}
}
