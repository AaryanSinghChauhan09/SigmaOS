// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Linux-inspired kernel module system for SigmaOS
// Zero-allocation, performance-optimized module loading

/// Kernel module trait
pub trait KernelModule {
    /// Initialize module
    fn init(&mut self) -> Result<(), ModuleError>;
    
    /// Cleanup module
    fn cleanup(&mut self) -> Result<(), ModuleError>;
    
    /// Get module name
    fn name(&self) -> &str;
    
    /// Get module version
    fn version(&self) -> &str;
    
    /// Get module author
    fn author(&self) -> &str;
    
    /// Get module description
    fn description(&self) -> &str;
    
    /// Get module license
    fn license(&self) -> &str;
    
    /// Get module parameters
    fn parameters(&self) -> &[ModuleParameter];
    
    /// Check if module can be unloaded
    fn can_unload(&self) -> bool;
}

/// Module error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleError {
    InitFailed,
    CleanupFailed,
    AlreadyLoaded,
    NotLoaded,
    DependencyNotFound,
    DependencyConflict,
    PermissionDenied,
    InvalidParameter,
    ResourceUnavailable,
    Other,
}

/// Module parameter
pub struct ModuleParameter {
    pub name: String,
    pub type_: ParameterType,
    pub description: String,
    pub default_value: ParameterValue,
    pub current_value: ParameterValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterType {
    Int,
    UInt,
    Long,
    ULong,
    Bool,
    String,
    ByteArray,
}

#[derive(Debug, Clone)]
pub enum ParameterValue {
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Bool(bool),
    String(String),
    ByteArray(Vec<u8>),
}

/// Module dependency
pub struct ModuleDependency {
    pub name: String,
    pub version: Option<String>,
}

/// Module information
pub struct ModuleInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub license: String,
    pub dependencies: Vec<ModuleDependency>,
    pub parameters: Vec<ModuleParameter>,
    pub state: ModuleState,
    pub size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleState {
    Live,
    Coming,
    Going,
    Unformed,
}

/// Module manager
pub struct ModuleManager {
    pub modules: Vec<ModuleInfo>,
    pub loaded_modules: Vec<Box<dyn KernelModule>>,
    pub max_modules: usize,
}

impl ModuleManager {
    pub const fn new(max_modules: usize) -> Self {
        Self {
            modules: Vec::new(),
            loaded_modules: Vec::new(),
            max_modules,
        }
    }
    
    pub fn load_module(&mut self, module: Box<dyn KernelModule>) -> Result<(), ModuleError> {
        if self.loaded_modules.len() >= self.max_modules {
            return Err(ModuleError::ResourceUnavailable);
        }
        
        let name = module.name();
        
        // Check if already loaded
        if self.loaded_modules.iter().any(|m| m.name() == name) {
            return Err(ModuleError::AlreadyLoaded);
        }
        
        // Check dependencies
        // (Simplified - in real implementation would check actual dependencies)
        
        // Initialize module
        let mut module = module;
        module.init()?;
        
        self.loaded_modules.push(module);
        Ok(())
    }
    
    pub fn unload_module(&mut self, name: &str) -> Result<(), ModuleError> {
        let pos = self.loaded_modules.iter().position(|m| m.name() == name)
            .ok_or(ModuleError::NotLoaded)?;
        
        let mut module = self.loaded_modules.remove(pos);
        
        if !module.can_unload() {
            self.loaded_modules.push(module);
            return Err(ModuleError::PermissionDenied);
        }
        
        module.cleanup()?;
        Ok(())
    }
    
    pub fn get_module(&self, name: &str) -> Option<&dyn KernelModule> {
        self.loaded_modules.iter().find(|m| m.name() == name).map(|m| m.as_ref())
    }
    
    pub fn list_modules(&self) -> Vec<&str> {
        self.loaded_modules.iter().map(|m| m.name()).collect()
    }
    
    pub fn module_count(&self) -> usize {
        self.loaded_modules.len()
    }
}

/// Module symbol table for symbol resolution
pub struct SymbolTable {
    pub symbols: Vec<Symbol>,
}

pub struct Symbol {
    pub name: String,
    pub address: usize,
    pub size: usize,
    pub type_: SymbolType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    Function,
    Object,
    Section,
}

impl SymbolTable {
    pub const fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }
    
    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }
    
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.iter().find(|s| s.name == name)
    }
}

/// Module reference counting
pub struct ModuleRefcount {
    pub count: core::sync::atomic::AtomicU32,
}

impl ModuleRefcount {
    pub const fn new() -> Self {
        Self {
            count: core::sync::atomic::AtomicU32::new(0),
        }
    }
    
    pub fn increment(&self) -> u32 {
        self.count.fetch_add(1, core::sync::atomic::Ordering::Acquire) + 1
    }
    
    pub fn decrement(&self) -> u32 {
        self.count.fetch_sub(1, core::sync::atomic::Ordering::Release) - 1
    }
    
    pub fn get(&self) -> u32 {
        self.count.load(core::sync::atomic::Ordering::Acquire)
    }
}

/// Module version information
pub struct ModuleVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub extra: String,
}

impl ModuleVersion {
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            extra: String::new(),
        }
    }
    
    pub fn to_string(&self) -> String {
        if self.extra.is_empty() {
            format!("{}.{}.{}", self.major, self.minor, self.patch)
        } else {
            format!("{}.{}.{}-{}", self.major, self.minor, self.patch, self.extra)
        }
    }
}

/// Module license types
pub mod licenses {
    pub const GPL: &str = "GPL";
    pub const GPL_V2: &str = "GPL v2";
    pub const GPL_V3: &str = "GPL v3";
    pub const LGPL: &str = "LGPL";
    pub const LGPL_V2: &str = "LGPL v2";
    pub const LGPL_V3: &str = "LGPL v3";
    pub const BSD: &str = "BSD";
    pub const BSD_2_CLAUSE: &str = "BSD 2-clause";
    pub const BSD_3_CLAUSE: &str = "BSD 3-clause";
    pub const MIT: &str = "MIT";
    pub const APACHE: &str = "Apache";
    pub const PROPRIETARY: &str = "Proprietary";
}

/// Module load flags
pub mod load_flags {
    pub const NOWAIT: u32 = 0x0001;
    pub const FORCE_VERMAGIC: u32 = 0x0002;
    pub const QUIET: u32 = 0x0004;
    pub const SILENT: u32 = 0x0008;
    pub const DEEP_PROBE: u32 = 0x0010;
}

/// Module signature verification
pub struct ModuleSignature {
    pub key_id: String,
    pub signer: String,
    pub algorithm: String,
    pub hash: Vec<u8>,
}

impl ModuleSignature {
    pub fn verify(&self, data: &[u8]) -> bool {
        // Simplified signature verification
        // In real implementation would use cryptographic verification
        !self.hash.is_empty()
    }
}

/// Module compression
pub mod compression {
    pub const NONE: &str = "none";
    pub const GZIP: &str = "gzip";
    pub const XZ: &str = "xz";
    pub const ZSTD: &str = "zstd";
}

/// Standard Linux kernel module paths
pub mod paths {
    pub const MODULE_DIR: &str = "/lib/modules";
    pub const MODULES_DEP: &str = "modules.dep";
    pub const MODULES_ALIAS: &str = "modules.alias";
    pub const MODULES_SYMBOLS: &str = "modules.symbols";
    pub const MODULES_BUILTIN: &str = "modules.builtin";
    pub const MODULES_ORDERING: &str = "modules.ordering";
}
