#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::new_without_default)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
extern crate alloc;

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[cfg(not(any(feature = "standalone_test", test)))]
use crate::klib::{BTreeMap, HashMap, HashSet};

#[cfg(any(feature = "standalone_test", test))]
use std::collections::{BTreeMap, HashMap, HashSet};

// ============================================================================
// Sovereign Thread Pool Subsystem
// Inspired by Linux Workqueues (system_wq, unbound_wq, highpri_wq),
// FreeBSD taskqueue(9), OpenBSD taskq(9), Apple GCD, Rayon work-stealing, and Tokio.
// ============================================================================

/// Thread Pool Category / Execution Paradigm
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ThreadPoolKind {
    FixedWorkerPool,      // Linux system_wq / fixed worker count
    WorkStealingPool,     // Rayon / Tokio work-stealing deque scheduler
    UnboundDynamicPool,   // Linux unbound_wq / auto-scaling thread pool
    PriorityTaskQueue,    // FreeBSD taskqueue_create_fast / GCD priority queue
    AffinityPinnedPool,   // Per-CPU core pinned workers (NUMA / PREEMPT_RT)
}

/// Task Priority Levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TaskPriority {
    Critical = 0,    // Real-time / Kernel interrupts
    Interactive = 1, // GUI / Audio / User input
    Normal = 2,      // Standard async tasks
    Idle = 3,        // Background garbage collection & maintenance
}

/// Task Execution State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// A Work Unit / Task Specifier
pub struct SovereignTask {
    pub id: u64,
    pub name: String,
    pub priority: TaskPriority,
    pub target_cpu_affinity: Option<usize>,
    pub payload: Box<dyn Fn() -> Result<String, String> + Send + Sync>,
}

impl core::fmt::Debug for SovereignTask {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SovereignTask")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("priority", &self.priority)
            .field("target_cpu_affinity", &self.target_cpu_affinity)
            .finish()
    }
}

/// Worker Thread Representation
#[derive(Debug, Clone)]
pub struct SovereignWorkerThread {
    pub worker_id: usize,
    pub cpu_affinity: Option<usize>,
    pub numa_node: u32,
    pub active_tasks_count: usize,
    pub total_tasks_executed: u64,
    pub is_idle: bool,
}

impl SovereignWorkerThread {
    pub fn new(worker_id: usize, cpu_affinity: Option<usize>) -> Self {
        Self {
            worker_id,
            cpu_affinity,
            numa_node: (cpu_affinity.unwrap_or(0) / 4) as u32,
            active_tasks_count: 0,
            total_tasks_executed: 0,
            is_idle: true,
        }
    }
}

/// Individual Thread Pool Instance
pub struct SovereignThreadPool {
    pub pool_id: String,
    pub kind: ThreadPoolKind,
    pub workers: Vec<SovereignWorkerThread>,
    pub task_queue: Vec<SovereignTask>,
    pub max_workers: usize,
    pub min_workers: usize,
    pub total_submitted: u64,
    pub total_completed: u64,
    pub total_failed: u64,
}

impl SovereignThreadPool {
    pub fn new(pool_id: &str, kind: ThreadPoolKind, min_workers: usize, max_workers: usize) -> Self {
        let mut workers = Vec::new();
        for i in 0..min_workers {
            let affinity = match kind {
                ThreadPoolKind::AffinityPinnedPool => Some(i),
                _ => None,
            };
            workers.push(SovereignWorkerThread::new(i, affinity));
        }

        Self {
            pool_id: pool_id.to_string(),
            kind,
            workers,
            task_queue: Vec::new(),
            max_workers,
            min_workers,
            total_submitted: 0,
            total_completed: 0,
            total_failed: 0,
        }
    }

    /// Submit a new task into the thread pool
    pub fn submit_task<F>(&mut self, name: &str, priority: TaskPriority, f: F) -> u64
    where
        F: Fn() -> Result<String, String> + Send + Sync + 'static,
    {
        self.total_submitted += 1;
        let task_id = self.total_submitted;

        let task = SovereignTask {
            id: task_id,
            name: name.to_string(),
            priority,
            target_cpu_affinity: None,
            payload: Box::new(f),
        };

        // Priority queues keep tasks ordered by TaskPriority (Critical first)
        let mut insert_idx = self.task_queue.len();
        for (i, t) in self.task_queue.iter().enumerate() {
            if task.priority < t.priority {
                insert_idx = i;
                break;
            }
        }
        self.task_queue.insert(insert_idx, task);

        // Dynamic worker expansion if pool is unbound and queue is backing up
        if self.kind == ThreadPoolKind::UnboundDynamicPool && self.workers.len() < self.max_workers {
            let next_id = self.workers.len();
            self.workers.push(SovereignWorkerThread::new(next_id, None));
        }

        task_id
    }

    /// Execute next pending task in queue (simulated worker thread execution loop)
    pub fn process_next_task(&mut self) -> Option<Result<String, String>> {
        if self.task_queue.is_empty() {
            return None;
        }

        // Work-stealing or priority dispatch: select idle worker
        let task = self.task_queue.remove(0);

        let idle_worker_opt = self.workers.iter_mut().find(|w| w.is_idle);
        let worker = if let Some(w) = idle_worker_opt {
            w
        } else {
            &mut self.workers[0]
        };

        worker.is_idle = false;
        worker.active_tasks_count += 1;

        let result = (task.payload)();

        worker.active_tasks_count -= 1;
        worker.total_tasks_executed += 1;
        worker.is_idle = true;

        match &result {
            Ok(_) => self.total_completed += 1,
            Err(_) => self.total_failed += 1,
        }

        Some(result)
    }

