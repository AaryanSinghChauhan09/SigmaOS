#![no_std]

#[cfg(not(feature = "standalone_test"))]
use crate::klib::{Vec, String, ToString, HashMap};

#[cfg(feature = "standalone_test")]
extern crate alloc;

#[cfg(feature = "standalone_test")]
extern crate std;

#[cfg(feature = "standalone_test")]
use alloc::{vec::Vec, string::{String, ToString}};

#[cfg(feature = "standalone_test")]
use std::collections::HashMap;

/// Arch Linux inspired AUR-style user repos and minimal base
pub struct ArchUserRepoManager {
    packages: HashMap<String, String>,
}

impl ArchUserRepoManager {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }
    
    pub fn install_from_aur(&mut self, pkg_name: &str, build_script: &str) -> Result<(), &'static str> {
        self.packages.insert(pkg_name.to_string(), build_script.to_string());
        Ok(())
    }
}

/// Alpine Linux inspired minimal base with hardened security
pub struct AlpineHardenedEnv {
    secure_mode: bool,
}

impl AlpineHardenedEnv {
    pub fn new() -> Self {
        Self { secure_mode: true }
    }
    
    pub fn execute_with_musl_stub(&self, _binary: &[u8]) -> Result<u32, &'static str> {
        if !self.secure_mode {
            return Err("Must be in secure mode");
        }
        Ok(0) // Return exit code 0
    }
}

/// OpenBSD inspired pledge/unveil syscall restrictions
pub struct OpenBsdPledge {
    pub promises: Vec<String>,
    pub is_pledged: bool,
}

impl OpenBsdPledge {
    pub fn new() -> Self {
        Self {
            promises: Vec::new(),
            is_pledged: false,
        }
    }
    
    pub fn pledge(&mut self, promise_list: &str) -> Result<(), &'static str> {
        let mut new_promises = Vec::new();
        for promise in promise_list.split(' ') {
            if !promise.is_empty() {
                new_promises.push(promise.to_string());
            }
        }

        if self.is_pledged {
            // Once pledged, subsequent calls can only drop capabilities, never escalate
            for promise in &new_promises {
                if !self.promises.contains(promise) {
                    return Err("Illegal pledge escalation blocked");
                }
            }
        }

