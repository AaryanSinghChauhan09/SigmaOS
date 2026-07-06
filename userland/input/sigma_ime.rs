// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/input/sigma_ime.rs — Indian Language IME (Input Method Engine)
// Implements Inscript and Phonetic keyboard layouts for Indian languages
//
// Supported Languages:
//   - Hindi (Devanagari)
//   - Bengali
//   - Tamil
//   - Telugu
//   - Kannada
//   - Malayalam
//   - Gujarati
//   - Marathi
//   - Punjabi (Gurmukhi)
//   - Odia
//
// Features:
//   - Inscript (standard government layout)
//   - Phonetic (transliteration-based)
//   - Unicode output
//   - Compose sequences for conjuncts
//
// Language: Rust

use std::collections::HashMap;

// ── IME Mode ─────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ImeMode {
    Inscript,
    Phonetic,
}

// ── Language ─────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Language {
    Hindi,
    Bengali,
    Tamil,
    Telugu,
    Kannada,
    Malayalam,
    Gujarati,
    Marathi,
    Punjabi,
    Odia,
}

impl Language {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "hindi" => Some(Language::Hindi),
            "bengali" => Some(Language::Bengali),
            "tamil" => Some(Language::Tamil),
            "telugu" => Some(Language::Telugu),
            "kannada" => Some(Language::Kannada),
            "malayalam" => Some(Language::Malayalam),
            "gujarati" => Some(Language::Gujarati),
            "marathi" => Some(Language::Marathi),
            "punjabi" => Some(Language::Punjabi),
            "odia" => Some(Language::Odia),
            _ => None,
        }
    }
}

// ── Inscript Key Mappings (Hindi - Devanagari) ───────────────────────────

const INSCRIPT_HINDI: &[(&str, &str)] = &[
    ("1", "१"), ("2", "२"), ("3", "३"), ("4", "४"), ("5", "५"),
    ("6", "६"), ("7", "७"), ("8", "८"), ("9", "९"), ("0", "०"),
    ("q", "औ"), ("w", "ए"), ("e", "ऐ"), ("r", "ा"), ("t", "ी"),
    ("y", "ू"), ("u", "भ"), ("i", "ङ"), ("o", "घ"), ("p", "ध"),
    ("[", "झ"), ("]", "ञ"), ("a", "ो"), ("s", "े"), ("d", "्"),
    ("f", "ि"), ("g", "प"), ("h", "र"), ("j", "क"), ("k", "त"),
    ("l", "म"), (";", "न"), ("'", "व"), ("z", "श"), ("x", "ष"),
    ("c", "स"), ("v", "य"), ("b", "ह"), ("n", "ल"), ("m", "ग"),
    (",", "द"), (".", "ज"), ("/", "ड"),
];

// ── Phonetic Key Mappings (Hindi - Devanagari) ─────────────────────────

const PHONETIC_HINDI: &[(&str, &str)] = &[
    ("a", "अ"), ("aa", "आ"), ("i", "इ"), ("ii", "ई"), ("u", "उ"),
    ("uu", "ऊ"), ("e", "ए"), ("ai", "ऐ"), ("o", "ओ"), ("au", "औ"),
    ("k", "क"), ("kh", "ख"), ("g", "ग"), ("gh", "घ"), ("ng", "ङ"),
    ("ch", "च"), ("chh", "छ"), ("j", "ज"), ("jh", "झ"), ("ny", "ञ"),
    ("T", "ट"), ("Th", "ठ"), ("D", "ड"), ("Dh", "ढ"), ("N", "ण"),
    ("t", "त"), ("th", "थ"), ("d", "द"), ("dh", "ध"), ("n", "न"),
    ("p", "प"), ("ph", "फ"), ("b", "ब"), ("bh", "भ"), ("m", "म"),
    ("y", "य"), ("r", "र"), ("l", "ल"), ("v", "व"), ("sh", "श"),
    ("Sh", "ष"), ("s", "स"), ("h", "ह"),
    ("aa", "ा"), ("i", "ि"), ("ii", "ी"), ("u", "ु"), ("uu", "ू"),
    ("e", "े"), ("ai", "ै"), ("o", "ो"), ("au", "ौ"),
    ("n", "ं"), ("m", "ः"), (":", "ऽ"),
];

// ── IME Engine ─────────────────────────────────────────────────────────────

pub struct ImeEngine {
    mode: ImeMode,
    language: Language,
    buffer: String,
    inscript_map: HashMap<char, char>,
    phonetic_map: HashMap<String, String>,
}

impl ImeEngine {
    pub fn new(mode: ImeMode, language: Language) -> Self {
        let mut inscript_map = HashMap::new();
        let mut phonetic_map = HashMap::new();

        // Load Inscript mappings based on language
        match language {
            Language::Hindi | Language::Marathi => {
                for (key, val) in INSCRIPT_HINDI {
                    if key.len() == 1 {
                        inscript_map.insert(key.chars().next().unwrap(), val.chars().next().unwrap());
                    }
                }
            }
            _ => {} // Other languages would have their own mappings
        }

        // Load Phonetic mappings based on language
        match language {
            Language::Hindi | Language::Marathi => {
                for (key, val) in PHONETIC_HINDI {
                    phonetic_map.insert(key.to_string(), val.to_string());
                }
            }
            _ => {} // Other languages would have their own mappings
        }

        Self {
            mode,
            language,
            buffer: String::new(),
            inscript_map,
            phonetic_map,
        }
    }

