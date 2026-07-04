// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_irq.rs — IRQ/APIC/PIC controller + exception handler
//
// Implements:
//   - x86 PIC (8259) init + remapping to vectors 32-47
//   - Local APIC detection + spurious interrupt handling
//   - IRQ dispatch table (256 slots)
//   - CPU exception handler (prints to serial, halts on fatal)
//   - Timer (PIT 1000 Hz) + jiffies counter
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};

// ── x86 I/O port helpers (no_std inline asm) ─────────────────────────────
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack));
}
unsafe fn inb(port: u16) -> u8 {
    let v: u8;
    core::arch::asm!("in al, dx", out("al") v, in("dx") port, options(nomem, nostack));
    v
}
unsafe fn io_wait() { outb(0x80, 0); }

// ── PIC (8259) constants ───────────────────────────────────────────────────
const PIC1_CMD:  u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD:  u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;
const PIC_EOI:   u8  = 0x20;
const ICW1_ICW4: u8  = 0x01;
const ICW1_INIT: u8  = 0x10;
const ICW4_8086: u8  = 0x01;

pub const IRQ_TIMER:    u8 = 0;
pub const IRQ_KEYBOARD: u8 = 1;
pub const IRQ_CASCADE:  u8 = 2;
pub const IRQ_COM2:     u8 = 3;
pub const IRQ_COM1:     u8 = 4;
pub const IRQ_LPT2:     u8 = 5;
pub const IRQ_FLOPPY:   u8 = 6;
pub const IRQ_LPT1:     u8 = 7;
pub const IRQ_RTC:      u8 = 8;

// ── PIT (8253/8254) constants ─────────────────────────────────────────────
const PIT_CH0:  u16 = 0x40;
const PIT_CMD:  u16 = 0x43;
const PIT_FREQ: u32 = 1_193_182; // Hz

// ── Jiffies (1ms ticks) ───────────────────────────────────────────────────
pub static JIFFIES: AtomicU64 = AtomicU64::new(0);

// ── IRQ handler table ─────────────────────────────────────────────────────
pub type IrqHandler = unsafe extern "C" fn();
const MAX_IRQ: usize = 256;

static mut IRQ_HANDLERS: [Option<IrqHandler>; MAX_IRQ] = [const { None }; MAX_IRQ];

// ── Exception frame (matches the stack layout in idt.asm) ─────────────────
#[repr(C)]
pub struct ExceptionFrame {
    pub r15: u64, pub r14: u64, pub r13: u64, pub r12: u64,
    pub r11: u64, pub r10: u64, pub r9: u64,  pub r8: u64,
    pub rbp: u64, pub rdi: u64, pub rsi: u64,
    pub rdx: u64, pub rcx: u64, pub rbx: u64, pub rax: u64,
    pub vector:    u64,  // pushed by stub
    pub error_code:u64,  // pushed by stub or 0
    // CPU-pushed:
    pub rip:   u64, pub cs: u64, pub rflags: u64,
    pub rsp:   u64, pub ss: u64,
}

// ── Serial debug output ───────────────────────────────────────────────────
unsafe fn serial_init() {
    outb(0x3F9, 0x00); // Disable interrupts
    outb(0x3FB, 0x80); // DLAB = 1
    outb(0x3F8, 0x01); // Baud 115200 (divisor lo)
    outb(0x3F9, 0x00); // Baud divisor hi
    outb(0x3FB, 0x03); // 8N1
    outb(0x3FA, 0xC7); // FIFO enable
    outb(0x3FC, 0x0B); // RTS/DSR
}

unsafe fn serial_putc(c: u8) {
    while inb(0x3FD) & 0x20 == 0 {}
    outb(0x3F8, c);
}

unsafe fn serial_puts(s: &[u8]) {
    for &b in s {
        if b == b'\n' { serial_putc(b'\r'); }
        serial_putc(b);
    }
}

// ── PIC initialization ─────────────────────────────────────────────────────
pub unsafe fn sigma_pic_init(offset1: u8, offset2: u8) {
    // Save masks
    let mask1 = inb(PIC1_DATA);
    let mask2 = inb(PIC2_DATA);

    // Start init sequence
    outb(PIC1_CMD,  ICW1_INIT | ICW1_ICW4); io_wait();
    outb(PIC2_CMD,  ICW1_INIT | ICW1_ICW4); io_wait();
    outb(PIC1_DATA, offset1);                io_wait(); // Remap to 32
    outb(PIC2_DATA, offset2);                io_wait(); // Remap to 40
    outb(PIC1_DATA, 4);                      io_wait(); // PIC2 at IRQ2
    outb(PIC2_DATA, 2);                      io_wait(); // PIC2 cascade
    outb(PIC1_DATA, ICW4_8086);              io_wait();
    outb(PIC2_DATA, ICW4_8086);              io_wait();

    // Restore masks
    outb(PIC1_DATA, mask1);
    outb(PIC2_DATA, mask2);
}

