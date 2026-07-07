// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/recovery/sigma_atomic_rollback.rs — Atomic System Generations
//
// Implements NixOS-inspired atomic system rollback for SigmaOS.
// Each "generation" is a CoW snapshot of the system state:
//   - Root filesystem hash
//   - Package set hash
//   - Kernel version
//   - Config snapshot
//
// Design:
//   - Up to MAX_GENERATIONS generations stored in a ring buffer
//   - Each generation is immutable once created
//   - Rollback = change active generation + reboot into it
//   - Boot menu integration via sigma_boot_menu_entry()
//
// Inspired by:
//   - NixOS generations (nix-env --rollback)
//   - OSTree (rpm-ostree, Fedora Silverblue)
//   - BTRFS/ZFS snapshot-based boot
//
// Language: Rust #![no_std] — no alloc, no external crates.
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, Ordering};

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── Constants ─────────────────────────────────────────────────────────────────
/// Maximum number of concurrent generations retained.
const MAX_GENERATIONS: SigmaUsize = 16;
/// Length of a generation description string.
const GEN_DESC_LEN: SigmaUsize = 64;
/// Length of a kernel version string.
const KVER_LEN: SigmaUsize = 32;
/// SHA-256 hash length in bytes.
const HASH_LEN: SigmaUsize = 32;
/// Maximum number of rollback hooks.
const MAX_HOOKS: SigmaUsize = 16;
/// Hook command length.
const HOOK_CMD_LEN: SigmaUsize = 128;

/// Special sentinel: no active generation.
const GEN_NONE: SigmaU32 = u32::MAX;

// ── Generation Status ─────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GenStatus {
    /// Slot is empty.
    Empty   = 0,
    /// Generation is complete and bootable.
    Valid   = 1,
    /// Generation is currently booted.
    Active  = 2,
    /// Generation has been superseded but kept for rollback.
    Old     = 3,
    /// Generation is corrupted / hash mismatch.
    Corrupt = 4,
}

// ── Hook Type ───────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HookType {
    /// Hook runs before rollback.
    PreRollback = 0,
    /// Hook runs after rollback.
    PostRollback = 1,
    /// Hook runs before generation creation.
    PreCreate = 2,
    /// Hook runs after generation creation.
    PostCreate = 3,
}

// ── Rollback Hook ───────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RollbackHook {
    pub hook_type: HookType,
    pub command: [SigmaU8; HOOK_CMD_LEN],
    pub enabled: SigmaBool,
    pub _pad: [SigmaU8; 7],
}

impl RollbackHook {
    pub const fn zeroed() -> Self {
        Self {
            hook_type: HookType::PreRollback,
            command: [0u8; HOOK_CMD_LEN],
            enabled: false,
            _pad: [0u8; 7],
        }
    }
}

// ── SystemGeneration — one generation record ──────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SystemGeneration {
    /// Monotonically increasing generation number (starts at 1).
    pub id:            SigmaU32,
    /// Unix timestamp (jiffies) when this generation was created.
    pub created_at:    SigmaU64,
    /// SHA-256 of the root filesystem tree.
    pub root_hash:     [SigmaU8; HASH_LEN],
    /// SHA-256 of the installed package set.
    pub pkg_hash:      [SigmaU8; HASH_LEN],
    /// Kernel version string ("sigmaos-0.9.0-x86_64\0...").
    pub kernel_ver:    [SigmaU8; KVER_LEN],
    /// Human-readable description.
    pub description:   [SigmaU8; GEN_DESC_LEN],
    /// LBA address of the CoW snapshot on disk (0 = RAM-only).
    pub snapshot_lba:  SigmaU64,
    /// Size of the CoW snapshot in bytes.
    pub snapshot_size: SigmaU64,
    /// Status.
    pub status:        GenStatus,
    pub _pad:          [SigmaU8; 7],
}

impl SystemGeneration {
    pub const fn zeroed() -> Self {
        Self {
            id:            0,
            created_at:    0,
            root_hash:     [0u8; HASH_LEN],
            pkg_hash:      [0u8; HASH_LEN],
            kernel_ver:    [0u8; KVER_LEN],
            description:   [0u8; GEN_DESC_LEN],
            snapshot_lba:  0,
            snapshot_size: 0,
            status:        GenStatus::Empty,
            _pad:          [0u8; 7],
        }
    }
}

// ── GenerationManager ─────────────────────────────────────────────────────────
pub struct GenerationManager {
    gens:        [SystemGeneration; MAX_GENERATIONS],
    hooks:       [RollbackHook; MAX_HOOKS],
    count:       SigmaUsize,
    hook_count:  SigmaUsize,
    active_id:   AtomicU32,
    next_gen_id: AtomicU32,
    initialized: SigmaBool,
}

