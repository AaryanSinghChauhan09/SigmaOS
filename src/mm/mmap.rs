//! SigmaOS — mmap/munmap/mprotect implementation
//! SPDX-License-Identifier: MIT OR GPL-2.0
//! Inspired by Linux mm/mmap.c
//!
//! Maintains a sorted, non-overlapping list of VMAs for each address space.
//! Supports anonymous and private mappings, MAP_FIXED, VMA merging / splitting.

#![allow(dead_code)]

use std::string::String;
use std::vec::Vec;

// ── Error type ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MmapError {
    /// EINVAL — invalid arguments
    InvalidArgs,
    /// ENOMEM — no virtual address space available
    NoMemory,
    /// EPERM — permission denied
    Permission,
    /// EBADF — invalid file descriptor for file-backed mapping
    BadFd,
    /// EEXIST — MAP_FIXED conflicts with existing mapping
    Exists,
    /// Address not mapped
    NotMapped,
}

impl core::fmt::Display for MmapError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidArgs => write!(f, "mmap: invalid arguments (EINVAL)"),
            Self::NoMemory => write!(f, "mmap: out of virtual memory (ENOMEM)"),
            Self::Permission => write!(f, "mmap: permission denied (EPERM)"),
            Self::BadFd => write!(f, "mmap: bad file descriptor (EBADF)"),
            Self::Exists => write!(f, "mmap: mapping exists (EEXIST)"),
            Self::NotMapped => write!(f, "mmap: address not mapped"),
        }
    }
}

// ── MmapProt ──────────────────────────────────────────────────────────────────

/// Memory protection flags (PROT_*)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MmapProt(pub u32);

impl MmapProt {
    pub const NONE: u32 = 0;
    pub const READ: u32 = 1;
    pub const WRITE: u32 = 2;
    pub const EXEC: u32 = 4;

    pub fn none() -> Self {
        Self(Self::NONE)
    }
    pub fn readable(&self) -> bool {
        self.0 & Self::READ != 0
    }
    pub fn writable(&self) -> bool {
        self.0 & Self::WRITE != 0
    }
    pub fn executable(&self) -> bool {
        self.0 & Self::EXEC != 0
    }
}

// ── MmapFlags ─────────────────────────────────────────────────────────────────

/// Mapping flags (MAP_*)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MmapFlags(pub u32);

impl MmapFlags {
    pub const SHARED: u32 = 0x01;
    pub const PRIVATE: u32 = 0x02;
    pub const FIXED: u32 = 0x10;
    pub const ANONYMOUS: u32 = 0x20;
    pub const POPULATE: u32 = 0x8000;
    pub const LOCKED: u32 = 0x2000;
    pub const GROWSDOWN: u32 = 0x0100;

    pub fn shared(&self) -> bool {
        self.0 & Self::SHARED != 0
    }
    pub fn private(&self) -> bool {
        self.0 & Self::PRIVATE != 0
    }
    pub fn fixed(&self) -> bool {
        self.0 & Self::FIXED != 0
    }
    pub fn anonymous(&self) -> bool {
        self.0 & Self::ANONYMOUS != 0
    }
    pub fn populate(&self) -> bool {
        self.0 & Self::POPULATE != 0
    }
    pub fn locked(&self) -> bool {
        self.0 & Self::LOCKED != 0
    }
    pub fn growsdown(&self) -> bool {
        self.0 & Self::GROWSDOWN != 0
    }
}

// ── Backing storage ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MmapBacking {
    Anonymous,
    File { path: String, offset: u64, fd: i32 },
}

// ── MemoryMap (VMA) ───────────────────────────────────────────────────────────

/// A virtual memory area descriptor
#[derive(Debug, Clone)]
pub struct MemoryMap {
    /// Start address (page-aligned, inclusive)
    pub addr: u64,
    /// Length in bytes (page-aligned)
    pub len: usize,
    /// Protection flags
    pub prot: MmapProt,
    /// Mapping flags
    pub flags: MmapFlags,
    /// Backing storage
    pub backing: MmapBacking,
}

impl MemoryMap {
    pub fn end(&self) -> u64 {
        self.addr + self.len as u64
    }

    /// True if this VMA can be merged with `other` (adjacent, same prot+flags+backing)
    pub fn can_merge(&self, other: &MemoryMap) -> bool {
        self.end() == other.addr
            && self.prot == other.prot
            && self.flags.0 == other.flags.0
            && self.backing == other.backing
    }

