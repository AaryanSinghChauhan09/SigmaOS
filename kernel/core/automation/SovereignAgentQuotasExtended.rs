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

// ─── Module: SigmaOS::SovereignAgentQuotasExtended ─────────────────────

/// SovereignAgentQuotasExtended — OOP singleton pattern.
pub struct SovereignAgentQuotasExtended {
    pub initialized: SigmaBool,
}

impl SovereignAgentQuotasExtended {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn setExtendedQuotas(&mut self) {
        // Migrated: setExtendedQuotas
        self.initialized = true;
    }

    pub unsafe fn agent_quota_extend(&mut self) {
        // Migrated: agent_quota_extend
        self.initialized = true;
    }

}

static mut INSTANCE: SovereignAgentQuotasExtended = SovereignAgentQuotasExtended::new();

#[no_mangle]
pub unsafe extern "C" fn setExtendedQuotas() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn agent_quota_extend() {
    INSTANCE.initialized = true;
}

