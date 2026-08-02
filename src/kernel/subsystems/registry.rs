#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

use core::sync::atomic::{AtomicUsize, Ordering};
/// SigmaOS Kernel Subsystem Registry
/// Inspired by Linux initcall mechanism — provides ordered, dependency-aware subsystem boot
/// OOP-based: every kernel module implements the KernelSubsystem trait
use crate::klib::HashMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// Initialization priority — mirrors Linux initcall levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InitOrder {
    EarlyBoot = 0,  // early_initcall — memory, IRQ
    CoreKernel = 1, // core_initcall  — core subsystems
    PostCore = 2,   // postcore_initcall — devices
    Arch = 3,       // arch_initcall — arch-specific
    Subsystem = 4,  // subsys_initcall — filesystem, net
    Filesystem = 5, // fs_initcall — vfs, proc
    Device = 6,     // device_initcall — drivers
    Late = 7,       // late_initcall — optional/user-mode
}

/// Subsystem lifecycle state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubsystemState {
    Unregistered,
    Registered,
    Initialized,
    Running,
    Suspended,
    ShutDown,
    Error,
}

/// Subsystem priority for resource conflict resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SubsystemPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
    Optional = 4,
}

#[derive(Debug, Clone)]
pub enum SubsystemError {
    AlreadyRegistered(String),
    NotFound(String),
    InitFailed(String),
    DependencyMissing(String),
    StateConflict(String),
}

/// Core OOP trait every kernel subsystem must implement
/// Mirrors Linux's module_init/module_exit pattern
pub trait KernelSubsystem: Send + Sync {
    /// Unique human-readable name (e.g., "uart_8250", "slab_allocator")
    fn name(&self) -> &str;

    /// Semantic version string (e.g., "1.0.0")
    fn version(&self) -> &str;

    /// Init order — when in the boot sequence this subsystem initializes
    fn init_order(&self) -> InitOrder;

    /// Priority for conflict resolution
    fn priority(&self) -> SubsystemPriority {
        SubsystemPriority::Normal
    }

    /// List of subsystem names this depends on (must be initialized first)
    fn dependencies(&self) -> Vec<&'static str> {
        Vec::new()
    }

    /// Initialize the subsystem — called once during boot
    fn initialize(&mut self) -> Result<(), SubsystemError>;

    /// Graceful shutdown
    fn shutdown(&mut self) -> Result<(), SubsystemError>;

    /// Called on system suspend (ACPI S3)
    fn suspend(&mut self) -> Result<(), SubsystemError> {
        Ok(())
    }

    /// Called on system resume
    fn resume(&mut self) -> Result<(), SubsystemError> {
        Ok(())
    }

    /// Health check — returns true if subsystem is healthy
    fn health_check(&self) -> bool {
        true
    }

    /// Human-readable status string
    fn status(&self) -> String {
        format!("{} v{} — OK", self.name(), self.version())
    }
}

/// Global registry of all kernel subsystems
/// Implements dependency-ordered initialization similar to Linux initcall_t
pub struct SubsystemRegistry {
    subsystems: Vec<Box<dyn KernelSubsystem>>,
    states: HashMap<String, SubsystemState>,
    init_count: AtomicUsize,
}

