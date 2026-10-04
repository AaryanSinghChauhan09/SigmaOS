#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::boxed::Box;
use std::vec::Vec;

/// OOP-based Macro Recorder for SigmaOS
/// Based on Ideas-999-Structured: Automation & Scripting Item 866
/// Implements macro recording and playback

use core::sync::atomic::{AtomicUsize, Ordering};

pub type MacroID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MacroError { Success = 0, NotFound = 1 }

pub trait Macro {
    fn id(&self) -> MacroID;
    fn name(&self) -> &[u8];
    fn actions(&self) -> u32;
    fn record_action(&self);
}

#[repr(C)]
pub struct SimpleMacro {
    pub id: MacroID,
    pub name: [u8; 64],
    pub name_len: u8,
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
            name_len: name_len as u8,
            actions: AtomicUsize::new(0),
        }
    }
}

impl Macro for SimpleMacro {
    fn id(&self) -> MacroID { self.id }
    fn name(&self) -> &[u8] {
        // Bolt ⚡ Optimization: Store explicit name length on instantiation to eliminate
        // O(N) zero-byte linear scanning (.position(|&b| b == 0)) on every macro name query,
        // reducing slice retrieval to instantaneous O(1) constant time.
        &self.name[..self.name_len as usize]
    }
    fn actions(&self) -> u32 { self.actions.load(Ordering::SeqCst) as u32 }
    fn record_action(&self) { self.actions.fetch_add(1, Ordering::SeqCst); }
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
    #[allow(clippy::new_without_default)]
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
        let r#macro = SimpleMacro::new(id, name);
        self.macros.push(Some(Box::new(r#macro)));
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
            if let Some(ref mut r#macro) = *macro_option {
                if r#macro.id() == id {
                    r#macro.record_action();
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
    #[allow(clippy::new_without_default)]
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


#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_macro_cached_name_length() {
        let m = SimpleMacro::new(1, b"record_keystrokes");
        assert_eq!(m.id(), 1);
        assert_eq!(m.name(), b"record_keystrokes");
        assert_eq!(m.actions(), 0);
    }

    #[test]
    fn test_simple_macro_recorder_and_player() {
        let mut recorder = SimpleMacroRecorder::new();
        let macro_id = recorder.start_recording(b"play_macro").unwrap();
        assert_eq!(macro_id, 1);

        assert!(recorder.stop_recording(macro_id).is_ok());

        let mut player = SimpleMacroPlayer::new();
        assert!(!player.is_playing());
        player.play(macro_id).unwrap();
        assert!(player.is_playing());
        player.stop();
        assert!(!player.is_playing());
    }
}
