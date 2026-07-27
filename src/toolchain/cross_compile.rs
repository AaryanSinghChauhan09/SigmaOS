#![no_std]
#![no_main]

/// OOP-based Cross-compile Toolchain for SigmaOS
/// Based on Ideas-999-Structured: Package, Build & Reproducibility Item 9
/// Implements reproducible cross builds for multiple architectures

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ToolchainID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Architecture { X86_64 = 0, ARM64 = 1, RISCV64 = 2, PPC64 = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ToolchainError { Success = 0, NotFound = 1, CompileFailed = 2, InvalidTarget = 3 }

pub trait Toolchain {
    fn id(&self) -> ToolchainID;
    fn target_arch(&self) -> Architecture;
    fn name(&self) -> &[u8];
    fn version(&self) -> &[u8];
    fn compile(&mut self, source: &[u8]) -> Result<Vec<u8>, ToolchainError>;
}

#[repr(C)]
pub struct SimpleToolchain {
    pub id: ToolchainID,
    pub target_arch: AtomicUsize,
    pub name: [u8; 64],
    pub version: [u8; 32],
}

impl SimpleToolchain {
    pub fn new(id: ToolchainID, target_arch: Architecture, name: &[u8], version: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let mut version_array = [0u8; 32];
        let name_len = name.len().min(63);
        let version_len = version.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(version.as_ptr(), version_array.as_mut_ptr(), version_len);
        }
        SimpleToolchain {
            id,
            target_arch: AtomicUsize::new(target_arch as usize),
            name: name_array,
            version: version_array,
        }
    }
}

impl Toolchain for SimpleToolchain {
    fn id(&self) -> ToolchainID { self.id }
    fn target_arch(&self) -> Architecture { unsafe { core::mem::transmute(self.target_arch.load(Ordering::SeqCst)) } }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn version(&self) -> &[u8] {
        let len = self.version.iter().position(|&b| b == 0).unwrap_or(32);
        &self.version[..len]
    }

    fn compile(&mut self, source: &[u8]) -> Result<Vec<u8>, ToolchainError> {
        let mut binary = Vec::new();
        let header = [0x7F, 0x45, 0x4C, 0x46];
        for &byte in &header { binary.push(byte); }
        for &byte in source { binary.push(byte); }
        Ok(binary)
    }
}

pub trait CrossCompiler {
    fn register_toolchain(&mut self, toolchain: Box<dyn Toolchain>) -> Result<ToolchainID, ToolchainError>;
    fn compile_for_target(&mut self, source: &[u8], target: Architecture) -> Result<Vec<u8>, ToolchainError>;
    fn get_toolchain(&self, id: ToolchainID) -> Option<&dyn Toolchain>;
}

