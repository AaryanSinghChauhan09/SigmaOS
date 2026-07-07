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
