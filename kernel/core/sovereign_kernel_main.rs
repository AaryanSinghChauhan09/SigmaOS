// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign Kernel Main (Rust, no_std)
//! Replaces: kernel/core/sigma_kernel_main.c
//! =========================================================================

#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Extern structures and functions
extern "C" {
    fn vga_putc(c: char, color: u8);
    fn serial_init();
    fn serial_puts(s: *const u8);
    fn idt_init();
    fn sigma_pic_init(offset1: i32, offset2: i32);
    fn sigma_rollback_check_fallback() -> i32;
    fn sigma_rollback_mark_boot_successful();
    fn sigma_resilient_fallback_entry(panic_reason: *const u8);
    fn sigma_keyboard_init();
    fn sigma_keyboard_read() -> char;
    fn sigma_slab_init();
    fn sigma_pit_init(hz: u32);
    
    // Rust modules initialized from main
    fn process_manager_init();
    fn ipc_init();
    fn asched_init();
}

pub struct SovereignVga {
    default_color: u8,
}

impl SovereignVga {
    pub const fn new(color: u8) -> Self {
        Self { default_color: color }
    }

    pub fn write_str(&self, s: &str) {
        for c in s.chars() {
            unsafe {
                vga_putc(c, self.default_color);
            }
        }
    }
}

pub trait BootStep {
    fn name(&self) -> &'static str;
    fn execute(&self) -> bool;
}

struct SerialInitStep;
impl BootStep for SerialInitStep {
    fn name(&self) -> &'static str { "Serial Debug Initialization" }
    fn execute(&self) -> bool {
        unsafe { serial_init(); }
        true
    }
}

struct InterruptInitStep;
impl BootStep for InterruptInitStep {
    fn name(&self) -> &'static str { "Interrupt Vector Table (IDT) Setup" }
    fn execute(&self) -> bool {
        unsafe {
            idt_init();
            sigma_pic_init(32, 40);
        }
        true
    }
}

struct MemoryAllocInitStep;
impl BootStep for MemoryAllocInitStep {
    fn name(&self) -> &'static str { "Slab Allocator Initialization" }
    fn execute(&self) -> bool {
        unsafe { sigma_slab_init(); }
        true
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_kernel_main(_multiboot_info: *mut u8, _magic: u32) {
    // 1. Initialise Serial
    let serial_step = SerialInitStep;
    serial_step.execute();
    serial_puts(b"\xCE\xA3 SigmaOS Zenith Booting (Rust core)...\n\0".as_ptr());

    // 1.1 Rollback Gate Check
    if sigma_rollback_check_fallback() != 0 {
        serial_puts(b"[BOOT] Rollback triggered. Loading safe recovery mode.\n\0".as_ptr());
        sigma_resilient_fallback_entry(b"Rollback gate forced recovery mode\0".as_ptr());
        loop {
            core::arch::asm!("hlt");
        }
    }

    let vga = SovereignVga::new(0x07);
    vga.write_str("Σ SigmaOS Zenith Kernel Initializing (Rust)\n");

    // 2. Load Core Interrupt structures
    let idt_step = InterruptInitStep;
    idt_step.execute();
    serial_puts(b"[HAL] Interrupt controllers and IDT setup complete.\n\0".as_ptr());

    // 3. Setup Memory Management
    let mem_step = MemoryAllocInitStep;
    mem_step.execute();
    serial_puts(b"[MEM] Slab memory manager initialized.\n\0".as_ptr());

    // 4. Devices Setup
    sigma_pit_init(1000);
    serial_puts(b"[HAL] PIT Timer Active (1000Hz).\n\0".as_ptr());
    sigma_keyboard_init();
    serial_puts(b"[HAL] PS/2 Input Handler Active.\n\0".as_ptr());

    // 5. Initialize Subsystems
    process_manager_init();
    ipc_init();
    asched_init();
    serial_puts(b"[SYS] Process Manager, IPC, and Schedulers loaded.\n\0".as_ptr());

    // 6. Finalise Boot
    sigma_rollback_mark_boot_successful();
    vga.write_str("System Ready. Waiting for input...\n");

    // Enable CPU Interrupts
    core::arch::asm!("sti");

    // Idle / Event Loop
    loop {
        let c = sigma_keyboard_read();
        if c != '\0' {
            vga_putc(c, 0x0A); // Echo input in green
            serial_puts(b"Key event intercepted\n\0".as_ptr());
        }
        core::arch::asm!("hlt");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        serial_puts(b"[KERNEL PANIC] Sovereign execution halted.\n\0".as_ptr());
    }
    loop {
        unsafe {
            core::arch::asm!("cli", "hlt");
        }
    }
}
