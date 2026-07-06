#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod drivers;
mod io;
mod fs;
mod panic;
mod log;

// Phase G kernel components
mod scheduler;
mod mm;
mod hal;
mod syscalls;

use core::panic::PanicInfo;
use drivers::DriverRegistry;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic::panic_handler(info);
}

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

/// The main entry point for the kernel.
/// Called from UEFI bootloader with boot info.
#[no_mangle]
pub extern "C" fn kernel_main(boot_info: *const BootInfo) -> ! {
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

    // Initialize Phase G kernel components
    unsafe {
        // 1. Initialize interrupt controller (APIC/PIC)
        registry.vga.print_str("[1/7] Initializing interrupt controller... ");
        log::info("kernel", "Initializing interrupt controller");
        if hal::interrupt_controller::sigma_apic_init() == 0 {
            registry.vga.print_str("OK\n");
            log::info("kernel", "APIC initialized successfully");
        } else {
            registry.vga.print_str("FAILED\n");
            log::warn("kernel", "APIC initialization failed, falling back to PIC");
            // Fall back to PIC
            if hal::interrupt_controller::sigma_pic_init() == 0 {
                registry.vga.print_str("      PIC fallback: OK\n");
                log::info("kernel", "PIC initialized successfully");
            } else {
                log::error("kernel", "PIC initialization failed");
            }
        }

        // 2. Initialize buddy physical allocator
        registry.vga.print_str("[2/7] Initializing buddy allocator... ");
        log::info("kernel", "Initializing buddy allocator");
        let total_mem = 512 * 1024 * 1024; // 512MB placeholder
        if mm::buddy_allocator::sigma_buddy_init(0x1000000, total_mem) == 0 {
            registry.vga.print_str("OK\n");
            log::info("kernel", "Buddy allocator initialized successfully");
        } else {
            registry.vga.print_str("FAILED\n");
            log::error("kernel", "Buddy allocator initialization failed");
        }

        // 3. Initialize slab allocator (kmalloc)
        registry.vga.print_str("[3/7] Initializing slab allocator... ");
        log::info("kernel", "Initializing slab allocator");
        if mm::slab_allocator::sigma_slab_init() == 0 {
            registry.vga.print_str("OK\n");
            log::info("kernel", "Slab allocator initialized successfully");
        } else {
            registry.vga.print_str("FAILED\n");
            log::error("kernel", "Slab allocator initialization failed");
        }

        // 4. Initialize page table walker
        registry.vga.print_str("[4/7] Initializing page table walker... ");
        log::info("kernel", "Initializing page table walker");
        if !boot_info.is_null() {
            let info = &*boot_info;
            mm::page_table_walker::sigma_pt_init(info.kernel_virt);
            registry.vga.print_str("OK\n");
            log::info("kernel", "Page table walker initialized successfully");
        } else {
            registry.vga.print_str("SKIPPED (no boot info)\n");
            log::warn("kernel", "Page table walker skipped - no boot info");
        }

        // 5. Initialize round-robin scheduler
        registry.vga.print_str("[5/7] Initializing scheduler... ");
        log::info("kernel", "Initializing round-robin scheduler");
        if scheduler::round_robin_scheduler::sigma_sched_init() == 0 {
            registry.vga.print_str("OK\n");
            log::info("kernel", "Scheduler initialized successfully");
        } else {
            registry.vga.print_str("FAILED\n");
            log::error("kernel", "Scheduler initialization failed");
        }

        // 6. Initialize system call dispatcher
        registry.vga.print_str("[6/7] Initializing syscall dispatcher... ");
        log::info("kernel", "Initializing syscall dispatcher");
        if syscalls::syscall_dispatcher::sigma_syscall_init() == 0 {
            registry.vga.print_str("OK\n");
            log::info("kernel", "Syscall dispatcher initialized successfully");
        } else {
            registry.vga.print_str("FAILED\n");
            log::error("kernel", "Syscall dispatcher initialization failed");
        }

        // 7. Initialize framebuffer driver
        registry.vga.print_str("[7/7] Initializing framebuffer... ");
        log::info("kernel", "Initializing framebuffer driver");
        if !boot_info.is_null() {
            let info = &*boot_info;
            if info.framebuffer != 0 {
                if drivers::framebuffer::sigma_fb_init_gop(
                    info.framebuffer,
                    info.fb_width,
                    info.fb_height,
                    info.fb_stride,
                    32
                ) == 0 {
                    registry.vga.print_str("OK\n");
                    log::info("kernel", "Framebuffer initialized successfully");
                } else {
                    registry.vga.print_str("FAILED\n");
                    log::error("kernel", "Framebuffer initialization failed");
                }
            } else {
                registry.vga.print_str("SKIPPED (no framebuffer)\n");
                log::warn("kernel", "Framebuffer skipped - no framebuffer in boot info");
            }
        } else {
            registry.vga.print_str("SKIPPED (no boot info)\n");
            log::warn("kernel", "Framebuffer skipped - no boot info");
        }

        registry.vga.print_str("\n========================================\n");
        registry.vga.print_str("Kernel initialization complete!\n");
        registry.vga.print_str("SigmaOS is running.\n");
        log::info("kernel", "Kernel initialization complete");
    }

    loop {}
}

/// Legacy entry point for compatibility
#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel_main(core::ptr::null())
}
