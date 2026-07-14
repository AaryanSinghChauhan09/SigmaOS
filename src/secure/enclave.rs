#![no_std]
#![no_main]

/// OOP-based Secure Enclave for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 592
/// Implements secure enclave for sensitive operations

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type EnclaveID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EnclaveError { Success = 0, NotFound = 1, OperationFailed = 2 }

pub trait SecureEnclave {
    fn id(&self) -> EnclaveID;
    fn is_active(&self) -> bool;
    fn memory_size(&self) -> u32;
}

#[repr(C)]
pub struct SimpleSecureEnclave {
    pub id: EnclaveID,
    pub active: AtomicUsize,
    pub memory_size: AtomicUsize,
}

impl SimpleSecureEnclave {
    pub fn new(id: EnclaveID, memory_size: u32) -> Self {
        SimpleSecureEnclave {
            id,
            active: AtomicUsize::new(1),
            memory_size: AtomicUsize::new(memory_size as usize),
        }
    }
}

impl SecureEnclave for SimpleSecureEnclave {
    fn id(&self) -> EnclaveID { self.id }
    fn is_active(&self) -> bool { self.active.load(Ordering::SeqCst) == 1 }
    fn memory_size(&self) -> u32 { self.memory_size.load(Ordering::SeqCst) as u32 }
}

pub trait EnclaveOperations {
    fn create_enclave(&mut self, memory_size: u32) -> Result<EnclaveID, EnclaveError>;
    fn destroy_enclave(&mut self, id: EnclaveID) -> Result<(), EnclaveError>;
    fn execute(&mut self, enclave_id: EnclaveID, code: &[u8]) -> Result<Vec<u8>, EnclaveError>;
}

#[repr(C)]
pub struct SimpleEnclaveOperations {
    pub enclaves: Vec<Option<Box<dyn SecureEnclave>>>,
    pub next_id: AtomicUsize,
}

impl SimpleEnclaveOperations {
    pub fn new() -> Self {
        SimpleEnclaveOperations {
            enclaves: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl EnclaveOperations for SimpleEnclaveOperations {
    fn create_enclave(&mut self, memory_size: u32) -> Result<EnclaveID, EnclaveError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let enclave = SimpleSecureEnclave::new(id, memory_size);
        self.enclaves.push(Some(Box::new(enclave)));
        Ok(id)
    }
    
    fn destroy_enclave(&mut self, id: EnclaveID) -> Result<(), EnclaveError> {
        for enclave_option in &mut self.enclaves {
            if let Some(ref enclave) = *enclave_option {
                if enclave.id() == id {
                    return Ok(());
                }
            }
        }
        Err(EnclaveError::NotFound)
    }
    
    fn execute(&mut self, enclave_id: EnclaveID, _code: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        for enclave_option in &self.enclaves {
            if let Some(ref enclave) = *enclave_option {
                if enclave.id() == enclave_id && enclave.is_active() {
                    let mut result = Vec::new();
                    result.push(0x00);
                    return Ok(result);
                }
            }
        }
        Err(EnclaveError::NotFound)
    }
}

pub trait SecureStorage {
    fn store_secret(&mut self, enclave_id: EnclaveID, key: &[u8], value: &[u8]) -> Result<(), EnclaveError>;
    fn retrieve_secret(&self, enclave_id: EnclaveID, key: &[u8]) -> Result<Vec<u8>, EnclaveError>;
}

#[repr(C)]
pub struct SimpleSecureStorage {
    pub storage: Vec<(EnclaveID, [u8; 64], Vec<u8>)>,
}

impl SimpleSecureStorage {
    pub fn new() -> Self {
        SimpleSecureStorage {
            storage: Vec::new(),
        }
    }
}

impl SecureStorage for SimpleSecureStorage {
    fn store_secret(&mut self, enclave_id: EnclaveID, key: &[u8], value: &[u8]) -> Result<(), EnclaveError> {
        let mut key_array = [0u8; 64];
        let key_len = key.len().min(63);
        for i in 0..key_len {
            key_array[i] = key[i];
        }
        let value_vec = value.to_vec();
        self.storage.push((enclave_id, key_array, value_vec));
        Ok(())
    }
    
    fn retrieve_secret(&self, enclave_id: EnclaveID, key: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        for &(id, ref stored_key, ref value) in &self.storage {
            if id == enclave_id {
                let stored_len = stored_key.iter().position(|&b| b == 0).unwrap_or(64);
                if &stored_key[..stored_len] == key {
                    return Ok(value.clone());
                }
            }
        }
        Err(EnclaveError::NotFound)
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
    fn clone(&self) -> Vec<T> {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                let item = core::ptr::read(self.data.add(i));
                new_vec.push(item);
            }
        }
        new_vec
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
