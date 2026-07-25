//! SigmaOS Uname Compatibility
//! System information (uname command)
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaU64 = u64;

/// System information
#[repr(C)]
pub struct UnameInfo {
    pub sysname: [u8; 64],
    pub nodename: [u8; 64],
    pub release: [u8; 64],
    pub version: [u8; 64],
    pub machine: [u8; 64],
}

/// Uname state
static mut UNAME_INFO: UnameInfo = UnameInfo {
    sysname: [0; 64],
    nodename: [0; 64],
    release: [0; 64],
    version: [0; 64],
    machine: [0; 64],
};

static mut UNAME_INITIALIZED: SigmaBool = false;

/// Initialize uname
#[no_mangle]
pub unsafe extern "C" fn uname_init() -> SigmaI32 {
    UNAME_INITIALIZED = true;
    
    // Initialize with SigmaOS information
    for i in 0..63 {
        UNAME_INFO.sysname[i] = b"SigmaOS"[i.min(7)];
    }
    
    for i in 0..63 {
        UNAME_INFO.nodename[i] = b"sigmaos"[i.min(7)];
    }
    
    for i in 0..63 {
        UNAME_INFO.release[i] = b"1.0.0"[i.min(5)];
    }
    
    for i in 0..63 {
        UNAME_INFO.version[i] = b"1.0.0-SigmaOS"[i.min(13)];
    }
    
    for i in 0..63 {
        UNAME_INFO.machine[i] = b"x86_64"[i.min(6)];
    }
    
    0 // Success
}

/// Get system name
#[no_mangle]
pub unsafe extern "C" fn uname_sysname(sysname: *mut u8, max_len: SigmaU32) -> SigmaI32 {
    if !UNAME_INITIALIZED || sysname.isnull() {
        return -1;
    }
    
    for i in 0..max_len as usize {
        if i < 64 {
            *sysname.add(i) = UNAME_INFO.sysname[i];
        } else {
            break;
        }
    }
    
    0 // Success
}

/// Get node name
#[no_mangle]
pub unsafe extern "C" fn uname_nodename(nodename: *mut u8, max_len: SigmaU32) -> SigmaI32 {
    if !UNAME_INITIALIZED || nodename.isnull() {
        return -1;
    }
    
    for i in 0..max_len as usize {
        if i < 64 {
            *nodename.add(i) = UNAME_INFO.nodename[i];
        } else {
            break;
        }
    }
    
    0 // Success
}

/// Get release
#[no_mangle]
pub unsafe extern "C" fn uname_release(release: *mut u8, max_len: SigmaU32) -> SigmaI32 {
    if !UNAME_INITIALIZED || release.isnull() {
        return -1;
    }
    
    for i in 0..max_len as usize {
        if i < 64 {
            *release.add(i) = UNAME_INFO.release[i];
        } else {
            break;
        }
    }
    
    0 // Success
}

/// Get version
#[no_mangle]
pub unsafe extern "C" fn uname_version(version: *mut u8, max_len: SigmaU32) -> SigmaI32 {
    if !UNAME_INITIALIZED || version.isnull() {
        return -1;
    }
    
    for i in 0..max_len as usize {
        if i < 64 {
            *version.add(i) = UNAME_INFO.version[i];
        } else {
            break;
        }
    }
    
    0 // Success
}

/// Get machine
#[no_mangle]
pub unsafe extern "C" fn uname_machine(machine: *mut u8, max_len: SigmaU32) -> SigmaI32 {
    if !UNAME_INITIALIZED || machine.isnull() {
        return -1;
    }
    
    for i in 0..max_len as usize {
        if i < 64 {
            *machine.add(i) = UNAME_INFO.machine[i];
        } else {
            break;
        }
    }
    
    0 // Success
}

/// Get all information
#[no_mangle]
pub unsafe extern "C" fn uname_all(info: *mut UnameInfo) -> SigmaI32 {
    if !UNAME_INITIALIZED || info.isnull() {
        return -1;
    }
    
    *info = UNAME_INFO;
    0 // Success
}

/// Set system name
#[no_mangle]
pub unsafe extern "C" fn uname_set_sysname(sysname: *const u8) -> SigmaI32 {
    if !UNAME_INITIALIZED || sysname.isnull() {
        return -1;
    }
    
    for i in 0..63 {
        let byte = *sysname.add(i);
        if byte == 0 { break; }
        UNAME_INFO.sysname[i] = byte;
    }
    
    0 // Success
}

/// Set node name
#[no_mangle]
pub unsafe extern "C" fn uname_set_nodename(nodename: *const u8) -> SigmaI32 {
    if !UNAME_INITIALIZED || nodename.isnull() {
        return -1;
    }
    
    for i in 0..63 {
        let byte = *nodename.add(i);
        if byte == 0 { break; }
        UNAME_INFO.nodename[i] = byte;
    }
    
    0 // Success
}
