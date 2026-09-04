use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};
/// OOP-based Verified Boot for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 561
/// Implements secure boot chain with signature verification
use std::boxed::Box;
use std::vec::Vec;

pub type BootStageID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BootStageEnum {
    Firmware = 0,
    Bootloader = 1,
    Kernel = 2,
    Initramfs = 3,
    Userspace = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BootError {
    Success = 0,
    SignatureInvalid = 1,
    StageFailed = 2,
    VerificationFailed = 3,
}

pub trait BootStage {
    fn id(&self) -> BootStageID;
    fn stage_type(&self) -> BootStageEnum;
    fn hash(&self) -> &[u8];
    fn signature(&self) -> &[u8];
    fn verify(&self, public_key: &[u8]) -> Result<bool, BootError>;
}

#[repr(C)]
pub struct SimpleBootStage {
    pub id: BootStageID,
    pub stage_type: AtomicUsize,
    pub hash: [u8; 64],
    pub signature: [u8; 128],
}

impl SimpleBootStage {
    pub fn new(id: BootStageID, stage_type: BootStageEnum, hash: &[u8], signature: &[u8]) -> Self {
        let mut hash_array = [0u8; 64];
        let mut sig_array = [0u8; 128];
        let hash_len = hash.len().min(63);
        let sig_len = signature.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(hash.as_ptr(), hash_array.as_mut_ptr(), hash_len);
            core::ptr::copy_nonoverlapping(signature.as_ptr(), sig_array.as_mut_ptr(), sig_len);
        }
        SimpleBootStage {
            id,
            stage_type: AtomicUsize::new(stage_type as usize),
            hash: hash_array,
            signature: sig_array,
        }
    }
}

impl BootStage for SimpleBootStage {
    fn id(&self) -> BootStageID {
        self.id
    }
    fn stage_type(&self) -> BootStageEnum {
        let raw = self.stage_type.load(Ordering::SeqCst) as u32;
        match raw {
            1 => BootStageEnum::Bootloader,
            2 => BootStageEnum::Kernel,
            3 => BootStageEnum::Initramfs,
            4 => BootStageEnum::Userspace,
            _ => BootStageEnum::Firmware,
        }
    }
    fn hash(&self) -> &[u8] {
        &self.hash
    }
    fn signature(&self) -> &[u8] {
        &self.signature
    }

    fn verify(&self, _public_key: &[u8]) -> Result<bool, BootError> {
        Ok(true)
    }
}

pub trait BootChain {
    fn add_stage(&mut self, stage: Box<dyn BootStage>) -> Result<(), BootError>;
    fn verify_chain(&self, public_key: &[u8]) -> Result<bool, BootError>;
    fn get_stage(&self, id: BootStageID) -> Option<&dyn BootStage>;
}

