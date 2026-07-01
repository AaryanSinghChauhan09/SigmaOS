#![no_std]

//! =========================================================================
//! SigmaOS Kernel ABI (kabi) Boundary Definitions
//! Exposes repr(C) stable types for Rust, Zig, Nim, and Ada integrations.
//! =========================================================================

/// Unified syscall argument register set.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SyscallArgs {
    pub id: u64,
    pub arg0: u64,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub ret: i64,
}

/// Standardized scheduler latency metric report.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MetricReport {
    pub cpu_id: u32,
    pub process_count: u32,
    pub active_interrupts: u64,
    pub memory_allocated: u64,
    pub context_switches: u64,
}

/// Exported C-ABI entry helper for logging metrics.
#[no_mangle]
pub unsafe extern "C" fn kabi_log_metrics(report: *const MetricReport) -> i32 {
    if report.is_null() {
        return -1;
    }
    // Perform zero-overhead recording logic here
    0
}