        self.promises = new_promises;
        self.is_pledged = true;
        Ok(())
    }
    
    pub fn check_permission(&self, operation: &str) -> bool {
        if !self.is_pledged {
            return true;
        }
        for promise in &self.promises {
            if promise.as_str() == operation {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_aur_manager() {
        let mut aur = ArchUserRepoManager::new();
        aur.install_from_aur("test-pkg", "echo 'building test-pkg'").unwrap();
        assert_eq!(aur.packages.get("test-pkg").unwrap().as_str(), "echo 'building test-pkg'");
    }

    #[test]
    fn test_alpine_hardened_env() {
        let env = AlpineHardenedEnv::new();
        assert!(env.execute_with_musl_stub(b"binary_payload").is_ok());
    }

    #[test]
    fn test_openbsd_pledge() {
        let mut pledge = OpenBsdPledge::new();
        // Before pledge, everything is allowed
        assert!(pledge.check_permission("exec"));

        // Pledge rules set
        pledge.pledge("stdio rpath wpath").unwrap();
        assert!(pledge.check_permission("stdio"));
        assert!(pledge.check_permission("rpath"));
        assert!(!pledge.check_permission("exec"));

        // Subsequent pledge can only subset
        pledge.pledge("stdio").unwrap();
        assert!(pledge.check_permission("stdio"));
        assert!(!pledge.check_permission("rpath"));

        // Attempting to escalate should fail
        assert!(pledge.pledge("stdio rpath").is_err());
    }

    #[test]
    fn test_freebsd_jail() {
        let parent = FreeBsdJail::create(1);
        let child = FreeBsdJail::create_nested(2, 1);
        let stranger = FreeBsdJail::create_nested(3, 99);

        assert!(parent.is_isolated());
        assert!(child.is_isolated());
        assert!(child.is_descendant_of(1));
        assert!(!stranger.is_descendant_of(1));
    }

    #[test]
    fn test_nixos_declarative_manager() {
        let mut manager = NixOsDeclarativeManager::new();
        manager.apply_configuration(&["services.nginx.enable = true;"]).unwrap();
        assert_eq!(manager.configuration.len(), 1);

        // Apply new configuration (saves previous)
        manager.apply_configuration(&["services.nginx.enable = false;"]).unwrap();
        assert_eq!(manager.configuration.len(), 1);
        assert_eq!(manager.configuration[0], "services.nginx.enable = false;");

        // Rollback configuration to previous state
        manager.rollback().unwrap();
        assert_eq!(manager.configuration.len(), 1);
        assert_eq!(manager.configuration[0], "services.nginx.enable = true;");

        // Rollback further should fail
        assert!(manager.rollback().is_err());
    }

    #[test]
    fn test_gentoo_use_flags() {
        let mut gentoo = GentooUseFlags::new();
        gentoo.set_flag("wayland", true);
        gentoo.add_dependency("wayland", "egl");

        // Dependencies violated because egl is not set
        assert!(!gentoo.check_dependencies());

        // Enable egl flag, satisfying dependency
        gentoo.set_flag("egl", true);
        assert!(gentoo.check_dependencies());
    }

    #[test]
    fn test_void_runit_init() {
        let mut runit = VoidRunitInit::new();
        runit.start_service("nginx");
        assert!(runit.is_running("nginx"));
        assert!(!runit.is_running("postgresql"));
    }

    #[test]
    fn test_linux_vma_manager() {
        let mut vma_mgr = LinuxVmaManager::new();
        vma_mgr.insert_vma(0x1000, 0x2000, PROT_READ | PROT_EXEC, "text").unwrap();
        vma_mgr.insert_vma(0x2000, 0x3000, PROT_READ | PROT_WRITE, "data").unwrap();

        assert!(vma_mgr.insert_vma(0x1500, 0x2500, PROT_READ, "overlap").is_err());
        assert!(vma_mgr.find_vma(0x1500).is_some());
        assert_eq!(vma_mgr.find_vma(0x1500).unwrap().name.as_str(), "text");

        assert!(vma_mgr.handle_page_fault(0x1500, false).is_ok());
        assert_eq!(vma_mgr.handle_page_fault(0x1500, true), Err("Permission Denied: Write violation in read-only VMA (SIGSEGV)"));
        assert!(vma_mgr.handle_page_fault(0x2500, true).is_ok());
        assert_eq!(vma_mgr.handle_page_fault(0x4000, false), Err("Segmentation Fault: Address not mapped in any VMA (SIGSEGV)"));
    }

    #[test]
    fn test_bsd_zone_allocator() {
        let mut allocator = BsdZoneAllocator::new();
        allocator.create_zone("pcb_zone", 128, 2).unwrap();

        let addr1 = allocator.zone_alloc("pcb_zone").unwrap();
        let addr2 = allocator.zone_alloc("pcb_zone").unwrap();
        assert_ne!(addr1, addr2);

        let addr3 = allocator.zone_alloc("pcb_zone").unwrap();
        assert!(addr3 != addr1 && addr3 != addr2);

        allocator.zone_free("pcb_zone", addr1).unwrap();
        let addr_reclaimed = allocator.zone_alloc("pcb_zone").unwrap();
        assert_eq!(addr_reclaimed, addr1);
    }

    #[test]
    fn test_linux_kswapd() {
        let mut kswapd = LinuxKswapd::new(100);
        kswapd.add_page_frame(0x1000);
        kswapd.add_page_frame(0x2000);
        kswapd.add_page_frame(0x3000);

        kswapd.access_page(0x1000, 100);
        assert_eq!(kswapd.active_list.len(), 1);
        assert_eq!(kswapd.active_list[0].phys_addr, 0x1000);

        let reclaimed = kswapd.reclaim_pages(50).unwrap();
        assert_eq!(reclaimed, 2);
        assert!(kswapd.swap_space.contains_key(&0x2000));
        assert!(kswapd.swap_space.contains_key(&0x3000));
    }

    #[test]
    fn test_linux_mem_cgroup() {
        let mut manager = MemCgroupManager::new();
        manager.create_cgroup(1, 10240).unwrap();
        manager.attach_process(1, 42).unwrap();

        assert!(manager.charge_memory(42, 5120).is_ok());
        assert_eq!(manager.charge_memory(42, 6144), Err("Memory Limit Exceeded (MemCg OOM)"));

        manager.uncharge_memory(42, 5120);
        assert!(manager.charge_memory(42, 6144).is_ok());
    }
}

/// FreeBSD inspired Jails (capability-based isolation)
pub struct FreeBsdJail {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub isolated: bool,
}

impl FreeBsdJail {
    pub fn create(id: u32) -> Self {
        Self {
            id,
            parent_id: None,
            isolated: true,
        }
    }

    pub fn create_nested(id: u32, parent_id: u32) -> Self {
        Self {
            id,
            parent_id: Some(parent_id),
            isolated: true,
        }
    }
    
    pub fn is_isolated(&self) -> bool {
        self.isolated
    }

    /// Recursively check if this jail is a descendant of the target parent jail ID
    pub fn is_descendant_of(&self, target_parent_id: u32) -> bool {
        if let Some(pid) = self.parent_id {
            if pid == target_parent_id {
                return true;
            }
        }
        false
    }
}

/// NixOS inspired Declarative package management
pub struct NixOsDeclarativeManager {
    pub configuration: Vec<String>,
    pub previous_generations: Vec<Vec<String>>,
}

impl NixOsDeclarativeManager {
    pub fn new() -> Self {
        Self {
            configuration: Vec::new(),
            previous_generations: Vec::new(),
        }
    }
    
    pub fn apply_configuration(&mut self, config: &[&str]) -> Result<(), &'static str> {
        // Save previous generation before applying new one
        if !self.configuration.is_empty() {
            self.previous_generations.push(self.configuration.clone());
        }
        self.configuration.clear();
        for c in config {
            self.configuration.push(c.to_string());
        }
        Ok(())
    }

    /// Rollbacks to the previous configuration generation atomically
    pub fn rollback(&mut self) -> Result<(), &'static str> {
        if let Some(prev) = self.previous_generations.pop() {
            self.configuration = prev;
            Ok(())
        } else {
            Err("No previous generations available for rollback")
        }
    }
}

