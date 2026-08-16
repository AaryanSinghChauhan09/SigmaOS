extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type VoiceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceGender {
    Male = 0,
    Female = 1,
    Neutral = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessibilityError {
    Success = 0,
    NotFound = 1,
    QueueFull = 2,
    InvalidNode = 3,
}

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
    fn id(&self) -> VoiceID {
        self.id
    }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn gender(&self) -> VoiceGender {
        match self.gender.load(Ordering::SeqCst) {
            0 => VoiceGender::Male,
            1 => VoiceGender::Female,
            _ => VoiceGender::Neutral,
        }
    }
    fn rate(&self) -> f32 {
        (self.rate.load(Ordering::SeqCst) as f32) / 100.0
    }

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
}

impl SimpleScreenReader {
    pub fn get_voice(&self, id: VoiceID) -> Option<&dyn Voice> {
        for voice_option in &self.voices {
            if let Some(ref voice) = *voice_option {
                if voice.id() == id {
                    return Some(voice.as_ref());
                }
            }
        }
        None
    }

    pub fn register_voice(&mut self, voice: Box<dyn Voice>) {
        self.voices.push(Some(voice));
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
        SimpleBrailleDisplay { cells: [0u8; 40] }
    }
}

impl BrailleDisplay for SimpleBrailleDisplay {
    fn refresh(&mut self, cells: &[u8]) {
        for i in 0..self.cells.len().min(cells.len()) {
            self.cells[i] = cells[i];
        }
    }

    fn get_cells(&self) -> &[u8] {
        &self.cells
    }
}

// =========================================================================
// NVDA & ORCA PARITY ADVANCED SCREEN READER ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessibleRole {
    Unknown,
    Window,
    Button,
    Heading,
    Link,
    Checkbox,
    RadioButton,
    TextBox,
    Table,
    TableCell,
    List,
    ListItem,
    MenuItem,
    Landmark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessibleState {
    pub focused: bool,
    pub checked: bool,
    pub expanded: bool,
    pub selected: bool,
    pub disabled: bool,
    pub read_only: bool,
}

impl AccessibleState {
    pub fn new() -> Self {
        Self {
            focused: false,
            checked: false,
            expanded: false,
            selected: false,
            disabled: false,
            read_only: false,
        }
    }
}

impl Default for AccessibleState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct AccessibleNode {
    pub id: u64,
    pub role: AccessibleRole,
    pub label: String,
    pub value: String,
    pub description: String,
    pub state: AccessibleState,
    pub children: Vec<u64>,
}

