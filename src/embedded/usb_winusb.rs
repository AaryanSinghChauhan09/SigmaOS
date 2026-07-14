#![no_std]
#![no_main]

/// OOP-based USB WinUSB for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2246
/// Implements USB WinUSB

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type WinUSBID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WinUSBError { Success = 0, NotFound = 1 }

pub trait USBWinUSB {
    fn id(&self) -> WinUSBID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleUSBWinUSB {
    pub id: WinUSBID,
    pub connected: AtomicUsize,
}

impl SimpleUSBWinUSB {
    pub fn new(id: WinUSBID) -> Self {
        SimpleUSBWinUSB {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl USBWinUSB for SimpleUSBWinUSB {
    fn id(&self) -> WinUSBID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait WinUSBController {
    fn init(&mut self, winusb_id: WinUSBID) -> Result<(), WinUSBError>;
    fn send(&self, winusb_id: WinUSBID, data: &[u8]) -> Result<usize, WinUSBError>;
    def receive(&self, winusb_id: WinUSBID, buffer: &mut [u8]) -> Result<usize, WinUSBError>;
}

#[repr(C)]
pub struct SimpleWinUSBController {
    pub winusb_devices: Vec<Option<Box<dyn USBWinUSB>>>,
    pub next_id: AtomicUsize,
}

impl SimpleWinUSBController {
    pub fn new() -> Self {
        SimpleWinUSBController {
            winusb_devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl WinUSBController for SimpleWinUSBController {
    fn init(&mut self, winusb_id: WinUSBID) -> Result<(), WinUSBError> {
        for winusb_option in &mut self.winusb_devices {
            if let Some(ref mut winusb) = *winusb_option {
                if winusb.id() == winusb_id {
                    winusb.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(WinUSBError::NotFound)
    }
    
    fn send(&self, winusb_id: WinUSBID, _data: &[u8]) -> Result<usize, WinUSBError> {
        if self.get_winusb(winusb_id).is_some() {
            Ok(0)
        } else {
            Err(WinUSBError::NotFound)
        }
    }
    
    fn receive(&self, winusb_id: WinUSBID, buffer: &mut [u8]) -> Result<usize, WinUSBError> {
        if self.get_winusb(winusb_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(WinUSBError::NotFound)
        }
    }
    
    fn get_winusb(&self, id: WinUSBID) -> Option<&dyn USBWinUSB> {
        for winusb_option in &self.winusb_devices {
            if let Some(ref winusb) = *winusb_option {
                if winusb.id() == id { return Some(winusb.as_ref()); }
            }
        }
        None
    }
}

pub trait WinUSBDescriptor {
    def set_guid(&mut self, winusb_id: WinUSBID, guid: &[u8]) -> Result<(), WinUSBError>;
    def get_guid(&self, winusb_id: WinUSBID) -> Result<[u8; 16], WinUSBError>;
}

#[repr(C)]
pub struct SimpleWinUSBDescriptor {
    pub controller: SimpleWinUSBController,
    pub guids: Vec<(WinUSBID, [u8; 16])>,
}

impl SimpleWinUSBDescriptor {
    pub fn new(controller: SimpleWinUSBController) -> Self {
        SimpleWinUSBDescriptor {
            controller,
            guids: Vec::new(),
        }
    }
}

impl WinUSBDescriptor for SimpleWinUSBDescriptor {
    fn set_guid(&mut self, winusb_id: WinUSBID, guid: &[u8]) -> Result<(), WinUSBError> {
        let mut guid_array = [0u8; 16];
        let len = guid.len().min(15);
        unsafe {
            core::ptr::copy_nonoverlapping(guid.as_ptr(), guid_array.as_mut_ptr(), len);
        }
        self.guids.push((winusb_id, guid_array));
        Ok(())
    }
    
    fn get_guid(&self, winusb_id: WinUSBID) -> Result<[u8; 16], WinUSBError> {
        for &(id, ref guid) in &self.guids {
            if id == winusb_id {
                return Ok(*guid);
            }
        }
        Err(WinUSBError::NotFound)
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
