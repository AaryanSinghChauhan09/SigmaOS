// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Sovereign Hypervisor (Rust, no_std)
//! =========================================================================

pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;

type U32 = u32;

pub struct SovereignHypervisor {
    active: bool,
    vm_count: U32,
}

impl SovereignHypervisor {
    pub const fn new() -> Self {
        SovereignHypervisor {
            active: false,
            vm_count: 0,
        }
    }

    pub fn init(&mut self) -> SigmaStatus {
        if self.active {
            return SIGMA_OK;
        }

        // Initialize Virtualization Extensions (VT-x / AMD-V)
        
        self.active = true;
        SIGMA_OK
    }

    pub fn create_vm(&mut self) -> U32 {
        if !self.active {
            return 0;
        }
        
        self.vm_count += 1;
        self.vm_count
    }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_HYPERVISOR: SovereignHypervisor = SovereignHypervisor::new();

// ── C-ABI Exports (Replacing SovereignHypervisor.cpp) ──────────────────────

#[no_mangle]
pub unsafe extern "C" fn hypervisor_init() -> SigmaStatus {
    G_HYPERVISOR.init()
}

#[no_mangle]
pub unsafe extern "C" fn hypervisor_create_vm() -> U32 {
    G_HYPERVISOR.create_vm()
}
