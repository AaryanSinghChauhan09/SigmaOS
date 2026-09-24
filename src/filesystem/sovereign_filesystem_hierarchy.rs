//! Sovereign Canonical Filesystem Hierarchy Specification Module for SigmaOS
//!
//! Implements `#![no_std]` compliant dual-layer hybrid filesystem hierarchy:
//! - Canonical FHS VFS Symlink Overlay Resolver (`SovereignCanonicalFhsResolver`)
//! - Dynamic Synthetic Procfs & Sysfs Kernel Provider (`SyntheticProcSysfsProvider`)
//! - RAM-Backed Volatile tmpfs Mount Governor (`EphemeralTmpfsMountGovernor`)
//! - Atomic Generation Rootfs & Store Guard (`SovereignAtomicGenerationRootfsGuard`)

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

/// Canonical VFS Symlink Overlay Resolver
#[derive(Debug, Clone)]
pub struct SovereignCanonicalFhsResolver {
    pub legacy_mappings: BTreeMap<String, String>,
}

impl SovereignCanonicalFhsResolver {
    pub fn new() -> Self {
        let mut map = BTreeMap::new();
        map.insert("/bin".to_string(), "/system/current/bin".to_string());
        map.insert("/sbin".to_string(), "/system/current/bin".to_string());
        map.insert("/usr/bin".to_string(), "/system/current/bin".to_string());
        map.insert("/usr/sbin".to_string(), "/system/current/bin".to_string());
        map.insert("/usr/lib".to_string(), "/system/current/lib".to_string());
        map.insert("/lib64".to_string(), "/system/current/lib".to_string());
        map.insert("/lib".to_string(), "/system/current/lib".to_string());
        map.insert("/etc".to_string(), "/state/etc".to_string());
        map.insert("/var".to_string(), "/state/var".to_string());
        map.insert("/opt".to_string(), "/state/opt".to_string());

        Self { legacy_mappings: map }
    }

    pub fn resolve_path(&self, requested_path: &str) -> String {
        for (legacy, canonical) in &self.legacy_mappings {
            if requested_path == legacy || requested_path.starts_with(&format!("{}/", legacy)) {
                return requested_path.replacen(legacy, canonical, 1);
            }
        }
        requested_path.to_string()
    }
}

impl Default for SovereignCanonicalFhsResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Dynamic Synthetic Procfs & Sysfs Kernel Read Provider
#[derive(Debug, Clone)]
pub struct SyntheticProcSysfsProvider {
    pub hostname: String,
    pub total_mem_mb: u64,
    pub free_mem_mb: u64,
    pub cpu_cores: u32,
}

impl SyntheticProcSysfsProvider {
    pub fn new() -> Self {
        Self {
            hostname: "sovereign-node".to_string(),
            total_mem_mb: 16384,
            free_mem_mb: 12288,
            cpu_cores: 8,
        }
    }

    pub fn read_synthetic_file(&self, path: &str) -> Option<String> {
        match path {
            "/proc/meminfo" => Some(format!(
                "MemTotal:        {} kB\nMemFree:         {} kB\nMemAvailable:    {} kB\n",
                self.total_mem_mb * 1024,
                self.free_mem_mb * 1024,
                (self.free_mem_mb + 1024) * 1024
            )),
            "/proc/cpuinfo" => Some(format!(
                "processor: 0\nvendor_id: GenuineIntel\nmodel name: Sovereign CPU Core\ncpu cores: {}\n",
                self.cpu_cores
            )),
            "/proc/sys/kernel/hostname" => Some(format!("{}\n", self.hostname)),
            "/sys/fs/cgroup/cgroup.controllers" => Some("cpu memory io pids\n".to_string()),
            _ => None,
        }
    }
}