    /// Drain and process all pending tasks
    pub fn process_all(&mut self) -> Vec<Result<String, String>> {
        let mut results = Vec::new();
        while let Some(res) = self.process_next_task() {
            results.push(res);
        }
        results
    }
}

/// Master Multi-Pool Orchestrator Manager
pub struct SovereignThreadPoolEngine {
    pub pools: HashMap<String, SovereignThreadPool>,
    pub default_pool_id: String,
}

impl SovereignThreadPoolEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            pools: HashMap::new(),
            default_pool_id: "system_wq".to_string(),
        };
        engine.register_default_pools();
        engine
    }

    fn register_default_pools(&mut self) {
        // Linux system_wq equivalent
        self.pools.insert(
            "system_wq".to_string(),
            SovereignThreadPool::new("system_wq", ThreadPoolKind::FixedWorkerPool, 4, 16),
        );

        // Linux highpri_wq / FreeBSD taskqueue_fast
        self.pools.insert(
            "highpri_wq".to_string(),
            SovereignThreadPool::new("highpri_wq", ThreadPoolKind::PriorityTaskQueue, 4, 32),
        );

        // Linux unbound_wq / Tokio multi-thread scheduler
        self.pools.insert(
            "unbound_wq".to_string(),
            SovereignThreadPool::new("unbound_wq", ThreadPoolKind::WorkStealingPool, 2, 64),
        );

        // CPU Core Pinned / Real-Time Pool
        self.pools.insert(
            "pinned_wq".to_string(),
            SovereignThreadPool::new("pinned_wq", ThreadPoolKind::AffinityPinnedPool, 8, 8),
        );
    }

    pub fn create_pool(&mut self, pool_id: &str, kind: ThreadPoolKind, min_workers: usize, max_workers: usize) {
        self.pools.insert(
            pool_id.to_string(),
            SovereignThreadPool::new(pool_id, kind, min_workers, max_workers),
        );
    }

    pub fn get_pool_mut(&mut self, pool_id: &str) -> Option<&mut SovereignThreadPool> {
        self.pools.get_mut(pool_id)
    }

    pub fn submit_to_pool<F>(
        &mut self,
        pool_id: &str,
        name: &str,
        priority: TaskPriority,
        f: F,
    ) -> Result<u64, String>
    where
        F: Fn() -> Result<String, String> + Send + Sync + 'static,
    {
        let pool = self
            .pools
            .get_mut(pool_id)
            .ok_or_else(|| format!("Thread pool '{}' not found", pool_id))?;
        Ok(pool.submit_task(name, priority, f))
    }

    pub fn process_pool_tasks(&mut self, pool_id: &str) -> Result<Vec<Result<String, String>>, String> {
        let pool = self
            .pools
            .get_mut(pool_id)
            .ok_or_else(|| format!("Thread pool '{}' not found", pool_id))?;
        Ok(pool.process_all())
    }

    pub fn total_pools(&self) -> usize {
        self.pools.len()
    }
}

impl Default for SovereignThreadPoolEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Standalone Unit Test Suite
// ============================================================================

#[cfg(test)]
mod thread_pool_tests {
    use super::*;

    #[test]
    fn test_thread_pool_creation_and_task_execution() {
        let mut pool = SovereignThreadPool::new("test_pool", ThreadPoolKind::FixedWorkerPool, 2, 4);
        assert_eq!(pool.workers.len(), 2);

        let t1 = pool.submit_task("task_1", TaskPriority::Normal, || Ok("res1".to_string()));
        let t2 = pool.submit_task("task_2", TaskPriority::Critical, || Ok("res2".to_string()));

        assert_eq!(pool.task_queue.len(), 2);
        // Critical priority task should be first in queue
        assert_eq!(pool.task_queue[0].name, "task_2");

        let results = pool.process_all();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].as_ref().unwrap(), "res2");
        assert_eq!(results[1].as_ref().unwrap(), "res1");
        assert_eq!(pool.total_completed, 2);
    }

    #[test]
    fn test_master_thread_pool_engine_orchestration() {
        let mut engine = SovereignThreadPoolEngine::new();
        assert!(engine.total_pools() >= 4);

        let submit_res = engine.submit_to_pool(
            "highpri_wq",
            "kernel_pqc_key_regen",
            TaskPriority::Critical,
            || Ok("Key re-generated".to_string()),
        );
        assert!(submit_res.is_ok());

        let exec_res = engine.process_pool_tasks("highpri_wq").unwrap();
        assert_eq!(exec_res.len(), 1);
        assert_eq!(exec_res[0].as_ref().unwrap(), "Key re-generated");
    }

    #[test]
    fn test_affinity_pinned_pool_numa_distribution() {
        let pool = SovereignThreadPool::new("numa_pool", ThreadPoolKind::AffinityPinnedPool, 8, 8);
        assert_eq!(pool.workers.len(), 8);
        assert_eq!(pool.workers[0].cpu_affinity, Some(0));
        assert_eq!(pool.workers[4].cpu_affinity, Some(4));
        assert_eq!(pool.workers[4].numa_node, 1);
    }
}
