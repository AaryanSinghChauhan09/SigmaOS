/// SigmaOS: HPET/APIC Timer Implementation
/// Phase G Blocker #6: HPET/APIC Timer
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.

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

// ─── HPET Constants ─────────────────────────────────────────────────────

const HPET_BASE: SigmaU64 = 0xFED00000;
const HPET_CAPABILITY: usize = 0x000;
const HPET_CONFIGURATION: usize = 0x010;
const HPET_INTERRUPT_STATUS: usize = 0x020;
const HPET_MAIN_COUNTER: usize = 0x0F0;
const HPET_PERIOD: usize = 0x004;

const HPET_ENABLE: SigmaU64 = 1 << 0;
const HPET_LEGACY_RT_CNF: SigmaU64 = 1 << 1;

// ─── Timer Wheel Constants ───────────────────────────────────────────────

const TIMER_WHEEL_SIZE: usize = 256;
const TIMER_WHEEL_MASK: usize = TIMER_WHEEL_SIZE - 1;

// ─── HPET Timer ─────────────────────────────────────────────────────────

pub struct HpetTimer {
    base: SigmaU64,
    enabled: SigmaBool,
    period: SigmaU64,
}

impl HpetTimer {
    pub const fn new() -> Self {
        Self {
            base: 0,
            enabled: false,
            period: 0,
        }
    }

    /// Initialize HPET timer
    pub unsafe fn init(&mut self) -> Result<(), &'static str> {
        self.base = HPET_BASE;

        // Read period
        let period_ptr = (self.base + HPET_PERIOD as SigmaU64) as *const SigmaU64;
        self.period = period_ptr.read_volatile();

        if self.period == 0 {
            return Err("Invalid HPET period");
        }

        // Enable HPET
        let config_ptr = (self.base + HPET_CONFIGURATION as SigmaU64) as *mut SigmaU64;
        let config = config_ptr.read_volatile();
        config_ptr.write_volatile(config | HPET_ENABLE);

        self.enabled = true;
        Ok(())
    }

    /// Read HPET counter
    pub unsafe fn read(&self) -> SigmaU64 {
        if !self.enabled {
            return 0;
        }

        let counter_ptr = (self.base + HPET_MAIN_COUNTER as SigmaU64) as *const SigmaU64;
        counter_ptr.read_volatile()
    }

    /// Convert counter value to nanoseconds
    pub fn to_ns(&self, counter: SigmaU64) -> SigmaU64 {
        (counter * self.period) / 1000000
    }

    /// Convert nanoseconds to counter value
    pub fn from_ns(&self, ns: SigmaU64) -> SigmaU64 {
        (ns * 1000000) / self.period
    }

    /// Sleep for specified nanoseconds
    pub unsafe fn sleep_ns(&self, ns: SigmaU64) {
        if !self.enabled {
            return;
        }

        let start = self.read();
        let target = self.from_ns(ns);

        while self.read() - start < target {
            asm!("nop", options(nostack, nomem));
        }
    }

    /// Sleep for specified milliseconds
    pub unsafe fn sleep_ms(&self, ms: SigmaU64) {
        self.sleep_ns(ms * 1000000);
    }
}

// ─── Timer Entry ───────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TimerEntry {
    pub callback: Option<unsafe extern "C" fn()>,
    pub expire_time: SigmaU64,
    pub active: SigmaBool,
}

// ─── Timer Wheel ───────────────────────────────────────────────────────

pub struct TimerWheel {
    wheel: [Option<TimerEntry>; TIMER_WHEEL_SIZE],
    current_slot: SigmaUsize,
    tick_count: SigmaU64,
    hpet: HpetTimer,
}

impl TimerWheel {
    pub const fn new() -> Self {
        Self {
            wheel: [None; TIMER_WHEEL_SIZE],
            current_slot: 0,
            tick_count: 0,
            hpet: HpetTimer::new(),
        }
    }

    /// Initialize timer wheel
    pub unsafe fn init(&mut self) -> Result<(), &'static str> {
        // Initialize HPET
        self.hpet.init()?;

        // Clear wheel
        for i in 0..TIMER_WHEEL_SIZE {
            self.wheel[i] = None;
        }

        self.current_slot = 0;
        self.tick_count = 0;

        Ok(())
    }

    /// Add timer to wheel
    pub unsafe fn add_timer(&mut self, callback: unsafe extern "C" fn(), delay_ns: SigmaU64) -> Result<(), &'static str> {
        let expire_time = self.hpet.read() + self.hpet.from_ns(delay_ns);
        let slot = ((expire_time / 1000000) as SigmaUsize) & TIMER_WHEEL_MASK;

        let entry = TimerEntry {
            callback: Some(callback),
            expire_time,
            active: true,
        };

        self.wheel[slot] = Some(entry);
        Ok(())
    }

    /// Timer tick - called by interrupt handler
    pub unsafe fn tick(&mut self) {
        self.tick_count += 1;

        // Check current slot
        if let Some(ref entry) = self.wheel[self.current_slot] {
            if entry.active && self.hpet.read() >= entry.expire_time {
                if let Some(callback) = entry.callback {
                    callback();
                }
                self.wheel[self.current_slot] = None;
            }
        }

        // Move to next slot
        self.current_slot = (self.current_slot + 1) & TIMER_WHEEL_MASK;
    }

    /// Cancel all timers in current slot
    pub unsafe fn cancel(&mut self) {
        self.wheel[self.current_slot] = None;
    }

    /// Get timer status
    pub unsafe fn status(&self) -> (SigmaU64, SigmaUsize, SigmaBool) {
        (self.tick_count, self.current_slot, self.hpet.enabled)
    }

    /// Get current time in nanoseconds
    pub unsafe fn get_time_ns(&self) -> SigmaU64 {
        self.hpet.to_ns(self.hpet.read())
    }

    /// Get current time in milliseconds
    pub unsafe fn get_time_ms(&self) -> SigmaU64 {
        self.get_time_ns() / 1000000
    }
}

// ─── Global Timer Instance ───────────────────────────────────────────────

static mut TIMER_WHEEL: TimerWheel = TimerWheel::new();

#[no_mangle]
pub unsafe extern "C" fn timer_wheel_init() -> SigmaI32 {
    match TIMER_WHEEL.init() {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn timer_add(callback: unsafe extern "C" fn(), delay_ns: SigmaU64) -> SigmaI32 {
    match TIMER_WHEEL.add_timer(callback, delay_ns) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn timer_cancel() {
    TIMER_WHEEL.cancel();
}

#[no_mangle]
pub unsafe extern "C" fn timer_tick() {
    TIMER_WHEEL.tick();
}

#[no_mangle]
pub unsafe extern "C" fn timer_status() -> SigmaU64 {
    let (tick_count, current_slot, enabled) = TIMER_WHEEL.status();
    if enabled {
        tick_count
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn timer_get_time_ns() -> SigmaU64 {
    TIMER_WHEEL.get_time_ns()
}

#[no_mangle]
pub unsafe extern "C" fn timer_get_time_ms() -> SigmaU64 {
    TIMER_WHEEL.get_time_ms()
}

#[no_mangle]
pub unsafe extern "C" fn timer_sleep_ns(ns: SigmaU64) {
    TIMER_WHEEL.hpet.sleep_ns(ns);
}

#[no_mangle]
pub unsafe extern "C" fn timer_sleep_ms(ms: SigmaU64) {
    TIMER_WHEEL.hpet.sleep_ms(ms);
}

