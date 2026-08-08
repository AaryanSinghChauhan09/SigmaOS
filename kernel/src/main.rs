#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod drivers;
mod io;
mod fs;
mod panic;
mod log;

// Phase G kernel components
#[path = "../scheduler/mod.rs"]
mod scheduler;
#[path = "../mm/mod.rs"]
mod mm;
#[path = "../hal/mod.rs"]
mod hal;
#[path = "../syscalls/mod.rs"]
mod syscalls;
#[path = "../security/mod.rs"]
mod security;
#[path = "../ipc/mod.rs"]
mod ipc;
#[path = "../fuzzing/mod.rs"]
mod fuzzing;

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
        registry.vga.print_str("[1/13] Initializing interrupt controller... ");
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

        // 1.5. Initialize deterministic interrupt handling
        registry.vga.print_str("[2/13] Initializing deterministic interrupt handling... ");
        log::info("kernel", "Initializing deterministic interrupt handling");
        if hal::deterministic_interrupt::sigma_deterministic_interrupt_init() == 0 {
            registry.vga.print_str("OK\n");
            log::info("kernel", "Deterministic interrupt handling initialized successfully");
        } else {
            registry.vga.print_str("FAILED\n");
            log::error("kernel", "Deterministic interrupt handling initialization failed");
        }

        // 2. Initialize buddy physical allocator
        registry.vga.print_str("[3/13] Initializing buddy allocator... ");
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
        registry.vga.print_str("[4/13] Initializing slab allocator... ");
        log::info("kernel", "Initializing slab allocator");
        if mm::slab_allocator::sigma_slab_init() == 0 {
            registry.vga.print_str("OK\n");
            log::info("kernel", "Slab allocator initialized successfully");
        } else {
            registry.vga.print_str("FAILED\n");
            log::error("kernel", "Slab allocator initialization failed");
        }

        // 4. Initialize page table walker
        registry.vga.print_str("[5/13] Initializing page table walker... ");
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
        registry.vga.print_str("[6/13] Initializing round-robin scheduler... ");
        log::info("kernel", "Initializing round-robin scheduler");
        if scheduler::round_robin_scheduler::sigma_sched_init() == 0 {
            registry.vga.print_str("OK\n");
            log::info("kernel", "Scheduler initialized successfully");
        } else {
            registry.vga.print_str("FAILED\n");
            log::error("kernel", "Scheduler initialization failed");
        }

        // 5.5. Initialize cache-aware scheduler
        registry.vga.print_str("[7/13] Initializing cache-aware scheduler... ");
        log::info("kernel", "Initializing cache-aware scheduler");
        if scheduler::cache_aware_scheduler::sigma_cache_aware_scheduler_init() == 0 {
            registry.vga.print_str("OK\n");
            log::info("kernel", "Cache-aware scheduler initialized successfully");
        } else {
            registry.vga.print_str("FAILED\n");
            log::error("kernel", "Cache-aware scheduler initialization failed");
        }

        // 6. Initialize system call dispatcher
        registry.vga.print_str("[8/13] Initializing syscall dispatcher... ");
        log::info("kernel", "Initializing syscall dispatcher");
        if syscalls::syscall_dispatcher::sigma_syscall_init() == 0 {
            registry.vga.print_str("OK\n");
            log::info("kernel", "Syscall dispatcher initialized successfully");
        } else {
            registry.vga.print_str("FAILED\n");
            log::error("kernel", "Syscall dispatcher initialization failed");
        }

        // 7. Initialize capability-based security system
        registry.vga.print_str("[9/13] Initializing capability system... ");
        log::info("kernel", "Initializing capability-based security system");
        if security::sigma_capability_init() == 0 {
            registry.vga.print_str("OK\n");
            log::info("kernel", "Capability system initialized successfully");
        } else {
            registry.vga.print_str("FAILED\n");
            log::error("kernel", "Capability system initialization failed");
        }

        // 8. Initialize framebuffer driver
        registry.vga.print_str("[10/13] Initializing framebuffer... ");
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

        // 9. Demonstrate capability system (create initial capabilities for init process)
        registry.vga.print_str("[11/13] Setting up initial capabilities... ");
        log::info("kernel", "Setting up initial capabilities for init process");
        
        // Grant init process comprehensive capabilities
        let init_pid: u64 = 1;
        let init_caps = [
            security::capability_system::CapabilityRight::CapProcessSpawn as u32,
            security::capability_system::CapabilityRight::CapProcessSignal as u32,
            security::capability_system::CapabilityRight::CapMemRead as u32,
            security::capability_system::CapabilityRight::CapMemWrite as u32,
            security::capability_system::CapabilityRight::CapFsRead as u32,
            security::capability_system::CapabilityRight::CapFsWrite as u32,
            security::capability_system::CapabilityRight::CapNetSocket as u32,
            security::capability_system::CapabilityRight::CapIpcSend as u32,
            security::capability_system::CapabilityRight::CapIpcReceive as u32,
            security::capability_system::CapabilityRight::CapIpcDelegate as u32,
        ];
        
        let mut caps_granted = 0;
        for &cap_right in &init_caps {
            let cap_id = security::capability_system::sigma_capability_create(init_pid, cap_right, true, 0);
            if cap_id != 0 {
                if security::capability_system::sigma_capability_grant(init_pid, cap_id) == 0 {
                    caps_granted += 1;
                }
            }
        }
        
        if caps_granted == init_caps.len() {
            registry.vga.print_str("OK\n");
            log::info("kernel", "Initial capabilities setup complete for init process");
        } else {
            registry.vga.print_str("PARTIAL\n");
            log::warn("kernel", "Some capabilities failed to initialize for init process");
        }

        // 10. Initialize zero-copy IPC system
        registry.vga.print_str("[12/13] Initializing zero-copy IPC... ");
        log::info("kernel", "Initializing zero-copy IPC system");
        if ipc::sigma_zerocopy_ipc_init() == 0 {
            registry.vga.print_str("OK\n");
            log::info("kernel", "Zero-copy IPC initialized successfully");
        } else {
            registry.vga.print_str("FAILED\n");
            log::error("kernel", "Zero-copy IPC initialization failed");
        }

        // 11. Initialize message fuzzer
        registry.vga.print_str("[13/13] Initializing message fuzzer... ");
        log::info("kernel", "Initializing message fuzzer");
        if fuzzing::sigma_fuzzer_init(0xDEADBEEFCAFEBABE) == 0 {
            registry.vga.print_str("OK\n");
            log::info("kernel", "Message fuzzer initialized successfully");
        } else {
            registry.vga.print_str("FAILED\n");
            log::error("kernel", "Message fuzzer initialization failed");
        }

        registry.vga.print_str("\n========================================\n");
        registry.vga.print_str("Kernel initialization complete!\n");
        registry.vga.print_str("SigmaOS is running.\n");
        log::info("kernel", "Kernel initialization complete");
    }

    loop {}
}
