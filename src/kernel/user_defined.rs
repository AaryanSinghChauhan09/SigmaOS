// SigmaOS User-Defined Kernel Extensions (S-EXTENSION)
// Implements safe OOP extension points and traits (User-Defined First Principle)
// Enables users to define custom schedulers, allocators, and filesystem behaviors.

extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::kernel::scheduler::{Process, ProcessState, Priority};
use crate::kernel::memory::MemoryBlock;

/// Custom Scheduler Policy interface (OOP user-defined extension point)
pub trait ISchedulerPolicy {
    fn select_next_task(&mut self, processes: &mut [Process]) -> Option<u64>;
    fn on_task_tick(&mut self, active_pid: u64);
}

/// Custom Memory Allocator Policy interface (OOP user-defined extension point)
pub trait IAllocatorPolicy {
    fn allocate_block(&mut self, size: usize) -> Result<MemoryBlock, ()>;
    fn deallocate_block(&mut self, block: MemoryBlock);
}

/// Custom Filesystem Plugin interface (OOP user-defined extension point)
pub trait IFilesystemPlugin {
    fn name(&self) -> &'static str;
    fn read_block(&self, sector: u64, buffer: &mut [u8]) -> Result<usize, ()>;
    fn write_block(&mut self, sector: u64, data: &[u8]) -> Result<usize, ()>;
}

/// User-Defined Extension Registry (Zero-Trust capability gated)
pub struct UserDefinedExtensionRegistry {
    pub scheduler_policy: Option<Box<dyn ISchedulerPolicy>>,
    pub allocator_policy: Option<Box<dyn IAllocatorPolicy>>,
    pub fs_plugins: Vec<Box<dyn IFilesystemPlugin>>,
    pub capabilities_gated: bool,
}

impl UserDefinedExtensionRegistry {
    pub fn new() -> Self {
        Self {
            scheduler_policy: None,
            allocator_policy: None,
            fs_plugins: Vec::new(),
            capabilities_gated: true,
        }
    }

    pub fn register_scheduler_policy(
        &mut self,
        policy: Box<dyn ISchedulerPolicy>,
    ) -> Result<(), &'static str> {
        if !self.capabilities_gated {
            return Err("CapabilityDenied: Cannot register kernel-level scheduler policy");
        }
        self.scheduler_policy = Some(policy);
        Ok(())
    }

    pub fn register_allocator_policy(
        &mut self,
        policy: Box<dyn IAllocatorPolicy>,
    ) -> Result<(), &'static str> {
        if !self.capabilities_gated {
            return Err("CapabilityDenied: Cannot register kernel-level allocator policy");
        }
        self.allocator_policy = Some(policy);
        Ok(())
    }

    pub fn register_fs_plugin(&mut self, plugin: Box<dyn IFilesystemPlugin>) -> Result<(), &'static str> {
        if !self.capabilities_gated {
            return Err("CapabilityDenied: Cannot register kernel-level VFS plugin");
        }
        self.fs_plugins.push(plugin);
        Ok(())
    }
}

impl Default for UserDefinedExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::time::Duration;

    /// Dummy Stride Scheduler implementation representing user-defined scheduling policies
    struct UserStrideScheduler {
        pub active_tickets: u32,
    }

    impl ISchedulerPolicy for UserStrideScheduler {
        fn select_next_task(&mut self, processes: &mut [Process]) -> Option<u64> {
            if processes.is_empty() {
                return None;
            }
            // Select first ready process
            processes.iter()
                .find(|p| p.state == ProcessState::Ready)
                .map(|p| p.pid)
        }

        fn on_task_tick(&mut self, _active_pid: u64) {
            self.active_tickets += 1;
        }
    }

    /// Dummy Buddy Allocator sub-policy representing user-defined allocation controls
    struct UserBuddyAllocatorPolicy {
        pub allocations_count: usize,
    }

    impl IAllocatorPolicy for UserBuddyAllocatorPolicy {
        fn allocate_block(&mut self, size: usize) -> Result<MemoryBlock, ()> {
            self.allocations_count += 1;
            Ok(MemoryBlock {
                address: 0x500000,
                size,
                free: false,
            })
        }

        fn deallocate_block(&mut self, _block: MemoryBlock) {
            self.allocations_count = self.allocations_count.saturating_sub(1);
        }
    }

    /// Dummy Encrypted Block storage overlay representing user-defined VFS plugins
    struct UserEncryptionFSPlugin {
        pub key_hash: u32,
    }

    impl IFilesystemPlugin for UserEncryptionFSPlugin {
        fn name(&self) -> &'static str {
            "SovereignEncryptionPlugin"
        }

        fn read_block(&self, _sector: u64, buffer: &mut [u8]) -> Result<usize, ()> {
            if buffer.len() > 0 {
                buffer[0] = 0xFF; // simulated decrypt
            }
            Ok(buffer.len())
        }

        fn write_block(&mut self, _sector: u64, data: &[u8]) -> Result<usize, ()> {
            Ok(data.len())
        }
    }

    #[test]
    fn test_user_defined_scheduler_policy() {
        let mut registry = UserDefinedExtensionRegistry::new();
        let policy = Box::new(UserStrideScheduler { active_tickets: 100 });
        assert!(registry.register_scheduler_policy(policy).is_ok());

        let mut processes = [
            Process::new(1, "shell".into(), Priority::Normal),
            Process::new(2, "ide".into(), Priority::High),
        ];

        let selected = registry.scheduler_policy.as_mut().unwrap().select_next_task(&mut processes);
        assert_eq!(selected, Some(1));
    }

    #[test]
    fn test_user_defined_allocator_policy() {
        let mut registry = UserDefinedExtensionRegistry::new();
        let policy = Box::new(UserBuddyAllocatorPolicy { allocations_count: 0 });
        assert!(registry.register_allocator_policy(policy).is_ok());

        let block = registry.allocator_policy.as_mut().unwrap().allocate_block(4096).unwrap();
        assert_eq!(block.address, 0x500000);
        assert_eq!(block.size, 4096);
    }

    #[test]
    fn test_user_defined_fs_plugins() {
        let mut registry = UserDefinedExtensionRegistry::new();
        let plugin = Box::new(UserEncryptionFSPlugin { key_hash: 0x99AA });
        assert!(registry.register_fs_plugin(plugin).is_ok());

        assert_eq!(registry.fs_plugins[0].name(), "SovereignEncryptionPlugin");
        let mut buf = [0u8; 1];
        assert_eq!(registry.fs_plugins[0].read_block(1, &mut buf), Ok(1));
        assert_eq!(buf[0], 0xFF);
    }
}
