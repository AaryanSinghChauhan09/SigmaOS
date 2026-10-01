// SigmaOS Energy-Aware Scheduler (EAS)
// Predicts and balances task thread execution energy cost vs. thermal/battery constraints
// Enhanced with CPU frequency scaling and thermal throttling integration

use std::vec::Vec;

/// CPU frequency states for energy management
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuFrequency {
    Min = 800,       // 800 MHz - minimum power
    Low = 1200,      // 1.2 GHz - power saving
    Nominal = 2400,  // 2.4 GHz - balanced
    Turbo = 3200,     // 3.2 GHz - performance
    Max = 4800,      // 4.8 GHz - maximum performance
}

impl CpuFrequency {
    pub fn as_mhz(&self) -> u32 {
        *self as u32
    }

    pub fn power_factor(&self) -> f32 {
        match self {
            CpuFrequency::Min => 0.3,
            CpuFrequency::Low => 0.5,
            CpuFrequency::Nominal => 1.0,
            CpuFrequency::Turbo => 1.8,
            CpuFrequency::Max => 2.5,
        }
    }
}

/// Thermal state for throttling decisions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalState {
    Normal = 0,
    Throttling = 1,
    Critical = 2,
}

pub struct TaskEnergyCost {
    pub task_id: u32,
    pub expected_cpu_cycles: u64,
    pub expected_temp_increase_celsius: f32,
    pub priority: u8, // 0-255, higher is more important
}

pub struct EnergyAwareScheduler {
    pub tasks_pool: Vec<TaskEnergyCost>,
    pub battery_level_percentage: u32,
    pub ec_throttle_threshold: u32,
    pub current_frequency: CpuFrequency,
    pub thermal_state: ThermalState,
    pub current_temperature_celsius: f32,
    pub thermal_threshold_critical: f32,
    pub thermal_threshold_throttle: f32,
}

impl EnergyAwareScheduler {
    pub fn new() -> Self {
        EnergyAwareScheduler {
            tasks_pool: Vec::new(),
            battery_level_percentage: 100,
            ec_throttle_threshold: 20, // Throttle tasks when battery drops below 20%
            current_frequency: CpuFrequency::Nominal,
            thermal_state: ThermalState::Normal,
            current_temperature_celsius: 45.0,
            thermal_threshold_critical: 85.0,
            thermal_threshold_throttle: 70.0,
        }
    }

    pub fn queue_task_prediction(&mut self, tid: u32, cycles: u64, temp: f32) {
        self.tasks_pool.push(TaskEnergyCost {
            task_id: tid,
            expected_cpu_cycles: cycles,
            expected_temp_increase_celsius: temp,
            priority: 128, // default medium priority
        });
    }

    pub fn queue_task_with_priority(&mut self, tid: u32, cycles: u64, temp: f32, priority: u8) {
        self.tasks_pool.push(TaskEnergyCost {
            task_id: tid,
            expected_cpu_cycles: cycles,
            expected_temp_increase_celsius: temp,
            priority,
        });
    }

    /// Update thermal state based on current temperature
    pub fn update_thermal_state(&mut self) {
        if self.current_temperature_celsius >= self.thermal_threshold_critical {
            self.thermal_state = ThermalState::Critical;
            self.current_frequency = CpuFrequency::Min;
        } else if self.current_temperature_celsius >= self.thermal_threshold_throttle {
            self.thermal_state = ThermalState::Throttling;
            self.current_frequency = CpuFrequency::Low;
        } else {
            self.thermal_state = ThermalState::Normal;
            // Restore frequency based on battery level
            if self.battery_level_percentage < self.ec_throttle_threshold {
                self.current_frequency = CpuFrequency::Low;
            } else {
                self.current_frequency = CpuFrequency::Nominal;
            }
        }
    }

    /// Set current temperature (simulated or from hardware sensor)
    pub fn set_temperature(&mut self, temp_celsius: f32) {
        self.current_temperature_celsius = temp_celsius;
        self.update_thermal_state();
    }

    /// Get current CPU frequency
    pub fn get_frequency(&self) -> CpuFrequency {
        self.current_frequency
    }

