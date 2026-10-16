// SPDX-License-Identifier: MIT
// SigmaOS - Memory Control Groups (memcg) with Charge Accounting
// Inspired by Linux memory cgroups (mm/memcontrol.c) providing per-cgroup
// memory limits, accounting, and OOM killer integration

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// OOM Killer Policy matching Linux oom_score_adj behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OomPolicy {
    KillHeuristicProcess, // Kill process with largest memory footprint
    KillYoungest,         // Kill process with highest PID
    KillHighestOomScore,  // Kill process with highest oom_score_adj
    PanicSystem,          // Trigger system panic
}

/// Memory event types for cgroup notifications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemEvent {
    Low,     // Memory usage approaching limit
    High,    // Memory usage exceeded limit
    Oom,     // Out of memory condition
    OomKill, // Process killed due to OOM
    Max,     // Memory usage hit max limit
}

/// Memory statistics for a cgroup
#[derive(Debug, Clone)]
pub struct MemStats {
    pub cache: usize,
    pub rss: usize,
    pub file: usize,
    pub anon: usize,
    pub swap: usize,
    pub dirty: usize,
    pub writeback: usize,
    pub shmem: usize,
    pub slab: usize,
    pub pagetables: usize,
}

impl Default for MemStats {
    fn default() -> Self {
        Self {
            cache: 0,
            rss: 0,
            file: 0,
            anon: 0,
            swap: 0,
            dirty: 0,
            writeback: 0,
            shmem: 0,
            slab: 0,
            pagetables: 0,
        }
    }
}

/// Memory control group
#[derive(Debug, Clone)]
pub struct MemCgroup {
    pub id: usize,
    pub name: String,
    pub usage: usize,
    pub limit: usize,
    pub max_usage: usize,
    pub swap_usage: usize,
    pub swap_limit: usize,
    pub oom_control_enabled: bool,
    pub oom_score_adj: i32,
    pub parent_id: Option<usize>,
    pub stats: MemStats,
    pub num_procs: usize,
    pub num_oom_kills: usize,
}

/// Process entry for cgroup accounting
#[derive(Debug, Clone)]
pub struct CgroupProcess {
    pub pid: u32,
    pub memory_usage: usize,
    pub swap_usage: usize,
    pub oom_score_adj: i32,
    pub alive: bool,
}

/// Memory cgroup manager with charge accounting
pub struct MemCgroupManager {
    pub groups: BTreeMap<usize, MemCgroup>,
    pub processes: BTreeMap<u32, CgroupProcess>,
    pub pid_to_cgroup: BTreeMap<u32, usize>,
    pub next_id: usize,
    pub oom_events: Vec<(usize, MemEvent, u32)>, // (cgroup_id, event, pid)
}

impl MemCgroupManager {
    pub fn new() -> Self {
        let mut manager = Self {
            groups: BTreeMap::new(),
            processes: BTreeMap::new(),
            pid_to_cgroup: BTreeMap::new(),
            next_id: 0,
            oom_events: Vec::new(),
        };
        // Create root cgroup (ID 0) with unlimited limit
        manager.create_cgroup("/", None, usize::MAX);
        manager
    }

    /// Create a new memory cgroup
    pub fn create_cgroup(&mut self, name: &str, parent_id: Option<usize>, limit: usize) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let cgroup = MemCgroup {
            id,
            name: name.to_string(),
            usage: 0,
            limit,
            max_usage: 0,
            swap_usage: 0,
            swap_limit: usize::MAX,
            oom_control_enabled: true,
            oom_score_adj: 0,
            parent_id,
            stats: MemStats::default(),
            num_procs: 0,
            num_oom_kills: 0,
        };

