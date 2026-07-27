#![no_std]
#![no_main]

/// OOP-based Key Management for SigmaOS
/// Based on Roadmap Item 16: Key management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type KeyID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KeyType { Symmetric = 0, Asymmetric = 1, HMAC = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KeyState { Active = 0, Revoked = 1, Expired = 2 }

pub trait Key {
    fn id(&self) -> KeyID;
    fn key_type(&self) -> KeyType;
    fn state(&self) -> KeyState;
    fn revoke(&mut self);
}

#[repr(C)]
pub struct SimpleKey {
    pub id: KeyID,
    pub key_type: KeyType,
    pub state: AtomicUsize,
    pub key_data: [u8; 64],
}

impl SimpleKey {
    pub fn new(id: KeyID, key_type: KeyType, key_data: &[u8]) -> Self {
        let mut key_array = [0u8; 64];
        let key_len = key_data.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(key_data.as_ptr(), key_array.as_mut_ptr(), key_len);
        }
        SimpleKey {
            id,
            key_type,
            state: AtomicUsize::new(KeyState::Active as usize),
            key_data: key_array,
        }
    }
}

impl Key for SimpleKey {
    fn id(&self) -> KeyID { self.id }
    fn key_type(&self) -> KeyType { self.key_type }
    fn state(&self) -> KeyState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    fn revoke(&mut self) { self.state.store(KeyState::Revoked as usize, Ordering::SeqCst); }
}

pub trait KeyManager {
    fn generate_key(&mut self, key_type: KeyType) -> Result<KeyID, KeyError>;
    fn store_key(&mut self, key: Box<dyn Key>) -> Result<KeyID, KeyError>;
    fn revoke_key(&mut self, id: KeyID) -> Result<(), KeyError>;
    fn get_key(&self, id: KeyID) -> Option<&dyn Key>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KeyError { Success = 0, KeyNotFound = 1, GenerationFailed = 2 }

pub struct SimpleKeyManager {
    keys: Vec<Option<Box<dyn Key>>>,
    next_id: AtomicUsize,
}

impl SimpleKeyManager {
    pub fn new() -> Self { SimpleKeyManager { keys: Vec::new(), next_id: AtomicUsize::new(1) } }
}

impl KeyManager for SimpleKeyManager {
    fn generate_key(&mut self, key_type: KeyType) -> Result<KeyID, KeyError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut key_data = [0u8; 64];
        for i in 0..64 { key_data[i] = (id as u8).wrapping_add(i as u8); }
        let key = SimpleKey::new(id, key_type, &key_data);
        self.keys.push(Some(Box::new(key)));
        Ok(id)
    }
    fn store_key(&mut self, key: Box<dyn Key>) -> Result<KeyID, KeyError> {
        let id = key.id();
        self.keys.push(Some(key));
        Ok(id)
    }
    fn revoke_key(&mut self, id: KeyID) -> Result<(), KeyError> {
        for key_option in &mut self.keys {
            if let Some(ref mut key) = *key_option {
                if key.id() == id {
                    key.revoke();
                    return Ok(());
                }
            }
        }
        Err(KeyError::KeyNotFound)
    }
    fn get_key(&self, id: KeyID) -> Option<&dyn Key> {
        for key_option in &self.keys {
            if let Some(ref key) = *key_option {
                if key.id() == id { return Some(key.as_ref()); }
            }
        }
        None
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