    /// Set CPU frequency manually (for testing or external control)
    pub fn set_frequency(&mut self, freq: CpuFrequency) {
        self.current_frequency = freq;
    }

    /// Get energy budget based on battery and thermal state
    pub fn get_energy_budget(&self) -> f32 {
        let battery_factor = self.battery_level_percentage as f32 / 100.0;
        let thermal_factor = match self.thermal_state {
            ThermalState::Normal => 1.0,
            ThermalState::Throttling => 0.7,
            ThermalState::Critical => 0.4,
        };
        battery_factor * thermal_factor
    }

    pub fn schedule_next_task(&mut self) -> Option<u32> {
        if self.tasks_pool.is_empty() {
            return None;
        }

        let energy_budget = self.get_energy_budget();

        // Priority-based scheduling with energy awareness
        let mut best_idx = 0;
        let mut best_score = -1.0f32;

        for (i, task) in self.tasks_pool.iter().enumerate() {
            // Calculate score: priority - energy cost factor
            let energy_cost = (task.expected_cpu_cycles as f32 / 1_000_000.0) * task.expected_temp_increase_celsius;
            let freq_factor = self.current_frequency.power_factor();
            let adjusted_cost = energy_cost * freq_factor;
            let score = (task.priority as f32) - (adjusted_cost / energy_budget);

            if score > best_score {
                best_score = score;
                best_idx = i;
            }
        }

        Some(self.tasks_pool.remove(best_idx).task_id)
    }

    /// Get scheduler statistics
    pub fn get_stats(&self) -> (usize, CpuFrequency, ThermalState, f32) {
        (
            self.tasks_pool.len(),
            self.current_frequency,
            self.thermal_state,
            self.current_temperature_celsius,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_aware_scheduling_balanced() {
        let mut eas = EnergyAwareScheduler::new();
        eas.queue_task_prediction(1, 10000, 1.5);
        eas.queue_task_prediction(2, 100, 0.1);

        let task = eas.schedule_next_task();
        assert!(task.is_some());
    }

    #[test]
    fn test_thermal_throttling() {
        let mut eas = EnergyAwareScheduler::new();

        // Set critical temperature
        eas.set_temperature(90.0);
        assert_eq!(eas.thermal_state, ThermalState::Critical);
        assert_eq!(eas.current_frequency, CpuFrequency::Min);

        // Return to normal
        eas.set_temperature(50.0);
        assert_eq!(eas.thermal_state, ThermalState::Normal);
    }

    #[test]
    fn test_frequency_scaling() {
        let mut eas = EnergyAwareScheduler::new();

        assert_eq!(eas.get_frequency(), CpuFrequency::Nominal);

        eas.set_frequency(CpuFrequency::Turbo);
        assert_eq!(eas.get_frequency(), CpuFrequency::Turbo);

        eas.set_frequency(CpuFrequency::Max);
        assert_eq!(eas.get_frequency(), CpuFrequency::Max);
    }

    #[test]
    fn test_priority_scheduling() {
        let mut eas = EnergyAwareScheduler::new();
        eas.queue_task_with_priority(1, 10000, 1.5, 255); // high priority
        eas.queue_task_with_priority(2, 100, 0.1, 0); // low priority
        eas.queue_task_with_priority(2, 100, 0.1, 0);   // low priority

        let task = eas.schedule_next_task();
        assert!(task.is_some());
        assert_eq!(task.unwrap(), 1); // high priority should schedule first
    }

    #[test]
    fn test_energy_budget() {
        let mut eas = EnergyAwareScheduler::new();

        // Normal state
        let budget = eas.get_energy_budget();
        assert!(budget > 0.9);

        // Low battery
        eas.battery_level_percentage = 10;
        let budget = eas.get_energy_budget();
        assert!(budget < 0.2);
    }

    #[test]
    fn test_scheduler_stats() {
        let eas = EnergyAwareScheduler::new();
        eas.queue_task_prediction(1, 10000, 1.5);

        let (pool_size, freq, thermal, temp) = eas.get_stats();
        assert_eq!(pool_size, 1);
        assert_eq!(freq, CpuFrequency::Nominal);
        assert_eq!(thermal, ThermalState::Normal);
        assert_eq!(temp, 45.0);
    }
}
