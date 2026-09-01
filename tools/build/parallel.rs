// SPDX-License-Identifier: GPL-3.0-or-later
// Parallel Build Graph Optimizer for SigmaOS
// Location: tools/build/parallel.rs

#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
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

// =========================================================================
// ARCH LINUX & DISTRO INSPIRED "BUILDBTW" HIGH-PERFORMANCE BUILD ENGINE
// =========================================================================

pub struct BuildBtwArtifactCache {
    pub cache_entries: BTreeMap<String, Vec<u8>>, // Hash -> Compiled Binary Artifact
    pub hits: usize,
    pub misses: usize,
}

impl BuildBtwArtifactCache {
    pub fn new() -> Self {
        Self {
            cache_entries: BTreeMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    pub fn get_or_compile<F>(&mut self, source_hash: &str, compile_fn: F) -> Result<Vec<u8>, &'static str>
    where
        F: FnOnce() -> Result<Vec<u8>, &'static str>,
    {
        if let Some(artifact) = self.cache_entries.get(source_hash) {
            self.hits += 1;
            return Ok(artifact.clone());
        }
        self.misses += 1;
        let compiled = compile_fn()?;
        self.cache_entries.insert(source_hash.to_string(), compiled.clone());
        Ok(compiled)
    }

    pub fn cache_hit_ratio(&self) -> f32 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f32 / total as f32
        }
    }
}

impl Default for BuildBtwArtifactCache {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BuildBtwCompilerFlags {
    pub opt_level: &'static str,
    pub target_cpu: &'static str,
    pub lto_enabled: bool,
    pub extra_flags: Vec<String>,
}

impl BuildBtwCompilerFlags {
    pub fn arch_cachyos_preset() -> Self {
        Self {
            opt_level: "-O3",
            target_cpu: "-march=native",
            lto_enabled: true,
            extra_flags: alloc::vec![
                "-flto=thin".to_string(),
                "-fno-plt".to_string(),
                "-mllvm".to_string(),
                "-polly".to_string(),
            ],
        }
    }

    pub fn format_flag_string(&self) -> String {
        let mut flags = alloc::format!("{} {}", self.opt_level, self.target_cpu);
        if self.lto_enabled {
            flags.push_str(" -C lto=thin");
        }
        for flag in &self.extra_flags {
            flags.push(' ');
            flags.push_str(flag);
        }
        flags
    }
}

pub struct SovereignBuildBtwEngine {
    pub parallel_optimizer: ParallelBuildOptimizer,
    pub artifact_cache: BuildBtwArtifactCache,
    pub flags: BuildBtwCompilerFlags,
}

impl SovereignBuildBtwEngine {
    pub fn new(concurrency: usize) -> Self {
        Self {
            parallel_optimizer: ParallelBuildOptimizer::new(concurrency),
            artifact_cache: BuildBtwArtifactCache::new(),
            flags: BuildBtwCompilerFlags::arch_cachyos_preset(),
        }
    }

    pub fn register_target(&mut self, id: u64, name: &str, deps: &[u64]) {
        self.parallel_optimizer.add_task(id, name, deps);
    }

    pub fn execute_next_batch<F>(&mut self, mut compile_task_fn: F) -> Result<usize, &'static str>
    where
        F: FnMut(u64, &str) -> Result<Vec<u8>, &'static str>,
    {
        let ready_ids = self.parallel_optimizer.get_ready_tasks();
        let count = ready_ids.len();

        for id in ready_ids {
            let task_name = self
                .parallel_optimizer
                .tasks
                .get(&id)
                .map(|t| t.name.clone())
                .unwrap_or_else(|| "unknown".to_string());

            let source_hash = alloc::format!("hash-task-{}-{}", id, task_name);
            let _artifact = self
                .artifact_cache
                .get_or_compile(&source_hash, || compile_task_fn(id, &task_name))?;

            self.parallel_optimizer.mark_completed(id);
        }

        Ok(count)
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

    #[test]
    fn test_buildbtw_engine_cache_and_flags() {
        let mut engine = SovereignBuildBtwEngine::new(4);
        engine.register_target(1, "kernel-base", &[]);
        engine.register_target(2, "kernel-modules", &[1]);

        let flags = engine.flags.format_flag_string();
        assert!(flags.contains("-O3"));
        assert!(flags.contains("-march=native"));

        let executed1 = engine
            .execute_next_batch(|_id, _name| Ok(alloc::vec![0x7F, b'E', b'L', b'F']))
            .unwrap();
        assert_eq!(executed1, 1); // Only target 1 compiled first

        let executed2 = engine
            .execute_next_batch(|_id, _name| Ok(alloc::vec![0x7F, b'E', b'L', b'F']))
            .unwrap();
        assert_eq!(executed2, 1); // Target 2 compiled after target 1

        assert!(engine.parallel_optimizer.is_build_finished());
        assert_eq!(engine.artifact_cache.misses, 2);
    }
}
