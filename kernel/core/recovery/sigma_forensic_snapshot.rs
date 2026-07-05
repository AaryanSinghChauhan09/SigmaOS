// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/recovery/sigma_forensic_snapshot.rs — Live Forensic Snapshot
//
// Implements userspace freezing/thawing for memory forensics and crash dumps.
// Inspired by Linux's freezer subsystem and Windows MiniDump mechanisms.
// Uses coordinated hooks into process_manager and sigma_sched.
//
// Language: Rust #![no_std] — no alloc, no external crates.
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// ── Kernel Primitive Types ────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── Forensic Dump Magic ───────────────────────────────────────────────────────
/// "SIGM" as a u32 little-endian magic word (0x4D_47_49_53)
const FORENSIC_MAGIC: SigmaU32 = 0x4D47_4953;

/// Maximum number of process snapshot entries we can record.
const MAX_SNAPSHOT_ENTRIES: SigmaUsize = 256;
/// Maximum process name length in a snapshot entry.
const SNAPSHOT_NAME_LEN: SigmaUsize = 32;

// ── Freeze State ─────────────────────────────────────────────────────────────
/// Global flag: true while userspace is frozen for a snapshot.
static USERSPACE_FROZEN: AtomicBool = AtomicBool::new(false);
/// Count of successfully frozen tasks.
static FROZEN_COUNT: AtomicU32 = AtomicU32::new(0);

// ── Hardware-compatible Structs ───────────────────────────────────────────────

/// ForensicDumpHeader — written at the top of every snapshot dump.
/// Packed repr(C) so it can be memory-mapped or DMA'd directly.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ForensicDumpHeader {
    /// Magic: 0x4D474953 ("SIGM" in LE) — identifies a valid SigmaOS dump.
    pub magic:            SigmaU32,
    /// Monotonic jiffies counter at freeze time.
    pub timestamp:        SigmaU64,
    /// Total physical memory size in bytes.
    pub memory_size:      SigmaU64,
    /// Number of processes that were frozen.
    pub active_processes: SigmaU32,
    /// SHA-512 hash of kernel .text segment (filled by caller).
    pub kernel_hash:      [SigmaU8; 64],
    /// SigmaOS release version byte-string, NUL-padded.
    pub version_str:      [SigmaU8; 32],
    /// Platform identifier: 0=x86_64, 1=aarch64, 2=riscv64.
    pub platform:         SigmaU8,
    pub _pad:             [SigmaU8; 7],
}

impl ForensicDumpHeader {
    pub const fn zeroed() -> Self {
        Self {
            magic:            0,
            timestamp:        0,
            memory_size:      0,
            active_processes: 0,
            kernel_hash:      [0u8; 64],
            version_str:      [0u8; 32],
            platform:         0,
            _pad:             [0u8; 7],
        }
    }
}

/// SnapshotEntry — one record per frozen process.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SnapshotEntry {
    pub pid:        SigmaU32,
    pub ppid:       SigmaU32,
    /// Saved scheduler state: 0=running,1=runnable,2=sleeping,3=zombie.
    pub prev_state: SigmaU8,
    pub _pad:       [SigmaU8; 3],
    pub name:       [SigmaU8; SNAPSHOT_NAME_LEN],
    /// Saved instruction pointer at freeze point.
    pub rip:        SigmaU64,
    /// Saved stack pointer at freeze point.
    pub rsp:        SigmaU64,
}

impl SnapshotEntry {
    pub const fn zeroed() -> Self {
        Self {
            pid:        0,
            ppid:       0,
            prev_state: 0,
            _pad:       [0u8; 3],
            name:       [0u8; SNAPSHOT_NAME_LEN],
            rip:        0,
            rsp:        0,
        }
    }
}

// ── Global Snapshot Storage ───────────────────────────────────────────────────
static mut DUMP_HEADER:      ForensicDumpHeader = ForensicDumpHeader::zeroed();
static mut SNAPSHOT_ENTRIES: [SnapshotEntry; MAX_SNAPSHOT_ENTRIES] =
    [SnapshotEntry::zeroed(); MAX_SNAPSHOT_ENTRIES];

// ── External kernel interfaces ────────────────────────────────────────────────
extern "C" {
    /// Returns current jiffies (monotonic tick counter).
    fn sigma_jiffies() -> SigmaU64;
    /// Returns total physical memory in bytes.
    fn sigma_mm_total_bytes() -> SigmaU64;
    /// Returns number of live tasks in the process table.
    fn sigma_task_count() -> SigmaUsize;
    /// Fills `entry` with info about task at index `idx`.
    /// Returns 0 on success, -1 if index is out of range.
    fn sigma_task_info(idx: SigmaUsize, entry: *mut SnapshotEntry) -> SigmaU32;
    /// Set task `pid` state to sleeping (freeze). Returns 0 on success.
    fn sigma_task_set_sleeping(pid: SigmaU32) -> i32;
    /// Restore task `pid` to its previous `state`. Returns 0 on success.
    fn sigma_task_set_runnable(pid: SigmaU32) -> i32;
    /// Signal the scheduler to skip all tasks until thaw.
    fn sched_freeze_all();
    /// Signal the scheduler to resume normal scheduling.
    fn sched_thaw_all();
    /// Emit a kernel log message (raw bytes, no formatting).
    fn sigma_log(msg: *const SigmaU8, len: SigmaUsize);
}

