// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Virtual CPU (Rust, no_std)
//! =========================================================================

pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;

type U32 = u32;
type U64 = u64;

#[derive(Clone, Copy)]
pub struct SigmaVcpu {
    id: U32,
    active: bool,
    rax: U64,
    rbx: U64,
    rcx: U64,
    rdx: U64,
    rip: U64,
    rsp: U64,
}

impl SigmaVcpu {
    pub const fn empty() -> Self {
        SigmaVcpu {
            id: 0,
            active: false,
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rip: 0,
            rsp: 0,
        }
    }
}

pub struct VcpuManager {
    vcpus: [SigmaVcpu; 64],
}

impl VcpuManager {
    pub const fn new() -> Self {
        VcpuManager {
            vcpus: [SigmaVcpu::empty(); 64],
        }
    }

    pub fn allocate_vcpu(&mut self, id: U32) -> SigmaStatus {
        let mut i = 0;
        while i < 64 {
            if !self.vcpus[i].active {
                self.vcpus[i].id = id;
                self.vcpus[i].active = true;
                return SIGMA_OK;
            }
            i += 1;
        }
        SIGMA_ERROR
    }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_VCPU_MGR: VcpuManager = VcpuManager::new();

// ── C-ABI Exports (Replacing sigma_vcpu.cpp) ───────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn vcpu_allocate(id: U32) -> SigmaStatus {
    G_VCPU_MGR.allocate_vcpu(id)
}
