#![no_std]
#![no_main]

/// OOP-based Microphone for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1656
/// Implements microphone audio input

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MicID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MicError { Success = 0, NotFound = 1 }

pub trait Microphone {
    fn id(&self) -> MicID;
    fn sample_rate(&self) -> u32;
}

#[repr(C)]
pub struct SimpleMicrophone {
    pub id: MicID,
    pub sample_rate: AtomicUsize,
}

impl SimpleMicrophone {
    pub fn new(id: MicID) -> Self {
        SimpleMicrophone {
            id,
            sample_rate: AtomicUsize::new(44100),
        }
    }
}

impl Microphone for SimpleMicrophone {
    fn id(&self) -> MicID { self.id }
    fn sample_rate(&self) -> u32 { self.sample_rate.load(Ordering::SeqCst) as u32 }
}

pub trait MicController {
    fn start_recording(&mut self, mic_id: MicID) -> Result<(), MicError>;
    fn stop_recording(&mut self, mic_id: MicID) -> Result<(), MicError>;
    def read_sample(&self, mic_id: MicID) -> Result<i16, MicError>;
}

#[repr(C)]
pub struct SimpleMicController {
    pub mics: Vec<Option<Box<dyn Microphone>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMicController {
    pub fn new() -> Self {
        SimpleMicController {
            mics: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MicController for SimpleMicController {
    fn start_recording(&mut self, _mic_id: MicID) -> Result<(), MicError> {
        Ok(())
    }
    
    fn stop_recording(&mut self, _mic_id: MicID) -> Result<(), MicError> {
        Ok(())
    }
    
    fn read_sample(&self, mic_id: MicID) -> Result<i16, MicError> {
        if self.get_mic(mic_id).is_some() {
            Ok(0)
        } else {
            Err(MicError::NotFound)
        }
    }
    
    fn get_mic(&self, id: MicID) -> Option<&dyn Microphone> {
        for mic_option in &self.mics {
            if let Some(ref mic) = *mic_option {
                if mic.id() == id { return Some(mic.as_ref()); }
            }
        }
        None
    }
}

pub trait AudioInput {
    def set_gain(&mut self, mic_id: MicID, gain: u8) -> Result<(), MicError>;
    def read_buffer(&self, mic_id: MicID, buffer: &mut [i16]) -> Result<usize, MicError>;
}

#[repr(C)]
pub struct SimpleAudioInput {
    pub controller: SimpleMicController,
    pub gains: Vec<(MicID, AtomicUsize)>,
}

impl SimpleAudioInput {
    pub fn new(controller: SimpleMicController) -> Self {
        SimpleAudioInput {
            controller,
            gains: Vec::new(),
        }
    }
}

impl AudioInput for SimpleAudioInput {
    fn set_gain(&mut self, mic_id: MicID, gain: u8) -> Result<(), MicError> {
        self.gains.push((mic_id, AtomicUsize::new(gain as usize)));
        Ok(())
    }
    
    fn read_buffer(&self, mic_id: MicID, buffer: &mut [i16]) -> Result<usize, MicError> {
        if self.controller.get_mic(mic_id).is_some() {
            for sample in buffer.iter_mut() {
                *sample = 0;
            }
            Ok(buffer.len())
        } else {
            Err(MicError::NotFound)
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
