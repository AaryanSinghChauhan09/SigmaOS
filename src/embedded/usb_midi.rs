#![no_std]
#![no_main]

/// OOP-based USB MIDI for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2266
/// Implements USB MIDI

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MIDIID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MIDIError { Success = 0, NotFound = 1 }

pub trait USBMIDI {
    fn id(&self) -> MIDIID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleUSBMIDI {
    pub id: MIDIID,
    pub connected: AtomicUsize,
}

impl SimpleUSBMIDI {
    pub fn new(id: MIDIID) -> Self {
        SimpleUSBMIDI {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl USBMIDI for SimpleUSBMIDI {
    fn id(&self) -> MIDIID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait MIDIController {
    fn init(&mut self, midi_id: MIDIID) -> Result<(), MIDIError>;
    fn send_message(&self, midi_id: MIDIID, message: &[u8]) -> Result<(), MIDIError>;
    def receive_message(&self, midi_id: MIDIID, buffer: &mut [u8]) -> Result<usize, MIDIError>;
}

#[repr(C)]
pub struct SimpleMIDIController {
    pub midi_devices: Vec<Option<Box<dyn USBMIDI>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMIDIController {
    pub fn new() -> Self {
        SimpleMIDIController {
            midi_devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MIDIController for SimpleMIDIController {
    fn init(&mut self, midi_id: MIDIID) -> Result<(), MIDIError> {
        for midi_option in &mut self.midi_devices {
            if let Some(ref mut midi) = *midi_option {
                if midi.id() == midi_id {
                    midi.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MIDIError::NotFound)
    }
    
    fn send_message(&self, midi_id: MIDIID, _message: &[u8]) -> Result<(), MIDIError> {
        if self.get_midi(midi_id).is_some() {
            Ok(())
        } else {
            Err(MIDIError::NotFound)
        }
    }
    
    fn receive_message(&self, midi_id: MIDIID, buffer: &mut [u8]) -> Result<usize, MIDIError> {
        if self.get_midi(midi_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(MIDIError::NotFound)
        }
    }
    
    fn get_midi(&self, id: MIDIID) -> Option<&dyn USBMIDI> {
        for midi_option in &self.midi_devices {
            if let Some(ref midi) = *midi_option {
                if midi.id() == id { return Some(midi.as_ref()); }
            }
        }
        None
    }
}

pub trait MIDINote {
    def send_note(&self, midi_id: MIDIID, note: u8, velocity: u8) -> Result<(), MIDIError>;
    def send_cc(&self, midi_id: MIDIID, cc: u8, value: u8) -> Result<(), MIDIError>;
}

#[repr(C)]
pub struct SimpleMIDINote {
    pub controller: SimpleMIDIController,
}

impl SimpleMIDINote {
    pub fn new(controller: SimpleMIDIController) -> Self {
        SimpleMIDINote { controller }
    }
}

impl MIDINote for SimpleMIDINote {
    fn send_note(&self, midi_id: MIDIID, _note: u8, _velocity: u8) -> Result<(), MIDIError> {
        if self.controller.get_midi(midi_id).is_some() {
            Ok(())
        } else {
            Err(MIDIError::NotFound)
        }
    }
    
    fn send_cc(&self, midi_id: MIDIID, _cc: u8, _value: u8) -> Result<(), MIDIError> {
        if self.controller.get_midi(midi_id).is_some() {
            Ok(())
        } else {
            Err(MIDIError::NotFound)
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
