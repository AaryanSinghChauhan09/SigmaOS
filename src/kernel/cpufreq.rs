#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::vec;

// CPUFreq - Linux-style CPU frequency scaling governor
// Supports performance, powersave, ondemand, conservative, and schedutil governors

// (no_std only applicable at crate root - removed)

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernorType {
    Performance,  // Always max frequency
    Powersave,    // Always min frequency
    Ondemand,     // Dynamic based on load
    Conservative, // Similar to ondemand but more gradual
    Schedutil,    // Scheduler-driven frequency scaling
    Userspace,    // User-controlled frequency
}

#[derive(Debug, Clone)]
pub struct CpufreqPolicy {
    pub cpu: u32,
    pub min_freq: u32, // kHz
    pub max_freq: u32, // kHz
    pub cur_freq: u32, // kHz
    pub governor: GovernorType,
    pub transition_latency: u32, // microseconds
}

#[derive(Debug, Clone)]
pub struct CpufreqStats {
    pub time_in_state: BTreeMap<u32, u64>, // frequency -> time in microseconds
    pub total_transitions: u64,
}

pub struct CpufreqManager {
    policies: BTreeMap<u32, CpufreqPolicy>,
    stats: BTreeMap<u32, CpufreqStats>,
    available_frequencies: Vec<u32>, // kHz
    cpu_count: u32,
}

impl CpufreqManager {
    pub fn new(cpu_count: u32, available_frequencies: Vec<u32>) -> Self {
        let mut policies = BTreeMap::new();
        let mut stats = BTreeMap::new();

        for cpu in 0..cpu_count {
            let min_freq = *available_frequencies.first().unwrap_or(&800000);
            let max_freq = *available_frequencies.last().unwrap_or(&4000000);

            let policy = CpufreqPolicy {
                cpu,
                min_freq,
                max_freq,
                cur_freq: max_freq, // Start at max frequency
                governor: GovernorType::Performance,
                transition_latency: 10, // 10 microseconds
            };

            policies.insert(cpu, policy);

            let mut time_in_state = BTreeMap::new();
            for &freq in &available_frequencies {
                time_in_state.insert(freq, 0);
            }

            let stat = CpufreqStats {
                time_in_state,
                total_transitions: 0,
            };

            stats.insert(cpu, stat);
        }

        Self {
            policies,
            stats,
            available_frequencies,
            cpu_count,
        }
    }

    /// Set the governor for a CPU
    pub fn set_governor(&mut self, cpu: u32, governor: GovernorType) -> Result<(), &'static str> {
        let policy = self.policies.get_mut(&cpu).ok_or("CPU not found")?;

        policy.governor = governor;

        // Apply governor-specific frequency
        match governor {
            GovernorType::Performance => {
                policy.cur_freq = policy.max_freq;
            }
            GovernorType::Powersave => {
                policy.cur_freq = policy.min_freq;
            }
            _ => {
                // Other governors would need load-based scaling
                policy.cur_freq = (policy.min_freq + policy.max_freq) / 2;
            }
        }

