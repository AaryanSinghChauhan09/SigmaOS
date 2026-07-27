#![no_std]
#![no_main]

/// OOP-based Ethernet for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1166
/// Implements Ethernet MAC and PHY

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MACID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LinkState { Down = 0, Up = 1, AutoNegotiating = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EthernetError { Success = 0, NotFound = 1 }

pub trait EthernetMAC {
    fn id(&self) -> MACID;
    fn mac_address(&self) -> [u8; 6];
    fn link_state(&self) -> LinkState;
}

#[repr(C)]
pub struct SimpleEthernetMAC {
    pub id: MACID,
    pub mac_address: [u8; 6],
    pub link_state: AtomicUsize,
}

impl SimpleEthernetMAC {
    pub fn new(id: MACID, mac_address: [u8; 6]) -> Self {
        SimpleEthernetMAC {
            id,
            mac_address,
            link_state: AtomicUsize::new(LinkState::Down as usize),
        }
    }
}

impl EthernetMAC for SimpleEthernetMAC {
    fn id(&self) -> MACID { self.id }
    fn mac_address(&self) -> [u8; 6] { self.mac_address }
    fn link_state(&self) -> LinkState { unsafe { core::mem::transmute(self.link_state.load(Ordering::SeqCst)) } }
}

pub trait EthernetController {
    fn init(&mut self, mac_id: MACID) -> Result<(), EthernetError>;
    fn send_packet(&self, mac_id: MACID, data: &[u8]) -> Result<(), EthernetError>;
    fn receive_packet(&self, mac_id: MACID, buffer: &mut [u8]) -> Result<usize, EthernetError>;
}

#[repr(C)]
pub struct SimpleEthernetController {
    pub macs: Vec<Option<Box<dyn EthernetMAC>>>,
    pub next_id: AtomicUsize,
}

impl SimpleEthernetController {
    pub fn new() -> Self {
        SimpleEthernetController {
            macs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl EthernetController for SimpleEthernetController {
    fn init(&mut self, mac_id: MACID) -> Result<(), EthernetError> {
        for mac_option in &mut self.macs {
            if let Some(ref mut mac) = *mac_option {
                if mac.id() == mac_id {
                    mac.link_state.store(LinkState::Up as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(EthernetError::NotFound)
    }
    
    fn send_packet(&self, mac_id: MACID, _data: &[u8]) -> Result<(), EthernetError> {
        if self.get_mac(mac_id).is_some() {
            Ok(())
        } else {
            Err(EthernetError::NotFound)
        }
    }
    
    fn receive_packet(&self, mac_id: MACID, buffer: &mut [u8]) -> Result<usize, EthernetError> {
        if self.get_mac(mac_id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
            Ok(buffer.len())
        } else {
            Err(EthernetError::NotFound)
        }
    }
    
    fn get_mac(&self, id: MACID) -> Option<&dyn EthernetMAC> {
        for mac_option in &self.macs {
            if let Some(ref mac) = *mac_option {
                if mac.id() == id { return Some(mac.as_ref()); }
            }
        }
        None
    }
}

pub trait PHYInterface {
    def read_register(&self, phy_addr: u8, reg: u8) -> Result<u16, EthernetError>;
    def write_register(&self, phy_addr: u8, reg: u8, value: u16) -> Result<(), EthernetError>;
}

#[repr(C)]
pub struct SimplePHYInterface {
    pub controller: SimpleEthernetController,
}

impl SimplePHYInterface {
    pub fn new(controller: SimpleEthernetController) -> Self {
        SimplePHYInterface { controller }
    }
}

impl PHYInterface for SimplePHYInterface {
    fn read_register(&self, _phy_addr: u8, _reg: u8) -> Result<u16, EthernetError> {
        Ok(0)
    }
    
    fn write_register(&self, _phy_addr: u8, _reg: u8, _value: u16) -> Result<(), EthernetError> {
        Ok(())
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