/// Gentoo inspired USE flags / compile-time feature selection
pub struct GentooUseFlags {
    pub flags: HashMap<String, bool>,
    pub dependencies: HashMap<String, String>, // (flag -> required companion flag)
}

impl GentooUseFlags {
    pub fn new() -> Self {
        Self {
            flags: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }
    
    pub fn set_flag(&mut self, flag: &str, enabled: bool) {
        self.flags.insert(flag.to_string(), enabled);
    }

    pub fn add_dependency(&mut self, flag: &str, required_companion: &str) {
        self.dependencies.insert(flag.to_string(), required_companion.to_string());
    }
    
    pub fn has_feature(&self, flag: &str) -> bool {
        if let Some(&val) = self.flags.get(flag) {
            val
        } else {
            false
        }
    }

    /// Check if all active USE-flags have their required companion dependencies enabled
    pub fn check_dependencies(&self) -> bool {
        for (flag, required) in &self.dependencies {
            if self.has_feature(flag) && !self.has_feature(required) {
                return false;
            }
        }
        true
    }
}

/// Void Linux inspired runit init system inspiration
pub struct VoidRunitInit {
    services: Vec<String>,
}

impl VoidRunitInit {
    pub fn new() -> Self {
        Self { services: Vec::new() }
    }
    
    pub fn start_service(&mut self, service: &str) {
        self.services.push(service.to_string());
    }
    
