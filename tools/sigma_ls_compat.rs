//! SigmaOS LS Compatibility
//! Directory listing (ls command)
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaU64 = u64;

/// File types
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum FileType {
    Regular,
    Directory,
    SymbolicLink,
    BlockDevice,
    CharacterDevice,
    NamedPipe,
    Socket,
}

/// File entry
#[repr(C)]
pub struct FileEntry {
    pub name: [u8; 256],
    pub file_type: FileType,
    pub size: SigmaU64,
    pub permissions: SigmaU32,
    pub owner: SigmaU32,
    pub group: SigmaU32,
    pub modified_time: SigmaU64,
}

/// LS state
const MAX_FILE_ENTRIES: usize = 10000;

static mut FILE_ENTRIES: [FileEntry; MAX_FILE_ENTRIES] = [FileEntry {
    name: [0; 256],
    file_type: FileType::Regular,
    size: 0,
    permissions: 0,
    owner: 0,
    group: 0,
    modified_time: 0,
}; MAX_FILE_ENTRIES];

static mut FILE_ENTRY_COUNT: SigmaU32 = 0;
static mut LS_INITIALIZED: SigmaBool = false;

/// Initialize LS
#[no_mangle]
pub unsafe extern "C" fn ls_init() -> SigmaI32 {
    LS_INITIALIZED = true;
    FILE_ENTRY_COUNT = 0;
    
    // Add sample entries
    let mut dot = FileEntry {
        name: [0; 256],
        file_type: FileType::Directory,
        size: 4096,
        permissions: 0o755,
        owner: 0,
        group: 0,
        modified_time: 0,
    };
    
    for i in 0..255 {
        dot.name[i] = b"."[i.min(1)];
    }
    
    FILE_ENTRIES[0] = dot;
    FILE_ENTRY_COUNT = 1;
    
    let mut dotdot = FileEntry {
        name: [0; 256],
        file_type: FileType::Directory,
        size: 4096,
        permissions: 0o755,
        owner: 0,
        group: 0,
        modified_time: 0,
    };
    
    for i in 0..255 {
        dotdot.name[i] = b".."[i.min(2)];
    }
    
    FILE_ENTRIES[1] = dotdot;
    FILE_ENTRY_COUNT = 2;
    
    0 // Success
}

/// List directory
#[no_mangle]
pub unsafe extern "C" fn ls_list(path: *const u8, entries: *mut FileEntry, max_count: SigmaU32) -> SigmaU32 {
    if !LS_INITIALIZED || entries.is_null() {
        return 0;
    }
    
    let mut count = 0;
    for i in 0..FILE_ENTRY_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        *entries.add(count) = FILE_ENTRIES[i];
        count += 1;
    }
    
    count
}

/// List with long format
#[no_mangle]
pub unsafe extern "C" fn ls_list_long(path: *const u8, entries: *mut FileEntry, max_count: SigmaU32) -> SigmaU32 {
    if !LS_INITIALIZED || entries.isnull() {
        return 0;
    }
    
    ls_list(path, entries, max_count)
}

/// List all files (including hidden)
#[no_mangle]
pub unsafe extern "C" fn ls_list_all(path: *const u8, entries: *mut FileEntry, max_count: SigmaU32) -> SigmaU32 {
    if !LS_INITIALIZED || entries.isnull() {
        return 0;
    }
    
    ls_list(path, entries, max_count)
}

/// Get file count
#[no_mangle]
pub unsafe extern "C" fn ls_get_count() -> SigmaU32 {
    FILE_ENTRY_COUNT
}

/// Add file entry
#[no_mangle]
pub unsafe extern "C" fn ls_add_entry(
    name: *const u8,
    file_type: FileType,
    size: SigmaU64,
    permissions: SigmaU32,
) -> SigmaI32 {
    if !LS_INITIALIZED || FILE_ENTRY_COUNT >= MAX_FILE_ENTRIES as SigmaU32 {
        return -1;
    }
    
    let mut entry = FileEntry {
        name: [0; 256],
        file_type,
        size,
        permissions,
        owner: 0,
        group: 0,
        modified_time: 0,
    };
    
    if !name.isnull() {
        for i in 0..255 {
            let byte = *name.add(i);
            if byte == 0 { break; }
            entry.name[i] = byte;
        }
    }
    
    FILE_ENTRIES[FILE_ENTRY_COUNT as usize] = entry;
    FILE_ENTRY_COUNT += 1;
    
    0 // Success
}
