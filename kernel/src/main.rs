#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod drivers;
mod io;
mod fs;
mod panic;
mod log;
pub mod linux_compat;
pub mod compat {
    pub mod wine {
        pub mod vsock;
    }
}

// Phase G kernel components - using stub implementations for build
// TODO: Integrate actual implementations from kernel/scheduler, kernel/mm, kernel/hal, kernel/syscalls

use drivers::DriverRegistry;

/// Boot info structure passed from UEFI bootloader
#[repr(C)]
pub struct BootInfo {
    pub magic: u64,
    pub memory_map: u64,
    pub memory_map_sz: usize,
    pub desc_sz: usize,
    pub rsdp_addr: u64,
    pub framebuffer: u64,
    pub fb_width: u32,
    pub fb_height: u32,
    pub fb_stride: u32,
    pub kernel_phys: u64,
    pub kernel_virt: u64,
    pub kernel_sz: u64,
    pub initramfs_phys: u64,
    pub initramfs_sz: u64,
}

extern "C" {
    fn sigma_fb_init_gop(fb: u64, width: u32, height: u32, stride: u32, bpp: u32) -> i32;
}

/// The main entry point for the kernel.
/// Called from UEFI bootloader with boot info.
#[no_mangle]
pub extern "C" fn kernel_main(_boot_info: *const BootInfo) -> ! {
    // Initialize logging system
    log::init();
    log::set_min_level(log::LogLevel::Info);
    log::info("kernel", "SigmaOS v15.0.0 Zenith - Phase G Kernel starting");

    // Initialize the Driver Registry (modular, OOP-based framework)
    let mut registry = DriverRegistry::new();
    
    // Initialize all hardware drivers (currently just VGA)
    registry.init_all();

    // Use the safe VGA driver to print to the screen
    registry.vga.print_str("SigmaOS v15.0.0 Zenith - Phase G Kernel\n");
    registry.vga.print_str("========================================\n");
    registry.vga.print_str("Initializing kernel subsystems...\n\n");

    // Initialize Phase G kernel components - stub implementations
    // TODO: Integrate actual implementations from kernel/scheduler, kernel/mm, kernel/hal, kernel/syscalls
    registry.vga.print_str("Phase G kernel components initialization skipped (stubs)\n");
    log::info("kernel", "Phase G kernel components skipped - using stub implementations");

    loop {}
}

/// Legacy entry point for compatibility
#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel_main(core::ptr::null())
}
