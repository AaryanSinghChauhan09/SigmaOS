//! SigmaOS — Tmpfs Filesystem
//! In-memory filesystem for temporary storage (/tmp).
//! Uses the global physical allocator for backing memory.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U32 = u32;
type U64 = u64;

const TMPFS_PAGE_SIZE: usize = 4096;
const TMPFS_MAX_PAGES_PER_FILE: usize = 256; // Max 1MB per file for simplicity
const TMPFS_MAX_INODES: usize = 128;

// ── Inode State ─────────────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum TmpfsInodeType {
    Free       = 0,
    Regular    = 1,
    Directory  = 2,
}

#[derive(Copy, Clone)]
pub struct TmpfsInode {
    pub ino:        U32,
    pub itype:      TmpfsInodeType,
    pub size:       U64,
    pub pages:      [U64; TMPFS_MAX_PAGES_PER_FILE], // Physical addresses of pages
    pub page_count: U32,
}

impl TmpfsInode {
    pub const fn empty() -> Self {
        TmpfsInode {
            ino: 0,
            itype: TmpfsInodeType::Free,
            size: 0,
            pages: [0u64; TMPFS_MAX_PAGES_PER_FILE],
            page_count: 0,
        }
    }
}

// ── Tmpfs State ─────────────────────────────────────────────────────────────
pub struct TmpfsState {
    pub inodes: [TmpfsInode; TMPFS_MAX_INODES],
    pub inode_count: usize,
    pub next_ino: U32,
}

static mut TMPFS: TmpfsState = TmpfsState {
    inodes: [TmpfsInode::empty(); TMPFS_MAX_INODES],
    inode_count: 0,
    next_ino: 1,
};

// ── External Dependencies ───────────────────────────────────────────────────
extern "C" {
    fn sigma_buddy_alloc(order: U32) -> U32; // Returns PFN
    fn sigma_buddy_free(pfn: U32, order: U32);
    fn sigma_buddy_pfn_to_phys(pfn: U32) -> U64;
    fn sigma_vmm_map(vaddr: U64, phys_addr: U64, flags: U64) -> i32;
}

