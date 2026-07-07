//! SigmaOS — Interrupt Request (IRQ) Manager (APIC / PIC)
//! Handles legacy 8259 PIC disable and APIC/IOAPIC initialization.
//! No std, no allocator.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U32 = u32;
type U64 = u64;
type Usize = usize;

// ── Legacy PIC Constants ────────────────────────────────────────────────────
const PIC1_CMD:  u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD:  u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;
const ICW4_8086: u8 = 0x01;

// ── APIC Constants ──────────────────────────────────────────────────────────
const APIC_ID:           Usize = 0x020;
const APIC_VERSION:      Usize = 0x030;
const APIC_TPR:          Usize = 0x080;
const APIC_EOI:          Usize = 0x0B0;
const APIC_SIVR:         Usize = 0x0F0;
const APIC_ESR:          Usize = 0x280;
const APIC_ICR_LOW:      Usize = 0x300;
const APIC_ICR_HIGH:     Usize = 0x310;
const APIC_LVT_TIMER:    Usize = 0x320;
const APIC_LVT_THERMAL:  Usize = 0x330;
const APIC_LVT_PERF:     Usize = 0x340;
const APIC_LVT_LINT0:    Usize = 0x350;
const APIC_LVT_LINT1:    Usize = 0x360;
const APIC_LVT_ERROR:    Usize = 0x370;
const APIC_TIMER_INIT:   Usize = 0x380;
const APIC_TIMER_CUR:    Usize = 0x390;
const APIC_TIMER_DIV:    Usize = 0x3E0;

// Spurious Interrupt Vector Register bits
const APIC_SIVR_ENABLE: U32 = 0x100;

// ── IOAPIC Constants ────────────────────────────────────────────────────────
const IOAPIC_REG_ID:      U32 = 0x00;
const IOAPIC_REG_VER:     U32 = 0x01;
const IOAPIC_REG_ARB:     U32 = 0x02;
const IOAPIC_REG_RED_TBL: U32 = 0x10;

// ── Port I/O Helpers ────────────────────────────────────────────────────────
#[inline]
unsafe fn outb(port: u16, val: u8) {
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let mut ret: u8;
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("in al, dx", out("al") ret, in("dx") port, options(nomem, nostack, preserves_flags));
    #[cfg(not(target_arch = "x86_64"))]
    { ret = 0; }
    ret
}

#[inline]
unsafe fn io_wait() {
    outb(0x80, 0);
}

// ── State ───────────────────────────────────────────────────────────────────
pub struct IrqController {
    pub apic_base: U64,
    pub ioapic_base: U64,
    pub spurious_vector: U8,
    pub initialized: bool,
}

static mut IRQ_CTRL: IrqController = IrqController {
    apic_base: 0,
    ioapic_base: 0,
    spurious_vector: 0xFF,
    initialized: false,
};

// ── Memory Mapped I/O ───────────────────────────────────────────────────────
unsafe fn apic_read(offset: Usize) -> U32 {
    let ptr = (IRQ_CTRL.apic_base as Usize + offset) as *const U32;
    core::ptr::read_volatile(ptr)
}

unsafe fn apic_write(offset: Usize, val: U32) {
    let ptr = (IRQ_CTRL.apic_base as Usize + offset) as *mut U32;
    core::ptr::write_volatile(ptr, val);
}

unsafe fn ioapic_read(reg: U32) -> U32 {
    let base = IRQ_CTRL.ioapic_base as *mut U32;
    core::ptr::write_volatile(base, reg);
    core::ptr::read_volatile(base.add(4))
}

unsafe fn ioapic_write(reg: U32, val: U32) {
    let base = IRQ_CTRL.ioapic_base as *mut U32;
    core::ptr::write_volatile(base, reg);
    core::ptr::write_volatile(base.add(4), val);
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Disable legacy 8259 PICs.
#[no_mangle]
pub unsafe extern "C" fn sigma_irq_disable_pic() {
    // Start initialization sequence in cascade mode
    outb(PIC1_CMD, ICW1_INIT | ICW1_ICW4);
    io_wait();
    outb(PIC2_CMD, ICW1_INIT | ICW1_ICW4);
    io_wait();

    // Set vector offsets to something we don't use (e.g. 0x20 and 0x28)
    outb(PIC1_DATA, 0x20);
    io_wait();
    outb(PIC2_DATA, 0x28);
    io_wait();

    // Cascade setup
    outb(PIC1_DATA, 4); // Tell Master PIC that there is a slave PIC at IRQ2
    io_wait();
    outb(PIC2_DATA, 2); // Tell Slave PIC its cascade identity
    io_wait();

    // 8086/88 mode
    outb(PIC1_DATA, ICW4_8086);
    io_wait();
    outb(PIC2_DATA, ICW4_8086);
    io_wait();

    // Mask all interrupts
    outb(PIC1_DATA, 0xFF);
    io_wait();
    outb(PIC2_DATA, 0xFF);
}

/// Initialize APIC and IOAPIC.
#[no_mangle]
pub unsafe extern "C" fn sigma_irq_init_apic(apic_base: U64, ioapic_base: U64, spurious_vec: U8) -> i32 {
    if apic_base == 0 || ioapic_base == 0 { return -1; }

    sigma_irq_disable_pic();

    IRQ_CTRL.apic_base = apic_base;
    IRQ_CTRL.ioapic_base = ioapic_base;
    IRQ_CTRL.spurious_vector = spurious_vec;

    // Enable Local APIC
    let siv = apic_read(APIC_SIVR);
    apic_write(APIC_SIVR, siv | APIC_SIVR_ENABLE | (spurious_vec as U32));

    // Clear Error Status Register
    apic_write(APIC_ESR, 0);

    // Acknowledge any outstanding interrupts
    apic_write(APIC_EOI, 0);

    // Send INIT IPI to all excluding self
    apic_write(APIC_ICR_HIGH, 0);
    apic_write(APIC_ICR_LOW, 0x000C4500); // Destination: All excluding self, Delivery Mode: INIT

    // Mask all IOAPIC interrupts initially
    let ver = ioapic_read(IOAPIC_REG_VER);
    let max_intr = ((ver >> 16) & 0xFF) + 1;
    for i in 0..max_intr {
        let reg = IOAPIC_REG_RED_TBL + (i * 2);
        ioapic_write(reg, 0x10000); // Mask bit set
        ioapic_write(reg + 1, 0);
    }

    IRQ_CTRL.initialized = true;
    0
}

/// Enable a specific IRQ line on the IOAPIC and route it to CPU 0.
#[no_mangle]
pub unsafe extern "C" fn sigma_irq_enable_irq(irq: U32, vector: U8) -> i32 {
    if !IRQ_CTRL.initialized { return -1; }

    let ver = ioapic_read(IOAPIC_REG_VER);
    let max_intr = ((ver >> 16) & 0xFF) + 1;

    if irq >= max_intr { return -2; }

    let reg = IOAPIC_REG_RED_TBL + (irq * 2);

    // Write lower 32 bits: vector, unmasked, fixed delivery, edge triggered, active high
    let low = vector as U32;
    ioapic_write(reg, low);

    // Write upper 32 bits: destination APIC ID 0 (CPU 0)
    let high = 0; // Target APIC ID 0
    ioapic_write(reg + 1, high);

    0
}

/// Send End of Interrupt (EOI) to Local APIC.
#[no_mangle]
pub unsafe extern "C" fn sigma_irq_eoi() {
    if IRQ_CTRL.initialized {
        apic_write(APIC_EOI, 0);
    }
}
