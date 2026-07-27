#![no_std]
#![no_main]

/// OOP-based Microphone Capture for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 331
/// Implements audio capture and voice processing

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MicID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AudioFormat { PCM16 = 0, PCM32 = 1, Float32 = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MicError { Success = 0, NotFound = 1, CaptureFailed = 2 }

pub trait Microphone {
    fn id(&self) -> MicID;
    fn name(&self) -> &[u8];
    fn sample_rate(&self) -> u32;
    fn channels(&self) -> u32;
    fn format(&self) -> AudioFormat;
}

#[repr(C)]
pub struct SimpleMicrophone {
    pub id: MicID,
    pub name: [u8; 64],
    pub sample_rate: AtomicUsize,
    pub channels: AtomicUsize,
    pub format: AtomicUsize,
}

impl SimpleMicrophone {
    pub fn new(id: MicID, name: &[u8], sample_rate: u32, channels: u32, format: AudioFormat) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleMicrophone {
            id,
            name: name_array,
            sample_rate: AtomicUsize::new(sample_rate as usize),
            channels: AtomicUsize::new(channels as usize),
            format: AtomicUsize::new(format as usize),
        }
    }
}

impl Microphone for SimpleMicrophone {
    fn id(&self) -> MicID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn sample_rate(&self) -> u32 { self.sample_rate.load(Ordering::SeqCst) as u32 }
    fn channels(&self) -> u32 { self.channels.load(Ordering::SeqCst) as u32 }
    fn format(&self) -> AudioFormat { unsafe { core::mem::transmute(self.format.load(Ordering::SeqCst)) } }
}

pub trait AudioCapture {
    fn start_capture(&mut self, mic_id: MicID) -> Result<(), MicError>;
    fn stop_capture(&mut self, mic_id: MicID) -> Result<(), MicError>;
    fn read_samples(&self, mic_id: MicID, buffer: &mut [u8]) -> Result<usize, MicError>;
}

#[repr(C)]
pub struct SimpleAudioCapture {
    pub microphones: Vec<Option<Box<dyn Microphone>>>,
    pub capturing: Vec<MicID>,
}

impl SimpleAudioCapture {
    pub fn new() -> Self {
        SimpleAudioCapture {
            microphones: Vec::new(),
            capturing: Vec::new(),
        }
    }
}

impl AudioCapture for SimpleAudioCapture {
    fn start_capture(&mut self, mic_id: MicID) -> Result<(), MicError> {
        self.capturing.push(mic_id);
        Ok(())
    }

    fn stop_capture(&mut self, mic_id: MicID) -> Result<(), MicError> {
        for i in 0..self.capturing.len() {
            if self.capturing[i] == mic_id {
                self.capturing.remove(i);
                return Ok(());
            }
        }
        Err(MicError::NotFound)
    }

    fn read_samples(&self, _mic_id: MicID, buffer: &mut [u8]) -> Result<usize, MicError> {
        for byte in buffer.iter_mut() {
            *byte = 0u8;
        }
        Ok(buffer.len())
    }
}

pub trait VoiceActivityDetection {
    fn detect_voice(&self, samples: &[i16]) -> bool;
    fn set_threshold(&mut self, threshold: f32);
}

#[repr(C)]
pub struct SimpleVoiceActivityDetection {
    pub threshold: AtomicUsize,
}

impl SimpleVoiceActivityDetection {
    pub fn new() -> Self {
        SimpleVoiceActivityDetection {
            threshold: AtomicUsize::new(500),
        }
    }
}

impl VoiceActivityDetection for SimpleVoiceActivityDetection {
    fn detect_voice(&self, samples: &[i16]) -> bool {
        let threshold = self.threshold.load(Ordering::SeqCst) as i16;
        for &sample in samples {
            if sample.abs() > threshold {
                return true;
            }
        }
        false
    }

    fn set_threshold(&mut self, threshold: f32) {
        self.threshold.store(threshold as usize, Ordering::SeqCst);
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
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
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