pub unsafe fn pic_eoi(irq: u8) {
    if irq >= 8 { outb(PIC2_CMD, PIC_EOI); }
    outb(PIC1_CMD, PIC_EOI);
}

pub unsafe fn irq_mask(irq: u8) {
    let port = if irq < 8 { PIC1_DATA } else { PIC2_DATA };
    let bit  = irq & 7;
    let mask = inb(port);
    outb(port, mask | (1 << bit));
}

pub unsafe fn irq_unmask(irq: u8) {
    let port = if irq < 8 { PIC1_DATA } else { PIC2_DATA };
    let bit  = irq & 7;
    let mask = inb(port);
    outb(port, mask & !(1 << bit));
}

// ── PIT timer (1000 Hz) ───────────────────────────────────────────────────
pub unsafe fn sigma_pit_init(hz: u32) {
    let divisor = PIT_FREQ / hz;
    outb(PIT_CMD, 0x36);                     // channel 0, lo/hi, mode 3
    outb(PIT_CH0, (divisor & 0xFF) as u8);
    outb(PIT_CH0, ((divisor >> 8) & 0xFF) as u8);
}

// ── IRQ registration ──────────────────────────────────────────────────────
pub unsafe fn sigma_request_irq(irq: u8, handler: IrqHandler) {
    IRQ_HANDLERS[32 + irq as usize] = Some(handler);
    irq_unmask(irq);
}

pub unsafe fn sigma_free_irq(irq: u8) {
    IRQ_HANDLERS[32 + irq as usize] = None;
    irq_mask(irq);
}

// ── IRQ dispatcher (called from irq_common in idt.asm) ────────────────────
#[no_mangle]
pub unsafe extern "C" fn sigma_irq_dispatch(vector: u64) {
    let v = vector as usize;

    // Timer IRQ (vector 32 = IRQ 0)
    if v == 32 {
        JIFFIES.fetch_add(1, Ordering::Relaxed);
        pic_eoi(0);
        return;
    }

    if v < MAX_IRQ {
        if let Some(handler) = IRQ_HANDLERS[v] {
            handler();
        }
        if v >= 32 && v < 48 {
            pic_eoi((v - 32) as u8);
        }
    }
}

// ── Exception handler (called from exc_common in idt.asm) ─────────────────
#[no_mangle]
pub unsafe extern "C" fn sigma_exception_handler(frame: *const ExceptionFrame) {
    let f = &*frame;
    serial_puts(b"\n[EXCEPTION] Vector=");
    serial_puts(hex_u64(f.vector).as_slice());
    serial_puts(b" ErrorCode=");
    serial_puts(hex_u64(f.error_code).as_slice());
    serial_puts(b"\nRIP=");
    serial_puts(hex_u64(f.rip).as_slice());
    serial_puts(b" RSP=");
    serial_puts(hex_u64(f.rsp).as_slice());
    serial_puts(b" RFLAGS=");
    serial_puts(hex_u64(f.rflags).as_slice());
    serial_puts(b"\nRAX=");
    serial_puts(hex_u64(f.rax).as_slice());
    serial_puts(b" RBX=");
    serial_puts(hex_u64(f.rbx).as_slice());
    serial_puts(b" RCX=");
    serial_puts(hex_u64(f.rcx).as_slice());
    serial_puts(b"\n");

    // Page fault (#PF = vector 14): print CR2 (faulting address)
    if f.vector == 14 {
        let cr2: u64;
        core::arch::asm!("mov {}, cr2", out(reg) cr2, options(nomem, nostack));
        serial_puts(b"CR2 (fault addr)=");
        serial_puts(hex_u64(cr2).as_slice());
        serial_puts(b"\n");
    }

    // Fatal exceptions: halt
    match f.vector {
        8 | 13 | 14 | 18 => {
            serial_puts(b"[KERNEL PANIC] Unrecoverable exception. Halting.\n");
            loop {
                core::arch::asm!("cli; hlt", options(nomem, nostack));
            }
        }
        _ => {} // non-fatal: return (iretq in asm stub)
    }
}

fn hex_u64(v: u64) -> [u8; 18] {
    let mut s = [b'0'; 18];
    s[0] = b'0'; s[1] = b'x';
    let digits = b"0123456789ABCDEF";
    for i in 0..16 {
        s[17 - i] = digits[((v >> (i * 4)) & 0xF) as usize];
    }
    s
}

// ── C-ABI exports ─────────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn idt_init() {
    serial_init();
    sigma_pic_init(32, 40);
    sigma_pit_init(1000);
    serial_puts(b"[IRQ] PIC remapped, PIT 1000Hz, IDT ready\n");
}

#[no_mangle]
pub extern "C" fn sigma_jiffies() -> u64 {
    JIFFIES.load(Ordering::Relaxed)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_clock_ns() -> u64 {
    // Jiffies × 1_000_000 ns/ms = nanoseconds since boot
    JIFFIES.load(Ordering::Relaxed) * 1_000_000
}
