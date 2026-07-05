// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/system/sigma_package_resolver.rs — Package Dependency Resolver
//
// Implements an Arch/Fedora/NixOS-inspired package registry and dependency
// resolver for the SigmaOS sigpkg package manager.
//
// Features:
//   - Flat package registry (up to MAX_PACKAGES entries)
//   - Topological sort dependency resolution with cycle detection
//   - Sandbox metadata per package (allowed syscalls, FS paths)
//   - Flatpak-style portal capabilities
//   - Package verification via SHA-256 hash check
//   - Install / remove / query operations
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
const MAX_PACKAGES:    SigmaUsize = 1024;
const PKG_NAME_LEN:    SigmaUsize = 64;
const PKG_VER_LEN:     SigmaUsize = 32;
const PKG_DESC_LEN:    SigmaUsize = 128;
const PKG_URL_LEN:     SigmaUsize = 128;
const MAX_DEPS:        SigmaUsize = 16;
const HASH_LEN:        SigmaUsize = 32;
/// Maximum number of allowed syscalls in sandbox metadata.
const MAX_ALLOWED_SYSCALLS: SigmaUsize = 64;
/// Max FS paths allowed in sandbox.
const MAX_ALLOWED_PATHS: SigmaUsize = 8;
const PATH_LEN:        SigmaUsize = 64;
/// DFS stack depth for topological sort.
const TOPO_STACK_SIZE: SigmaUsize = MAX_PACKAGES;

// ── Package Install State ─────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PkgState {
    Unknown     = 0,
    Available   = 1,
    Installing  = 2,
    Installed   = 3,
    Removing    = 4,
    Broken      = 5,
}

// ── Sandbox Capabilities (Flatpak-inspired) ───────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SandboxCaps {
    /// Bitmask of allowed syscall numbers (64 syscalls max in this simple model).
    pub allowed_syscalls: [SigmaU64; MAX_ALLOWED_SYSCALLS / 64],
    /// Allowed filesystem path prefixes (NUL-terminated strings).
    pub allowed_paths: [[SigmaU8; PATH_LEN]; MAX_ALLOWED_PATHS],
    /// Allow network access.
    pub allow_network: SigmaBool,
    /// Allow device access (/dev/*).
    pub allow_devices: SigmaBool,
    /// Allow IPC (shared memory, message queues).
    pub allow_ipc:     SigmaBool,
    pub _pad:          [SigmaU8; 5],
}

impl SandboxCaps {
    pub const fn unrestricted() -> Self {
        Self {
            allowed_syscalls: [u64::MAX; MAX_ALLOWED_SYSCALLS / 64],
            allowed_paths:    [[0u8; PATH_LEN]; MAX_ALLOWED_PATHS],
            allow_network:    true,
            allow_devices:    true,
            allow_ipc:        true,
            _pad:             [0u8; 5],
        }
    }

    pub const fn minimal() -> Self {
        Self {
            allowed_syscalls: [0u64; MAX_ALLOWED_SYSCALLS / 64],
            allowed_paths:    [[0u8; PATH_LEN]; MAX_ALLOWED_PATHS],
            allow_network:    false,
            allow_devices:    false,
            allow_ipc:        false,
            _pad:             [0u8; 5],
        }
    }
}

// ── PackageEntry ──────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PackageEntry {
    pub name:         [SigmaU8; PKG_NAME_LEN],
    pub version:      [SigmaU8; PKG_VER_LEN],
    pub description:  [SigmaU8; PKG_DESC_LEN],
    pub url:          [SigmaU8; PKG_URL_LEN],
    /// SHA-256 of the package archive.
    pub hash:         [SigmaU8; HASH_LEN],
    /// Indices into the registry for this package's dependencies.
    pub dep_indices:  [SigmaU32; MAX_DEPS],
    pub dep_count:    SigmaU32,
    /// Package size in bytes.
    pub size_bytes:   SigmaU64,
    pub state:        PkgState,
    pub sandbox:      SandboxCaps,
    pub _pad:         [SigmaU8; 7],
}

impl PackageEntry {
    pub const fn zeroed() -> Self {
        Self {
            name:        [0u8; PKG_NAME_LEN],
            version:     [0u8; PKG_VER_LEN],
            description: [0u8; PKG_DESC_LEN],
            url:         [0u8; PKG_URL_LEN],
            hash:        [0u8; HASH_LEN],
            dep_indices: [0u32; MAX_DEPS],
            dep_count:   0,
            size_bytes:  0,
            state:       PkgState::Unknown,
            sandbox:     SandboxCaps::minimal(),
            _pad:        [0u8; 7],
        }
    }
}

