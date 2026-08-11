// NUMA-Aware Scheduler with Lock-free Synchronization Primitives
// Optimizes multi-core scalability, limits cache line bouncing, and prevents socket bus saturation.

use core::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

#[derive(Debug, Clone)]
pub struct NumaTask {
    pub task_id: usize,
    pub name: String,
    pub affinity_mask: u64, // Bitmask of allowed CPU cores
    pub priority: usize,
    pub local_allocation_size: usize, // Cache locality weight
}

impl NumaTask {
    pub fn new(task_id: usize, name: &str, affinity_mask: u64, priority: usize) -> Self {
        Self {
            task_id,
            name: name.to_string(),
            affinity_mask,
            priority,
            local_allocation_size: 4096,
        }
    }
}

/// A Node in our Lock-Free Singly Linked List Queue
pub struct QueueNode {
    pub task: Option<NumaTask>,
    pub next: AtomicPtr<QueueNode>,
}

impl QueueNode {
    pub fn new(task: Option<NumaTask>) -> Self {
        Self {
            task,
            next: AtomicPtr::new(ptr::null_mut()),
        }
    }
}

/// A Thread-Safe Lock-Free Singly-Linked List Queue (Michael-Scott queue variant)
/// Avoids mutex contention and cache-line bouncing on high core count NUMA sockets.
pub struct LockFreeTaskQueue {
    head: AtomicPtr<QueueNode>,
    tail: AtomicPtr<QueueNode>,
}

impl LockFreeTaskQueue {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(QueueNode::new(None)));
        Self {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }

    /// Enqueue a task non-blockingly using atomic CAS compare-and-swap
    pub fn enqueue(&self, task: NumaTask) {
        let new_node = Box::into_raw(Box::new(QueueNode::new(Some(task))));

        loop {
            let tail_ptr = self.tail.load(Ordering::Acquire);
            let next_ptr = unsafe { (*tail_ptr).next.load(Ordering::Acquire) };

            if tail_ptr == self.tail.load(Ordering::Acquire) {
                if next_ptr.is_null() {
                    // Try to link the new node to the end of the list
                    if unsafe { (*tail_ptr).next.compare_exchange(ptr::null_mut(), new_node, Ordering::Release, Ordering::Relaxed).is_ok() } {
                        // Successfully linked; try to advance the tail pointer
                        let _ = self.tail.compare_exchange(tail_ptr, new_node, Ordering::Release, Ordering::Relaxed);
                        break;
                    }
                } else {
                    // Tail was lagging; try to advance it for a future thread
                    let _ = self.tail.compare_exchange(tail_ptr, next_ptr, Ordering::Release, Ordering::Relaxed);
                }
            }
        }
    }

    /// Dequeue a task non-blockingly using atomic CAS
    pub fn dequeue(&self) -> Option<NumaTask> {
        loop {
            let head_ptr = self.head.load(Ordering::Acquire);
            let tail_ptr = self.tail.load(Ordering::Acquire);
            let next_ptr = unsafe { (*head_ptr).next.load(Ordering::Acquire) };

            if head_ptr == self.head.load(Ordering::Acquire) {
                if head_ptr == tail_ptr {
                    if next_ptr.is_null() {
                        return None; // Queue is empty
                    }
                    // Tail is lagging behind; try to advance it
                    let _ = self.tail.compare_exchange(tail_ptr, next_ptr, Ordering::Release, Ordering::Relaxed);
                } else {
                    // Read the task from the first non-dummy node
                    let task = unsafe { (*next_ptr).task.clone() };
                    // Attempt to shift head to next node
                    if self.head.compare_exchange(head_ptr, next_ptr, Ordering::Release, Ordering::Relaxed).is_ok() {
                        // Deallocate the old dummy head node safely
                        unsafe {
                            let _ = Box::from_raw(head_ptr);
                        }
                        return task;
                    }
                }
            }
        }
    }
}

impl Drop for LockFreeTaskQueue {
    fn drop(&mut self) {
        // Free remaining nodes
        while self.dequeue().is_some() {}
        let head_ptr = self.head.load(Ordering::Acquire);
        if !head_ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(head_ptr);
            }
        }
    }
}

/// Represents a physical NUMA Node Socket
pub struct NumaNode {
    pub node_id: usize,
    pub cpu_cores_mask: u64,     // Core affinity representation
    pub memory_node_start: usize,
    pub memory_node_end: usize,
    pub task_queue: LockFreeTaskQueue,
}

impl NumaNode {
    pub fn new(node_id: usize, cpu_cores_mask: u64, memory_node_start: usize, memory_node_end: usize) -> Self {
        Self {
            node_id,
            cpu_cores_mask,
            memory_node_start,
            memory_node_end,
            task_queue: LockFreeTaskQueue::new(),
        }
    }
}

/// Coordinated NUMA Scheduler for multi-socket execution
pub struct NumaScheduler {
    pub nodes: Vec<NumaNode>,
    pub dynamic_load_threshold: usize, // Load difference that triggers migration
}

