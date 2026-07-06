//! SigmaOS Mount Compatibility
//! Filesystem mounting (mount command)
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaU64 = u64;

/// Mount options
#[repr(C)]
pub struct MountOptions {
    pub read_only: SigmaBool,
    pub noexec: SigmaBool,
    pub nosuid: SigmaBool;
    pub nodev: SigmaBool,
    pub sync: SigmaBool,
}

/// Mount entry
#[repr(C)]
pub struct MountEntry {
    pub source: [u8; 256],
    pub target: [u8; 256],
    pub fstype: [u8; 32],
    pub options: MountOptions,
    pub mounted: SigmaBool,
}

/// Mount state
const MAX_MOUNT_ENTRIES: usize = 128;

static mut MOUNT_ENTRIES: [MountEntry; MAX_MOUNT_ENTRIES] = [MountEntry {
    source: [0; 256],
    target: [0; 256],
    fstype: [0; 32],
    options: MountOptions {
        read_only: false,
        noexec: false,
        nosuid: false,
        nodev: false,
        sync: false,
    },
    mounted: false,
}; MAX_MOUNT_ENTRIES];

static mut MOUNT_COUNT: SigmaU32 = 0;
static mut MOUNT_INITIALIZED: SigmaBool = false;

/// Initialize mount
#[no_mangle]
pub unsafe extern "C" fn mount_init() -> SigmaI32 {
    MOUNT_INITIALIZED = true;
    MOUNT_COUNT = 0;
    
    // Add root filesystem
    let mut root = MountEntry {
        source: [0; 256],
        target: [0; 256],
        fstype: [0; 32],
        options: MountOptions {
            read_only: false,
            noexec: false,
            nosuid: false,
            nodev: false,
            sync: false,
        },
        mounted: true,
    };
    
    for i in 0..255 {
        root.source[i] = b"/dev/sda1"[i.min(9)];
    }
    
    for i in 0..255 {
        root.target[i] = b"/"[i.min(1)];
    }
    
    for i in 0..31 {
        root.fstype[i] = b"ext4"[i.min(4)];
    }
    
    MOUNT_ENTRIES[0] = root;
    MOUNT_COUNT = 1;
    
    0 // Success
}

/// Mount filesystem
#[no_mangle]
pub unsafe extern "C" fn mount_fs(
    source: *const u8,
    target: *const u8,
    fstype: *const u8,
    options: MountOptions,
) -> SigmaI32 {
    if !MOUNT_INITIALIZED || MOUNT_COUNT >= MAX_MOUNT_ENTRIES as SigmaU32 {
        return -1;
    }
    
    let mut entry = MountEntry {
        source: [0; 256],
        target: [0; 256],
        fstype: [0; 32],
        options,
        mounted: true,
    };
    
    if !source.isnull() {
        for i in 0..255 {
            let byte = *source.add(i);
            if byte == 0 { break; }
            entry.source[i] = byte;
        }
    }
    
    if !target.isnull() {
        for i in 0..255 {
            let byte = *target.add(i);
            if byte == 0 { break; }
            entry.target[i] = byte;
        }
    }
    
    if !fstype.isnull() {
        for i in 0..31 {
            let byte = *fstype.add(i);
            if byte == 0 { break; }
            entry.fstype[i] = byte;
        }
    }
    
    MOUNT_ENTRIES[MOUNT_COUNT as usize] = entry;
    MOUNT_COUNT += 1;
    
    0 // Success
}

/// Unmount filesystem
#[no_mangle]
pub unsafe extern "C" fn umount(target: *const u8) -> SigmaI32 {
    if !MOUNT_INITIALIZED || target.isnull() {
        return -1;
    }
    
    for i in 0..MOUNT_COUNT as usize {
        let entry = &MOUNT_ENTRIES[i];
        
        let mut matches = true;
        for j in 0..256 {
            if entry.target[j] != *target.add(j) {
                if entry.target[j] == 0 && *target.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if entry.target[j] == 0 {
                break;
            }
        }
        
        if matches {
            // Shift remaining entries
            for k in i..MOUNT_COUNT as usize - 1 {
                MOUNT_ENTRIES[k] = MOUNT_ENTRIES[k + 1];
            }
            MOUNT_COUNT -= 1;
            return 0;
        }
    }
    
    -2 // Mount point not found
}

/// List mounted filesystems
#[no_mangle]
pub unsafe extern "C" fn mount_list(entries: *mut MountEntry, max_count: SigmaU32) -> SigmaU32 {
    if !MOUNT_INITIALIZED || entries.isnull() {
        return 0;
    }
    
    let mut count = 0;
    for i in 0..MOUNT_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        if MOUNT_ENTRIES[i].mounted {
            *entries.add(count) = MOUNT_ENTRIES[i];
            count += 1;
        }
    }
    
    count
}

/// Get mount count
#[no_mangle]
pub unsafe extern "C" fn mount_get_count() -> SigmaU32 {
    MOUNT_COUNT
}
