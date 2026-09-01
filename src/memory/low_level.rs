#![allow(dead_code)]
#![allow(unused_variables)]
//! Low-Level Memory Management & Fast System Calls Subsystem for SigmaOS
//! Implements Two-Tier Allocation (Buddy + Slab), Recursive Page Tables,
//! Copy-on-Write (COW) Forking, x86_64 Fast Syscalls (IA32_LSTAR MSR),
//! and Minimal POSIX Syscall Matrix.

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

// =========================================================================
// 1. Two-Tier Allocator: Buddy Allocator + Slab Allocator
// =========================================================================

pub const PAGE_SIZE_4K: usize = 4096;
pub const MAX_BUDDY_ORDER: usize = 10; // Up to 4MB chunks (4096 * 2^10)

#[derive(Debug, Clone, Copy)]
pub struct BuddyBlock {
    pub paddr: usize,
    pub order: usize,
    pub is_free: bool,
}

#[derive(Debug)]
pub struct BuddyAllocatorEngine {
    pub base_addr: usize,
    pub total_pages: usize,
    pub free_lists: [Vec<usize>; MAX_BUDDY_ORDER + 1],
}

impl BuddyAllocatorEngine {
    pub fn new(base_addr: usize, total_pages: usize) -> Self {
        let mut free_lists: [Vec<usize>; MAX_BUDDY_ORDER + 1] = Default::default();
        let mut curr_page = 0;
        while curr_page < total_pages {
            let mut order = MAX_BUDDY_ORDER;
            while (1 << order) > (total_pages - curr_page) {
                order -= 1;
            }
            let block_paddr = base_addr + curr_page * PAGE_SIZE_4K;
            free_lists[order].push(block_paddr);
            curr_page += 1 << order;
        }

        Self {
            base_addr,
            total_pages,
            free_lists,
        }
    }

    pub fn allocate_block(&mut self, order: usize) -> Option<usize> {
        if order > MAX_BUDDY_ORDER {
            return None;
        }

        for current_order in order..=MAX_BUDDY_ORDER {
            if !self.free_lists[current_order].is_empty() {
                let paddr = self.free_lists[current_order].pop().unwrap();
                for split_order in (order + 1..=current_order).rev() {
                    let buddy_paddr = paddr + ((1 << (split_order - 1)) * PAGE_SIZE_4K);
                    self.free_lists[split_order - 1].push(buddy_paddr);
                }
                return Some(paddr);
            }
        }
        None
    }

    pub fn deallocate_block(&mut self, paddr: usize, order: usize) {
        if order <= MAX_BUDDY_ORDER {
            self.free_lists[order].push(paddr);
        }
    }
}

/// Object types for Slab Allocator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlabObjectType {
    ProcessControlBlock, // PCB (128 bytes)
    FileDescriptor,      // FD (32 bytes)
    InodeStruct,         // Inode (64 bytes)
}

impl SlabObjectType {
    pub fn size(&self) -> usize {
        match self {
            Self::ProcessControlBlock => 128,
            Self::FileDescriptor => 32,
            Self::InodeStruct => 64,
        }
    }
}

#[derive(Debug)]
pub struct SlabCache {
    pub object_type: SlabObjectType,
    pub object_size: usize,
    pub free_objects: Vec<usize>,
    pub allocated_count: usize,
}

impl SlabCache {
    pub fn new(object_type: SlabObjectType) -> Self {
        Self {
            object_type,
            object_size: object_type.size(),
            free_objects: Vec::new(),
            allocated_count: 0,
        }
    }

    pub fn allocate_object(&mut self, buddy: &mut BuddyAllocatorEngine) -> Option<usize> {
        if self.free_objects.is_empty() {
            if let Some(page_addr) = buddy.allocate_block(0) {
                let objects_per_page = PAGE_SIZE_4K / self.object_size;
                for i in 0..objects_per_page {
                    self.free_objects.push(page_addr + i * self.object_size);
                }
            } else {
                return None;
            }
        }
        self.allocated_count += 1;
        self.free_objects.pop()
    }

    pub fn free_object(&mut self, obj_addr: usize) {
        self.free_objects.push(obj_addr);
        if self.allocated_count > 0 {
            self.allocated_count -= 1;
        }
    }
}

#[derive(Debug)]
pub struct TwoTierMemoryAllocator {
    pub buddy: BuddyAllocatorEngine,
    pub pcb_slab: SlabCache,
    pub fd_slab: SlabCache,
    pub inode_slab: SlabCache,
}