        Ok(())
    }

    /// Set frequency range for a CPU
    pub fn set_frequency_range(
        &mut self,
        cpu: u32,
        min_freq: u32,
        max_freq: u32,
    ) -> Result<(), &'static str> {
        let policy = self.policies.get_mut(&cpu).ok_or("CPU not found")?;

        if min_freq < *self.available_frequencies.first().unwrap_or(&800000) {
            return Err("Minimum frequency too low");
        }

        if max_freq > *self.available_frequencies.last().unwrap_or(&4000000) {
            return Err("Maximum frequency too high");
        }

        if min_freq > max_freq {
            return Err("Minimum frequency cannot exceed maximum");
        }

        policy.min_freq = min_freq;
        policy.max_freq = max_freq;

        // Adjust current frequency if needed
        if policy.cur_freq < min_freq {
            policy.cur_freq = min_freq;
        } else if policy.cur_freq > max_freq {
            policy.cur_freq = max_freq;
        }

        Ok(())
    }

    /// Set specific frequency for a CPU
    pub fn set_frequency(&mut self, cpu: u32, freq: u32) -> Result<(), &'static str> {
        let policy = self.policies.get_mut(&cpu).ok_or("CPU not found")?;

        if freq < policy.min_freq || freq > policy.max_freq {
            return Err("Frequency out of allowed range");
        }

        if !self.available_frequencies.contains(&freq) {
            return Err("Frequency not available");
        }

        policy.cur_freq = freq;

        // Update statistics
        if let Some(stat) = self.stats.get_mut(&cpu) {
            stat.total_transitions += 1;
        }

        Ok(())
    }

    /// Get current frequency for a CPU
    pub fn get_frequency(&self, cpu: u32) -> Result<u32, &'static str> {
        let policy = self.policies.get(&cpu).ok_or("CPU not found")?;

        Ok(policy.cur_freq)
    }

    /// Get policy for a CPU
    pub fn get_policy(&self, cpu: u32) -> Result<&CpufreqPolicy, &'static str> {
        self.policies.get(&cpu).ok_or("CPU not found")
    }

    /// Get statistics for a CPU
    pub fn get_stats(&self, cpu: u32) -> Result<&CpufreqStats, &'static str> {
        self.stats.get(&cpu).ok_or("CPU not found")
    }

    /// Simulate load-based frequency scaling
    pub fn update_frequency_based_on_load(
        &mut self,
        cpu: u32,
        load: f64,
    ) -> Result<(), &'static str> {
        let policy = self.policies.get_mut(&cpu).ok_or("CPU not found")?;

        match policy.governor {
            GovernorType::Ondemand => {
                // Scale frequency based on load
                let range = (policy.max_freq - policy.min_freq) as f64;
                let target_freq = policy.min_freq as f64 + (load * range);
                policy.cur_freq = target_freq as u32;
            }
            GovernorType::Conservative => {
                // More gradual scaling
                let range = (policy.max_freq - policy.min_freq) as f64;
                let target_freq = policy.min_freq as f64 + (load * range * 0.5);
                policy.cur_freq = target_freq as u32;
            }
            GovernorType::Schedutil => {
                // Scheduler-driven (simplified)
                if load > 0.8 {
                    policy.cur_freq = policy.max_freq;
                } else if load < 0.2 {
                    policy.cur_freq = policy.min_freq;
                } else {
                    policy.cur_freq = (policy.min_freq + policy.max_freq) / 2;
                }
            }
            _ => {
                // Performance and powersave don't change based on load
            }
        }

        Ok(())
    }

    /// Get available frequencies
    pub fn available_frequencies(&self) -> &[u32] {
        &self.available_frequencies
    }

    /// Get CPU count
    pub fn cpu_count(&self) -> u32 {
        self.cpu_count
    }
}

impl Default for CpufreqManager {
    fn default() -> Self {
        Self::new(
            4,
            vec![
                800000, 1200000, 1600000, 2000000, 2400000, 2800000, 3200000, 3600000, 4000000,
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpufreq_manager() {
        let manager = CpufreqManager::new(4, vec![800000, 1600000, 2400000, 3200000, 4000000]);
        assert_eq!(manager.cpu_count(), 4);
    }

    #[test]
    fn test_set_governor() {
        let mut manager = CpufreqManager::new(2, vec![800000, 1600000, 2400000, 3200000, 4000000]);

        manager.set_governor(0, GovernorType::Powersave).unwrap();
        let freq = manager.get_frequency(0).unwrap();

        assert_eq!(freq, 800000); // Min frequency
    }

    #[test]
    fn test_set_frequency_range() {
        let mut manager = CpufreqManager::new(2, vec![800000, 1600000, 2400000, 3200000, 4000000]);

        manager.set_frequency_range(0, 1600000, 3200000).unwrap();
        let policy = manager.get_policy(0).unwrap();

        assert_eq!(policy.min_freq, 1600000);
        assert_eq!(policy.max_freq, 3200000);
    }

    #[test]
    fn test_set_frequency() {
        let mut manager = CpufreqManager::new(2, vec![800000, 1600000, 2400000, 3200000, 4000000]);

        manager.set_frequency(0, 2400000).unwrap();
        let freq = manager.get_frequency(0).unwrap();

        assert_eq!(freq, 2400000);
    }

    #[test]
    fn test_load_based_scaling() {
        let mut manager = CpufreqManager::new(2, vec![800000, 1600000, 2400000, 3200000, 4000000]);

        manager.set_governor(0, GovernorType::Ondemand).unwrap();
        manager.update_frequency_based_on_load(0, 0.8).unwrap();

        let freq = manager.get_frequency(0).unwrap();
        assert!(freq > 2400000); // Should be high with 80% load
    }

    #[test]
    fn test_stats() {
        let mut manager = CpufreqManager::new(2, vec![800000, 1600000, 2400000, 3200000, 4000000]);

        manager.set_frequency(0, 2400000).unwrap();
        let stats = manager.get_stats(0).unwrap();

        assert_eq!(stats.total_transitions, 1);
    }

    #[test]
    fn test_performance_governor() {
        let mut manager = CpufreqManager::new(2, vec![800000, 1600000, 2400000, 3200000, 4000000]);

        manager.set_governor(0, GovernorType::Performance).unwrap();
        let freq = manager.get_frequency(0).unwrap();

        assert_eq!(freq, 4000000); // Max frequency
    }

    #[test]
    fn test_invalid_frequency() {
        let mut manager = CpufreqManager::new(2, vec![800000, 1600000, 2400000, 3200000, 4000000]);

        let result = manager.set_frequency(0, 5000000);
        assert!(result.is_err());
    }
}
