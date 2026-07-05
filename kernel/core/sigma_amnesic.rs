// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_amnesic.rs — Amnesic Boot Mode (Tails OS-inspired)
//
// Implements the "SigmaAmnesic" secure boot persona:
//   - RAM scrubbing on shutdown (multi-pass volatile zeroing)
//   - Session isolation flags (no persistence, no swap)
//   - Emergency wipe trigger (USB-removal watchdog equivalent)
//   - Session audit log stored only in RAM
//
// Algorithm mirrors Tails OS sdmem + kexec RAM wipe pattern:
//   Pass 1: write 0x00 to every physical page
//   Pass 2: write 0xFF to every physical page
//   Pass 3: write 0x00 to every physical page (final)
//
// Language: Rust #![no_std] — no alloc, no external crates.
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── Constants ─────────────────────────────────────────────────────────────────
/// Maximum length of an audit log entry string.
const LOG_ENTRY_LEN:  SigmaUsize = 128;
/// Maximum number of in-RAM audit log entries per session.
const MAX_LOG_ENTRIES: SigmaUsize = 256;
/// Physical page size in bytes.
const PAGE_SIZE: SigmaUsize = 4096;
/// Number of overwrite passes (matches Tails OS 3-pass zero/FF/zero).
const SCRUB_PASSES: SigmaUsize = 3;

// ── Scrub pass fill values ────────────────────────────────────────────────────
const PASS_PATTERNS: [SigmaU8; SCRUB_PASSES] = [0x00, 0xFF, 0x00];

// ── Global State ──────────────────────────────────────────────────────────────
/// True once sigma_amnesic_init() has been called.
static AMNESIC_ACTIVE:        AtomicBool = AtomicBool::new(false);
/// True if an emergency wipe has been triggered.
static EMERGENCY_WIPE_ACTIVE: AtomicBool = AtomicBool::new(false);
/// Session start timestamp (jiffies).
static SESSION_START:         AtomicU64  = AtomicU64::new(0);
/// Bytes scrubbed so far (updated during scrub for progress queries).
static BYTES_SCRUBBED:        AtomicU64  = AtomicU64::new(0);
/// Number of audit log entries written this session.
static LOG_COUNT:             AtomicU32  = AtomicU32::new(0);

// ── AmnesicState — session metadata ──────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AmnesicState {
    /// True when amnesic mode is active.
    pub active:              SigmaBool,
    /// True when emergency wipe is in progress.
    pub emergency:           SigmaBool,
    pub _pad:                [SigmaU8; 6],
    /// jiffies at session start.
    pub session_start:       SigmaU64,
    /// Bytes scrubbed so far.
    pub bytes_scrubbed:      SigmaU64,
    /// Total bytes to scrub (filled at scrub start).
    pub bytes_total:         SigmaU64,
    /// Number of log entries recorded.
    pub log_count:           SigmaU32,
    pub _pad2:               [SigmaU8; 4],
}

impl AmnesicState {
    pub const fn zeroed() -> Self {
        Self {
            active:         false,
            emergency:      false,
            _pad:           [0u8; 6],
            session_start:  0,
            bytes_scrubbed: 0,
            bytes_total:    0,
            log_count:      0,
            _pad2:          [0u8; 4],
        }
    }
}

static mut G_AMNESIC_STATE: AmnesicState = AmnesicState::zeroed();

// ── In-RAM Audit Log ──────────────────────────────────────────────────────────
/// Single audit log entry — stored only in RAM, never persisted to disk.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AuditEntry {
    pub timestamp: SigmaU64,
    pub msg:       [SigmaU8; LOG_ENTRY_LEN],
}

impl AuditEntry {
    pub const fn zeroed() -> Self {
        Self { timestamp: 0, msg: [0u8; LOG_ENTRY_LEN] }
    }
}

static mut AUDIT_LOG: [AuditEntry; MAX_LOG_ENTRIES] =
    [AuditEntry::zeroed(); MAX_LOG_ENTRIES];

