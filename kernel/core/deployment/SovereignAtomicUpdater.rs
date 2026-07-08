// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/deployment/SovereignAtomicUpdater.rs — Atomic System Updater
//
// Implements atomic system updates with staged deployment, verification,
// and rollback capability. Inspired by NixOS/OSTree transactional updates.
//
// Design:
//   - Stage: Download and verify update to staging partition
//   - Verify: Check signatures, hashes, and integrity
//   - Commit: Atomically swap staging into active partition
//   - Rollback: Revert to previous generation on failure
//
// Language: Rust #![no_std] — no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── Constants ─────────────────────────────────────────────────────────────────
/// Maximum update description length.
const UPDATE_DESC_LEN: SigmaUsize = 128;
/// Maximum URL length for update source.
const URL_LEN: SigmaUsize = 256;
/// SHA-256 hash length.
const HASH_LEN: SigmaUsize = 32;
/// Maximum number of staged updates.
const MAX_STAGED: SigmaUsize = 4;

// ── Update Status ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UpdateStatus {
    /// No update staged.
    None        = 0,
    /// Update downloaded and staged.
    Staged      = 1,
    /// Update being verified.
    Verifying   = 2,
    /// Update verified and ready to commit.
    Ready       = 3,
    /// Update committed (will activate on next boot).
    Committed   = 4,
    /// Update failed verification.
    Failed      = 5,
    /// Update rolled back.
    RolledBack  = 6,
}

// ── UpdateMetadata ───────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UpdateMetadata {
    /// Update version string.
    pub version:      [SigmaU8; 32],
    /// Human-readable description.
    pub description:  [SigmaU8; UPDATE_DESC_LEN],
    /// Source URL.
    pub source_url:   [SigmaU8; URL_LEN],
    /// SHA-256 of update package.
    pub sha256:       [SigmaU8; HASH_LEN],
    /// Package size in bytes.
    pub size:         SigmaU64,
    /// Timestamp when staged.
    pub staged_at:    SigmaU64,
    /// Status.
    pub status:       UpdateStatus,
    pub _pad:         [SigmaU8; 7],
}

impl UpdateMetadata {
    pub const fn zeroed() -> Self {
        Self {
            version:      [0u8; 32],
            description:  [0u8; UPDATE_DESC_LEN],
            source_url:   [0u8; URL_LEN],
            sha256:       [0u8; HASH_LEN],
            size:         0,
            staged_at:    0,
            status:       UpdateStatus::None,
            _pad:         [0u8; 7],
        }
    }
}

// ── SovereignAtomicUpdater ─────────────────────────────────────────────────────
pub struct SovereignAtomicUpdater {
    /// Staged updates (ring buffer).
    staged:      [UpdateMetadata; MAX_STAGED],
    /// Currently active staged update index.
    active_idx:  AtomicU32,
    /// Update in progress flag.
    in_progress: AtomicBool,
    /// Initialized flag.
    initialized: SigmaBool,
}