        self.groups.insert(id, cgroup);
        id
    }

    /// Add a process to a cgroup
    pub fn add_process(
        &mut self,
        pid: u32,
        cgroup_id: usize,
        oom_score_adj: i32,
    ) -> Result<(), String> {
        if !self.groups.contains_key(&cgroup_id) {
            return Err(format!("Cgroup {} does not exist", cgroup_id));
        }

        let process = CgroupProcess {
            pid,
            memory_usage: 0,
            swap_usage: 0,
            oom_score_adj,
            alive: true,
        };

        self.processes.insert(pid, process);
        self.pid_to_cgroup.insert(pid, cgroup_id);

        if let Some(cgroup) = self.groups.get_mut(&cgroup_id) {
            cgroup.num_procs += 1;
        }

        Ok(())
    }

    /// Remove a process from its cgroup
    pub fn remove_process(&mut self, pid: u32) {
        if let Some(cgroup_id) = self.pid_to_cgroup.remove(&pid) {
            if let Some(process) = self.processes.remove(&pid) {
                // Extract values before uncharging to avoid borrow conflicts
                let memory_usage = process.memory_usage;
                let swap_usage = process.swap_usage;

                // Uncharge process memory
                self.uncharge_memory(cgroup_id, memory_usage);
                self.uncharge_swap(cgroup_id, swap_usage);

                if let Some(cgroup) = self.groups.get_mut(&cgroup_id) {
                    cgroup.num_procs = cgroup.num_procs.saturating_sub(1);
                }
            }
        }
    }

    /// Try to charge memory to a cgroup and its parents with accounting
    /// If charging would exceed any limit, returns Err(cgroup_id_that_failed)
    pub fn charge_memory(&mut self, cgroup_id: usize, bytes: usize) -> Result<(), usize> {
        let mut current_id = Some(cgroup_id);
        let mut charged_ids = Vec::new();

        while let Some(id) = current_id {
            if let Some(cgroup) = self.groups.get_mut(&id) {
                if cgroup.usage + bytes > cgroup.limit {
                    // Back out previous successful charges
                    for back_id in charged_ids {
                        if let Some(back_cgroup) = self.groups.get_mut(&back_id) {
                            back_cgroup.usage = back_cgroup.usage.saturating_sub(bytes);
                        }
                    }
                    self.oom_events.push((id, MemEvent::High, 0));
                    return Err(id); // Limit exceeded at this level
                }
                cgroup.usage += bytes;
                if cgroup.usage > cgroup.max_usage {
                    cgroup.max_usage = cgroup.usage;
                }
                charged_ids.push(id);
                current_id = cgroup.parent_id;
            } else {
                break;
            }
        }
        Ok(())
    }

    /// Charge memory for a specific process
    pub fn charge_process_memory(&mut self, pid: u32, bytes: usize) -> Result<(), String> {
        let cgroup_id = *self
            .pid_to_cgroup
            .get(&pid)
            .ok_or_else(|| format!("Process {} not in any cgroup", pid))?;

        // Charge to cgroup
        self.charge_memory(cgroup_id, bytes)
            .map_err(|id| format!("Memory limit exceeded in cgroup {}", id))?;

        // Update process accounting
        if let Some(process) = self.processes.get_mut(&pid) {
            process.memory_usage += bytes;
        }

        Ok(())
    }

    /// Uncharge memory from a cgroup and its parents
    pub fn uncharge_memory(&mut self, cgroup_id: usize, bytes: usize) {
        let mut current_id = Some(cgroup_id);
        while let Some(id) = current_id {
            if let Some(cgroup) = self.groups.get_mut(&id) {
                cgroup.usage = cgroup.usage.saturating_sub(bytes);
                current_id = cgroup.parent_id;
            } else {
                break;
            }
        }
    }

    /// Charge swap to a cgroup
    pub fn charge_swap(&mut self, cgroup_id: usize, bytes: usize) -> Result<(), usize> {
        let mut current_id = Some(cgroup_id);
        let mut charged_ids = Vec::new();

        while let Some(id) = current_id {
            if let Some(cgroup) = self.groups.get_mut(&id) {
                if cgroup.swap_usage + bytes > cgroup.swap_limit {
                    for back_id in charged_ids {
                        if let Some(back_cgroup) = self.groups.get_mut(&back_id) {
                            back_cgroup.swap_usage = back_cgroup.swap_usage.saturating_sub(bytes);
                        }
                    }
                    return Err(id);
                }
                cgroup.swap_usage += bytes;
                charged_ids.push(id);
                current_id = cgroup.parent_id;
            } else {
                break;
            }
        }
        Ok(())
    }

    /// Uncharge swap from a cgroup
    pub fn uncharge_swap(&mut self, cgroup_id: usize, bytes: usize) {
        let mut current_id = Some(cgroup_id);
        while let Some(id) = current_id {
            if let Some(cgroup) = self.groups.get_mut(&id) {
                cgroup.swap_usage = cgroup.swap_usage.saturating_sub(bytes);
                current_id = cgroup.parent_id;
            } else {
                break;
            }
        }
    }

    /// Update cgroup statistics (simulated)
    pub fn update_stats(&mut self, cgroup_id: usize, stat_type: &str, value: usize) {
        if let Some(cgroup) = self.groups.get_mut(&cgroup_id) {
            match stat_type {
                "cache" => cgroup.stats.cache = value,
                "rss" => cgroup.stats.rss = value,
                "file" => cgroup.stats.file = value,
                "anon" => cgroup.stats.anon = value,
                "swap" => cgroup.stats.swap = value,
                "dirty" => cgroup.stats.dirty = value,
                "writeback" => cgroup.stats.writeback = value,
                "shmem" => cgroup.stats.shmem = value,
                "slab" => cgroup.stats.slab = value,
                "pagetables" => cgroup.stats.pagetables = value,
                _ => {}
            }
        }
    }

    /// Check if a cgroup is under memory pressure
    pub fn is_under_pressure(&self, cgroup_id: usize) -> bool {
        if let Some(cgroup) = self.groups.get(&cgroup_id) {
            let usage_ratio = cgroup.usage as f64 / cgroup.limit as f64;
            usage_ratio > 0.9 // Consider 90% as pressure threshold
        } else {
            false
        }
    }

    /// OOM Killer: Select and kill a process based on policy
    pub fn trigger_oom_killer(&mut self, cgroup_id: usize, policy: OomPolicy) -> Option<u32> {
        if !self.groups.contains_key(&cgroup_id) {
            return None;
        }

        // Collect alive processes in this cgroup
        let mut candidates: Vec<&CgroupProcess> = self
            .processes
            .values()
            .filter(|p| p.alive && self.pid_to_cgroup.get(&p.pid) == Some(&cgroup_id))
            .collect();

        if candidates.is_empty() {
            return None;
        }

        let target_pid = match policy {
            OomPolicy::PanicSystem => {
                self.oom_events.push((cgroup_id, MemEvent::Oom, 0));
                return None;
            }
            OomPolicy::KillHeuristicProcess => {
                // Kill process with largest memory usage
                candidates.sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage));
                candidates.first().map(|p| p.pid)
            }
            OomPolicy::KillYoungest => {
                // Kill process with highest PID (youngest)
                candidates.sort_by(|a, b| b.pid.cmp(&a.pid));
                candidates.first().map(|p| p.pid)
            }
            OomPolicy::KillHighestOomScore => {
                // Kill process with highest oom_score_adj
                candidates.sort_by(|a, b| b.oom_score_adj.cmp(&a.oom_score_adj));
                candidates.first().map(|p| p.pid)
            }
        };

        if let Some(pid) = target_pid {
            self.kill_process(pid);
            self.oom_events.push((cgroup_id, MemEvent::OomKill, pid));

            if let Some(cgroup) = self.groups.get_mut(&cgroup_id) {
                cgroup.num_oom_kills += 1;
            }
        }

        target_pid
    }

    /// Kill a process and free its memory
    fn kill_process(&mut self, pid: u32) {
        if let Some(&cgroup_id) = self.pid_to_cgroup.get(&pid) {
            if let Some(process) = self.processes.get_mut(&pid) {
                process.alive = false;

                // Extract values before uncharging to avoid borrow conflicts
                let memory_usage = process.memory_usage;
                let swap_usage = process.swap_usage;

                self.uncharge_memory(cgroup_id, memory_usage);
                self.uncharge_swap(cgroup_id, swap_usage);
            }
        }
    }

    /// Get cgroup memory usage as percentage
    pub fn get_usage_percent(&self, cgroup_id: usize) -> Option<f64> {
        self.groups.get(&cgroup_id).map(|cgroup| {
            if cgroup.limit == 0 {
                0.0
            } else {
                (cgroup.usage as f64 / cgroup.limit as f64) * 100.0
            }
        })
    }

    /// Get memory statistics for a cgroup in human-readable format
    pub fn get_stats_string(&self, cgroup_id: usize) -> Option<String> {
        self.groups.get(&cgroup_id).map(|cgroup| {
            format!(
                "cache: {} kB\n\
                 rss: {} kB\n\
                 file: {} kB\n\
                 anon: {} kB\n\
                 swap: {} kB\n\
                 dirty: {} kB\n\
                 writeback: {} kB\n\
                 shmem: {} kB\n\
                 slab: {} kB\n\
                 pagetables: {} kB",
                cgroup.stats.cache / 1024,
                cgroup.stats.rss / 1024,
                cgroup.stats.file / 1024,
                cgroup.stats.anon / 1024,
                cgroup.stats.swap / 1024,
                cgroup.stats.dirty / 1024,
                cgroup.stats.writeback / 1024,
                cgroup.stats.shmem / 1024,
                cgroup.stats.slab / 1024,
                cgroup.stats.pagetables / 1024
            )
        })
    }
}

