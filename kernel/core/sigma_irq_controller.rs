// SigmaOS — IRQ / Interrupt Controller (Issue #1006)
// x86-64 APIC + GIC-400 (ARM64) interrupt management.
// No external dependencies — sovereign implementation.
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// ─── x86-64 IDT ──────────────────────────────────────────────────────────────

pub const IDT_ENTRIES: usize = 256;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    offset_low:  u16,
    selector:    u16,
    ist:         u8,
    type_attr:   u8,
    offset_mid:  u16,
    offset_high: u32,
    zero:        u32,
}

impl IdtEntry {
    pub const fn new() -> Self {
        IdtEntry {
            offset_low: 0, selector: 0, ist: 0,
            type_attr: 0, offset_mid: 0,
            offset_high: 0, zero: 0,
        }
    }

    pub fn set_handler(&mut self, handler: u64, selector: u16, gate_type: u8, dpl: u8) {
        self.offset_low  = (handler & 0xFFFF) as u16;
        self.offset_mid  = ((handler >> 16) & 0xFFFF) as u16;
        self.offset_high = (handler >> 32) as u32;
        self.selector    = selector;
        self.ist         = 0;
        self.type_attr   = 0x80 | ((dpl & 0x3) << 5) | (gate_type & 0x1F);
        self.zero        = 0;
    }
}

#[repr(C, packed)]
pub struct IdtDescriptor {
    pub limit: u16,
    pub base:  u64,
}

pub static mut IDT: [IdtEntry; IDT_ENTRIES] = [IdtEntry::new(); IDT_ENTRIES];
static mut IDT_DESC: IdtDescriptor = IdtDescriptor { limit: 0, base: 0 };

pub fn idt_load() {
    unsafe {
        IDT_DESC.limit = (core::mem::size_of::<[IdtEntry; IDT_ENTRIES]>() - 1) as u16;
        IDT_DESC.base  = IDT.as_ptr() as u64;
        core::arch::asm!("lidt [{0}]", in(reg) &IDT_DESC, options(nostack));
    }
}

pub fn idt_set_gate(vec: u8, handler: u64) {
    unsafe {
        IDT[vec as usize].set_handler(handler, 0x08, 0x0E, 0); // kernel code seg, 64-bit interrupt gate
    }
}

// ─── APIC Registers (memory-mapped) ─────────────────────────────────────────

pub const APIC_BASE_DEFAULT: u64 = 0xFEE0_0000;
pub const APIC_ID:           u32 = 0x020;
pub const APIC_VERSION:      u32 = 0x030;
pub const APIC_TPR:          u32 = 0x080;
pub const APIC_EOI:          u32 = 0x0B0;
pub const APIC_LDR:          u32 = 0x0D0;
pub const APIC_SVR:          u32 = 0x0F0;  // Spurious Vector Register
pub const APIC_ESR:          u32 = 0x280;
pub const APIC_ICR_LOW:      u32 = 0x300;
pub const APIC_ICR_HIGH:     u32 = 0x310;
pub const APIC_LVTT:         u32 = 0x320;  // Timer LVT
pub const APIC_LINT0:        u32 = 0x350;
pub const APIC_LINT1:        u32 = 0x360;
pub const APIC_TIMER_INIT:   u32 = 0x380;
pub const APIC_TIMER_CUR:    u32 = 0x390;
pub const APIC_TIMER_DIV:    u32 = 0x3E0;

static mut APIC_BASE: u64 = APIC_BASE_DEFAULT;

fn apic_read(reg: u32) -> u32 {
    unsafe {
        let ptr = (APIC_BASE + reg as u64) as *const u32;
        ptr.read_volatile()
    }
}

fn apic_write(reg: u32, val: u32) {
    unsafe {
        let ptr = (APIC_BASE + reg as u64) as *mut u32;
        ptr.write_volatile(val);
    }
}

pub fn apic_init(base: u64) {
    unsafe { APIC_BASE = base; }
    // Enable APIC: SVR bit 8 = software enable, vector 0xFF = spurious
    apic_write(APIC_SVR, apic_read(APIC_SVR) | 0x100 | 0xFF);
    // Mask LINT0/LINT1
    apic_write(APIC_LINT0, 1 << 16);
    apic_write(APIC_LINT1, 1 << 16);
    // Set timer: divide by 16
    apic_write(APIC_TIMER_DIV, 0x3);
    // Set timer LVT: vector 0x20, periodic
    apic_write(APIC_LVTT, 0x20 | (1 << 17));
    // Initial count — calibrate based on PIT, assume 1 GHz bus → 62500 for 1ms
    apic_write(APIC_TIMER_INIT, 62500);
}

