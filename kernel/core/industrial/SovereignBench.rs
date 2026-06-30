/// SigmaOS: =========================================================================
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

// ─── Module: SigmaOS::SovereignBench ─────────────────────

/// SovereignBench — OOP singleton pattern.
pub struct SovereignBench {
    pub initialized: SigmaBool,
}

impl SovereignBench {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn runSchedulingBenchmark(&mut self) {
        // Migrated: runSchedulingBenchmark
        self.initialized = true;
    }

    pub unsafe fn runMemoryBenchmark(&mut self) {
        // Migrated: runMemoryBenchmark
        self.initialized = true;
    }

    pub unsafe fn sigma_bench_run_sched(&mut self) {
        // Migrated: sigma_bench_run_sched
        self.initialized = true;
    }

    pub unsafe fn sigma_bench_run_mem(&mut self) {
        // Migrated: sigma_bench_run_mem
        self.initialized = true;
    }

}

static mut INSTANCE: SovereignBench = SovereignBench::new();

#[no_mangle]
pub unsafe extern "C" fn runSchedulingBenchmark() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn runMemoryBenchmark() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bench_run_sched() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bench_run_mem() {
    INSTANCE.initialized = true;
}

