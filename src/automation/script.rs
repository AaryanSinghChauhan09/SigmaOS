/// OOP-based Advanced Script Engine, Decompressor & File Monitor for SigmaOS
/// Implements interactive scripting, dynamic script-like functions, positional arguments,
/// script aliases, basic UPX-style binary unpacking, filesystem monitoring, and string descrambling.

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type ScriptID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptLanguage { Python = 0, JavaScript = 1, Lua = 2, Shell = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptError { Success = 0, NotFound = 1, ExecutionFailed = 2, InvalidArgument = 3 }

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
    fn id(&self) -> ScriptID {
        self.id
    }

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

    fn source(&self) -> &[u8] {
        &self.source
    }
}

pub trait ScriptEngine {
    fn load_script(&mut self, script: Box<dyn Script>) -> Result<ScriptID, ScriptError>;
    fn unload_script(&mut self, id: ScriptID) -> Result<(), ScriptError>;
    fn execute_script(&self, id: ScriptID) -> Result<Vec<u8>, ScriptError>;
    fn get_script(&self, id: ScriptID) -> Option<&dyn Script>;
}

pub struct SimpleScriptEngine {
    pub scripts: Vec<Option<Box<dyn Script>>>,
    pub next_id: AtomicUsize,
    pub aliases: SimpleScriptEnvironment, // maps @call alias -> target script name
}

impl SimpleScriptEngine {
    pub fn new() -> Self {
        SimpleScriptEngine {
            scripts: Vec::new(),
            next_id: AtomicUsize::new(1),
            aliases: SimpleScriptEnvironment::new(),
        }
    }

    /// Register a @call alias for a script
    pub fn set_script_alias(&mut self, alias: &[u8], script_name: &[u8]) {
        self.aliases.set(alias, script_name);
    }

    /// Executes a loaded script after performing positional parameter expansion (e.g., replacing $1, $2 with arguments)
    pub fn execute_script_with_args(&self, id: ScriptID, args: &[&[u8]]) -> Result<Vec<u8>, ScriptError> {
        let script = self.get_script(id).ok_or(ScriptError::NotFound)?;
        let source = script.source();

        let mut expanded = Vec::new();
        let mut i = 0;
        while i < source.len() {
            // Check for positional arguments: e.g. $1, $2
            if source[i] == b'$' && i + 1 < source.len() && source[i + 1] >= b'1' && source[i + 1] <= b'9' {
                let arg_index = (source[i + 1] - b'1') as usize;
                if arg_index < args.len() {
                    for &byte in args[arg_index] {
                        expanded.push(byte);
                    }
                }
                i += 2;
            } else {
                expanded.push(source[i]);
                i += 1;
            }
        }

        Ok(expanded)
    }

    /// Resolve and execute script via @call alias
    pub fn execute_by_alias(&self, alias: &[u8], args: &[&[u8]]) -> Result<Vec<u8>, ScriptError> {
        let target_name = self.aliases.get(alias).ok_or(ScriptError::NotFound)?;

        for script_option in &self.scripts {
            if let Some(ref script) = *script_option {
                if script.name() == target_name {
                    return self.execute_script_with_args(script.id(), args);
                }
            }
        }

        Err(ScriptError::NotFound)
    }
}

impl Default for SimpleScriptEngine {
    fn default() -> Self {
        Self::new()
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
                if script.id() == id {
                    return Some(script.as_ref());
                }
            }
        }
        None
    }
}

pub trait ScriptAPI {
    fn register_function(&mut self, name: &[u8], func: fn() -> Vec<u8>);
    fn call_function(&self, name: &[u8]) -> Result<Vec<u8>, ScriptError>;
}

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

impl Default for SimpleScriptAPI {
    fn default() -> Self {
        Self::new()
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

/// Simple ShellEnvironment helper recycled for scripts alias maps
#[repr(C)]
pub struct SimpleScriptEnvironment {
    pub keys: Vec<[u8; 64]>,
    pub values: Vec<[u8; 64]>,
    pub key_lengths: Vec<usize>,
    pub value_lengths: Vec<usize>,
}

impl SimpleScriptEnvironment {
    pub fn new() -> Self {
        SimpleScriptEnvironment {
            keys: Vec::new(),
            values: Vec::new(),
            key_lengths: Vec::new(),
            value_lengths: Vec::new(),
        }
    }

    pub fn set(&mut self, key: &[u8], value: &[u8]) {
        let key_len = key.len().min(63);
        let value_len = value.len().min(63);

        let mut key_entry = [0u8; 64];
        let mut value_entry = [0u8; 64];

        for i in 0..key_len { key_entry[i] = key[i]; }
        for i in 0..value_len { value_entry[i] = value[i]; }

        for i in 0..self.keys.len() {
            if self.key_lengths[i] == key_len && &self.keys[i][..key_len] == key {
                self.values[i] = value_entry;
                self.value_lengths[i] = value_len;
                return;
            }
        }

        self.keys.push(key_entry);
        self.values.push(value_entry);
        self.key_lengths.push(key_len);
        self.value_lengths.push(value_len);
    }

    pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
        let key_len = key.len();
        for i in 0..self.keys.len() {
            if self.key_lengths[i] == key_len && &self.keys[i][..key_len] == key {
                let value_len = self.value_lengths[i];
                return Some(&self.values[i][..value_len]);
            }
        }
        None
    }
}

impl Default for SimpleScriptEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// ADDITIONAL DIAGNOSTICS & SYSTEM UTILITIES
// ==========================================

/// Basic UPX-style dynamic payload decompressor (XOR/header-shift unpacker)
pub struct UpxUnpacker {
    pub magic_header: [u8; 4], // e.g. b"UPX!"
}

impl UpxUnpacker {
    pub fn new() -> Self {
        UpxUnpacker {
            magic_header: *b"UPX!",
        }
    }

