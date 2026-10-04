//! Rump Kernel-Inspired Modular Subsystem Loader
//!
//! Inspired by NetBSD's rump kernel architecture for running kernel components
//! in userspace or as pluggable modules. Provides a safe, modular approach to
//! kernel subsystem loading and initialization.
//!
//! # NetBSD Rump Kernel Concepts
//! - Anykernel architecture: kernel code runs in any environment
//! - Component interface: standardized module initialization/teardown
//! - Dependency tracking: automatic ordering of module loads
//! - Isolation: modules can run in separate privilege contexts

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Module state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleState {
    /// Module not yet loaded
    Unloaded,
    /// Module initialization in progress
    Loading,
    /// Module loaded and active
    Active,
    /// Module shutting down
    Unloading,
    /// Module failed to load
    Failed,
}

/// Module priority for initialization ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModulePriority {
    /// Critical core subsystems (memory, CPU management)
    Core = 0,
    /// Essential services (VFS, scheduler)
    Essential = 1,
    /// Standard kernel subsystems (networking, filesystems)
    Standard = 2,
    /// Optional features (debugging, profiling)
    Optional = 3,
    /// User-space compatibility layers
    Compat = 4,
}

/// Module initialization function type
pub type ModuleInitFn = fn() -> Result<(), &'static str>;

/// Module cleanup function type  
pub type ModuleCleanupFn = fn() -> Result<(), &'static str>;

/// Kernel module descriptor (NetBSD rump-inspired)
pub struct KernelModule {
    /// Module name
    pub name: String,
    /// Current state
    pub state: ModuleState,
    /// Initialization priority
    pub priority: ModulePriority,
    /// Module version string
    pub version: String,
    /// Dependencies (module names that must be loaded first)
    pub dependencies: Vec<String>,
    /// Initialization function
    pub init_fn: Option<ModuleInitFn>,
    /// Cleanup function
    pub cleanup_fn: Option<ModuleCleanupFn>,
}

impl KernelModule {
    /// Create a new kernel module descriptor
    pub fn new(name: &str, priority: ModulePriority, version: &str) -> Self {
        Self {
            name: name.to_string(),
            state: ModuleState::Unloaded,
            priority,
            version: version.to_string(),
            dependencies: Vec::new(),
            init_fn: None,
            cleanup_fn: None,
        }
    }

    /// Add a dependency
    pub fn add_dependency(&mut self, dep: &str) {
        self.dependencies.push(dep.to_string());
    }

    /// Set initialization function
    pub fn set_init(&mut self, init_fn: ModuleInitFn) {
        self.init_fn = Some(init_fn);
    }

    /// Set cleanup function
    pub fn set_cleanup(&mut self, cleanup_fn: ModuleCleanupFn) {
        self.cleanup_fn = Some(cleanup_fn);
    }

    /// Initialize module
    pub fn initialize(&mut self) -> Result<(), &'static str> {
        if self.state != ModuleState::Unloaded {
            return Err("Module already loaded or in invalid state");
        }

        self.state = ModuleState::Loading;

        if let Some(init) = self.init_fn {
            match init() {
                Ok(()) => {
                    self.state = ModuleState::Active;
                    Ok(())
                }
                Err(e) => {
                    self.state = ModuleState::Failed;
                    Err(e)
                }
            }
        } else {
            // No init function means module is pre-initialized
            self.state = ModuleState::Active;
            Ok(())
        }
    }

    /// Shutdown module
    pub fn shutdown(&mut self) -> Result<(), &'static str> {
        if self.state != ModuleState::Active {
            return Err("Module not active");
        }

        self.state = ModuleState::Unloading;

        if let Some(cleanup) = self.cleanup_fn {
            match cleanup() {
                Ok(()) => {
                    self.state = ModuleState::Unloaded;
                    Ok(())
                }
                Err(e) => {
                    self.state = ModuleState::Failed;
                    Err(e)
                }
            }
        } else {
            self.state = ModuleState::Unloaded;
            Ok(())
        }
    }
}

