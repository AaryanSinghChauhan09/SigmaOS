#![no_std]
#![no_main]

/// OOP-based Script Engine for SigmaOS
/// Based on Ideas-999-Structured: Automation & Scripting Item 856
/// Implements scripting engine and execution

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ScriptID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ScriptLanguage { Python = 0, JavaScript = 1, Lua = 2, Shell = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ScriptError { Success = 0, NotFound = 1, ExecutionFailed = 2 }

pub trait Script {
    fn id(&self) -> ScriptID;
    fn name(&self) -> &[u8];
    fn language(&self) -> ScriptLanguage;
    fn source(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleScript {
    pub id: ScriptID,
    pub name: [u8; 128],
    pub language: AtomicUsize,
    pub source: Vec<u8>,
}

impl SimpleScript {
    pub fn new(id: ScriptID, name: &[u8], language: ScriptLanguage, source: &[u8]) -> Self {
        let mut name_array = [0u8; 128];
        let name_len = name.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        let mut source_vec = Vec::new();
        for &byte in source {
            source_vec.push(byte);
        }
        SimpleScript {
            id,
            name: name_array,
            language: AtomicUsize::new(language as usize),
            source: source_vec,
        }
    }
}

impl Script for SimpleScript {
    fn id(&self) -> ScriptID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(128);
        &self.name[..len]
    }
    fn language(&self) -> ScriptLanguage { unsafe { core::mem::transmute(self.language.load(Ordering::SeqCst)) } }
    fn source(&self) -> &[u8] { &self.source }
}

pub trait ScriptEngine {
    fn load_script(&mut self, script: Box<dyn Script>) -> Result<ScriptID, ScriptError>;
    fn unload_script(&mut self, id: ScriptID) -> Result<(), ScriptError>;
    fn execute_script(&self, id: ScriptID) -> Result<Vec<u8>, ScriptError>;
    fn get_script(&self, id: ScriptID) -> Option<&dyn Script>;
}

#[repr(C)]
pub struct SimpleScriptEngine {
    pub scripts: Vec<Option<Box<dyn Script>>>,
    pub next_id: AtomicUsize,
}

impl SimpleScriptEngine {
    pub fn new() -> Self {
        SimpleScriptEngine {
            scripts: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ScriptEngine for SimpleScriptEngine {
    fn load_script(&mut self, script: Box<dyn Script>) -> Result<ScriptID, ScriptError> {
        let id = script.id();
        self.scripts.push(Some(script));
        Ok(id)
    }
    
    fn unload_script(&mut self, id: ScriptID) -> Result<(), ScriptError> {
        for script_option in &mut self.scripts {
            if let Some(ref script) = *script_option {
                if script.id() == id {
                    return Ok(());
                }
            }
        }
        Err(ScriptError::NotFound)
    }
    
    fn execute_script(&self, id: ScriptID) -> Result<Vec<u8>, ScriptError> {
        if let Some(script) = self.get_script(id) {
            let mut output = Vec::new();
            for &byte in script.source() {
                output.push(byte);
            }
            Ok(output)
        } else {
            Err(ScriptError::NotFound)
        }
    }
    
    fn get_script(&self, id: ScriptID) -> Option<&dyn Script> {
        for script_option in &self.scripts {
            if let Some(ref script) = *script_option {
                if script.id() == id { return Some(script.as_ref()); }
            }
        }
        None
    }
}

pub trait ScriptAPI {
    fn register_function(&mut self, name: &[u8], func: fn() -> Vec<u8>);
    fn call_function(&self, name: &[u8]) -> Result<Vec<u8>, ScriptError>;
}

#[repr(C)]
pub struct SimpleScriptAPI {
    pub functions: Vec<([u8; 64], fn() -> Vec<u8>)>,
}

impl SimpleScriptAPI {
    pub fn new() -> Self {
        SimpleScriptAPI {
            functions: Vec::new(),
        }
    }
}

impl ScriptAPI for SimpleScriptAPI {
    fn register_function(&mut self, name: &[u8], func: fn() -> Vec<u8>) {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        for i in 0..name_len {
            name_array[i] = name[i];
        }
        self.functions.push((name_array, func));
    }
    
    fn call_function(&self, name: &[u8]) -> Result<Vec<u8>, ScriptError> {
        for &(ref func_name, func) in &self.functions {
            let len = func_name.iter().position(|&b| b == 0).unwrap_or(64);
            if &func_name[..len] == name {
                return Ok(func());
            }
        }
        Err(ScriptError::NotFound)
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
