// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: sigpkg — Package Manager Core (Rust, no_std)
//! =========================================================================
//!
//! Core package management struct: SigPkgManager
//! Replaces: usr/SovereignPkgManager.cpp
//!
//! Features:
//!   - Static package registry (no heap allocation)
//!   - Ed25519 signature verification stubs (sovereign crypto)
//!   - Merkle-hash content addressing
//!   - Sovereign syscall ABI integration
//! =========================================================================

pub type SigmaStatus = i32;
pub const SIGMA_OK:    SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;

type U32 = u32;
type U64 = u64;

/// Maximum number of simultaneously tracked packages.
pub const MAX_PACKAGES: usize = 512;

/// Length of a SHA-256 / Ed25519-compatible hash (32 bytes).
const HASH_LEN: usize = 32;

// ── Data Structures (OOP-style structs) ───────────────────────────────────

/// Metadata record for a single installed package.
#[derive(Clone, Copy)]
pub struct SigPkgRecord {
    name_hash:   U64,           // FNV-1a hash of the package name
    version:     U32,           // Packed major.minor.patch (8+12+12 bits)
    installed:   bool,
    sig_valid:   bool,
    content_hash: [u8; HASH_LEN],
}

impl SigPkgRecord {
    pub const fn empty() -> Self {
        SigPkgRecord {
            name_hash:    0,
            version:      0,
            installed:    false,
            sig_valid:    false,
            content_hash: [0u8; HASH_LEN],
        }
    }

    pub fn is_active(&self) -> bool {
        self.installed
    }
}

// ── Manager ────────────────────────────────────────────────────────────────

/// Sovereign Package Manager — owns the static package registry.
pub struct SigPkgManager {
    packages:  [SigPkgRecord; MAX_PACKAGES],
    count:     U32,
    initialized: bool,
}

impl SigPkgManager {
    pub const fn new() -> Self {
        SigPkgManager {
            packages:    [SigPkgRecord::empty(); MAX_PACKAGES],
            count:       0,
            initialized: false,
        }
    }

    // ── Lifecycle ──────────────────────────────────────────────────────────

    pub fn init(&mut self) -> SigmaStatus {
        if self.initialized { return SIGMA_OK; }
        // TODO: mount sovereign package index from SigmaFS
        self.initialized = true;
        SIGMA_OK
    }

    // ── FNV-1a hash (hand-rolled, no stdlib) ──────────────────────────────

    fn fnv1a_hash(bytes: &[u8]) -> U64 {
        let mut hash: U64 = 0xcbf29ce484222325;
        let mut i = 0;
        while i < bytes.len() {
            hash ^= bytes[i] as U64;
            hash = hash.wrapping_mul(0x100000001b3);
            i += 1;
        }
        hash
    }

    // ── Core Operations ────────────────────────────────────────────────────

    /// Register a new package into the sovereign registry.
    pub fn install(
        &mut self,
        name:         &[u8],
        version:      U32,
        content_hash: &[u8; HASH_LEN],
        sig_valid:    bool,
    ) -> SigmaStatus {
        if !self.initialized       { return SIGMA_ERROR; }
        if self.count as usize >= MAX_PACKAGES { return SIGMA_ERROR; }

        let name_hash = Self::fnv1a_hash(name);
        let slot = self.count as usize;

        self.packages[slot].name_hash    = name_hash;
        self.packages[slot].version      = version;
        self.packages[slot].installed    = true;
        self.packages[slot].sig_valid    = sig_valid;

        let mut i = 0;
        while i < HASH_LEN {
            self.packages[slot].content_hash[i] = content_hash[i];
            i += 1;
        }

        self.count += 1;
        SIGMA_OK
    }

    /// Remove a package by name hash.
    pub fn remove(&mut self, name: &[u8]) -> SigmaStatus {
        if !self.initialized { return SIGMA_ERROR; }
        let name_hash = Self::fnv1a_hash(name);
        let mut i = 0;
        while i < self.count as usize {
            if self.packages[i].name_hash == name_hash {
                self.packages[i] = SigPkgRecord::empty();
                return SIGMA_OK;
            }
            i += 1;
        }
        SIGMA_ERROR
    }

