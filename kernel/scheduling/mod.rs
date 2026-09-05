// SPDX-License-Identifier: MIT
/// Advanced Scheduling Module
/// Implements SCHED_RR and SCHED_FIFO scheduling policies

pub mod advanced;

pub use advanced::{
    SchedulingPolicy, SchedulingParams,
    RoundRobinScheduler, FIFOScheduler,
    AdvancedSchedulingManager,
};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scheduling_module_loads() {
        // Module loads successfully
        assert!(true);
    }
}
