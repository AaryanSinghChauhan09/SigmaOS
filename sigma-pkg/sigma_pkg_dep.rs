// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Package Manager (sigpkg) - Dependency Resolver
//! Minimal SAT-lite resolution.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaI32 = i32;

pub const MAX_DEPS_PER_PKG: usize = 8;
pub const MAX_RESOLVE_QUEUE: usize = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PkgDep {
    pub name: [u8; 32],
    pub min_version: SigmaU32,
    pub required: bool,
}

// Stub function representing checking installed database
extern "C" {
    fn sigma_pkg_is_installed(name: *const u8) -> bool;
}

// In a real implementation, this would look up the repo DB.
// We simulate finding dependencies for a package.
unsafe fn get_package_deps(pkg_name: *const u8, out_deps: *mut PkgDep, max_deps: usize) -> usize {
    // Stub: returns 0 dependencies.
    0
}

/// SAT-lite dependency resolver state
static mut RESOLVE_QUEUE: [[u8; 32]; MAX_RESOLVE_QUEUE] = [[0; 32]; MAX_RESOLVE_QUEUE];
static mut RESOLVE_HEAD: usize = 0;
static mut RESOLVE_TAIL: usize = 0;

unsafe fn queue_push(name: *const u8) -> bool {
    let next = (RESOLVE_TAIL + 1) % MAX_RESOLVE_QUEUE;
    if next == RESOLVE_HEAD { return false; }
    
    let mut i = 0;
    while i < 31 {
        let c = *name.add(i);
        RESOLVE_QUEUE[RESOLVE_TAIL][i] = c;
        if c == 0 { break; }
        i += 1;
    }
    RESOLVE_QUEUE[RESOLVE_TAIL][31] = 0;
    RESOLVE_TAIL = next;
    true
}

unsafe fn queue_pop(name_out: *mut u8) -> bool {
    if RESOLVE_HEAD == RESOLVE_TAIL { return false; }
    for i in 0..32 {
        *name_out.add(i) = RESOLVE_QUEUE[RESOLVE_HEAD][i];
    }
    RESOLVE_HEAD = (RESOLVE_HEAD + 1) % MAX_RESOLVE_QUEUE;
    true
}

/// Resolve all dependencies for a target package.
/// Fills an output buffer with names of packages that must be installed.
#[no_mangle]
pub unsafe extern "C" fn sigma_pkg_resolve_deps(target_pkg: *const u8, out_list: *mut u8, list_capacity: usize) -> SigmaI32 {
    if target_pkg.is_null() || out_list.is_null() { return -1; }
    
    RESOLVE_HEAD = 0;
    RESOLVE_TAIL = 0;
    
    if !queue_push(target_pkg) { return -1; }
    
    let mut resolved_count = 0;
    let mut current_pkg = [0u8; 32];
    
    while queue_pop(current_pkg.as_mut_ptr()) {
        if !sigma_pkg_is_installed(current_pkg.as_ptr()) {
            // Add to output list
            if resolved_count * 32 >= list_capacity { return -1; } // buffer too small
            for i in 0..32 {
                *out_list.add(resolved_count * 32 + i) = current_pkg[i];
            }
            resolved_count += 1;
            
            // Get sub-dependencies and push them
            let mut deps = [PkgDep { name: [0; 32], min_version: 0, required: false }; MAX_DEPS_PER_PKG];
            let dep_count = get_package_deps(current_pkg.as_ptr(), deps.as_mut_ptr(), MAX_DEPS_PER_PKG);
            
            for i in 0..dep_count {
                if deps[i].required {
                    if !queue_push(deps[i].name.as_ptr()) { return -1; }
                }
            }
        }
    }
    
    resolved_count as SigmaI32
}
