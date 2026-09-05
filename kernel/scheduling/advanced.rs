// SPDX-License-Identifier: MIT
/// Advanced Scheduling Policies
/// Implements SCHED_RR (round-robin) and SCHED_FIFO (first-in-first-out)

use alloc::collections::{VecDeque, BTreeMap};
use alloc::sync::Arc;
use core::sync::atomic::Mutex;

/// Scheduling Policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingPolicy {
    Normal,      // Default EEVDF scheduler
    RoundRobin,  // SCHED_RR - time sliced
    FIFO,        // SCHED_FIFO - fixed priority
}

/// Scheduling Parameters
#[derive(Debug, Clone, Copy)]
pub struct SchedulingParams {
    pub policy: SchedulingPolicy,
    pub priority: i32,      // -20 (high) to 19 (low) for normal, 1-99 for RT
    pub time_slice_ms: u32, // Time slice for RR in milliseconds
}

impl SchedulingParams {
    /// Create default parameters
    pub fn default() -> Self {
        Self {
            policy: SchedulingPolicy::Normal,
            priority: 0,
            time_slice_ms: 100,
        }
    }
    
    /// Create round-robin parameters
    pub fn round_robin(priority: i32, time_slice_ms: u32) -> Self {
        Self {
            policy: SchedulingPolicy::RoundRobin,
            priority,
            time_slice_ms,
        }
    }
    
    /// Create FIFO parameters
    pub fn fifo(priority: i32) -> Self {
        Self {
            policy: SchedulingPolicy::FIFO,
            priority,
            time_slice_ms: u32::MAX,  // No time slice for FIFO
        }
    }
    
    /// Validate parameters
    pub fn validate(&self) -> Result<(), &'static str> {
        match self.policy {
            SchedulingPolicy::Normal => {
                if self.priority < -20 || self.priority > 19 {
                    return Err("Invalid priority for normal scheduler");
                }
            }
            SchedulingPolicy::RoundRobin | SchedulingPolicy::FIFO => {
                if self.priority < 1 || self.priority > 99 {
                    return Err("Real-time priority must be 1-99");
                }
            }
        }
        Ok(())
    }
}

/// Round-Robin Scheduler
/// Uses time-sliced scheduling for equal-priority processes
pub struct RoundRobinScheduler {
    /// Ready queue per priority level
    ready_queues: BTreeMap<i32, VecDeque<u32>>,
    /// Current time slice counter
    current_time_slice: u32,
    /// Time slice duration
    time_slice_duration: u32,
}

impl RoundRobinScheduler {
    /// Create new round-robin scheduler
    pub fn new(time_slice_duration: u32) -> Self {
        Self {
            ready_queues: BTreeMap::new(),
            current_time_slice: 0,
            time_slice_duration,
        }
    }
    
    /// Add process to scheduler
    pub fn add_process(&mut self, pid: u32, priority: i32) -> Result<(), &'static str> {
        if priority < 1 || priority > 99 {
            return Err("Invalid priority");
        }
        
        self.ready_queues
            .entry(priority)
            .or_insert_with(VecDeque::new)
            .push_back(pid);
        
        Ok(())
    }
    
    /// Remove process from scheduler
    pub fn remove_process(&mut self, pid: u32) -> Result<(), &'static str> {
        for queue in self.ready_queues.values_mut() {
            queue.retain(|&p| p != pid);
        }
        Ok(())
    }
    
    /// Get next process to run
    pub fn next_process(&mut self) -> Option<u32> {
        // Find highest priority queue with processes
        for queue in self.ready_queues.values_mut().rev() {
            if let Some(pid) = queue.pop_front() {
                self.current_time_slice = 0;
                return Some(pid);
            }
        }
        None
    }
    
    /// Update time slice counter
    /// Returns true if time slice expired
    pub fn update_time_slice(&mut self, elapsed_ms: u32) -> bool {
        self.current_time_slice += elapsed_ms;
        self.current_time_slice >= self.time_slice_duration
    }
    
    /// Requeue process after time slice expiration
    pub fn requeue_process(&mut self, pid: u32, priority: i32) -> Result<(), &'static str> {
        self.ready_queues
            .entry(priority)
            .or_insert_with(VecDeque::new)
            .push_back(pid);
        
        Ok(())
    }
}

/// FIFO Scheduler
/// Fixed-priority scheduling without time slicing
pub struct FIFOScheduler {
    /// Ready queue per priority level
    ready_queues: BTreeMap<i32, VecDeque<u32>>,
}

impl FIFOScheduler {
    /// Create new FIFO scheduler
    pub fn new() -> Self {
        Self {
            ready_queues: BTreeMap::new(),
        }
    }
    
    /// Add process to scheduler
    pub fn add_process(&mut self, pid: u32, priority: i32) -> Result<(), &'static str> {
        if priority < 1 || priority > 99 {
            return Err("Invalid priority");
        }
        
        self.ready_queues
            .entry(priority)
            .or_insert_with(VecDeque::new)
            .push_back(pid);
        