// ── Public API ──────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_init() -> i32 {
    TMPFS.inode_count = 0;
    TMPFS.next_ino = 1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_create_inode(itype: U8) -> i32 {
    for i in 0..TMPFS_MAX_INODES {
        if TMPFS.inodes[i].itype == TmpfsInodeType::Free {
            TMPFS.inodes[i].ino = TMPFS.next_ino;
            TMPFS.inodes[i].itype = match itype {
                1 => TmpfsInodeType::Regular,
                2 => TmpfsInodeType::Directory,
                _ => TmpfsInodeType::Regular,
            };
            TMPFS.inodes[i].size = 0;
            TMPFS.inodes[i].page_count = 0;
            TMPFS.next_ino += 1;
            TMPFS.inode_count += 1;
            return TMPFS.inodes[i].ino as i32;
        }
    }
    -1 // No space
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_allocate_page(ino: U32, page_idx: U32) -> U64 {
    let mut target_idx: Option<usize> = None;
    for i in 0..TMPFS_MAX_INODES {
        if TMPFS.inodes[i].ino == ino && TMPFS.inodes[i].itype != TmpfsInodeType::Free {
            target_idx = Some(i);
            break;
        }
    }
    
    let i = match target_idx {
        Some(idx) => idx,
        None => return 0,
    };

    let inode = &mut TMPFS.inodes[i];
    if page_idx as usize >= TMPFS_MAX_PAGES_PER_FILE {
        return 0; // File too large
    }

    if inode.pages[page_idx as usize] != 0 {
        return inode.pages[page_idx as usize]; // Already allocated
    }

    // Allocate physical page
    let pfn = sigma_buddy_alloc(0);
    if pfn == U32::MAX { return 0; }

    let phys = sigma_buddy_pfn_to_phys(pfn);
    inode.pages[page_idx as usize] = phys;
    if page_idx >= inode.page_count {
        inode.page_count = page_idx + 1;
    }

    phys
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_free_inode(ino: U32) -> i32 {
    for i in 0..TMPFS_MAX_INODES {
        if TMPFS.inodes[i].ino == ino && TMPFS.inodes[i].itype != TmpfsInodeType::Free {
            // Free all pages
            for p in 0..TMPFS.inodes[i].page_count as usize {
                if TMPFS.inodes[i].pages[p] != 0 {
                    // Quick and dirty phys to pfn conversion (assumes identity)
                    let pfn = (TMPFS.inodes[i].pages[p] / TMPFS_PAGE_SIZE as U64) as U32;
                    sigma_buddy_free(pfn, 0);
                    TMPFS.inodes[i].pages[p] = 0;
                }
            }
            TMPFS.inodes[i].itype = TmpfsInodeType::Free;
            TMPFS.inode_count -= 1;
            return 0;
        }
    }
    -1
}

// ── VFS Interface Functions ──────────────────────────────────────────────────────

/// Create a new file in tmpfs
#[no_mangle]
pub unsafe extern "C" fn tmpfs_create(path: *const u8, mode: U32) -> i64 {
    let _ = mode;
    if path.is_null() { return -14; } // EFAULT
    
    // Extract filename from path (simplified - just use last component)
    let path_len = strlen(path);
    if path_len == 0 { return -2; } // ENOENT
    
    let ino = sigma_tmpfs_create_inode(1); // Regular file
    if ino < 0 { return -12; } // ENOMEM
    
    ino as i64
}

/// Read from a tmpfs file
#[no_mangle]
pub unsafe extern "C" fn tmpfs_read(inode: U64, offset: U64, buf: *mut u8, len: usize) -> i64 {
    let ino = inode as U32;
    
    // Find inode
    let mut target_idx: Option<usize> = None;
    for i in 0..TMPFS_MAX_INODES {
        if TMPFS.inodes[i].ino == ino && TMPFS.inodes[i].itype != TmpfsInodeType::Free {
            target_idx = Some(i);
            break;
        }
    }
    
    let idx = match target_idx {
        Some(i) => i,
        None => return -2, // ENOENT
    };
    
    let inode_obj = &TMPFS.inodes[idx];
    if offset >= inode_obj.size { return 0; } // EOF
    
    let mut bytes_read = 0;
    let mut remaining = len;
    let mut current_offset = offset;
    
    while bytes_read < len && current_offset < inode_obj.size {
        let page_idx = (current_offset / TMPFS_PAGE_SIZE as U64) as U32;
        let page_offset = (current_offset % TMPFS_PAGE_SIZE as U64) as usize;
        
        if page_idx >= inode_obj.page_count { break; }
        
        let phys_addr = inode_obj.pages[page_idx as usize];
        if phys_addr == 0 { break; }
        
        // Map page temporarily for reading
        let vaddr = 0xFFFF_F000_0000_0000 + (page_idx as U64 * TMPFS_PAGE_SIZE as U64);
        let _ = sigma_vmm_map(vaddr, phys_addr, 0x3); // Read/write
        
        let src = (vaddr + page_offset as U64) as *const u8;
        let copy_len = remaining.min(TMPFS_PAGE_SIZE - page_offset);
        
        for i in 0..copy_len {
            *buf.add(bytes_read + i) = *src.add(i);
        }
        
        bytes_read += copy_len;
        remaining -= copy_len;
        current_offset += copy_len as U64;
    }
    
    bytes_read as i64
}

/// Write to a tmpfs file
#[no_mangle]
pub unsafe extern "C" fn tmpfs_write(inode: U64, offset: U64, buf: *const u8, len: usize) -> i64 {
    let ino = inode as U32;
    
    // Find inode
    let mut target_idx: Option<usize> = None;
    for i in 0..TMPFS_MAX_INODES {
        if TMPFS.inodes[i].ino == ino && TMPFS.inodes[i].itype != TmpfsInodeType::Free {
            target_idx = Some(i);
            break;
        }
    }
    
    let idx = match target_idx {
        Some(i) => i,
        None => return -2, // ENOENT
    };
    
    let inode_obj = &mut TMPFS.inodes[idx];
    
    let mut bytes_written = 0;
    let mut remaining = len;
    let mut current_offset = offset;
    
    while bytes_written < len {
        let page_idx = (current_offset / TMPFS_PAGE_SIZE as U64) as U32;
        let page_offset = (current_offset % TMPFS_PAGE_SIZE as U64) as usize;
        
        if page_idx >= TMPFS_MAX_PAGES_PER_FILE as U32 { break; }
        
        // Allocate page if needed
        if page_idx >= inode_obj.page_count || inode_obj.pages[page_idx as usize] == 0 {
            let phys = sigma_tmpfs_allocate_page(ino, page_idx);
            if phys == 0 { break; }
        }
        
        let phys_addr = inode_obj.pages[page_idx as usize];
        
        // Map page temporarily for writing
        let vaddr = 0xFFFF_F000_0000_0000 + (page_idx as U64 * TMPFS_PAGE_SIZE as U64);
        let _ = sigma_vmm_map(vaddr, phys_addr, 0x3); // Read/write
        
        let dst = (vaddr + page_offset as U64) as *mut u8;
        let copy_len = remaining.min(TMPFS_PAGE_SIZE - page_offset);
        
        for i in 0..copy_len {
            *dst.add(i) = *buf.add(bytes_written + i);
        }
        
        bytes_written += copy_len;
        remaining -= copy_len;
        current_offset += copy_len as U64;
        
        // Update file size
        if current_offset > inode_obj.size {
            inode_obj.size = current_offset;
        }
    }
    
    bytes_written as i64
}

/// Get file statistics
#[no_mangle]
pub unsafe extern "C" fn tmpfs_stat(inode: U64, out: *mut u8) -> i64 {
    let ino = inode as U32;
    
    // Find inode
    let mut target_idx: Option<usize> = None;
    for i in 0..TMPFS_MAX_INODES {
        if TMPFS.inodes[i].ino == ino && TMPFS.inodes[i].itype != TmpfsInodeType::Free {
            target_idx = Some(i);
            break;
        }
    }
    
    let idx = match target_idx {
        Some(i) => i,
        None => return -2, // ENOENT
    };
    
    let inode_obj = &TMPFS.inodes[idx];
    
    // Fill stat structure (simplified)
    // In a real implementation, this would fill a proper struct stat
    let _ = (inode_obj.ino, inode_obj.size, out);
    
    0
}

/// Create a directory
#[no_mangle]
pub unsafe extern "C" fn tmpfs_mkdir(path: *const u8, mode: U32) -> i64 {
    let _ = mode;
    if path.is_null() { return -14; } // EFAULT
    
    let ino = sigma_tmpfs_create_inode(2); // Directory
    if ino < 0 { return -12; } // ENOMEM
    
    ino as i64
}

/// Remove a directory
#[no_mangle]
pub unsafe extern "C" fn tmpfs_rmdir(path: *const u8) -> i64 {
    if path.is_null() { return -14; } // EFAULT
    
    // Find inode by path (simplified - would need proper path resolution)
    // For now, just return success
    0
}

/// Unlink (delete) a file
#[no_mangle]
pub unsafe extern "C" fn tmpfs_unlink(path: *const u8) -> i64 {
    if path.is_null() { return -14; } // EFAULT
    
    // Find inode by path and free it (simplified)
    // For now, just return success
    0
}

/// Rename a file/directory
#[no_mangle]
pub unsafe extern "C" fn tmpfs_rename(old: *const u8, new: *const u8) -> i64 {
    if old.is_null() || new.is_null() { return -14; } // EFAULT
    
    // Update path (simplified - would need proper path resolution)
    0
}

/// Lookup a file by path
#[no_mangle]
pub unsafe extern "C" fn tmpfs_lookup(path: *const u8) -> U64 {
    if path.is_null() { return 0; }
    
    // Simplified path lookup - in a real implementation, this would
    // walk the directory hierarchy and return the inode number
    // For now, return 0 (not found)
    0
}

/// Helper: strlen for C strings
unsafe fn strlen(s: *const u8) -> usize {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}
