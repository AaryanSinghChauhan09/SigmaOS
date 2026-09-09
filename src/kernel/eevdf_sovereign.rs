#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
#![allow(clippy::new_without_default)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ─── EEVDF Task State ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskState {
    Runnable,
    Running,
    Blocked,
    Dead,
}

// ─── EEVDF Sched Entity ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct EevdfSchedEntity {
    pub pid: u32,
    pub name: String,
    pub weight: u32,            // task weight (1-10000, default 1024 like NICE 0)
    pub slice_ns: u64,          // requested slice (e.g. 5ms for latency, 20ms for batch)
    pub vruntime: u64,          // virtual runtime in ns
    pub deadline: u64,          // virtual deadline = vruntime + slice / weight
    pub exec_time_ns: u64,      // actual physical runtime accumulated
    pub state: TaskState,
}

impl EevdfSchedEntity {
    pub fn new(pid: u32, name: &str, weight: u32, slice_ns: u64) -> Self {
        let weight = weight.clamp(1, 10000);
        let slice_ns = slice_ns.clamp(1_000_000, 100_000_000); // 1ms - 100ms
        let deadline = (slice_ns * 1024) / weight as u64;
        EevdfSchedEntity {
            pid,
            name: name.to_string(),
            weight,
            slice_ns,
            vruntime: 0,
            deadline,
            exec_time_ns: 0,
            state: TaskState::Runnable,
        }
    }

    /// Calculate virtual deadline based on current vruntime, slice, and weight
    pub fn update_deadline(&mut self) {
        let scaled_slice = (self.slice_ns * 1024) / self.weight as u64;
        self.deadline = self.vruntime.saturating_add(scaled_slice);
    }
}

// ─── Sovereign EEVDF Scheduler ────────────────────────────────────────────────

pub struct SovereignEevdfScheduler {
    pub entities: Vec<EevdfSchedEntity>,
    pub current_running: Option<usize>,
    pub avg_vruntime: u64,      // V: average virtual runtime (system virtual time)
    pub total_weight: u64,
    pub total_switches: u64,
}

impl SovereignEevdfScheduler {
    pub fn new() -> Self {
        SovereignEevdfScheduler {
            entities: Vec::new(),
            current_running: None,
            avg_vruntime: 0,
            total_weight: 0,
            total_switches: 0,
        }
    }

    pub fn add_task(&mut self, entity: EevdfSchedEntity) {
        self.total_weight = self.total_weight.saturating_add(entity.weight as u64);
        self.entities.push(entity);
        self.recompute_avg_vruntime();
    }

    pub fn recompute_avg_vruntime(&mut self) {
        let runnable_count = self.entities.iter().filter(|e| e.state == TaskState::Runnable || e.state == TaskState::Running).count();
        if runnable_count == 0 {
            return;
        }
        let sum_vruntime: u64 = self.entities.iter()
            .filter(|e| e.state == TaskState::Runnable || e.state == TaskState::Running)
            .map(|e| e.vruntime)
            .sum();
        self.avg_vruntime = sum_vruntime / runnable_count as u64;
    }

    /// Check if task is eligible: vruntime <= V (lag >= 0)
    pub fn is_eligible(&self, entity: &EevdfSchedEntity) -> bool {
        entity.vruntime <= self.avg_vruntime
    }

    /// EEVDF Core Selection: Earliest Virtual Deadline among all Eligible tasks
    pub fn pick_next_task(&mut self) -> Option<u32> {
        self.recompute_avg_vruntime();

        let mut best_idx: Option<usize> = None;
        let mut earliest_deadline = u64::MAX;

        // First pass: search among eligible runnable tasks
        for (i, e) in self.entities.iter().enumerate() {
            if e.state == TaskState::Runnable && self.is_eligible(e) {
                if e.deadline < earliest_deadline {
                    earliest_deadline = e.deadline;
                    best_idx = Some(i);
                }
            }
        }

        // Fallback: if no eligible task exists, pick task with minimum vruntime (classic CFS fallback)
        if best_idx.is_none() {
            let mut min_vruntime = u64::MAX;
            for (i, e) in self.entities.iter().enumerate() {
                if e.state == TaskState::Runnable && e.vruntime < min_vruntime {
                    min_vruntime = e.vruntime;
                    best_idx = Some(i);
                }
            }
        }

        if let Some(idx) = best_idx {
            // Context switch
            if let Some(curr_idx) = self.current_running {
                if curr_idx < self.entities.len() && self.entities[curr_idx].state == TaskState::Running {
                    self.entities[curr_idx].state = TaskState::Runnable;
                }
            }
            self.entities[idx].state = TaskState::Running;
            self.current_running = Some(idx);
            self.total_switches = self.total_switches.saturating_add(1);
            Some(self.entities[idx].pid)
        } else {
            None
        }
    }

