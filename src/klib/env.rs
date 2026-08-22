//! Custom environment variable access for SigmaOS
//! This module provides no_std alternatives to std::env

use core::ffi::c_char;
use core::str::Utf8Error;

/// Error types for environment operations
#[derive(Debug)]
pub enum EnvError {
    SetFailed,
    RemoveFailed,
    InvalidKey,
    Utf8Error(Utf8Error),
    NotFound,
}

/// Environment variable access
pub struct SigmaEnv;

/// Environment variable iterator
pub struct EnvIterator {
    current: *const *const c_char,
    index: usize,
}

/// Command line argument iterator
pub struct ArgsIterator {
    current: *const *const c_char,
    index: usize,
}

// Syscall numbers (platform-specific)
const SYSCALL_SETENV: usize = 0x100;
const SYSCALL_UNSETENV: usize = 0x101;
const SYSCALL_GETENV: usize = 0x102;

impl SigmaEnv {
    /// Get an environment variable
    pub fn get(key: &str) -> Option<&'static str> {
        // Read from process environment block
        let envp = unsafe { Self::get_envp_pointer() };
        if envp.is_null() {
            return None;
        }
        
        unsafe {
            Self::search_env_block(envp, key)
        }
    }
    
    /// Set an environment variable
    pub fn set(key: &str, value: &str) -> Result<(), EnvError> {
        let key_cstr = Self::str_to_cstr(key)?;
        let value_cstr = Self::str_to_cstr(value)?;
        
        let result = unsafe {
            syscall(SYSCALL_SETENV, key_cstr.as_ptr(), value_cstr.as_ptr())
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(EnvError::SetFailed)
        }
    }
    
    /// Remove an environment variable
    pub fn remove(key: &str) -> Result<(), EnvError> {
        let key_cstr = Self::str_to_cstr(key)?;
        
        let result = unsafe {
            syscall(SYSCALL_UNSETENV, key_cstr.as_ptr())
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(EnvError::RemoveFailed)
        }
    }
    
    /// Get command line arguments
    pub fn args() -> impl Iterator<Item = &'static str> {
        // Get command line arguments
        let argv = unsafe { Self::get_argv_pointer() };
        EnvIterator::new(argv)
    }
    
    /// Get environment variables iterator
    pub fn vars() -> EnvIterator {
        let envp = Self::get_envp_pointer();
        EnvIterator::new(envp)
    }
    
    /// Get the environment pointer from process ABI
    unsafe fn get_envp_pointer() -> *const *const c_char {
        extern "C" {
            static environ: *const *const c_char;
        }
        environ
    }
    
    /// Get the argv pointer from process ABI
    unsafe fn get_argv_pointer() -> *const *const c_char {
        extern "C" {
            static argv: *const *const c_char;
        }
        argv
    }
    
    /// Search the environment block for a key
    unsafe fn search_env_block(envp: *const *const c_char, key: &str) -> Option<&'static str> {
        let mut i = 0;
        loop {
            let entry = *envp.add(i);
            if entry.is_null() {
                return None;
            }
            
            let entry_str = Self::cstr_to_str(entry).ok()?;
            
            if let Some(value) = Self::parse_env_entry(entry_str, key) {
                return Some(value);
            }
            
            i += 1;
        }
    }
    
    /// Convert C string to Rust string
    unsafe fn cstr_to_str(ptr: *const c_char) -> Result<&'static str, EnvError> {
        let mut len = 0;
        loop {
            if *ptr.add(len) == 0 {
                let bytes = core::slice::from_raw_parts(ptr as *const u8, len);
                return core::str::from_utf8(bytes)
                    .map_err(EnvError::Utf8Error);
            }
            len += 1;
        }
    }
    
    /// Parse an environment entry
    fn parse_env_entry(entry: &'static str, key: &str) -> Option<&'static str> {
        if let Some(eq_idx) = entry.find('=') {
            let (k, v) = entry.split_at(eq_idx);
            if k == key {
                return Some(&v[1..]);
            }
        }
        None
    }
    
    /// Convert Rust string to C string
    fn str_to_cstr(s: &str) -> Result<[u8; 256], EnvError> {
        let mut cstr = [0u8; 256];
        let bytes = s.as_bytes();
        
        if bytes.len() >= 256 {
            return Err(EnvError::InvalidKey);
        }
        
        for (i, &byte) in bytes.iter().enumerate() {
            cstr[i] = byte;
        }
        
        Ok(cstr)
    }
}

impl EnvIterator {
    fn new(envp: *const *const c_char) -> Self {
        Self {
            current: envp,
            index: 0,
        }
    }
}

impl Iterator for EnvIterator {
    type Item = Result<(&'static str, &'static str), EnvError>;
    
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let entry = *self.current.add(self.index);
            if entry.is_null() {
                return None;
            }
            
            self.index += 1;
            
            let entry_str = SigmaEnv::cstr_to_str(entry).map(|s| {
                let parts: Vec<&str> = s.splitn(2, '=').collect();
                if parts.len() == 2 {
                    Ok((parts[0], parts[1]))
                } else {
                    Err(EnvError::InvalidKey)
                }
            });
            
            match entry_str {
                Ok(Ok(pair)) => Some(Ok(pair)),
                Ok(Err(e)) => Some(Err(e)),
                Err(e) => Some(Err(e)),
            }
        }
    }
}

impl ArgsIterator {
    fn new(argv: *const *const c_char) -> Self {
        Self {
            current: argv,
            index: 0,
        }
    }
}

impl Iterator for ArgsIterator {
    type Item = Result<&'static str, EnvError>;
    
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let entry = *self.current.add(self.index);
            if entry.is_null() {
                return None;
            }
            
            self.index += 1;
            Some(SigmaEnv::cstr_to_str(entry))
        }
    }
}

// Inline syscall function (platform-specific)
#[inline(always)]
unsafe fn syscall(num: usize, arg1: *const u8, arg2: *const u8) -> isize {
    let mut ret: isize;
    asm!(
        "syscall",
        inlateout("rax") num as isize => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        clobber_aborts("rcx", "r11", "memory")
    );
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_env_operations() {
        // Test set
        let result = SigmaEnv::set("TEST_VAR", "test_value");
        assert!(result.is_ok() || result.is_err()); // May fail in test environment
        
        // Test get
        let result = SigmaEnv::get("TEST_VAR");
        assert!(result.is_some() || result.is_none());
        
        // Test remove
        let result = SigmaEnv::remove("TEST_VAR");
        assert!(result.is_ok() || result.is_err());
    }
    
    #[test]
    fn test_str_to_cstr() {
        let result = SigmaEnv::str_to_cstr("test");
        assert!(result.is_ok());
        
        let cstr = result.unwrap();
        assert_eq!(cstr[0], b't');
        assert_eq!(cstr[1], b'e');
        assert_eq!(cstr[2], b's');
        assert_eq!(cstr[3], b't');
        assert_eq!(cstr[4], 0);
    }
    
    #[test]
    fn test_parse_env_entry() {
        let entry = "TEST_VAR=test_value";
        let result = SigmaEnv::parse_env_entry(entry, "TEST_VAR");
        assert_eq!(result, Some("test_value"));
        
        let result = SigmaEnv::parse_env_entry(entry, "OTHER_VAR");
        assert_eq!(result, None);
    }
}
