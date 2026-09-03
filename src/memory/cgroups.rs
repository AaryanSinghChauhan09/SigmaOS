use alloc::vec;
extern crate alloc;
// SigmaOS Linux-inspired Memory Control Groups (memcg) and OOM Killer Subsystem

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OomPolicy {
    KillHeuristicProcess, // Kill process with largest memory footprint
    KillYoungest,         // Kill process with highest PID
    PanicSystem,          // Trigger system panic
}

#[derive(Debug, Clone)]
pub struct MemCgroup {
    pub id: usize,
    pub name: String,
    pub usage: usize,
    pub limit: usize,
    pub oom_control_enabled: bool,
    pub parent_id: Option<usize>,
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
        // Create root cgroup (ID 0)
        manager.create_cgroup("/", None, usize::MAX);
        manager
    }

    pub fn create_cgroup(&mut self, name: &str, parent_id: Option<usize>, limit: usize) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let cgroup = MemCgroup {
            id,
            name: name.to_string(),
            usage: 0,
            limit,
            oom_control_enabled: true,
            parent_id,
        };

        self.groups.insert(id, cgroup);
        id
    }

    /// Try to charge memory to a cgroup and its parents.
    /// If charging would exceed any limit, returns Err(cgroup_id_that_failed).
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
                    return Err(id); // Limit exceeded at this level
                }
                cgroup.usage += bytes;
                charged_ids.push(id);
                current_id = cgroup.parent_id;
            } else {
                break;
            }
        }
        Ok(())
    }

    /// Uncharge memory from a cgroup and its parents.
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

        // Create container cgroup with limit 1MB
        let container_cg = manager.create_cgroup("/docker/container-1", Some(0), 1024 * 1024);

        // Charge 400KB - should succeed
        assert!(manager.charge_memory(container_cg, 400 * 1024).is_ok());
        assert_eq!(manager.groups.get(&container_cg).unwrap().usage, 400 * 1024);
        assert_eq!(manager.groups.get(&0).unwrap().usage, 400 * 1024); // Root also charged

        // Charge 700KB - exceeds 1MB limit! Should fail and roll back
        assert_eq!(
            manager.charge_memory(container_cg, 700 * 1024),
            Err(container_cg)
        );
        assert_eq!(manager.groups.get(&container_cg).unwrap().usage, 400 * 1024);
        assert_eq!(manager.groups.get(&0).unwrap().usage, 400 * 1024); // Root rolled back

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
        assert_eq!(manager.groups.get(&container_cg).unwrap().usage, 150 * 1024);
        // Memory uncharged from 400KB to 150KB
    }

    #[test]
    fn test_oom_policy_selection() {
        let mut manager = MemCgroupManager::new();
        let container_cg = manager.create_cgroup("/docker/container-2", Some(0), 1024 * 1024);

        let mut processes = vec![
            (101, 100 * 1024, true),
            (205, 50 * 1024, true), // Highest PID (youngest)
        ];

        let killed = manager
            .trigger_oom_killer_with_policy(container_cg, &mut processes, OomPolicy::KillYoungest)
            .unwrap();
        assert_eq!(killed, 205);
        assert_eq!(processes[1].2, false);
    }
}