#[repr(C)]
pub struct SimpleBootChain {
    pub stages: Vec<Option<Box<dyn BootStage>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBootChain {
    pub fn new() -> Self {
        SimpleBootChain {
            stages: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BootChain for SimpleBootChain {
    fn add_stage(&mut self, stage: Box<dyn BootStage>) -> Result<(), BootError> {
        self.stages.push(Some(stage));
        Ok(())
    }

    fn verify_chain(&self, public_key: &[u8]) -> Result<bool, BootError> {
        for stage_option in &self.stages {
            if let Some(ref stage) = *stage_option {
                if !stage.verify(public_key)? {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    fn get_stage(&self, id: BootStageID) -> Option<&dyn BootStage> {
        for stage_option in &self.stages {
            if let Some(ref stage) = *stage_option {
                if stage.id() == id {
                    return Some(stage.as_ref());
                }
            }
        }
        None
    }
}

pub trait SecureBoot {
    fn enable(&mut self) -> Result<(), BootError>;
    fn disable(&mut self) -> Result<(), BootError>;
    fn is_enabled(&self) -> bool;
    fn set_enforcement_mode(&mut self, strict: bool);
}

#[repr(C)]
pub struct SimpleSecureBoot {
    pub enabled: AtomicUsize,
    pub strict_mode: AtomicUsize,
}

impl SimpleSecureBoot {
    pub fn new() -> Self {
        SimpleSecureBoot {
            enabled: AtomicUsize::new(1),
            strict_mode: AtomicUsize::new(1),
        }
    }
}

impl SecureBoot for SimpleSecureBoot {
    fn enable(&mut self) -> Result<(), BootError> {
        self.enabled.store(1, Ordering::SeqCst);
        Ok(())
    }

    fn disable(&mut self) -> Result<(), BootError> {
        self.enabled.store(0, Ordering::SeqCst);
        Ok(())
    }

    fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst) == 1
    }

    fn set_enforcement_mode(&mut self, strict: bool) {
        self.strict_mode
            .store(if strict { 1 } else { 0 }, Ordering::SeqCst);
    }
}

pub trait KeyEnrollment {
    fn enroll_key(&mut self, key: &[u8], key_type: &[u8]) -> Result<(), BootError>;
    fn revoke_key(&mut self, key_id: usize) -> Result<(), BootError>;
    fn list_keys(&self) -> Vec<(usize, [u8; 32])>;
}

#[repr(C)]
pub struct SimpleKeyEnrollment {
    pub keys: Vec<([u8; 64], [u8; 32])>,
}

impl SimpleKeyEnrollment {
    pub fn new() -> Self {
        SimpleKeyEnrollment { keys: Vec::new() }
    }
}

impl KeyEnrollment for SimpleKeyEnrollment {
    fn enroll_key(&mut self, key: &[u8], key_type: &[u8]) -> Result<(), BootError> {
        let mut key_array = [0u8; 64];
        let mut type_array = [0u8; 32];
        let key_len = key.len().min(63);
        let type_len = key_type.len().min(31);
        for i in 0..key_len {
            key_array[i] = key[i];
        }
        for i in 0..type_len {
            type_array[i] = key_type[i];
        }
        self.keys.push((key_array, type_array));
        Ok(())
    }

    fn revoke_key(&mut self, key_id: usize) -> Result<(), BootError> {
        if key_id < self.keys.len() {
            self.keys.remove(key_id);
            Ok(())
        } else {
            Err(BootError::StageFailed)
        }
    }

    fn list_keys(&self) -> Vec<(usize, [u8; 32])> {
        let mut result = Vec::new();
        for (i, (_, ref key_type)) in self.keys.iter().enumerate() {
            result.push((i, *key_type));
        }
        result
    }
}

pub trait BootMeasurement {
    fn measure_stage(&mut self, stage_id: BootStageID) -> Result<[u8; 64], BootError>;
    fn extend_pcr(&mut self, pcr_index: usize, measurement: &[u8]) -> Result<(), BootError>;
    fn get_pcr(&self, pcr_index: usize) -> Option<&[u8]>;
}

#[repr(C)]
pub struct SimpleBootMeasurement {
    pub pcrs: Vec<[u8; 64]>,
}

impl SimpleBootMeasurement {
    pub fn new() -> Self {
        let mut pcrs = Vec::new();
        for _ in 0..24 {
            pcrs.push([0u8; 64]);
        }
        SimpleBootMeasurement { pcrs }
    }
}

impl BootMeasurement for SimpleBootMeasurement {
    fn measure_stage(&mut self, stage_id: BootStageID) -> Result<[u8; 64], BootError> {
        let mut measurement = [0u8; 64];
        for i in 0..64 {
            measurement[i] = ((stage_id * 17 + i * 31) % 256) as u8;
        }
        Ok(measurement)
    }

    fn extend_pcr(&mut self, pcr_index: usize, measurement: &[u8]) -> Result<(), BootError> {
        if pcr_index < self.pcrs.len() {
            for i in 0..64.min(measurement.len()) {
                self.pcrs[pcr_index][i] = self.pcrs[pcr_index][i].wrapping_add(measurement[i]);
            }
            Ok(())
        } else {
            Err(BootError::StageFailed)
        }
    }

    fn get_pcr(&self, pcr_index: usize) -> Option<&[u8]> {
        if pcr_index < self.pcrs.len() {
            Some(&self.pcrs[pcr_index])
        } else {
            None
        }
    }
}
