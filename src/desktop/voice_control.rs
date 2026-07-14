#![no_std]

/// Voice-Controlled Desktop Actions for SigmaOS
/// Based on 100-Improvement-Ideas.md #48: Voice-controlled desktop actions
/// Implements voice command recognition and execution

use core::sync::atomic::{AtomicU64, Ordering};

/// Voice command type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceCommandType {
    OpenApp = 0,
    CloseApp = 1,
    MinimizeWindow = 2,
    MaximizeWindow = 3,
    SwitchWorkspace = 4,
    ShowDesktop = 5,
    LaunchWebsite = 6,
    Search = 7,
    SystemCommand = 8,
}

/// Voice command
#[repr(C)]
pub struct VoiceCommand {
    pub id: u64,
    pub command_type: VoiceCommandType,
    pub phrase: [u8; 128],
    pub action: [u8; 256],
    pub confidence: f32,
}

impl VoiceCommand {
    pub fn new(id: u64, command_type: VoiceCommandType, phrase: &str, action: &str) -> Self {
        let mut phrase_array = [0u8; 128];
        let phrase_bytes = phrase.as_bytes();
        let phrase_len = phrase_bytes.len().min(127);
        
        unsafe {
            core::ptr::copy_nonoverlapping(phrase_bytes.as_ptr(), phrase_array.as_mut_ptr(), phrase_len);
        }
        
        let mut action_array = [0u8; 256];
        let action_bytes = action.as_bytes();
        let action_len = action_bytes.len().min(255);
        
        unsafe {
            core::ptr::copy_nonoverlapping(action_bytes.as_ptr(), action_array.as_mut_ptr(), action_len);
        }
        
        VoiceCommand {
            id,
            command_type,
            phrase: phrase_array,
            action: action_array,
            confidence: 0.0,
        }
    }
    
    pub fn phrase_str(&self) -> &str {
        unsafe {
            let len = self.phrase.iter().position(|&b| b == 0).unwrap_or(128);
            core::str::from_utf8_unchecked(&self.phrase[..len])
        }
    }
    
    pub fn action_str(&self) -> &str {
        unsafe {
            let len = self.action.iter().position(|&b| b == 0).unwrap_or(256);
            core::str::from_utf8_unchecked(&self.action[..len])
        }
    }
}

/// Voice recognition result
#[repr(C)]
pub struct VoiceRecognitionResult {
    pub recognized_phrase: [u8; 128],
    pub confidence: f32,
    pub matched_command: Option<VoiceCommand>,
}

impl VoiceRecognitionResult {
    pub fn new() -> Self {
        VoiceRecognitionResult {
            recognized_phrase: [0u8; 128],
            confidence: 0.0,
            matched_command: None,
        }
    }
}

/// Voice control engine
pub struct VoiceControlEngine {
    pub commands: Vec<Option<VoiceCommand>>,
    pub next_command_id: AtomicU64,
    pub listening: bool,
}

impl VoiceControlEngine {
    pub fn new() -> Self {
        VoiceControlEngine {
            commands: Vec::new(),
            next_command_id: AtomicU64::new(1),
            listening: false,
        }
    }
    
    /// Add voice command
    pub fn add_command(&mut self, command_type: VoiceCommandType, phrase: &str, action: &str) -> u64 {
        let id = self.next_command_id.fetch_add(1, Ordering::SeqCst);
        let command = VoiceCommand::new(id, command_type, phrase, action);
        self.commands.push(Some(command));
        id
    }
    
    /// Remove voice command
    pub fn remove_command(&mut self, id: u64) -> bool {
        for command_option in &mut self.commands {
            if let Some(ref command) = *command_option {
                if command.id == id {
                    *command_option = None;
                    return true;
                }
            }
        }
        false
    }
    
    /// Start listening
    pub fn start_listening(&mut self) {
        self.listening = true;
    }
    
    /// Stop listening
    pub fn stop_listening(&mut self) {
        self.listening = false;
    }
    
    /// Process voice input
    pub fn process_voice_input(&self, input: &str) -> VoiceRecognitionResult {
        let mut result = VoiceRecognitionResult::new();
        
        // Store recognized phrase
        let input_bytes = input.as_bytes();
        let len = input_bytes.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(input_bytes.as_ptr(), result.recognized_phrase.as_mut_ptr(), len);
        }
        
        // Match against commands
        let input_lower = to_lowercase(input);
        
        for command_option in &self.commands {
            if let Some(ref command) = *command_option {
                let command_phrase = to_lowercase(command.phrase_str());
                
                // Simple phrase matching
                if input_lower.contains(&command_phrase) {
                    result.confidence = 0.9;
                    result.matched_command = Some(*command);
                    break;
                }
            }
        }
        
        result
    }
    
    /// Execute matched command
    pub fn execute_command(&self, result: &VoiceRecognitionResult) -> bool {
        if let Some(ref command) = result.matched_command {
            // In real implementation, execute the action
            true
        } else {
            false
        }
    }
    
    /// Initialize default commands
    pub fn initialize_defaults(&mut self) {
        self.add_command(VoiceCommandType::OpenApp, "open", "launch_app");
        self.add_command(VoiceCommandType::CloseApp, "close", "close_app");
        self.add_command(VoiceCommandType::MinimizeWindow, "minimize", "minimize_window");
        self.add_command(VoiceCommandType::MaximizeWindow, "maximize", "maximize_window");
        self.add_command(VoiceCommandType::SwitchWorkspace, "switch workspace", "switch_workspace");
        self.add_command(VoiceCommandType::ShowDesktop, "show desktop", "show_desktop");
        self.add_command(VoiceCommandType::LaunchWebsite, "open website", "launch_website");
        self.add_command(VoiceCommandType::Search, "search", "search");
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

/// Simple string utilities
fn to_lowercase(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if c >= 'A' && c <= 'Z' {
            result.push((c as u8 + 32) as char);
        } else {
            result.push(c);
        }
    }
    result
}

/// Simple String implementation for no_std
struct String {
    data: Vec<u8>,
}

impl String {
    fn new() -> Self {
        String {
            data: Vec::new(),
        }
    }
    
    fn push(&mut self, c: char) {
        self.data.push(c as u8);
    }
    
    fn contains(&self, pattern: &String) -> bool {
        if self.data.len() < pattern.data.len() {
            return false;
        }
        
        for i in 0..=(self.data.len() - pattern.data.len()) {
            let mut match_found = true;
            for j in 0..pattern.data.len() {
                if self.data[i + j] != pattern.data[j] {
                    match_found = false;
                    break;
                }
            }
            if match_found {
                return true;
            }
        }
        
        false
    }
}
