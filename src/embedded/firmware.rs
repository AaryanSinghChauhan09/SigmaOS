#![no_std]
#![no_main]

/// OOP-based Firmware Update for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1016
/// Implements firmware update and OTA

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type FirmwareID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FirmwareState { Idle = 0, Downloading = 1, Installing = 2, Completed = 3, Error = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FirmwareError { Success = 0, NotFound = 1, UpdateFailed = 2 }

pub trait Firmware {
    fn id(&self) -> FirmwareID;
    fn version(&self) -> &[u8];
    fn size(&self) -> u64;
    fn checksum(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleFirmware {
    pub id: FirmwareID,
    pub version: [u8; 16],
    pub size: AtomicUsize,
    pub checksum: [u8; 32],
}

impl SimpleFirmware {
    pub fn new(id: FirmwareID, version: &[u8], size: u64, checksum: &[u8]) -> Self {
        let mut ver_array = [0u8; 16];
        let mut chk_array = [0u8; 32];
        let ver_len = version.len().min(15);
        let chk_len = checksum.len().min(31);
        for i in 0..ver_len { ver_array[i] = version[i]; }
        for i in 0..chk_len { chk_array[i] = checksum[i]; }
        SimpleFirmware {
            id,
            version: ver_array,
            size: AtomicUsize::new(size as usize),
            checksum: chk_array,
        }
    }
}

impl Firmware for SimpleFirmware {
    fn id(&self) -> FirmwareID { self.id }
    fn version(&self) -> &[u8] {
        let len = self.version.iter().position(|&b| b == 0).unwrap_or(16);
        &self.version[..len]
    }
    fn size(&self) -> u64 { self.size.load(Ordering::SeqCst) as u64 }
    fn checksum(&self) -> &[u8] {
        let len = self.checksum.iter().position(|&b| b == 0).unwrap_or(32);
        &self.checksum[..len]
    }
}

pub trait FirmwareUpdater {
    fn download(&mut self, url: &[u8]) -> Result<FirmwareID, FirmwareError>;
    fn install(&mut self, id: FirmwareID) -> Result<(), FirmwareError>;
    fn get_state(&self, id: FirmwareID) -> FirmwareState;
}

#[repr(C)]
pub struct SimpleFirmwareUpdater {
    pub firmwares: Vec<Option<Box<dyn Firmware>>>,
    pub states: Vec<(FirmwareID, AtomicUsize)>,
    pub next_id: AtomicUsize,
}

impl SimpleFirmwareUpdater {
    pub fn new() -> Self {
        SimpleFirmwareUpdater {
            firmwares: Vec::new(),
            states: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl FirmwareUpdater for SimpleFirmwareUpdater {
    fn download(&mut self, _url: &[u8]) -> Result<FirmwareID, FirmwareError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let firmware = SimpleFirmware::new(id, b"1.0.0", 1024, b"abc123");
        self.firmwares.push(Some(Box::new(firmware)));
        self.states.push((id, AtomicUsize::new(FirmwareState::Downloading as usize)));
        Ok(id)
    }
    
    fn install(&mut self, id: FirmwareID) -> Result<(), FirmwareError> {
        for &mut (fw_id, ref state) in &mut self.states {
            if fw_id == id {
                state.store(FirmwareState::Installing as usize, Ordering::SeqCst);
                state.store(FirmwareState::Completed as usize, Ordering::SeqCst);
                return Ok(());
            }
        }
        Err(FirmwareError::NotFound)
    }
    
    fn get_state(&self, id: FirmwareID) -> FirmwareState {
        for &(fw_id, ref state) in &self.states {
            if fw_id == id {
                return unsafe { core::mem::transmute(state.load(Ordering::SeqCst)) };
            }
        }
        FirmwareState::Idle
    }
}

pub trait OTAUpdate {
    def check_update(&self, current_version: &[u8]) -> Option<&[u8]>;
    def rollback(&mut self) -> Result<(), FirmwareError>;
}

#[repr(C)]
pub struct SimpleOTAUpdate {
    pub latest_version: [u8; 16],
}

impl SimpleOTAUpdate {
    pub fn new() -> Self {
        let mut ver_array = [0u8; 16];
        let ver_len = b"2.0.0".len().min(15);
        for i in 0..ver_len {
            ver_array[i] = b"2.0.0"[i];
        }
        SimpleOTAUpdate {
            latest_version: ver_array,
        }
    }
}

impl OTAUpdate for SimpleOTAUpdate {
    fn check_update(&self, current_version: &[u8]) -> Option<&[u8]> {
        if current_version != b"2.0.0" {
            Some(&self.latest_version)
        } else {
            None
        }
    }
    
    fn rollback(&mut self) -> Result<(), FirmwareError> {
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
