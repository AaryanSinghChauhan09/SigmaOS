#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based Script Engine for SigmaOS
/// Based on Ideas-999-Structured: Automation & Scripting Item 856
/// Implements scripting engine and execution

#[cfg(not(target_os = "none"))]
use std::sync::atomic::{AtomicUsize, Ordering};
#[cfg(not(target_os = "none"))]
use std::mem;
#[cfg(not(target_os = "none"))]
use std::ops::{Deref, DerefMut};

#[cfg(target_os = "none")]
use core::sync::atomic::{AtomicUsize, Ordering};
#[cfg(target_os = "none")]
use core::mem;
#[cfg(target_os = "none")]
use core::ops::{Deref, DerefMut};

pub type ScriptID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ScriptLanguage { Python = 0, JavaScript = 1, Lua = 2, Shell = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptError {
    Success = 0,
    NotFound = 1,
    ExecutionFailed = 2,
    SyntaxError = 3,
    LoopOverflow = 4,
}

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
    fn language(&self) -> ScriptLanguage {
        match self.language.load(Ordering::SeqCst) {
            0 => ScriptLanguage::Python,
            1 => ScriptLanguage::JavaScript,
            2 => ScriptLanguage::Lua,
            _ => ScriptLanguage::Shell,
        }
    }
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
    #[allow(clippy::new_without_default)]
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
    #[allow(clippy::new_without_default)]
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


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.deref_mut().iter_mut()
    }
}

// ================= Linux & BSD-Inspired Shell Script Interpreter =================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellLoopType {
    For,
    While,
    DoWhile,
    ForeachExtension,
}

/// Shell interpreter supporting string/file/command tokenization and counting/foreach repetition loops
pub struct ShellScriptInterpreter;

impl ShellScriptInterpreter {
    pub fn tokenize_string(input: &str, delimiter: char) -> std::vec::Vec<std::string::String> {
        let mut tokens = std::vec::Vec::new();
        for token in input.split(delimiter) {
            if !token.is_empty() {
                tokens.push(token.to_string());
            }
        }
        tokens
    }

    pub fn tokenize_command_output(output: &str) -> std::vec::Vec<std::string::String> {
        let mut fields = std::vec::Vec::new();
        for line in output.lines() {
            for field in line.split_whitespace() {
                if !field.is_empty() {
                    fields.push(field.to_string());
                }
            }
        }
        fields
    }

    pub fn tokenize_from_file(file_content: &str) -> std::vec::Vec<std::string::String> {
        let mut tokens = std::vec::Vec::new();
        for line in file_content.lines() {
            let clean = line.split('#').next().unwrap_or("").trim();
            if !clean.is_empty() {
                tokens.push(clean.to_string());
            }
        }
        tokens
    }

    pub fn execute_shell_loop(loop_type: ShellLoopType, variable: &str, limit_or_files: &[&str]) -> Result<std::vec::Vec<std::string::String>, ScriptError> {
        let mut results = std::vec::Vec::new();
        if limit_or_files.len() > 1000 {
            return Err(ScriptError::LoopOverflow);
        }

        match loop_type {
            ShellLoopType::For | ShellLoopType::While | ShellLoopType::DoWhile => {
                for &val in limit_or_files {
                    results.push(std::format!("{}={}", variable, val));
                }
            }
            ShellLoopType::ForeachExtension => {
                for &file in limit_or_files {
                    if file.ends_with(".rs") || file.ends_with(".sh") {
                        results.push(std::format!("processed_{}:{}", variable, file));
                    }
                }
            }
        }
        Ok(results)
    }

    pub fn evaluate_conditional_block(block_code: &str, flag_condition: bool) -> Result<std::string::String, ScriptError> {
        if !block_code.contains("if") || !block_code.contains("fi") {
            return Err(ScriptError::SyntaxError);
        }
        if flag_condition {
            Ok("Executing then block".to_string())
        } else {
            Ok("Executing else block".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_tokenizer_primitives() {
        // String tokenization
        let tokens = ShellScriptInterpreter::tokenize_string("param1:param2:param3", ':');
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], "param1");

        // Command output tokenization
        let command_out = "Active   Size\neth0     1500\n";
        let fields = ShellScriptInterpreter::tokenize_command_output(command_out);
        assert_eq!(fields.len(), 4);
        assert_eq!(fields[2], "eth0");

        // File tokenization with comment stripping
        let file_conf = "# Core Config\nnameserver 8.8.8.8 # Google DNS\n";
        let file_tokens = ShellScriptInterpreter::tokenize_from_file(file_conf);
        assert_eq!(file_tokens.len(), 1);
        assert_eq!(file_tokens[0], "nameserver 8.8.8.8");
    }

    #[test]
    fn test_shell_repetition_loops() {
        // Count loop
        let list = ["1", "2", "3"];
        let count_res = ShellScriptInterpreter::execute_shell_loop(ShellLoopType::For, "idx", &list).unwrap();
        assert_eq!(count_res.len(), 3);
        assert_eq!(count_res[0], "idx=1");

        // Extension provided foreach loop
        let files = ["main.rs", "config.toml", "init.sh", "readme.md"];
        let foreach_res = ShellScriptInterpreter::execute_shell_loop(ShellLoopType::ForeachExtension, "file", &files).unwrap();
        assert_eq!(foreach_res.len(), 2);
        assert_eq!(foreach_res[0], "processed_file:main.rs");
        assert_eq!(foreach_res[1], "processed_file:init.sh");
    }

    #[test]
    fn test_shell_conditional_syntax_errors() {
        // Successful match
        let res_then = ShellScriptInterpreter::evaluate_conditional_block("if [ x ]; then y; fi", true).unwrap();
        assert_eq!(res_then, "Executing then block");

        // Syntax error check
        let res_err = ShellScriptInterpreter::evaluate_conditional_block("invalid conditional block", true);
        assert_eq!(res_err, Err(ScriptError::SyntaxError));

        // Loop overflow check
        let giant_list = ["1"; 1005];
        let res_overflow = ShellScriptInterpreter::execute_shell_loop(ShellLoopType::For, "idx", &giant_list);
        assert_eq!(res_overflow, Err(ScriptError::LoopOverflow));
    }

    #[test]
    fn test_script_argument_router() {
        let router = ScriptArgumentRouter::new("#!/bin/sh -x");
        assert_eq!(router.shebang_interpreter, "/bin/sh -x");

        let args = ["app", "arg1", "arg2"];
        let res = router.substitute_arguments("Echo $1 then $2 all $@", &args);
        assert!(res.contains("arg1"));
        assert!(res.contains("arg2"));
    }
}

pub struct ScriptArgumentRouter {
    pub shebang_interpreter: String,
}

impl ScriptArgumentRouter {
    pub fn new(shebang_line: &str) -> Self {
        let interp = if shebang_line.starts_with("#!") {
            shebang_line.trim_start_matches("#!").trim()
        } else {
            "/bin/sh"
        };
        Self {
            shebang_interpreter: interp.to_string(),
        }
    }

    pub fn substitute_arguments(&self, script: &str, args: &[&str]) -> String {
        let mut result = script.to_string();
        for (i, arg) in args.iter().enumerate() {
            let var_name = format!("${}", i);
            result = result.replace(&var_name, arg);
        }

        let mut all_args = String::new();
        for (i, arg) in args.iter().skip(1).enumerate() {
            if i > 0 { all_args.push(' '); }
            all_args.push_str(arg);
        }
        result = result.replace("$@", &all_args);
        result
    }
}

impl Default for ScriptArgumentRouter {
    fn default() -> Self {
        Self::new("#!/bin/sh")
    }
}
