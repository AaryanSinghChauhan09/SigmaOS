#![no_std]
#![no_main]

/// OOP-based Delta Updates for SigmaOS
/// Based on Ideas-999-Structured: Package, Build & Reproducibility Item 7
/// Implements binary diffs to minimize bandwidth for updates

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PatchID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DeltaError { Success = 0, InvalidPatch = 1, ApplyFailed = 2, GenerateFailed = 3 }

pub trait DeltaPatch {
    fn id(&self) -> PatchID;
    fn source_version(&self) -> &[u8];
    fn target_version(&self) -> &[u8];
    fn size(&self) -> usize;
}

#[repr(C)]
pub struct SimpleDeltaPatch {
    pub id: PatchID,
    pub source_version: [u8; 32],
    pub target_version: [u8; 32],
    pub size: AtomicUsize,
    pub operations: Vec<[u8; 256]>,
}

impl SimpleDeltaPatch {
    pub fn new(id: PatchID, source: &[u8], target: &[u8]) -> Self {
        let mut source_array = [0u8; 32];
        let mut target_array = [0u8; 32];
        let source_len = source.len().min(31);
        let target_len = target.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(source.as_ptr(), source_array.as_mut_ptr(), source_len);
            core::ptr::copy_nonoverlapping(target.as_ptr(), target_array.as_mut_ptr(), target_len);
        }
        SimpleDeltaPatch {
            id,
            source_version: source_array,
            target_version: target_array,
            size: AtomicUsize::new(0),
            operations: Vec::new(),
        }
    }
}

impl DeltaPatch for SimpleDeltaPatch {
    fn id(&self) -> PatchID { self.id }
    fn source_version(&self) -> &[u8] {
        let len = self.source_version.iter().position(|&b| b == 0).unwrap_or(32);
        &self.source_version[..len]
    }
    fn target_version(&self) -> &[u8] {
        let len = self.target_version.iter().position(|&b| b == 0).unwrap_or(32);
        &self.target_version[..len]
    }
    fn size(&self) -> usize { self.size.load(Ordering::SeqCst) }
}

pub trait DeltaGenerator {
    fn generate_delta(&mut self, old_data: &[u8], new_data: &[u8]) -> Result<PatchID, DeltaError>;
    fn optimize_delta(&mut self, patch_id: PatchID) -> Result<(), DeltaError>;
}

#[repr(C)]
pub struct SimpleDeltaGenerator {
    pub patches: Vec<Option<Box<dyn DeltaPatch>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDeltaGenerator {
    pub fn new() -> Self {
        SimpleDeltaGenerator {
            patches: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DeltaGenerator for SimpleDeltaGenerator {
    fn generate_delta(&mut self, old_data: &[u8], new_data: &[u8]) -> Result<PatchID, DeltaError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut patch = SimpleDeltaPatch::new(id, b"1.0.0", b"1.1.0");
        
        let mut ops = Vec::new();
        let min_len = old_data.len().min(new_data.len());
        
        for i in 0..min_len {
            if old_data[i] != new_data[i] {
                let mut op = [0u8; 256];
                op[0] = b'C';
                op[1] = i as u8;
                op[2] = new_data[i];
                ops.push(op);
            }
        }
        
        if new_data.len() > old_data.len() {
            for i in min_len..new_data.len() {
                let mut op = [0u8; 256];
                op[0] = b'A';
                op[1] = i as u8;
                op[2] = new_data[i];
                ops.push(op);
            }
        }
        
        patch.size.store(ops.len() * 256, Ordering::SeqCst);
        patch.operations = ops;
        
        self.patches.push(Some(Box::new(patch)));
        Ok(id)
    }
    
    fn optimize_delta(&mut self, patch_id: PatchID) -> Result<(), DeltaError> {
        for patch_option in &mut self.patches {
            if let Some(ref mut patch) = *patch_option {
                if patch.id() == patch_id {
                    return Ok(());
                }
            }
        }
        Err(DeltaError::InvalidPatch)
    }
}

pub trait DeltaApplier {
    fn apply_patch(&mut self, data: &mut [u8], patch_id: PatchID) -> Result<(), DeltaError>;
    fn verify_patch(&self, patch_id: PatchID) -> Result<bool, DeltaError>;
}

#[repr(C)]
pub struct SimpleDeltaApplier {
    pub generator: SimpleDeltaGenerator,
}

impl SimpleDeltaApplier {
    pub fn new(generator: SimpleDeltaGenerator) -> Self {
        SimpleDeltaApplier { generator }
    }
}

impl DeltaApplier for SimpleDeltaApplier {
    fn apply_patch(&mut self, data: &mut [u8], patch_id: PatchID) -> Result<(), DeltaError> {
        for patch_option in &self.generator.patches {
            if let Some(ref patch) = *patch_option {
                if patch.id() == patch_id {
                    if let SimpleDeltaPatch { ref operations, .. } = **patch {
                        for op in operations {
                            match op[0] {
                                b'C' => {
                                    let offset = op[1] as usize;
                                    if offset < data.len() {
                                        data[offset] = op[2];
                                    }
                                }
                                b'A' => {
                                    let offset = op[1] as usize;
                                    if offset < data.len() {
                                        data[offset] = op[2];
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    return Ok(());
                }
            }
        }
        Err(DeltaError::InvalidPatch)
    }
    
    fn verify_patch(&self, patch_id: PatchID) -> Result<bool, DeltaError> {
        for patch_option in &self.generator.patches {
            if let Some(ref patch) = *patch_option {
                if patch.id() == patch_id {
                    return Ok(true);
                }
            }
        }
        Err(DeltaError::InvalidPatch)
    }
}

pub trait BandwidthOptimizer {
    fn calculate_savings(&self, patch_id: PatchID, full_size: usize) -> usize;
    fn estimate_download_time(&self, patch_id: PatchID, bandwidth_kbps: usize) -> usize;
}

#[repr(C)]
pub struct SimpleBandwidthOptimizer {
    pub generator: SimpleDeltaGenerator,
}

impl SimpleBandwidthOptimizer {
    pub fn new(generator: SimpleDeltaGenerator) -> Self {
        SimpleBandwidthOptimizer { generator }
    }
}

impl BandwidthOptimizer for SimpleBandwidthOptimizer {
    fn calculate_savings(&self, patch_id: PatchID, full_size: usize) -> usize {
        for patch_option in &self.generator.patches {
            if let Some(ref patch) = *patch_option {
                if patch.id() == patch_id {
                    let patch_size = patch.size();
                    if patch_size < full_size {
                        return full_size - patch_size;
                    }
                }
            }
        }
        0
    }
    
    fn estimate_download_time(&self, patch_id: PatchID, bandwidth_kbps: usize) -> usize {
        for patch_option in &self.generator.patches {
            if let Some(ref patch) = *patch_option {
                if patch.id() == patch_id {
                    let patch_size = patch.size();
                    if bandwidth_kbps > 0 {
                        return (patch_size * 8) / bandwidth_kbps;
                    }
                }
            }
        }
        0
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
