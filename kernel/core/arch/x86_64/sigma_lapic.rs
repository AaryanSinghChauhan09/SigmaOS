/// SigmaOS: sigma_lapic module
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

// ─── LAPIC Constants (BUG-001 Fix) ─────────────────────────────────────────

const LAPIC_BASE: SigmaU64 = 0xFEE00000;
const LAPIC_ID: usize = 0x020;
const LAPIC_VER: usize = 0x030;
const LAPIC_TPR: usize = 0x080;
const LAPIC_APR: usize = 0x090;
const LAPIC_PPR: usize = 0x0A0;
const LAPIC_EOI: usize = 0x0B0;
const LAPIC_LDR: usize = 0x0D0;
const LAPIC_DFR: usize = 0x0E0;
const LAPIC_SVR: usize = 0x0F0;
const LAPIC_ISR: usize = 0x100;
const LAPIC_TMR: usize = 0x180;
const LAPIC_IRR: usize = 0x200;
const LAPIC_ESR: usize = 0x280;
const LAPIC_ICR_LOW: usize = 0x300;
const LAPIC_ICR_HIGH: usize = 0x310;
const LAPIC_TIMER_LVT: usize = 0x320;
const LAPIC_TIMER_INITIAL: usize = 0x380;
const LAPIC_TIMER_CURRENT: usize = 0x390;
const LAPIC_TIMER_DIV: usize = 0x3E0;

const LAPIC_SVR_ENABLE: SigmaU32 = 0x100;
const LAPIC_ICR_DELIVERY_STATUS: SigmaU32 = 1 << 12;
const LAPIC_ICR_LEVEL: SigmaU32 = 1 << 14;
const LAPIC_ICR_ASSERT: SigmaU32 = 1 << 15;
const LAPIC_ICR_INIT: SigmaU32 = 0x500;
const LAPIC_ICR_STARTUP: SigmaU32 = 0x600;

static mut LAPIC_BASE_ADDR: SigmaU64 = 0;

// ─── LAPIC Functions (BUG-001 Fix) ─────────────────────────────────────────

#[inline(always)]
unsafe fn lapic_read32(offset: usize) -> SigmaU32 {
    let addr = LAPIC_BASE_ADDR + offset as SigmaU64;
    let ptr = addr as *const SigmaU32;
    ptr.read_volatile()
}

#[inline(always)]
unsafe fn lapic_write32(offset: usize, value: SigmaU32) {
    let addr = LAPIC_BASE_ADDR + offset as SigmaU64;
    let ptr = addr as *mut SigmaU32;
    ptr.write_volatile(value);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_lapic_set_base(base: SigmaU64) {
    LAPIC_BASE_ADDR = base;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_lapic_enable() {
    if LAPIC_BASE_ADDR == 0 {
        return;
    }

    // Set spurious interrupt vector and enable APIC
    let svr = lapic_read32(LAPIC_SVR);
    lapic_write32(LAPIC_SVR, svr | LAPIC_SVR_ENABLE | 0xFF);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_lapic_eoi() {
    if LAPIC_BASE_ADDR == 0 {
        return;
    }
    lapic_write32(LAPIC_EOI, 0);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_lapic_timer_init(divisor: SigmaU32, initial: SigmaU32, vector: SigmaU32) {
    if LAPIC_BASE_ADDR == 0 {
        return;
    }

    // Set divisor
    lapic_write32(LAPIC_TIMER_DIV, divisor);

    // Set initial count
    lapic_write32(LAPIC_TIMER_INITIAL, initial);

    // Set timer LVT (one-shot mode)
    lapic_write32(LAPIC_TIMER_LVT, vector);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_lapic_timer_calibrate() -> SigmaU64 {
    if LAPIC_BASE_ADDR == 0 {
        return 0;
    }

    // Simple calibration using PIT
    // For now, return a reasonable default
    1000000 // 1ms in APIC ticks (approximate)
}

#[no_mangle]
pub unsafe extern "C" fn lapic_wait_icr_idle() {
    if LAPIC_BASE_ADDR == 0 {
        return;
    }

    while lapic_read32(LAPIC_ICR_LOW) & LAPIC_ICR_DELIVERY_STATUS != 0 {
        asm!("nop", options(nostack, nomem));
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_lapic_send_ipi(apic_id: SigmaU32, vector: SigmaU32) {
    if LAPIC_BASE_ADDR == 0 {
        return;
    }

    lapic_wait_icr_idle();

    // Set destination
    lapic_write32(LAPIC_ICR_HIGH, (apic_id << 24) as SigmaU32);

    // Send IPI
    lapic_write32(LAPIC_ICR_LOW, vector | LAPIC_ICR_LEVEL | LAPIC_ICR_ASSERT);

    lapic_wait_icr_idle();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_lapic_broadcast_ipi(vector: SigmaU32) {
    if LAPIC_BASE_ADDR == 0 {
        return;
    }

    lapic_wait_icr_idle();

    // Set shorthand to all excluding self
    lapic_write32(LAPIC_ICR_HIGH, 0xC0000000);

    // Send IPI
    lapic_write32(LAPIC_ICR_LOW, vector | LAPIC_ICR_LEVEL | LAPIC_ICR_ASSERT);

    lapic_wait_icr_idle();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_lapic_register_cpu() -> SigmaU32 {
    if LAPIC_BASE_ADDR == 0 {
        return 0;
    }

    // Read APIC ID
    let apic_id = lapic_read32(LAPIC_ID) >> 24;
    apic_id
}

#[no_mangle]
pub unsafe extern "C" fn sigma_lapic_get_id() -> SigmaU32 {
    if LAPIC_BASE_ADDR == 0 {
        return 0;
    }

    let apic_id = lapic_read32(LAPIC_ID) >> 24;
    apic_id
}

#[no_mangle]
pub unsafe extern "C" fn sigma_lapic_get_version() -> SigmaU32 {
    if LAPIC_BASE_ADDR == 0 {
        return 0;
    }

    lapic_read32(LAPIC_VER)
}

