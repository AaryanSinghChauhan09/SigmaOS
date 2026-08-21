// Environment variable access module for SigmaOS
// Replaces std::env functionality

use crate::klib::custom_string::SigmaString;

pub struct SigmaEnv;

#[derive(Debug)]
pub enum EnvError {
    GetFailed,
    SetFailed,
    RemoveFailed,
    InvalidKey,
}

impl SigmaEnv {
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
    
    pub fn set(key: &str, value: &str) -> Result<(), EnvError> {
        // Set environment variable via syscall
        let syscall_num = 3; // SYSCALL_SETENV placeholder
        let result = unsafe {
            Self::syscall(syscall_num, key.as_ptr(), value.as_ptr())
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(EnvError::SetFailed)
        }
    }
    
    pub fn remove(key: &str) -> Result<(), EnvError> {
        // Remove environment variable via syscall
        let syscall_num = 4; // SYSCALL_UNSETENV placeholder
        let result = unsafe {
            Self::syscall(syscall_num, key.as_ptr())
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(EnvError::RemoveFailed)
        }
    }
    
    pub fn args() -> impl Iterator<Item = &'static str> {
        // Get command line arguments
        let argv = unsafe { Self::get_argv_pointer() };
        EnvIterator::new(argv)
    }
    
    unsafe fn get_envp_pointer() -> *const *const u8 {
        // Get envp from process ABI
        extern "C" {
            static environ: *const *const u8;
        }
        environ
    }
    
    unsafe fn search_env_block(envp: *const *const u8, key: &str) -> Option<&'static str> {
        let mut i = 0;
        loop {
            let entry = *envp.add(i);
            if entry.is_null() {
                return None;
            }
            
            let entry_str = Self::get_c_string(entry);
            
            if let Some(value) = Self::parse_env_entry(entry_str, key) {
                return value;
            }
            
            i += 1;
        }
    }
    
    unsafe fn get_c_string(ptr: *const u8) -> &'static str {
        let mut len = 0;
        loop {
            if *ptr.add(len) == 0 {
                return core::str::from_utf8_unchecked(
                    core::slice::from_raw_parts(ptr, len)
                );
            }
            len += 1;
        }
    }
    
    fn parse_env_entry(entry: &'static str, key: &str) -> Option<&'static str> {
        if let Some(eq_idx) = entry.find('=') {
            let (k, v) = entry.split_at(eq_idx);
            if k == key {
                return Some(&v[1..]);
            }
        }
        None
    }
    
    unsafe fn get_argv_pointer() -> *const *const u8 {
        extern "C" {
            static argv: *const *const u8;
        }
        argv
    }
    
    unsafe fn syscall(num: i32, arg1: *const u8, arg2: *const u8) -> i32 {
        // Placeholder for actual syscall implementation
        // This would be replaced with actual syscall when running on SigmaOS
        0
    }
}

pub struct EnvIterator {
    current: *const *const u8,
    index: usize,
}

impl EnvIterator {
    fn new(argv: *const *const u8) -> Self {
        Self {
            current: argv,
            index: 0,
        }
    }
}

impl Iterator for EnvIterator {
    type Item = &'static str;
    
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let entry = *self.current.add(self.index);
            if entry.is_null() {
                return None;
            }
            
            self.index += 1;
            Some(SigmaEnv::get_c_string(entry))
        }
    }
}