        Ok(())
    }
    
    /// Remove process from scheduler
    pub fn remove_process(&mut self, pid: u32) -> Result<(), &'static str> {
        for queue in self.ready_queues.values_mut() {
            queue.retain(|&p| p != pid);
        }
        Ok(())
    }
    
    /// Get next process to run (never preempts in FIFO)
    pub fn next_process(&mut self) -> Option<u32> {
        // Find highest priority queue with processes
        for queue in self.ready_queues.values_mut().rev() {
            if let Some(pid) = queue.pop_front() {
                // Requeue at end (FIFO within priority)
                queue.push_back(pid);
                return Some(*queue.back().unwrap());
            }
        }
        None
    }
    
    /// Change priority of process
    pub fn change_priority(
        &mut self,
        pid: u32,
        old_priority: i32,
        new_priority: i32,
    ) -> Result<(), &'static str> {
        if new_priority < 1 || new_priority > 99 {
            return Err("Invalid priority");
        }
        
        // Remove from old queue
        if let Some(queue) = self.ready_queues.get_mut(&old_priority) {
            queue.retain(|&p| p != pid);
        }
        
        // Add to new queue at front (priority boost)
        self.ready_queues
            .entry(new_priority)
            .or_insert_with(VecDeque::new)
            .push_front(pid);
        
        Ok(())
    }
}

impl Default for FIFOScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Advanced Scheduling Manager
/// Manages both RR and FIFO schedulers
pub struct AdvancedSchedulingManager {
    /// Round-robin scheduler
    rr_scheduler: Arc<Mutex<RoundRobinScheduler>>,
    /// FIFO scheduler
    fifo_scheduler: Arc<Mutex<FIFOScheduler>>,
    /// Process to policy mapping
    process_policies: Arc<Mutex<BTreeMap<u32, SchedulingPolicy>>>,
}

impl AdvancedSchedulingManager {
    /// Create new advanced scheduling manager
    pub fn new(time_slice_duration: u32) -> Self {
        Self {
            rr_scheduler: Arc::new(Mutex::new(RoundRobinScheduler::new(time_slice_duration))),
            fifo_scheduler: Arc::new(Mutex::new(FIFOScheduler::new())),
            process_policies: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }
    
    /// Add process with scheduling policy
    pub fn add_process(
        &self,
        pid: u32,
        params: SchedulingParams,
    ) -> Result<(), &'static str> {
        params.validate()?;
        
        let mut policies = self.process_policies.lock().unwrap();
        policies.insert(pid, params.policy);
        
        match params.policy {
            SchedulingPolicy::RoundRobin => {
                let mut rr = self.rr_scheduler.lock().unwrap();
                rr.add_process(pid, params.priority)
            }
            SchedulingPolicy::FIFO => {
                let mut fifo = self.fifo_scheduler.lock().unwrap();
                fifo.add_process(pid, params.priority)
            }
            SchedulingPolicy::Normal => {
                // Normal scheduling handled elsewhere
                Ok(())
            }
        }
    }
    
    /// Remove process from schedulers
    pub fn remove_process(&self, pid: u32) -> Result<(), &'static str> {
        let mut policies = self.process_policies.lock().unwrap();
        policies.remove(&pid);
        
        let mut rr = self.rr_scheduler.lock().unwrap();
        let mut fifo = self.fifo_scheduler.lock().unwrap();
        
        rr.remove_process(pid)?;
        fifo.remove_process(pid)?;
        
        Ok(())
    }
    
    /// Get next process to run
    pub fn next_process(&self, policy: SchedulingPolicy) -> Option<u32> {
        match policy {
            SchedulingPolicy::RoundRobin => {
                let mut rr = self.rr_scheduler.lock().unwrap();
                rr.next_process()
            }
            SchedulingPolicy::FIFO => {
                let mut fifo = self.fifo_scheduler.lock().unwrap();
                fifo.next_process()
            }
            SchedulingPolicy::Normal => None,
        }
    }
}

impl Default for AdvancedSchedulingManager {
    fn default() -> Self {
        Self::new(100)  // 100ms default time slice
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scheduling_params_creation() {
        let params = SchedulingParams::default();
        assert_eq!(params.policy, SchedulingPolicy::Normal);
        assert_eq!(params.priority, 0);
    }
    
    #[test]
    fn test_rr_params_valid() {
        let params = SchedulingParams::round_robin(50, 100);
        assert!(params.validate().is_ok());
    }
    
    #[test]
    fn test_fifo_params_valid() {
        let params = SchedulingParams::fifo(50);
        assert!(params.validate().is_ok());
    }
    
    #[test]
    fn test_invalid_priority() {
        let params = SchedulingParams::fifo(100);  // Invalid
        assert!(params.validate().is_err());
    }
    
    #[test]
    fn test_rr_scheduler_add_process() {
        let mut rr = RoundRobinScheduler::new(100);
        assert!(rr.add_process(1, 50).is_ok());
        assert!(rr.add_process(2, 50).is_ok());
        assert!(rr.add_process(3, 25).is_ok());
    }
    
    #[test]
    fn test_rr_scheduler_next_process() {
        let mut rr = RoundRobinScheduler::new(100);
        rr.add_process(1, 50).unwrap();
        rr.add_process(2, 60).unwrap();
        
        // Should get highest priority first
        let pid = rr.next_process();
        assert!(pid.is_some());
    }
    
    #[test]
    fn test_fifo_scheduler_add_process() {
        let mut fifo = FIFOScheduler::new();
        assert!(fifo.add_process(1, 50).is_ok());
        assert!(fifo.add_process(2, 50).is_ok());
    }
    
    #[test]
    fn test_advanced_manager_creation() {
        let manager = AdvancedSchedulingManager::new(100);
        let params = SchedulingParams::round_robin(50, 100);
        assert!(manager.add_process(1, params).is_ok());
    }
}
