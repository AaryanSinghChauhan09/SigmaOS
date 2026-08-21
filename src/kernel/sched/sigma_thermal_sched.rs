#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::kernel::sched::task::{ProcessState, SchedPolicy, Task};
use crate::kernel::sched::scheduler::{SchedClass, RunQueue};
use crate::filesystem::FsError;

/// Thermal zone reading
#[derive(Debug, Clone, Copy)]
pub struct ThermalZone {
    pub id: u32,
    pub temperature_c: f32,
    pub critical_temp_c: f32,
    pub passive_delay_ms: u32,
}

/// Power consumption model
#[derive(Debug, Clone, Copy)]
pub struct PowerConsumption {
    pub cpu_id: u32,
    pub watts: f32,
    pub voltage_mv: u32,
    pub current_ma: u32,
}

/// Thermal-aware Scheduler
///
/// Adjusts scheduling decisions based on temperature readings
/// and power consumption to prevent thermal throttling.
pub struct ThermalScheduler {
    pub zones: Vec<ThermalZone>,
    pub power_history: Vec<PowerConsumption>,
    pub throttle_threshold: f32,
    pub current_temp: f32,
    pub cpu_power_budget: f32,
}

impl ThermalScheduler {
    pub fn new() -> Self {
        ThermalScheduler {
            zones: Vec::new(),
            power_history: Vec::new(),
            throttle_threshold: 90.0,
            current_temp: 40.0,
            cpu_power_budget: 150.0,
        }
    }

    pub fn register_zone(&mut self, zone: ThermalZone) {
        self.zones.push(zone);
    }

    pub fn update_temperature(&mut self, zone_id: u32, temp_c: f32) {
        if let Some(zone) = self.zones.iter_mut().find(|z| z.id == zone_id) {
            zone.temperature_c = temp_c;
        }
        self.current_temp = temp_c;
    }

    pub fn is_throttling(&self) -> bool {
        self.current_temp >= self.throttle_threshold
            || self.zones.iter().any(|z| z.temperature_c >= z.critical_temp_c)
    }

    pub fn adjust_scheduling(&self, task: &Task) -> SchedulingDecision {
        if self.is_throttling() {
            if task.policy == SchedPolicy::Batch || task.policy == SchedPolicy::Normal {
                return SchedulingDecision::Throttle {
                    max_freq_mhz: 800,
                    time_slice_reduction: 0.5,
                };
            }
        }
        SchedulingDecision::Normal
    }

    pub fn estimate_discharge(&self, cpu_id: u32, load_pct: f32) -> f32 {
        let base_power = if let Some(p) = self.power_history.iter().find(|p| p.cpu_id == cpu_id) {
            p.watts
        } else {
            10.0
        };
        base_power * (load_pct / 100.0)
    }

    pub fn per_shard_power(&self, shard_id: u64) -> Option<f32> {
        let _ = shard_id;
        Some(self.cpu_power_budget / self.zones.len().max(1) as f32)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SchedulingDecision {
    Normal,
    Throttle { max_freq_mhz: u32, time_slice_reduction: f32 },
    Migrate { target_cpu: u32 },
}

pub struct ThermalSchedClass {
    pub thermal: ThermalScheduler,
}

impl ThermalSchedClass {
    pub fn new() -> Self {
        ThermalSchedClass {
            thermal: ThermalScheduler::new(),
        }
    }
}

impl SchedClass for ThermalSchedClass {
    fn enqueue_task(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }

    fn dequeue_task(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }

    fn yield_task(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }

    fn check_preempt_curr(&self, _rq: &mut RunQueue, _task: &Task) -> bool {
        false
    }

    fn pick_next_task(&self, _rq: &mut RunQueue) -> Option<u64> {
        None
    }

    fn put_prev_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}

    fn set_curr_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}

    fn task_tick(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }

    fn task_fork(
        &self,
        _rq: &mut RunQueue,
        _child: &mut Task,
        _parent: &Task,
    ) -> Result<(), FsError> {
        Ok(())
    }

    fn task_dead(&self, _rq: &mut RunQueue, _task: &mut Task) {}

    fn prio_changed(&self, _rq: &mut RunQueue, _task: &mut Task) {}
}

impl Default for ThermalScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_scheduler_creation() {
        let sched = ThermalScheduler::new();
        assert_eq!(sched.current_temp, 40.0);
    }

    #[test]
    fn test_thermal_throttling() {
        let mut sched = ThermalScheduler::new();
        sched.register_zone(ThermalZone {
            id: 0,
            temperature_c: 95.0,
            critical_temp_c: 100.0,
            passive_delay_ms: 100,
        });
        sched.update_temperature(0, 95.0);
        assert!(sched.is_throttling());
    }

    #[test]
    fn test_thermal_decision() {
        let sched = ThermalScheduler::new();
        let task = Task {
            pid: 1,
            tgid: 1,
            parent_pid: 0,
            name: String::new(),
            state: ProcessState::Running,
            priority: 120,
            static_prio: 120,
            normal_prio: 120,
            policy: SchedPolicy::Normal,
            cred: crate::kernel::sched::task::Cred::new(),
            mm: None,
            files: Vec::new(),
            fs: None,
            signal: crate::kernel::sched::task::SignalStruct {
                pending: Vec::new(),
                blocked: 0,
                signal: 0,
                flags: 0,
                si_code: 0,
            },
            children: Vec::new(),
            siblings: Vec::new(),
            group_leader: 1,
            real_parent: 0,
            thread_group: Vec::new(),
            vmas: Vec::new(),
            flags: 0,
            exit_code: 0,
            exit_signal: 0,
            start_time: 0,
            utime: 0,
            stime: 0,
            cutime: 0,
            cstime: 0,
            nvcsw: 0,
            nivcsw: 0,
            seccomp_mode: 0,
            seccomp_filter: None,
        };
        let decision = sched.adjust_scheduling(&task);
        assert!(matches!(decision, SchedulingDecision::Normal));
    }
}