// ── External kernel interfaces ────────────────────────────────────────────────
extern "C" {
    fn sigma_jiffies()         -> SigmaU64;
    fn sigma_mm_total_bytes()  -> SigmaU64;
    fn sigma_mm_phys_base()    -> SigmaU64;
    fn sigma_log(msg: *const SigmaU8, len: SigmaUsize);
    /// Disable swap partition / swap file.
    fn sigma_swap_disable();
    /// Unmount all non-essential filesystems.
    fn sigma_fs_unmount_all();
    /// Trigger system halt after scrub completes.
    fn sigma_system_halt();
}

macro_rules! klog {
    ($msg:expr) => {
        unsafe { sigma_log($msg.as_ptr(), $msg.len()) }
    };
}

// ── Audit Log Helper ─────────────────────────────────────────────────────────
unsafe fn audit_append(msg: &[SigmaU8]) {
    let idx = LOG_COUNT.load(Ordering::Relaxed) as SigmaUsize;
    if idx >= MAX_LOG_ENTRIES { return; }

    let mut entry = AuditEntry::zeroed();
    entry.timestamp = sigma_jiffies();
    let len = msg.len().min(LOG_ENTRY_LEN - 1);
    let mut i = 0;
    while i < len { entry.msg[i] = msg[i]; i += 1; }
    entry.msg[len] = 0;

    core::ptr::write(AUDIT_LOG.as_mut_ptr().add(idx), entry);
    LOG_COUNT.fetch_add(1, Ordering::Relaxed);
}

// ── Volatile memory overwrite ─────────────────────────────────────────────────
/// Write `fill` byte to every byte in the memory range [base, base+size).
/// Uses `write_volatile` to prevent compiler optimization of the scrub.
/// This is the core of the Tails OS sdmem equivalent.
unsafe fn scrub_range(base: SigmaU64, size: SigmaU64, fill: SigmaU8) {
    let ptr = base as *mut SigmaU8;
    let mut offset: SigmaU64 = 0;
    while offset < size {
        core::ptr::write_volatile(ptr.add(offset as SigmaUsize), fill);
        offset += 1;
    }
}

// ── Core Implementation ───────────────────────────────────────────────────────

/// sigma_amnesic_init — activate amnesic boot mode.
///
/// Call early in boot before any userspace process starts.
/// Effects:
///   - Disables swap (no data spills to disk)
///   - Records session start time
///   - Sets AMNESIC_ACTIVE flag
///   - Emits boot audit entry
#[no_mangle]
pub unsafe extern "C" fn sigma_amnesic_init() {
    if AMNESIC_ACTIVE.swap(true, Ordering::SeqCst) {
        klog!(b"[amnesic] already active\n");
        return;
    }

    klog!(b"[amnesic] SigmaAmnesic mode activated\n");
    klog!(b"[amnesic] disabling swap...\n");
    sigma_swap_disable();

    let now = sigma_jiffies();
    SESSION_START.store(now, Ordering::SeqCst);

    G_AMNESIC_STATE = AmnesicState {
        active:         true,
        emergency:      false,
        _pad:           [0u8; 6],
        session_start:  now,
        bytes_scrubbed: 0,
        bytes_total:    sigma_mm_total_bytes(),
        log_count:      0,
        _pad2:          [0u8; 4],
    };

    audit_append(b"[amnesic] Session started — no-persistence mode");
    klog!(b"[amnesic] init complete\n");
}

