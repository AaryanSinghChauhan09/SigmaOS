#![no_std]
#![no_main]

/// OOP-based Protocol Handler for SigmaOS
/// Based on Ideas-999-Structured: Integration & Interoperability Item 896
/// Implements protocol handlers for various protocols

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ProtocolID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ProtocolType { HTTP = 0, FTP = 1, SSH = 2, SMB = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ProtocolError { Success = 0, NotFound = 1, Unsupported = 2 }

pub trait ProtocolHandler {
    fn id(&self) -> ProtocolID;
    fn protocol_type(&self) -> ProtocolType;
    fn scheme(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleProtocolHandler {
    pub id: ProtocolID,
    pub protocol_type: AtomicUsize,
    pub scheme: [u8; 16],
}

impl SimpleProtocolHandler {
    pub fn new(id: ProtocolID, protocol_type: ProtocolType, scheme: &[u8]) -> Self {
        let mut scheme_array = [0u8; 16];
        let scheme_len = scheme.len().min(15);
        for i in 0..scheme_len {
            scheme_array[i] = scheme[i];
        }
        SimpleProtocolHandler {
            id,
            protocol_type: AtomicUsize::new(protocol_type as usize),
            scheme: scheme_array,
        }
    }
}

impl ProtocolHandler for SimpleProtocolHandler {
    fn id(&self) -> ProtocolID { self.id }
    fn protocol_type(&self) -> ProtocolType { unsafe { core::mem::transmute(self.protocol_type.load(Ordering::SeqCst)) } }
    fn scheme(&self) -> &[u8] {
        let len = self.scheme.iter().position(|&b| b == 0).unwrap_or(16);
        &self.scheme[..len]
    }
}

pub trait ProtocolManager {
    fn register_protocol(&mut self, protocol_type: ProtocolType, scheme: &[u8]) -> Result<ProtocolID, ProtocolError>;
    fn unregister_protocol(&mut self, id: ProtocolID) -> Result<(), ProtocolError>;
    fn get_handler(&self, scheme: &[u8]) -> Option<&dyn ProtocolHandler>;
    def open_url(&self, url: &[u8]) -> Result<(), ProtocolError>;
}

#[repr(C)]
pub struct SimpleProtocolManager {
    pub handlers: Vec<Option<Box<dyn ProtocolHandler>>>,
    pub next_id: AtomicUsize,
}

impl SimpleProtocolManager {
    pub fn new() -> Self {
        SimpleProtocolManager {
            handlers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ProtocolManager for SimpleProtocolManager {
    fn register_protocol(&mut self, protocol_type: ProtocolType, scheme: &[u8]) -> Result<ProtocolID, ProtocolError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let handler = SimpleProtocolHandler::new(id, protocol_type, scheme);
        self.handlers.push(Some(Box::new(handler)));
        Ok(id)
    }
    
    fn unregister_protocol(&mut self, id: ProtocolID) -> Result<(), ProtocolError> {
        for handler_option in &mut self.handlers {
            if let Some(ref handler) = *handler_option {
                if handler.id() == id {
                    return Ok(());
                }
            }
        }
        Err(ProtocolError::NotFound)
    }
    
    fn get_handler(&self, scheme: &[u8]) -> Option<&dyn ProtocolHandler> {
        for handler_option in &self.handlers {
            if let Some(ref handler) = *handler_option {
                if handler.scheme() == scheme { return Some(handler.as_ref()); }
            }
        }
        None
    }
    
    fn open_url(&self, url: &[u8]) -> Result<(), ProtocolError> {
        for handler_option in &self.handlers {
            if let Some(ref handler) = *handler_option {
                let scheme = handler.scheme();
                if url.starts_with(scheme) {
                    return Ok(());
                }
            }
        }
        Err(ProtocolError::Unsupported)
    }
}

pub trait URIResolver {
    fn resolve(&self, uri: &[u8]) -> Result<Vec<u8>, ProtocolError>;
    def register_scheme(&mut self, scheme: &[u8], handler: ProtocolID);
}

#[repr(C)]
pub struct SimpleURIResolver {
    pub schemes: Vec<([u8; 16], ProtocolID)>,
}

impl SimpleURIResolver {
    pub fn new() -> Self {
        SimpleURIResolver {
            schemes: Vec::new(),
        }
    }
}

impl URIResolver for SimpleURIResolver {
    fn resolve(&self, _uri: &[u8]) -> Result<Vec<u8>, ProtocolError> {
        let mut result = Vec::new();
        result.push(0x00);
        Ok(result)
    }
    
    fn register_scheme(&mut self, scheme: &[u8], handler: ProtocolID) {
        let mut scheme_array = [0u8; 16];
        let scheme_len = scheme.len().min(15);
        for i in 0..scheme_len {
            scheme_array[i] = scheme[i];
        }
        self.schemes.push((scheme_array, handler));
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
