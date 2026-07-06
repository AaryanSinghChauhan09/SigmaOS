// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Mandatory Access Control (MAC)
//! SELinux-style context labeling and policy enforcement.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaI32 = i32;

pub const MAX_MAC_POLICIES: usize = 128;
pub const MAC_CONTEXT_LEN: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MacPolicy {
    pub subject_context: [u8; MAC_CONTEXT_LEN],
    pub object_context: [u8; MAC_CONTEXT_LEN],
    pub permissions: SigmaU32, // Bitmask: Read/Write/Execute/Transition
    pub active: bool,
}

pub const MAC_PERM_READ:   SigmaU32 = 0x1;
pub const MAC_PERM_WRITE:  SigmaU32 = 0x2;
pub const MAC_PERM_EXEC:   SigmaU32 = 0x4;
pub const MAC_PERM_TRANS:  SigmaU32 = 0x8;

static mut POLICIES: [MacPolicy; MAX_MAC_POLICIES] = [MacPolicy {
    subject_context: [0; MAC_CONTEXT_LEN],
    object_context: [0; MAC_CONTEXT_LEN],
    permissions: 0,
    active: false,
}; MAX_MAC_POLICIES];

unsafe fn c_str_match(s1: *const u8, s2: *const u8) -> bool {
    let mut i = 0;
    loop {
        let c1 = *s1.add(i);
        let c2 = *s2.add(i);
        if c1 != c2 { return false; }
        if c1 == 0 { return true; }
        i += 1;
        if i >= MAC_CONTEXT_LEN { return true; }
    }
}

unsafe fn copy_context(dst: &mut [u8; MAC_CONTEXT_LEN], src: *const u8) {
    let mut i = 0;
    while i < MAC_CONTEXT_LEN - 1 {
        let c = *src.add(i);
        dst[i] = c;
        if c == 0 { break; }
        i += 1;
    }
    dst[MAC_CONTEXT_LEN - 1] = 0;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_init() {
    for i in 0..MAX_MAC_POLICIES {
        POLICIES[i].active = false;
    }
}

/// Add a new MAC policy rule.
#[no_mangle]
pub unsafe extern "C" fn sigma_mac_add_rule(sub: *const u8, obj: *const u8, perms: SigmaU32) -> SigmaI32 {
    if sub.is_null() || obj.is_null() { return -1; }
    for i in 0..MAX_MAC_POLICIES {
        if !POLICIES[i].active {
            copy_context(&mut POLICIES[i].subject_context, sub);
            copy_context(&mut POLICIES[i].object_context, obj);
            POLICIES[i].permissions = perms;
            POLICIES[i].active = true;
            return 0;
        }
    }
    -1 // Policy table full
}

/// Check if subject is allowed to perform action on object.
#[no_mangle]
pub unsafe extern "C" fn sigma_mac_check_access(sub: *const u8, obj: *const u8, requested_perms: SigmaU32) -> SigmaI32 {
    if sub.is_null() || obj.is_null() { return -1; }
    
    for i in 0..MAX_MAC_POLICIES {
        if POLICIES[i].active {
            if c_str_match(POLICIES[i].subject_context.as_ptr(), sub) && 
               c_str_match(POLICIES[i].object_context.as_ptr(), obj) {
                // If policy grants ALL requested permissions
                if (POLICIES[i].permissions & requested_perms) == requested_perms {
                    return 0; // Access granted
                }
            }
        }
    }
    -1 // Default deny
}
