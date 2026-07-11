// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/ostree/rollback.rs — OSTree Rollback System
//
// Provides rollback capability for OSTree deployments
// Allows reverting to previous system states atomically
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U32 = u32;
type U64 = u64;
type I32 = i32;

pub const ROLLBACK_OK: I32 = 0;
pub const ROLLBACK_ERR_NO_PREV: I32 = -1;
pub const ROLLBACK_ERR_INVALID: I32 = -2;

const MAX_ROLLBACK_POINTS: usize = 5;
const ROLLBACK_ID_LEN: usize = 64;

// ─── Rollback Point Structure ───────────────────────────────────────────────────

#[repr(C)]
pub struct RollbackPoint {
    pub id: [U8; ROLLBACK_ID_LEN],
    pub deployment_id: [U8; 64],
    pub commit: [U8; 64],
    pub timestamp: U64,
    pub reason: [U8; 256],
    pub automatic: bool,
}

impl RollbackPoint {
    pub const fn empty() -> Self {
        Self {
            id: [0; ROLLBACK_ID_LEN],
            deployment_id: [0; 64],
            commit: [0; 64],
            timestamp: 0,
            reason: [0; 256],
            automatic: false,
        }
    }
}

// ─── Rollback Manager ─────────────────────────────────────────────────────────

pub struct RollbackManager {
    pub rollback_points: [RollbackPoint; MAX_ROLLBACK_POINTS],
    pub current_index: isize,
    pub rollback_count: U32,
}

impl RollbackManager {
    pub const fn new() -> Self {
        Self {
            rollback_points: [RollbackPoint::empty(); MAX_ROLLBACK_POINTS],
            current_index: -1,
            rollback_count: 0,
        }
    }

    /// Initialize rollback manager
    pub unsafe fn init(&mut self) -> I32 {
        self.load_rollback_points();
        ROLLBACK_OK
    }

    /// Create rollback point before deployment
    pub unsafe fn create_rollback_point(&mut self, deployment_id: &[U8], commit: &[U8], reason: &[U8]) -> I32 {
        // Find free slot
        let slot = self.find_free_slot();
        if slot < 0 {
            // Remove oldest rollback point
            self.remove_oldest();
            let slot = self.find_free_slot();
            if slot < 0 {
                return ROLLBACK_ERR_NO_PREV;
            }
        }

        let point = &mut self.rollback_points[slot as usize];

        // Copy deployment ID
        let len = deployment_id.len().min(64);
        for i in 0..len {
            point.deployment_id[i] = deployment_id[i];
        }

        // Copy commit checksum
        let commit_len = commit.len().min(64);
        for i in 0..commit_len {
            point.commit[i] = commit[i];
        }

        // Copy reason
        let reason_len = reason.len().min(256);
        for i in 0..reason_len {
            point.reason[i] = reason[i];
        }

        point.timestamp = self.get_timestamp();
        point.automatic = false;

        // Generate rollback point ID
        self.generate_rollback_id(&mut point.id);

        self.current_index = slot;
        self.rollback_count += 1;

        self.save_rollback_points();

        ROLLBACK_OK
    }

    /// Perform rollback to previous deployment
    pub unsafe fn rollback(&mut self, rollback_index: isize) -> I32 {
        if rollback_index < 0 || rollback_index >= MAX_ROLLBACK_POINTS as isize {
            return ROLLBACK_ERR_INVALID;
        }

        let point = &self.rollback_points[rollback_index as usize];

        if point.deployment_id[0] == 0 {
            return ROLLBACK_ERR_INVALID;
        }

        // In real implementation, would:
        // 1. Call deployment manager to switch to deployment
        // 2. Update bootloader configuration
        // 3. Reboot system

        self.current_index = rollback_index;

        ROLLBACK_OK
    }

    /// Get available rollback points
    pub fn get_rollback_points(&self) -> &[RollbackPoint] {
        &self.rollback_points
    }

    /// Get current rollback point
    pub fn get_current(&self) -> Option<&RollbackPoint> {
        if self.current_index >= 0 && self.current_index < MAX_ROLLBACK_POINTS as isize {
            Some(&self.rollback_points[self.current_index as usize])
        } else {
            None
        }
    }

    /// Create automatic rollback point (for failed boots)
    pub unsafe fn create_automatic_rollback(&mut self, deployment_id: &[U8], commit: &[U8]) -> I32 {
        let reason = b"Automatic rollback due to boot failure";
        self.create_rollback_point(deployment_id, commit, reason)
    }

    /// Find free rollback slot
    fn find_free_slot(&self) -> isize {
        for i in 0..MAX_ROLLBACK_POINTS {
            if self.rollback_points[i].id[0] == 0 {
                return i as isize;
            }
        }
        -1
    }

    /// Remove oldest rollback point
    unsafe fn remove_oldest(&mut self) {
        let mut oldest_idx = 0;
        let mut oldest_timestamp = U64::MAX;

        for i in 0..MAX_ROLLBACK_POINTS {
            if self.rollback_points[i].id[0] != 0 && self.rollback_points[i].timestamp < oldest_timestamp {
                oldest_timestamp = self.rollback_points[i].timestamp;
                oldest_idx = i;
            }
        }

        self.rollback_points[oldest_idx] = RollbackPoint::empty();
    }

    /// Generate rollback point ID
    fn generate_rollback_id(&self, id: &mut [U8]) {
        let timestamp = self.get_timestamp();
        for i in 0..ROLLBACK_ID_LEN {
            id[i] = ((timestamp >> (i * 8)) & 0xFF) as U8;
        }
    }

    /// Get current timestamp
    fn get_timestamp(&self) -> U64 {
        // In real implementation, get from RTC
        0
    }

    /// Load rollback points from disk
    unsafe fn load_rollback_points(&mut self) {
        // In real implementation, read from rollback metadata
    }

    /// Save rollback points to disk
    unsafe fn save_rollback_points(&self) {
        // In real implementation, write to rollback metadata
    }
}

// ─── Global Rollback Manager ─────────────────────────────────────────────────

static mut ROLLBACK_MANAGER: RollbackManager = RollbackManager::new();

// ─── C-ABI Exports ───────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_rollback_init() -> I32 {
    ROLLBACK_MANAGER.init()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_rollback_create_point(deployment_id: *const U8, deployment_len: U32, commit: *const U8, commit_len: U32, reason: *const U8, reason_len: U32) -> I32 {
    let deployment_slice = core::slice::from_raw_parts(deployment_id, deployment_len as usize);
    let commit_slice = core::slice::from_raw_parts(commit, commit_len as usize);
    let reason_slice = core::slice::from_raw_parts(reason, reason_len as usize);
    ROLLBACK_MANAGER.create_rollback_point(deployment_slice, commit_slice, reason_slice)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_rollback_perform(index: I32) -> I32 {
    ROLLBACK_MANAGER.rollback(index)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_rollback_create_automatic(deployment_id: *const U8, deployment_len: U32, commit: *const U8, commit_len: U32) -> I32 {
    let deployment_slice = core::slice::from_raw_parts(deployment_id, deployment_len as usize);
    let commit_slice = core::slice::from_raw_parts(commit, commit_len as usize);
    ROLLBACK_MANAGER.create_automatic_rollback(deployment_slice, commit_slice)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_rollback_get_count() -> U32 {
    ROLLBACK_MANAGER.rollback_count
}
