//! SigmaOS — HPET (High Precision Event Timer) Driver
//! Provides nanosecond-resolution timekeeping without external dependencies.
//! Pure no_std, zero-dependency implementation.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U32 = u32;
type U64 = u64;
type Usize = usize;

// ── HPET Register Offsets (MMIO) ────────────────────────────────────────────
const HPET_REG_CAP:        Usize = 0x000; // General Capabilities and ID
const HPET_REG_CONFIG:     Usize = 0x010; // General Configuration
const HPET_REG_INT_STATUS: Usize = 0x020; // General Interrupt Status
const HPET_REG_MAIN_CNT:   Usize = 0x0F0; // Main Counter Value
const HPET_TIMER0_CONFIG:  Usize = 0x100; // Timer 0 Config and Capabilities
const HPET_TIMER0_CMP:     Usize = 0x108; // Timer 0 Comparator Value
const HPET_TIMER1_CONFIG:  Usize = 0x120; // Timer 1 Config and Capabilities
const HPET_TIMER1_CMP:     Usize = 0x128; // Timer 1 Comparator Value
const HPET_TIMER2_CONFIG:  Usize = 0x140; // Timer 2 Config and Capabilities
const HPET_TIMER2_CMP:     Usize = 0x148; // Timer 2 Comparator Value

// ── Configuration bits ──────────────────────────────────────────────────────
const HPET_CFG_ENABLE:        U64 = 1 << 0; // Overall Enable
const HPET_CFG_LEGACY_REPLACE: U64 = 1 << 1; // Legacy Replacement Route

const TIMER_CFG_INT_ENABLE: U64 = 1 << 2;  // Timer Interrupt Enable
const TIMER_CFG_PERIODIC:   U64 = 1 << 3;  // Periodic Mode
const TIMER_CFG_SET_VALUE:  U64 = 1 << 6;  // Set Accumulator (periodic)
const TIMER_CFG_32BIT:      U64 = 1 << 8;  // Force 32-bit mode

// ── Capability register fields ──────────────────────────────────────────────
const CAP_NUM_TIMERS_SHIFT: u32 = 8;
const CAP_NUM_TIMERS_MASK:  U64 = 0x1F;
const CAP_COUNTER_64BIT:    U64 = 1 << 13;
const CAP_LEGACY_CAPABLE:   U64 = 1 << 15;
const CAP_PERIOD_MASK:      U64 = 0xFFFF_FFFF_0000_0000;
const CAP_PERIOD_SHIFT:     u32 = 32;

// ── Driver State ────────────────────────────────────────────────────────────
const MAX_TIMERS: usize = 8;

#[derive(Copy, Clone)]
pub struct HpetTimer {
    pub config_cap: U64,
    pub comparator: U64,
    pub supports_periodic: bool,
    pub supports_64bit: bool,
    pub irq_routing: U32,
}

