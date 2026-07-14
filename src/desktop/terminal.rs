#![no_std]
#![no_main]

/// OOP-based Desktop Terminal for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 786
/// Implements terminal emulator and shell integration

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TerminalID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TerminalError { Success = 0, NotFound = 1, CommandFailed = 2 }

pub trait Terminal {
    fn id(&self) -> TerminalID;
    fn title(&self) -> &[u8];
    def working_directory(&self) -> &[u8];
    fn set_working_directory(&mut self, path: &[u8]);
}

#[repr(C)]
pub struct SimpleTerminal {
    pub id: TerminalID,
    pub title: [u8; 128],
    pub working_directory: [u8; 256],
}

impl SimpleTerminal {
    pub fn new(id: TerminalID, title: &[u8]) -> Self {
        let mut title_array = [0u8; 128];
        let mut dir_array = [0u8; 256];
        let title_len = title.len().min(127);
        let dir_len = b"/home/user".len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(title.as_ptr(), title_array.as_mut_ptr(), title_len);
            core::ptr::copy_nonoverlapping(b"/home/user".as_ptr(), dir_array.as_mut_ptr(), dir_len);
        }
        SimpleTerminal {
            id,
            title: title_array,
            working_directory: dir_array,
        }
    }
}

impl Terminal for SimpleTerminal {
    fn id(&self) -> TerminalID { self.id }
    fn title(&self) -> &[u8] {
        let len = self.title.iter().position(|&b| b == 0).unwrap_or(128);
        &self.title[..len]
    }
    fn working_directory(&self) -> &[u8] {
        let len = self.working_directory.iter().position(|&b| b == 0).unwrap_or(256);
        &self.working_directory[..len]
    }
    
    fn set_working_directory(&mut self, path: &[u8]) {
        let path_len = path.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(path.as_ptr(), self.working_directory.as_mut_ptr(), path_len);
        }
    }
}

pub trait TerminalManager {
    fn create_terminal(&mut self, title: &[u8]) -> Result<TerminalID, TerminalError>;
    fn close_terminal(&mut self, id: TerminalID) -> Result<(), TerminalError>;
    fn get_terminal(&self, id: TerminalID) -> Option<&dyn Terminal>;
    def execute_command(&mut self, terminal_id: TerminalID, command: &[u8]) -> Result<Vec<u8>, TerminalError>;
}

#[repr(C)]
pub struct SimpleTerminalManager {
    pub terminals: Vec<Option<Box<dyn Terminal>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTerminalManager {
    pub fn new() -> Self {
        SimpleTerminalManager {
            terminals: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TerminalManager for SimpleTerminalManager {
    fn create_terminal(&mut self, title: &[u8]) -> Result<TerminalID, TerminalError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let terminal = SimpleTerminal::new(id, title);
        self.terminals.push(Some(Box::new(terminal)));
        Ok(id)
    }
    
    fn close_terminal(&mut self, id: TerminalID) -> Result<(), TerminalError> {
        for terminal_option in &mut self.terminals {
            if let Some(ref terminal) = *terminal_option {
                if terminal.id() == id {
                    return Ok(());
                }
            }
        }
        Err(TerminalError::NotFound)
    }
    
    fn get_terminal(&self, id: TerminalID) -> Option<&dyn Terminal> {
        for terminal_option in &self.terminals {
            if let Some(ref terminal) = *terminal_option {
                if terminal.id() == id { return Some(terminal.as_ref()); }
            }
        }
        None
    }
    
    fn execute_command(&mut self, terminal_id: TerminalID, command: &[u8]) -> Result<Vec<u8>, TerminalError> {
        if self.get_terminal(terminal_id).is_some() {
            let mut output = Vec::new();
            for &byte in command {
                output.push(byte);
            }
            output.push(b'\n');
            Ok(output)
        } else {
            Err(TerminalError::NotFound)
        }
    }
}

pub trait ShellIntegration {
    fn get_shell(&self) -> &[u8];
    def set_shell(&mut self, shell: &[u8]);
    fn get_env_var(&self, key: &[u8]) -> Option<&[u8]>;
    def set_env_var(&mut self, key: &[u8], value: &[u8]);
}

#[repr(C)]
pub struct SimpleShellIntegration {
    pub shell: [u8; 64],
    pub env_vars: Vec<([u8; 64], [u8; 256])>,
}

impl SimpleShellIntegration {
    pub fn new() -> Self {
        let mut shell_array = [0u8; 64];
        let shell_len = b"/bin/bash".len().min(63);
        for i in 0..shell_len {
            shell_array[i] = b"/bin/bash"[i];
        }
        SimpleShellIntegration {
            shell: shell_array,
            env_vars: Vec::new(),
        }
    }
}

impl ShellIntegration for SimpleShellIntegration {
    fn get_shell(&self) -> &[u8] {
        let len = self.shell.iter().position(|&b| b == 0).unwrap_or(64);
        &self.shell[..len]
    }
    
    fn set_shell(&mut self, shell: &[u8]) {
        let shell_len = shell.len().min(63);
        for i in 0..shell_len {
            self.shell[i] = shell[i];
        }
    }
    
    fn get_env_var(&self, key: &[u8]) -> Option<&[u8]> {
        for &(ref k, ref v) in &self.env_vars {
            let k_len = k.iter().position(|&b| b == 0).unwrap_or(64);
            if &k[..k_len] == key {
                let v_len = v.iter().position(|&b| b == 0).unwrap_or(256);
                return Some(&v[..v_len]);
            }
        }
        None
    }
    
    fn set_env_var(&mut self, key: &[u8], value: &[u8]) {
        let mut key_array = [0u8; 64];
        let mut value_array = [0u8; 256];
        let key_len = key.len().min(63);
        let value_len = value.len().min(255);
        for i in 0..key_len { key_array[i] = key[i]; }
        for i in 0..value_len { value_array[i] = value[i]; }
        self.env_vars.push((key_array, value_array));
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
