#![no_std]
#![no_main]

/// OOP-based Speaker for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1646
/// Implements speaker audio output

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SpeakerID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SpeakerError { Success = 0, NotFound = 1 }

pub trait Speaker {
    fn id(&self) -> SpeakerID;
    fn volume(&self) -> u8;
}

#[repr(C)]
pub struct SimpleSpeaker {
    pub id: SpeakerID,
    pub volume: AtomicUsize,
}

impl SimpleSpeaker {
    pub fn new(id: SpeakerID) -> Self {
        SimpleSpeaker {
            id,
            volume: AtomicUsize::new(128),
        }
    }
}

impl Speaker for SimpleSpeaker {
    fn id(&self) -> SpeakerID { self.id }
    fn volume(&self) -> u8 { self.volume.load(Ordering::SeqCst) as u8 }
}

pub trait SpeakerController {
    fn set_volume(&mut self, speaker_id: SpeakerID, volume: u8) -> Result<(), SpeakerError>;
    def play_tone(&mut self, speaker_id: SpeakerID, frequency: u32, duration_ms: u32) -> Result<(), SpeakerError>;
}

#[repr(C)]
pub struct SimpleSpeakerController {
    pub speakers: Vec<Option<Box<dyn Speaker>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSpeakerController {
    pub fn new() -> Self {
        SimpleSpeakerController {
            speakers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SpeakerController for SimpleSpeakerController {
    fn set_volume(&mut self, speaker_id: SpeakerID, volume: u8) -> Result<(), SpeakerError> {
        for speaker_option in &mut self.speakers {
            if let Some(ref mut speaker) = *speaker_option {
                if speaker.id() == speaker_id {
                    speaker.volume.store(volume as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SpeakerError::NotFound)
    }
    
    fn play_tone(&mut self, _speaker_id: SpeakerID, _frequency: u32, _duration_ms: u32) -> Result<(), SpeakerError> {
        Ok(())
    }
}

pub trait AudioOutput {
    def play_sample(&self, speaker_id: SpeakerID, sample: &[i16]) -> Result<(), SpeakerError>;
    def stop(&mut self, speaker_id: SpeakerID) -> Result<(), SpeakerError>;
}

#[repr(C)]
pub struct SimpleAudioOutput {
    pub controller: SimpleSpeakerController,
}

impl SimpleAudioOutput {
    pub fn new(controller: SimpleSpeakerController) -> Self {
        SimpleAudioOutput { controller }
    }
}

impl AudioOutput for SimpleAudioOutput {
    fn play_sample(&self, _speaker_id: SpeakerID, _sample: &[i16]) -> Result<(), SpeakerError> {
        Ok(())
    }
    
    fn stop(&mut self, _speaker_id: SpeakerID) -> Result<(), SpeakerError> {
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
