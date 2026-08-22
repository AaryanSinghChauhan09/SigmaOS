// Parallel Build Graph Optimizer for SigmaOS
// Location: tools/build/parallel.rs

#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;

pub struct BuildTaskNode {
    pub id: u64,
    pub name: String,
    pub dependencies: Vec<u64>,
    pub completed: bool,
}

pub struct ParallelBuildOptimizer {
    pub max_threads: usize,
    pub tasks: BTreeMap<u64, BuildTaskNode>,
}

impl ParallelBuildOptimizer {
    pub fn new(max_threads: usize) -> Self {
        ParallelBuildOptimizer {
            max_threads,
            tasks: BTreeMap::new(),
        }
    }

    pub fn add_task(&mut self, id: u64, name: &str, dependencies: &[u64]) {
        self.tasks.insert(
            id,
            BuildTaskNode {
                id,
                name: String::from(name),
                dependencies: dependencies.to_vec(),
                completed: false,
            },
        );
    }

    pub fn get_ready_tasks(&self) -> Vec<u64> {
        let mut ready = Vec::new();
        for (id, node) in &self.tasks {
            if !node.completed {
                // Check if all dependencies are completed
                let deps_satisfied = node.dependencies.iter().all(|dep_id| {
                    self.tasks.get(dep_id).map(|d| d.completed).unwrap_or(false)
                });
                if deps_satisfied {
                    ready.push(*id);
                }
            }
        }
        ready.truncate(self.max_threads);
        ready
    }

    pub fn mark_completed(&mut self, id: u64) {
        if let Some(node) = self.tasks.get_mut(&id) {
            node.completed = true;
        }
    }

    pub fn is_build_finished(&self) -> bool {
        self.tasks.values().all(|n| n.completed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_build_dag_execution() {
        let mut optimizer = ParallelBuildOptimizer::new(4);
        optimizer.add_task(1, "libcore", &[]);
        optimizer.add_task(2, "liballoc", &[1]);
        optimizer.add_task(3, "sigos_kernel", &[1, 2]);

        let ready1 = optimizer.get_ready_tasks();
        assert_eq!(ready1, alloc::vec![1]); // Only task 1 has 0 dependencies

        optimizer.mark_completed(1);
        let ready2 = optimizer.get_ready_tasks();
        assert_eq!(ready2, alloc::vec![2]); // Task 2 dependencies met

        optimizer.mark_completed(2);
        let ready3 = optimizer.get_ready_tasks();
        assert_eq!(ready3, alloc::vec![3]); // Task 3 dependencies met

        optimizer.mark_completed(3);
        assert!(optimizer.is_build_finished());
    }
}