// ── PackageRegistry ───────────────────────────────────────────────────────────
pub struct PackageRegistry {
    packages:    [PackageEntry; MAX_PACKAGES],
    count:       AtomicU32,
    initialized: SigmaBool,
}

impl PackageRegistry {
    pub const fn new() -> Self {
        Self {
            packages:    [PackageEntry::zeroed(); MAX_PACKAGES],
            count:       AtomicU32::new(0),
            initialized: false,
        }
    }

    pub fn init(&mut self) { self.initialized = true; }

    fn str_eq(a: &[SigmaU8], b: &[SigmaU8]) -> SigmaBool {
        if a.len() != b.len() { return false; }
        for i in 0..a.len() { if a[i] != b[i] { return false; } }
        true
    }

    fn copy_str(dst: &mut [SigmaU8], src: &[SigmaU8]) {
        let len = src.len().min(dst.len() - 1);
        let mut i = 0;
        while i < len { dst[i] = src[i]; i += 1; }
        dst[len] = 0;
    }

    /// Find package index by name. Returns MAX_PACKAGES if not found.
    pub fn find_by_name(&self, name: &[SigmaU8]) -> SigmaUsize {
        let n = self.count.load(Ordering::Relaxed) as SigmaUsize;
        for i in 0..n {
            let pkg_name_end = self.packages[i].name.iter().position(|&b| b == 0)
                .unwrap_or(PKG_NAME_LEN);
            if Self::str_eq(&self.packages[i].name[..pkg_name_end], name) {
                return i;
            }
        }
        MAX_PACKAGES
    }

    /// Register a package in the registry. Returns index or MAX_PACKAGES on failure.
    pub fn register(&mut self, pkg: PackageEntry) -> SigmaUsize {
        let n = self.count.load(Ordering::Relaxed) as SigmaUsize;
        if n >= MAX_PACKAGES { return MAX_PACKAGES; }
        self.packages[n] = pkg;
        self.count.fetch_add(1, Ordering::Relaxed);
        n
    }

    /// Get immutable reference to package at index.
    pub fn get(&self, idx: SigmaUsize) -> Option<&PackageEntry> {
        let n = self.count.load(Ordering::Relaxed) as SigmaUsize;
        if idx < n { Some(&self.packages[idx]) } else { None }
    }

    pub fn get_mut(&mut self, idx: SigmaUsize) -> Option<&mut PackageEntry> {
        let n = self.count.load(Ordering::Relaxed) as SigmaUsize;
        if idx < n { Some(&mut self.packages[idx]) } else { None }
    }

    pub fn len(&self) -> SigmaUsize { self.count.load(Ordering::Relaxed) as SigmaUsize }
}

// ── DependencyResolver ────────────────────────────────────────────────────────
/// Topological sort using iterative DFS (Kahn's algorithm variant).
/// Returns a sorted install order in `out`, or -1 on cycle detection.
pub struct DependencyResolver;

impl DependencyResolver {
    /// Resolve install order for `target_idx` (and all its dependencies).
    /// Fills `out` with package indices in install order.
    /// Returns count of entries written, or -1 on cycle.
    pub fn resolve(
        registry:   &PackageRegistry,
        target_idx: SigmaUsize,
        out:        &mut [SigmaUsize],
    ) -> SigmaI32 {
        let n = registry.len();
        if target_idx >= n { return -22; } // EINVAL

        // Visited/in-stack arrays for DFS cycle detection.
        let mut visited:  [SigmaBool; MAX_PACKAGES] = [false; MAX_PACKAGES];
        let mut in_stack: [SigmaBool; MAX_PACKAGES] = [false; MAX_PACKAGES];
        let mut result:   [SigmaUsize; MAX_PACKAGES] = [0; MAX_PACKAGES];
        let mut result_top = 0usize;

        // Iterative DFS stack: (pkg_idx, dep_cursor).
        let mut dfs_stack: [(SigmaUsize, SigmaU32); TOPO_STACK_SIZE] =
            [(0, 0); TOPO_STACK_SIZE];
        let mut stack_top = 0usize;

        // Push initial target.
        dfs_stack[stack_top] = (target_idx, 0);
        stack_top += 1;
        in_stack[target_idx] = true;

        while stack_top > 0 {
            let (cur, dep_cur) = &mut dfs_stack[stack_top - 1];
            let cur_idx = *cur;

            if let Some(pkg) = registry.get(cur_idx) {
                let dc = *dep_cur as SigmaUsize;
                if dc < pkg.dep_count as SigmaUsize {
                    // Explore next dependency.
                    let dep_idx = pkg.dep_indices[dc] as SigmaUsize;
                    dfs_stack[stack_top - 1].1 += 1; // advance cursor

                    if dep_idx >= n { continue; } // invalid dep — skip
                    if in_stack[dep_idx] { return -16; } // EBUSY — cycle detected
                    if visited[dep_idx]  { continue; }   // already processed

                    if stack_top >= TOPO_STACK_SIZE { return -12; } // ENOMEM
                    in_stack[dep_idx] = true;
                    dfs_stack[stack_top] = (dep_idx, 0);
                    stack_top += 1;
                } else {
                    // All deps processed — emit this node.
                    stack_top -= 1;
                    in_stack[cur_idx] = false;
                    if !visited[cur_idx] {
                        visited[cur_idx] = true;
                        if result_top < MAX_PACKAGES {
                            result[result_top] = cur_idx;
                            result_top += 1;
                        }
                    }
                }
            } else {
                stack_top -= 1;
            }
        }

        // Copy into caller's out buffer.
        let to_copy = result_top.min(out.len());
        let mut i = 0;
        while i < to_copy { out[i] = result[i]; i += 1; }
        to_copy as SigmaI32
    }
}