    /// Decompresses / Unpacks a raw binary chunk if it contains the valid signature header
    pub fn decompress_payload(&self, compressed: &[u8]) -> Result<Vec<u8>, &'static str> {
        if compressed.len() < 8 {
            return Err("UPX: Payload is too small.");
        }
        if &compressed[..4] != &self.magic_header {
            return Err("UPX: Signature mismatch (not compressed with UPX).");
        }

        // Simple decompressive decryption: XOR shift with offset bytes
        let mut decompressed = Vec::new();
        for i in 4..compressed.len() {
            decompressed.push(compressed[i] ^ 0x5A);
        }
        Ok(decompressed)
    }
}

impl Default for UpxUnpacker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsEvent {
    Modified,
    Added,
    Deleted,
}

/// Basic File Monitor (fs watcher) tracking directory and folder additions/modifications
pub struct FileMonitor {
    pub monitored_path: [u8; 64],
    pub events_count: u32,
}

impl FileMonitor {
    pub fn new(path: &[u8]) -> Self {
        let mut path_arr = [0u8; 64];
        let len = path.len().min(63);
        path_arr[..len].copy_from_slice(&path[..len]);
        FileMonitor {
            monitored_path: path_arr,
            events_count: 0,
        }
    }

    /// Polls/simulates a modification event on a file name
    pub fn simulate_event(&mut self, _file_name: &str, event: FsEvent) -> (FsEvent, u32) {
        self.events_count += 1;
        (event, self.events_count)
    }
}

/// Basic String Descrambler (XOR key anti-obfuscation utility)
pub struct StringDescrambler {
    pub xor_key: u8,
}

impl StringDescrambler {
    pub fn new(key: u8) -> Self {
        StringDescrambler { xor_key: key }
    }

    /// Descrambles an obfuscated byte sequence on-the-fly
    pub fn descramble_string(&self, scrambled: &[u8]) -> Vec<u8> {
        let mut cleartext = Vec::new();
        for &byte in scrambled {
            cleartext.push(byte ^ self.xor_key);
        }
        cleartext
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
            if i > 0 {
                all_args.push(' ');
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_positional_arguments_expansion() {
        let script = SimpleScript::new(
            1,
            b"greet.sh",
            ScriptLanguage::Shell,
            b"echo hello $1, welcome back $2!",
        );

        let script_id = 1;
        let mut scripts = Vec::new();
        scripts.push(Some(Box::new(script) as Box<dyn Script>));

        let engine_with_script = SimpleScriptEngine {
            scripts,
            next_id: AtomicUsize::new(2),
            aliases: SimpleScriptEnvironment::new(),
        };

        let result = engine_with_script
            .execute_script_with_args(script_id, &[b"alice", b"sovereign"])
            .unwrap();
        assert_eq!(result, b"echo hello alice, welcome back sovereign!");
    }

    #[test]
    fn test_script_alias_mapping_and_call() {
        let mut engine = SimpleScriptEngine::new();
        let script = SimpleScript::new(1, b"backup.sh", ScriptLanguage::Shell, b"tar -cvf $1");

        engine.load_script(Box::new(script)).unwrap();
        engine.set_script_alias(b"backup", b"backup.sh");

        let res = engine
            .execute_by_alias(b"backup", &[b"/home/state"])
            .unwrap();
        assert_eq!(res, b"tar -cvf /home/state");
    }

    #[test]
    fn test_upx_unpacker_decompression() {
        let unpacker = UpxUnpacker::new();

        assert!(unpacker.decompress_payload(&[0; 5]).is_err());
        assert!(unpacker.decompress_payload(b"NOT_UPX!").is_err());

        let compressed_payload = [
            b'U',
            b'P',
            b'X',
            b'!',
            b'H' ^ 0x5A,
            b'E' ^ 0x5A,
            b'L' ^ 0x5A,
            b'L' ^ 0x5A,
            b'O' ^ 0x5A,
        ];

        let decompressed = unpacker.decompress_payload(&compressed_payload).unwrap();
        assert_eq!(decompressed, b"HELLO");
    }

    #[test]
    fn test_file_monitor_events() {
        let mut monitor = FileMonitor::new(b"/var/log");
        assert_eq!(monitor.events_count, 0);

        let (event, count) = monitor.simulate_event("auth.log", FsEvent::Modified);
        assert_eq!(event, FsEvent::Modified);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_string_descrambling() {
        let descrambler = StringDescrambler::new(0x33);
        let scrambled = [b'A' ^ 0x33, b'B' ^ 0x33, b'C' ^ 0x33];

        let descrambled = descrambler.descramble_string(&scrambled);
        assert_eq!(descrambled, b"ABC");
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
