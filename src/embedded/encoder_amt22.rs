#![no_std]
#![no_main]

/// OOP-based AMT22 Encoder for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3296
/// Implements AMT22 magnetic encoder

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type AMT22ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AMT22Error { Success = 0, NotFound = 1 }

pub trait AMT22Encoder {
    fn id(&self) -> AMT22ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleAMT22Encoder {
    pub id: AMT22ID,
    pub initialized: AtomicUsize,
}

impl SimpleAMT22Encoder {
    pub fn new(id: AMT22ID) -> Self {
        SimpleAMT22Encoder {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl AMT22Encoder for SimpleAMT22Encoder {
    fn id(&self) -> AMT22ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait AMT22Controller {
    fn init(&mut self, encoder_id: AMT22ID) -> Result<(), AMT22Error>;
    fn read_angle(&self, encoder_id: AMT22ID) -> Result<u16, AMT22Error>;
    def zero(&mut self, encoder_id: AMT22ID) -> Result<(), AMT22Error>;
}

#[repr(C)]
pub struct SimpleAMT22Controller {
    pub encoders: Vec<Option<Box<dyn AMT22Encoder>>>,
    pub next_id: AtomicUsize,
}

impl SimpleAMT22Controller {
    pub fn new() -> Self {
        SimpleAMT22Controller {
            encoders: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl AMT22Controller for SimpleAMT22Controller {
    fn init(&mut self, encoder_id: AMT22ID) -> Result<(), AMT22Error> {
        for encoder_option in &mut self.encoders {
            if let Some(ref mut encoder) = *encoder_option {
                if encoder.id() == encoder_id {
                    encoder.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(AMT22Error::NotFound)
    }
    
    fn read_angle(&self, encoder_id: AMT22ID) -> Result<u16, AMT22Error> {
        if self.get_encoder(encoder_id).is_some() {
            Ok(0)
        } else {
            Err(AMT22Error::NotFound)
        }
    }
    
    fn zero(&mut self, encoder_id: AMT22ID) -> Result<(), AMT22Error> {
        if self.get_encoder(encoder_id).is_some() {
            Ok(())
        } else {
            Err(AMT22Error::NotFound)
        }
    }
    
    fn get_encoder(&self, id: AMT22ID) -> Option<&dyn AMT22Encoder> {
        for encoder_option in &self.encoders {
            if let Some(ref encoder) = *encoder_option {
                if encoder.id() == id { return Some(encoder.as_ref()); }
            }
        }
        None
    }
}

pub trait AMT22Resolution {
    def set_resolution(&mut self, encoder_id: AMT22ID, res: u16) -> Result<(), AMT22Error>;
}

#[repr(C)]
pub struct SimpleAMT22Resolution {
    pub controller: SimpleAMT22Controller,
    pub resolutions: Vec<(AMT22ID, AtomicUsize)>,
}

impl SimpleAMT22Resolution {
    pub fn new(controller: SimpleAMT22Controller) -> Self {
        SimpleAMT22Resolution {
            controller,
            resolutions: Vec::new(),
        }
    }
}

impl AMT22Resolution for SimpleAMT22Resolution {
    fn set_resolution(&mut self, encoder_id: AMT22ID, res: u16) -> Result<(), AMT22Error> {
        self.resolutions.push((encoder_id, AtomicUsize::new(res as usize)));
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
