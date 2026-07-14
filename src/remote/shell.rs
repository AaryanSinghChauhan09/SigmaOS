#![no_std]
#![no_main]

/// OOP-based Remote Shell for SigmaOS
/// Based on Ideas-999-Structured: Cloud & Remote Item 966
/// Implements remote shell access

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ShellID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ShellError { Success = 0, NotFound = 1, CommandFailed = 2 }

pub trait RemoteShell {
    fn id(&self) -> ShellID;
    fn host(&self) -> &[u8];
    def execute(&self, command: &[u8]) -> Result<Vec<u8>, ShellError>;
}

#[repr(C)]
pub struct SimpleRemoteShell {
    pub id: ShellID,
    pub host: [u8; 128],
}

impl SimpleRemoteShell {
    pub fn new(id: ShellID, host: &[u8]) -> Self {
        let mut host_array = [0u8; 128];
        let host_len = host.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(host.as_ptr(), host_array.as_mut_ptr(), host_len);
        }
        SimpleRemoteShell {
            id,
            host: host_array,
        }
    }
}

impl RemoteShell for SimpleRemoteShell {
    fn id(&self) -> ShellID { self.id }
    fn host(&self) -> &[u8] {
        let len = self.host.iter().position(|&b| b == 0).unwrap_or(128);
        &self.host[..len]
    }
    
    fn execute(&self, command: &[u8]) -> Result<Vec<u8>, ShellError> {
        let mut output = Vec::new();
        for &byte in command {
            output.push(byte);
        }
        output.push(b'\n');
        Ok(output)
    }
}

pub trait ShellManager {
    fn connect(&mut self, host: &[u8], port: u16) -> Result<ShellID, ShellError>;
    fn disconnect(&mut self, id: ShellID) -> Result<(), ShellError>;
    fn get_shell(&self, id: ShellID) -> Option<&dyn RemoteShell>;
}

#[repr(C)]
pub struct SimpleShellManager {
    pub shells: Vec<Option<Box<dyn RemoteShell>>>,
    pub next_id: AtomicUsize,
}

impl SimpleShellManager {
    pub fn new() -> Self {
        SimpleShellManager {
            shells: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ShellManager for SimpleShellManager {
    fn connect(&mut self, host: &[u8], _port: u16) -> Result<ShellID, ShellError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let shell = SimpleRemoteShell::new(id, host);
        self.shells.push(Some(Box::new(shell)));
        Ok(id)
    }
    
    fn disconnect(&mut self, id: ShellID) -> Result<(), ShellError> {
        for shell_option in &mut self.shells {
            if let Some(ref shell) = *shell_option {
                if shell.id() == id {
                    return Ok(());
                }
            }
        }
        Err(ShellError::NotFound)
    }
    
    fn get_shell(&self, id: ShellID) -> Option<&dyn RemoteShell> {
        for shell_option in &self.shells {
            if let Some(ref shell) = *shell_option {
                if shell.id() == id { return Some(shell.as_ref()); }
            }
        }
        None
    }
}

pub trait FileTransfer {
    fn upload(&self, shell_id: ShellID, local_path: &[u8], remote_path: &[u8]) -> Result<(), ShellError>;
    fn download(&self, shell_id: ShellID, remote_path: &[u8], local_path: &[u8]) -> Result<(), ShellError>;
}

#[repr(C)]
pub struct SimpleFileTransfer {
    pub manager: SimpleShellManager,
}

impl SimpleFileTransfer {
    pub fn new(manager: SimpleShellManager) -> Self {
        SimpleFileTransfer { manager }
    }
}

impl FileTransfer for SimpleFileTransfer {
    fn upload(&self, shell_id: ShellID, _local_path: &[u8], _remote_path: &[u8]) -> Result<(), ShellError> {
        if self.manager.get_shell(shell_id).is_some() {
            Ok(())
        } else {
            Err(ShellError::NotFound)
        }
    }
    
    fn download(&self, shell_id: ShellID, _remote_path: &[u8], _local_path: &[u8]) -> Result<(), ShellError> {
        if self.manager.get_shell(shell_id).is_some() {
            Ok(())
        } else {
            Err(ShellError::NotFound)
        }
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
