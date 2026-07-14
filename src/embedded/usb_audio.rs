#![no_std]
#![no_main]

/// OOP-based USB Audio for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2256
/// Implements USB Audio

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type AudioUSBID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AudioUSBError { Success = 0, NotFound = 1 }

pub trait USBAudio {
    fn id(&self) -> AudioUSBID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleUSBAudio {
    pub id: AudioUSBID,
    pub connected: AtomicUsize,
}

impl SimpleUSBAudio {
    pub fn new(id: AudioUSBID) -> Self {
        SimpleUSBAudio {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl USBAudio for SimpleUSBAudio {
    fn id(&self) -> AudioUSBID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait AudioUSBController {
    fn init(&mut self, audio_id: AudioUSBID) -> Result<(), AudioUSBError>;
    fn send_audio(&self, audio_id: AudioUSBID, samples: &[i16]) -> Result<usize, AudioUSBError>;
    def receive_audio(&self, audio_id: AudioUSBID, buffer: &mut [i16]) -> Result<usize, AudioUSBError>;
}

#[repr(C)]
pub struct SimpleAudioUSBController {
    pub audio_devices: Vec<Option<Box<dyn USBAudio>>>,
    pub next_id: AtomicUsize,
}

impl SimpleAudioUSBController {
    pub fn new() -> Self {
        SimpleAudioUSBController {
            audio_devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl AudioUSBController for SimpleAudioUSBController {
    fn init(&mut self, audio_id: AudioUSBID) -> Result<(), AudioUSBError> {
        for audio_option in &mut self.audio_devices {
            if let Some(ref mut audio) = *audio_option {
                if audio.id() == audio_id {
                    audio.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(AudioUSBError::NotFound)
    }
    
    fn send_audio(&self, audio_id: AudioUSBID, _samples: &[i16]) -> Result<usize, AudioUSBError> {
        if self.get_audio(audio_id).is_some() {
            Ok(0)
        } else {
            Err(AudioUSBError::NotFound)
        }
    }
    
    fn receive_audio(&self, audio_id: AudioUSBID, buffer: &mut [i16]) -> Result<usize, AudioUSBError> {
        if self.get_audio(audio_id).is_some() {
            for sample in buffer.iter_mut() { *sample = 0; }
            Ok(buffer.len())
        } else {
            Err(AudioUSBError::NotFound)
        }
    }
    
    fn get_audio(&self, id: AudioUSBID) -> Option<&dyn USBAudio> {
        for audio_option in &self.audio_devices {
            if let Some(ref audio) = *audio_option {
                if audio.id() == id { return Some(audio.as_ref()); }
            }
        }
        None
    }
}

pub trait AudioStream {
    def set_sample_rate(&mut self, audio_id: AudioUSBID, rate: u32) -> Result<(), AudioUSBError>;
    def get_sample_rate(&self, audio_id: AudioUSBID) -> Result<u32, AudioUSBError>;
}

#[repr(C)]
pub struct SimpleAudioStream {
    pub controller: SimpleAudioUSBController,
    pub sample_rates: Vec<(AudioUSBID, AtomicUsize)>,
}

impl SimpleAudioStream {
    pub fn new(controller: SimpleAudioUSBController) -> Self {
        SimpleAudioStream {
            controller,
            sample_rates: Vec::new(),
        }
    }
}

impl AudioStream for SimpleAudioStream {
    fn set_sample_rate(&mut self, audio_id: AudioUSBID, rate: u32) -> Result<(), AudioUSBError> {
        self.sample_rates.push((audio_id, AtomicUsize::new(rate as usize)));
        Ok(())
    }
    
    fn get_sample_rate(&self, audio_id: AudioUSBID) -> Result<u32, AudioUSBError> {
        for &(id, ref rate) in &self.sample_rates {
            if id == audio_id {
                return Ok(rate.load(Ordering::SeqCst) as u32);
            }
        }
        Err(AudioUSBError::NotFound)
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
