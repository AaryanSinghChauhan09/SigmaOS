// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/ostree/deployment.rs — OSTree Deployment Management
//
// Manages OSTree deployments with atomic switching and rollback support
// Inspired by Fedora Silverblue and SteamOS OSTree implementation
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U32 = u32;
type U64 = u64;
type I32 = i32;

pub const DEPLOYMENT_OK: I32 = 0;
pub const DEPLOYMENT_ERR_INVALID: I32 = -1;
pub const DEPLOYMENT_ERR_NO_SPACE: I32 = -2;
pub const DEPLOYMENT_ERR_CHECKSUM: I32 = -3;

const MAX_DEPLOYMENTS: usize = 3;
const DEPLOYMENT_ID_LEN: usize = 64;
const BOOTCONFIG_LEN: usize = 512;

// ─── Deployment Structure ─────────────────────────────────────────────────────

#[repr(C)]
pub struct Deployment {
    pub id: [U8; DEPLOYMENT_ID_LEN],
    pub commit: [U8; 64], // BLAKE3 checksum
    pub bootconfig: [U8; BOOTCONFIG_LEN],
    pub booted: bool,
    pub pending: bool,
    pub timestamp: U64,
    pub is_default: bool,
}

impl Deployment {
    pub const fn empty() -> Self {
        Self {
            id: [0; DEPLOYMENT_ID_LEN],
            commit: [0; 64],
            bootconfig: [0; BOOTCONFIG_LEN],
            booted: false,
            pending: false,
            timestamp: 0,
            is_default: false,
        }
    }
}

// ─── Deployment Manager ─────────────────────────────────────────────────────

pub struct DeploymentManager {
    pub deployments: [Deployment; MAX_DEPLOYMENTS],
    pub current_index: isize,
    pub default_index: isize,
    pub boot_partition: U32,
    pub root_partition: U32,
}

impl DeploymentManager {
    pub const fn new() -> Self {
        Self {
            deployments: [Deployment::empty(); MAX_DEPLOYMENTS],
            current_index: -1,
            default_index: -1,
            boot_partition: 0,
            root_partition: 0,
        }
    }

    /// Initialize deployment manager
    pub unsafe fn init(&mut self, boot_part: U32, root_part: U32) -> I32 {
        self.boot_partition = boot_part;
        self.root_partition = root_part;

        // Load existing deployments from disk
        self.load_deployments();

        DEPLOYMENT_OK
    }

    /// Create new deployment from commit
    pub unsafe fn create_deployment(&mut self, commit: &[U8], bootconfig: &[U8]) -> I32 {
        // Find free slot
        let slot = self.find_free_slot();
        if slot < 0 {
            return DEPLOYMENT_ERR_NO_SPACE;
        }

        let deployment = &mut self.deployments[slot as usize];
        
        // Copy commit checksum
        let len = commit.len().min(64);
        for i in 0..len {
            deployment.commit[i] = commit[i];
        }

        // Copy boot config
        let config_len = bootconfig.len().min(BOOTCONFIG_LEN);
        for i in 0..config_len {
            deployment.bootconfig[i] = bootconfig[i];
        }

        // Generate deployment ID
        self.generate_deployment_id(&mut deployment.id);

        deployment.timestamp = self.get_timestamp();
        deployment.pending = true;
        deployment.booted = false;

        // Set as default
        self.set_default(slot);

        // Save to disk
        self.save_deployments();

        DEPLOYMENT_OK
    }

    /// Set deployment as default
    pub unsafe fn set_default(&mut self, index: isize) -> I32 {
        if index < 0 || index >= MAX_DEPLOYMENTS as isize {
            return DEPLOYMENT_ERR_INVALID;
        }

        // Clear default flag from all deployments
        for i in 0..MAX_DEPLOYMENTS {
            self.deployments[i].is_default = false;
        }

        // Set new default
        self.deployments[index as usize].is_default = true;
        self.default_index = index;

        // Update bootloader configuration
        self.update_bootloader_config();

        DEPLOYMENT_OK
    }