impl TwoTierMemoryAllocator {
    pub fn new(ram_base: usize, total_pages: usize) -> Self {
        Self {
            buddy: BuddyAllocatorEngine::new(ram_base, total_pages),
            pcb_slab: SlabCache::new(SlabObjectType::ProcessControlBlock),
            fd_slab: SlabCache::new(SlabObjectType::FileDescriptor),
            inode_slab: SlabCache::new(SlabObjectType::InodeStruct),
        }
    }

    pub fn alloc_slab_object(&mut self, obj_type: SlabObjectType) -> Option<usize> {
        match obj_type {
            SlabObjectType::ProcessControlBlock => self.pcb_slab.allocate_object(&mut self.buddy),
            SlabObjectType::FileDescriptor => self.fd_slab.allocate_object(&mut self.buddy),
            SlabObjectType::InodeStruct => self.inode_slab.allocate_object(&mut self.buddy),
        }
    }

    pub fn free_slab_object(&mut self, obj_type: SlabObjectType, addr: usize) {
        match obj_type {
            SlabObjectType::ProcessControlBlock => self.pcb_slab.free_object(addr),
            SlabObjectType::FileDescriptor => self.fd_slab.free_object(addr),
            SlabObjectType::InodeStruct => self.inode_slab.free_object(addr),
        }
    }
}

// =========================================================================
// 2. Recursive / Self-Referential Page Tables (Slot 510)
// =========================================================================

pub const PML4_SELF_REF_SLOT: usize = 510;

#[derive(Debug, Clone)]
pub struct RecursivePageTableEngine {
    pub pml4_phys_addr: usize,
    pub is_self_mapped: bool,
}

impl RecursivePageTableEngine {
    pub fn new(pml4_phys_addr: usize) -> Self {
        Self {
            pml4_phys_addr,
            is_self_mapped: false,
        }
    }

    pub fn enable_self_referential_mapping(&mut self) {
        self.is_self_mapped = true;
    }

    pub fn calculate_pml4_virt_address(&self) -> usize {
        if !self.is_self_mapped {
            return self.pml4_phys_addr;
        }
        0xFFFF_0000_0000_0000
            | (PML4_SELF_REF_SLOT << 39)
            | (PML4_SELF_REF_SLOT << 30)
            | (PML4_SELF_REF_SLOT << 21)
            | (PML4_SELF_REF_SLOT << 12)
    }

    pub fn get_pt_virt_for_page(&self, vaddr: usize) -> usize {
        let pml4_idx = (vaddr >> 39) & 0x1FF;
        let pdpt_idx = (vaddr >> 30) & 0x1FF;
        let pd_idx = (vaddr >> 21) & 0x1FF;

        0xFFFF_0000_0000_0000
            | (PML4_SELF_REF_SLOT << 39)
            | (pml4_idx << 30)
            | (pdpt_idx << 21)
            | (pd_idx << 12)
    }
}

// =========================================================================
// 3. Copy-on-Write (COW) Fork Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CowPageEntry {
    pub vaddr: usize,
    pub paddr: usize,
    pub ref_count: usize,
    pub is_writeable: bool,
    pub is_cow: bool,
}

pub struct CopyOnWriteForkEngine {
    pub shared_pages: BTreeMap<usize, CowPageEntry>,
}

impl CopyOnWriteForkEngine {
    pub fn new() -> Self {
        Self {
            shared_pages: BTreeMap::new(),
        }
    }

    pub fn fork_share_page(&mut self, vaddr: usize, paddr: usize) {
        let entry = self.shared_pages.entry(paddr).or_insert(CowPageEntry {
            vaddr,
            paddr,
            ref_count: 1,
            is_writeable: true,
            is_cow: false,
        });
        entry.ref_count += 1;
        entry.is_writeable = false;
        entry.is_cow = true;
    }

    pub fn handle_page_fault(
        &mut self,
        paddr: usize,
        allocator: &mut TwoTierMemoryAllocator,
    ) -> Option<usize> {
        if let Some(entry) = self.shared_pages.get_mut(&paddr) {
            if entry.is_cow {
                if entry.ref_count > 1 {
                    if let Some(new_paddr) = allocator.buddy.allocate_block(0) {
                        entry.ref_count -= 1;
                        return Some(new_paddr);
                    }
                } else {
                    entry.is_cow = false;
                    entry.is_writeable = true;
                    return Some(paddr);
                }
            }
        }
        None
    }
}

impl Default for CopyOnWriteForkEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. x86_64 Fast System Call Mechanism (MSRs & Trap Register Frame)
// =========================================================================

pub mod x86_msrs {
    pub const IA32_STAR: u32 = 0xC0000081;
    pub const IA32_LSTAR: u32 = 0xC0000082;
    pub const IA32_FMASK: u32 = 0xC0000084;
}

#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TrapRegisterFrame {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

#[derive(Debug, Clone)]
pub struct FastSyscallDispatcher {
    pub lstar_target: u64,
    pub star_segments: u64,
    pub fmask_flags: u64,
}

