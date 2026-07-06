// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Filesystem Snapshot Orchestrator (ZFS/Btrfs-inspired)
//! Copy-on-Write (CoW) snapshot tracking.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;

pub const MAX_SNAPSHOTS: usize = 16;
pub const MAX_SNAP_NAME: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FsSnapshot {
    pub name: [u8; MAX_SNAP_NAME],
    pub root_block: SigmaU32,
    pub timestamp_ns: u64,
    pub active: bool,
}

static mut SNAPSHOTS: [FsSnapshot; MAX_SNAPSHOTS] = [FsSnapshot {
    name: [0; MAX_SNAP_NAME], root_block: 0, timestamp_ns: 0, active: false
}; MAX_SNAPSHOTS];

#[no_mangle]
pub unsafe extern "C" fn sigma_snap_init() {
    for i in 0..MAX_SNAPSHOTS {
        SNAPSHOTS[i].active = false;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_snap_create(
    name: *const u8, root_block: SigmaU32, timestamp_ns: u64
) -> i32 {
    if name.is_null() { return -1; }
    
    for i in 0..MAX_SNAPSHOTS {
        if !SNAPSHOTS[i].active {
            let mut j = 0;
            while j < MAX_SNAP_NAME - 1 && *name.add(j) != 0 {
                SNAPSHOTS[i].name[j] = *name.add(j);
                j += 1;
            }
            SNAPSHOTS[i].name[j] = 0;
            SNAPSHOTS[i].root_block = root_block;
            SNAPSHOTS[i].timestamp_ns = timestamp_ns;
            SNAPSHOTS[i].active = true;
            return i as i32;
        }
    }
    -1 // Table full
}

#[no_mangle]
pub unsafe extern "C" fn sigma_snap_get_root(name: *const u8, out_root: *mut SigmaU32) -> i32 {
    if name.is_null() || out_root.is_null() { return -1; }
    
    for i in 0..MAX_SNAPSHOTS {
        if SNAPSHOTS[i].active {
            let mut matches = true;
            for j in 0..MAX_SNAP_NAME {
                if SNAPSHOTS[i].name[j] != *name.add(j) {
                    matches = false;
                    break;
                }
                if SNAPSHOTS[i].name[j] == 0 { break; }
            }
            if matches {
                *out_root = SNAPSHOTS[i].root_block;
                return 0;
            }
        }
    }
    -1 // Not found
}