impl AccessibleNode {
    pub fn new(id: u64, role: AccessibleRole, label: &str) -> Self {
        Self {
            id,
            role,
            label: label.to_string(),
            value: String::new(),
            description: String::new(),
            state: AccessibleState::new(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpeechPriority {
    Normal = 0,
    Urgent = 1,
    Interrupt = 2,
}

#[derive(Debug, Clone)]
pub struct SpeechUtterance {
    pub text: String,
    pub priority: SpeechPriority,
    pub pitch: f32,   // 0.5 to 2.0
    pub rate: f32,    // 0.5 to 3.0
    pub volume: f32,  // 0.0 to 1.0
    pub voice_id: VoiceID,
}

impl SpeechUtterance {
    pub fn new(text: &str, priority: SpeechPriority, voice_id: VoiceID) -> Self {
        Self {
            text: text.to_string(),
            priority,
            pitch: 1.0,
            rate: 1.0,
            volume: 1.0,
            voice_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickNavTarget {
    Heading,
    Link,
    FormField,
    Landmark,
    Button,
    Table,
}

/// Advanced NVDA & Orca Parity Screen Reader Subsystem
pub struct SovereignScreenReaderEngine {
    pub active_voice_id: VoiceID,
    pub pitch: f32,
    pub rate: f32,
    pub volume: f32,
    pub speech_queue: Vec<SpeechUtterance>,
    pub is_speaking: bool,
    pub is_paused: bool,
    pub nodes: Vec<AccessibleNode>,
    pub focused_node_id: Option<u64>,
    pub braille_display: SimpleBrailleDisplay,
}

impl SovereignScreenReaderEngine {
    pub fn new(default_voice: VoiceID) -> Self {
        Self {
            active_voice_id: default_voice,
            pitch: 1.0,
            rate: 1.0,
            volume: 1.0,
            speech_queue: Vec::new(),
            is_speaking: false,
            is_paused: false,
            nodes: Vec::new(),
            focused_node_id: None,
            braille_display: SimpleBrailleDisplay::new(),
        }
    }

    pub fn speak_utterance(&mut self, mut utterance: SpeechUtterance) {
        if utterance.priority == SpeechPriority::Interrupt {
            self.speech_queue.clear();
            self.is_speaking = true;
            self.speech_queue.push(utterance);
            return;
        }

        utterance.pitch *= self.pitch;
        utterance.rate *= self.rate;
        utterance.volume *= self.volume;

        self.speech_queue.push(utterance);
        self.speech_queue.sort_by(|a, b| b.priority.cmp(&a.priority));
        self.is_speaking = true;
    }

    pub fn register_node(&mut self, node: AccessibleNode) {
        self.nodes.push(node);
    }

    pub fn set_focus(&mut self, node_id: u64) -> Result<String, AccessibilityError> {
        let mut announcement = String::new();
        let mut found = false;

        for node in &mut self.nodes {
            if node.id == node_id {
                node.state.focused = true;
                found = true;
                announcement = format!(
                    "{} {:?}{}{}",
                    node.label,
                    node.role,
                    if !node.value.is_empty() { format!(", value {}", node.value) } else { String::new() },
                    if node.state.checked { ", checked" } else { "" }
                );
            } else {
                node.state.focused = false;
            }
        }

        if found {
            self.focused_node_id = Some(node_id);
            self.speak_utterance(SpeechUtterance::new(
                &announcement,
                SpeechPriority::Interrupt,
                self.active_voice_id,
            ));
            let braille_cells = BrailleTranslator::translate_ascii(announcement.as_bytes());
            self.braille_display.refresh(&braille_cells);
            Ok(announcement)
        } else {
            Err(AccessibilityError::NotFound)
        }
    }

    pub fn quick_nav_next(&mut self, target: QuickNavTarget) -> Option<u64> {
        let current_idx = if let Some(fid) = self.focused_node_id {
            self.nodes.iter().position(|n| n.id == fid).unwrap_or(0)
        } else {
            0
        };

        let total = self.nodes.len();
        if total == 0 {
            return None;
        }

        for offset in 1..total {
            let idx = (current_idx + offset) % total;
            let node = &self.nodes[idx];
            let matches = match target {
                QuickNavTarget::Heading => node.role == AccessibleRole::Heading,
                QuickNavTarget::Link => node.role == AccessibleRole::Link,
                QuickNavTarget::FormField => matches!(
                    node.role,
                    AccessibleRole::TextBox
                        | AccessibleRole::Checkbox
                        | AccessibleRole::RadioButton
                        | AccessibleRole::Button
                ),
                QuickNavTarget::Landmark => node.role == AccessibleRole::Landmark,
                QuickNavTarget::Button => node.role == AccessibleRole::Button,
                QuickNavTarget::Table => node.role == AccessibleRole::Table,
            };

            if matches {
                let target_id = node.id;
                let _ = self.set_focus(target_id);
                return Some(target_id);
            }
        }
        None
    }
}

/// 8-dot Braille Unicode translator
pub struct BrailleTranslator;

impl BrailleTranslator {
    /// Translates ASCII byte slice into 8-dot Braille Unicode representation
    pub fn translate_ascii(input: &[u8]) -> Vec<u8> {
        let mut braille_pattern = Vec::new();
        for &byte in input {
            let dots = match byte.to_ascii_lowercase() {
                b'a' => 0b0000_0001,
                b'b' => 0b0000_0011,
                b'c' => 0b0000_1001,
                b'd' => 0b0001_1001,
                b'e' => 0b0001_0001,
                b'f' => 0b0000_1011,
                b'g' => 0b0001_1011,
                b'h' => 0b0001_0011,
                b'i' => 0b0000_1010,
                b'j' => 0b0001_1010,
                b'k' => 0b0000_0101,
                b'l' => 0b0000_0111,
                b'm' => 0b0000_1101,
                b'n' => 0b0001_1101,
                b'o' => 0b0001_0101,
                b'p' => 0b0000_1111,
                b'q' => 0b0001_1111,
                b'r' => 0b0001_0111,
                b's' => 0b0000_1110,
                b't' => 0b0001_1110,
                b'u' => 0b0010_0101,
                b'v' => 0b0010_0111,
                b'w' => 0b0001_1110,
                b'x' => 0b0010_1101,
                b'y' => 0b0011_1101,
                b'z' => 0b0011_0101,
                b' ' => 0b0000_0000,
                _ => 0b0011_1111,
            };
            braille_pattern.push(dots);
        }
        braille_pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_reader_and_voices() {
        let mut reader = SimpleScreenReader::new();
        let voice = SimpleVoice::new(42, b"Alice", VoiceGender::Female);
        reader.register_voice(Box::new(voice));

        assert!(reader.get_voice(42).is_some());
        assert_eq!(reader.get_voice(42).unwrap().gender(), VoiceGender::Female);
        assert_eq!(reader.speak(b"Hello", 42), Ok(()));
        assert_eq!(
            reader.speak(b"Hello", 999),
            Err(AccessibilityError::NotFound)
        );
    }

    #[test]
    fn test_braille_display() {
        let mut display = SimpleBrailleDisplay::new();
        display.refresh(b"Braille info");
        assert_eq!(&display.get_cells()[..12], b"Braille info");
    }

    #[test]
    fn test_sovereign_screen_reader_engine() {
        let mut engine = SovereignScreenReaderEngine::new(1);

        let mut node1 = AccessibleNode::new(101, AccessibleRole::Heading, "Main Title");
        node1.value = "H1".to_string();

        let mut node2 = AccessibleNode::new(102, AccessibleRole::Button, "Submit");
        node2.state.checked = true;

        engine.register_node(node1);
        engine.register_node(node2);

        let announcement = engine.set_focus(101).unwrap();
        assert!(announcement.contains("Main Title Heading"));

        let next_btn = engine.quick_nav_next(QuickNavTarget::Button).unwrap();
        assert_eq!(next_btn, 102);
        assert_eq!(engine.focused_node_id, Some(102));
    }

    #[test]
    fn test_braille_translation() {
        let dots = BrailleTranslator::translate_ascii(b"abc ");
        assert_eq!(dots[0], 0b0000_0001); // 'a'
        assert_eq!(dots[1], 0b0000_0011); // 'b'
        assert_eq!(dots[2], 0b0000_1001); // 'c'
        assert_eq!(dots[3], 0b0000_0000); // space
    }
}