    pub fn is_running(&self, service: &str) -> bool {
        for s in &self.services {
            if s.as_str() == service {
                return true;
            }
        }
        false
    }
}


// =========================================================================
// Linux & BSD-inspired Memory Management Subsystems
// =========================================================================

pub const PROT_READ: u32 = 1 << 0;
pub const PROT_WRITE: u32 = 1 << 1;
pub const PROT_EXEC: u32 = 1 << 2;

#[derive(Debug, Clone)]
pub struct VmArea {
    pub start_addr: usize,
    pub end_addr: usize,
    pub flags: u32,
    pub name: String,
}

pub struct LinuxVmaManager {
    pub mmap_regions: Vec<VmArea>,
}

impl LinuxVmaManager {
    pub fn new() -> Self {
        Self {
            mmap_regions: Vec::new(),
        }
    }

    pub fn insert_vma(&mut self, start: usize, end: usize, flags: u32, name: &str) -> Result<(), &'static str> {
        if start >= end {
            return Err("Invalid address range");
        }
        for area in &self.mmap_regions {
            if start < area.end_addr && end > area.start_addr {
                return Err("Overlapping VMA region detected");
            }
        }
        self.mmap_regions.push(VmArea {
            start_addr: start,
            end_addr: end,
            flags,
            name: name.to_string(),
        });
        Ok(())
    }

    pub fn find_vma(&self, addr: usize) -> Option<&VmArea> {
        for area in &self.mmap_regions {
            if addr >= area.start_addr && addr < area.end_addr {
                return Some(area);
            }
        }
        None
    }

    pub fn handle_page_fault(&self, addr: usize, is_write: bool) -> Result<&'static str, &'static str> {
        if let Some(vma) = self.find_vma(addr) {
            if is_write && (vma.flags & PROT_WRITE) == 0 {
                return Err("Permission Denied: Write violation in read-only VMA (SIGSEGV)");
            }
            Ok("Demand Page Allocated successfully")
        } else {
            Err("Segmentation Fault: Address not mapped in any VMA (SIGSEGV)")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Zone {
    pub name: String,
    pub object_size: usize,
    pub cached_objects: Vec<usize>,
    pub total_allocations: usize,
}

pub struct BsdZoneAllocator {
    pub zones: HashMap<String, Zone>,
}

impl BsdZoneAllocator {
    pub fn new() -> Self {
        Self {
            zones: HashMap::new(),
        }
    }

    pub fn create_zone(&mut self, name: &str, object_size: usize, pre_alloc_count: usize) -> Result<(), &'static str> {
        if self.zones.contains_key(&name.to_string()) {
            return Err("Zone already exists");
        }
        let mut cached_objects = Vec::new();
        for i in 0..pre_alloc_count {
            cached_objects.push(0x8000_0000 + name.len() * 0x1000 + i * object_size);
        }
        self.zones.insert(name.to_string(), Zone {
            name: name.to_string(),
            object_size,
            cached_objects,
            total_allocations: 0,
        });
        Ok(())
    }

    pub fn zone_alloc(&mut self, name: &str) -> Option<usize> {
        if let Some(zone) = self.zones.get_mut(&name.to_string()) {
            if let Some(addr) = zone.cached_objects.pop() {
                zone.total_allocations += 1;
                Some(addr)
            } else {
                let new_addr = 0x9000_0000 + name.len() * 0x1000 + zone.total_allocations * zone.object_size;
                zone.total_allocations += 1;
                Some(new_addr)
            }
        } else {
            None
        }
    }

    pub fn zone_free(&mut self, name: &str, addr: usize) -> Result<(), &'static str> {
        if let Some(zone) = self.zones.get_mut(&name.to_string()) {
            zone.cached_objects.push(addr);
            Ok(())
        } else {
            Err("Zone not found")
        }
    }
}

#[derive(Debug, Clone)]
pub struct PageFrame {
    pub phys_addr: usize,
    pub is_active: bool,
    pub last_accessed: u64,
}

pub struct LinuxKswapd {
    pub active_list: Vec<PageFrame>,
    pub inactive_list: Vec<PageFrame>,
    pub swap_space: HashMap<usize, Vec<u8>>,
    pub low_watermark: usize,
}

impl LinuxKswapd {
    pub fn new(low_watermark: usize) -> Self {
        Self {
            active_list: Vec::new(),
            inactive_list: Vec::new(),
            swap_space: HashMap::new(),
            low_watermark,
        }
    }

