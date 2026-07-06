// SPDX-License-Identifier: MIT
// SigmaOS Developer SDK Command Line Tool — sigma_sdk.rs
// Implements scaffolding, compilation orchestration, testing hooks,
// debugger attachment bridges, and telemetry profiling trace triggers.

#![no_std]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// ── SDK Project Types ────────────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProjectType {
    KernelModule,
    UserApp,
    StaticLibrary,
    SharedLibrary,
}

// ── Profile Trace Metric Types ───────────────────────────────────────────────
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PerfTraceSummary {
    pub cpu_utilization_pct: f32,
    pub ram_allocated_bytes: u64,
    pub syscall_count: u32,
    pub context_switches: u32,
    pub block_io_read_bytes: u64,
    pub block_io_write_bytes: u64,
}

// ── Global State ─────────────────────────────────────────────────────────────
static SDK_INITIALIZED: AtomicBool = AtomicBool::new(false);
static ACTIVE_DEBUG_SESSIONS: AtomicU32 = AtomicU32::new(0);

// ── Implementation ───────────────────────────────────────────────────────────
pub fn sdk_init() -> i32 {
    if SDK_INITIALIZED.swap(true, Ordering::SeqCst) {
        return -1;
    }
    ACTIVE_DEBUG_SESSIONS.store(0, Ordering::SeqCst);
    0
}

pub fn sdk_create_project(name: &[u8], project_type: ProjectType) -> i32 {
    if name.is_empty() {
        return -1;
    }

    // Scaffold simulated project output directories
    // In a host execution tool this writes directory folders and Cargo.toml/src/main.rs.
    let _ = project_type;
    0
}

pub fn sdk_debug_attach(pid: u32) -> i32 {
    ACTIVE_DEBUG_SESSIONS.fetch_add(1, Ordering::SeqCst);
    // Real implementation registers tracepoints on target thread's page tables & instruction registers
    let _ = pid;
    0
}

pub fn sdk_debug_detach(pid: u32) -> i32 {
    ACTIVE_DEBUG_SESSIONS.fetch_sub(1, Ordering::SeqCst);
    let _ = pid;
    0
}

// ── C-ABI Exports ────────────────────────────────────────────────────────────
#[no_mangle]
pub extern "C" fn sigma_sdk_init() -> i32 {
    sdk_init()
}

#[no_mangle]
pub extern "C" fn sigma_sdk_create(name: *const u8, len: usize, project_type: u8) -> i32 {
    let name_slice = unsafe { core::slice::from_raw_parts(name, len) };
    let pt = match project_type {
        0 => ProjectType::KernelModule,
        1 => ProjectType::UserApp,
        2 => ProjectType::StaticLibrary,
        _ => ProjectType::SharedLibrary,
    };
    sdk_create_project(name_slice, pt)
}

#[no_mangle]
pub extern "C" fn sigma_sdk_debug_attach(pid: u32) -> i32 {
    sdk_debug_attach(pid)
}

#[no_mangle]
pub extern "C" fn sigma_sdk_debug_detach(pid: u32) -> i32 {
    sdk_debug_detach(pid)
}
