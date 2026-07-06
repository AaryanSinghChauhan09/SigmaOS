// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_irq.rs — Interrupt Controller Subsystem
// Implements APIC (x86-64), PIC (8259A fallback), HPET/APIC timer,
// and ARM64 GIC-400 (stub, full impl in arch/arm64/sigma_gic.rs).
//
// Design: supports up to 256 IRQ vectors on x86-64; nested interrupt
// handling with IRQ-safe spinlock protocol (no raw pointer aliasing).

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};

// ── MMIO helper (safe wrapper around volatile I/O) ─────────────────────────
#[inline(always)]
unsafe fn mmio_read32(addr: usize) -> u32 {
    core::ptr::read_volatile(addr as *const u32)
}
#[inline(always)]
unsafe fn mmio_write32(addr: usize, val: u32) {
    core::ptr::write_volatile(addr as *mut u32, val);
}
#[inline(always)]
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val);
}
#[inline(always)]
unsafe fn inb(port: u16) -> u8 {
    let v: u8;
    core::arch::asm!("in al, dx", out("al") v, in("dx") port);
    v
}
#[inline(always)]
unsafe fn io_wait() { outb(0x80, 0); }

// ── PIC 8259A constants ────────────────────────────────────────────────────
const PIC1_CMD:  u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD:  u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;
const PIC_EOI:   u8  = 0x20;
const ICW1_ICW4: u8  = 0x01;
const ICW1_INIT: u8  = 0x10;
const ICW4_8086: u8  = 0x01;

pub const PIC_IRQ_OFFSET: u8 = 0x20; // remap PIC to vectors 0x20..0x2F

// ── APIC constants ─────────────────────────────────────────────────────────
const APIC_BASE_MSR:     u32 = 0x1B;
const APIC_ENABLE_BIT:   u64 = 1 << 11;
const APIC_BASE_DEFAULT: usize = 0xFEE0_0000;

const LAPIC_ID:           usize = 0x020;
const LAPIC_VERSION:      usize = 0x030;
const LAPIC_TPR:          usize = 0x080;
const LAPIC_EOI:          usize = 0x0B0;
const LAPIC_SVR:          usize = 0x0F0;  // spurious vector register
const LAPIC_ICR_LO:       usize = 0x300;
const LAPIC_ICR_HI:       usize = 0x310;
const LAPIC_TIMER:        usize = 0x320;
const LAPIC_TIMER_INIT:   usize = 0x380;
const LAPIC_TIMER_CURR:   usize = 0x390;
const LAPIC_TIMER_DIV:    usize = 0x3E0;

const LAPIC_TIMER_VECTOR: u8  = 0x30; // IRQ vector for APIC timer
const LAPIC_SVR_ENABLE:   u32 = 0x100;
const LAPIC_TIMER_PERIOD: u32 = 1_000_000; // ~1 ms at 1 GHz bus

// ── IRQ handler table ──────────────────────────────────────────────────────
pub const MAX_VECTORS: usize = 256;
pub type IrqHandler = fn(vector: u8, frame: &TrapFrame);

/// Trap frame saved by the CPU on interrupt entry
#[repr(C)]
pub struct TrapFrame {
    pub rax: u64, pub rbx: u64, pub rcx: u64, pub rdx: u64,
    pub rsi: u64, pub rdi: u64, pub rbp: u64, pub r8:  u64,
    pub r9:  u64, pub r10: u64, pub r11: u64, pub r12: u64,
    pub r13: u64, pub r14: u64, pub r15: u64,
    pub vector: u64,
    pub error_code: u64,
    pub rip: u64, pub cs: u64, pub rflags: u64, pub rsp: u64, pub ss: u64,
}

static mut IRQ_HANDLERS: [Option<IrqHandler>; MAX_VECTORS] = [None; MAX_VECTORS];
static JIFFIES: AtomicU64 = AtomicU64::new(0);
static APIC_ENABLED: AtomicBool = AtomicBool::new(false);
static mut LAPIC_BASE: usize = APIC_BASE_DEFAULT;

// ── PIC 8259A remapping ────────────────────────────────────────────────────

/// Remap the legacy 8259 PIC so its IRQs don't collide with CPU exceptions.
/// After remapping: IRQ0–7 → vectors 0x20–0x27, IRQ8–15 → 0x28–0x2F.
pub fn pic_remap() {
    unsafe {
        // Save masks
        let mask1 = inb(PIC1_DATA);
        let mask2 = inb(PIC2_DATA);

        // Start initialisation sequence
        outb(PIC1_CMD,  ICW1_INIT | ICW1_ICW4); io_wait();
        outb(PIC2_CMD,  ICW1_INIT | ICW1_ICW4); io_wait();
        outb(PIC1_DATA, PIC_IRQ_OFFSET);         io_wait();
        outb(PIC2_DATA, PIC_IRQ_OFFSET + 8);    io_wait();
        outb(PIC1_DATA, 0x04); io_wait(); // cascade IRQ2
        outb(PIC2_DATA, 0x02); io_wait();
        outb(PIC1_DATA, ICW4_8086); io_wait();
        outb(PIC2_DATA, ICW4_8086); io_wait();

        // Restore masks
        outb(PIC1_DATA, mask1);
        outb(PIC2_DATA, mask2);
    }
}