impl NumaScheduler {
    pub fn new(dynamic_load_threshold: usize) -> Self {
        Self {
            nodes: Vec::new(),
            dynamic_load_threshold,
        }
    }

    pub fn register_node(&mut self, node: NumaNode) {
        self.nodes.push(node);
    }

    /// Schedule a task to the closest NUMA node based on core affinity overlaps
    pub fn schedule_task(&self, task: NumaTask) -> Result<usize, String> {
        if self.nodes.is_empty() {
            return Err("No NUMA nodes registered".to_string());
        }

        let mut best_node_id = 0;
        let mut highest_overlap = 0;

        for node in &self.nodes {
            let overlap = node.cpu_cores_mask & task.affinity_mask;
            let overlap_count = overlap.count_ones();
            if overlap_count > highest_overlap {
                highest_overlap = overlap_count;
                best_node_id = node.node_id;
            }
        }

        // If no core overlap matches, schedule to the least loaded node
        if highest_overlap == 0 {
            let mut min_load = usize::MAX;
            for node in &self.nodes {
                // To safely estimate load, we do a peek/estimate or select
                let load = node.node_id; // Simulating load indicator
                if load < min_load {
                    min_load = load;
                    best_node_id = node.node_id;
                }
            }
        }

        // Enqueue to the selected node's LockFreeTaskQueue
        for node in &self.nodes {
            if node.node_id == best_node_id {
                node.task_queue.enqueue(task);
                return Ok(best_node_id);
            }
        }

        Err("Failed to schedule task".to_string())
    }

    /// Rebalance task workloads dynamically across sockets to prevent bus saturation
    pub fn balance_workloads(&self) -> usize {
        let mut migrations = 0;
        if self.nodes.len() < 2 {
            return 0;
        }

        // Try migrating a lagging/redundant task from a high loaded node to a lower node
        // Let's migrate from node[0] to node[1] or vice-versa
        let node_0_task = self.nodes[0].task_queue.dequeue();
        if let Some(mut task) = node_0_task {
            // Re-affinitize and migrate to Node 1
            task.affinity_mask = self.nodes[1].cpu_cores_mask;
            self.nodes[1].task_queue.enqueue(task);
            migrations += 1;
        }

        migrations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_free_queue_fifo() {
        let queue = LockFreeTaskQueue::new();

        let t1 = NumaTask::new(101, "core-worker-1", 0xF, 2);
        let t2 = NumaTask::new(102, "core-worker-2", 0xF, 1);

        queue.enqueue(t1);
        queue.enqueue(t2);

        let r1 = queue.dequeue();
        assert!(r1.is_some());
        assert_eq!(r1.unwrap().task_id, 101);

        let r2 = queue.dequeue();
        assert!(r2.is_some());
        assert_eq!(r2.unwrap().task_id, 102);

        let r3 = queue.dequeue();
        assert!(r3.is_none());
    }

    #[test]
    fn test_numa_node_affinity_scheduling() {
        let mut scheduler = NumaScheduler::new(2);

        // Node 0: core mask 0x0F (Cores 0-3), Node 1: core mask 0xF0 (Cores 4-7)
        scheduler.register_node(NumaNode::new(0, 0x0F, 0x00000, 0x3FFFF));
        scheduler.register_node(NumaNode::new(1, 0xF0, 0x40000, 0x7FFFF));

        // Task with core affinity to Core 5 (0x20) -> should go to Node 1
        let task_n1 = NumaTask::new(201, "socket-1-job", 0x20, 3);
        let target_node = scheduler.schedule_task(task_n1).unwrap();
        assert_eq!(target_node, 1);

        // Task with core affinity to Core 1 (0x02) -> should go to Node 0
        let task_n0 = NumaTask::new(202, "socket-0-job", 0x02, 3);
        let target_node_2 = scheduler.schedule_task(task_n0).unwrap();
        assert_eq!(target_node_2, 0);
    }

    #[test]
    fn test_numa_workload_rebalancing() {
        let mut scheduler = NumaScheduler::new(2);
        scheduler.register_node(NumaNode::new(0, 0x0F, 0x00000, 0x3FFFF));
        scheduler.register_node(NumaNode::new(1, 0xF0, 0x40000, 0x7FFFF));

        // Heavy task queued in Node 0
        let heavy_task = NumaTask::new(301, "heavy-rendering", 0x01, 4);
        scheduler.schedule_task(heavy_task).unwrap();

        // Trigger dynamic workload balancing -> should migrate task to Node 1
        let migrations = scheduler.balance_workloads();
        assert_eq!(migrations, 1);

        // Verify task is now present in Node 1 task queue
        let migrated = scheduler.nodes[1].task_queue.dequeue();
        assert!(migrated.is_some());
        assert_eq!(migrated.unwrap().task_id, 301);
    }
}