    /// Switch to deployment
    pub unsafe fn switch_deployment(&mut self, index: isize) -> I32 {
        if index < 0 || index >= MAX_DEPLOYMENTS as isize {
            return DEPLOYMENT_ERR_INVALID;
        }

        if !self.deployments[index as usize].pending {
            return DEPLOYMENT_ERR_INVALID;
        }

        // Mark as current
        self.current_index = index;

        // Set as default
        self.set_default(index);

        DEPLOYMENT_OK
    }

    /// Mark deployment as booted
    pub unsafe fn mark_booted(&mut self, index: isize) -> I32 {
        if index < 0 || index >= MAX_DEPLOYMENTS as isize {
            return DEPLOYMENT_ERR_INVALID;
        }

        self.deployments[index as usize].booted = true;
        self.deployments[index as usize].pending = false;

        DEPLOYMENT_OK
    }

    /// Get current deployment
    pub fn get_current(&self) -> Option<&Deployment> {
        if self.current_index >= 0 && self.current_index < MAX_DEPLOYMENTS as isize {
            Some(&self.deployments[self.current_index as usize])
        } else {
            None
        }
    }

    /// Get default deployment
    pub fn get_default(&self) -> Option<&Deployment> {
        if self.default_index >= 0 && self.default_index < MAX_DEPLOYMENTS as isize {
            Some(&self.deployments[self.default_index as usize])
        } else {
            None
        }
    }

    /// Get all deployments
    pub fn get_all(&self) -> &[Deployment] {
        &self.deployments
    }

    /// Find free deployment slot
    fn find_free_slot(&self) -> isize {
        for i in 0..MAX_DEPLOYMENTS {
            if self.deployments[i].id[0] == 0 {
                return i as isize;
            }
        }
        -1
    }

    /// Generate deployment ID
    fn generate_deployment_id(&self, id: &mut [U8]) {
        // Simple ID generation based on timestamp
        let timestamp = self.get_timestamp();
        for i in 0..DEPLOYMENT_ID_LEN {
            id[i] = ((timestamp >> (i * 8)) & 0xFF) as U8;
        }
    }

    /// Get current timestamp
    fn get_timestamp(&self) -> U64 {
        // In real implementation, get from RTC
        0
    }

    /// Load deployments from disk
    unsafe fn load_deployments(&mut self) {
        // In real implementation, read from deployment metadata
        // For now, initialize empty
    }

    /// Save deployments to disk
    unsafe fn save_deployments(&self) {
        // In real implementation, write to deployment metadata
    }

    /// Update bootloader configuration
    unsafe fn update_bootloader_config(&self) {
        // In real implementation, update bootloader config (systemd-boot, GRUB, etc.)
        // For now, stub
    }
}

// ─── Global Deployment Manager ─────────────────────────────────────────────

static mut DEPLOYMENT_MANAGER: DeploymentManager = DeploymentManager::new();

// ─── C-ABI Exports ───────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_deployment_init(boot_part: U32, root_part: U32) -> I32 {
    DEPLOYMENT_MANAGER.init(boot_part, root_part)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_deployment_create(commit: *const U8, commit_len: U32, bootconfig: *const U8, config_len: U32) -> I32 {
    let commit_slice = core::slice::from_raw_parts(commit, commit_len as usize);
    let config_slice = core::slice::from_raw_parts(bootconfig, config_len as usize);
    DEPLOYMENT_MANAGER.create_deployment(commit_slice, config_slice)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_deployment_set_default(index: I32) -> I32 {
    DEPLOYMENT_MANAGER.set_default(index as isize)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_deployment_switch(index: I32) -> I32 {
    DEPLOYMENT_MANAGER.switch_deployment(index as isize)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_deployment_mark_booted(index: I32) -> I32 {
    DEPLOYMENT_MANAGER.mark_booted(index as isize)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_deployment_get_current() -> I32 {
    DEPLOYMENT_MANAGER.current_index as I32
}

#[no_mangle]
pub unsafe extern "C" fn sigma_deployment_get_default() -> I32 {
    DEPLOYMENT_MANAGER.default_index as I32
}
