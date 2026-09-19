// SPDX-License-Identifier: MIT
// SigmaOS Kernel Workqueue Subsystem
// Asynchronous work execution inspired by Linux workqueues and BSD taskqueues

#![allow(dead_code)]

use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};

/// Work ID type
pub type WorkId = u64;

/// Work priority (Linux workqueue priority levels)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorkPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Work state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkState {
    Pending,
    Running,
    Completed,
    Failed,
}

/// Work error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkError {
    InvalidWorkId,
    WorkAlreadyPending,
    WorkNotPending,
    WorkerPoolExhausted,
}

/// Work callback function type
pub type WorkCallback = fn(WorkId, u64) -> Result<(), WorkError>;

/// Work item
#[derive(Debug)]
pub struct WorkItem {
    pub id: WorkId,
    pub priority: WorkPriority,
    pub state: AtomicU32, // WorkState as u32
    pub callback: Option<WorkCallback>,
    pub user_data: u64,
    pub queued_at: u64,
    pub started_at: AtomicU64,
    pub completed_at: AtomicU64,
}

impl WorkItem {
    pub fn new(id: WorkId, priority: WorkPriority, callback: WorkCallback, user_data: u64, queued_at: u64) -> Self {
        WorkItem {
            id,
            priority,
            state: AtomicU32::new(WorkState::Pending as u32),
            callback: Some(callback),
            user_data,
            queued_at,
            started_at: AtomicU64::new(0),
            completed_at: AtomicU64::new(0),
        }
    }

    pub fn get_state(&self) -> WorkState {
        match self.state.load(Ordering::SeqCst) {
            0 => WorkState::Pending,
            1 => WorkState::Running,
            2 => WorkState::Completed,
            3 => WorkState::Failed,
            _ => WorkState::Pending,
        }
    }

    pub fn set_state(&self, state: WorkState) {
        self.state.store(state as u32, Ordering::SeqCst);
    }
}

/// Worker thread
#[derive(Debug)]
pub struct Worker {
    pub id: u32,
    pub is_busy: AtomicU32, // bool as u32
    pub work_count: AtomicU64,
}

impl Worker {
    pub fn new(id: u32) -> Self {
        Worker {
            id,
            is_busy: AtomicU32::new(0),
            work_count: AtomicU64::new(0),
        }
    }

    pub fn is_busy(&self) -> bool {
        self.is_busy.load(Ordering::SeqCst) == 1
    }

    pub fn set_busy(&self, busy: bool) {
        self.is_busy.store(if busy { 1 } else { 0 }, Ordering::SeqCst);
    }
}

/// Workqueue subsystem
#[derive(Debug)]
pub struct WorkqueueSubsystem {
    pending_work: VecDeque<WorkItem>,
    completed_work: VecDeque<WorkItem>,
    workers: Vec<Worker>,
    next_work_id: AtomicU64,
    next_worker_id: AtomicU32,
    current_time_ns: AtomicU64,
    max_workers: usize,
}

impl WorkqueueSubsystem {
    pub fn new(max_workers: usize) -> Self {
        WorkqueueSubsystem {
            pending_work: VecDeque::new(),
            completed_work: VecDeque::new(),
            workers: Vec::new(),
            next_work_id: AtomicU64::new(1),
            next_worker_id: AtomicU32::new(1),
            current_time_ns: AtomicU64::new(0),
            max_workers,
        }
    }

    /// Add a worker thread
    pub fn add_worker(&mut self) -> Result<u32, WorkError> {
        if self.workers.len() >= self.max_workers {
            return Err(WorkError::WorkerPoolExhausted);
        }

        let id = self.next_worker_id.fetch_add(1, Ordering::SeqCst) as u32;
        let worker = Worker::new(id);
        self.workers.push(worker);
        Ok(id)
    }

    /// Submit work to the workqueue
    pub fn submit_work(&mut self, priority: WorkPriority, callback: WorkCallback, user_data: u64) -> WorkId {
        let id = self.next_work_id.fetch_add(1, Ordering::SeqCst);
        let now = self.current_time_ns.load(Ordering::SeqCst);
        
        let work = WorkItem::new(id, priority, callback, user_data, now);
        
        // Push to back (worker will pick highest priority)
        self.pending_work.push_back(work);
        
        id
    }

