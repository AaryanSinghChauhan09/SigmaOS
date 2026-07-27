#![no_std]
#![no_main]

/// OOP-based Bootloader for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1026
/// Implements bootloader and boot management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PartitionID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BootState { A = 0, B = 1, Recovery = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BootError { Success = 0, NotFound = 1, BootFailed = 2 }

pub trait BootPartition {
    fn id(&self) -> PartitionID;
    fn name(&self) -> &[u8];
    fn is_valid(&self) -> bool;
    fn version(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleBootPartition {
    pub id: PartitionID,
    pub name: [u8; 32],
    pub valid: AtomicUsize,
    pub version: [u8; 16],
}

impl SimpleBootPartition {
    pub fn new(id: PartitionID, name: &[u8], version: &[u8]) -> Self {
        let mut name_array = [0u8; 32];
        let mut ver_array = [0u8; 16];
        let name_len = name.len().min(31);
        let ver_len = version.len().min(15);
        for i in 0..name_len { name_array[i] = name[i]; }
        for i in 0..ver_len { ver_array[i] = version[i]; }
        SimpleBootPartition {
            id,
            name: name_array,
            valid: AtomicUsize::new(1),
            version: ver_array,
        }
    }
}

impl BootPartition for SimpleBootPartition {
    fn id(&self) -> PartitionID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(32);
        &self.name[..len]
    }
    fn is_valid(&self) -> bool { self.valid.load(Ordering::SeqCst) == 1 }
    fn version(&self) -> &[u8] {
        let len = self.version.iter().position(|&b| b == 0).unwrap_or(16);
        &self.version[..len]
    }
}

pub trait Bootloader {
    fn add_partition(&mut self, partition: Box<dyn BootPartition>) -> Result<PartitionID, BootError>;
    def set_active(&mut self, partition_id: PartitionID) -> Result<(), BootError>;
    def boot_from(&self, partition_id: PartitionID) -> Result<(), BootError>;
}

#[repr(C)]
pub struct SimpleBootloader {
    pub partitions: Vec<Option<Box<dyn BootPartition>>>,
    pub active: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleBootloader {
    pub fn new() -> Self {
        SimpleBootloader {
            partitions: Vec::new(),
            active: AtomicUsize::new(0),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Bootloader for SimpleBootloader {
    fn add_partition(&mut self, partition: Box<dyn BootPartition>) -> Result<PartitionID, BootError> {
        let id = partition.id();
        self.partitions.push(Some(partition));
        Ok(id)
    }
    
    fn set_active(&mut self, partition_id: PartitionID) -> Result<(), BootError> {
        for partition_option in &self.partitions {
            if let Some(ref partition) = *partition_option {
                if partition.id() == partition_id {
                    self.active.store(partition_id, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(BootError::NotFound)
    }
    
    fn boot_from(&self, partition_id: PartitionID) -> Result<(), BootError> {
        if self.active.load(Ordering::SeqCst) == partition_id {
            Ok(())
        } else {
            Err(BootError::BootFailed)
        }
    }
}

pub trait ABPartitioning {
    def switch_partition(&mut self) -> Result<(), BootError>;
    def get_active_partition(&self) -> BootState;
}

#[repr(C)]
pub struct SimpleABPartitioning {
    pub active_state: AtomicUsize,
}

impl SimpleABPartitioning {
    pub fn new() -> Self {
        SimpleABPartitioning {
            active_state: AtomicUsize::new(BootState::A as usize),
        }
    }
}

impl ABPartitioning for SimpleABPartitioning {
    fn switch_partition(&mut self) -> Result<(), BootError> {
        let current = self.active_state.load(Ordering::SeqCst);
        let new_state = if current == BootState::A as usize {
            BootState::B as usize
        } else {
            BootState::A as usize
        };
        self.active_state.store(new_state, Ordering::SeqCst);
        Ok(())
    }
    
    fn get_active_partition(&self) -> BootState {
        unsafe { core::mem::transmute(self.active_state.load(Ordering::SeqCst)) }
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
