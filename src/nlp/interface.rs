#![no_std]
#![no_main]

/// OOP-based Natural Language Interface for SigmaOS
/// Based on Ideas-999-Structured: AI & Automation Item 356
/// Implements NL→CLI translator with intent classification

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type IntentID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum IntentType { ExecuteCommand = 0, QuerySystem = 1, Configure = 2, Help = 3, Unknown = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NLLError { Success = 0, ParseFailed = 1, ClassificationFailed = 2, TranslationFailed = 3 }

pub trait Intent {
    fn id(&self) -> IntentID;
    fn intent_type(&self) -> IntentType;
    fn confidence(&self) -> f32;
    fn parameters(&self) -> &[[u8; 64]];
}

#[repr(C)]
pub struct SimpleIntent {
    pub id: IntentID,
    pub intent_type: AtomicUsize,
    pub confidence: AtomicUsize,
    pub parameters: Vec<[u8; 64]>,
}

impl SimpleIntent {
    pub fn new(id: IntentID, intent_type: IntentType, confidence: f32) -> Self {
        let conf_bits = (confidence * 100.0) as usize;
        SimpleIntent {
            id,
            intent_type: AtomicUsize::new(intent_type as usize),
            confidence: AtomicUsize::new(conf_bits),
            parameters: Vec::new(),
        }
    }
}

impl Intent for SimpleIntent {
    fn id(&self) -> IntentID { self.id }
    fn intent_type(&self) -> IntentType { unsafe { core::mem::transmute(self.intent_type.load(Ordering::SeqCst)) } }
    fn confidence(&self) -> f32 { (self.confidence.load(Ordering::SeqCst) as f32) / 100.0 }
    fn parameters(&self) -> &[[u8; 64]] { &self.parameters }
}

pub trait Tokenizer {
    fn tokenize(&self, input: &[u8]) -> Vec<[u8; 64]>;
    fn normalize(&self, token: &[u8]) -> [u8; 64];
}

#[repr(C)]
pub struct SimpleTokenizer;

impl SimpleTokenizer {
    pub fn new() -> Self { SimpleTokenizer }
}

impl Tokenizer for SimpleTokenizer {
    fn tokenize(&self, input: &[u8]) -> Vec<[u8; 64]> {
        let mut tokens = Vec::new();
        let mut current_token = [0u8; 64];
        let mut token_index = 0;
        
        for &byte in input {
            if byte == b' ' || byte == b'\n' || byte == b'\t' {
                if token_index > 0 {
                    tokens.push(current_token);
                    current_token = [0u8; 64];
                    token_index = 0;
                }
            } else {
                if token_index < 63 {
                    current_token[token_index] = byte;
                    token_index += 1;
                }
            }
        }
        
        if token_index > 0 {
            tokens.push(current_token);
        }
        
        tokens
    }
    
    fn normalize(&self, token: &[u8]) -> [u8; 64] {
        let mut normalized = [0u8; 64];
        let len = token.len().min(63);
        for i in 0..len {
            let byte = token[i];
            if byte >= b'A' && byte <= b'Z' {
                normalized[i] = byte + 32;
            } else {
                normalized[i] = byte;
            }
        }
        normalized
    }
}

pub trait IntentClassifier {
    fn classify(&self, tokens: &[[u8; 64]]) -> Result<IntentType, NLLError>;
    fn extract_parameters(&self, tokens: &[[u8; 64]], intent_type: IntentType) -> Vec<[u8; 64]>;
}

#[repr(C)]
pub struct SimpleIntentClassifier {
    pub command_keywords: Vec<[u8; 32]>,
    pub query_keywords: Vec<[u8; 32]>,
}

impl SimpleIntentClassifier {
    pub fn new() -> Self {
        let mut command_keywords = Vec::new();
        let mut query_keywords = Vec::new();
        
        command_keywords.push(*b"run");
        command_keywords.push(*b"execute");
        command_keywords.push(*b"start");
        command_keywords.push(*b"launch");
        
        query_keywords.push(*b"what");
        query_keywords.push(*b"how");
        query_keywords.push(*b"show");
        query_keywords.push(*b"list");
        
        SimpleIntentClassifier {
            command_keywords,
            query_keywords,
        }
    }
}

impl IntentClassifier for SimpleIntentClassifier {
    fn classify(&self, tokens: &[[u8; 64]]) -> Result<IntentType, NLLError> {
        for token in tokens {
            let len = token.iter().position(|&b| b == 0).unwrap_or(64);
            let token_str = &token[..len];
            
            for &keyword in &self.command_keywords {
                let klen = keyword.iter().position(|&b| b == 0).unwrap_or(32);
                if token_str == &keyword[..klen] {
                    return Ok(IntentType::ExecuteCommand);
                }
            }
            
            for &keyword in &self.query_keywords {
                let klen = keyword.iter().position(|&b| b == 0).unwrap_or(32);
                if token_str == &keyword[..klen] {
                    return Ok(IntentType::QuerySystem);
                }
            }
        }
        
        Ok(IntentType::Unknown)
    }
    
    fn extract_parameters(&self, tokens: &[[u8; 64]], _intent_type: IntentType) -> Vec<[u8; 64]> {
        let mut parameters = Vec::new();
        for token in tokens {
            let len = token.iter().position(|&b| b == 0).unwrap_or(64);
            if len > 0 {
                parameters.push(*token);
            }
        }
        parameters
    }
}

pub trait CommandTranslator {
    fn translate(&self, intent: &dyn Intent) -> Result<Vec<u8>, NLLError>;
    fn get_template(&self, intent_type: IntentType) -> &[u8];
}

#[repr(C)]
pub struct SimpleCommandTranslator {
    pub templates: Vec<(IntentType, [u8; 128])>,
}

impl SimpleCommandTranslator {
    pub fn new() -> Self {
        let mut templates = Vec::new();
        
        templates.push((IntentType::ExecuteCommand, *b"sigma-exec {args}"));
        templates.push((IntentType::QuerySystem, *b"sigma-query {args}"));
        templates.push((IntentType::Configure, *b"sigma-config {args}"));
        templates.push((IntentType::Help, *b"sigma-help {args}"));
        
        SimpleCommandTranslator { templates }
    }
}

impl CommandTranslator for SimpleCommandTranslator {
    fn translate(&self, intent: &dyn Intent) -> Result<Vec<u8>, NLLError> {
        let template = self.get_template(intent.intent_type());
        let mut command = Vec::new();
        
        for &byte in template {
            command.push(byte);
        }
        
        for param in intent.parameters() {
            let len = param.iter().position(|&b| b == 0).unwrap_or(64);
            command.push(b' ');
            for &byte in &param[..len] {
                command.push(byte);
            }
        }
        
        Ok(command)
    }
    
    fn get_template(&self, intent_type: IntentType) -> &[u8] {
        for &(itype, ref template) in &self.templates {
            if itype == intent_type {
                let len = template.iter().position(|&b| b == 0).unwrap_or(128);
                return &template[..len];
            }
        }
        b"sigma-unknown"
    }
}

pub trait NLInterface {
    fn process_input(&mut self, input: &[u8]) -> Result<Vec<u8>, NLLError>;
    fn add_training_example(&mut self, input: &[u8], expected_intent: IntentType);
}

#[repr(C)]
pub struct SimpleNLInterface {
    pub tokenizer: SimpleTokenizer,
    pub classifier: SimpleIntentClassifier,
    pub translator: SimpleCommandTranslator,
    pub next_id: AtomicUsize,
}

impl SimpleNLInterface {
    pub fn new() -> Self {
        SimpleNLInterface {
            tokenizer: SimpleTokenizer::new(),
            classifier: SimpleIntentClassifier::new(),
            translator: SimpleCommandTranslator::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl NLInterface for SimpleNLInterface {
    fn process_input(&mut self, input: &[u8]) -> Result<Vec<u8>, NLLError> {
        let tokens = self.tokenizer.tokenize(input);
        
        let intent_type = self.classifier.classify(&tokens)?;
        let parameters = self.classifier.extract_parameters(&tokens, intent_type);
        
        let mut intent = SimpleIntent::new(self.next_id.fetch_add(1, Ordering::SeqCst), intent_type, 0.95);
        intent.parameters = parameters;
        
        self.translator.translate(&intent)
    }
    
    fn add_training_example(&mut self, _input: &[u8], _expected_intent: IntentType) {
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
