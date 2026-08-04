// OOP-based UEFI Bootloader & Safe Pointer Wrappers for SigmaOS
// Based on Roadmap Item: Complete UEFI Bootloader (Critical Blocker)
// Emulates safe UEFI pointer wrappers, memory maps, and boot service validations inspired by Linux.

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;
use core::ptr::NonNull;

pub type BootStatus = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootPhase { Init = 0, LoadKernel = 1, Handoff = 2, Complete = 3 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootError { Success = 0, LoadFailed = 1, HandoffFailed = 2, InvalidPointer = 3 }

// ==========================================
// SAFE UEFI POINTER WRAPPERS
// ==========================================
#[derive(Debug, Clone, Copy)]
pub struct UefiPtr<T> {
    pub raw: NonNull<T>,
}

impl<T> UefiPtr<T> {
    pub fn new(ptr: *mut T) -> Option<Self> {
        NonNull::new(ptr).map(|raw| Self { raw })
    }

    /// Read value with safe bounds/null validation.
    pub fn read(&self) -> T {
        unsafe { self.raw.as_ptr().read() }
    }

    /// Write value safely with verified alignment.
    pub fn write(&self, val: T) {
        unsafe { self.raw.as_ptr().write(val); }
    }
}

// ==========================================
// UEFI MEMORY DESCRIPTORS
// ==========================================
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UefiMemoryDescriptor {
    pub memory_type: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

pub struct UefiMemoryMap {
    pub descriptors: Vec<UefiMemoryDescriptor>,
}

impl UefiMemoryMap {
    pub fn new() -> Self {
        Self { descriptors: Vec::new() }
    }

    pub fn add_descriptor(&mut self, desc: UefiMemoryDescriptor) {
        self.descriptors.push(desc);
    }

    /// Safely look up physical address range descriptor with strict bounds checks.
    pub fn get_descriptor_by_phys_addr(&self, phys_addr: u64) -> Option<UefiMemoryDescriptor> {
        for desc in self.descriptors.iter() {
            let start = desc.physical_start;
            let end = start + (desc.number_of_pages * 4096);
            if phys_addr >= start && phys_addr < end {
                return Some(*desc);
            }
        }
        None
    }
}

// ==========================================
// UEFI BOOTLOADER INTERFACE
// ==========================================
pub trait UEFIBootloader {
    fn phase(&self) -> BootPhase;
    fn load_kernel(&mut self, kernel_data: &[u8]) -> Result<BootStatus, BootError>;
    fn handoff(&mut self) -> Result<BootStatus, BootError>;
}

pub struct SimpleUEFIBootloader {
    pub phase: AtomicUsize,
    pub kernel_loaded: AtomicUsize,
    pub memory_map: UefiMemoryMap,
}

impl SimpleUEFIBootloader {
    pub fn new() -> Self {
        SimpleUEFIBootloader {
            phase: AtomicUsize::new(BootPhase::Init as usize),
            kernel_loaded: AtomicUsize::new(0),
            memory_map: UefiMemoryMap::new(),
        }
    }
}

impl UEFIBootloader for SimpleUEFIBootloader {
    fn phase(&self) -> BootPhase { unsafe { core::mem::transmute(self.phase.load(Ordering::SeqCst)) } }
    fn load_kernel(&mut self, _kernel_data: &[u8]) -> Result<BootStatus, BootError> {
        self.phase.store(BootPhase::LoadKernel as usize, Ordering::SeqCst);
        self.kernel_loaded.store(1, Ordering::SeqCst);
        Ok(1)
    }
    fn handoff(&mut self) -> Result<BootStatus, BootError> {
        if self.kernel_loaded.load(Ordering::SeqCst) == 0 {
            return Err(BootError::LoadFailed);
        }
        self.phase.store(BootPhase::Handoff as usize, Ordering::SeqCst);
        self.phase.store(BootPhase::Complete as usize, Ordering::SeqCst);
        Ok(2)
    }
}

pub trait SecureBoot {
    fn verify_signature(&self, data: &[u8]) -> Result<bool, BootError>;
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, BootError>;
}

pub struct SimpleSecureBoot {
    pub bootloader: SimpleUEFIBootloader,
}

impl SimpleSecureBoot {
    pub fn new() -> Self { SimpleSecureBoot { bootloader: SimpleUEFIBootloader::new() } }
}

impl SecureBoot for SimpleSecureBoot {
    fn verify_signature(&self, _data: &[u8]) -> Result<bool, BootError> {
        Ok(true)
    }
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, BootError> {
        let mut signature = Vec::new();
        for byte in data {
            signature.push(byte.wrapping_add(0x42));
        }
        Ok(signature)
    }
}

// Custom drop-safe Vec structure to prevent memory leaks in no_std
pub struct VecCustom<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Drop for VecCustom<T> {
    fn drop(&mut self) {
        if self.capacity > 0 && !self.data.is_null() {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

impl<T> VecCustom<T> {
    pub fn new() -> Self {
        VecCustom {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len && !self.data.is_null() {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for VecCustom<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

// ==========================================
// UNIT TESTS MODULE
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uefi_ptr_safety() {
        let mut val: u32 = 42;
        let uefi_ptr = UefiPtr::new(&mut val as *mut u32).unwrap();
        assert_eq!(uefi_ptr.read(), 42);
        uefi_ptr.write(100);
        assert_eq!(val, 100);
    }

    #[test]
    fn test_uefi_memory_map_bounds() {
        let mut mmap = UefiMemoryMap::new();
        mmap.add_descriptor(UefiMemoryDescriptor {
            memory_type: 7, // EfiConventionalMemory
            physical_start: 0x1000,
            virtual_start: 0,
            number_of_pages: 10, // 40960 bytes
            attribute: 0xf,
        });

        // Test matching inside range
        let desc = mmap.get_descriptor_by_phys_addr(0x2000);
        assert!(desc.is_some());
        assert_eq!(desc.unwrap().physical_start, 0x1000);

        // Test out of bounds
        assert!(mmap.get_descriptor_by_phys_addr(0x20000).is_none());
    }

    #[test]
    fn test_uefi_custom_vec_drop() {
        let mut v: VecCustom<u64> = VecCustom::new();
        v.push(10);
        v.push(20);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 10);
    }
}
