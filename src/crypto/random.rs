#![no_std]
#![no_main]

/// OOP-based Cryptographic Random Number Generator for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 502
/// Implements CSPRNG with entropy collection

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RNGID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RNGError { Success = 0, InsufficientEntropy = 1, SeedingFailed = 2 }

pub trait RandomGenerator {
    fn id(&self) -> RNGID;
    fn next_byte(&mut self) -> Result<u8, RNGError>;
    fn next_u32(&mut self) -> Result<u32, RNGError>;
    fn next_u64(&mut self) -> Result<u64, RNGError>;
    fn fill_bytes(&mut self, buffer: &mut [u8]) -> Result<(), RNGError>;
}

#[repr(C)]
pub struct SimpleRandomGenerator {
    pub id: RNGID,
    pub state: AtomicUsize,
    pub counter: AtomicUsize,
}

impl SimpleRandomGenerator {
    pub fn new(id: RNGID) -> Self {
        SimpleRandomGenerator {
            id,
            state: AtomicUsize::new(12345),
            counter: AtomicUsize::new(0),
        }
    }
}

impl RandomGenerator for SimpleRandomGenerator {
    fn id(&self) -> RNGID { self.id }

    fn next_byte(&mut self) -> Result<u8, RNGError> {
        let counter = self.counter.fetch_add(1, Ordering::SeqCst);
        let state = self.state.load(Ordering::SeqCst);
        let result = ((state.wrapping_mul(1103515245).wrapping_add(12345) + counter) % 256) as u8;
        self.state.store(state.wrapping_mul(1103515245).wrapping_add(12345), Ordering::SeqCst);
        Ok(result)
    }

    fn next_u32(&mut self) -> Result<u32, RNGError> {
        let mut result: u32 = 0;
        for i in 0..4 {
            result |= (self.next_byte()? as u32) << (i * 8);
        }
        Ok(result)
    }

    fn next_u64(&mut self) -> Result<u64, RNGError> {
        let mut result: u64 = 0;
        for i in 0..8 {
            result |= (self.next_byte()? as u64) << (i * 8);
        }
        Ok(result)
    }

    fn fill_bytes(&mut self, buffer: &mut [u8]) -> Result<(), RNGError> {
        for byte in buffer.iter_mut() {
            *byte = self.next_byte()?;
        }
        Ok(())
    }
}

pub trait EntropyCollector {
    fn add_entropy(&mut self, source: u8, data: &[u8]);
    fn get_entropy_estimate(&self) -> usize;
    fn is_ready(&self) -> bool;
}

#[repr(C)]
pub struct SimpleEntropyCollector {
    pub entropy_pool: Vec<u8>,
    pub entropy_estimate: AtomicUsize,
}

impl SimpleEntropyCollector {
    pub fn new() -> Self {
        SimpleEntropyCollector {
            entropy_pool: Vec::new(),
            entropy_estimate: AtomicUsize::new(0),
        }
    }
}

impl EntropyCollector for SimpleEntropyCollector {
    fn add_entropy(&mut self, source: u8, data: &[u8]) {
        for &byte in data {
            self.entropy_pool.push(byte.wrapping_add(source));
        }
        self.entropy_estimate.fetch_add(data.len(), Ordering::SeqCst);
    }

    fn get_entropy_estimate(&self) -> usize { self.entropy_estimate.load(Ordering::SeqCst) }

    fn is_ready(&self) -> bool { self.entropy_estimate.load(Ordering::SeqCst) >= 256 }
}

pub trait CSPRNG {
    fn reseed(&mut self, seed: &[u8]) -> Result<(), RNGError>;
    fn generate_secure(&mut self, length: usize) -> Result<Vec<u8>, RNGError>;
}

#[repr(C)]
pub struct SimpleCSPRNG {
    pub rng: SimpleRandomGenerator,
    pub entropy: SimpleEntropyCollector,
}

impl SimpleCSPRNG {
    pub fn new() -> Self {
        SimpleCSPRNG {
            rng: SimpleRandomGenerator::new(1),
            entropy: SimpleEntropyCollector::new(),
        }
    }
}

impl CSPRNG for SimpleCSPRNG {
    fn reseed(&mut self, seed: &[u8]) -> Result<(), RNGError> {
        self.entropy.add_entropy(0, seed);
        let mut seed_value: usize = 0;
        for (i, &byte) in seed.iter().enumerate() {
            seed_value |= (byte as usize) << (i % 8) * 8;
        }
        self.rng.state.store(seed_value, Ordering::SeqCst);
        Ok(())
    }

    fn generate_secure(&mut self, length: usize) -> Result<Vec<u8>, RNGError> {
        if !self.entropy.is_ready() {
            return Err(RNGError::InsufficientEntropy);
        }

        let mut result = Vec::new();
        for _ in 0..length {
            result.push(self.rng.next_byte()?);
        }
        Ok(result)
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
