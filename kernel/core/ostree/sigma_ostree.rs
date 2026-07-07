// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/ostree/sigma_ostree.rs — OSTree/Immutable OS Model
//
// Implements OSTree-inspired immutable OS model for SigmaOS.
// Provides atomic system updates, rollback support, and filesystem immutability.
// Inspired by: OSTree, Fedora Silverblue, Endless OS
// Language: Rust #![no_std] — no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── Constants ─────────────────────────────────────────────────────────────────
/// Maximum number of deployments.
const MAX_DEPLOYMENTS: SigmaUsize = 10;
/// Commit hash length (SHA-256).
const COMMIT_HASH_LEN: SigmaUsize = 64;
/// Ref name length.
const REF_NAME_LEN: SigmaUsize = 128;

// ── Deployment State ───────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DeploymentState {
    /// Booted deployment.
    Booted = 0,
    /// Staged for next boot.
    Staged = 1,
    /// Available but not staged.
    Available = 2,
    /// Pending rollback.
    PendingRollback = 3,
}

// ── Deployment ───────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Deployment {
    pub id: SigmaU32,
    pub commit_hash: [SigmaU8; COMMIT_HASH_LEN],
    pub ref_name: [SigmaU8; REF_NAME_LEN],
    pub state: DeploymentState,
    pub timestamp: SigmaU64,
    pub size: SigmaU64,
    pub pinned: SigmaBool,
    pub _pad: [SigmaU8; 7],
}

impl Deployment {
    pub const fn new() -> Self {
        Self {
            id: 0,
            commit_hash: [0u8; COMMIT_HASH_LEN],
            ref_name: [0u8; REF_NAME_LEN],
            state: DeploymentState::Available,
            timestamp: 0,
            size: 0,
            pinned: false,
            _pad: [0u8; 7],
        }
    }
}

// ── OSTree Repository ─────────────────────────────────────────────────────
#[repr(C)]
pub struct OstreeRepo {
    pub deployments: [Deployment; MAX_DEPLOYMENTS],
    pub deployment_count: SigmaUsize,
    pub next_deployment_id: SigmaU32,
    pub booted_deployment: SigmaU32,
    pub staged_deployment: SigmaU32,
    pub immutable_root: SigmaBool,
    pub initialized: SigmaBool,
}

