#![no_std]
#![no_main]

/// OOP-based Ethernet PHY for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1956
/// Implements Ethernet PHY controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PHYID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PHYError { Success = 0, NotFound = 1 }

pub trait EthernetPHY {
    fn id(&self) -> PHYID;
    fn is_linked(&self) -> bool;
}

#[repr(C)]
pub struct SimpleEthernetPHY {
    pub id: PHYID,
    pub linked: AtomicUsize,
}

impl SimpleEthernetPHY {
    pub fn new(id: PHYID) -> Self {
        SimpleEthernetPHY {
            id,
            linked: AtomicUsize::new(0),
        }
    }
}

impl EthernetPHY for SimpleEthernetPHY {
    fn id(&self) -> PHYID { self.id }
    fn is_linked(&self) -> bool { self.linked.load(Ordering::SeqCst) == 1 }
}

pub trait PHYController {
    fn reset(&mut self, phy_id: PHYID) -> Result<(), PHYError>;
    fn read_reg(&self, phy_id: PHYID, reg: u8) -> Result<u16, PHYError>;
    def write_reg(&mut self, phy_id: PHYID, reg: u8, value: u16) -> Result<(), PHYError>;
}

#[repr(C)]
pub struct SimplePHYController {
    pub phys: Vec<Option<Box<dyn EthernetPHY>>>,
    pub next_id: AtomicUsize,
}

impl SimplePHYController {
    pub fn new() -> Self {
        SimplePHYController {
            phys: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PHYController for SimplePHYController {
    fn reset(&mut self, _phy_id: PHYID) -> Result<(), PHYError> {
        Ok(())
    }
    
    fn read_reg(&self, phy_id: PHYID, _reg: u8) -> Result<u16, PHYError> {
        if self.get_phy(phy_id).is_some() {
            Ok(0)
        } else {
            Err(PHYError::NotFound)
        }
    }
    
    fn write_reg(&mut self, phy_id: PHYID, _reg: u8, _value: u16) -> Result<(), PHYError> {
        if self.get_phy(phy_id).is_some() {
            Ok(())
        } else {
            Err(PHYError::NotFound)
        }
    }
    
    fn get_phy(&self, id: PHYID) -> Option<&dyn EthernetPHY> {
        for phy_option in &self.phys {
            if let Some(ref phy) = *phy_option {
                if phy.id() == id { return Some(phy.as_ref()); }
            }
        }
        None
    }
}

pub trait AutoNegotiation {
    def enable_autoneg(&mut self, phy_id: PHYID) -> Result<(), PHYError>;
    def get_speed(&self, phy_id: PHYID) -> Result<u8, PHYError>;
}

#[repr(C)]
pub struct SimpleAutoNegotiation {
    pub controller: SimplePHYController,
}

impl SimpleAutoNegotiation {
    pub fn new(controller: SimplePHYController) -> Self {
        SimpleAutoNegotiation { controller }
    }
}

impl AutoNegotiation for SimpleAutoNegotiation {
    fn enable_autoneg(&mut self, _phy_id: PHYID) -> Result<(), PHYError> {
        Ok(())
    }
    
    fn get_speed(&self, phy_id: PHYID) -> Result<u8, PHYError> {
        if self.controller.get_phy(phy_id).is_some() {
            Ok(0)
        } else {
            Err(PHYError::NotFound)
        }
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
