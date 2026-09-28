// Sovereign Software Engineering Paradigms & Core OS Principles Engine for SigmaOS (`src/kernel/sovereign_software_engineering_paradigms.rs`)
// Demonstrates clean code, OOP principles (Encapsulation, Abstraction, Polymorphism, Composition),
// SOLID guidelines (SRP, OCP, LSP, ISP, DIP), Design by Contract (DbC), and Concurrency/Deadlock Detection.

extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// =========================================================================
// 1. SOLID: INTERFACE SEGREGATION PRINCIPLE (ISP) & ABSTRACTION
// Fine-grained interfaces for I/O operations instead of bloated traits.
// =========================================================================

pub trait SovereignStreamReadable {
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, &'static str>;
}

pub trait SovereignStreamWritable {
    fn write_bytes(&mut self, buf: &[u8]) -> Result<usize, &'static str>;
}

pub trait SovereignSeekable {
    fn seek_offset(&mut self, offset: u64) -> Result<u64, &'static str>;
}

// =========================================================================
// 2. OOP POLYMORPHISM & SCHEDULER ABSTRACTION (Open/Closed Principle)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SovereignTaskControlBlock {
    pub pid: u32,
    pub name: String,
    pub priority: u8,
    pub cpu_time_ms: u64,
}

pub trait ISovereignTaskSchedulerPolicy {
    fn select_next_task<'a>(
        &self,
        ready_queue: &'a [SovereignTaskControlBlock],
    ) -> Option<&'a SovereignTaskControlBlock>;
    fn policy_name(&self) -> &'static str;
}

/// EEVDF (Earliest Eligible Virtual Deadline First) Scheduler
pub struct EevdfSchedulerPolicy;

impl ISovereignTaskSchedulerPolicy for EevdfSchedulerPolicy {
    fn select_next_task<'a>(
        &self,
        ready_queue: &'a [SovereignTaskControlBlock],
    ) -> Option<&'a SovereignTaskControlBlock> {
        ready_queue.iter().min_by_key(|t| t.cpu_time_ms)
    }

    fn policy_name(&self) -> &'static str {
        "EEVDF Scheduler"
    }
}

/// Round-Robin Scheduler Policy
pub struct RoundRobinSchedulerPolicy;

impl ISovereignTaskSchedulerPolicy for RoundRobinSchedulerPolicy {
    fn select_next_task<'a>(
        &self,
        ready_queue: &'a [SovereignTaskControlBlock],
    ) -> Option<&'a SovereignTaskControlBlock> {
        ready_queue.first()
    }

    fn policy_name(&self) -> &'static str {
        "Round-Robin Scheduler"
    }
}

// =========================================================================
// 3. DESIGN BY CONTRACT (DbC) & DEADLOCK DETECTION (Coffman Conditions)
// Preconditions, Postconditions, Invariants & Coffman Circular Wait Detector
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceLockRequest {
    pub process_id: u32,
    pub resource_id: u32,
}

pub struct SovereignDeadlockLockdepVerifier {
    pub held_locks: BTreeMap<u32, u32>,       // resource_id -> process_id
    pub waiting_locks: BTreeMap<u32, u32>,    // process_id -> resource_id requested
}

impl SovereignDeadlockLockdepVerifier {
    pub fn new() -> Self {
        Self {
            held_locks: BTreeMap::new(),
            waiting_locks: BTreeMap::new(),
        }
    }

    /// Precondition: Process ID and Resource ID must be non-zero
    /// Invariant: No process can hold and wait for the exact same resource
    /// Postcondition: Lock request registered or circular wait deadlock detected
    pub fn request_resource(&mut self, pid: u32, resource_id: u32) -> Result<(), &'static str> {
        // Contract Precondition
        if pid == 0 || resource_id == 0 {
            return Err("Precondition Failed: Invalid PID or Resource ID");
        }

        // Check Coffman Circular Wait Condition
        if let Some(&holding_pid) = self.held_locks.get(&resource_id) {
            self.waiting_locks.insert(pid, resource_id);
            if self.detect_circular_wait(pid, holding_pid) {
                // Postcondition Exception
                return Err("Deadlock Detected: Coffman Circular Wait Condition Violates Invariant");
            }
        } else {
            self.held_locks.insert(resource_id, pid);
        }

        Ok(())
    }

    fn detect_circular_wait(&self, start_pid: u32, current_holding_pid: u32) -> bool {
        let mut curr = current_holding_pid;
        let mut visited = Vec::new();
        visited.push(curr);

        while let Some(&req_res) = self.waiting_locks.get(&curr) {
            if let Some(&next_holding_pid) = self.held_locks.get(&req_res) {
                if next_holding_pid == start_pid {
                    return true; // Circular dependency graph detected
                }
                if visited.contains(&next_holding_pid) {
                    break; // Non-start cycle detected, prevent infinite loop
                }
                visited.push(next_holding_pid);
                curr = next_holding_pid;
            } else {
                break;
            }
        }
        false
    }
}

impl Default for SovereignDeadlockLockdepVerifier {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. COMPOSITION OVER INHERITANCE: PROCESS MANAGEMENT HUB
// Single Responsibility Principle (SRP) Orchestrator
// =========================================================================

pub struct SovereignProcessManagementEngine {
    pub active_scheduler: Box<dyn ISovereignTaskSchedulerPolicy>,
    pub task_queue: Vec<SovereignTaskControlBlock>,
    pub deadlock_verifier: SovereignDeadlockLockdepVerifier,
}

impl SovereignProcessManagementEngine {
    pub fn new(scheduler: Box<dyn ISovereignTaskSchedulerPolicy>) -> Self {
        Self {
            active_scheduler: scheduler,
            task_queue: Vec::new(),
            deadlock_verifier: SovereignDeadlockLockdepVerifier::new(),
        }
    }

    pub fn spawn_task(&mut self, pid: u32, name: &str, priority: u8) {
        self.task_queue.push(SovereignTaskControlBlock {
            pid,
            name: name.to_string(),
            priority,
            cpu_time_ms: 0,
        });
    }

    pub fn schedule_next(&self) -> Option<SovereignTaskControlBlock> {
        self.active_scheduler
            .select_next_task(&self.task_queue)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polymorphic_scheduler() {
        let eevdf = EevdfSchedulerPolicy;
        let mut engine = SovereignProcessManagementEngine::new(Box::new(eevdf));
        engine.spawn_task(1, "init", 10);
        engine.spawn_task(2, "systemd-daemon", 5);

        let scheduled = engine.schedule_next();
        assert!(scheduled.is_some());
        assert_eq!(engine.active_scheduler.policy_name(), "EEVDF Scheduler");
    }

    #[test]
    fn test_design_by_contract_and_deadlock_detection() {
        let mut lockdep = SovereignDeadlockLockdepVerifier::new();

        // P1 acquires R1
        assert!(lockdep.request_resource(1, 101).is_ok());
        // P2 acquires R2
        assert!(lockdep.request_resource(2, 102).is_ok());

        // P1 requests R2 (held by P2)
        assert!(lockdep.request_resource(1, 102).is_ok());

        // P2 requests R1 (held by P1) -> Circular Wait Deadlock!
        let result = lockdep.request_resource(2, 101);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Deadlock Detected"));
    }
}
