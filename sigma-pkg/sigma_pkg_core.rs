// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Unified Package Manager (sigpkg) - Core
//! Install, remove, search logic.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaI32 = i32;

pub const MAX_INSTALLED_PKGS: usize = 1024;
pub const MAX_PKG_NAME_LEN: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PkgInfo {
    pub name: [u8; MAX_PKG_NAME_LEN],
    pub version: SigmaU32, // encoded as major.minor.patch
    pub size_kb: SigmaU32,
    pub installed: bool,
}

static mut INSTALLED_PKGS: [PkgInfo; MAX_INSTALLED_PKGS] = [PkgInfo {
    name: [0; MAX_PKG_NAME_LEN],
    version: 0,
    size_kb: 0,
    installed: false,
}; MAX_INSTALLED_PKGS];

static mut INSTALLED_COUNT: SigmaU32 = 0;

unsafe fn c_str_match(s1: *const u8, s2: *const u8) -> bool {
    let mut i = 0;
    loop {
        let c1 = *s1.add(i);
        let c2 = *s2.add(i);
        if c1 != c2 { return false; }
        if c1 == 0 { return true; }
        i += 1;
        if i >= MAX_PKG_NAME_LEN { return true; }
    }
}

unsafe fn copy_name(dst: &mut [u8; MAX_PKG_NAME_LEN], src: *const u8) {
    let mut i = 0;
    while i < MAX_PKG_NAME_LEN - 1 {
        let c = *src.add(i);
        dst[i] = c;
        if c == 0 { break; }
        i += 1;
    }
    dst[MAX_PKG_NAME_LEN - 1] = 0;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pkg_init() {
    INSTALLED_COUNT = 0;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pkg_is_installed(name: *const u8) -> bool {
    for i in 0..MAX_INSTALLED_PKGS {
        if INSTALLED_PKGS[i].installed && c_str_match(INSTALLED_PKGS[i].name.as_ptr(), name) {
            return true;
        }
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pkg_install(name: *const u8, version: SigmaU32, size_kb: SigmaU32) -> SigmaI32 {
    if name.is_null() { return -1; }
    if sigma_pkg_is_installed(name) { return 1; } // Already installed
    
    for i in 0..MAX_INSTALLED_PKGS {
        if !INSTALLED_PKGS[i].installed {
            copy_name(&mut INSTALLED_PKGS[i].name, name);
            INSTALLED_PKGS[i].version = version;
            INSTALLED_PKGS[i].size_kb = size_kb;
            INSTALLED_PKGS[i].installed = true;
            INSTALLED_COUNT += 1;
            
            // Here: invoke fs layer to unpack .spkg to rootfs
            
            return 0;
        }
    }
    -1 // DB full
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pkg_remove(name: *const u8) -> SigmaI32 {
    if name.is_null() { return -1; }
    for i in 0..MAX_INSTALLED_PKGS {
        if INSTALLED_PKGS[i].installed && c_str_match(INSTALLED_PKGS[i].name.as_ptr(), name) {
            INSTALLED_PKGS[i].installed = false;
            INSTALLED_COUNT -= 1;
            
            // Here: invoke fs layer to remove tracked files
            
            return 0;
        }
    }
    -1 // Not found
}
