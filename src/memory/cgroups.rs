// SigmaOS Linux-inspired Memory Control Groups (memcg) and OOM Killer Subsystem
// cgroups v2 memory controller with hierarchical accounting and pressure notifications

use std::collections::BTreeMap;
use std::string::String;
use std::string::ToString;
use std::vec::Vec;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OomPolicy {
    KillHeuristicProcess, // Kill process with largest memory footprint
    KillYoungest,         // Kill process with highest PID
    PanicSystem,          // Trigger system panic
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPressureLevel {
    Low = 0,
    Medium = 1,
    Critical = 2,
}

#[derive(Debug)]
pub struct MemCgroup {
    pub id: usize,
    pub name: String,
    pub usage: AtomicUsize,
    pub limit: usize,
    pub swap_usage: AtomicUsize,
    pub swap_limit: usize,
    pub oom_control_enabled: bool,
    pub parent_id: Option<usize>,
    pub pressure_level: AtomicUsize,
    pub failcnt: AtomicU64,
}

pub struct MemCgroupManager {
    pub groups: BTreeMap<usize, MemCgroup>,
    next_id: usize,
}

impl MemCgroupManager {
    pub fn new() -> Self {
        let mut manager = MemCgroupManager {
            groups: BTreeMap::new(),
            next_id: 0,
        };
        // Create root cgroup (ID 0) with unlimited limits
        manager.create_cgroup("/", None, usize::MAX, usize::MAX);
        manager
    }

    pub fn create_cgroup(&mut self, name: &str, parent_id: Option<usize>, limit: usize, swap_limit: usize) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let cgroup = MemCgroup {
            id,
            name: name.to_string(),
            usage: AtomicUsize::new(0),
            limit,
            swap_usage: AtomicUsize::new(0),
            swap_limit,
            oom_control_enabled: true,
            parent_id,
            pressure_level: AtomicUsize::new(MemoryPressureLevel::Low as usize),
            failcnt: AtomicU64::new(0),
        };

        self.groups.insert(id, cgroup);
        id
    }

    /// Try to charge memory to a cgroup and its parents.
    /// If charging would exceed any limit, returns Err(cgroup_id_that_failed).
    pub fn charge_memory(&mut self, cgroup_id: usize, bytes: usize) -> Result<(), usize> {
        let mut current_id = Some(cgroup_id);
        let mut charged_ids = Vec::new();

        // First pass: check limits without modifying
        let mut check_id = Some(cgroup_id);
        while let Some(id) = check_id {
            if let Some(cgroup) = self.groups.get(&id) {
                let current_usage = cgroup.usage.load(Ordering::SeqCst);
                if current_usage + bytes > cgroup.limit {
                    // Increment failcnt for the cgroup that failed
                    if let Some(fail_cgroup) = self.groups.get_mut(&id) {
                        fail_cgroup.failcnt.fetch_add(1, Ordering::SeqCst);
                    }
                    return Err(id); // Limit exceeded at this level
                }
                charged_ids.push(id);
                check_id = cgroup.parent_id;
            } else {
                break;
            }
        }

        // Second pass: actually charge the memory
        for id in &charged_ids {
            if let Some(cgroup) = self.groups.get_mut(id) {
                cgroup.usage.fetch_add(bytes, Ordering::SeqCst);
            }
        }
        
        // Update pressure level
        self.update_pressure_level(cgroup_id);
        Ok(())
    }

    /// Try to charge swap to a cgroup and its parents.
    /// If charging would exceed any swap limit, returns Err(cgroup_id_that_failed).
    pub fn charge_swap(&mut self, cgroup_id: usize, bytes: usize) -> Result<(), usize> {
        let mut current_id = Some(cgroup_id);
        let mut charged_ids = Vec::new();

        // First pass: check limits without modifying
        let mut check_id = Some(cgroup_id);
        while let Some(id) = check_id {
            if let Some(cgroup) = self.groups.get(&id) {
                let current_swap = cgroup.swap_usage.load(Ordering::SeqCst);
                if current_swap + bytes > cgroup.swap_limit {
                    // Increment failcnt for the cgroup that failed
                    if let Some(fail_cgroup) = self.groups.get_mut(&id) {
                        fail_cgroup.failcnt.fetch_add(1, Ordering::SeqCst);
                    }
                    return Err(id); // Limit exceeded at this level
                }
                charged_ids.push(id);
                check_id = cgroup.parent_id;
            } else {
                break;
            }
        }

        // Second pass: actually charge the swap
        for id in &charged_ids {
            if let Some(cgroup) = self.groups.get_mut(id) {
                cgroup.swap_usage.fetch_add(bytes, Ordering::SeqCst);
            }
        }
        Ok(())
    }

    /// Uncharge memory from a cgroup and its parents.
    pub fn uncharge_memory(&mut self, cgroup_id: usize, bytes: usize) {
        let mut current_id = Some(cgroup_id);
        while let Some(id) = current_id {
            if let Some(cgroup) = self.groups.get_mut(&id) {
                cgroup.usage.fetch_sub(bytes, Ordering::SeqCst);
                current_id = cgroup.parent_id;
            } else {
                break;
            }
        }
        
        // Update pressure level
        self.update_pressure_level(cgroup_id);
    }

    /// Uncharge swap from a cgroup and its parents.
    pub fn uncharge_swap(&mut self, cgroup_id: usize, bytes: usize) {
        let mut current_id = Some(cgroup_id);
        while let Some(id) = current_id {
            if let Some(cgroup) = self.groups.get_mut(&id) {
                cgroup.swap_usage.fetch_sub(bytes, Ordering::SeqCst);
                current_id = cgroup.parent_id;
            } else {
                break;
            }
        }
    }

    /// Update memory pressure level based on usage
    pub fn update_pressure_level(&mut self, cgroup_id: usize) {
        if let Some(cgroup) = self.groups.get_mut(&cgroup_id) {
            let usage = cgroup.usage.load(Ordering::SeqCst);
            let limit = cgroup.limit;
            
            let level = if usage < limit / 2 {
                MemoryPressureLevel::Low
            } else if usage < limit * 3 / 4 {
                MemoryPressureLevel::Medium
            } else {
                MemoryPressureLevel::Critical
            };
            
            cgroup.pressure_level.store(level as usize, Ordering::SeqCst);
        }
    }

    /// Get current memory pressure level
    pub fn get_pressure_level(&self, cgroup_id: usize) -> Option<MemoryPressureLevel> {
        if let Some(cgroup) = self.groups.get(&cgroup_id) {
            let level = cgroup.pressure_level.load(Ordering::SeqCst);
            match level {
                0 => Some(MemoryPressureLevel::Low),
                1 => Some(MemoryPressureLevel::Medium),
                2 => Some(MemoryPressureLevel::Critical),
                _ => Some(MemoryPressureLevel::Low),
            }
        } else {
            None
        }
    }

    /// Get memory usage and limit
    pub fn get_memory_usage(&self, cgroup_id: usize) -> Option<(usize, usize)> {
        if let Some(cgroup) = self.groups.get(&cgroup_id) {
            Some((cgroup.usage.load(Ordering::SeqCst), cgroup.limit))
        } else {
            None
        }
    }

    /// Get swap usage and limit
    pub fn get_swap_usage(&self, cgroup_id: usize) -> Option<(usize, usize)> {
        if let Some(cgroup) = self.groups.get(&cgroup_id) {
            Some((cgroup.swap_usage.load(Ordering::SeqCst), cgroup.swap_limit))
        } else {
            None
        }
    }

    /// Get failure count
    pub fn get_failcnt(&self, cgroup_id: usize) -> Option<u64> {
        if let Some(cgroup) = self.groups.get(&cgroup_id) {
            Some(cgroup.failcnt.load(Ordering::SeqCst))
        } else {
            None
        }
    }

    /// OOM (Out Of Memory) Killer simulation.
    /// Selects the process with the largest memory consumption in the cgroup and terminates it.
    /// Returns the process ID of the killed process.
    pub fn trigger_oom_killer(
        &mut self,
        cgroup_id: usize,
        processes: &mut Vec<(usize, usize, bool)>, // (pid, memory_usage, alive)
    ) -> Option<usize> {
        // Find alive process with largest memory usage
        let mut target_pid = None;
        let mut max_usage = 0;

        for (pid, usage, alive) in processes.iter() {
            if *alive && *usage > max_usage {
                max_usage = *usage;
                target_pid = Some(*pid);
            }
        }

        if let Some(pid) = target_pid {
            // Kill the process (mark alive = false)
            for (p, usage, alive) in processes.iter_mut() {
                if *p == pid {
                    *alive = false;
                    // Uncharge memory of terminated process
                    self.uncharge_memory(cgroup_id, *usage);
                    break;
                }
            }
        }

        target_pid
    }

    /// OOM Killer execution with policy selection
    pub fn trigger_oom_killer_with_policy(
        &mut self,
        cgroup_id: usize,
        processes: &mut Vec<(usize, usize, bool)>, // (pid, memory_usage, alive)
        policy: OomPolicy,
    ) -> Option<usize> {
        match policy {
            OomPolicy::PanicSystem => None,
            OomPolicy::KillYoungest => {
                let mut target_pid = None;
                let mut max_pid = 0;
                for (pid, _usage, alive) in processes.iter() {
                    if *alive && *pid > max_pid {
                        max_pid = *pid;
                        target_pid = Some(*pid);
                    }
                }
                if let Some(pid) = target_pid {
                    for (p, usage, alive) in processes.iter_mut() {
                        if *p == pid {
                            *alive = false;
                            self.uncharge_memory(cgroup_id, *usage);
                            break;
                        }
                    }
                }
                target_pid
            }
            OomPolicy::KillHeuristicProcess => self.trigger_oom_killer(cgroup_id, processes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgroups_charging_and_oom() {
        let mut manager = MemCgroupManager::new();

        // Create container cgroup with limit 1MB and swap limit 512MB
        let container_cg = manager.create_cgroup("/docker/container-1", Some(0), 1024 * 1024, 512 * 1024 * 1024);

        // Charge 400KB - should succeed
        assert!(manager.charge_memory(container_cg, 400 * 1024).is_ok());
        assert_eq!(manager.groups.get(&container_cg).unwrap().usage.load(Ordering::SeqCst), 400 * 1024);
        assert_eq!(manager.groups.get(&0).unwrap().usage.load(Ordering::SeqCst), 400 * 1024); // Root also charged

        // Charge 700KB - exceeds 1MB limit! Should fail and roll back
        assert_eq!(
            manager.charge_memory(container_cg, 700 * 1024),
            Err(container_cg)
        );
        assert_eq!(manager.groups.get(&container_cg).unwrap().usage.load(Ordering::SeqCst), 400 * 1024);
        assert_eq!(manager.groups.get(&0).unwrap().usage.load(Ordering::SeqCst), 400 * 1024); // Root rolled back

        // Trigger OOM Killer on simulated process list
        let mut processes = vec![
            (101, 150 * 1024, true), // pid 101, 150KB
            (102, 250 * 1024, true), // pid 102, 250KB (largest)
        ];

        let killed = manager
            .trigger_oom_killer(container_cg, &mut processes)
            .unwrap();
        assert_eq!(killed, 102);
        assert_eq!(processes[1].2, false); // pid 102 is now terminated
        assert_eq!(manager.groups.get(&container_cg).unwrap().usage.load(Ordering::SeqCst), 150 * 1024);
        // Memory uncharged from 400KB to 150KB
    }

    #[test]
    fn test_oom_policy_selection() {
        let mut manager = MemCgroupManager::new();
        let container_cg = manager.create_cgroup("/docker/container-2", Some(0), 1024 * 1024, 512 * 1024 * 1024);

        let mut processes = vec![
            (101, 100 * 1024, true),
            (205, 50 * 1024, true), // Highest PID (youngest)
        ];

        let killed = manager
            .trigger_oom_killer_with_policy(container_cg, &mut processes, OomPolicy::KillYoungest)
            .unwrap();
        assert_eq!(killed, 205);
    }

    #[test]
    fn test_swap_charging() {
        let mut manager = MemCgroupManager::new();
        let container_cg = manager.create_cgroup("/docker/container-3", Some(0), 1024 * 1024, 512 * 1024);

        // Charge 256KB swap - should succeed
        assert!(manager.charge_swap(container_cg, 256 * 1024).is_ok());
        assert_eq!(manager.groups.get(&container_cg).unwrap().swap_usage.load(Ordering::SeqCst), 256 * 1024);

        // Charge 300KB swap - exceeds 512KB limit! Should fail
        assert_eq!(
            manager.charge_swap(container_cg, 300 * 1024),
            Err(container_cg)
        );
        assert_eq!(manager.groups.get(&container_cg).unwrap().swap_usage.load(Ordering::SeqCst), 256 * 1024);

        // Uncharge swap
        manager.uncharge_swap(container_cg, 256 * 1024);
        assert_eq!(manager.groups.get(&container_cg).unwrap().swap_usage.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_pressure_levels() {
        let mut manager = MemCgroupManager::new();
        let container_cg = manager.create_cgroup("/docker/container-4", Some(0), 1024 * 1024, 512 * 1024);

        // Initial pressure should be Low
        assert_eq!(manager.get_pressure_level(container_cg), Some(MemoryPressureLevel::Low));

        // Charge 400KB - still Low (< 50%)
        manager.charge_memory(container_cg, 400 * 1024).unwrap();
        assert_eq!(manager.get_pressure_level(container_cg), Some(MemoryPressureLevel::Low));

        // Charge 100KB - now Medium (50-75%)
        manager.charge_memory(container_cg, 100 * 1024).unwrap();
        assert_eq!(manager.get_pressure_level(container_cg), Some(MemoryPressureLevel::Medium));

        // Charge 300KB - now Critical (> 75%)
        manager.charge_memory(container_cg, 300 * 1024).unwrap();
        assert_eq!(manager.get_pressure_level(container_cg), Some(MemoryPressureLevel::Critical));
    }

    #[test]
    fn test_failcnt() {
        let mut manager = MemCgroupManager::new();
        let container_cg = manager.create_cgroup("/docker/container-5", Some(0), 1024 * 1024, 512 * 1024);

        // Initial failcnt should be 0
        assert_eq!(manager.get_failcnt(container_cg), Some(0));

        // Charge 500KB - should succeed
        assert!(manager.charge_memory(container_cg, 500 * 1024).is_ok());
        assert_eq!(manager.get_failcnt(container_cg), Some(0));

        // Charge 600KB - should fail and increment failcnt
        assert!(manager.charge_memory(container_cg, 600 * 1024).is_err());
        assert_eq!(manager.get_failcnt(container_cg), Some(1));

        // Another failure
        assert!(manager.charge_memory(container_cg, 600 * 1024).is_err());
        assert_eq!(manager.get_failcnt(container_cg), Some(2));
    }

    #[test]
    fn test_memory_and_swap_usage() {
        let mut manager = MemCgroupManager::new();
        let container_cg = manager.create_cgroup("/docker/container-6", Some(0), 1024 * 1024, 512 * 1024);

        // Get initial usage
        let (mem_usage, mem_limit) = manager.get_memory_usage(container_cg).unwrap();
        assert_eq!(mem_usage, 0);
        assert_eq!(mem_limit, 1024 * 1024);

        let (swap_usage, swap_limit) = manager.get_swap_usage(container_cg).unwrap();
        assert_eq!(swap_usage, 0);
        assert_eq!(swap_limit, 512 * 1024);

        // Charge memory
        manager.charge_memory(container_cg, 256 * 1024).unwrap();
        let (mem_usage, _) = manager.get_memory_usage(container_cg).unwrap();
        assert_eq!(mem_usage, 256 * 1024);

        // Charge swap
        manager.charge_swap(container_cg, 128 * 1024).unwrap();
        let (swap_usage, _) = manager.get_swap_usage(container_cg).unwrap();
        assert_eq!(swap_usage, 128 * 1024);
    }
}