/// sigma_amnesic_scrub_ram — overwrite all physical RAM on shutdown.
///
/// This is the "sdmem equivalent" — performs SCRUB_PASSES passes over
/// the entire physical address range, alternating 0x00 and 0xFF patterns.
/// Uses volatile writes to defeat compiler dead-store elimination.
///
/// Expected call site: shutdown/reboot path, after all filesystems
/// have been unmounted and processes have been killed.
///
/// Safety: Must be called with all CPUs in single-CPU mode (secondary
/// CPUs parked), interrupts disabled. After this returns, RAM is wiped.
#[no_mangle]
pub unsafe extern "C" fn sigma_amnesic_scrub_ram() {
    if !AMNESIC_ACTIVE.load(Ordering::SeqCst) {
        klog!(b"[amnesic] scrub_ram called outside amnesic mode — aborting\n");
        return;
    }

    klog!(b"[amnesic] RAM scrub starting...\n");

    // Unmount all non-essential filesystems first.
    sigma_fs_unmount_all();

    let phys_base  = sigma_mm_phys_base();
    let total_size = sigma_mm_total_bytes();

    G_AMNESIC_STATE.bytes_total = total_size;

    // Three-pass scrub: 0x00, 0xFF, 0x00 (matches Tails OS sdmem behaviour)
    for pass in 0..SCRUB_PASSES {
        let fill = PASS_PATTERNS[pass];

        // Log pass start (only possible before RAM is wiped!)
        if pass == 0 {
            klog!(b"[amnesic] scrub pass 1/3: zeroing RAM\n");
        } else if pass == 1 {
            klog!(b"[amnesic] scrub pass 2/3: writing 0xFF\n");
        } else {
            klog!(b"[amnesic] scrub pass 3/3: final zero\n");
        }

        // Scrub page by page for progress tracking.
        let total_pages = (total_size / PAGE_SIZE as SigmaU64) as SigmaUsize;
        for page in 0..total_pages {
            let page_addr = phys_base + (page * PAGE_SIZE) as SigmaU64;
            scrub_range(page_addr, PAGE_SIZE as SigmaU64, fill);

            // Update progress counter every 256 pages.
            if page & 0xFF == 0 {
                let scrubbed = (page * PAGE_SIZE) as SigmaU64;
                BYTES_SCRUBBED.store(scrubbed, Ordering::Relaxed);
                G_AMNESIC_STATE.bytes_scrubbed = scrubbed;
            }
        }
    }

    BYTES_SCRUBBED.store(total_size, Ordering::SeqCst);
    G_AMNESIC_STATE.bytes_scrubbed = total_size;

    klog!(b"[amnesic] RAM scrub complete — halting\n");
    sigma_system_halt();
}

/// sigma_amnesic_emergency_wipe — immediate, asynchronous RAM wipe.
///
/// Triggered by USB-removal watchdog or panic handler.
/// Skips the orderly shutdown sequence and goes straight to RAM scrub.
/// Equivalent to Tails' udev watchdog emergency shutdown path.
#[no_mangle]
pub unsafe extern "C" fn sigma_amnesic_emergency_wipe() {
    if EMERGENCY_WIPE_ACTIVE.swap(true, Ordering::SeqCst) {
        return; // Already in progress.
    }
    G_AMNESIC_STATE.emergency = true;
    klog!(b"[amnesic] EMERGENCY WIPE TRIGGERED\n");
    // In emergency mode we skip the fs_unmount_all to be faster.
    let phys_base  = sigma_mm_phys_base();
    let total_size = sigma_mm_total_bytes();
    // Single-pass zero for speed in emergency (cold-boot window is very short).
    scrub_range(phys_base, total_size, 0x00);
    sigma_system_halt();
}

// ── C-ABI accessors ───────────────────────────────────────────────────────────

/// Returns pointer to the current AmnesicState snapshot.
#[no_mangle]
pub unsafe extern "C" fn sigma_amnesic_state() -> *const AmnesicState {
    &G_AMNESIC_STATE as *const AmnesicState
}

/// Returns 1 if amnesic mode is active, 0 otherwise.
#[no_mangle]
pub unsafe extern "C" fn sigma_amnesic_is_active() -> SigmaU32 {
    if AMNESIC_ACTIVE.load(Ordering::SeqCst) { 1 } else { 0 }
}

/// Returns bytes scrubbed so far (for progress display).
#[no_mangle]
pub unsafe extern "C" fn sigma_amnesic_scrub_progress() -> SigmaU64 {
    BYTES_SCRUBBED.load(Ordering::Relaxed)
}

/// Returns pointer to the in-RAM audit log array.
#[no_mangle]
pub unsafe extern "C" fn sigma_amnesic_audit_log() -> *const AuditEntry {
    AUDIT_LOG.as_ptr()
}

/// Returns number of audit entries written this session.
#[no_mangle]
pub unsafe extern "C" fn sigma_amnesic_audit_count() -> SigmaU32 {
    LOG_COUNT.load(Ordering::Relaxed)
}
