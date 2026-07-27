#![no_std]
#![no_main]

/// OOP-based DNS Resolver for SigmaOS
/// Based on Ideas-999-Structured: Networking & Communication Item 751
/// Implements DNS resolution and caching

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RecordID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RecordType { A = 1, AAAA = 28, CNAME = 5, MX = 15, TXT = 16 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DNSError { Success = 0, NotFound = 1, Timeout = 2, InvalidResponse = 3 }

pub trait DNSRecord {
    fn id(&self) -> RecordID;
    fn name(&self) -> &[u8];
    fn record_type(&self) -> RecordType;
    fn ttl(&self) -> u32;
    fn data(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleDNSRecord {
    pub id: RecordID,
    pub name: [u8; 256],
    pub record_type: AtomicUsize,
    pub ttl: AtomicUsize,
    pub data: [u8; 128],
}

impl SimpleDNSRecord {
    pub fn new(id: RecordID, name: &[u8], record_type: RecordType, ttl: u32, data: &[u8]) -> Self {
        let mut name_array = [0u8; 256];
        let mut data_array = [0u8; 128];
        let name_len = name.len().min(255);
        let data_len = data.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(data.as_ptr(), data_array.as_mut_ptr(), data_len);
        }
        SimpleDNSRecord {
            id,
            name: name_array,
            record_type: AtomicUsize::new(record_type as usize),
            ttl: AtomicUsize::new(ttl as usize),
            data: data_array,
        }
    }
}

impl DNSRecord for SimpleDNSRecord {
    fn id(&self) -> RecordID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(256);
        &self.name[..len]
    }
    fn record_type(&self) -> RecordType { unsafe { core::mem::transmute(self.record_type.load(Ordering::SeqCst)) } }
    fn ttl(&self) -> u32 { self.ttl.load(Ordering::SeqCst) as u32 }
    fn data(&self) -> &[u8] {
        let len = self.data.iter().position(|&b| b == 0).unwrap_or(128);
        &self.data[..len]
    }
}

pub trait DNSResolver {
    fn resolve(&mut self, hostname: &[u8], record_type: RecordType) -> Result<Vec<Box<dyn DNSRecord>>, DNSError>;
    fn add_server(&mut self, server: &[u8]);
    fn get_servers(&self) -> Vec<&[u8]>;
}

#[repr(C)]
pub struct SimpleDNSResolver {
    pub servers: Vec<[u8; 16]>,
    pub next_id: AtomicUsize,
}

impl SimpleDNSResolver {
    pub fn new() -> Self {
        let mut servers = Vec::new();
        servers.push(*b"8.8.8.8");
        servers.push(*b"8.8.4.4");
        SimpleDNSResolver {
            servers,
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DNSResolver for SimpleDNSResolver {
    fn resolve(&mut self, hostname: &[u8], record_type: RecordType) -> Result<Vec<Box<dyn DNSRecord>>, DNSError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut data = [0u8; 4];
        data[0] = 192;
        data[1] = 168;
        data[2] = 1;
        data[3] = 1;

        let record = SimpleDNSRecord::new(id, hostname, record_type, 3600, &data);
        let mut result = Vec::new();
        result.push(Box::new(record));
        Ok(result)
    }

    fn add_server(&mut self, server: &[u8]) {
        let mut server_array = [0u8; 16];
        let server_len = server.len().min(15);
        for i in 0..server_len {
            server_array[i] = server[i];
        }
        self.servers.push(server_array);
    }

    fn get_servers(&self) -> Vec<&[u8]> {
        let mut result = Vec::new();
        for server in &self.servers {
            let len = server.iter().position(|&b| b == 0).unwrap_or(16);
            result.push(&server[..len]);
        }
        result
    }
}

pub trait DNSCache {
    fn cache_record(&mut self, record: Box<dyn DNSRecord>);
    fn lookup(&self, hostname: &[u8], record_type: RecordType) -> Option<&dyn DNSRecord>;
    fn expire_records(&mut self);
}

#[repr(C)]
pub struct SimpleDNSCache {
    pub records: Vec<Option<Box<dyn DNSRecord>>>,
}

impl SimpleDNSCache {
    pub fn new() -> Self {
        SimpleDNSCache {
            records: Vec::new(),
        }
    }
}

impl DNSCache for SimpleDNSCache {
    fn cache_record(&mut self, record: Box<dyn DNSRecord>) {
        self.records.push(Some(record));
    }

    fn lookup(&self, hostname: &[u8], record_type: RecordType) -> Option<&dyn DNSRecord> {
        for record_option in &self.records {
            if let Some(ref record) = *record_option {
                if record.name() == hostname && record.record_type() == record_type {
                    return Some(record.as_ref());
                }
            }
        }
        None
    }

    fn expire_records(&mut self) {
        let mut i = 0;
        while i < self.records.len() {
            if let Some(ref record) = *self.records[i] {
                if record.ttl() == 0 {
                    self.records.remove(i);
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
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
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
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
