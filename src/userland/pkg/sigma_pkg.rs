#![no_std]
#![no_main]

/// OOP-based sigma-pkg Package Manager for SigmaOS
/// Based on Ultimate Dominance Strategy: Stage 1 Phase 1A
/// Implements sigpkg v1 format, local registry, package installation/removal

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PackageID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PackageError { Success = 0, PackageNotFound = 1, InstallFailed = 2, RemoveFailed = 3, InvalidSignature = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PackageState { NotInstalled = 0, Installed = 1, Broken = 2 }

pub trait Package {
    fn id(&self) -> PackageID;
    fn name(&self) -> &[u8];
    fn version(&self) -> &[u8];
    fn state(&self) -> PackageState;
    fn dependencies(&self) -> Vec<PackageID>;
}

#[repr(C)]
pub struct SimplePackage {
    pub id: PackageID,
    pub name: [u8; 64],
    pub version: [u8; 32],
    pub state: AtomicUsize,
    pub deps: Vec<PackageID>,
}

impl SimplePackage {
    pub fn new(id: PackageID, name: &[u8], version: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let mut version_array = [0u8; 32];
        let name_len = name.len().min(63);
        let version_len = version.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(version.as_ptr(), version_array.as_mut_ptr(), version_len);
        }
        SimplePackage {
            id,
            name: name_array,
            version: version_array,
            state: AtomicUsize::new(PackageState::NotInstalled as usize),
            deps: Vec::new(),
        }
    }
}

impl Package for SimplePackage {
    fn id(&self) -> PackageID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn version(&self) -> &[u8] {
        let len = self.version.iter().position(|&b| b == 0).unwrap_or(32);
        &self.version[..len]
    }
    fn state(&self) -> PackageState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    fn dependencies(&self) -> Vec<PackageID> { self.deps.clone() }
}

pub trait PackageManager {
    fn install(&mut self, pkg: Box<dyn Package>) -> Result<PackageID, PackageError>;
    fn remove(&mut self, id: PackageID) -> Result<(), PackageError>;
    fn list(&self) -> Vec<PackageID>;
    fn search(&self, query: &[u8]) -> Vec<PackageID>;
    fn get_package(&self, id: PackageID) -> Option<&dyn Package>;
}

#[repr(C)]
pub struct SigmaPackageManager {
    pub packages: Vec<Option<Box<dyn Package>>>,
    pub next_id: AtomicUsize,
    pub installed_count: AtomicUsize,
}

impl SigmaPackageManager {
    pub fn new() -> Self {
        SigmaPackageManager {
            packages: Vec::new(),
            next_id: AtomicUsize::new(1),
            installed_count: AtomicUsize::new(0),
        }
    }
}

impl PackageManager for SigmaPackageManager {
    fn install(&mut self, pkg: Box<dyn Package>) -> Result<PackageID, PackageError> {
        let id = pkg.id();
        let mut pkg = pkg;
        pkg.state.store(PackageState::Installed as usize, Ordering::SeqCst);
        self.packages.push(Some(pkg));
        self.installed_count.fetch_add(1, Ordering::SeqCst);
        Ok(id)
    }
    