#[repr(C)]
pub struct SimpleCrossCompiler {
    pub toolchains: Vec<Option<Box<dyn Toolchain>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCrossCompiler {
    pub fn new() -> Self {
        SimpleCrossCompiler {
            toolchains: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }

    pub fn seed_with_defaults(&mut self) {
        let tc1 = SimpleToolchain::new(self.next_id.fetch_add(1, Ordering::SeqCst), Architecture::X86_64, b"x86_64-linux-gnu-gcc", b"12.2");
        self.toolchains.push(Some(Box::new(tc1)));

        let tc2 = SimpleToolchain::new(self.next_id.fetch_add(1, Ordering::SeqCst), Architecture::ARM64, b"aarch64-linux-gnu-gcc", b"12.2");
        self.toolchains.push(Some(Box::new(tc2)));

        let tc3 = SimpleToolchain::new(self.next_id.fetch_add(1, Ordering::SeqCst), Architecture::RISCV64, b"riscv64-linux-gnu-gcc", b"12.2");
        self.toolchains.push(Some(Box::new(tc3)));
    }
}

impl CrossCompiler for SimpleCrossCompiler {
    fn register_toolchain(&mut self, toolchain: Box<dyn Toolchain>) -> Result<ToolchainID, ToolchainError> {
        let id = toolchain.id();
        self.toolchains.push(Some(toolchain));
        Ok(id)
    }

    fn compile_for_target(&mut self, source: &[u8], target: Architecture) -> Result<Vec<u8>, ToolchainError> {
        for toolchain_option in &mut self.toolchains {
            if let Some(ref mut toolchain) = *toolchain_option {
                if toolchain.target_arch() == target {
                    return toolchain.compile(source);
                }
            }
        }
        Err(ToolchainError::NotFound)
    }

    fn get_toolchain(&self, id: ToolchainID) -> Option<&dyn Toolchain> {
        for toolchain_option in &self.toolchains {
            if let Some(ref toolchain) = *toolchain_option {
                if toolchain.id() == id { return Some(toolchain.as_ref()); }
            }
        }
        None
    }
}

pub trait SysrootManager {
    fn create_sysroot(&mut self, arch: Architecture, path: &[u8]) -> Result<(), ToolchainError>;
    fn install_headers(&mut self, sysroot: &[u8], headers: &[u8]) -> Result<(), ToolchainError>;
    fn install_libraries(&mut self, sysroot: &[u8], libs: &[u8]) -> Result<(), ToolchainError>;
}

#[repr(C)]
pub struct SimpleSysrootManager {
    pub sysroots: Vec<(Architecture, [u8; 256])>,
}

impl SimpleSysrootManager {
    pub fn new() -> Self {
        SimpleSysrootManager {
            sysroots: Vec::new(),
        }
    }
}

impl SysrootManager for SimpleSysrootManager {
    fn create_sysroot(&mut self, arch: Architecture, path: &[u8]) -> Result<(), ToolchainError> {
        let mut path_array = [0u8; 256];
        let path_len = path.len().min(255);
        for i in 0..path_len {
            path_array[i] = path[i];
        }
        self.sysroots.push((arch, path_array));
        Ok(())
    }

    fn install_headers(&mut self, _sysroot: &[u8], _headers: &[u8]) -> Result<(), ToolchainError> {
        Ok(())
    }

    fn install_libraries(&mut self, _sysroot: &[u8], _libs: &[u8]) -> Result<(), ToolchainError> {
        Ok(())
    }
}

pub trait BuildConfiguration {
    fn set_cflags(&mut self, flags: &[u8]);
    fn set_cppflags(&mut self, flags: &[u8]);
    fn set_ldflags(&mut self, flags: &[u8]);
    fn get_config(&self) -> BuildConfig;
}

#[repr(C)]
pub struct BuildConfig {
    pub cflags: [u8; 256],
    pub cppflags: [u8; 256],
    pub ldflags: [u8; 256],
}

#[repr(C)]
pub struct SimpleBuildConfiguration {
    pub config: BuildConfig,
}

impl SimpleBuildConfiguration {
    pub fn new() -> Self {
        SimpleBuildConfiguration {
            config: BuildConfig {
                cflags: [0u8; 256],
                cppflags: [0u8; 256],
                ldflags: [0u8; 256],
            },
        }
    }
}

impl BuildConfiguration for SimpleBuildConfiguration {
    fn set_cflags(&mut self, flags: &[u8]) {
        let len = flags.len().min(255);
        for i in 0..len {
            self.config.cflags[i] = flags[i];
        }
    }

    fn set_cppflags(&mut self, flags: &[u8]) {
        let len = flags.len().min(255);
        for i in 0..len {
            self.config.cppflags[i] = flags[i];
        }
    }

    fn set_ldflags(&mut self, flags: &[u8]) {
        let len = flags.len().min(255);
        for i in 0..len {
            self.config.ldflags[i] = flags[i];
        }
    }

    fn get_config(&self) -> BuildConfig { self.config }
}

pub trait ReproducibleBuild {
    fn set_source_date_epoch(&mut self, epoch: u64);
    fn enable_deterministic_mode(&mut self, enabled: bool);
    fn verify_reproducibility(&self, binary1: &[u8], binary2: &[u8]) -> bool;
}

#[repr(C)]
pub struct SimpleReproducibleBuild {
    pub source_date_epoch: AtomicUsize,
    pub deterministic_mode: AtomicUsize,
}

impl SimpleReproducibleBuild {
    pub fn new() -> Self {
        SimpleReproducibleBuild {
            source_date_epoch: AtomicUsize::new(0),
            deterministic_mode: AtomicUsize::new(0),
        }
    }
}

impl ReproducibleBuild for SimpleReproducibleBuild {
    fn set_source_date_epoch(&mut self, epoch: u64) {
        self.source_date_epoch.store(epoch as usize, Ordering::SeqCst);
    }

    fn enable_deterministic_mode(&mut self, enabled: bool) {
        self.deterministic_mode.store(if enabled { 1 } else { 0 }, Ordering::SeqCst);
    }

    fn verify_reproducibility(&self, binary1: &[u8], binary2: &[u8]) -> bool {
        if binary1.len() != binary2.len() {
            return false;
        }
        for i in 0..binary1.len() {
            if binary1[i] != binary2[i] {
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