/// Signal EOI to local APIC.
pub fn apic_eoi() {
    apic_write(APIC_EOI, 0);
}

/// Send IPI to APIC id `dest`.
pub fn apic_send_ipi(dest: u8, vector: u8) {
    apic_write(APIC_ICR_HIGH, (dest as u32) << 24);
    apic_write(APIC_ICR_LOW,  vector as u32 | (1 << 14)); // assert, fixed delivery
}

// ─── I/O APIC ────────────────────────────────────────────────────────────────

pub const IOAPIC_BASE_DEFAULT: u64 = 0xFEC0_0000;
pub const IOAPIC_ID:    u8 = 0x00;
pub const IOAPIC_VER:   u8 = 0x01;
pub const IOAPIC_REDTBL: u8 = 0x10;

static mut IOAPIC_BASE: u64 = IOAPIC_BASE_DEFAULT;

fn ioapic_read(reg: u8) -> u32 {
    unsafe {
        let base = IOAPIC_BASE;
        let ioregsel = base as *mut u32;
        let iowin = (base + 0x10) as *mut u32;
        ioregsel.write_volatile(reg as u32);
        iowin.read_volatile()
    }
}

fn ioapic_write(reg: u8, val: u32) {
    unsafe {
        let base = IOAPIC_BASE;
        let ioregsel = base as *mut u32;
        let iowin = (base + 0x10) as *mut u32;
        ioregsel.write_volatile(reg as u32);
        iowin.write_volatile(val);
    }
}

pub fn ioapic_init(base: u64) {
    unsafe { IOAPIC_BASE = base; }
}

/// Map IRQ line to vector and destination APIC.
pub fn ioapic_map_irq(irq: u8, vector: u8, dest_apic: u8) {
    let idx = IOAPIC_REDTBL + 2 * irq;
    // Low: vector, fixed delivery, active-high, edge-triggered, not masked
    ioapic_write(idx,     vector as u32);
    // High: destination
    ioapic_write(idx + 1, (dest_apic as u32) << 24);
}

/// Mask an IRQ line.
pub fn ioapic_mask_irq(irq: u8) {
    let idx = IOAPIC_REDTBL + 2 * irq;
    let lo = ioapic_read(idx);
    ioapic_write(idx, lo | (1 << 16));
}

pub fn ioapic_unmask_irq(irq: u8) {
    let idx = IOAPIC_REDTBL + 2 * irq;
    let lo = ioapic_read(idx);
    ioapic_write(idx, lo & !(1 << 16));
}

// ─── IRQ Handler Table ────────────────────────────────────────────────────────

pub type IrqHandler = fn(vector: u8);

static mut IRQ_HANDLERS: [Option<IrqHandler>; IDT_ENTRIES] = [None; IDT_ENTRIES];
static IRQ_COUNTS: [AtomicU64; IDT_ENTRIES] = {
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; IDT_ENTRIES]
};

pub fn register_irq_handler(vector: u8, handler: IrqHandler) {
    unsafe { IRQ_HANDLERS[vector as usize] = Some(handler); }
}

/// Dispatch an IRQ — called from assembly stub.
#[no_mangle]
pub extern "C" fn sigma_irq_dispatch(vector: u8) {
    IRQ_COUNTS[vector as usize].fetch_add(1, Ordering::Relaxed);
    let h = unsafe { IRQ_HANDLERS[vector as usize] };
    if let Some(handler) = h {
        handler(vector);
    }
    apic_eoi();
}

/// ARM GIC-400 support (basic)
pub mod gic {
    pub const GIC_DIST_BASE: u64 = 0x0800_0000;
    pub const GIC_CPU_BASE:  u64 = 0x0801_0000;

    pub fn gic_init() {
        unsafe {
            // Distributor: enable group 0
            let dist = GIC_DIST_BASE as *mut u32;
            dist.write_volatile(1);
            // CPU interface: enable, priority mask 0xFF
            let cpu = GIC_CPU_BASE as *mut u32;
            cpu.add(1).write_volatile(0xFF); // priority mask
            cpu.write_volatile(1);           // enable
        }
    }

    pub fn gic_eoi(irq: u32) {
        unsafe {
            let cpu = (GIC_CPU_BASE + 0x10) as *mut u32;
            cpu.write_volatile(irq);
        }
    }

    pub fn gic_enable_irq(irq: u32) {
        unsafe {
            let en_reg = (GIC_DIST_BASE + 0x100 + (irq / 32) as u64 * 4) as *mut u32;
            let val = en_reg.read_volatile();
            en_reg.write_volatile(val | (1 << (irq % 32)));
        }
    }
}

pub fn sigma_irq_stats() -> u64 {
    IRQ_COUNTS.iter().map(|c| c.load(Ordering::Relaxed)).sum()
}
