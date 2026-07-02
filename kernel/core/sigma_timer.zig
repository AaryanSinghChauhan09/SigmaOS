// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/core/sigma_timer.zig — HPET/APIC Timer → jiffies clock
// Language: Zig — direct MMIO, comptime register layout, no libc

// ── HPET Register Offsets ─────────────────────────────────────────────────────
const HPET_CAPS:       usize = 0x000;
const HPET_CONFIG:     usize = 0x010;
const HPET_INT_STATUS: usize = 0x020;
const HPET_COUNTER:    usize = 0x0F0;
const HPET_T0_CONFIG:  usize = 0x100;
const HPET_T0_COMPARE: usize = 0x108;
const HPET_T0_ROUTE:   usize = 0x110;

// ── HPET Config bits ──────────────────────────────────────────────────────────
const HPET_ENABLE:     u64 = 1 << 0;
const HPET_LEG_ROUTE:  u64 = 1 << 1;

const T0_INT_ENB:      u64 = 1 << 2;
const T0_TYPE_PERIOD:  u64 = 1 << 3; // periodic mode
const T0_32BIT:        u64 = 1 << 8;
const T0_SETVAL:       u64 = 1 << 6; // write to load initial comparator

// ── APIC Timer Registers ──────────────────────────────────────────────────────
const APIC_BASE:      usize = 0xFEE0_0000;
const APIC_LVT_TIMER: usize = 0x320;
const APIC_INIT_CNT:  usize = 0x380;
const APIC_CURR_CNT:  usize = 0x390;
const APIC_DIV_CFG:   usize = 0x3E0;

// ── Jiffies Counter (incremented by timer ISR) ────────────────────────────────
pub var jiffies: u64 = 0;
pub var ticks_per_ms: u64 = 1; // calibrated against HPET

// ── Timer Driver ──────────────────────────────────────────────────────────────
pub const TimerDriver = struct {
    hpet_base:   usize,
    hpet_period: u64, // femtoseconds per tick
    apic_vec:    u8,

    pub fn init(hpet_mmio: usize, apic_irq_vec: u8) TimerDriver {
        return TimerDriver{
            .hpet_base   = hpet_mmio,
            .hpet_period = 0,
            .apic_vec    = apic_irq_vec,
        };
    }

    /// Initialise HPET and extract clock period
    pub fn init_hpet(self: *TimerDriver) bool {
        const caps = self.hpet_read(HPET_CAPS);
        const period_fs = caps >> 32; // counter tick period in femtoseconds
        if (period_fs == 0 or period_fs > 100_000_000) return false;
        self.hpet_period = period_fs;

        // Disable HPET before configuration
        self.hpet_write(HPET_CONFIG, 0);
        // Reset main counter
        self.hpet_write(HPET_COUNTER, 0);

        // Configure Timer 0 as periodic, 10 ms interval
        const interval_fs: u64 = 10_000_000_000_000; // 10ms in femtoseconds
        const interval_ticks = interval_fs / period_fs;
        var t0cfg: u64 = T0_INT_ENB | T0_TYPE_PERIOD | T0_SETVAL;
        self.hpet_write(HPET_T0_CONFIG, t0cfg);
        self.hpet_write(HPET_T0_COMPARE, interval_ticks);

        // Enable HPET
        self.hpet_write(HPET_CONFIG, HPET_ENABLE);

        // Calibrate APIC timer against HPET
        self.calibrate_apic();
        return true;
    }

    /// Calibrate APIC timer: run it for 10ms (1 HPET interval) and measure
    fn calibrate_apic(self: *TimerDriver) void {
        // Set APIC divider to 1
        apic_write(APIC_DIV_CFG, 0x0B); // divide by 1
        // Set initial count to max
        apic_write(APIC_INIT_CNT, 0xFFFF_FFFF);
        // Wait for HPET to tick 10ms (poll main counter)
        const start = self.hpet_read(HPET_COUNTER);
        const target = start + (10_000_000_000_000 / self.hpet_period);
        while (self.hpet_read(HPET_COUNTER) < target) {}
        const apic_elapsed = 0xFFFF_FFFF - apic_read(APIC_CURR_CNT);
        // apic_elapsed ticks = 10ms → ticks_per_ms = apic_elapsed / 10
        ticks_per_ms = apic_elapsed / 10;
        apic_write(APIC_INIT_CNT, 0);
    }

    /// Set APIC timer to fire at interval_ms, vector vec (periodic)
    pub fn set_apic_periodic(self: *TimerDriver, interval_ms: u32, vec: u8) void {
        _ = self;
        apic_write(APIC_DIV_CFG, 0x0B); // divide by 1
        // LVT: periodic (bit 17) | vector
        apic_write(APIC_LVT_TIMER, (1 << 17) | @as(u32, vec));
        apic_write(APIC_INIT_CNT, ticks_per_ms * interval_ms);
    }

    /// Called from timer ISR — increment jiffies, return ms elapsed
    pub fn tick() u64 {
        jiffies += 1;
        return jiffies * 10; // 10ms per tick
    }

    /// Busy-wait for `ms` milliseconds using HPET
    pub fn delay_ms(self: *const TimerDriver, ms: u64) void {
        const target_ticks = ms * 1_000_000_000_000 / self.hpet_period;
        const start = self.hpet_read(HPET_COUNTER);
        while (self.hpet_read(HPET_COUNTER) - start < target_ticks) {}
    }

    pub fn uptime_ms() u64 { return jiffies * 10; }

    fn hpet_read(self: *const TimerDriver, off: usize) u64 {
        const ptr: *const volatile u64 = @ptrFromInt(self.hpet_base + off);
        return ptr.*;
    }
    fn hpet_write(self: *TimerDriver, off: usize, val: u64) void {
        const ptr: *volatile u64 = @ptrFromInt(self.hpet_base + off);
        ptr.* = val;
    }
};

fn apic_read(off: usize) u32 {
    const ptr: *const volatile u32 = @ptrFromInt(APIC_BASE + off);
    return ptr.*;
}
fn apic_write(off: usize, val: u32) void {
    const ptr: *volatile u32 = @ptrFromInt(APIC_BASE + off);
    ptr.* = val;
}