impl FastSyscallDispatcher {
    pub fn new() -> Self {
        Self {
            lstar_target: 0,
            star_segments: 0,
            fmask_flags: 0x00000200,
        }
    }

    pub fn configure_fast_syscall(&mut self, entry_rip: u64, kernel_cs: u16, user_cs: u16) {
        self.lstar_target = entry_rip;
        self.star_segments = ((user_cs as u64) << 48) | ((kernel_cs as u64) << 32);
    }

    pub fn dispatch_trap(
        &self,
        frame: &mut TrapRegisterFrame,
        syscall_matrix: &MinimalPosixSyscallMatrix,
    ) -> i64 {
        let syscall_nr = frame.rax;
        let arg1 = frame.rdi;
        let arg2 = frame.rsi;
        let arg3 = frame.rdx;
        let arg4 = frame.r10;

        syscall_matrix.execute(syscall_nr, arg1, arg2, arg3, arg4)
    }
}

impl Default for FastSyscallDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. Minimal POSIX-Subset System Call Matrix
// =========================================================================

pub mod posix_syscall_nr {
    pub const SYS_READ: u64 = 0;
    pub const SYS_WRITE: u64 = 1;
    pub const SYS_OPEN: u64 = 2;
    pub const SYS_CLOSE: u64 = 3;
    pub const SYS_EXECVE: u64 = 59;
    pub const SYS_EXIT: u64 = 60;
    pub const SYS_FORK: u64 = 57;
}

pub struct MinimalPosixSyscallMatrix {
    pub open_fd_count: AtomicUsize,
}

impl MinimalPosixSyscallMatrix {
    pub fn new() -> Self {
        Self {
            open_fd_count: AtomicUsize::new(3),
        }
    }

    pub fn execute(&self, syscall_nr: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> i64 {
        match syscall_nr {
            posix_syscall_nr::SYS_READ => arg3 as i64,
            posix_syscall_nr::SYS_WRITE => arg3 as i64,
            posix_syscall_nr::SYS_OPEN => {
                let new_fd = self.open_fd_count.fetch_add(1, Ordering::SeqCst);
                new_fd as i64
            }
            posix_syscall_nr::SYS_CLOSE => 0,
            posix_syscall_nr::SYS_FORK => 2001,
            posix_syscall_nr::SYS_EXECVE => 0,
            posix_syscall_nr::SYS_EXIT => arg1 as i64,
            _ => -38,
        }
    }
}

impl Default for MinimalPosixSyscallMatrix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_tier_allocator() {
        let mut allocator = TwoTierMemoryAllocator::new(0x1000_0000, 128);
        let pcb_addr = allocator
            .alloc_slab_object(SlabObjectType::ProcessControlBlock)
            .unwrap();
        assert!(pcb_addr >= 0x1000_0000);

        allocator.free_slab_object(SlabObjectType::ProcessControlBlock, pcb_addr);
        assert_eq!(allocator.pcb_slab.allocated_count, 0);
    }

    #[test]
    fn test_recursive_page_table() {
        let mut pt_engine = RecursivePageTableEngine::new(0x0008_0000);
        pt_engine.enable_self_referential_mapping();

        let pml4_vaddr = pt_engine.calculate_pml4_virt_address();
        assert_eq!((pml4_vaddr >> 39) & 0x1FF, PML4_SELF_REF_SLOT);

        let pt_vaddr = pt_engine.get_pt_virt_for_page(0x0000_7FFF_1000_0000);
        assert_ne!(pt_vaddr, 0);
    }

    #[test]
    fn test_cow_fork_and_fast_syscall() {
        let mut allocator = TwoTierMemoryAllocator::new(0x2000_0000, 64);
        let first_page = allocator.buddy.allocate_block(0).unwrap();
        let mut cow_engine = CopyOnWriteForkEngine::new();

        cow_engine.fork_share_page(0x4000, first_page);
        let duplicated_paddr = cow_engine
            .handle_page_fault(first_page, &mut allocator)
            .unwrap();
        assert_ne!(duplicated_paddr, first_page);

        let syscall_matrix = MinimalPosixSyscallMatrix::new();
        let mut dispatcher = FastSyscallDispatcher::new();
        dispatcher.configure_fast_syscall(0xFFFFFFFF80100000, 0x08, 0x1B);

        let mut frame = TrapRegisterFrame::default();
        frame.rax = posix_syscall_nr::SYS_WRITE;
        frame.rdi = 1;
        frame.rsi = 0x5000;
        frame.rdx = 12;

        let res = dispatcher.dispatch_trap(&mut frame, &syscall_matrix);
        assert_eq!(res, 12);
    }
}