// ── Helper: emit a static string log ─────────────────────────────────────────
macro_rules! klog {
    ($msg:expr) => {
        unsafe { sigma_log($msg.as_ptr(), $msg.len()) }
    };
}

// ── Core Implementation ───────────────────────────────────────────────────────

/// freeze_userspace — halt all userspace tasks for a live forensic snapshot.
///
/// Algorithm (mirrors Linux freezer + Windows MiniDump):
///  1. Raise the FROZEN flag so new tasks see it immediately.
///  2. Signal the scheduler to skip all frozen tasks.
///  3. Walk every live task, set state = Sleeping, record entry.
///  4. Write the ForensicDumpHeader with timestamp, memory size, count.
///
/// Safety: must be called with interrupts disabled (cli) in the caller.
#[no_mangle]
pub unsafe extern "C" fn freeze_userspace() {
    // Guard: don't double-freeze.
    if USERSPACE_FROZEN.swap(true, Ordering::SeqCst) {
        klog!(b"[forensic] freeze_userspace: already frozen\n");
        return;
    }

    klog!(b"[forensic] freeze_userspace: begin\n");

    // Tell scheduler to skip all non-kernel tasks on next tick.
    sched_freeze_all();

    // Walk the process table and freeze every task.
    let n = sigma_task_count();
    let mut frozen: SigmaU32 = 0;

    let entries_ptr = SNAPSHOT_ENTRIES.as_mut_ptr();

    for i in 0..n.min(MAX_SNAPSHOT_ENTRIES) {
        let mut entry = SnapshotEntry::zeroed();
        let rc = sigma_task_info(i, &mut entry as *mut SnapshotEntry);
        if rc != 0 { continue; }

        // Skip PID 0 (idle) and PID 1 (init) — they must keep running.
        if entry.pid <= 1 { continue; }

        // Freeze the task in the process manager.
        if sigma_task_set_sleeping(entry.pid) == 0 {
            entry.prev_state = 1; // was Runnable
            core::ptr::write(entries_ptr.add(i), entry);
            frozen += 1;
        }
    }

    FROZEN_COUNT.store(frozen, Ordering::SeqCst);

    // Write the dump header.
    DUMP_HEADER = ForensicDumpHeader {
        magic:            FORENSIC_MAGIC,
        timestamp:        sigma_jiffies(),
        memory_size:      sigma_mm_total_bytes(),
        active_processes: frozen,
        kernel_hash:      [0u8; 64],   // caller fills via sigma_snapshot_hash()
        version_str:      {
            let mut v = [0u8; 32];
            let tag = b"SigmaOS-0.9.0";
            let len = tag.len().min(31);
            let mut j = 0;
            while j < len { v[j] = tag[j]; j += 1; }
            v
        },
        platform:         0, // x86_64
        _pad:             [0u8; 7],
    };

    klog!(b"[forensic] freeze_userspace: complete\n");
}

/// thaw_userspace — reverse freeze_userspace, resume all frozen tasks.
///
/// Algorithm:
///  1. Walk the snapshot table and restore each task to Runnable.
///  2. Signal scheduler to resume normal operation.
///  3. Clear FROZEN flag.
#[no_mangle]
pub unsafe extern "C" fn thaw_userspace() {
    if !USERSPACE_FROZEN.load(Ordering::SeqCst) {
        klog!(b"[forensic] thaw_userspace: not frozen\n");
        return;
    }

    klog!(b"[forensic] thaw_userspace: begin\n");

    let n = FROZEN_COUNT.load(Ordering::SeqCst) as SigmaUsize;
    let entries_ptr = SNAPSHOT_ENTRIES.as_ptr();

    for i in 0..n.min(MAX_SNAPSHOT_ENTRIES) {
        let entry = core::ptr::read(entries_ptr.add(i));
        if entry.pid > 1 {
            sigma_task_set_runnable(entry.pid);
        }
    }

    // Tell scheduler to resume.
    sched_thaw_all();

    // Reset header magic so it can't be mistaken for a live dump.
    DUMP_HEADER.magic = 0;
    FROZEN_COUNT.store(0, Ordering::SeqCst);
    USERSPACE_FROZEN.store(false, Ordering::SeqCst);

    klog!(b"[forensic] thaw_userspace: complete\n");
}

// ── C-ABI accessors ───────────────────────────────────────────────────────────

/// Returns pointer to the current dump header (valid only while frozen).
#[no_mangle]
pub unsafe extern "C" fn sigma_snapshot_header() -> *const ForensicDumpHeader {
    &DUMP_HEADER as *const ForensicDumpHeader
}

/// Returns pointer to the snapshot entry array.
#[no_mangle]
pub unsafe extern "C" fn sigma_snapshot_entries() -> *const SnapshotEntry {
    SNAPSHOT_ENTRIES.as_ptr()
}

/// Returns the frozen task count (0 if not frozen).
#[no_mangle]
pub unsafe extern "C" fn sigma_snapshot_count() -> SigmaU32 {
    FROZEN_COUNT.load(Ordering::SeqCst)
}

/// Returns 1 if userspace is currently frozen, 0 otherwise.
#[no_mangle]
pub unsafe extern "C" fn sigma_is_frozen() -> SigmaU32 {
    if USERSPACE_FROZEN.load(Ordering::SeqCst) { 1 } else { 0 }
}