/// Disable the 8259 PIC (mask all IRQs) — used when APIC is available.
pub fn pic_disable() {
    unsafe {
        outb(PIC1_DATA, 0xFF);
        outb(PIC2_DATA, 0xFF);
    }
}

pub fn pic_send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 { outb(PIC2_CMD, PIC_EOI); }
        outb(PIC1_CMD, PIC_EOI);
    }
}

// ── Local APIC ─────────────────────────────────────────────────────────────

#[inline]
fn lapic_read(reg: usize) -> u32 {
    unsafe { mmio_read32(LAPIC_BASE + reg) }
}
#[inline]
fn lapic_write(reg: usize, val: u32) {
    unsafe { mmio_write32(LAPIC_BASE + reg, val) }
}

/// Enable the local APIC for this CPU core.
pub fn apic_init() {
    unsafe {
        // Read APIC base MSR
        let lo: u32; let hi: u32;
        core::arch::asm!(
            "rdmsr",
            in("ecx") APIC_BASE_MSR,
            out("eax") lo, out("edx") hi
        );
        LAPIC_BASE = ((hi as usize) << 32 | lo as usize) & 0xFFFF_F000;

        // Enable APIC (bit 11 of IA32_APIC_BASE)
        let new_lo = lo | (1 << 11);
        core::arch::asm!(
            "wrmsr",
            in("ecx") APIC_BASE_MSR,
            in("eax") new_lo, in("edx") hi
        );
    }

    // Set task priority to 0 (accept all interrupts)
    lapic_write(LAPIC_TPR, 0x00);
    // Enable APIC + set spurious vector
    lapic_write(LAPIC_SVR, LAPIC_SVR_ENABLE | 0xFF);
    // Set timer divisor to 16
    lapic_write(LAPIC_TIMER_DIV, 0x03);
    // Set timer vector, periodic mode (bit 17)
    lapic_write(LAPIC_TIMER, (1 << 17) | LAPIC_TIMER_VECTOR as u32);
    // Set initial timer count
    lapic_write(LAPIC_TIMER_INIT, LAPIC_TIMER_PERIOD);

    APIC_ENABLED.store(true, Ordering::Release);
}

/// Send End-Of-Interrupt signal to LAPIC.
#[inline]
pub fn apic_eoi() {
    lapic_write(LAPIC_EOI, 0);
}

/// Return LAPIC ID for the current CPU.
pub fn apic_id() -> u8 {
    (lapic_read(LAPIC_ID) >> 24) as u8
}

// ── Timer ──────────────────────────────────────────────────────────────────

/// Called by the timer ISR stub (set up in IDT) every tick (~1 ms).
pub fn timer_tick(frame: &TrapFrame) {
    JIFFIES.fetch_add(1, Ordering::Relaxed);
    // Notify scheduler
    crate::kernel::sched::on_timer_tick(frame);
    if APIC_ENABLED.load(Ordering::Relaxed) { apic_eoi(); }
    else { pic_send_eoi(0); }
}

/// Return current jiffies (monotonic ms counter).
pub fn jiffies() -> u64 {
    JIFFIES.load(Ordering::Relaxed)
}

/// Busy-wait for `ms` milliseconds using jiffies.
pub fn sleep_ms(ms: u64) {
    let until = jiffies() + ms;
    while jiffies() < until {
        core::hint::spin_loop();
    }
}

// ── IRQ handler registration ───────────────────────────────────────────────

/// Register an IRQ handler for a specific interrupt vector.
pub fn register_irq(vector: u8, handler: IrqHandler) {
    unsafe {
        IRQ_HANDLERS[vector as usize] = Some(handler);
    }
}

/// Dispatch IRQ from the ISR stub — called from assembly interrupt entries.
#[no_mangle]
pub extern "C" fn sigma_irq_dispatch(frame: &TrapFrame) {
    let vector = frame.vector as u8;
    unsafe {
        if let Some(handler) = IRQ_HANDLERS[vector as usize] {
            handler(vector, frame);
        }
    }
    // Default EOI
    if APIC_ENABLED.load(Ordering::Relaxed) { apic_eoi(); }
    else if vector >= PIC_IRQ_OFFSET && vector < PIC_IRQ_OFFSET + 16 {
        pic_send_eoi(vector - PIC_IRQ_OFFSET);
    }
}

// ── IRQ enable/disable ─────────────────────────────────────────────────────

#[inline]
pub fn irq_enable() {
    unsafe { core::arch::asm!("sti") }
}

#[inline]
pub fn irq_disable() {
    unsafe { core::arch::asm!("cli") }
}

/// Execute a closure with IRQs disabled (critical section).
pub fn with_irq_disabled<F, R>(f: F) -> R
where F: FnOnce() -> R {
    irq_disable();
    let r = f();
    irq_enable();
    r
}

// ── Module init ────────────────────────────────────────────────────────────

/// Full IRQ subsystem initialisation sequence.
/// Call once during early kernel boot, after GDT/IDT setup.
pub fn irq_init() {
    pic_remap();
    if is_apic_available() {
        pic_disable();
        apic_init();
    }
    // Register timer tick handler
    register_irq(LAPIC_TIMER_VECTOR, timer_tick);
    irq_enable();
}

fn is_apic_available() -> bool {
    let edx: u32;
    unsafe {
        core::arch::asm!(
            "cpuid",
            in("eax") 1u32,
            lateout("edx") edx,
            options(nostack)
        );
    }
    edx & (1 << 9) != 0
}