    /// True if this VMA overlaps the range [addr, addr+len)
    pub fn overlaps(&self, addr: u64, len: usize) -> bool {
        self.addr < addr + len as u64 && self.end() > addr
    }

    /// True if this VMA fully contains [addr, addr+len)
    pub fn contains_range(&self, addr: u64, len: usize) -> bool {
        self.addr <= addr && self.end() >= addr + len as u64
    }
}

// ── MmapAddressSpace ──────────────────────────────────────────────────────────

const PAGE_SIZE: u64 = 4096;
/// Default mmap base (start of user mmap region): 0x7f00_0000_0000 on x86-64
const MMAP_BASE: u64 = 0x0000_7F00_0000_0000;
/// Top of user address space
const USER_TOP: u64 = 0x0000_7FFF_FFFF_F000;

/// Per-process virtual address space
#[derive(Debug, Default)]
pub struct MmapAddressSpace {
    /// VMAs sorted by start address
    vmas: Vec<MemoryMap>,
    /// Next address hint for `mmap_find_free`
    mmap_hint: u64,
}

impl MmapAddressSpace {
    pub fn new() -> Self {
        Self {
            vmas: Vec::new(),
            mmap_hint: MMAP_BASE,
        }
    }

    /// Find an unmapped region of `len` bytes starting at or after `hint`.
    fn find_free_region(&self, len: usize) -> Option<u64> {
        let mut candidate = self.mmap_hint;
        // Align to page
        candidate = (candidate + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);

        loop {
            if candidate + len as u64 > USER_TOP {
                return None;
            }
            // Check if any existing VMA overlaps [candidate, candidate+len)
            let collision = self.vmas.iter().any(|v| v.overlaps(candidate, len));
            if !collision {
                return Some(candidate);
            }
            // Skip past conflicting VMA
            let conflicting = self
                .vmas
                .iter()
                .filter(|v| v.overlaps(candidate, len))
                .max_by_key(|v| v.end())
                .unwrap();
            candidate = (conflicting.end() + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        }
    }

    /// Insert a VMA and merge with adjacent VMAs where possible.
    fn insert_vma(&mut self, vma: MemoryMap) {
        self.vmas.push(vma);
        self.vmas.sort_by_key(|v| v.addr);
        self.merge_vmas();
    }

    /// Merge adjacent VMAs with identical properties.
    fn merge_vmas(&mut self) {
        let mut i = 0;
        while i + 1 < self.vmas.len() {
            let can = {
                let (a, b) = (&self.vmas[i], &self.vmas[i + 1]);
                a.can_merge(b)
            };
            if can {
                let b_len = self.vmas[i + 1].len;
                self.vmas[i].len += b_len;
                self.vmas.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    /// Remove a VMA that exactly matches `addr`
    fn remove_exact(&mut self, addr: u64) {
        self.vmas.retain(|v| v.addr != addr);
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Create a new memory mapping.
///
/// Returns the base virtual address of the new mapping.
pub fn do_mmap(
    addr_space: &mut MmapAddressSpace,
    hint_addr: u64,
    len: usize,
    prot: MmapProt,
    flags: MmapFlags,
    fd: i32,
    file_offset: u64,
) -> Result<u64, MmapError> {
    if len == 0 {
        return Err(MmapError::InvalidArgs);
    }
    // Round up to page size
    let len = ((len as u64 + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)) as usize;

    // Must be SHARED or PRIVATE
    if !flags.shared() && !flags.private() {
        return Err(MmapError::InvalidArgs);
    }
    // File-backed mappings need a valid fd
    if !flags.anonymous() && fd < 0 {
        return Err(MmapError::BadFd);
    }

    let addr = if flags.fixed() {
        // MAP_FIXED: must use exact hint_addr, fail if not page-aligned
        if hint_addr & (PAGE_SIZE - 1) != 0 {
            return Err(MmapError::InvalidArgs);
        }
        // Remove any existing mappings in [hint_addr, hint_addr+len)
        do_munmap(addr_space, hint_addr, len)?;
        hint_addr
    } else {
        let base = if hint_addr != 0 {
            hint_addr
        } else {
            addr_space.mmap_hint
        };
        // Temporarily adjust hint
        let old_hint = addr_space.mmap_hint;
        addr_space.mmap_hint = (base + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        let found = addr_space.find_free_region(len);
        addr_space.mmap_hint = old_hint;
        found.ok_or(MmapError::NoMemory)?
    };

    let backing = if flags.anonymous() {
        MmapBacking::Anonymous
    } else {
        // Simulate a file path lookup from fd
        MmapBacking::File {
            path: format!("/proc/self/fd/{}", fd),
            offset: file_offset,
            fd,
        }
    };

    let vma = MemoryMap {
        addr,
        len,
        prot,
        flags,
        backing,
    };
    addr_space.insert_vma(vma);

    // Advance hint for next allocation
    addr_space.mmap_hint = addr + len as u64;

    Ok(addr)
}

/// Unmap a region. Handles partial unmap with VMA splitting.
pub fn do_munmap(
    addr_space: &mut MmapAddressSpace,
    addr: u64,
    len: usize,
) -> Result<(), MmapError> {
    if addr & (PAGE_SIZE - 1) != 0 || len == 0 {
        return Err(MmapError::InvalidArgs);
    }
    let len = ((len as u64 + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)) as usize;
    let end = addr + len as u64;

    let mut new_vmas: Vec<MemoryMap> = Vec::new();

    for vma in addr_space.vmas.drain(..) {
        if !vma.overlaps(addr, len) {
            new_vmas.push(vma);
            continue;
        }
        // Split: keep prefix [vma.addr, addr)
        if vma.addr < addr {
            new_vmas.push(MemoryMap {
                addr: vma.addr,
                len: (addr - vma.addr) as usize,
                prot: vma.prot,
                flags: vma.flags,
                backing: vma.backing.clone(),
            });
        }
        // Split: keep suffix [end, vma.end())
        if vma.end() > end {
            new_vmas.push(MemoryMap {
                addr: end,
                len: (vma.end() - end) as usize,
                prot: vma.prot,
                flags: vma.flags,
                backing: vma.backing.clone(),
            });
        }
        // Middle portion [addr, end) is unmapped
    }

    addr_space.vmas = new_vmas;
    Ok(())
}

/// Change memory protection on a mapped region.
pub fn do_mprotect(
    addr_space: &mut MmapAddressSpace,
    addr: u64,
    len: usize,
    new_prot: MmapProt,
) -> Result<(), MmapError> {
    if addr & (PAGE_SIZE - 1) != 0 || len == 0 {
        return Err(MmapError::InvalidArgs);
    }
    let len = ((len as u64 + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)) as usize;
    let end = addr + len as u64;

    // Collect VMAs that overlap with [addr, end)
    let overlapping: Vec<MemoryMap> = addr_space
        .vmas
        .iter()
        .filter(|v| v.overlaps(addr, len))
        .cloned()
        .collect();

    if overlapping.is_empty() {
        return Err(MmapError::NotMapped);
    }

    // Remove overlapping VMAs, then re-add split pieces
    for vma in &overlapping {
        addr_space.vmas.retain(|v| v.addr != vma.addr);
    }

    for vma in overlapping {
        // prefix before [addr, end)
        if vma.addr < addr {
            addr_space.vmas.push(MemoryMap {
                addr: vma.addr,
                len: (addr - vma.addr) as usize,
                prot: vma.prot,
                flags: vma.flags,
                backing: vma.backing.clone(),
            });
        }
        // The protected portion
        let prot_start = vma.addr.max(addr);
        let prot_end = vma.end().min(end);
        if prot_start < prot_end {
            addr_space.vmas.push(MemoryMap {
                addr: prot_start,
                len: (prot_end - prot_start) as usize,
                prot: new_prot,
                flags: vma.flags,
                backing: vma.backing.clone(),
            });
        }
        // suffix after [addr, end)
        if vma.end() > end {
            addr_space.vmas.push(MemoryMap {
                addr: end,
                len: (vma.end() - end) as usize,
                prot: vma.prot,
                flags: vma.flags,
                backing: vma.backing.clone(),
            });
        }
    }

    addr_space.vmas.sort_by_key(|v| v.addr);
    addr_space.merge_vmas();
    Ok(())
}

/// Find the VMA covering `addr`, if any.
pub fn find_vma(addr_space: &MmapAddressSpace, addr: u64) -> Option<&MemoryMap> {
    addr_space
        .vmas
        .iter()
        .find(|v| v.addr <= addr && addr < v.end())
}

/// Return list of all VMAs (for debugging)
pub fn list_vmas(addr_space: &MmapAddressSpace) -> &[MemoryMap] {
    &addr_space.vmas
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    fn anon_flags() -> MmapFlags {
        MmapFlags(MmapFlags::PRIVATE | MmapFlags::ANONYMOUS)
    }
    fn rw_prot() -> MmapProt {
        MmapProt(MmapProt::READ | MmapProt::WRITE)
    }

    #[test]
    fn test_anon_mmap_basic() {
        let mut as_ = MmapAddressSpace::new();
        let addr = do_mmap(&mut as_, 0, 4096, rw_prot(), anon_flags(), -1, 0).unwrap();
        assert!(addr >= MMAP_BASE);
        assert!(find_vma(&as_, addr).is_some());
    }

    #[test]
    fn test_vma_merge() {
        let mut as_ = MmapAddressSpace::new();
        // Two adjacent anonymous mappings with same prot → should merge
        let a1 = do_mmap(&mut as_, 0, 4096, rw_prot(), anon_flags(), -1, 0).unwrap();
        let _a2 = do_mmap(&mut as_, a1 + 4096, 4096, rw_prot(), anon_flags(), -1, 0).unwrap();
        // After merge there should be 1 VMA covering 8192 bytes
        let vmas = list_vmas(&as_);
        assert_eq!(vmas.len(), 1);
        assert_eq!(vmas[0].len, 8192);
    }

    #[test]
    fn test_munmap_full() {
        let mut as_ = MmapAddressSpace::new();
        let addr = do_mmap(&mut as_, 0, 4096, rw_prot(), anon_flags(), -1, 0).unwrap();
        do_munmap(&mut as_, addr, 4096).unwrap();
        assert!(find_vma(&as_, addr).is_none());
    }

    #[test]
    fn test_munmap_partial_split() {
        let mut as_ = MmapAddressSpace::new();
        let addr = do_mmap(&mut as_, 0, 3 * 4096, rw_prot(), anon_flags(), -1, 0).unwrap();
        // Unmap middle page
        do_munmap(&mut as_, addr + 4096, 4096).unwrap();
        let vmas = list_vmas(&as_);
        assert_eq!(vmas.len(), 2);
        assert_eq!(vmas[0].addr, addr);
        assert_eq!(vmas[0].len, 4096);
        assert_eq!(vmas[1].addr, addr + 2 * 4096);
        assert_eq!(vmas[1].len, 4096);
    }

    #[test]
    fn test_mprotect() {
        let mut as_ = MmapAddressSpace::new();
        let addr = do_mmap(&mut as_, 0, 2 * 4096, rw_prot(), anon_flags(), -1, 0).unwrap();
        // Make first page read-only
        do_mprotect(&mut as_, addr, 4096, MmapProt(MmapProt::READ)).unwrap();
        let vmas = list_vmas(&as_);
        // Should be split into 2: RO + RW
        assert_eq!(vmas.len(), 2);
        assert!(!vmas[0].prot.writable());
        assert!(vmas[1].prot.writable());
    }

    #[test]
    fn test_map_fixed() {
        let mut as_ = MmapAddressSpace::new();
        let fixed_addr = 0x0000_7F80_0000_0000u64;
        let fixed_flags = MmapFlags(MmapFlags::PRIVATE | MmapFlags::ANONYMOUS | MmapFlags::FIXED);
        let addr = do_mmap(&mut as_, fixed_addr, 4096, rw_prot(), fixed_flags, -1, 0).unwrap();
        assert_eq!(addr, fixed_addr);
    }

    #[test]
    fn test_invalid_args() {
        let mut as_ = MmapAddressSpace::new();
        // len=0 → error
        assert_eq!(
            do_mmap(&mut as_, 0, 0, rw_prot(), anon_flags(), -1, 0),
            Err(MmapError::InvalidArgs)
        );
        // No SHARED/PRIVATE flag
        assert_eq!(
            do_mmap(
                &mut as_,
                0,
                4096,
                rw_prot(),
                MmapFlags(MmapFlags::ANONYMOUS),
                -1,
                0
            ),
            Err(MmapError::InvalidArgs)
        );
    }
}
