#![no_std]
#![no_main]

/// OOP-based Package Management for SigmaOS
/// Based on Roadmap Item: Package Management + Reproducible Builds

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PackageID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PackageState { Installed = 0, Available = 1, Updating = 2, Corrupted = 3 }

pub trait Package {
    fn id(&self) -> PackageID;
    fn name(&self) -> &[u8];
    fn version(&self) -> &[u8];
    fn state(&self) -> PackageState;
}

#[repr(C)]
pub struct SimplePackage {
    pub id: PackageID,
    pub name: [u8; 64],
    pub version: [u8; 32],
    pub state: AtomicUsize,
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
            state: AtomicUsize::new(PackageState::Available as usize),
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
}

pub trait PackageManager {
    fn install(&mut self, package: Box<dyn Package>) -> Result<PackageID, PackageError>;
    fn uninstall(&mut self, id: PackageID) -> Result<(), PackageError>;
    fn update(&mut self, id: PackageID) -> Result<(), PackageError>;
    fn get_package(&self, id: PackageID) -> Option<&dyn Package>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PackageError { Success = 0, PackageNotFound = 1, InstallFailed = 2, UpdateFailed = 3 }

pub struct SimplePackageManager {
    packages: Vec<Option<Box<dyn Package>>>,
    next_id: AtomicUsize,
}

impl SimplePackageManager {
    pub fn new() -> Self { SimplePackageManager { packages: Vec::new(), next_id: AtomicUsize::new(1) } }
}

impl PackageManager for SimplePackageManager {
    fn install(&mut self, package: Box<dyn Package>) -> Result<PackageID, PackageError> {
        let id = package.id();
        self.packages.push(Some(package));
        Ok(id)
    }
    fn uninstall(&mut self, id: PackageID) -> Result<(), PackageError> {
        for pkg_option in &mut self.packages {
            if let Some(ref pkg) = *pkg_option {
                if pkg.id() == id {
                    self.packages.clear();
                    return Ok(());
                }
            }
        }
        Err(PackageError::PackageNotFound)
    }
    fn update(&mut self, id: PackageID) -> Result<(), PackageError> {
        for pkg_option in &mut self.packages {
            if let Some(ref mut pkg) = *pkg_option {
                if pkg.id() == id {
                    return Ok(());
                }
            }
        }
        Err(PackageError::PackageNotFound)
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

pub trait Repository {
    fn add_package(&mut self, package: Box<dyn Package>) -> Result<(), PackageError>;
    fn remove_package(&mut self, id: PackageID) -> Result<(), PackageError>;
    fn list_packages(&self) -> Vec<PackageID>;
}

pub struct SimpleRepository {
    packages: Vec<Option<Box<dyn Package>>>,
}

impl SimpleRepository {
    pub fn new() -> Self { SimpleRepository { packages: Vec::new() } }
}

impl Repository for SimpleRepository {
    fn add_package(&mut self, package: Box<dyn Package>) -> Result<(), PackageError> {
        self.packages.push(Some(package));
        Ok(())
    }
    fn remove_package(&mut self, id: PackageID) -> Result<(), PackageError> {
        for pkg_option in &mut self.packages {
            if let Some(ref pkg) = *pkg_option {
                if pkg.id() == id {
                    self.packages.clear();
                    return Ok(());
                }
            }
        }
        Err(PackageError::PackageNotFound)
    }
    fn list_packages(&self) -> Vec<PackageID> {
        let mut ids = Vec::new();
        for pkg_option in &self.packages {
            if let Some(ref pkg) = *pkg_option {
                ids.push(pkg.id());
            }
        }
        ids
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
    fn clear(&mut self) { self.len = 0; }
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