impl SubsystemRegistry {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SubsystemRegistry {
            subsystems: Vec::new(),
            states: HashMap::new(),
            init_count: AtomicUsize::new(0),
        }
    }

    /// Register a subsystem — analogous to module_init()
    pub fn register(&mut self, subsystem: Box<dyn KernelSubsystem>) -> Result<(), SubsystemError> {
        let name = subsystem.name().to_string();
        if self.states.contains_key(&name) {
            return Err(SubsystemError::AlreadyRegistered(name));
        }
        self.states.insert(name, SubsystemState::Registered);
        self.subsystems.push(subsystem);
        Ok(())
    }

    /// Boot all registered subsystems in dependency + init_order sequence
    pub fn boot_all(&mut self) -> Result<usize, SubsystemError> {
        // Sort by init_order then priority
        self.subsystems.sort_by(|a, b| {
            a.init_order()
                .cmp(&b.init_order())
                .then(a.priority().cmp(&b.priority()))
        });

        let mut count = 0usize;
        let names: Vec<String> = self
            .subsystems
            .iter()
            .map(|s| s.name().to_string())
            .collect();

        for i in 0..self.subsystems.len() {
            // Verify dependencies
            for dep in self.subsystems[i].dependencies() {
                let dep_state = self
                    .states
                    .get(dep)
                    .copied()
                    .unwrap_or(SubsystemState::Unregistered);
                if dep_state != SubsystemState::Initialized && dep_state != SubsystemState::Running
                {
                    return Err(SubsystemError::DependencyMissing(format!(
                        "{} requires {}",
                        names[i], dep
                    )));
                }
            }

            let result = self.subsystems[i].initialize();
            let name = names[i].clone();
            match result {
                Ok(()) => {
                    self.states.insert(name, SubsystemState::Running);
                    count += 1;
                    self.init_count.fetch_add(1, Ordering::SeqCst);
                }
                Err(e) => {
                    self.states.insert(name, SubsystemState::Error);
                    // Non-critical subsystems: log and continue
                    if self.subsystems[i].priority() == SubsystemPriority::Optional {
                        continue;
                    }
                    return Err(e);
                }
            }
        }
        Ok(count)
    }

    /// Shutdown all subsystems in reverse init order
    pub fn shutdown_all(&mut self) -> usize {
        let mut count = 0usize;
        for subsystem in self.subsystems.iter_mut().rev() {
            if let Ok(()) = subsystem.shutdown() {
                let name = subsystem.name().to_string();
                self.states.insert(name, SubsystemState::ShutDown);
                count += 1;
            }
        }
        count
    }

    pub fn get_state(&self, name: &str) -> SubsystemState {
        self.states
            .get(name)
            .copied()
            .unwrap_or(SubsystemState::Unregistered)
    }

    pub fn count(&self) -> usize {
        self.subsystems.len()
    }

    pub fn initialized_count(&self) -> usize {
        self.init_count.load(Ordering::SeqCst)
    }
}

impl Default for SubsystemRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockSubsystem {
        name: &'static str,
        initialized: bool,
    }

    impl KernelSubsystem for MockSubsystem {
        fn name(&self) -> &str {
            self.name
        }
        fn version(&self) -> &str {
            "1.0.0"
        }
        fn init_order(&self) -> InitOrder {
            InitOrder::Device
        }
        fn initialize(&mut self) -> Result<(), SubsystemError> {
            self.initialized = true;
            Ok(())
        }
        fn shutdown(&mut self) -> Result<(), SubsystemError> {
            Ok(())
        }
    }

    #[test]
    fn test_registry_creation() {
        let reg = SubsystemRegistry::new();
        assert_eq!(reg.count(), 0);
    }

    #[test]
    fn test_register_and_boot() {
        let mut reg = SubsystemRegistry::new();
        reg.register(Box::new(MockSubsystem {
            name: "uart_8250",
            initialized: false,
        }))
        .unwrap();
        reg.register(Box::new(MockSubsystem {
            name: "ne2000",
            initialized: false,
        }))
        .unwrap();
        let booted = reg.boot_all().unwrap();
        assert_eq!(booted, 2);
        assert_eq!(reg.get_state("uart_8250"), SubsystemState::Running);
    }

    #[test]
    fn test_duplicate_registration() {
        let mut reg = SubsystemRegistry::new();
        reg.register(Box::new(MockSubsystem {
            name: "uart_8250",
            initialized: false,
        }))
        .unwrap();
        let res = reg.register(Box::new(MockSubsystem {
            name: "uart_8250",
            initialized: false,
        }));
        assert!(matches!(res, Err(SubsystemError::AlreadyRegistered(_))));
    }

    #[test]
    fn test_shutdown_all() {
        let mut reg = SubsystemRegistry::new();
        reg.register(Box::new(MockSubsystem {
            name: "test_sub",
            initialized: false,
        }))
        .unwrap();
        reg.boot_all().unwrap();
        let shut = reg.shutdown_all();
        assert_eq!(shut, 1);
    }
}
