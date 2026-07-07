/// SigmaOS: sigma_pic module
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.

#![no_std]
#![allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── PIC Constants ─────────────────────────────────────────────────────

const PIC1_CMD: SigmaU16 = 0x20;
const PIC1_DATA: SigmaU16 = 0x21;
const PIC2_CMD: SigmaU16 = 0xA0;
const PIC2_DATA: SigmaU16 = 0xA1;

const PIC_EOI: SigmaU8 = 0x20;
const PIC_ICW1_ICW4: SigmaU8 = 0x01;
const PIC_ICW1_INIT: SigmaU8 = 0x10;

// ─── I/O Port Functions (BUG-001 Fix) ─────────────────────────────────────

#[inline(always)]
unsafe fn outb(port: SigmaU16, value: SigmaU8) {
    asm!("outb %al, %dx", in("al") value, in("dx") port, options(nostack, nomem));
}

#[inline(always)]
unsafe fn inb(port: SigmaU16) -> SigmaU8 {
    let value: SigmaU8;
    asm!("inb %dx, %al", out("al") value, in("dx") port, options(nostack, nomem));
    value
}

unsafe fn io_wait() {
    // Small delay for PIC
    for _ in 0..100 {
        asm!("nop", options(nostack, nomem));
    }
}

// ─── PIC Initialization (BUG-001 Fix) ─────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_pic_init() {
    // Save current masks
    let pic1_mask = inb(PIC1_DATA);
    let pic2_mask = inb(PIC2_DATA);

    // Start initialization sequence
    outb(PIC1_CMD, PIC_ICW1_INIT | PIC_ICW1_ICW4);
    io_wait();
    outb(PIC2_CMD, PIC_ICW1_INIT | PIC_ICW1_ICW4);
    io_wait();

    // Set offset vectors (IRQ 0-7 -> 32-39, IRQ 8-15 -> 40-47)
    outb(PIC1_DATA, 32);
    io_wait();
    outb(PIC2_DATA, 40);
    io_wait();

    // Configure cascade
    outb(PIC1_DATA, 4);
    io_wait();
    outb(PIC2_DATA, 2);
    io_wait();

    // Set 8086 mode
    outb(PIC1_DATA, 1);
    io_wait();
    outb(PIC2_DATA, 1);
    io_wait();

    // Restore masks
    outb(PIC1_DATA, pic1_mask);
    outb(PIC2_DATA, pic2_mask);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pic_send_eoi(irq: SigmaU8) {
    if irq >= 8 {
        outb(PIC2_CMD, PIC_EOI);
    }
    outb(PIC1_CMD, PIC_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pic_mask_irq(irq: SigmaU8) {
    let port = if irq < 8 { PIC1_DATA } else { PIC2_DATA };
    let irq_mask = 1 << (irq % 8);
    let mask = inb(port) | irq_mask;
    outb(port, mask);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pic_unmask_irq(irq: SigmaU8) {
    let port = if irq < 8 { PIC1_DATA } else { PIC2_DATA };
    let irq_mask = 1 << (irq % 8);
    let mask = inb(port) & !irq_mask;
    outb(port, mask);
}

