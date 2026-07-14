#![no_std]
#![no_main]

/// OOP-based Local Package Cache & Proxy for SigmaOS
/// Based on Ideas-999-Structured: Package, Build & Reproducibility Item 11
/// Implements offline-first package caching and registry proxy

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PackageID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CacheError { Success = 0, NotFound = 1, WriteFailed = 2, CacheFull = 3 }

pub trait CachedPackage {
    fn id(&self) -> PackageID;
    fn name(&self) -> &[u8];
    fn version(&self) -> &[u8];
    fn size(&self) -> usize;
    fn cached_at(&self) -> u64;
}

#[repr(C)]
pub struct SimpleCachedPackage {
    pub id: PackageID,
    pub name: [u8; 64],
    pub version: [u8; 32],
    pub size: AtomicUsize,
    pub cached_at: AtomicUsize,
    pub data: [u8; 4096],
}

impl SimpleCachedPackage {
    pub fn new(id: PackageID, name: &[u8], version: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let mut version_array = [u8; 32];
        let name_len = name.len().min(63);
        let version_len = version.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(version.as_ptr(), version_array.as_mut_ptr(), version_len);
        }
        SimpleCachedPackage {
            id,
            name: name_array,
            version: version_array,
            size: AtomicUsize::new(0),
            cached_at: AtomicUsize::new(0),
            data: [0u8; 4096],
        }
    }
}

impl CachedPackage for SimpleCachedPackage {
    fn id(&self) -> PackageID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn version(&self) -> &[u8] {
        let len = self.version.iter().position(|&b| b == 0).unwrap_or(32);
        &self.version[..len]
    }
    fn size(&self) -> usize { self.size.load(Ordering::SeqCst) }
    fn cached_at(&self) -> u64 { self.cached_at.load(Ordering::SeqCst) as u64 }
}

pub trait PackageCache {
    fn store(&mut self, package: Box<dyn CachedPackage>) -> Result<PackageID, CacheError>;
    fn retrieve(&self, id: PackageID) -> Option<&dyn CachedPackage>;
    fn remove(&mut self, id: PackageID) -> Result<(), CacheError>;
    fn find_by_name(&self, name: &[u8]) -> Vec<PackageID>;
    fn get_usage(&self) -> CacheUsage;
}

#[repr(C)]
pub struct CacheUsage {
    pub total_size: usize,
    pub package_count: usize,
    pub max_size: usize,
}

#[repr(C)]
pub struct SimplePackageCache {
    pub packages: Vec<Option<Box<dyn CachedPackage>>>,
    pub next_id: AtomicUsize,
    pub max_size: AtomicUsize,
    pub current_size: AtomicUsize,
}

impl SimplePackageCache {
    pub fn new(max_size_mb: usize) -> Self {
        SimplePackageCache {
            packages: Vec::new(),
            next_id: AtomicUsize::new(1),
            max_size: AtomicUsize::new(max_size_mb * 1024 * 1024),
            current_size: AtomicUsize::new(0),
        }
    }
}

impl PackageCache for SimplePackageCache {
    fn store(&mut self, package: Box<dyn CachedPackage>) -> Result<PackageID, CacheError> {
        let package_size = package.size();
        let current = self.current_size.load(Ordering::SeqCst);
        let max = self.max_size.load(Ordering::SeqCst);

        if current + package_size > max {
            return Err(CacheError::CacheFull);
        }

        let id = package.id();
        self.current_size.fetch_add(package_size, Ordering::SeqCst);
        self.packages.push(Some(package));
        Ok(id)
    }

    fn retrieve(&self, id: PackageID) -> Option<&dyn CachedPackage> {
        for package_option in &self.packages {
            if let Some(ref package) = *package_option {
                if package.id() == id { return Some(package.as_ref()); }
            }
        }
        None
    }