impl Default for MemCgroupManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgroups_charge_accounting() {
        let mut manager = MemCgroupManager::new();

        // Create container cgroup with 1MB limit
        let container_cg = manager.create_cgroup("/docker/container-1", Some(0), 1024 * 1024);

        // Add processes to cgroup
        manager.add_process(101, container_cg, 0).unwrap();
        manager.add_process(102, container_cg, 100).unwrap();

        // Charge memory to processes
        assert!(manager.charge_process_memory(101, 400 * 1024).is_ok());
        assert!(manager.charge_process_memory(102, 300 * 1024).is_ok());

        // Verify cgroup usage
        assert_eq!(manager.groups.get(&container_cg).unwrap().usage, 700 * 1024);
        assert_eq!(manager.groups.get(&0).unwrap().usage, 700 * 1024); // Root also charged

        // Charge exceeding limit should fail
        assert!(manager.charge_process_memory(101, 500 * 1024).is_err());
    }

    #[test]
    fn test_cgroups_swap_accounting() {
        let mut manager = MemCgroupManager::new();
        let container_cg = manager.create_cgroup("/docker/container-2", Some(0), 1024 * 1024);

        manager.add_process(201, container_cg, 0).unwrap();

        // Charge swap
        assert!(manager.charge_swap(container_cg, 512 * 1024).is_ok());
        assert_eq!(
            manager.groups.get(&container_cg).unwrap().swap_usage,
            512 * 1024
        );

        // Uncharge swap
        manager.uncharge_swap(container_cg, 256 * 1024);
        assert_eq!(
            manager.groups.get(&container_cg).unwrap().swap_usage,
            256 * 1024
        );
    }

    #[test]
    fn test_oom_killer_with_policies() {
        let mut manager = MemCgroupManager::new();
        let container_cg = manager.create_cgroup("/docker/container-3", Some(0), 1024 * 1024);

        manager.add_process(301, container_cg, 50).unwrap();
        manager.add_process(302, container_cg, 100).unwrap();
        manager.add_process(303, container_cg, 10).unwrap();

        // Charge memory to trigger OOM
        manager.charge_process_memory(301, 300 * 1024).unwrap();
        manager.charge_process_memory(302, 500 * 1024).unwrap();
        manager.charge_process_memory(303, 200 * 1024).unwrap();

        // Test KillHeuristicProcess (kills largest memory user)
        let killed = manager.trigger_oom_killer(container_cg, OomPolicy::KillHeuristicProcess);
        assert_eq!(killed, Some(302)); // Process 302 had 500KB

        // Test KillYoungest (kills highest PID)
        let killed = manager.trigger_oom_killer(container_cg, OomPolicy::KillYoungest);
        assert_eq!(killed, Some(303)); // Process 303 has highest PID

        // Test KillHighestOomScore (kills highest oom_score_adj)
        let killed = manager.trigger_oom_killer(container_cg, OomPolicy::KillHighestOomScore);
        assert_eq!(killed, Some(301)); // Process 301 has oom_score_adj 50
    }

    #[test]
    fn test_cgroup_statistics() {
        let mut manager = MemCgroupManager::new();
        let container_cg = manager.create_cgroup("/docker/container-4", Some(0), 1024 * 1024);

        manager.update_stats(container_cg, "cache", 1024 * 1024);
        manager.update_stats(container_cg, "rss", 512 * 1024);
        manager.update_stats(container_cg, "anon", 256 * 1024);

        let stats_str = manager.get_stats_string(container_cg).unwrap();
        assert!(stats_str.contains("cache: 1024 kB"));
        assert!(stats_str.contains("rss: 512 kB"));
        assert!(stats_str.contains("anon: 256 kB"));
    }

    #[test]
    fn test_memory_pressure_detection() {
        let mut manager = MemCgroupManager::new();
        let container_cg = manager.create_cgroup("/docker/container-5", Some(0), 1024 * 1024);

        manager.charge_memory(container_cg, 900 * 1024).unwrap();
        assert!(manager.is_under_pressure(container_cg));

        manager.uncharge_memory(container_cg, 400 * 1024);
        assert!(!manager.is_under_pressure(container_cg));
    }

    #[test]
    fn test_process_removal_and_uncharge() {
        let mut manager = MemCgroupManager::new();
        let container_cg = manager.create_cgroup("/docker/container-6", Some(0), 1024 * 1024);

        manager.add_process(401, container_cg, 0).unwrap();
        manager.charge_process_memory(401, 500 * 1024).unwrap();

        assert_eq!(manager.groups.get(&container_cg).unwrap().usage, 500 * 1024);
        assert_eq!(manager.groups.get(&container_cg).unwrap().num_procs, 1);

        manager.remove_process(401);

        assert_eq!(manager.groups.get(&container_cg).unwrap().usage, 0);
        assert_eq!(manager.groups.get(&container_cg).unwrap().num_procs, 0);
    }

    #[test]
    fn test_max_usage_tracking() {
        let mut manager = MemCgroupManager::new();
        let container_cg = manager.create_cgroup("/docker/container-7", Some(0), 1024 * 1024);

        manager.charge_memory(container_cg, 400 * 1024).unwrap();
        assert_eq!(
            manager.groups.get(&container_cg).unwrap().max_usage,
            400 * 1024
        );

        manager.charge_memory(container_cg, 200 * 1024).unwrap();
        assert_eq!(
            manager.groups.get(&container_cg).unwrap().max_usage,
            600 * 1024
        );

        manager.uncharge_memory(container_cg, 300 * 1024);
        assert_eq!(
            manager.groups.get(&container_cg).unwrap().max_usage,
            600 * 1024
        ); // Max unchanged
    }
}
