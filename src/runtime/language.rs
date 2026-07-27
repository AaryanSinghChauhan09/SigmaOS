#![no_std]
#![no_main]

/// OOP-based Language Runtime Management for SigmaOS
/// Based on Ideas-999-Structured: Package, Build & Reproducibility Item 14
/// Implements unified handling for Python, Node, Java runtimes

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RuntimeID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LanguageType { Python = 0, Node = 1, Java = 2, Go = 3, Rust = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RuntimeError { Success = 0, NotFound = 1, InstallFailed = 2, UninstallFailed = 3 }

pub trait LanguageRuntime {
    fn id(&self) -> RuntimeID;
    fn language_type(&self) -> LanguageType;
    fn version(&self) -> &[u8];
    fn install(&mut self) -> Result<(), RuntimeError>;
    fn uninstall(&mut self) -> Result<(), RuntimeError>;
    fn is_installed(&self) -> bool;
}

#[repr(C)]
pub struct SimpleLanguageRuntime {
    pub id: RuntimeID,
    pub language_type: AtomicUsize,
    pub version: [u8; 32],
    pub installed: AtomicUsize,
}

impl SimpleLanguageRuntime {
    pub fn new(id: RuntimeID, language_type: LanguageType, version: &[u8]) -> Self {
        let mut version_array = [0u8; 32];
        let version_len = version.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(version.as_ptr(), version_array.as_mut_ptr(), version_len);
        }
        SimpleLanguageRuntime {
            id,
            language_type: AtomicUsize::new(language_type as usize),
            version: version_array,
            installed: AtomicUsize::new(0),
        }
    }
}

impl LanguageRuntime for SimpleLanguageRuntime {
    fn id(&self) -> RuntimeID { self.id }
    fn language_type(&self) -> LanguageType { unsafe { core::mem::transmute(self.language_type.load(Ordering::SeqCst)) } }
    fn version(&self) -> &[u8] {
        let len = self.version.iter().position(|&b| b == 0).unwrap_or(32);
        &self.version[..len]
    }

    fn install(&mut self) -> Result<(), RuntimeError> {
        self.installed.store(1, Ordering::SeqCst);
        Ok(())
    }

    fn uninstall(&mut self) -> Result<(), RuntimeError> {
        self.installed.store(0, Ordering::SeqCst);
        Ok(())
    }

    fn is_installed(&self) -> bool { self.installed.load(Ordering::SeqCst) == 1 }
}

pub trait RuntimeManager {
    fn register_runtime(&mut self, runtime: Box<dyn LanguageRuntime>) -> Result<RuntimeID, RuntimeError>;
    fn get_runtime(&self, id: RuntimeID) -> Option<&dyn LanguageRuntime>;
    fn list_installed(&self) -> Vec<RuntimeID>;
    fn set_default(&mut self, language_type: LanguageType, id: RuntimeID) -> Result<(), RuntimeError>;
}

#[repr(C)]
pub struct SimpleRuntimeManager {
    pub runtimes: Vec<Option<Box<dyn LanguageRuntime>>>,
    pub defaults: [AtomicUsize; 5],
    pub next_id: AtomicUsize,
}

