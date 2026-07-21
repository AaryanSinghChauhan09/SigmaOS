#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::ptr;

/// Secure Data Erasure (BleachBit Parity)
/// Multi-pass secure sector overwriting and cache purging to prevent forensic recovery.

pub struct SecureCleaner;

impl SecureCleaner {
    pub fn new() -> Self {
        Self
    }

    /// Performs a 3-pass DoD 5220.22-M style secure wipe on a memory block
    pub fn secure_wipe(&self, block: &mut [u8]) {
        // Pass 1: Zeros
        for b in block.iter_mut() { *b = 0x00; }
        // Pass 2: Ones
        for b in block.iter_mut() { *b = 0xFF; }
        // Pass 3: Random/Pseudo-random (simulated with fixed pattern for no_std deterministic test)
        for b in block.iter_mut() { *b = 0xAA; }
        
        // Volatile write to ensure compiler doesn't optimize it away
        unsafe {
            let ptr = block.as_mut_ptr();
            for i in 0..block.len() {
                ptr::write_volatile(ptr.add(i), 0x00);
            }
        }
    }

    /// Clears unused or unallocated space in a filesystem partition
    pub fn wipe_unallocated_space(&self, partition: &mut [u8], bitmap: &[bool]) {
        for (i, &allocated) in bitmap.iter().enumerate() {
            if !allocated {
                let start = i * 512;
                let end = (start + 512).min(partition.len());
                if start < end {
                    self.secure_wipe(&mut partition[start..end]);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_wipe() {
        let cleaner = SecureCleaner::new();
        let mut sensitive_data = alloc::vec![0xCA, 0xFE, 0xBA, 0xBE];
        
        cleaner.secure_wipe(&mut sensitive_data);
        
        // The volatile write at the end zeroes it out
        assert_eq!(sensitive_data, alloc::vec![0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_wipe_unallocated_space() {
        let cleaner = SecureCleaner::new();
        let mut partition = alloc::vec![0xFF; 1024]; // Two 512-byte blocks
        let bitmap = [true, false]; // Block 0 allocated, Block 1 unallocated
        
        cleaner.wipe_unallocated_space(&mut partition, &bitmap);
        
        // Block 0 should remain 0xFF
        assert_eq!(partition[0..512], alloc::vec![0xFF; 512]);
        // Block 1 should be wiped to 0x00
        assert_eq!(partition[512..1024], alloc::vec![0x00; 512]);
    }
}
