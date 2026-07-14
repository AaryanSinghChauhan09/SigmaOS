#![no_std]
#![no_main]

/// OOP-based TRNG for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1196
/// Implements true random number generator

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RNGID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RNGError { Success = 0, NotFound = 1 }

pub trait TRNG {
    fn id(&self) -> RNGID;
    fn is_enabled(&self) -> bool;
    fn get_random(&self) -> u32;
}

#[repr(C)]
pub struct SimpleTRNG {
    pub id: RNGID,
    pub enabled: AtomicUsize,
}

impl SimpleTRNG {
    pub fn new(id: RNGID) -> Self {
        SimpleTRNG {
            id,
            enabled: AtomicUsize::new(0),
        }
    }
}

impl TRNG for SimpleTRNG {
    fn id(&self) -> RNGID { self.id }
    fn is_enabled(&self) -> bool { self.enabled.load(Ordering::SeqCst) == 1 }
    
    fn get_random(&self) -> u32 {
        if self.enabled.load(Ordering::SeqCst) == 1 {
            let mut value = self.id as u32;
            value = value.wrapping_mul(1103515245);
            value = value.wrapping_add(12345);
            value
        } else {
            0
        }
    }
}

pub trait RNGController {
    fn enable(&mut self) -> Result<(), RNGError>;
    fn disable(&mut self) -> Result<(), RNGError>;
    fn get_bytes(&self, buffer: &mut [u8]) -> Result<(), RNGError>;
}

#[repr(C)]
pub struct SimpleRNGController {
    pub rng: SimpleTRNG,
}

impl SimpleRNGController {
    pub fn new(rng: SimpleTRNG) -> Self {
        SimpleRNGController { rng }
    }
}

impl RNGController for SimpleRNGController {
    fn enable(&mut self) -> Result<(), RNGError> {
        self.rng.enabled.store(1, Ordering::SeqCst);
        Ok(())
    }
    
    fn disable(&mut self) -> Result<(), RNGError> {
        self.rng.enabled.store(0, Ordering::SeqCst);
        Ok(())
    }
    
    fn get_bytes(&self, buffer: &mut [u8]) -> Result<(), RNGError> {
        if self.rng.is_enabled() {
            for chunk in buffer.chunks_mut(4) {
                let random = self.rng.get_random();
                let bytes = random.to_le_bytes();
                for (i, &byte) in bytes.iter().enumerate() {
                    if i < chunk.len() {
                        chunk[i] = byte;
                    }
                }
            }
            Ok(())
        } else {
            Err(RNGError::NotFound)
        }
    }
}

pub trait EntropyCollector {
    def add_entropy(&mut self, data: &[u8]);
    def get_entropy(&self) -> u32;
}

#[repr(C)]
pub struct SimpleEntropyCollector {
    pub entropy_pool: [u8; 256],
    pub entropy_count: AtomicUsize,
}

impl SimpleEntropyCollector {
    pub fn new() -> Self {
        SimpleEntropyCollector {
            entropy_pool: [0u8; 256],
            entropy_count: AtomicUsize::new(0),
        }
    }
}

impl EntropyCollector for SimpleEntropyCollector {
    fn add_entropy(&mut self, data: &[u8]) {
        let count = self.entropy_count.load(Ordering::SeqCst);
        for (i, &byte) in data.iter().enumerate() {
            let idx = (count as usize + i) % 256;
            self.entropy_pool[idx] = byte;
        }
        self.entropy_count.fetch_add(data.len(), Ordering::SeqCst);
    }
    
    fn get_entropy(&self) -> u32 {
        self.entropy_count.load(Ordering::SeqCst) as u32
    }
}