impl SimpleRuntimeManager {
    pub fn new() -> Self {
        SimpleRuntimeManager {
            runtimes: Vec::new(),
            defaults: [
                AtomicUsize::new(0),
                AtomicUsize::new(0),
                AtomicUsize::new(0),
                AtomicUsize::new(0),
                AtomicUsize::new(0),
            ],
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RuntimeManager for SimpleRuntimeManager {
    fn register_runtime(&mut self, runtime: Box<dyn LanguageRuntime>) -> Result<RuntimeID, RuntimeError> {
        let id = runtime.id();
        self.runtimes.push(Some(runtime));
        Ok(id)
    }

    fn get_runtime(&self, id: RuntimeID) -> Option<&dyn LanguageRuntime> {
        for runtime_option in &self.runtimes {
            if let Some(ref runtime) = *runtime_option {
                if runtime.id() == id { return Some(runtime.as_ref()); }
            }
        }
        None
    }

    fn list_installed(&self) -> Vec<RuntimeID> {
        let mut ids = Vec::new();
        for runtime_option in &self.runtimes {
            if let Some(ref runtime) = *runtime_option {
                if runtime.is_installed() {
                    ids.push(runtime.id());
                }
            }
        }
        ids
    }

    fn set_default(&mut self, language_type: LanguageType, id: RuntimeID) -> Result<(), RuntimeError> {
        let idx = language_type as usize;
        if idx < 5 {
            self.defaults[idx].store(id, Ordering::SeqCst);
            Ok(())
        } else {
            Err(RuntimeError::NotFound)
        }
    }
}

pub trait PackageDependency {
    fn add_dependency(&mut self, runtime_id: RuntimeID, package: &[u8]) -> Result<(), RuntimeError>;
    fn remove_dependency(&mut self, runtime_id: RuntimeID, package: &[u8]) -> Result<(), RuntimeError>;
    fn list_dependencies(&self, runtime_id: RuntimeID) -> Vec<[u8; 128]>;
}

#[repr(C)]
pub struct SimplePackageDependency {
    pub dependencies: Vec<(RuntimeID, [u8; 128])>,
}

impl SimplePackageDependency {
    pub fn new() -> Self {
        SimplePackageDependency {
            dependencies: Vec::new(),
        }
    }
}

impl PackageDependency for SimplePackageDependency {
    fn add_dependency(&mut self, runtime_id: RuntimeID, package: &[u8]) -> Result<(), RuntimeError> {
        let mut package_array = [0u8; 128];
        let package_len = package.len().min(127);
        for i in 0..package_len {
            package_array[i] = package[i];
        }
        self.dependencies.push((runtime_id, package_array));
        Ok(())
    }

    fn remove_dependency(&mut self, runtime_id: RuntimeID, package: &[u8]) -> Result<(), RuntimeError> {
        for i in 0..self.dependencies.len() {
            if self.dependencies[i].0 == runtime_id {
                let dep = &self.dependencies[i].1;
                let len = dep.iter().position(|&b| b == 0).unwrap_or(128);
                if &dep[..len] == package {
                    self.dependencies.remove(i);
                    return Ok(());
                }
            }
        }
        Err(RuntimeError::NotFound)
    }

    fn list_dependencies(&self, runtime_id: RuntimeID) -> Vec<[u8; 128]> {
        let mut packages = Vec::new();
        for &(rt_id, ref pkg) in &self.dependencies {
            if rt_id == runtime_id {
                packages.push(*pkg);
            }
        }
        packages
    }
}

pub trait VirtualEnvironment {
    fn create_venv(&mut self, runtime_id: RuntimeID, name: &[u8]) -> Result<usize, RuntimeError>;
    fn activate_venv(&mut self, venv_id: usize) -> Result<(), RuntimeError>;
    fn delete_venv(&mut self, venv_id: usize) -> Result<(), RuntimeError>;
}

#[repr(C)]
pub struct SimpleVirtualEnvironment {
    pub venvs: Vec<(usize, RuntimeID, [u8; 128])>,
    pub next_id: AtomicUsize,
}

impl SimpleVirtualEnvironment {
    pub fn new() -> Self {
        SimpleVirtualEnvironment {
            venvs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl VirtualEnvironment for SimpleVirtualEnvironment {
    fn create_venv(&mut self, runtime_id: RuntimeID, name: &[u8]) -> Result<usize, RuntimeError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut name_array = [0u8; 128];
        let name_len = name.len().min(127);
        for i in 0..name_len {
            name_array[i] = name[i];
        }
        self.venvs.push((id, runtime_id, name_array));
        Ok(id)
    }

    fn activate_venv(&mut self, venv_id: usize) -> Result<(), RuntimeError> {
        for venv in &self.venvs {
            if venv.0 == venv_id {
                return Ok(());
            }
        }
        Err(RuntimeError::NotFound)
    }

    fn delete_venv(&mut self, venv_id: usize) -> Result<(), RuntimeError> {
        for i in 0..self.venvs.len() {
            if self.venvs[i].0 == venv_id {
                self.venvs.remove(i);
                return Ok(());
            }
        }
        Err(RuntimeError::NotFound)
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
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
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