    /// Advance scheduler by executing current task for `delta_ns`
    pub fn tick(&mut self, delta_ns: u64) {
        if let Some(idx) = self.current_running {
            if idx < self.entities.len() {
                let e = &mut self.entities[idx];
                e.exec_time_ns = e.exec_time_ns.saturating_add(delta_ns);
                // virtual runtime progress scaled inversely to weight
                let delta_vruntime = (delta_ns * 1024) / e.weight as u64;
                e.vruntime = e.vruntime.saturating_add(delta_vruntime);
                e.update_deadline();
            }
        }
        self.recompute_avg_vruntime();
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eevdf_deadline_calculation() {
        let e1 = EevdfSchedEntity::new(1, "latency_task", 1024, 2_000_000);  // 2ms slice
        let e2 = EevdfSchedEntity::new(2, "throughput_task", 1024, 20_000_000); // 20ms slice
        assert!(e1.deadline < e2.deadline); // Short slice gets earlier initial deadline
    }

    #[test]
    fn test_eevdf_priority_scheduling() {
        let mut sched = SovereignEevdfScheduler::new();
        // Two tasks with same slice, but task 1 has 4x higher weight
        let e1 = EevdfSchedEntity::new(1, "high_weight", 4096, 10_000_000);
        let e2 = EevdfSchedEntity::new(2, "low_weight", 1024, 10_000_000);
        sched.add_task(e1);
        sched.add_task(e2);

        // e1 should have earlier deadline due to higher weight
        let next_pid = sched.pick_next_task().unwrap();
        assert_eq!(next_pid, 1);
    }

    #[test]
    fn test_eevdf_virtual_runtime_tick() {
        let mut sched = SovereignEevdfScheduler::new();
        let e = EevdfSchedEntity::new(10, "worker", 1024, 10_000_000);
        sched.add_task(e);
        sched.pick_next_task();
        sched.tick(5_000_000); // 5ms execution

        assert_eq!(sched.entities[0].exec_time_ns, 5_000_000);
        assert_eq!(sched.entities[0].vruntime, 5_000_000);
    }

    #[test]
    fn test_eevdf_fair_alternation() {
        let mut sched = SovereignEevdfScheduler::new();
        let e1 = EevdfSchedEntity::new(1, "task1", 1024, 5_000_000);
        let e2 = EevdfSchedEntity::new(2, "task2", 1024, 5_000_000);
        sched.add_task(e1);
        sched.add_task(e2);

        let p1 = sched.pick_next_task().unwrap();
        sched.tick(10_000_000); // Task 1 runs and advances its vruntime past task 2
        let p2 = sched.pick_next_task().unwrap();
        assert_ne!(p1, p2); // Should switch to task 2
    }

    #[test]
    fn test_eevdf_eligibility_enforcement() {
        let mut sched = SovereignEevdfScheduler::new();
        let mut e1 = EevdfSchedEntity::new(1, "early", 1024, 5_000_000);
        e1.vruntime = 0;
        let mut e2 = EevdfSchedEntity::new(2, "future", 1024, 5_000_000);
        e2.vruntime = 100_000_000; // Far in the future
        sched.add_task(e1);
        sched.add_task(e2);

        // Task 1 should be picked as it is eligible
        let next = sched.pick_next_task().unwrap();
        assert_eq!(next, 1);
    }

    #[test]
    fn test_eevdf_empty_scheduler() {
        let mut sched = SovereignEevdfScheduler::new();
        assert!(sched.pick_next_task().is_none());
        sched.tick(1_000_000);
        assert_eq!(sched.total_switches, 0);
    }
}