impl GenerationManager {
    pub const fn new() -> Self {
        Self {
            gens:        [SystemGeneration::zeroed(); MAX_GENERATIONS],
            hooks:       [RollbackHook::zeroed(); MAX_HOOKS],
            count:       0,
            hook_count:  0,
            active_id:   AtomicU32::new(GEN_NONE),
            next_gen_id: AtomicU32::new(1),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
        // Generation 1 = the initial boot state (created at genesis).
        let _ = self.create_generation(
            b"Initial system generation",
            &[0xAB; HASH_LEN], // placeholder root hash
            &[0xCD; HASH_LEN], // placeholder pkg hash
            b"sigmaos-0.9.0",
        );
        self.active_id.store(1, Ordering::SeqCst);
    }

    fn find_slot(&self) -> Option<SigmaUsize> {
        for i in 0..MAX_GENERATIONS {
            if self.gens[i].status == GenStatus::Empty { return Some(i); }
        }
        None
    }

    fn find_by_id(&self, id: SigmaU32) -> Option<SigmaUsize> {
        for i in 0..MAX_GENERATIONS {
            if self.gens[i].id == id
            && self.gens[i].status != GenStatus::Empty {
                return Some(i);
            }
        }
        None
    }

    /// Copy up to `src.len().min(dst.len()-1)` bytes from `src` into `dst`, NUL-terminating.
    fn copy_str(dst: &mut [SigmaU8], src: &[SigmaU8]) {
        let len = src.len().min(dst.len() - 1);
        let mut i = 0;
        while i < len { dst[i] = src[i]; i += 1; }
        dst[len] = 0;
    }

    // ── Public API ────────────────────────────────────────────────────────────

    /// Create a new generation and return its ID, or 0 on failure.
    pub fn create_generation(
        &mut self,
        desc:       &[SigmaU8],
        root_hash:  &[SigmaU8],
        pkg_hash:   &[SigmaU8],
        kernel_ver: &[SigmaU8],
    ) -> SigmaU32 {
        let slot = match self.find_slot() { Some(s) => s, None => return 0 };

        let gen_id = self.next_gen_id.fetch_add(1, Ordering::Relaxed);
        let mut g = SystemGeneration::zeroed();
        g.id     = gen_id;
        g.status = GenStatus::Valid;

        Self::copy_str(&mut g.description, desc);
        Self::copy_str(&mut g.kernel_ver,  kernel_ver);

        // Copy hashes.
        let rlen = root_hash.len().min(HASH_LEN);
        let plen = pkg_hash.len().min(HASH_LEN);
        let mut i = 0;
        while i < rlen { g.root_hash[i] = root_hash[i]; i += 1; }
        let mut i = 0;
        while i < plen { g.pkg_hash[i] = pkg_hash[i]; i += 1; }

        self.gens[slot] = g;
        self.count += 1;
        gen_id
    }

    /// Mark `gen_id` as the active (currently-booted) generation.
    /// Returns 0 on success, -1 if not found.
    pub fn set_active(&mut self, gen_id: SigmaU32) -> SigmaI32 {
        // Clear old active.
        for i in 0..MAX_GENERATIONS {
            if self.gens[i].status == GenStatus::Active {
                self.gens[i].status = GenStatus::Old;
            }
        }
        match self.find_by_id(gen_id) {
            Some(idx) => {
                self.gens[idx].status = GenStatus::Active;
                self.active_id.store(gen_id, Ordering::SeqCst);
                0
            }
            None => -1,
        }
    }

    /// Roll back to `gen_id` — marks it as the next boot target.
    /// The actual reboot is performed by the caller.
    /// Returns 0 on success, -1 if generation not found/valid.
    pub fn rollback_to(&mut self, gen_id: SigmaU32) -> SigmaI32 {
        match self.find_by_id(gen_id) {
            Some(idx) if self.gens[idx].status != GenStatus::Corrupt => {
                // NixOS pattern: simply activate the old generation.
                self.set_active(gen_id)
            }
            _ => -1,
        }
    }

    /// Delete a generation (free its slot). Cannot delete Active generation.
    pub fn delete_generation(&mut self, gen_id: SigmaU32) -> SigmaI32 {
        match self.find_by_id(gen_id) {
            Some(idx) if self.gens[idx].status != GenStatus::Active => {
                self.gens[idx] = SystemGeneration::zeroed();
                self.count = self.count.saturating_sub(1);
                0
            }
            _ => -1,
        }
    }

    /// List all non-empty generations into `out` buffer.
    /// Returns number of entries written.
    pub fn list(&self, out: *mut SystemGeneration, max: SigmaUsize) -> SigmaUsize {
        let mut written = 0;
        for i in 0..MAX_GENERATIONS {
            if written >= max { break; }
            if self.gens[i].status != GenStatus::Empty {
                unsafe { core::ptr::write(out.add(written), self.gens[i]); }
                written += 1;
            }
        }
        written
    }

    // ── Hook Management ───────────────────────────────────────────────────────

    /// Register a rollback hook.
    pub fn register_hook(&mut self, hook_type: HookType, command: &[SigmaU8]) -> SigmaI32 {
        if self.hook_count >= MAX_HOOKS {
            return -1;
        }
        let idx = self.hook_count;
        self.hooks[idx].hook_type = hook_type;
        self.hooks[idx].enabled = true;
        let len = command.len().min(HOOK_CMD_LEN - 1);
        let mut i = 0;
        while i < len { self.hooks[idx].command[i] = command[i]; i += 1; }
        self.hooks[idx].command[len] = 0;
        self.hook_count += 1;
        0
    }

    /// Run hooks of a specific type.
    pub fn run_hooks(&self, hook_type: HookType) {
        for i in 0..self.hook_count {
            if self.hooks[i].hook_type == hook_type && self.hooks[i].enabled {
                // In production: execute hook command
                // For now, just mark as executed
            }
        }
    }

    /// Remove a hook by index.
    pub fn remove_hook(&mut self, index: SigmaUsize) -> SigmaI32 {
        if index >= self.hook_count {
            return -1;
        }
        self.hooks[index] = RollbackHook::zeroed();
        self.hook_count -= 1;
        0
    }

    pub fn active_id(&self) -> SigmaU32 { self.active_id.load(Ordering::Relaxed) }
    pub fn generation_count(&self) -> SigmaUsize { self.count }
    pub fn hook_count(&self) -> SigmaUsize { self.hook_count }
}

// ── Global Instance ───────────────────────────────────────────────────────────
static mut G_GEN_MGR: GenerationManager = GenerationManager::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_rollback_init() {
    G_GEN_MGR.init();
}

/// Create a new generation. Returns generation ID (>0) or 0 on failure.
#[no_mangle]
pub unsafe extern "C" fn sigma_generation_create(
    desc:       *const SigmaU8, desc_len:  SigmaUsize,
    root_hash:  *const SigmaU8,
    pkg_hash:   *const SigmaU8,
    kernel_ver: *const SigmaU8, kver_len:  SigmaUsize,
) -> SigmaU32 {
    let d  = core::slice::from_raw_parts(desc,       desc_len.min(GEN_DESC_LEN));
    let rh = core::slice::from_raw_parts(root_hash,  HASH_LEN);
    let ph = core::slice::from_raw_parts(pkg_hash,   HASH_LEN);
    let kv = core::slice::from_raw_parts(kernel_ver, kver_len.min(KVER_LEN));
    G_GEN_MGR.create_generation(d, rh, ph, kv)
}

/// Roll back to a specific generation. Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn sigma_generation_rollback(gen_id: SigmaU32) -> SigmaI32 {
    G_GEN_MGR.rollback_to(gen_id)
}

