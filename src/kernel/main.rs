// SigmaOS Kernel Main Entry Point
// Zero-dependency, no-std kernel bootstrap for bare-metal execution
#![allow(warnings)]
#![allow(clippy::all)]
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

/// Kernel entry point for bare-metal targets.
/// Initializes the CPU, sets up memory management, and enters the kernel main loop.
#[cfg(target_os = "none")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize core subsystems
    // 1. Set up GDT/IDT
    // 2. Enable paging
    // 3. Initialize memory allocator
    // 4. Start scheduler
    loop {}
}

/// Host-mode entry point for testing and simulation
#[cfg(not(target_os = "none"))]
fn main() {}

/// Panic handler for bare-metal targets
#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // TODO: Display panic info on VGA/serial
    loop {}
}