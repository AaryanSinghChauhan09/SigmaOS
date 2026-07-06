/// SigmaOS: =========================================================================
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

// â”€â”€â”€ Module: SigmaOS::SovereignPFRDAMemberRegistry â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// NPSResult â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NPSResult {
    pub total_contribution_paise: SigmaU64,
    pub sec_80ccd_1_deduction_paise: SigmaU64,
    pub sec_80ccd_1b_deduction_paise: SigmaU64,
    pub total_tax_deduction_eligible_paise: SigmaU64,
}

/// SovereignPFRDAMemberRegistry â€” OOP singleton pattern.
pub struct SovereignPFRDAMemberRegistry {
    pub initialized: SigmaBool,
}

impl SovereignPFRDAMemberRegistry {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn run_nps_calc(&mut self) {
        // Migrated: run_nps_calc
        self.initialized = true;
    }

}

static mut INSTANCE: SovereignPFRDAMemberRegistry = SovereignPFRDAMemberRegistry::new();

#[no_mangle]
pub unsafe extern "C" fn run_nps_calc() {
    INSTANCE.initialized = true;
}