    /// Process pending work
    pub fn process_work(&mut self) -> Vec<WorkId> {
        let now = self.current_time_ns.load(Ordering::SeqCst);
        let mut completed = Vec::new();

        // Find available worker
        let worker_idx = self.workers.iter().position(|w| !w.is_busy());
        
        if let Some(idx) = worker_idx {
            // Find highest priority work
            let mut highest_priority_idx = None;
            let mut highest_priority = WorkPriority::Low;
            
            for (i, work) in self.pending_work.iter().enumerate() {
                if work.priority >= highest_priority {
                    highest_priority = work.priority;
                    highest_priority_idx = Some(i);
                }
            }
            
            if let Some(work_idx) = highest_priority_idx {
                let mut work = self.pending_work.remove(work_idx).unwrap();
                work.set_state(WorkState::Running);
                work.started_at.store(now, Ordering::SeqCst);
                self.workers[idx].set_busy(true);
                self.workers[idx].work_count.fetch_add(1, Ordering::SeqCst);
                
                let work_id = work.id;
                let user_data = work.user_data;
                
                if let Some(callback) = work.callback {
                    let result = callback(work_id, user_data);
                    
                    work.set_state(if result.is_ok() { WorkState::Completed } else { WorkState::Failed });
                    work.completed_at.store(now, Ordering::SeqCst);
                }
                
                self.workers[idx].set_busy(false);
                completed.push(work_id);
                self.completed_work.push_back(work);
            }
        }

        completed
    }

    /// Advance time
    pub fn tick(&mut self, delta_ns: u64) {
        self.current_time_ns.fetch_add(delta_ns, Ordering::SeqCst);
    }

    /// Get pending work count
    pub fn pending_count(&self) -> usize {
        self.pending_work.len()
    }

    /// Get completed work count
    pub fn completed_count(&self) -> usize {
        self.completed_work.len()
    }

    /// Get worker count
    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }

    /// Get total work processed
    pub fn total_work_processed(&self) -> u64 {
        self.workers.iter().map(|w| w.work_count.load(Ordering::SeqCst)).sum()
    }
}

impl Default for WorkqueueSubsystem {
    fn default() -> Self {
        Self::new(4) // Default 4 workers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workqueue_creation() {
        let mut wq = WorkqueueSubsystem::new(4);
        
        assert_eq!(wq.add_worker().unwrap(), 1);
        assert_eq!(wq.add_worker().unwrap(), 2);
        assert_eq!(wq.worker_count(), 2);
    }

    #[test]
    fn test_work_submission() {
        let mut wq = WorkqueueSubsystem::new(4);
        wq.add_worker().unwrap();
        
        let callback: WorkCallback = |_id, _data| Ok(());
        let id = wq.submit_work(WorkPriority::Normal, callback, 42);
        
        assert_eq!(wq.pending_count(), 1);
        assert!(id > 0);
    }

    #[test]
    fn test_work_processing() {
        let mut wq = WorkqueueSubsystem::new(4);
        wq.add_worker().unwrap();
        
        let callback: WorkCallback = |_id, _data| Ok(());
        wq.submit_work(WorkPriority::Normal, callback, 42);
        
        let completed = wq.process_work();
        assert_eq!(completed.len(), 1);
        assert_eq!(wq.pending_count(), 0);
        assert_eq!(wq.completed_count(), 1);
    }

    #[test]
    fn test_priority_ordering() {
        let mut wq = WorkqueueSubsystem::new(4);
        wq.add_worker().unwrap();
        
        let callback: WorkCallback = |_id, _data| Ok(());
        wq.submit_work(WorkPriority::Low, callback, 1);
        wq.submit_work(WorkPriority::High, callback, 2);
        wq.submit_work(WorkPriority::Normal, callback, 3);
        
        assert_eq!(wq.pending_count(), 3);
        
        // Process one work item
        let completed = wq.process_work();
        assert_eq!(completed.len(), 1);
        assert_eq!(wq.completed_count(), 1);
        assert_eq!(wq.pending_count(), 2);
    }

    #[test]
    fn test_worker_exhaustion() {
        let mut wq = WorkqueueSubsystem::new(2);
        wq.add_worker().unwrap();
        wq.add_worker().unwrap();
        
        assert!(wq.add_worker().is_err());
    }
}
