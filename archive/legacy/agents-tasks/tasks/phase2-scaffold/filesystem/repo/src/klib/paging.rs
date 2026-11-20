use crate::klib::vec::Vec;
/// OOP-based Paging + Virtual Memory for SigmaOS
/// Based on Ultimate Dominance Strategy: Stage 0 Week 7-8
/// Implements 4-level page tables, PML4, userspace isolation, page fault handling
use core::sync::atomic::{AtomicUsize, Ordering};

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageSize {
    Standard4KB,
    Huge2MB,
    Giant1GB,
}

impl PageSize {
    pub fn byte_size(&self) -> usize {
        match self {
            PageSize::Standard4KB => 4096,
            PageSize::Huge2MB => 2 * 1024 * 1024,
            PageSize::Giant1GB => 1024 * 1024 * 1024,
        }
    }
}

#[derive(Debug)]
pub struct SimplePageTableEntry {
    pub present: AtomicUsize,
    pub writable: AtomicUsize,
    pub user_accessible: AtomicUsize,
    pub physical_addr: AtomicUsize,
    pub accessed: AtomicUsize,
    pub dirty: AtomicUsize,
    pub cow: AtomicUsize,
}

impl SimplePageTableEntry {
    pub fn new() -> Self {
        Self {
            present: AtomicUsize::new(0),
            writable: AtomicUsize::new(0),
            user_accessible: AtomicUsize::new(0),
            physical_addr: AtomicUsize::new(0),
            accessed: AtomicUsize::new(0),
            dirty: AtomicUsize::new(0),
            cow: AtomicUsize::new(0),
        }
    }
}

impl Default for SimplePageTableEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SimplePageTableEntry {
    fn clone(&self) -> Self {
        Self {
            present: AtomicUsize::new(self.present.load(Ordering::SeqCst)),
            writable: AtomicUsize::new(self.writable.load(Ordering::SeqCst)),
            user_accessible: AtomicUsize::new(self.user_accessible.load(Ordering::SeqCst)),
            physical_addr: AtomicUsize::new(self.physical_addr.load(Ordering::SeqCst)),
            accessed: AtomicUsize::new(self.accessed.load(Ordering::SeqCst)),
            dirty: AtomicUsize::new(self.dirty.load(Ordering::SeqCst)),
            cow: AtomicUsize::new(self.cow.load(Ordering::SeqCst)),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PageTableLevel {
    PML4 = 0,
    PDPT = 1,
    PD = 2,
    PT = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFaultError {
    Success = 0,
    NotPresent = 1,
    PermissionDenied = 2,
    InvalidAddress = 3,
}