impl Default for SyntheticProcSysfsProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// Ephemeral RAM-backed tmpfs Mount Governor (/tmp and /run)
#[derive(Debug, Clone)]
pub struct EphemeralTmpfsMountGovernor {
    pub run_max_size_mb: u64,
    pub tmp_max_size_mb: u64,
    pub active_tmp_files: Vec<String>,
}

impl EphemeralTmpfsMountGovernor {
    pub fn new() -> Self {
        Self {
            run_max_size_mb: 512,
            tmp_max_size_mb: 4096,
            active_tmp_files: Vec::new(),
        }
    }

    pub fn allocate_tmp_file(&mut self, file_name: &str) -> String {
        let full_path = format!("/tmp/{}", file_name);
        self.active_tmp_files.push(full_path.clone());
        full_path
    }

    pub fn purge_volatile_tmp(&mut self) -> usize {
        let count = self.active_tmp_files.len();
        self.active_tmp_files.clear();
        count
    }
}

impl Default for EphemeralTmpfsMountGovernor {
    fn default() -> Self {
        Self::new()
    }
}

/// Sovereign Atomic Generation Rootfs & Store Guard
#[derive(Debug, Clone)]
pub struct SovereignAtomicGenerationRootfsGuard {
    pub active_generation: u32,
    pub store_generations: Vec<u32>,
    pub is_cleanroom_factory: bool,
}

impl SovereignAtomicGenerationRootfsGuard {
    pub fn new() -> Self {
        Self {
            active_generation: 1,
            store_generations: vec![1],
            is_cleanroom_factory: true,
        }
    }

    pub fn create_generation(&mut self) -> u32 {
        let next_gen = self.active_generation + 1;
        self.store_generations.push(next_gen);
        self.active_generation = next_gen;
        next_gen
    }

    pub fn rollback_generation(&mut self, gen: u32) -> bool {
        if self.store_generations.contains(&gen) {
            self.active_generation = gen;
            true
        } else {
            false
        }
    }

    pub fn perform_factory_reset(&mut self) -> String {
        self.active_generation = 1;
        self.store_generations.retain(|&g| g == 1);
        "Factory reset complete. Reverted system state to cleanroom factory defaults.".to_string()
    }
}

impl Default for SovereignAtomicGenerationRootfsGuard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_fhs_resolver() {
        let resolver = SovereignCanonicalFhsResolver::new();
        assert_eq!(resolver.resolve_path("/usr/bin/bash"), "/system/current/bin/bash");
        assert_eq!(resolver.resolve_path("/lib64/libc.so.6"), "/system/current/lib/libc.so.6");
        assert_eq!(resolver.resolve_path("/etc/os-release"), "/state/etc/os-release");
    }

    #[test]
    fn test_synthetic_proc_sysfs_provider() {
        let provider = SyntheticProcSysfsProvider::new();
        let mem = provider.read_synthetic_file("/proc/meminfo").unwrap();
        assert!(mem.contains("MemTotal:"));
        let host = provider.read_synthetic_file("/proc/sys/kernel/hostname").unwrap();
        assert_eq!(host.trim(), "sovereign-node");
    }

    #[test]
    fn test_ephemeral_tmpfs_governor() {
        let mut tmpfs = EphemeralTmpfsMountGovernor::new();
        tmpfs.allocate_tmp_file("session.lock");
        assert_eq!(tmpfs.active_tmp_files.len(), 1);
        let purged = tmpfs.purge_volatile_tmp();
        assert_eq!(purged, 1);
        assert!(tmpfs.active_tmp_files.is_empty());
    }

    #[test]
    fn test_atomic_generation_rootfs_guard() {
        let mut guard = SovereignAtomicGenerationRootfsGuard::new();
        let gen2 = guard.create_generation();
        assert_eq!(gen2, 2);
        assert!(guard.rollback_generation(1));
        assert_eq!(guard.active_generation, 1);
        let reset_msg = guard.perform_factory_reset();
        assert!(reset_msg.contains("Factory reset complete"));
    }
}