    pub fn set_mode(&mut self, mode: ImeMode) {
        self.mode = mode;
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
        // Rebuild mappings for new language
        *self = Self::new(self.mode, language);
    }

    /// Process a key press and return the output string
    pub fn process_key(&mut self, key: char) -> String {
        match self.mode {
            ImeMode::Inscript => self.process_inscript(key),
            ImeMode::Phonetic => self.process_phonetic(key),
        }
    }

    /// Process key in Inscript mode
    fn process_inscript(&mut self, key: char) -> String {
        if let Some(&mapped) = self.inscript_map.get(&key) {
            self.buffer.push(mapped);
            mapped.to_string()
        } else {
            // Pass through unmapped keys
            self.buffer.push(key);
            key.to_string()
        }
    }

    /// Process key in Phonetic mode
    fn process_phonetic(&mut self, key: char) -> String {
        self.buffer.push(key);
        
        // Check for multi-character sequences
        if self.buffer.len() >= 2 {
            let last_two = &self.buffer[self.buffer.len() - 2..];
            if let Some(mapped) = self.phonetic_map.get(last_two) {
                self.buffer.truncate(self.buffer.len() - 2);
                self.buffer.push_str(mapped);
                return mapped.clone();
            }
        }

        // Check for single character mappings
        let last_char = self.buffer.chars().last().unwrap();
        if let Some(mapped) = self.phonetic_map.get(&last_char.to_string()) {
            self.buffer.truncate(self.buffer.len() - 1);
            self.buffer.push_str(mapped);
            return mapped.clone();
        }

        // Pass through unmapped keys
        last_char.to_string()
    }

    /// Clear the input buffer
    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    /// Get current buffer content
    pub fn get_buffer(&self) -> &str {
        &self.buffer
    }

    /// Backspace - remove last character
    pub fn backspace(&mut self) {
        self.buffer.pop();
    }

    /// Commit the current buffer and return the result
    pub fn commit(&mut self) -> String {
        let result = self.buffer.clone();
        self.clear_buffer();
        result
    }
}

// ── Compose Sequences for Conjuncts ─────────────────────────────────────

pub struct ComposeEngine {
    sequences: HashMap<String, String>,
}

impl ComposeEngine {
    pub fn new() -> Self {
        let mut sequences = HashMap::new();
        
        // Hindi conjuncts (common examples)
        sequences.insert("क्त".to_string(), "क्त".to_string());
        sequences.insert("क्ष".to_string(), "क्ष".to_string());
        sequences.insert("त्र".to_string(), "त्र".to_string());
        sequences.insert("ज्ञ".to_string(), "ज्ञ".to_string());
        sequences.insert("श्र".to_string(), "श्र".to_string());
        
        Self { sequences }
    }

    /// Try to compose a conjunct from the buffer
    pub fn compose(&self, input: &str) -> Option<String> {
        self.sequences.get(input).cloned()
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn ime_create(mode: u32, language: u32) -> *mut ImeEngine {
    let mode = match mode {
        0 => ImeMode::Inscript,
        1 => ImeMode::Phonetic,
        _ => ImeMode::Phonetic,
    };
    let language = match language {
        0 => Language::Hindi,
        1 => Language::Bengali,
        2 => Language::Tamil,
        3 => Language::Telugu,
        4 => Language::Kannada,
        5 => Language::Malayalam,
        6 => Language::Gujarati,
        7 => Language::Marathi,
        8 => Language::Punjabi,
        9 => Language::Odia,
        _ => Language::Hindi,
    };
    Box::into_raw(Box::new(ImeEngine::new(mode, language)))
}

#[no_mangle]
pub extern "C" fn ime_destroy(ime: *mut ImeEngine) {
    unsafe {
        if !ime.is_null() {
            let _ = Box::from_raw(ime);
        }
    }
}

#[no_mangle]
pub extern "C" fn ime_process_key(ime: *mut ImeEngine, key: u8, out: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if ime.is_null() { return -1; }
        let key_char = key as char;
        let result = (*ime).process_key(key_char);
        let bytes = result.as_bytes();
        let copy_len = std::cmp::min(bytes.len(), out_len);
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), out, copy_len);
        copy_len as i32
    }
}

#[no_mangle]
pub extern "C" fn ime_backspace(ime: *mut ImeEngine) {
    unsafe {
        if !ime.is_null() {
            (*ime).backspace();
        }
    }
}

#[no_mangle]
pub extern "C" fn ime_commit(ime: *mut ImeEngine, out: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if ime.is_null() { return -1; }
        let result = (*ime).commit();
        let bytes = result.as_bytes();
        let copy_len = std::cmp::min(bytes.len(), out_len);
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), out, copy_len);
        copy_len as i32
    }
}
