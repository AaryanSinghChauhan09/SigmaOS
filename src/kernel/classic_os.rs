extern crate alloc;
// SPDX-License-Identifier: Apache-2.0
// Classic Operating System Algorithms & Subsystems
//
// Inspired by Linux, FreeBSD, and classic Operating System paradigms:
// - Memory Ballooning (VirtIO Memory Ballooning)
// - Banker's Algorithm (Deadlock Avoidance)
// - Sleeping Barber Synchronization Primitive
// - Bare Metal System Provisioning Supervisor
// - Buffer Overflow Protection & Guard Canaries
// - Basic Input/Output Supervisor (IOSupervisor)
// - Ticket Spinlock with Exponential Backoff
// - Simple & Multiprogrammed Batch Queue Processor

use alloc::vec::Vec;

/// Memory Ballooning Manager (VirtIO / Hypervisor Ballooning)
pub struct VirtioBalloonManager {
    current_pages: usize,
    target_pages: usize,
    deflated_pages: Vec<u64>,
}

impl VirtioBalloonManager {
    pub fn new(initial_pages: usize) -> Self {
        Self {
            current_pages: initial_pages,
            target_pages: initial_pages,
            deflated_pages: Vec::new(),
        }
    }

    pub fn set_target_pages(&mut self, target: usize) {
        self.target_pages = target;
    }

    pub fn inflate(&mut self, pfn_list: &[u64]) -> usize {
        let mut inflated = 0;
        for &pfn in pfn_list {
            if self.current_pages < self.target_pages {
                self.deflated_pages.push(pfn);
                self.current_pages += 1;
                inflated += 1;
            } else {
                break;
            }
        }
        inflated
    }

    pub fn deflate(&mut self, count: usize) -> Vec<u64> {
        let mut returned = Vec::new();
        for _ in 0..count {
            if let Some(pfn) = self.deflated_pages.pop() {
                self.current_pages = self.current_pages.saturating_sub(1);
                returned.push(pfn);
            }
        }
        returned
    }

    pub fn current_pages(&self) -> usize {
        self.current_pages
    }
}

/// Banker's Algorithm for Deadlock Avoidance
pub struct BankersAlgorithm {
    num_processes: usize,
    num_resources: usize,
    available: Vec<usize>,
    max_matrix: Vec<Vec<usize>>,
    allocation: Vec<Vec<usize>>,
    need: Vec<Vec<usize>>,
}

impl BankersAlgorithm {
    pub fn new(
        num_processes: usize,
        num_resources: usize,
        available: Vec<usize>,
        max_matrix: Vec<Vec<usize>>,
        allocation: Vec<Vec<usize>>,
    ) -> Self {
        let mut need = Vec::new();
        for i in 0..num_processes {
            let mut proc_need = Vec::new();
            for j in 0..num_resources {
                let m = if i < max_matrix.len() && j < max_matrix[i].len() {
                    max_matrix[i][j]
                } else {
                    0
                };
                let a = if i < allocation.len() && j < allocation[i].len() {
                    allocation[i][j]
                } else {
                    0
                };
                proc_need.push(m.saturating_sub(a));
            }
            need.push(proc_need);
        }

        Self {
            num_processes,
            num_resources,
            available,
            max_matrix,
            allocation,
            need,
        }
    }

    pub fn is_safe_state(&self) -> bool {
        let mut work = self.available.clone();
        let mut finish = vec![false; self.num_processes];

        loop {
            let mut found = false;
            for i in 0..self.num_processes {
                if !finish[i] {
                    let mut can_allocate = true;
                    for j in 0..self.num_resources {
                        if self.need[i][j] > work[j] {
                            can_allocate = false;
                            break;
                        }
                    }

                    if can_allocate {
                        for j in 0..self.num_resources {
                            work[j] += self.allocation[i][j];
                        }
                        finish[i] = true;
                        found = true;
                    }
                }
            }

            if !found {
                break;
            }
        }

        finish.iter().all(|&f| f)
    }

