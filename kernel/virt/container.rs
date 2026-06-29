// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Container Management (Rust, no_std)
//! =========================================================================

pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;

type U32 = u32;

pub struct SigmaContainer {
    id: U32,
    active: bool,
    mem_limit: U32,
}

impl SigmaContainer {
    pub const fn empty() -> Self {
        SigmaContainer {
            id: 0,
            active: false,
            mem_limit: 0,
        }
    }
}

pub struct ContainerManager {
    containers: [SigmaContainer; 64],
}

impl ContainerManager {
    pub const fn new() -> Self {
        ContainerManager {
            containers: [SigmaContainer::empty(); 64],
        }
    }

    pub fn spawn_container(&mut self, id: U32, mem_limit: U32) -> SigmaStatus {
        let mut i = 0;
        while i < 64 {
            if !self.containers[i].active {
                self.containers[i].id = id;
                self.containers[i].mem_limit = mem_limit;
                self.containers[i].active = true;
                return SIGMA_OK;
            }
            i += 1;
        }
        SIGMA_ERROR
    }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_CONTAINER_MGR: ContainerManager = ContainerManager::new();

// ── C-ABI Exports (Replacing sigma_container.cpp) ──────────────────────────

#[no_mangle]
pub unsafe extern "C" fn container_spawn(id: U32, mem_limit: U32) -> SigmaStatus {
    G_CONTAINER_MGR.spawn_container(id, mem_limit)
}