impl HpetTimer {
    pub const fn empty() -> Self {
        HpetTimer {
            config_cap: 0,
            comparator: 0,
            supports_periodic: false,
            supports_64bit: false,
            irq_routing: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct HpetState {
    pub base_addr: U64,
    pub period_fs: U64,           // Period in femtoseconds
    pub frequency_hz: U64,        // Derived frequency
    pub num_timers: U32,
    pub counter_64bit: bool,
    pub legacy_capable: bool,
    pub enabled: bool,
    pub timers: [HpetTimer; MAX_TIMERS],
}

static mut HPET: HpetState = HpetState {
    base_addr: 0,
    period_fs: 0,
    frequency_hz: 0,
    num_timers: 0,
    counter_64bit: false,
    legacy_capable: false,
    enabled: false,
    timers: [HpetTimer::empty(); MAX_TIMERS],
};

// ── MMIO Helpers ────────────────────────────────────────────────────────────

unsafe fn hpet_read(offset: Usize) -> U64 {
    let addr = HPET.base_addr as *const U64;
    let ptr = (addr as Usize + offset) as *const U64;
    core::ptr::read_volatile(ptr)
}

unsafe fn hpet_write(offset: Usize, value: U64) {
    let addr = HPET.base_addr as *mut U64;
    let ptr = (addr as Usize + offset) as *mut U64;
    core::ptr::write_volatile(ptr, value);
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Initialize the HPET from its MMIO base address (found via ACPI HPET table).
#[no_mangle]
pub unsafe extern "C" fn sigma_hpet_init(mmio_base: U64) -> i32 {
    if mmio_base == 0 {
        return -1; // EINVAL
    }
    HPET.base_addr = mmio_base;

    // Read capabilities
    let cap = hpet_read(HPET_REG_CAP);
    HPET.num_timers = ((cap >> CAP_NUM_TIMERS_SHIFT) & CAP_NUM_TIMERS_MASK) as U32 + 1;
    HPET.counter_64bit = (cap & CAP_COUNTER_64BIT) != 0;
    HPET.legacy_capable = (cap & CAP_LEGACY_CAPABLE) != 0;

    // Extract period in femtoseconds (bits 63:32)
    HPET.period_fs = (cap & CAP_PERIOD_MASK) >> CAP_PERIOD_SHIFT;
    if HPET.period_fs == 0 {
        return -2; // Invalid HPET — period cannot be zero
    }

    // Derive frequency: 10^15 fs / period_fs = Hz
    HPET.frequency_hz = 1_000_000_000_000_000 / HPET.period_fs;

    // Discover per-timer capabilities
    let max = if (HPET.num_timers as usize) < MAX_TIMERS {
        HPET.num_timers as usize
    } else {
        MAX_TIMERS
    };

    for i in 0..max {
        let timer_offset = 0x100 + 0x20 * i;
        let tcap = hpet_read(timer_offset);
        HPET.timers[i].config_cap = tcap;
        HPET.timers[i].supports_periodic = (tcap & (1 << 4)) != 0;
        HPET.timers[i].supports_64bit = (tcap & (1 << 5)) != 0;
        HPET.timers[i].irq_routing = ((tcap >> 32) & 0xFFFF_FFFF) as U32;
    }

    // Disable HPET while configuring
    let mut cfg = hpet_read(HPET_REG_CONFIG);
    cfg &= !HPET_CFG_ENABLE;
    hpet_write(HPET_REG_CONFIG, cfg);

    // Reset main counter
    hpet_write(HPET_REG_MAIN_CNT, 0);

    // Enable legacy replacement routing if available
    if HPET.legacy_capable {
        cfg |= HPET_CFG_LEGACY_REPLACE;
    }

    // Enable HPET
    cfg |= HPET_CFG_ENABLE;
    hpet_write(HPET_REG_CONFIG, cfg);
    HPET.enabled = true;

    0
}

/// Read the current HPET main counter value.
#[no_mangle]
pub unsafe extern "C" fn sigma_hpet_read_counter() -> U64 {
    hpet_read(HPET_REG_MAIN_CNT)
}

/// Convert HPET ticks to nanoseconds.
#[no_mangle]
pub unsafe extern "C" fn sigma_hpet_ticks_to_ns(ticks: U64) -> U64 {
    // ns = ticks * period_fs / 1_000_000
    // Use 128-bit intermediate to avoid overflow
    let period = HPET.period_fs;
    let hi = (ticks >> 32) * period;
    let lo = (ticks & 0xFFFF_FFFF) * period;
    let total_fs = (hi << 32) + lo;
    total_fs / 1_000_000
}

/// Busy-wait sleep for the given number of nanoseconds.
#[no_mangle]
pub unsafe extern "C" fn sigma_hpet_sleep_ns(ns: U64) {
    if HPET.period_fs == 0 || !HPET.enabled {
        return;
    }
    // target_ticks = ns * 1_000_000 / period_fs
    let target_ticks = (ns * 1_000_000) / HPET.period_fs;
    let start = sigma_hpet_read_counter();
    loop {
        let now = sigma_hpet_read_counter();
        if now.wrapping_sub(start) >= target_ticks {
            break;
        }
        core::hint::spin_loop();
    }
}

/// Busy-wait sleep for the given number of milliseconds.
#[no_mangle]
pub unsafe extern "C" fn sigma_hpet_sleep_ms(ms: U32) {
    sigma_hpet_sleep_ns(ms as U64 * 1_000_000);
}

/// Configure Timer 0 as a periodic interrupt at the given Hz.
#[no_mangle]
pub unsafe extern "C" fn sigma_hpet_set_periodic(hz: U32) -> i32 {
    if hz == 0 || !HPET.enabled {
        return -1;
    }
    if !HPET.timers[0].supports_periodic {
        return -2; // Timer 0 does not support periodic mode
    }

    // Disable HPET temporarily
    let mut global_cfg = hpet_read(HPET_REG_CONFIG);
    global_cfg &= !HPET_CFG_ENABLE;
    hpet_write(HPET_REG_CONFIG, global_cfg);

    // ticks_per_interrupt = frequency_hz / hz
    let ticks = HPET.frequency_hz / hz as U64;

    // Configure timer 0: periodic + interrupt enable + set accumulator
    let timer_cfg = TIMER_CFG_INT_ENABLE | TIMER_CFG_PERIODIC | TIMER_CFG_SET_VALUE;
    hpet_write(HPET_TIMER0_CONFIG, timer_cfg);

    // Write comparator value (period)
    hpet_write(HPET_TIMER0_CMP, ticks);

    // Reset and re-enable
    hpet_write(HPET_REG_MAIN_CNT, 0);
    global_cfg |= HPET_CFG_ENABLE;
    hpet_write(HPET_REG_CONFIG, global_cfg);

    0
}

/// Get the HPET frequency in Hz.
#[no_mangle]
pub unsafe extern "C" fn sigma_hpet_frequency() -> U64 {
    HPET.frequency_hz
}

/// Get the number of HPET timers available.
#[no_mangle]
pub unsafe extern "C" fn sigma_hpet_timer_count() -> U32 {
    HPET.num_timers
}
