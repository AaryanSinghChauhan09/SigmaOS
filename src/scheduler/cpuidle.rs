// SigmaOS CPU Idle Subsystem (cpuidle)
// Governs dynamic CPU power states (C-states) based on predicted idle residency and exit latencies.

#![allow(dead_code)]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// CPU C-State definition
#[derive(Debug, Clone)]
pub struct CState {
    pub name: String,
    pub exit_latency_us: u32,
    pub target_residency_us: u32,
    pub power_consumption_mw: u32,
    pub usage_count: u64,
    pub total_time_spent_us: u64,
}

/// CPU Idle Governor
#[derive(Debug, Clone)]
pub struct CpuIdleGovernor {
    pub states: Vec<CState>,
    pub current_state_idx: usize,
}

impl CpuIdleGovernor {
    pub fn new_standard_x86() -> Self {
        let states = alloc::vec![
            CState {
                name: String::from("C1_POLL"),
                exit_latency_us: 1,
                target_residency_us: 2,
                power_consumption_mw: 25000,
                usage_count: 0,
                total_time_spent_us: 0,
            },
            CState {
                name: String::from("C1E_HALT"),
                exit_latency_us: 10,
                target_residency_us: 20,
                power_consumption_mw: 15000,
                usage_count: 0,
                total_time_spent_us: 0,
            },
            CState {
                name: String::from("C3_SLEEP"),
                exit_latency_us: 50,
                target_residency_us: 100,
                power_consumption_mw: 5000,
                usage_count: 0,
                total_time_spent_us: 0,
            },
            CState {
                name: String::from("C6_DEEP"),
                exit_latency_us: 150,
                target_residency_us: 400,
                power_consumption_mw: 800,
                usage_count: 0,
                total_time_spent_us: 0,
            },
        ];

        Self {
            states,
            current_state_idx: 0,
        }
    }

    /// Select deepest C-state where target residency <= predicted idle time
    /// and exit latency <= max latency requirement
    pub fn select_cstate(&self, predicted_idle_us: u32, max_allowed_latency_us: u32) -> usize {
        let mut best_idx = 0;

        for (idx, state) in self.states.iter().enumerate() {
            if state.exit_latency_us <= max_allowed_latency_us
                && state.target_residency_us <= predicted_idle_us
            {
                best_idx = idx;
            }
        }

        best_idx
    }

    /// Record actual sleep duration upon CPU wakeup
    pub fn record_idle_wakeup(&mut self, state_idx: usize, actual_slept_us: u64) {
        if state_idx < self.states.len() {
            self.states[state_idx].usage_count += 1;
            self.states[state_idx].total_time_spent_us += actual_slept_us;
            self.current_state_idx = state_idx;
        }
    }
}

impl Default for CpuIdleGovernor {
    fn default() -> Self {
        Self::new_standard_x86()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpuidle_selection() {
        let governor = CpuIdleGovernor::new_standard_x86();

        // Short idle (15us) -> C1E (residency 20 is too high for C1E? residency 2 for C1)
        let cstate_short = governor.select_cstate(15, 100);
        assert_eq!(cstate_short, 0); // C1_POLL

        // Medium idle (200us) with relaxed latency -> C3
        let cstate_med = governor.select_cstate(200, 100);
        assert_eq!(cstate_med, 2); // C3_SLEEP

        // Long idle (1000us) -> C6_DEEP
        let cstate_long = governor.select_cstate(1000, 500);
        assert_eq!(cstate_long, 3); // C6_DEEP
    }
}
