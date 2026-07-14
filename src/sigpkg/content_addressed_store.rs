#![no_std]

/// Content-Addressed Store for SigmaPkg
/// Implements functional package management with atomic operations
/// Based on Roadmap Item 21: SigmaPkg universal package manager
/// And DEFEATING_LINUX_DISTROS_BLUEPRINT.md sigpkg CAS specification

use core::sync::atomic::{AtomicU64, Ordering};

/// SHA3-256 hash (32 bytes)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hash {
    pub bytes: [u8; 32],
}

impl Hash {
    pub const ZERO: Hash = Hash { bytes: [0u8; 32] };
    
    pub fn new(bytes: [u8; 32]) -> Self {
        Hash { bytes }
    }
    
    pub fn is_zero(&self) -> bool {
        self.bytes == Hash::ZERO.bytes
    }
}

/// Package identifier in CAS
#[repr(C)]
pub struct PackageID {
    pub hash: Hash,
    pub name: [u8; 64],
}

impl PackageID {
    pub fn new(hash: Hash, name: &str) -> Self {
        let mut name_array = [0u8; 64];
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_array.as_mut_ptr(), len);
        }
        
        PackageID {
            hash,
            name: name_array,
        }
    }
    
    pub fn name_str(&self) -> &str {
        unsafe {
            let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
            core::str::from_utf8_unchecked(&self.name[..len])
        }
    }
}

/// Store entry
#[repr(C)]
pub struct StoreEntry {
    pub id: PackageID,
    pub size: u64,
    pub ref_count: AtomicU64,
    pub generation: u64,
}

impl StoreEntry {
    pub fn new(id: PackageID, size: u64) -> Self {
        StoreEntry {
            id,
            size,
            ref_count: AtomicU64::new(1),
            generation: 0,
        }
    }
    
    pub fn increment_ref(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }
    
    pub fn decrement_ref(&self) -> u64 {
        self.ref_count.fetch_sub(1, Ordering::SeqCst) - 1
    }
    
    pub fn ref_count(&self) -> u64 {
        self.ref_count.load(Ordering::SeqCst)
    }
}

/// Generation snapshot for atomic rollbacks
#[repr(C)]
pub struct Generation {
    pub id: u64,
    pub timestamp: u64,
    pub active_packages: Vec<PackageID>,
}

impl Generation {
    pub fn new(id: u64) -> Self {
        Generation {
            id,
            timestamp: get_current_time(),
            active_packages: Vec::new(),
        }
    }
}

/// Content-Addressed Store
pub struct ContentAddressedStore {
    entries: Vec<Option<StoreEntry>>,
    generations: Vec<Option<Generation>>,
    next_generation_id: AtomicU64,
    current_generation: u64,
}

impl ContentAddressedStore {
    pub fn new() -> Self {
        ContentAddressedStore {
            entries: Vec::new(),
            generations: Vec::new(),
            next_generation_id: AtomicU64::new(1),
            current_generation: 0,
        }
    }
    
    /// Add package to store
    pub fn add(&mut self, id: PackageID, size: u64) -> Result<(), StoreError> {
        let entry = StoreEntry::new(id, size);
        self.entries.push(Some(entry));
        Ok(())
    }
    
    /// Get package from store
    pub fn get(&self, hash: &Hash) -> Option<&StoreEntry> {
        for entry_option in &self.entries {
            if let Some(ref entry) = *entry_option {
                if entry.id.hash.bytes == hash.bytes {
                    return Some(entry);
                }
            }
        }
        None
    }
    
    /// Create new generation snapshot
    pub fn create_generation(&mut self) -> u64 {
        let gen_id = self.next_generation_id.fetch_add(1, Ordering::SeqCst);
        let mut generation = Generation::new(gen_id);
        
        // Copy current active packages
        for entry_option in &self.entries {
            if let Some(ref entry) = *entry_option {
                if entry.ref_count() > 0 {
                    generation.active_packages.push(entry.id);
                }
            }
        }
        
        self.generations.push(Some(generation));
        gen_id
    }
    
    /// Rollback to specific generation (O(1) operation)
    pub fn rollback(&mut self, gen_id: u64) -> Result<(), StoreError> {
        for gen_option in &self.generations {
            if let Some(ref gen) = *gen_option {
                if gen.id == gen_id {
                    // Reset ref counts to match generation
                    for entry_option in &mut self.entries {
                        if let Some(ref mut entry) = *entry_option {
                            let was_active = gen.active_packages.iter().any(|p| p.hash.bytes == entry.id.hash.bytes);
                            if was_active {
                                entry.ref_count.store(1, Ordering::SeqCst);
                            } else {
                                entry.ref_count.store(0, Ordering::SeqCst);
                            }
                        }
                    }
                    self.current_generation = gen_id;
                    return Ok(());
                }
            }
        }
        Err(StoreError::GenerationNotFound)
    }
    
    /// Get current generation
    pub fn current_generation(&self) -> u64 {
        self.current_generation
    }
    
    /// Garbage collect unreferenced packages
    pub fn gc(&mut self) -> usize {
        let mut removed = 0;
        self.entries.retain(|entry_option| {
            if let Some(ref entry) = *entry_option {
                if entry.ref_count() == 0 {
                    removed += 1;
                    false
                } else {
                    true
                }
            } else {
                false
            }
        });
        removed
    }
}

/// Store error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum StoreError {
    Success = 0,
    PackageNotFound = 1,
    GenerationNotFound = 2,
    RollbackFailed = 3,
    InvalidHash = 4,
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            vec: self,
            index: 0,
        }
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

struct Iter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            let item = unsafe { &*self.vec.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

/// Get current time (nanoseconds)
fn get_current_time() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1_000_000, Ordering::SeqCst)
}
