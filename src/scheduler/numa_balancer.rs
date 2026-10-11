// SigmaOS Cross-NUMA Task Load Balancer & Migration Engine
// Balances run-queue loads across multi-socket NUMA nodes while penalizing remote memory overhead.

#![allow(dead_code)]
extern crate alloc;

use alloc::vec::Vec;

/// Task scheduled on a NUMA node
#[derive(Debug, Clone)]
pub struct NumaTask {
    pub pid: u32,
    pub preferred_node: usize,
    pub current_node: usize,
    pub load_weight: u64,
    pub memory_locality_score: u32, // Percentage of pages local to preferred_node (0..100)
}

/// Statistics for a NUMA domain
#[derive(Debug, Clone, Default)]
pub struct NumaNodeStats {
    pub node_id: usize,
    pub cpu_count: usize,
    pub total_load: u64,
    pub tasks_count: usize,
}

/// Cross-NUMA Load Balancer
#[derive(Debug, Clone)]
pub struct NumaLoadBalancer {
    pub nodes: Vec<NumaNodeStats>,
    pub imbalance_threshold_percent: u32,
    pub migration_cost_penalty: u64,
}

impl NumaLoadBalancer {
    pub fn new(node_count: usize, cpus_per_node: usize) -> Self {
        let mut nodes = Vec::new();
        for id in 0..node_count {
            nodes.push(NumaNodeStats {
                node_id: id,
                cpu_count: cpus_per_node,
                total_load: 0,
                tasks_count: 0,
            });
        }

        Self {
            nodes,
            imbalance_threshold_percent: 25, // 25% imbalance triggers migration
            migration_cost_penalty: 100,
        }
    }

    /// Update load stats across nodes given a task list
    pub fn update_loads(&mut self, tasks: &[NumaTask]) {
        for n in &mut self.nodes {
            n.total_load = 0;
            n.tasks_count = 0;
        }

        for t in tasks {
            if t.current_node < self.nodes.len() {
                self.nodes[t.current_node].total_load += t.load_weight;
                self.nodes[t.current_node].tasks_count += 1;
            }
        }
    }

    /// Identify candidate task to migrate from the most loaded node to the least loaded node
    pub fn balance(&mut self, tasks: &mut [NumaTask]) -> Option<(u32, usize, usize)> {
        self.update_loads(tasks);

        if self.nodes.len() < 2 {
            return None;
        }

        let mut max_node = 0;
        let mut min_node = 0;
        let mut max_load = 0u64;
        let mut min_load = u64::MAX;

        for (idx, n) in self.nodes.iter().enumerate() {
            if n.total_load > max_load {
                max_load = n.total_load;
                max_node = idx;
            }
            if n.total_load < min_load {
                min_load = n.total_load;
                min_node = idx;
            }
        }

        if max_load == 0 || max_node == min_node {
            return None;
        }

        let load_diff = max_load.saturating_sub(min_load);
        let imbalance_pct = (load_diff * 100) / max_load;

        if imbalance_pct < (self.imbalance_threshold_percent as u64) {
            return None; // Balanced within threshold
        }

        // Find a task on max_node with low memory locality or preferred_node == min_node
        let mut chosen_task_idx: Option<usize> = None;
        for (idx, t) in tasks.iter().enumerate() {
            if t.current_node == max_node {
                if t.preferred_node == min_node || t.memory_locality_score < 50 {
                    chosen_task_idx = Some(idx);
                    break;
                }
            }
        }

        // If none found with low locality, pick any task on max_node
        if chosen_task_idx.is_none() {
            chosen_task_idx = tasks.iter().position(|t| t.current_node == max_node);
        }

        if let Some(idx) = chosen_task_idx {
            let pid = tasks[idx].pid;
            tasks[idx].current_node = min_node;
            Some((pid, max_node, min_node))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numa_load_balancing() {
        let mut balancer = NumaLoadBalancer::new(2, 4);
        let mut tasks = [
            NumaTask {
                pid: 1,
                preferred_node: 0,
                current_node: 0,
                load_weight: 1000,
                memory_locality_score: 90,
            },
            NumaTask {
                pid: 2,
                preferred_node: 1, // preferred node 1, but ran on node 0
                current_node: 0,
                load_weight: 1000,
                memory_locality_score: 20,
            },
        ];

        let migration = balancer.balance(&mut tasks);
        assert_eq!(migration, Some((2, 0, 1))); // Task 2 migrated to node 1!
        assert_eq!(tasks[1].current_node, 1);
    }
}