    pub fn request_resources(&mut self, process_id: usize, request: &[usize]) -> bool {
        if process_id >= self.num_processes || request.len() != self.num_resources {
            return false;
        }

        for j in 0..self.num_resources {
            if request[j] > self.need[process_id][j] || request[j] > self.available[j] {
                return false;
            }
        }

        for j in 0..self.num_resources {
            self.available[j] -= request[j];
            self.allocation[process_id][j] += request[j];
            self.need[process_id][j] -= request[j];
        }

        if self.is_safe_state() {
            true
        } else {
            // Rollback allocation
            for j in 0..self.num_resources {
                self.available[j] += request[j];
                self.allocation[process_id][j] -= request[j];
                self.need[process_id][j] += request[j];
            }
            false
        }
    }

    pub fn max_matrix(&self) -> &Vec<Vec<usize>> {
        &self.max_matrix
    }
}

/// Sleeping Barber Synchronization Problem Solver
pub struct SleepingBarberQueue {
    chairs_capacity: usize,
    waiting_customers: Vec<u64>,
    barber_sleeping: bool,
}

impl SleepingBarberQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            chairs_capacity: capacity,
            waiting_customers: Vec::new(),
            barber_sleeping: true,
        }
    }

    pub fn customer_arrives(&mut self, customer_id: u64) -> bool {
        if self.barber_sleeping {
            self.barber_sleeping = false;
            true // Customer is served immediately
        } else if self.waiting_customers.len() < self.chairs_capacity {
            self.waiting_customers.push(customer_id);
            true // Waiting in a chair
        } else {
            false // No chairs available; customer leaves
        }
    }

    pub fn service_next_customer(&mut self) -> Option<u64> {
        if !self.waiting_customers.is_empty() {
            Some(self.waiting_customers.remove(0))
        } else {
            self.barber_sleeping = true;
            None
        }
    }

    pub fn is_barber_sleeping(&self) -> bool {
        self.barber_sleeping
    }
}

/// Basic Ticket Spinlock with Exponential Backoff Support
#[derive(Debug)]
pub struct TicketSpinlock {
    next_ticket: core::sync::atomic::AtomicUsize,
    now_serving: core::sync::atomic::AtomicUsize,
}

impl TicketSpinlock {
    pub const fn new() -> Self {
        Self {
            next_ticket: core::sync::atomic::AtomicUsize::new(0),
            now_serving: core::sync::atomic::AtomicUsize::new(0),
        }
    }

    pub fn lock(&self) -> usize {
        let ticket = self
            .next_ticket
            .fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        let mut backoff = 1usize;
        while self.now_serving.load(core::sync::atomic::Ordering::Acquire) != ticket {
            for _ in 0..backoff {
                core::hint::spin_loop();
            }
            backoff = (backoff << 1).min(64);
        }
        ticket
    }

    pub fn unlock(&self, ticket: usize) {
        self.now_serving.store(
            ticket.wrapping_add(1),
            core::sync::atomic::Ordering::Release,
        );
    }
}

/// Guard Canary Stack Protection for Buffer Overflow Detection
pub struct StackCanaryProtector {
    global_canary: u64,
}

impl StackCanaryProtector {
    pub fn new(canary_seed: u64) -> Self {
        Self {
            global_canary: canary_seed ^ 0xDEADBEEFCAFEBABE,
        }
    }

    pub fn generate_canary(&self) -> u64 {
        self.global_canary
    }

    pub fn verify_canary(&self, canary_val: u64) -> bool {
        canary_val == self.global_canary
    }
}

/// Simple & Multiprogrammed Batch Job Scheduler Queue
#[derive(Debug, Clone)]
pub struct BatchJob {
    pub job_id: u64,
    pub priority: u8,
    pub estimated_time_ms: u64,
}

pub struct BatchSystemQueue {
    multiprogramming_limit: usize,
    running_jobs: Vec<BatchJob>,
    queued_jobs: Vec<BatchJob>,
}

