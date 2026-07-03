// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/linux_compat/vdso_shim.rs — vDSO shim for linux-compat processes
//
// Provides a minimal virtual dynamic shared object mapped into every
// linux-compat process address space. Linux glibc calls these functions
// directly without a syscall to avoid context-switch overhead.
//
// Functions provided:
//   __vdso_clock_gettime  — reads sigma_clock_ns()
//   __vdso_gettimeofday   — converts sigma_clock_ns() to timeval
//   __vdso_time           — returns Unix seconds
//   __vdso_getcpu         — returns CPU/NUMA node (stub: CPU 0, node 0)
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

// ── POSIX time types ──────────────────────────────────────────────────────
#[repr(C)]
pub struct Timespec {
    pub tv_sec:  i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct Timeval {
    pub tv_sec:  i64,
    pub tv_usec: i64,
}

#[repr(C)]
pub struct Timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime:     i32,
}

// ── Clock IDs ─────────────────────────────────────────────────────────────
const CLOCK_REALTIME:  i32 = 0;
const CLOCK_MONOTONIC: i32 = 1;
const CLOCK_PROCESS_CPUTIME_ID: i32 = 2;
const CLOCK_THREAD_CPUTIME_ID:  i32 = 3;
const CLOCK_BOOTTIME:  i32 = 7;

// ── SigmaOS clock primitive ───────────────────────────────────────────────
extern "C" { fn sigma_clock_ns() -> u64; }

// Unix epoch offset: nanoseconds between 1970-01-01 and boot
// In real kernel: set from RTC at boot. Here: static offset.
static mut BOOT_REALTIME_NS: u64 = 1_751_500_000_000_000_000; // approx 2026-07-03

// ── vDSO implementations ──────────────────────────────────────────────────

/// __vdso_clock_gettime — fast path for clock_gettime()
#[no_mangle]
pub unsafe extern "C" fn __vdso_clock_gettime(
    clk_id: i32,
    tp: *mut Timespec,
) -> i32 {
    if tp.is_null() { return -22; } // EFAULT

    let mono_ns = sigma_clock_ns();

    let abs_ns = match clk_id {
        CLOCK_REALTIME  => BOOT_REALTIME_NS.wrapping_add(mono_ns),
        CLOCK_MONOTONIC | CLOCK_BOOTTIME => mono_ns,
        CLOCK_PROCESS_CPUTIME_ID | CLOCK_THREAD_CPUTIME_ID => mono_ns,
        _ => return -22, // EINVAL
    };

    (*tp).tv_sec  = (abs_ns / 1_000_000_000) as i64;
    (*tp).tv_nsec = (abs_ns % 1_000_000_000) as i64;
    0
}

/// __vdso_gettimeofday — fast path for gettimeofday()
#[no_mangle]
pub unsafe extern "C" fn __vdso_gettimeofday(
    tv: *mut Timeval,
    tz: *mut Timezone,
) -> i32 {
    if !tv.is_null() {
        let abs_ns = BOOT_REALTIME_NS.wrapping_add(sigma_clock_ns());
        (*tv).tv_sec  = (abs_ns / 1_000_000_000) as i64;
        (*tv).tv_usec = ((abs_ns % 1_000_000_000) / 1_000) as i64;
    }
    if !tz.is_null() {
        (*tz).tz_minuteswest = 0;
        (*tz).tz_dsttime     = 0;
    }
    0
}

/// __vdso_time — fast path for time()
#[no_mangle]
pub unsafe extern "C" fn __vdso_time(t: *mut i64) -> i64 {
    let abs_ns = BOOT_REALTIME_NS.wrapping_add(sigma_clock_ns());
    let secs = (abs_ns / 1_000_000_000) as i64;
    if !t.is_null() { *t = secs; }
    secs
}

/// __vdso_getcpu — returns current CPU and NUMA node
#[no_mangle]
pub unsafe extern "C" fn __vdso_getcpu(
    cpu:  *mut u32,
    node: *mut u32,
    _cache: *mut u8,
) -> i32 {
    // TODO: read from per-CPU GSBASE when SMP is wired
    if !cpu.is_null()  { *cpu  = 0; }
    if !node.is_null() { *node = 0; }
    0
}

// ── Set the boot realtime offset (called from kernel init) ────────────────
#[no_mangle]
pub unsafe extern "C" fn vdso_set_boot_time(realtime_ns: u64) {
    BOOT_REALTIME_NS = realtime_ns.saturating_sub(sigma_clock_ns());
}
