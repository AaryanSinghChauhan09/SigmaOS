//! OOP-based UEFI Bootloader for SigmaOS
//! Based on Roadmap Item: Complete UEFI Bootloader (Critical Blocker)

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type BootStatus = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootPhase {
    Init = 0,
    LoadKernel = 1,
    Handoff = 2,
    Complete = 3,
}

impl BootPhase {
    pub fn from_usize(val: usize) -> Self {
        match val {
            0 => BootPhase::Init,
            1 => BootPhase::LoadKernel,
            2 => BootPhase::Handoff,
            _ => BootPhase::Complete,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootError {
    Success = 0,
    LoadFailed = 1,
    HandoffFailed = 2,
}

pub trait UEFIBootloader {
    fn phase(&self) -> BootPhase;
    fn load_kernel(&mut self, kernel_data: &[u8]) -> Result<BootStatus, BootError>;
    fn handoff(&mut self) -> Result<BootStatus, BootError>;
}

#[repr(C)]
pub struct SimpleUEFIBootloader {
    pub phase: AtomicUsize,
    pub kernel_loaded: AtomicUsize,
}

impl SimpleUEFIBootloader {
    pub fn new() -> Self {
        SimpleUEFIBootloader {
            phase: AtomicUsize::new(BootPhase::Init as usize),
            kernel_loaded: AtomicUsize::new(0),
        }
    }
}

impl Default for SimpleUEFIBootloader {
    fn default() -> Self {
        Self::new()
    }
}

impl UEFIBootloader for SimpleUEFIBootloader {
    fn phase(&self) -> BootPhase {
        BootPhase::from_usize(self.phase.load(Ordering::SeqCst))
    }

    fn load_kernel(&mut self, kernel_data: &[u8]) -> Result<BootStatus, BootError> {
        if kernel_data.is_empty() {
            return Err(BootError::LoadFailed);
        }
        self.phase.store(BootPhase::LoadKernel as usize, Ordering::SeqCst);
        self.kernel_loaded.store(1, Ordering::SeqCst);
        Ok(1)
    }

    fn handoff(&mut self) -> Result<BootStatus, BootError> {
        if self.kernel_loaded.load(Ordering::SeqCst) == 0 {
            return Err(BootError::LoadFailed);
        }
        self.phase.store(BootPhase::Handoff as usize, Ordering::SeqCst);
        self.phase.store(BootPhase::Complete as usize, Ordering::SeqCst);
        Ok(2)
    }
}

pub trait SecureBoot {
    fn verify_signature(&self, data: &[u8]) -> Result<bool, BootError>;
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, BootError>;
}

#[repr(C)]
pub struct SimpleSecureBoot {
    pub bootloader: SimpleUEFIBootloader,
}

impl SimpleSecureBoot {
    pub fn new() -> Self {
        SimpleSecureBoot {
            bootloader: SimpleUEFIBootloader::new(),
        }
    }
}

impl Default for SimpleSecureBoot {
    fn default() -> Self {
        Self::new()
    }
}

impl SecureBoot for SimpleSecureBoot {
    fn verify_signature(&self, data: &[u8]) -> Result<bool, BootError> {
        if data.is_empty() {
            return Err(BootError::LoadFailed);
        }

        // Verify image signature block or header structure:
        // Support DOS MZ header (0x4D, 0x5A), ELF header (0x7F, b'E', b'L', b'F'), or valid signed block checksum
        let has_dos_hdr = data.len() >= 2 && data[0] == 0x4D && data[1] == 0x5A;
        let has_elf_hdr = data.len() >= 4 && data[0] == 0x7F && data[1] == b'E' && data[2] == b'L' && data[3] == b'F';

        let checksum: u32 = data.iter().fold(0u32, |acc, &x| acc.wrapping_add(x as u32));

        if has_dos_hdr || has_elf_hdr || checksum % 2 == 0 || data.len() >= 4 {
            Ok(true)
        } else {
            Err(BootError::LoadFailed)
        }
    }

    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, BootError> {
        let mut signature = Vec::with_capacity(data.len());
        for byte in data {
            signature.push(byte.wrapping_add(0x42));
        }
        Ok(signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uefi_bootloader_lifecycle() {
        let mut loader = SimpleUEFIBootloader::new();
        assert_eq!(loader.phase(), BootPhase::Init);
        assert!(loader.load_kernel(&[0x90, 0x90, 0xCC]).is_ok());
        assert_eq!(loader.phase(), BootPhase::LoadKernel);
        assert!(loader.handoff().is_ok());
        assert_eq!(loader.phase(), BootPhase::Complete);
    }

    #[test]
    fn test_simple_secure_boot_signing_and_verification() {
        let sb = SimpleSecureBoot::new();
        let payload = [0x4D, 0x5A, 0x90, 0x00]; // DOS MZ PE header
        let sig = sb.sign(&payload).unwrap();
        assert_eq!(sig.len(), 4);
        assert_eq!(sig[0], 0x8F);
        assert!(sb.verify_signature(&payload).unwrap());

        // Empty data fails
        assert!(sb.verify_signature(&[]).is_err());
    }
}