    /// Verify whether a package signature is valid.
    pub fn verify(&self, name: &[u8]) -> SigmaStatus {
        let name_hash = Self::fnv1a_hash(name);
        let mut i = 0;
        while i < self.count as usize {
            if self.packages[i].name_hash == name_hash {
                return if self.packages[i].sig_valid { SIGMA_OK } else { SIGMA_ERROR };
            }
            i += 1;
        }
        SIGMA_ERROR
    }

    pub fn installed_count(&self) -> U32 { self.count }

    /// List all installed packages by iterating the registry
    pub fn list(&self, names: &mut [[u8; 64]; MAX_PACKAGES]) -> U32 {
        let mut count = 0u32;
        let mut i = 0;
        while i < self.count as usize {
            if self.packages[i].installed {
                // In a real implementation, we'd have a name-to-hash reverse lookup
                // For now, return the hash as a placeholder
                let hash_bytes = self.packages[i].name_hash.to_le_bytes();
                let mut j = 0;
                while j < 8 && j < 64 {
                    names[count as usize][j] = hash_bytes[j];
                    j += 1;
                }
                count += 1;
            }
            i += 1;
        }
        count
    }

    /// Search for packages by partial name hash match
    pub fn search(&self, pattern: &[u8]) -> U32 {
        let pattern_hash = Self::fnv1a_hash(pattern);
        let mut count = 0u32;
        let mut i = 0;
        while i < self.count as usize {
            if self.packages[i].name_hash == pattern_hash {
                count += 1;
            }
            i += 1;
        }
        count
    }

    /// Update a package to a new version
    pub fn update(&mut self, name: &[u8], new_version: U32) -> SigmaStatus {
        if !self.initialized { return SIGMA_ERROR; }
        let name_hash = Self::fnv1a_hash(name);
        let mut i = 0;
        while i < self.count as usize {
            if self.packages[i].name_hash == name_hash {
                self.packages[i].version = new_version;
                return SIGMA_OK;
            }
            i += 1;
        }
        SIGMA_ERROR
    }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_PKG_MGR: SigPkgManager = SigPkgManager::new();

// ── C-ABI Exports (Replacing SovereignPkgManager.cpp / sigma_pkg.c) ────────

#[no_mangle]
pub unsafe extern "C" fn sigpkg_init() -> SigmaStatus {
    G_PKG_MGR.init()
}

#[no_mangle]
pub unsafe extern "C" fn sigpkg_install(
    name:         *const u8,
    name_len:     U32,
    version:      U32,
    content_hash: *const u8,
    sig_valid:    u8,
) -> SigmaStatus {
    let name_slice = core::slice::from_raw_parts(name, name_len as usize);
    let hash_arr   = &*(content_hash as *const [u8; HASH_LEN]);
    G_PKG_MGR.install(name_slice, version, hash_arr, sig_valid != 0)
}

#[no_mangle]
pub unsafe extern "C" fn sigpkg_remove(name: *const u8, name_len: U32) -> SigmaStatus {
    let name_slice = core::slice::from_raw_parts(name, name_len as usize);
    G_PKG_MGR.remove(name_slice)
}

#[no_mangle]
pub unsafe extern "C" fn sigpkg_verify(name: *const u8, name_len: U32) -> SigmaStatus {
    let name_slice = core::slice::from_raw_parts(name, name_len as usize);
    G_PKG_MGR.verify(name_slice)
}

#[no_mangle]
pub unsafe extern "C" fn sigpkg_count() -> U32 {
    G_PKG_MGR.installed_count()
}

#[no_mangle]
pub unsafe extern "C" fn sigpkg_list(names: *mut [u8; 64]) -> U32 {
    let names_slice = core::slice::from_raw_parts_mut(names, MAX_PACKAGES);
    G_PKG_MGR.list(names_slice)
}

#[no_mangle]
pub unsafe extern "C" fn sigpkg_search(pattern: *const u8, pattern_len: U32) -> U32 {
    let pattern_slice = core::slice::from_raw_parts(pattern, pattern_len as usize);
    G_PKG_MGR.search(pattern_slice)
}

#[no_mangle]
pub unsafe extern "C" fn sigpkg_update(name: *const u8, name_len: U32, new_version: U32) -> SigmaStatus {
    let name_slice = core::slice::from_raw_parts(name, name_len as usize);
    G_PKG_MGR.update(name_slice, new_version)
}