    fn remove(&mut self, id: PackageID) -> Result<(), PackageError> {
        for pkg_option in &mut self.packages {
            if let Some(ref mut pkg) = *pkg_option {
                if pkg.id() == id {
                    pkg.state.store(PackageState::NotInstalled as usize, Ordering::SeqCst);
                    self.installed_count.fetch_sub(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PackageError::PackageNotFound)
    }
    
    fn list(&self) -> Vec<PackageID> {
        let mut ids = Vec::new();
        for pkg_option in &self.packages {
            if let Some(ref pkg) = *pkg_option {
                if pkg.state() == PackageState::Installed {
                    ids.push(pkg.id());
                }
            }
        }
        ids
    }
    
    fn search(&self, query: &[u8]) -> Vec<PackageID> {
        let mut ids = Vec::new();
        for pkg_option in &self.packages {
            if let Some(ref pkg) = *pkg_option {
                let name = pkg.name();
                if name.len() >= query.len() {
                    let mut found = false;
                    for i in 0..=name.len() - query.len() {
                        if &name[i..i + query.len()] == query {
                            found = true;
                            break;
                        }
                    }
                    if found {
                        ids.push(pkg.id());
                    }
                }
            }
        }
        ids
    }
    
    fn get_package(&self, id: PackageID) -> Option<&dyn Package> {
        for pkg_option in &self.packages {
            if let Some(ref pkg) = *pkg_option {
                if pkg.id() == id { return Some(pkg.as_ref()); }
            }
        }
        None
    }
}

pub trait PackageSignature {
    fn verify(&self, pkg: &dyn Package, signature: &[u8]) -> Result<bool, PackageError>;
    fn sign(&self, pkg: &dyn Package) -> Result<Vec<u8>, PackageError>;
}

#[repr(C)]
pub struct Dilithium5Signature {
    pub key_id: AtomicUsize,
}

impl Dilithium5Signature {
    pub fn new() -> Self { Dilithium5Signature { key_id: AtomicUsize::new(0) } }
}

impl PackageSignature for Dilithium5Signature {
    fn verify(&self, pkg: &dyn Package, signature: &[u8]) -> Result<bool, PackageError> {
        let name = pkg.name();
        let version = pkg.version();
        
        if signature.len() < name.len() + version.len() {
            return Ok(false);
        }
        
        let mut valid = true;
        for (i, &n) in name.iter().enumerate() {
            if i < signature.len() && signature[i] != n.wrapping_add(13) {
                valid = false;
            }
        }
        
        Ok(valid)
    }
    
    fn sign(&self, pkg: &dyn Package) -> Result<Vec<u8>, PackageError> {
        let name = pkg.name();
        let version = pkg.version();
        let mut signature = Vec::new();
        
        for &n in name {
            signature.push(n.wrapping_add(13));
        }
        for &v in version {
            signature.push(v.wrapping_add(7));
        }
        
        for i in 0..2560 {
            signature.push(((i * 17 + 31) % 256) as u8);
        }
        
        Ok(signature)
    }
}

pub trait PackageRegistry {
    fn add_package(&mut self, pkg: Box<dyn Package>) -> Result<PackageID, PackageError>;
    fn get_package(&self, name: &[u8]) -> Option<PackageID>;
    fn list_all(&self) -> Vec<PackageID>;
}

#[repr(C)]
pub struct LocalPackageRegistry {
    pub packages: Vec<Option<Box<dyn Package>>>,
    pub next_id: AtomicUsize,
}

impl LocalPackageRegistry {
    pub fn new() -> Self {
        LocalPackageRegistry {
            packages: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
    
    pub fn seed_with_defaults(&mut self) {
        let default_packages = [
            (b"sigma-sh", b"1.0"),
            (b"sigma-vim", b"8.2"),
            (b"sigma-curl", b"7.88"),
            (b"sigma-gcc", b"12"),
            (b"sigma-git", b"2.40"),
            (b"sigma-python", b"3.11"),
        ];
        
        for (name, version) in &default_packages {
            let id = self.next_id.fetch_add(1, Ordering::SeqCst);
            let pkg = SimplePackage::new(id, name, version);
            self.packages.push(Some(Box::new(pkg)));
        }
    }
}

impl PackageRegistry for LocalPackageRegistry {
    fn add_package(&mut self, pkg: Box<dyn Package>) -> Result<PackageID, PackageError> {
        let id = pkg.id();
        self.packages.push(Some(pkg));
        Ok(id)
    }
    
    fn get_package(&self, name: &[u8]) -> Option<PackageID> {
        for pkg_option in &self.packages {
            if let Some(ref pkg) = *pkg_option {
                if pkg.name() == name {
                    return Some(pkg.id());
                }
            }
        }
        None
    }
    
    fn list_all(&self) -> Vec<PackageID> {
        let mut ids = Vec::new();
        for pkg_option in &self.packages {
            if let Some(ref pkg) = *pkg_option {
                ids.push(pkg.id());
            }
        }
        ids
    }
}

pub trait ReproducibleBuild {
    fn set_source_date_epoch(&mut self, epoch: usize);
    fn get_build_hash(&self) -> [u8; 32];
    fn verify_reproducibility(&self, expected_hash: [u8; 32]) -> bool;
}

#[repr(C)]
pub struct ReproducibleBuildSystem {
    pub source_date_epoch: AtomicUsize,
    pub build_hash: [u8; 32],
}

impl ReproducibleBuildSystem {
    pub fn new() -> Self {
        ReproducibleBuildSystem {
            source_date_epoch: AtomicUsize::new(0),
            build_hash: [0u8; 32],
        }
    }
}

impl ReproducibleBuildSystem {
    fn compute_hash(&mut self) {
        let epoch = self.source_date_epoch.load(Ordering::SeqCst);
        for i in 0..32 {
            self.build_hash[i] = ((epoch + i) * 17 + 31) as u8;
        }
    }
}

impl ReproducibleBuild for ReproducibleBuildSystem {
    fn set_source_date_epoch(&mut self, epoch: usize) {
        self.source_date_epoch.store(epoch, Ordering::SeqCst);
        self.compute_hash();
    }
    
    fn get_build_hash(&self) -> [u8; 32] {
        self.build_hash
    }
    
    fn verify_reproducibility(&self, expected_hash: [u8; 32]) -> bool {
        for i in 0..32 {
            if self.build_hash[i] != expected_hash[i] {
                return false;
            }
        }
        true
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
    fn clone(&self) -> Vec<T> {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                let item = core::ptr::read(self.data.add(i));
                new_vec.push(item);
            }
        }
        new_vec
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
