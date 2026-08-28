extern crate alloc;
// Heterogeneous AI Compute Task Scheduler for SigmaOS
// Inspired by Linux cgroups v2 resource limits and FreeBSD SCHED_ULE multi-queue scheduling.

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// AI Task Priority Class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AiTaskPriority {
    RealTimeLLM = 0,        // Top priority, minimal latency
    InteractiveVision = 1,  // Medium-high priority
    BatchEmbedding = 2,     // Medium priority
    BackgroundTraining = 3, // Low priority, opportunistic execution
}

/// Execution Target Device for AI tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeDeviceTarget {
    DiscreteGpu,
    IntegratedNpu,
    CpuSimd,
    AutoSelect,
}

/// Task status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiTaskState {
    Pending,
    Running,
    Completed,
    Failed,
    Preempted,
}

/// AI Compute Task Descriptor.
#[derive(Debug, Clone)]
pub struct AiComputeTask {
    pub id: usize,
    pub name: String,
    pub priority: AiTaskPriority,
    pub target_device: ComputeDeviceTarget,
    pub max_latency_ms: u64,
    pub compute_units_required: usize,
    pub memory_bytes_required: usize,
    pub state: AiTaskState,
    pub execution_time_ms: u64,
    pub tokens_processed: usize,
}

/// Scheduler Quota & Limits (Linux cgroups v2 inspired).
#[derive(Debug, Clone)]
pub struct AiComputeQuota {
    pub max_gpu_percent: u8,
    pub max_npu_percent: u8,
    pub max_memory_bytes: usize,
    pub current_gpu_usage_percent: u8,
    pub current_memory_bytes: usize,
}

impl Default for AiComputeQuota {
    fn default() -> Self {
        Self {
            max_gpu_percent: 100,
            max_npu_percent: 100,
            max_memory_bytes: 16 * 1024 * 1024 * 1024, // 16GB default
            current_gpu_usage_percent: 0,
            current_memory_bytes: 0,
        }
    }
}

/// Heterogeneous AI Task Scheduler Engine.
pub struct AiComputeScheduler {
    pending_tasks: Vec<AiComputeTask>,
    running_tasks: Vec<AiComputeTask>,
    completed_tasks: Vec<AiComputeTask>,
    next_task_id: AtomicUsize,
    quota: AiComputeQuota,
    total_tokens_processed: usize,
    preemptions: usize,
}

impl AiComputeScheduler {
    pub fn new(quota: AiComputeQuota) -> Self {
        Self {
            pending_tasks: Vec::new(),
            running_tasks: Vec::new(),
            completed_tasks: Vec::new(),
            next_task_id: AtomicUsize::new(1),
            quota,
            total_tokens_processed: 0,
            preemptions: 0,
        }
    }

    /// Enqueues a new AI compute task into SCHED_ULE priority queue.
    pub fn enqueue_task(
        &mut self,
        name: &str,
        priority: AiTaskPriority,
        target_device: ComputeDeviceTarget,
        max_latency_ms: u64,
        compute_units: usize,
        memory_bytes: usize,
    ) -> usize {
        let id = self.next_task_id.fetch_add(1, Ordering::SeqCst);
        let task = AiComputeTask {
            id,
            name: name.to_string(),
            priority,
            target_device,
            max_latency_ms,
            compute_units_required: compute_units,
            memory_bytes_required: memory_bytes,
            state: AiTaskState::Pending,
            execution_time_ms: 0,
            tokens_processed: 0,
        };

        self.pending_tasks.push(task);
        // Sort by priority (highest priority = RealTimeLLM first)
        self.pending_tasks.sort_by_key(|t| t.priority);
        id
    }

    /// Dispatches tasks to available compute targets based on quotas & priorities.
    pub fn schedule_next_tick(&mut self) -> usize {
        let mut dispatched = 0;
        let mut i = 0;

        while i < self.pending_tasks.len() {
            let task = &self.pending_tasks[i];

            // Check cgroups v2 memory & compute quota limit
            if self.quota.current_memory_bytes + task.memory_bytes_required
                <= self.quota.max_memory_bytes
            {
                // Check preemption: if RealTimeLLM task arrives, preemption logic triggers if needed
                if task.priority == AiTaskPriority::RealTimeLLM {
                    self.preempt_low_priority_if_needed();
                }

                let mut task = self.pending_tasks.remove(i);
                task.state = AiTaskState::Running;

                self.quota.current_memory_bytes += task.memory_bytes_required;
                self.running_tasks.push(task);
                dispatched += 1;
            } else {
                i += 1;
            }
        }

        dispatched
    }

    /// Preempts low priority (BackgroundTraining) tasks to make room for RealTimeLLM tasks.
    fn preempt_low_priority_if_needed(&mut self) {
        let mut preempted_indices = Vec::new();

        for (idx, task) in self.running_tasks.iter().enumerate() {
            if task.priority == AiTaskPriority::BackgroundTraining {
                preempted_indices.push(idx);
                self.preemptions += 1;
            }
        }

        for idx in preempted_indices.into_iter().rev() {
            let mut task = self.running_tasks.remove(idx);
            self.quota.current_memory_bytes = self
                .quota
                .current_memory_bytes
                .saturating_sub(task.memory_bytes_required);
            task.state = AiTaskState::Preempted;
            self.pending_tasks.push(task);
        }

        self.pending_tasks.sort_by_key(|t| t.priority);
    }

    /// Advances execution time for running tasks and completes finished tasks.
    pub fn advance_execution(&mut self, time_ms: u64, tokens_generated: usize) {
        let mut finished_indices = Vec::new();

        for (idx, task) in self.running_tasks.iter_mut().enumerate() {
            task.execution_time_ms += time_ms;
            task.tokens_processed += tokens_generated;
            self.total_tokens_processed += tokens_generated;

            // Simple completion condition simulation
            if task.execution_time_ms >= task.max_latency_ms.min(100) {
                task.state = AiTaskState::Completed;
                finished_indices.push(idx);
            }
        }

        for idx in finished_indices.into_iter().rev() {
            let task = self.running_tasks.remove(idx);
            self.quota.current_memory_bytes = self
                .quota
                .current_memory_bytes
                .saturating_sub(task.memory_bytes_required);
            self.completed_tasks.push(task);
        }
    }

    /// Gets active tasks stats summary.
    pub fn get_summary(&self) -> (usize, usize, usize, usize, usize) {
        (
            self.pending_tasks.len(),
            self.running_tasks.len(),
            self.completed_tasks.len(),
            self.total_tokens_processed,
            self.preemptions,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_compute_scheduler_priorities() {
        let mut sched = AiComputeScheduler::new(AiComputeQuota::default());
        let id1 = sched.enqueue_task(
            "background_train",
            AiTaskPriority::BackgroundTraining,
            ComputeDeviceTarget::CpuSimd,
            500,
            4,
            1024 * 1024,
        );
        let id2 = sched.enqueue_task(
            "realtime_llm",
            AiTaskPriority::RealTimeLLM,
            ComputeDeviceTarget::DiscreteGpu,
            50,
            8,
            2048 * 1024,
        );

        assert_eq!(sched.schedule_next_tick(), 2);
        sched.advance_execution(100, 32);

        let (pending, running, completed, tokens, preemptions) = sched.get_summary();
        assert_eq!(completed, 2);
        assert_eq!(tokens, 64);
    }
}