    fn remove(&mut self, id: PackageID) -> Result<(), CacheError> {
        for package_option in &mut self.packages {
            if let Some(ref package) = *package_option {
                if package.id() == id {
                    let size = package.size();
                    self.current_size.fetch_sub(size, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(CacheError::NotFound)
    }

    fn find_by_name(&self, name: &[u8]) -> Vec<PackageID> {
        let mut ids = Vec::new();
        for package_option in &self.packages {
            if let Some(ref package) = *package_option {
                if package.name() == name {
                    ids.push(package.id());
                }
            }
        }
        ids
    }

    fn get_usage(&self) -> CacheUsage {
        CacheUsage {
            total_size: self.current_size.load(Ordering::SeqCst),
            package_count: self.packages.len(),
            max_size: self.max_size.load(Ordering::SeqCst),
        }
    }
}

pub trait CacheEviction {
    fn evict_lru(&mut self) -> Result<PackageID, CacheError>;
    fn evict_by_size(&mut self, target_size: usize) -> Result<Vec<PackageID>, CacheError>;
    fn set_eviction_policy(&mut self, policy: EvictionPolicy);
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EvictionPolicy { LRU = 0, LFU = 1, FIFO = 2 }

#[repr(C)]
pub struct SimpleCacheEviction {
    pub cache: SimplePackageCache,
    pub policy: AtomicUsize,
}

impl SimpleCacheEviction {
    pub fn new(cache: SimplePackageCache) -> Self {
        SimpleCacheEviction {
            cache,
            policy: AtomicUsize::new(EvictionPolicy::LRU as usize),
        }
    }
}

impl CacheEviction for SimpleCacheEviction {
    fn evict_lru(&mut self) -> Result<PackageID, CacheError> {
        if let Some(package_option) = self.cache.packages.first() {
            if let Some(ref package) = *package_option {
                let id = package.id();
                self.cache.remove(id)?;
                return Ok(id);
            }
        }
        Err(CacheError::NotFound)
    }

    fn evict_by_size(&mut self, target_size: usize) -> Result<Vec<PackageID>, CacheError> {
        let mut evicted = Vec::new();
        let mut freed = 0;

        while freed < target_size && self.cache.packages.len() > 0 {
            if let Some(id) = self.evict_lru()? {
                if let Some(package) = self.cache.retrieve(id) {
                    freed += package.size();
                    evicted.push(id);
                }
            }
        }

        Ok(evicted)
    }

    fn set_eviction_policy(&mut self, policy: EvictionPolicy) {
        self.policy.store(policy as usize, Ordering::SeqCst);
    }
}

pub trait RegistryProxy {
    fn proxy_request(&mut self, package: &[u8]) -> Result<Vec<u8>, CacheError>;
    fn cache_response(&mut self, package: &[u8], data: &[u8]) -> Result<(), CacheError>;
    fn get_proxy_stats(&self) -> ProxyStats;
}

#[repr(C)]
pub struct ProxyStats {
    pub requests_served: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

#[repr(C)]
pub struct SimpleRegistryProxy {
    pub cache: SimplePackageCache,
    pub stats: ProxyStats,
}

impl SimpleRegistryProxy {
    pub fn new(cache: SimplePackageCache) -> Self {
        SimpleRegistryProxy {
            cache,
            stats: ProxyStats {
                requests_served: 0,
                cache_hits: 0,
                cache_misses: 0,
            },
        }
    }
}

impl RegistryProxy for SimpleRegistryProxy {
    fn proxy_request(&mut self, package: &[u8]) -> Result<Vec<u8>, CacheError> {
        let ids = self.cache.find_by_name(package);

        if !ids.is_empty() {
            if let Some(cached) = self.cache.retrieve(ids[0]) {
                return Ok(cached.name().to_vec());
            }
        }

        Err(CacheError::NotFound)
    }

    fn cache_response(&mut self, package: &[u8], data: &[u8]) -> Result<(), CacheError> {
        let id = self.cache.next_id.fetch_add(1, Ordering::SeqCst);
        let mut cached = SimpleCachedPackage::new(id, package, b"1.0.0");
        cached.size.store(data.len(), Ordering::SeqCst);
        cached.cached_at.store(1000000, Ordering::SeqCst);

        let data_len = data.len().min(4095);
        for i in 0..data_len {
            cached.data[i] = data[i];
        }

        self.cache.store(Box::new(cached))?;
        Ok(())
    }

    fn get_proxy_stats(&self) -> ProxyStats {
        self.stats
    }
}

pub trait OfflineMode {
    fn enable_offline(&mut self, enabled: bool);
    fn is_offline(&self) -> bool;
    fn sync_when_online(&mut self) -> Result<(), CacheError>;
}

#[repr(C)]
pub struct SimpleOfflineMode {
    pub offline: AtomicUsize,
    pub cache: SimplePackageCache,
}

impl SimpleOfflineMode {
    pub fn new(cache: SimplePackageCache) -> Self {
        SimpleOfflineMode {
            offline: AtomicUsize::new(0),
            cache,
        }
    }
}

impl OfflineMode for SimpleOfflineMode {
    fn enable_offline(&mut self, enabled: bool) {
        self.offline.store(if enabled { 1 } else { 0 }, Ordering::SeqCst);
    }

    fn is_offline(&self) -> bool { self.offline.load(Ordering::SeqCst) == 1 }

    fn sync_when_online(&mut self) -> Result<(), CacheError> {
        if self.is_offline() {
            return Err(CacheError::NotFound);
        }
        Ok(())
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
    fn first(&self) -> Option<&T> {
        if self.len > 0 {
            unsafe { Some(&*self.data) }
        } else {
            None
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
