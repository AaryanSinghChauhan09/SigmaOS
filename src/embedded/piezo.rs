#![no_std]
#![no_main]

/// OOP-based Piezo for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1636
/// Implements piezo buzzer

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PiezoID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PiezoError { Success = 0, NotFound = 1 }

pub trait Piezo {
    fn id(&self) -> PiezoID;
    fn frequency(&self) -> u32;
}

#[repr(C)]
pub struct SimplePiezo {
    pub id: PiezoID,
    pub frequency: AtomicUsize,
}

impl SimplePiezo {
    pub fn new(id: PiezoID) -> Self {
        SimplePiezo {
            id,
            frequency: AtomicUsize::new(0),
        }
    }
}

impl Piezo for SimplePiezo {
    fn id(&self) -> PiezoID { self.id }
    fn frequency(&self) -> u32 { self.frequency.load(Ordering::SeqCst) as u32 }
}

pub trait PiezoController {
    fn set_frequency(&mut self, piezo_id: PiezoID, frequency: u32) -> Result<(), PiezoError>;
    def play_note(&mut self, piezo_id: PiezoID, note: u8, duration_ms: u32) -> Result<(), PiezoError>;
}

#[repr(C)]
pub struct SimplePiezoController {
    pub piezos: Vec<Option<Box<dyn Piezo>>>,
    pub next_id: AtomicUsize,
}

impl SimplePiezoController {
    pub fn new() -> Self {
        SimplePiezoController {
            piezos: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PiezoController for SimplePiezoController {
    fn set_frequency(&mut self, piezo_id: PiezoID, frequency: u32) -> Result<(), PiezoError> {
        for piezo_option in &mut self.piezos {
            if let Some(ref mut piezo) = *piezo_option {
                if piezo.id() == piezo_id {
                    piezo.frequency.store(frequency as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PiezoError::NotFound)
    }
    
    fn play_note(&mut self, piezo_id: PiezoID, note: u8, _duration_ms: u32) -> Result<(), PiezoError> {
        const NOTE_FREQS: [u32; 12] = [262, 277, 294, 311, 330, 349, 370, 392, 415, 440, 466, 494];
        let freq = if note < 12 { NOTE_FREQS[note as usize] } else { 440 };
        self.set_frequency(piezo_id, freq)
    }
}

pub trait MelodyPlayer {
    def play_melody(&mut self, piezo_id: PiezoID, notes: &[u8], durations: &[u32]) -> Result<(), PiezoError>;
    def stop(&mut self, piezo_id: PiezoID) -> Result<(), PiezoError>;
}

#[repr(C)]
pub struct SimpleMelodyPlayer {
    pub controller: SimplePiezoController,
    pub playing: AtomicUsize,
}

impl SimpleMelodyPlayer {
    pub fn new(controller: SimplePiezoController) -> Self {
        SimpleMelodyPlayer {
            controller,
            playing: AtomicUsize::new(0),
        }
    }
}

impl MelodyPlayer for SimpleMelodyPlayer {
    fn play_melody(&mut self, piezo_id: PiezoID, notes: &[u8], durations: &[u32]) -> Result<(), PiezoError> {
        self.playing.store(1, Ordering::SeqCst);
        for (i, &note) in notes.iter().enumerate() {
            if i < durations.len() {
                self.controller.play_note(piezo_id, note, durations[i])?;
            }
        }
        self.playing.store(0, Ordering::SeqCst);
        Ok(())
    }
    
    fn stop(&mut self, _piezo_id: PiezoID) -> Result<(), PiezoError> {
        self.playing.store(0, Ordering::SeqCst);
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