/// Set the active (booted) generation. Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn sigma_generation_set_active(gen_id: SigmaU32) -> SigmaI32 {
    G_GEN_MGR.set_active(gen_id)
}

/// Delete a generation. Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn sigma_generation_delete(gen_id: SigmaU32) -> SigmaI32 {
    G_GEN_MGR.delete_generation(gen_id)
}

/// Returns the currently active generation ID.
#[no_mangle]
pub unsafe extern "C" fn sigma_active_generation() -> SigmaU32 {
    G_GEN_MGR.active_id()
}

/// Returns the total number of stored generations.
#[no_mangle]
pub unsafe extern "C" fn sigma_generation_count() -> SigmaU32 {
    G_GEN_MGR.generation_count() as SigmaU32
}

/// Fill `out` with up to `max` SystemGeneration entries.
/// Returns actual count written.
#[no_mangle]
pub unsafe extern "C" fn sigma_generation_list(
    out: *mut SystemGeneration, max: SigmaU32,
) -> SigmaU32 {
    G_GEN_MGR.list(out, max as SigmaUsize) as SigmaU32
}

/// Register a rollback hook. Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn sigma_hook_register(
    hook_type: SigmaU32,
    command: *const SigmaU8,
    cmd_len: SigmaUsize,
) -> SigmaI32 {
    let ht = match hook_type {
        0 => HookType::PreRollback,
        1 => HookType::PostRollback,
        2 => HookType::PreCreate,
        3 => HookType::PostCreate,
        _ => HookType::PreRollback,
    };
    let cmd = core::slice::from_raw_parts(command, cmd_len.min(HOOK_CMD_LEN));
    G_GEN_MGR.register_hook(ht, cmd)
}

/// Run hooks of a specific type.
#[no_mangle]
pub unsafe extern "C" fn sigma_hook_run(hook_type: SigmaU32) {
    let ht = match hook_type {
        0 => HookType::PreRollback,
        1 => HookType::PostRollback,
        2 => HookType::PreCreate,
        3 => HookType::PostCreate,
        _ => HookType::PreRollback,
    };
    G_GEN_MGR.run_hooks(ht);
}

/// Remove a hook by index. Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn sigma_hook_remove(index: SigmaU32) -> SigmaI32 {
    G_GEN_MGR.remove_hook(index as SigmaUsize)
}

/// Returns the total number of registered hooks.
#[no_mangle]
pub unsafe extern "C" fn sigma_hook_count() -> SigmaU32 {
    G_GEN_MGR.hook_count() as SigmaU32
}
