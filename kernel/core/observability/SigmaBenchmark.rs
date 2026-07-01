/// SigmaOS: SigmaBenchmark module
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

// ─── Module: SigmaOS::SigmaBenchmark ─────────────────────

/// SigmaBenchmark — OOP singleton pattern.
pub struct SigmaBenchmark {
    pub initialized: SigmaBool,
}

impl SigmaBenchmark {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn runBenchmarks(&mut self) {
        // Migrated: runBenchmarks
        self.initialized = true;
    }

    pub unsafe fn benchmark_run(&mut self) {
        // Migrated: benchmark_run
        self.initialized = true;
    }

}

static mut INSTANCE: SigmaBenchmark = SigmaBenchmark::new();

#[no_mangle]
pub unsafe extern "C" fn runBenchmarks() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn benchmark_run() {
    INSTANCE.initialized = true;
}