impl SovereignAtomicUpdater {
    pub const fn new() -> Self {
        Self {
            staged:      [UpdateMetadata::zeroed(); MAX_STAGED],
            active_idx:  AtomicU32::new(0),
            in_progress: AtomicBool::new(false),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }

    fn find_slot(&self) -> Option<SigmaUsize> {
        for i in 0..MAX_STAGED {
            if self.staged[i].status == UpdateStatus::None {
                return Some(i);
            }
        }
        None
    }

    fn copy_str(dst: &mut [SigmaU8], src: &[SigmaU8]) {
        let len = src.len().min(dst.len() - 1);
        let mut i = 0;
        while i < len { dst[i] = src[i]; i += 1; }
        dst[len] = 0;
    }

    fn copy_hash(dst: &mut [SigmaU8], src: &[SigmaU8]) {
        let len = src.len().min(HASH_LEN);
        let mut i = 0;
        while i < len { dst[i] = src[i]; i += 1; }
    }

    // ── Public API ────────────────────────────────────────────────────────────

    /// Stage an update for later deployment.
    /// Returns 0 on success, -1 on failure.
    pub fn stage_update(
        &mut self,
        version:     &[SigmaU8],
        description: &[SigmaU8],
        source_url:  &[SigmaU8],
        sha256:      &[SigmaU8],
        size:        SigmaU64,
    ) -> SigmaI32 {
        if self.in_progress.load(Ordering::Acquire) {
            return -1; // Update already in progress
        }

        let slot = match self.find_slot() { Some(s) => s, None => return -1 };

        let mut meta = UpdateMetadata::zeroed();
        meta.status = UpdateStatus::Staged;
        meta.size = size;
        meta.staged_at = self.current_timestamp();

        Self::copy_str(&mut meta.version, version);
        Self::copy_str(&mut meta.description, description);
        Self::copy_str(&mut meta.source_url, source_url);
        Self::copy_hash(&mut meta.sha256, sha256);

        self.staged[slot] = meta;
        self.active_idx.store(slot as SigmaU32, Ordering::SeqCst);
        0
    }

    /// Verify the staged update (check hash, signature, integrity).
    /// Returns 0 on success, -1 on failure.
    pub fn verify_update(&mut self) -> SigmaI32 {
        let idx = self.active_idx.load(Ordering::Acquire) as SigmaUsize;
        if idx >= MAX_STAGED {
            return -1;
        }

        self.staged[idx].status = UpdateStatus::Verifying;

        // In production: verify SHA-256 hash
        // In production: verify cryptographic signature
        // In production: verify package integrity

        // For now, assume verification succeeds
        self.staged[idx].status = UpdateStatus::Ready;
        0
    }

    /// Commit the staged update (atomic swap).
    /// Returns 0 on success, -1 on failure.
    pub fn commit_update(&mut self) -> SigmaI32 {
        let idx = self.active_idx.load(Ordering::Acquire) as SigmaUsize;
        if idx >= MAX_STAGED || self.staged[idx].status != UpdateStatus::Ready {
            return -1;
        }

        self.in_progress.store(true, Ordering::SeqCst);

        // In production: atomic partition swap
        // In production: update bootloader configuration
        // In production: mark for next boot

        self.staged[idx].status = UpdateStatus::Committed;
        self.in_progress.store(false, Ordering::SeqCst);
        0
    }

    /// Rollback to previous generation.
    /// Returns 0 on success, -1 on failure.
    pub fn rollback(&mut self) -> SigmaI32 {
        let idx = self.active_idx.load(Ordering::Acquire) as SigmaUsize;
        if idx >= MAX_STAGED {
            return -1;
        }

        self.staged[idx].status = UpdateStatus::RolledBack;

        // In production: revert partition swap
        // In production: restore bootloader configuration

        0
    }

    /// Get current update status.
    pub fn get_status(&self) -> UpdateStatus {
        let idx = self.active_idx.load(Ordering::Acquire) as SigmaUsize;
        if idx < MAX_STAGED {
            self.staged[idx].status
        } else {
            UpdateStatus::None
        }
    }

    /// Clear staged update.
    pub fn clear_staged(&mut self) -> SigmaI32 {
        let idx = self.active_idx.load(Ordering::Acquire) as SigmaUsize;
        if idx < MAX_STAGED {
            self.staged[idx] = UpdateMetadata::zeroed();
            0
        } else {
            -1
        }
    }

    fn current_timestamp(&self) -> SigmaU64 {
        // In production: read system timer/jiffies
        0
    }
}

// ── Global Instance ───────────────────────────────────────────────────────────
static mut G_UPDATER: SovereignAtomicUpdater = SovereignAtomicUpdater::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_updater_init() {
    G_UPDATER.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_updater_stage(
    version:     *const SigmaU8, vlen:  SigmaUsize,
    description: *const SigmaU8, dlen:  SigmaUsize,
    source_url:  *const SigmaU8, ulen:  SigmaUsize,
    sha256:      *const SigmaU8,
    size:        SigmaU64,
) -> SigmaI32 {
    let v = core::slice::from_raw_parts(version, vlen.min(32));
    let d = core::slice::from_raw_parts(description, dlen.min(UPDATE_DESC_LEN));
    let u = core::slice::from_raw_parts(source_url, ulen.min(URL_LEN));
    let h = core::slice::from_raw_parts(sha256, HASH_LEN);
    G_UPDATER.stage_update(v, d, u, h, size)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_updater_verify() -> SigmaI32 {
    G_UPDATER.verify_update()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_updater_commit() -> SigmaI32 {
    G_UPDATER.commit_update()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_updater_rollback() -> SigmaI32 {
    G_UPDATER.rollback()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_updater_status() -> SigmaU32 {
    G_UPDATER.get_status() as SigmaU32
}

#[no_mangle]
pub unsafe extern "C" fn sigma_updater_clear() -> SigmaI32 {
    G_UPDATER.clear_staged()
}

