// Processor Assignment, CPU Affinity & NUMA Topology Manager for SigmaOS
// Inspired by Linux sched_setaffinity(2), FreeBSD cpuset(2) / cpuset_setaffinity, and Windows NUMA node affinity.

use std::collections::HashMap;

/// 64-bit CPU Core Affinity Bitmask (supports up to 64 logical processors per mask)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuAffinityMask {
    pub mask_bits: u64,
}

impl CpuAffinityMask {
    pub fn all_cores() -> Self {
        Self { mask_bits: !0 }
    }

    pub fn single_core(core_id: usize) -> Self {
        let mut mask = Self { mask_bits: 0 };
        mask.set_core(core_id);
        mask
    }

    pub fn from_cores(core_ids: &[usize]) -> Self {
        let mut mask = Self { mask_bits: 0 };
        for &c in core_ids {
            mask.set_core(c);
        }
        mask
    }

    pub fn set_core(&mut self, core_id: usize) {
        if core_id < 64 {
            self.mask_bits |= 1u64 << core_id;
        }
    }

    pub fn clear_core(&mut self, core_id: usize) {
        if core_id < 64 {
            self.mask_bits &= !(1u64 << core_id);
        }
    }

    pub fn is_core_set(&self, core_id: usize) -> bool {
        if core_id < 64 {
            (self.mask_bits & (1u64 << core_id)) != 0
        } else {
            false
        }
    }

    pub fn count_assigned_cores(&self) -> u32 {
        self.mask_bits.count_ones()
    }
}

/// NUMA Domain Topology Record
#[derive(Debug, Clone)]
pub struct NumaDomainTopology {
    pub node_id: usize,
    pub cpu_cores: Vec<usize>,
    pub local_memory_mb: usize,
    pub interconnect_latency_distance: HashMap<usize, u32>, // Target Node ID -> Latency penalty
}

impl NumaDomainTopology {
    pub fn new(node_id: usize, cpu_cores: &[usize], local_memory_mb: usize) -> Self {
        let mut latency = HashMap::new();
        latency.insert(node_id, 10); // Local access distance baseline = 10
        Self {
            node_id,
            cpu_cores: cpu_cores.to_vec(),
            local_memory_mb,
            interconnect_latency_distance: latency,
        }
    }
}

/// Process CPU Assigner & Scheduler Affinity Manager
pub struct ProcessCpuAssigner {
    pub process_affinity: HashMap<usize, CpuAffinityMask>,
    pub thread_pinned_core: HashMap<usize, usize>,
    pub numa_nodes: HashMap<usize, NumaDomainTopology>,
    pub system_core_count: usize,
}

impl ProcessCpuAssigner {
    pub fn new(system_core_count: usize) -> Self {
        let mut assigner = Self {
            process_affinity: HashMap::new(),
            thread_pinned_core: HashMap::new(),
            numa_nodes: HashMap::new(),
            system_core_count,
        };

        // Default NUMA Node 0 setup
        let default_cores: Vec<usize> = (0..system_core_count.min(64)).collect();
        assigner.numa_nodes.insert(0, NumaDomainTopology::new(0, &default_cores, 32768));
        assigner
    }

    /// Set process CPU affinity mask (Linux sched_setaffinity & FreeBSD cpuset_setaffinity)
    pub fn assign_process_affinity(&mut self, pid: usize, mask: CpuAffinityMask) -> Result<(), &'static str> {
        if mask.mask_bits == 0 {
            return Err("Invalid affinity mask: Must assign at least 1 CPU core");
        }
        self.process_affinity.insert(pid, mask);
        Ok(())
    }

    /// Query process CPU affinity mask (Linux sched_getaffinity & FreeBSD cpuset_getaffinity)
    pub fn get_process_affinity(&self, pid: usize) -> CpuAffinityMask {
        self.process_affinity.get(&pid).copied().unwrap_or_else(CpuAffinityMask::all_cores)
    }

    /// Pin thread to a specific CPU core (Hard Core Pinning)
    pub fn pin_thread_to_core(&mut self, tid: usize, core_id: usize) -> Result<(), &'static str> {
        if core_id >= self.system_core_count {
            return Err("Core ID exceeds total system core count");
        }
        self.thread_pinned_core.insert(tid, core_id);
        Ok(())
    }

    /// Migrate process to preferred NUMA node
    pub fn migrate_process_numa_node(&mut self, pid: usize, target_node_id: usize) -> Result<(), &'static str> {
        let node = self.numa_nodes.get(&target_node_id).ok_or("NUMA node not found")?;
        let mask = CpuAffinityMask::from_cores(&node.cpu_cores);
        self.assign_process_affinity(pid, mask)?;
        Ok(())
    }

    /// Select optimal CPU core for task dispatch based on affinity mask and load
    pub fn select_optimal_core_for_task(&self, pid: usize, core_loads: &[usize]) -> usize {
        let mask = self.get_process_affinity(pid);
        let mut min_load = usize::MAX;
        let mut best_core = 0;

        for core_id in 0..self.system_core_count {
            if mask.is_core_set(core_id) {
                let load = core_loads.get(core_id).copied().unwrap_or(0);
                if load < min_load {
                    min_load = load;
                    best_core = core_id;
                }
            }
        }

        best_core
    }
}

impl Default for ProcessCpuAssigner {
    fn default() -> Self {
        Self::new(16) // Default 16-core system
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_affinity_mask_operations() {
        let mut mask = CpuAffinityMask::from_cores(&[0, 2, 4]);
        assert_eq!(mask.count_assigned_cores(), 3);
        assert!(mask.is_core_set(0));
        assert!(!mask.is_core_set(1));
        assert!(mask.is_core_set(2));

        mask.set_core(1);
        assert!(mask.is_core_set(1));
        assert_eq!(mask.count_assigned_cores(), 4);

        mask.clear_core(0);
        assert!(!mask.is_core_set(0));
    }

    #[test]
    fn test_process_cpu_assigner_and_numa_migration() {
        let mut assigner = ProcessCpuAssigner::new(16);

        // Assign PID 100 to cores 0, 1
        let mask = CpuAffinityMask::from_cores(&[0, 1]);
        assigner.assign_process_affinity(100, mask).unwrap();

        let queried_mask = assigner.get_process_affinity(100);
        assert_eq!(queried_mask, mask);

        // Select optimal core based on load [Core 0: 80% load, Core 1: 20% load]
        let loads = vec![80, 20, 5, 0];
        let optimal = assigner.select_optimal_core_for_task(100, &loads);
        assert_eq!(optimal, 1); // Selects Core 1 (lowest load among assigned cores 0 and 1)

        // Hard thread pinning
        assert!(assigner.pin_thread_to_core(1001, 3).is_ok());
        assert_eq!(assigner.thread_pinned_core.get(&1001), Some(&3));
    }
}
