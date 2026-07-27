#![no_std]
#![no_main]

/// OOP-based Screen Reader for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 816
/// Implements text-to-speech and accessibility

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type VoiceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum VoiceGender { Male = 0, Female = 1, Neutral = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AccessibilityError { Success = 0, NotFound = 1 }

pub trait Voice {
    fn id(&self) -> VoiceID;
    fn name(&self) -> &[u8];
    fn gender(&self) -> VoiceGender;
    fn rate(&self) -> f32;
    fn set_rate(&mut self, rate: f32);
}

#[repr(C)]
pub struct SimpleVoice {
    pub id: VoiceID,
    pub name: [u8; 64],
    pub gender: AtomicUsize,
    pub rate: AtomicUsize,
}

impl SimpleVoice {
    pub fn new(id: VoiceID, name: &[u8], gender: VoiceGender) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleVoice {
            id,
            name: name_array,
            gender: AtomicUsize::new(gender as usize),
            rate: AtomicUsize::new(100),
        }
    }
}

impl Voice for SimpleVoice {
    fn id(&self) -> VoiceID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn gender(&self) -> VoiceGender { unsafe { core::mem::transmute(self.gender.load(Ordering::SeqCst)) } }
    fn rate(&self) -> f32 { (self.rate.load(Ordering::SeqCst) as f32) / 100.0 }
    
    fn set_rate(&mut self, rate: f32) {
        self.rate.store((rate * 100.0) as usize, Ordering::SeqCst);
    }
}

pub trait ScreenReader {
    fn speak(&self, text: &[u8], voice_id: VoiceID) -> Result<(), AccessibilityError>;
    fn stop(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
}

#[repr(C)]
pub struct SimpleScreenReader {
    pub voices: Vec<Option<Box<dyn Voice>>>,
    pub speaking: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleScreenReader {
    pub fn new() -> Self {
        SimpleScreenReader {
            voices: Vec::new(),
            speaking: AtomicUsize::new(0),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ScreenReader for SimpleScreenReader {
    fn speak(&self, _text: &[u8], voice_id: VoiceID) -> Result<(), AccessibilityError> {
        if self.get_voice(voice_id).is_some() {
            Ok(())
        } else {
            Err(AccessibilityError::NotFound)
        }
    }
    
    fn stop(&mut self) {
        self.speaking.store(0, Ordering::SeqCst);
    }
    
    fn pause(&mut self) {
        self.speaking.store(2, Ordering::SeqCst);
    }
    
    fn resume(&mut self) {
        self.speaking.store(1, Ordering::SeqCst);
    }
    
    fn get_voice(&self, id: VoiceID) -> Option<&dyn Voice> {
        for voice_option in &self.voices {
            if let Some(ref voice) = *voice_option {
                if voice.id() == id { return Some(voice.as_ref()); }
            }
        }
        None
    }
}

pub trait BrailleDisplay {
    fn refresh(&mut self, cells: &[u8]);
    fn get_cells(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleBrailleDisplay {
    pub cells: [u8; 40],
}

impl SimpleBrailleDisplay {
    pub fn new() -> Self {
        SimpleBrailleDisplay {
            cells: [0u8; 40],
        }
    }
}

impl BrailleDisplay for SimpleBrailleDisplay {
    fn refresh(&mut self, cells: &[u8]) {
        for i in 0..self.cells.len().min(cells.len()) {
            self.cells[i] = cells[i];
        }
    }
    
    fn get_cells(&self) -> &[u8] { &self.cells }
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