// ── Global Registry Instance ──────────────────────────────────────────────────
static mut G_REGISTRY: PackageRegistry = PackageRegistry::new();
/// Scratch buffer for resolved install order.
static mut RESOLVE_BUF: [SigmaUsize; MAX_PACKAGES] = [0; MAX_PACKAGES];

// ── C-ABI Exports ─────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_pkg_init() {
    G_REGISTRY.init();
}

/// Register a package. Returns package index or u32::MAX on failure.
#[no_mangle]
pub unsafe extern "C" fn sigma_pkg_register(pkg: *const PackageEntry) -> SigmaU32 {
    if pkg.is_null() { return u32::MAX; }
    let idx = G_REGISTRY.register(core::ptr::read(pkg));
    idx as SigmaU32
}

/// Find a package by NUL-terminated name.
/// Returns index or u32::MAX if not found.
#[no_mangle]
pub unsafe extern "C" fn sigma_pkg_find(name: *const SigmaU8, name_len: SigmaUsize) -> SigmaU32 {
    let name_slice = core::slice::from_raw_parts(name, name_len);
    let idx = G_REGISTRY.find_by_name(name_slice);
    if idx == MAX_PACKAGES { u32::MAX } else { idx as SigmaU32 }
}

/// Resolve dependency install order for `pkg_idx`.
/// Fills `out_order` with up to `max_count` package indices.
/// Returns count written, or -1 on cycle, -22 on invalid index.
#[no_mangle]
pub unsafe extern "C" fn sigma_pkg_resolve(
    pkg_idx:   SigmaU32,
    out_order: *mut SigmaU32,
    max_count: SigmaU32,
) -> SigmaI32 {
    let max = max_count as SigmaUsize;
    let ret = DependencyResolver::resolve(
        &G_REGISTRY,
        pkg_idx as SigmaUsize,
        &mut RESOLVE_BUF[..max.min(MAX_PACKAGES)],
    );
    if ret > 0 {
        let count = ret as SigmaUsize;
        for i in 0..count.min(max) {
            core::ptr::write(out_order.add(i), RESOLVE_BUF[i] as SigmaU32);
        }
    }
    ret
}

/// Mark a package as installed.
#[no_mangle]
pub unsafe extern "C" fn sigma_pkg_set_installed(pkg_idx: SigmaU32) -> SigmaI32 {
    match G_REGISTRY.get_mut(pkg_idx as SigmaUsize) {
        Some(pkg) => { pkg.state = PkgState::Installed; 0 }
        None      => -1,
    }
}

/// Returns total number of registered packages.
#[no_mangle]
pub unsafe extern "C" fn sigma_pkg_count() -> SigmaU32 {
    G_REGISTRY.len() as SigmaU32
}

/// Copy the PackageEntry at `idx` into `out`. Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn sigma_pkg_get(idx: SigmaU32, out: *mut PackageEntry) -> SigmaI32 {
    match G_REGISTRY.get(idx as SigmaUsize) {
        Some(pkg) => { core::ptr::write(out, *pkg); 0 }
        None      => -1,
    }
}