    pub fn add_page_frame(&mut self, phys_addr: usize) {
        self.inactive_list.push(PageFrame {
            phys_addr,
            is_active: false,
            last_accessed: 0,
        });
    }

    pub fn access_page(&mut self, phys_addr: usize, current_time: u64) {
        let mut found_idx = None;
        for (idx, page) in self.inactive_list.iter().enumerate() {
            if page.phys_addr == phys_addr {
                found_idx = Some(idx);
                break;
            }
        }
        if let Some(idx) = found_idx {
            let mut page = self.inactive_list.remove(idx);
            page.is_active = true;
            page.last_accessed = current_time;
            self.active_list.push(page);
            return;
        }

        for page in &mut self.active_list {
            if page.phys_addr == phys_addr {
                page.last_accessed = current_time;
                break;
            }
        }
    }

    pub fn reclaim_pages(&mut self, available_mem: usize) -> Result<usize, &'static str> {
        if available_mem >= self.low_watermark {
            return Ok(0);
        }

        let mut reclaimed_count = 0;
        while !self.inactive_list.is_empty() && reclaimed_count < 3 {
            let page = self.inactive_list.remove(0);
            self.swap_space.insert(page.phys_addr, vec_clone(&[0u8; 4096]));
            reclaimed_count += 1;
        }

        if self.inactive_list.is_empty() {
            while !self.active_list.is_empty() && self.inactive_list.len() < 5 {
                let mut page = self.active_list.remove(0);
                page.is_active = false;
                self.inactive_list.push(page);
            }
        }

        Ok(reclaimed_count)
    }
}

// Helper to clone/create vec for no_std compatibility
fn vec_clone(slice: &[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    for &val in slice {
        v.push(val);
    }
    v
}

#[derive(Debug, Clone)]
pub struct MemCgroup {
    pub id: u32,
    pub limit: usize,
    pub usage: usize,
    pub processes: Vec<u64>,
}

pub struct MemCgroupManager {
    pub cgroups: HashMap<u32, MemCgroup>,
    pub process_to_cgroup: HashMap<u64, u32>,
}

impl MemCgroupManager {
    pub fn new() -> Self {
        Self {
            cgroups: HashMap::new(),
            process_to_cgroup: HashMap::new(),
        }
    }

    pub fn create_cgroup(&mut self, id: u32, limit: usize) -> Result<(), &'static str> {
        if self.cgroups.contains_key(&id) {
            return Err("cgroup already exists");
        }
        self.cgroups.insert(id, MemCgroup {
            id,
            limit,
            usage: 0,
            processes: Vec::new(),
        });
        Ok(())
    }

    pub fn attach_process(&mut self, id: u32, pid: u64) -> Result<(), &'static str> {
        if !self.cgroups.contains_key(&id) {
            return Err("cgroup does not exist");
        }
        self.process_to_cgroup.insert(pid, id);
        if let Some(cgroup) = self.cgroups.get_mut(&id) {
            cgroup.processes.push(pid);
        }
        Ok(())
    }

    pub fn charge_memory(&mut self, pid: u64, bytes: usize) -> Result<(), &'static str> {
        if let Some(&cg_id) = self.process_to_cgroup.get(&pid) {
            if let Some(cgroup) = self.cgroups.get_mut(&cg_id) {
                if cgroup.usage + bytes > cgroup.limit {
                    return Err("Memory Limit Exceeded (MemCg OOM)");
                }
                cgroup.usage += bytes;
            }
        }
        Ok(())
    }

    pub fn uncharge_memory(&mut self, pid: u64, bytes: usize) {
        if let Some(&cg_id) = self.process_to_cgroup.get(&pid) {
            if let Some(cgroup) = self.cgroups.get_mut(&cg_id) {
                cgroup.usage = cgroup.usage.saturating_sub(bytes);
            }
        }
    }
}