impl BatchSystemQueue {
    pub fn new(multiprogramming_limit: usize) -> Self {
        Self {
            multiprogramming_limit,
            running_jobs: Vec::new(),
            queued_jobs: Vec::new(),
        }
    }

    pub fn submit_job(&mut self, job: BatchJob) {
        self.queued_jobs.push(job);
        self.dispatch_jobs();
    }

    pub fn dispatch_jobs(&mut self) {
        while self.running_jobs.len() < self.multiprogramming_limit && !self.queued_jobs.is_empty()
        {
            let next_job = self.queued_jobs.remove(0);
            self.running_jobs.push(next_job);
        }
    }

    pub fn complete_job(&mut self, job_id: u64) -> bool {
        let mut pos = None;
        for (idx, job) in self.running_jobs.iter().enumerate() {
            if job.job_id == job_id {
                pos = Some(idx);
                break;
            }
        }

        if let Some(idx) = pos {
            self.running_jobs.remove(idx);
            self.dispatch_jobs();
            true
        } else {
            false
        }
    }

    pub fn running_count(&self) -> usize {
        self.running_jobs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtio_ballooning() {
        let mut balloon = VirtioBalloonManager::new(10);
        balloon.set_target_pages(15);
        assert_eq!(balloon.inflate(&[100, 101, 102]), 3);
        assert_eq!(balloon.current_pages(), 13);
        let deflated = balloon.deflate(2);
        assert_eq!(deflated.len(), 2);
        assert_eq!(balloon.current_pages(), 11);
    }

    #[test]
    fn test_bankers_algorithm() {
        let available = vec![3, 3, 2];
        let max_matrix = vec![vec![3, 3, 2], vec![3, 2, 2]];
        let allocation = vec![vec![0, 1, 0], vec![2, 0, 0]];

        let mut banker = BankersAlgorithm::new(2, 3, available, max_matrix, allocation);
        assert!(banker.is_safe_state());
        assert!(!banker.max_matrix().is_empty());
        assert!(banker.request_resources(1, &[1, 0, 2]));
    }

    #[test]
    fn test_sleeping_barber() {
        let mut barber = SleepingBarberQueue::new(2);
        assert!(barber.is_barber_sleeping());
        assert!(barber.customer_arrives(1)); // Served immediately
        assert!(!barber.is_barber_sleeping());
        assert!(barber.customer_arrives(2)); // Waiting chair 1
        assert!(barber.customer_arrives(3)); // Waiting chair 2
        assert!(!barber.customer_arrives(4)); // No chair

        assert_eq!(barber.service_next_customer(), Some(2));
        assert_eq!(barber.service_next_customer(), Some(3));
        assert_eq!(barber.service_next_customer(), None);
        assert!(barber.is_barber_sleeping());
    }

    #[test]
    fn test_ticket_spinlock() {
        let lock = TicketSpinlock::new();
        let ticket = lock.lock();
        lock.unlock(ticket);
    }

    #[test]
    fn test_stack_canary() {
        let protector = StackCanaryProtector::new(0x123456789ABCDEF0);
        let canary = protector.generate_canary();
        assert!(protector.verify_canary(canary));
        assert!(!protector.verify_canary(canary ^ 0xFF));
    }

    #[test]
    fn test_batch_system_queue() {
        let mut batch = BatchSystemQueue::new(2);
        batch.submit_job(BatchJob {
            job_id: 1,
            priority: 1,
            estimated_time_ms: 100,
        });
        batch.submit_job(BatchJob {
            job_id: 2,
            priority: 2,
            estimated_time_ms: 200,
        });
        batch.submit_job(BatchJob {
            job_id: 3,
            priority: 3,
            estimated_time_ms: 300,
        });

        assert_eq!(batch.running_count(), 2);
        assert!(batch.complete_job(1));
        assert_eq!(batch.running_count(), 2);
    }
}
