// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign Registry Manager (Rust, no_std)
//! Replaces: kernel/core/sigma_registry_manager.cpp
//! =========================================================================

#![no_std]

use core::cell::UnsafeCell;

pub const MAX_REGISTRY_KEYS: usize = 1024;
pub const KEY_NAME_MAX: usize = 64;
pub const KEY_VALUE_MAX: usize = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaRegistryEntry {
    pub key: [u8; KEY_NAME_MAX],
    pub value: [u8; KEY_VALUE_MAX],
    pub in_use: bool,
}

impl SigmaRegistryEntry {
    pub const fn empty() -> Self {
        Self {
            key: [0; KEY_NAME_MAX],
            value: [0; KEY_VALUE_MAX],
            in_use: false,
        }
    }
}

pub struct RegistryStore {
    store: [SigmaRegistryEntry; MAX_REGISTRY_KEYS],
}

impl RegistryStore {
    pub const fn new() -> Self {
        Self {
            store: [SigmaRegistryEntry::empty(); MAX_REGISTRY_KEYS],
        }
    }
}

struct SafeRegistryStore {
    inner: UnsafeCell<RegistryStore>,
}

unsafe impl Sync for SafeRegistryStore {}

static REGISTRY_STORE: SafeRegistryStore = SafeRegistryStore {
    inner: UnsafeCell::new(RegistryStore::new()),
};

unsafe fn streq(s1: *const u8, s2: *const u8) -> bool {
    let mut idx = 0;
    loop {
        let char1 = *s1.add(idx);
        let char2 = *s2.add(idx);
        if char1 != char2 {
            return false;
        }
        if char1 == 0 {
            return true;
        }
        idx += 1;
    }
}

unsafe fn copy_str(dest: &mut [u8], src: *const u8, max_len: usize) {
    let mut i = 0;
    while i < max_len - 1 && *src.add(i) != 0 {
        dest[i] = *src.add(i);
        i += 1;
    }
    while i < max_len {
        dest[i] = 0;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_registry_init() {
    let reg = &mut *REGISTRY_STORE.inner.get();
    for i in 0..MAX_REGISTRY_KEYS {
        reg.store[i] = SigmaRegistryEntry::empty();
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_registry_set(key: *const u8, value: *const u8) -> bool {
    if key.is_null() || value.is_null() {
        return false;
    }

    let reg = &mut *REGISTRY_STORE.inner.get();

    // Try overwrite
    for i in 0..MAX_REGISTRY_KEYS {
        let entry = &mut reg.store[i];
        if entry.in_use && streq(entry.key.as_ptr(), key) {
            copy_str(&mut entry.value, value, KEY_VALUE_MAX);
            return true;
        }
    }

    // Try new slot
    for i in 0..MAX_REGISTRY_KEYS {
        let entry = &mut reg.store[i];
        if !entry.in_use {
            copy_str(&mut entry.key, key, KEY_NAME_MAX);
            copy_str(&mut entry.value, value, KEY_VALUE_MAX);
            entry.in_use = true;
            return true;
        }
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn sigma_registry_get(key: *const u8, out_value: *mut u8, max_len: u32) -> bool {
    if key.is_null() || out_value.is_null() || max_len == 0 {
        return false;
    }

    let reg = &*REGISTRY_STORE.inner.get();

    for i in 0..MAX_REGISTRY_KEYS {
        let entry = &reg.store[i];
        if entry.in_use && streq(entry.key.as_ptr(), key) {
            let out_slice = core::slice::from_raw_parts_mut(out_value, max_len as usize);
            copy_str(out_slice, entry.value.as_ptr(), max_len as usize);
            return true;
        }
    }

    false
}
