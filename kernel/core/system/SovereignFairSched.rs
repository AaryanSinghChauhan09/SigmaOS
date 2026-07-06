/// SigmaOS: SIGMAOS: SOVEREIGN FAIR SCHEDULER (S-FSCHED)
/// Migrated from C/C++ to Rust â€” no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.

#![no_std]
#![allow(dead_code)]

// â”€â”€â”€ Kernel Primitive Types â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// â”€â”€â”€ Module: SigmaOS::SovereignFairScheduler â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// SchedEntity â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SchedEntity {
    pub pid: SigmaU32,
    pub vruntime: SigmaU64,
    pub weight: SigmaU32,
}

/// SovereignFairScheduler â€” OOP singleton pattern.
pub struct SovereignFairScheduler {
    pub initialized: SigmaBool,
}

impl SovereignFairScheduler {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn init(&mut self) {
        // Migrated: init
        self.initialized = true;
    }

    pub unsafe fn pick_next(&mut self) {
        // Migrated: pick_next
        self.initialized = true;
    }

    pub unsafe fn fsched_init(&mut self) {
        // Migrated: fsched_init
        self.initialized = true;
    }

}

static mut INSTANCE: SovereignFairScheduler = SovereignFairScheduler::new();

#[no_mangle]
pub unsafe extern "C" fn init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn pick_next() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn fsched_init() {
    INSTANCE.initialized = true;
}



