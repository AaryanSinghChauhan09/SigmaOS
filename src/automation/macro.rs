#![no_std]
#![no_main]

/// OOP-based Macro Recorder for SigmaOS
/// Based on Ideas-999-Structured: Automation & Scripting Item 866
/// Implements macro recording and playback

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MacroID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MacroError { Success = 0, NotFound = 1 }

pub trait Macro {
    fn id(&self) -> MacroID;
    fn name(&self) -> &[u8];
    fn actions(&self) -> u32;
}

#[repr(C)]
pub struct SimpleMacro {
    pub id: MacroID,
    pub name: [u8; 64],
    pub actions: AtomicUsize,
}

impl SimpleMacro {
    pub fn new(id: MacroID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleMacro {
            id,
            name: name_array,
            actions: AtomicUsize::new(0),
        }
    }
}

impl Macro for SimpleMacro {
    fn id(&self) -> MacroID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn actions(&self) -> u32 { self.actions.load(Ordering::SeqCst) as u32 }
}

pub trait MacroRecorder {
    fn start_recording(&mut self, name: &[u8]) -> Result<MacroID, MacroError>;
    fn stop_recording(&mut self, id: MacroID) -> Result<(), MacroError>;
    fn record_action(&mut self, id: MacroID, action: u32) -> Result<(), MacroError>;
}

#[repr(C)]
pub struct SimpleMacroRecorder {
    pub macros: Vec<Option<Box<dyn Macro>>>,
    pub recording: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleMacroRecorder {
    pub fn new() -> Self {
        SimpleMacroRecorder {
            macros: Vec::new(),
            recording: AtomicUsize::new(0),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MacroRecorder for SimpleMacroRecorder {
    fn start_recording(&mut self, name: &[u8]) -> Result<MacroID, MacroError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let macro = SimpleMacro::new(id, name);
        self.macros.push(Some(Box::new(macro)));
        self.recording.store(id, Ordering::SeqCst);
        Ok(id)
    }
    
    fn stop_recording(&mut self, id: MacroID) -> Result<(), MacroError> {
        if self.recording.load(Ordering::SeqCst) == id {
            self.recording.store(0, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MacroError::NotFound)
        }
    }
    
    fn record_action(&mut self, id: MacroID, _action: u32) -> Result<(), MacroError> {
        for macro_option in &mut self.macros {
            if let Some(ref mut macro) = *macro_option {
                if macro.id() == id {
                    macro.actions.fetch_add(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MacroError::NotFound)
    }
}

pub trait MacroPlayer {
    fn play(&self, id: MacroID) -> Result<(), MacroError>;
    fn stop(&mut self);
    fn is_playing(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMacroPlayer {
    pub playing: AtomicUsize,
}

impl SimpleMacroPlayer {
    pub fn new() -> Self {
        SimpleMacroPlayer {
            playing: AtomicUsize::new(0),
        }
    }
}

impl MacroPlayer for SimpleMacroPlayer {
    fn play(&self, _id: MacroID) -> Result<(), MacroError> {
        self.playing.store(1, Ordering::SeqCst);
        Ok(())
    }
    
    fn stop(&mut self) {
        self.playing.store(0, Ordering::SeqCst);
    }
    
    fn is_playing(&self) -> bool { self.playing.load(Ordering::SeqCst) == 1 }
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