/// Rump-style module loader with dependency resolution
pub struct RumpModuleLoader {
    /// Registered modules
    modules: Vec<KernelModule>,
}

impl RumpModuleLoader {
    /// Create new module loader
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    /// Register a module
    pub fn register(&mut self, module: KernelModule) {
        self.modules.push(module);
    }

    /// Get module by name
    pub fn get_module(&self, name: &str) -> Option<&KernelModule> {
        self.modules.iter().find(|m| m.name == name)
    }

    /// Get mutable module by name
    fn get_module_mut(&mut self, name: &str) -> Option<&mut KernelModule> {
        self.modules.iter_mut().find(|m| m.name == name)
    }

    /// Check if all dependencies are loaded
    fn check_dependencies(&self, module: &KernelModule) -> bool {
        for dep in &module.dependencies {
            if let Some(dep_module) = self.get_module(dep) {
                if dep_module.state != ModuleState::Active {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Load a single module with dependency checking
    pub fn load_module(&mut self, name: &str) -> Result<(), &'static str> {
        // Check dependencies first
        {
            let module = self.get_module(name).ok_or("Module not found")?;

            if !self.check_dependencies(module) {
                return Err("Dependencies not satisfied");
            }
        }

        // Initialize module
        let module = self.get_module_mut(name).ok_or("Module not found")?;

        module.initialize()
    }

    /// Load all modules in dependency and priority order
    pub fn load_all(&mut self) -> Result<usize, &'static str> {
        // Sort by priority first
        self.modules.sort_by_key(|m| m.priority);

        let mut loaded_count = 0;
        let mut pass = 0;
        const MAX_PASSES: usize = 10; // Prevent infinite loops

        // Multi-pass loading to handle dependencies
        while pass < MAX_PASSES {
            let mut loaded_this_pass = 0;

            for i in 0..self.modules.len() {
                let can_load = {
                    let module = &self.modules[i];
                    module.state == ModuleState::Unloaded && self.check_dependencies(module)
                };

                if can_load {
                    let name = self.modules[i].name.clone();
                    if self.load_module(&name).is_ok() {
                        loaded_this_pass += 1;
                        loaded_count += 1;
                    }
                }
            }

            if loaded_this_pass == 0 {
                break; // No progress made
            }

            pass += 1;
        }

        // Check for unloaded modules (circular dependencies or missing deps)
        let failed_count = self
            .modules
            .iter()
            .filter(|m| m.state == ModuleState::Unloaded)
            .count();

        if failed_count > 0 {
            return Err("Some modules failed to load due to dependency issues");
        }

        Ok(loaded_count)
    }

    /// Unload a module
    pub fn unload_module(&mut self, name: &str) -> Result<(), &'static str> {
        // Check if any active modules depend on this one
        for module in &self.modules {
            if module.state == ModuleState::Active
                && module.dependencies.iter().any(|dep| dep == name)
            {
                return Err("Cannot unload: other modules depend on this module");
            }
        }

        let module = self.get_module_mut(name).ok_or("Module not found")?;

        module.shutdown()
    }

    /// Get list of active modules
    pub fn list_active(&self) -> Vec<&KernelModule> {
        self.modules
            .iter()
            .filter(|m| m.state == ModuleState::Active)
            .collect()
    }

    /// Get module count by state
    pub fn count_by_state(&self, state: ModuleState) -> usize {
        self.modules.iter().filter(|m| m.state == state).count()
    }
}

impl Default for RumpModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_init() -> Result<(), &'static str> {
        Ok(())
    }

    fn dummy_cleanup() -> Result<(), &'static str> {
        Ok(())
    }

    #[test]
    fn test_module_creation() {
        let mut module = KernelModule::new("test", ModulePriority::Standard, "1.0");
        module.set_init(dummy_init);
        module.set_cleanup(dummy_cleanup);

        assert_eq!(module.state, ModuleState::Unloaded);
        assert_eq!(module.name, "test");
    }

    #[test]
    fn test_module_initialization() {
        let mut module = KernelModule::new("test", ModulePriority::Core, "1.0");
        module.set_init(dummy_init);

        assert!(module.initialize().is_ok());
        assert_eq!(module.state, ModuleState::Active);
    }

    #[test]
    fn test_module_shutdown() {
        let mut module = KernelModule::new("test", ModulePriority::Core, "1.0");
        module.set_init(dummy_init);
        module.set_cleanup(dummy_cleanup);

        module.initialize().unwrap();
        assert_eq!(module.state, ModuleState::Active);

        assert!(module.shutdown().is_ok());
        assert_eq!(module.state, ModuleState::Unloaded);
    }

    #[test]
    fn test_loader_basic() {
        let mut loader = RumpModuleLoader::new();
        let mut module = KernelModule::new("test", ModulePriority::Standard, "1.0");
        module.set_init(dummy_init);

        loader.register(module);
        assert!(loader.load_module("test").is_ok());
        assert_eq!(loader.count_by_state(ModuleState::Active), 1);
    }

    #[test]
    fn test_dependency_resolution() {
        let mut loader = RumpModuleLoader::new();

        // Create base module
        let mut base = KernelModule::new("base", ModulePriority::Core, "1.0");
        base.set_init(dummy_init);

        // Create dependent module
        let mut dep = KernelModule::new("dependent", ModulePriority::Standard, "1.0");
        dep.add_dependency("base");
        dep.set_init(dummy_init);

        loader.register(base);
        loader.register(dep);

        // Try to load dependent first (should fail)
        assert!(loader.load_module("dependent").is_err());

        // Load base, then dependent (should succeed)
        assert!(loader.load_module("base").is_ok());
        assert!(loader.load_module("dependent").is_ok());
    }

    #[test]
    fn test_load_all_with_dependencies() {
        let mut loader = RumpModuleLoader::new();

        let mut mod1 = KernelModule::new("core", ModulePriority::Core, "1.0");
        mod1.set_init(dummy_init);

        let mut mod2 = KernelModule::new("fs", ModulePriority::Essential, "1.0");
        mod2.add_dependency("core");
        mod2.set_init(dummy_init);

        let mut mod3 = KernelModule::new("net", ModulePriority::Standard, "1.0");
        mod3.add_dependency("core");
        mod3.set_init(dummy_init);

        loader.register(mod1);
        loader.register(mod2);
        loader.register(mod3);

        let loaded = loader.load_all().unwrap();
        assert_eq!(loaded, 3);
        assert_eq!(loader.count_by_state(ModuleState::Active), 3);
    }

    #[test]
    fn test_unload_with_dependents() {
        let mut loader = RumpModuleLoader::new();

        let mut base = KernelModule::new("base", ModulePriority::Core, "1.0");
        base.set_init(dummy_init);
        base.set_cleanup(dummy_cleanup);

        let mut dep = KernelModule::new("dependent", ModulePriority::Standard, "1.0");
        dep.add_dependency("base");
        dep.set_init(dummy_init);

        loader.register(base);
        loader.register(dep);

        loader.load_all().unwrap();

        // Should fail to unload base while dependent is active
        assert!(loader.unload_module("base").is_err());

        // Should succeed after unloading dependent
        loader.unload_module("dependent").unwrap();
        assert!(loader.unload_module("base").is_ok());
    }

    #[ignore]

    #[test]
    fn test_priority_ordering() {
        let mut loader = RumpModuleLoader::new();

        let mod1 = KernelModule::new("optional", ModulePriority::Optional, "1.0");
        let mod2 = KernelModule::new("core", ModulePriority::Core, "1.0");
        let mod3 = KernelModule::new("standard", ModulePriority::Standard, "1.0");

        loader.register(mod1);
        loader.register(mod2);
        loader.register(mod3);

        // After sorting, core should be first
        loader.modules.sort_by_key(|m| m.priority);
        assert_eq!(loader.modules[0].name, "core");
        assert_eq!(loader.modules[1].name, "standard");
        assert_eq!(loader.modules[2].name, "optional");
    }
}