impl OstreeRepo {
    pub const fn new() -> Self {
        Self {
            deployments: [Deployment::new(); MAX_DEPLOYMENTS],
            deployment_count: 0,
            next_deployment_id: 1,
            booted_deployment: 0,
            staged_deployment: 0,
            immutable_root: true,
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
        self.immutable_root = true;
    }

    fn copy_str(dst: &mut [SigmaU8], src: &[SigmaU8]) {
        let len = src.len().min(dst.len() - 1);
        let mut i = 0;
        while i < len { dst[i] = src[i]; i += 1; }
        dst[len] = 0;
    }

    // ── Public API ────────────────────────────────────────────────────────────

    /// Create a new deployment.
    pub fn create_deployment(
        &mut self,
        commit_hash: &[SigmaU8],
        ref_name: &[SigmaU8],
        size: SigmaU64,
    ) -> SigmaU32 {
        if self.deployment_count >= MAX_DEPLOYMENTS {
            return 0;
        }

        let idx = self.deployment_count;
        let id = self.next_deployment_id;
        self.next_deployment_id += 1;

        self.deployments[idx].id = id;
        let hash_len = commit_hash.len().min(COMMIT_HASH_LEN);
        let mut i = 0;
        while i < hash_len {
            self.deployments[idx].commit_hash[i] = commit_hash[i];
            i += 1;
        }
        Self::copy_str(&mut self.deployments[idx].ref_name, ref_name);
        self.deployments[idx].state = DeploymentState::Available;
        self.deployments[idx].timestamp = 0; // In production: get timestamp
        self.deployments[idx].size = size;
        self.deployments[idx].pinned = false;

        self.deployment_count += 1;
        id
    }

    /// Stage a deployment for next boot.
    pub fn stage_deployment(&mut self, deployment_id: SigmaU32) -> SigmaI32 {
        // Unstage current staged deployment
        for i in 0..self.deployment_count {
            if self.deployments[i].id == self.staged_deployment {
                self.deployments[i].state = DeploymentState::Available;
            }
        }

        // Stage new deployment
        for i in 0..self.deployment_count {
            if self.deployments[i].id == deployment_id {
                self.deployments[i].state = DeploymentState::Staged;
                self.staged_deployment = deployment_id;
                return 0;
            }
        }
        -1
    }

    /// Set booted deployment.
    pub fn set_booted_deployment(&mut self, deployment_id: SigmaU32) -> SigmaI32 {
        for i in 0..self.deployment_count {
            if self.deployments[i].id == deployment_id {
                self.deployments[i].state = DeploymentState::Booted;
                self.booted_deployment = deployment_id;
                return 0;
            }
        }
        -1
    }

    /// Rollback to previous deployment.
    pub fn rollback(&mut self) -> SigmaI32 {
        // Find previous deployment
        let mut prev_id: Option<SigmaU32> = None;
        for i in 0..self.deployment_count {
            if self.deployments[i].id != self.booted_deployment {
                if prev_id.is_none() || self.deployments[i].id > prev_id.unwrap() {
                    prev_id = Some(self.deployments[i].id);
                }
            }
        }

        if let Some(id) = prev_id {
            self.stage_deployment(id)
        } else {
            -1
        }
    }

    /// Pin a deployment (prevent garbage collection).
    pub fn pin_deployment(&mut self, deployment_id: SigmaU32) -> SigmaI32 {
        for i in 0..self.deployment_count {
            if self.deployments[i].id == deployment_id {
                self.deployments[i].pinned = true;
                return 0;
            }
        }
        -1
    }

    /// Unpin a deployment.
    pub fn unpin_deployment(&mut self, deployment_id: SigmaU32) -> SigmaI32 {
        for i in 0..self.deployment_count {
            if self.deployments[i].id == deployment_id {
                self.deployments[i].pinned = false;
                return 0;
            }
        }
        -1
    }

    /// Delete a deployment.
    pub fn delete_deployment(&mut self, deployment_id: SigmaU32) -> SigmaI32 {
        for i in 0..self.deployment_count {
            if self.deployments[i].id == deployment_id {
                if self.deployments[i].pinned {
                    return -1; // Cannot delete pinned deployment
                }
                if self.deployments[i].id == self.booted_deployment {
                    return -1; // Cannot delete booted deployment
                }
                self.deployments[i] = Deployment::new();
                self.deployment_count -= 1;
                return 0;
            }
        }
        -1
    }

    /// Enable/disable immutable root.
    pub fn set_immutable_root(&mut self, immutable: SigmaBool) {
        self.immutable_root = immutable;
    }

    /// Get booted deployment.
    pub fn get_booted_deployment(&self) -> Option<&Deployment> {
        for i in 0..self.deployment_count {
            if self.deployments[i].id == self.booted_deployment {
                return Some(&self.deployments[i]);
            }
        }
        None
    }

    /// Get staged deployment.
    pub fn get_staged_deployment(&self) -> Option<&Deployment> {
        for i in 0..self.deployment_count {
            if self.deployments[i].id == self.staged_deployment {
                return Some(&self.deployments[i]);
            }
        }
        None
    }

    /// List all deployments.
    pub fn list_deployments(&self) -> Vec<&Deployment> {
        let mut result = Vec::new();
        for i in 0..self.deployment_count {
            result.push(&self.deployments[i]);
        }
        result
    }

    /// Get deployment count.
    pub fn deployment_count(&self) -> SigmaUsize {
        self.deployment_count
    }

    /// Get total size of all deployments.
    pub fn total_size(&self) -> SigmaU64 {
        let mut total = 0;
        for i in 0..self.deployment_count {
            total += self.deployments[i].size;
        }
        total
    }
}

static mut G_OSTREE: OstreeRepo = OstreeRepo::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_ostree_init() {
    G_OSTREE.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ostree_create_deployment(
    commit_hash: *const SigmaU8,
    hash_len: SigmaUsize,
    ref_name: *const SigmaU8,
    ref_len: SigmaUsize,
    size: SigmaU64,
) -> SigmaU32 {
    let ch = core::slice::from_raw_parts(commit_hash, hash_len.min(COMMIT_HASH_LEN));
    let rn = core::slice::from_raw_parts(ref_name, ref_len.min(REF_NAME_LEN));
    G_OSTREE.create_deployment(ch, rn, size)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ostree_stage_deployment(deployment_id: SigmaU32) -> SigmaI32 {
    G_OSTREE.stage_deployment(deployment_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ostree_set_booted_deployment(deployment_id: SigmaU32) -> SigmaI32 {
    G_OSTREE.set_booted_deployment(deployment_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ostree_rollback() -> SigmaI32 {
    G_OSTREE.rollback()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ostree_pin_deployment(deployment_id: SigmaU32) -> SigmaI32 {
    G_OSTREE.pin_deployment(deployment_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ostree_unpin_deployment(deployment_id: SigmaU32) -> SigmaI32 {
    G_OSTREE.unpin_deployment(deployment_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ostree_delete_deployment(deployment_id: SigmaU32) -> SigmaI32 {
    G_OSTREE.delete_deployment(deployment_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ostree_set_immutable_root(immutable: SigmaU32) {
    G_OSTREE.set_immutable_root(immutable != 0)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ostree_immutable_root() -> SigmaU32 {
    if G_OSTREE.immutable_root { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ostree_booted_deployment() -> SigmaU32 {
    G_OSTREE.booted_deployment
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ostree_staged_deployment() -> SigmaU32 {
    G_OSTREE.staged_deployment
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ostree_deployment_count() -> SigmaU32 {
    G_OSTREE.deployment_count() as SigmaU32
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ostree_total_size() -> SigmaU64 {
    G_OSTREE.total_size()
}
