#![no_std]

/// Natural Language Command Shell for SigmaOS
/// Implements natural language to system command translation
/// Based on 100-Improvement-Ideas.md #55: Natural language command shell

use core::sync::atomic::{AtomicU64, Ordering};

/// Command intent types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandIntent {
    InstallPackage = 0,
    RemovePackage = 1,
    UpdateSystem = 2,
    StartService = 3,
    StopService = 4,
    CheckStatus = 5,
    ConfigureSetting = 6,
    ListFiles = 7,
    Search = 8,
    Unknown = 99,
}

/// Parsed command
#[repr(C)]
pub struct ParsedCommand {
    pub intent: CommandIntent,
    pub target: [u8; 128],
    pub parameters: [u8; 256],
    pub confidence: f32,
}

impl ParsedCommand {
    pub fn new(intent: CommandIntent) -> Self {
        ParsedCommand {
            intent,
            target: [0u8; 128],
            parameters: [0u8; 256],
            confidence: 0.0,
        }
    }
    
    pub fn set_target(&mut self, target: &str) {
        let target_bytes = target.as_bytes();
        let len = target_bytes.len().min(127);
        
        unsafe {
            core::ptr::copy_nonoverlapping(target_bytes.as_ptr(), self.target.as_mut_ptr(), len);
        }
    }
    
    pub fn set_parameters(&mut self, params: &str) {
        let params_bytes = params.as_bytes();
        let len = params_bytes.len().min(255);
        
        unsafe {
            core::ptr::copy_nonoverlapping(params_bytes.as_ptr(), self.parameters.as_mut_ptr(), len);
        }
    }
}

/// Natural language parser
pub struct NaturalLanguageParser {
    total_commands: AtomicU64,
    successful_parses: AtomicU64,
}

impl NaturalLanguageParser {
    pub fn new() -> Self {
        NaturalLanguageParser {
            total_commands: AtomicU64::new(0),
            successful_parses: AtomicU64::new(0),
        }
    }
    
    /// Parse natural language input into command
    pub fn parse(&self, input: &str) -> ParsedCommand {
        self.total_commands.fetch_add(1, Ordering::SeqCst);
        
        let input_lower = to_lowercase(input);
        let intent = self.detect_intent(&input_lower);
        let (target, params) = self.extract_target_params(&input_lower, intent);
        
        let mut command = ParsedCommand::new(intent);
        command.set_target(&target);
        command.set_parameters(&params);
        command.confidence = self.calculate_confidence(&input_lower, intent);
        
        if command.confidence > 0.5 {
            self.successful_parses.fetch_add(1, Ordering::SeqCst);
        }
        
        command
    }
    
    fn detect_intent(&self, input: &str) -> CommandIntent {
        // Simple keyword-based intent detection
        if input.contains("install") || input.contains("add") {
            CommandIntent::InstallPackage
        } else if input.contains("remove") || input.contains("delete") || input.contains("uninstall") {
            CommandIntent::RemovePackage
        } else if input.contains("update") || input.contains("upgrade") {
            CommandIntent::UpdateSystem
        } else if input.contains("start") || input.contains("enable") {
            CommandIntent::StartService
        } else if input.contains("stop") || input.contains("disable") {
            CommandIntent::StopService
        } else if input.contains("status") || input.contains("running") {
            CommandIntent::CheckStatus
        } else if input.contains("config") || input.contains("setting") {
            CommandIntent::ConfigureSetting
        } else if input.contains("list") || input.contains("show") {
            CommandIntent::ListFiles
        } else if input.contains("search") || input.contains("find") {
            CommandIntent::Search
        } else {
            CommandIntent::Unknown
        }
    }
    
    fn extract_target_params(&self, input: &str, intent: CommandIntent) -> (String, String) {
        // Simple extraction logic
        let words: Vec<&str> = input.split_whitespace().collect();
        let mut target = String::new();
        let mut params = String::new();
        
        match intent {
            CommandIntent::InstallPackage => {
                if words.len() > 1 {
                    target = words[1].to_string();
                }
            }
            CommandIntent::RemovePackage => {
                if words.len() > 1 {
                    target = words[1].to_string();
                }
            }
            CommandIntent::StartService => {
                if words.len() > 1 {
                    target = words[1].to_string();
                }
            }
            CommandIntent::StopService => {
                if words.len() > 1 {
                    target = words[1].to_string();
                }
            }
            _ => {}
        }
        
        (target, params)
    }
    
    fn calculate_confidence(&self, input: &str, intent: CommandIntent) -> f32 {
        if intent == CommandIntent::Unknown {
            return 0.0;
        }
        
        let mut confidence = 0.5;
        
        // Boost confidence for clear commands
        if input.len() > 5 && input.len() < 100 {
            confidence += 0.2;
        }
        
        // Boost for specific keywords
        if input.contains("please") || input.contains("can you") {
            confidence += 0.1;
        }
        
        if confidence > 1.0 {
            confidence = 1.0;
        }
        
        confidence
    }
    
    pub fn stats(&self) -> ParserStats {
        ParserStats {
            total_commands: self.total_commands.load(Ordering::SeqCst),
            successful_parses: self.successful_parses.load(Ordering::SeqCst),
        }
    }
}

/// Parser statistics
#[repr(C)]
pub struct ParserStats {
    pub total_commands: u64,
    pub successful_parses: u64,
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
    
    fn to_string(&self) -> String {
        String {
            data: self.data.clone(),
        }
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        String {
            data: self.data.clone(),
        }
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

    fn clone(&self) -> Self {
        let mut new_vec = Vec::new();
        unsafe {
            for i in 0..self.len {
                new_vec.push(core::ptr::read(self.data.add(i)));
            }
        }
        new_vec
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
