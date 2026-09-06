#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::vec::Vec;
// SigmaOS Energy-Aware Scheduler (EAS)
// Predicts and balances task thread execution energy cost vs. thermal/battery constraints

pub struct TaskEnergyCost {
    pub task_id: u32,
    pub expected_cpu_cycles: u64,
    pub expected_temp_increase_celsius: f32,
}

pub struct EnergyAwareScheduler {
    pub tasks_pool: Vec<TaskEnergyCost>,
    pub battery_level_percentage: u32,
    pub ec_throttle_threshold: u32,
}

impl EnergyAwareScheduler {
    pub fn new() -> Self {
        EnergyAwareScheduler {
            tasks_pool: Vec::new(),
            battery_level_percentage: 100,
            ec_throttle_threshold: 20, // Throttle tasks when battery drops below 20%
        }
    }

    pub fn queue_task_prediction(&mut self, tid: u32, cycles: u64, temp: f32) {
        self.tasks_pool.push(TaskEnergyCost {
            task_id: tid,
            expected_cpu_cycles: cycles,
            expected_temp_increase_celsius: temp,
        });
    }

    pub fn schedule_next_task(&mut self) -> Option<u32> {
        if self.tasks_pool.is_empty() {
            return None;
        }

        // Under energy-conserving / low-battery states, prioritize lower cycle/cooler tasks to prevent thermal burst
        if self.battery_level_percentage < self.ec_throttle_threshold {
            let mut best_idx = 0;
            let mut min_cycles = self.tasks_pool[0].expected_cpu_cycles;
            for (i, task) in self.tasks_pool.iter().enumerate() {
                if task.expected_cpu_cycles < min_cycles {
                    min_cycles = task.expected_cpu_cycles;
                    best_idx = i;
                }
            }
            Some(self.tasks_pool.remove(best_idx).task_id)
        } else {
            // Otherwise, schedule FIFO standard queues
            Some(self.tasks_pool.remove(0).task_id)
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_aware_scheduling_balanced() {
        let mut eas = EnergyAwareScheduler::new();
        eas.queue_task_prediction(1, 10000, 1.5);
        eas.queue_task_prediction(2, 100, 0.1); // Small fast task

        // Default battery is 100%, schedules sequentially (FIFO)
        assert_eq!(eas.schedule_next_task().unwrap(), 1);
        assert_eq!(eas.schedule_next_task().unwrap(), 2);
    }

    #[test]
    fn test_energy_aware_scheduling_conserve() {
        let mut eas = EnergyAwareScheduler::new();
        eas.battery_level_percentage = 15; // Low battery!

        eas.queue_task_prediction(1, 10000, 1.5); // heavy math task
        eas.queue_task_prediction(2, 100, 0.1); // light UI task

        // Throttled: schedules the lighter task 2 first to conserve CPU cycles!
        assert_eq!(eas.schedule_next_task().unwrap(), 2);
        assert_eq!(eas.schedule_next_task().unwrap(), 1);
    }
}
