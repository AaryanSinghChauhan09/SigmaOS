// SigmaOS Schedutil CPUfreq Governor
// Adjusts CPU frequency and hardware P-states directly based on scheduler run-queue utilization.

#![allow(dead_code)]
extern crate alloc;

use alloc::vec::Vec;

/// Frequency scaling governor mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreqGovernorMode {
    Performance,
    Powersave,
    Ondemand,
    Schedutil,
}

/// CPU Core Frequency Policy & Operating Performance Points (OPP)
#[derive(Debug, Clone)]
pub struct CpuFreqPolicy {
    pub cpu_id: u32,
    pub min_freq_mhz: u32,
    pub max_freq_mhz: u32,
    pub current_freq_mhz: u32,
    pub available_frequencies_mhz: Vec<u32>,
    pub governor: FreqGovernorMode,
    pub rate_limit_us: u64,
    pub last_freq_update_time_us: u64,
}

/// Schedutil Governor Engine
#[derive(Debug, Clone)]
pub struct SchedutilGovernor {
    pub policies: Vec<CpuFreqPolicy>,
}

impl SchedutilGovernor {
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
        }
    }

    /// Register a CPU policy
    pub fn register_cpu(&mut self, cpu_id: u32, min_mhz: u32, max_mhz: u32, opps: &[u32]) {
        self.policies.push(CpuFreqPolicy {
            cpu_id,
            min_freq_mhz: min_mhz,
            max_freq_mhz: max_mhz,
            current_freq_mhz: min_mhz,
            available_frequencies_mhz: opps.to_vec(),
            governor: FreqGovernorMode::Schedutil,
            rate_limit_us: 10_000, // 10ms default rate limit
            last_freq_update_time_us: 0,
        });
    }

    /// Calculate next frequency target using the Linux schedutil formula:
    /// target = 1.25 * max_freq * (util / max_util)
    pub fn compute_target_freq(policy: &CpuFreqPolicy, cpu_util: u64, max_util: u64) -> u32 {
        if max_util == 0 {
            return policy.min_freq_mhz;
        }

        // 1.25 headroom factor: (5 * util * max_freq) / (4 * max_util)
        let raw_target = (5 * cpu_util * (policy.max_freq_mhz as u64)) / (4 * max_util);
        let clamped = (raw_target as u32)
            .max(policy.min_freq_mhz)
            .min(policy.max_freq_mhz);

        // Select lowest available frequency OPP >= clamped
        let mut best_opp = policy.max_freq_mhz;
        for &opp in &policy.available_frequencies_mhz {
            if opp >= clamped && opp <= best_opp {
                best_opp = opp;
            }
        }
        best_opp
    }

    /// Update CPU frequency based on incoming scheduler run-queue load
    pub fn update_cpu_util(
        &mut self,
        cpu_id: u32,
        util: u64,
        max_util: u64,
        now_us: u64,
    ) -> Option<u32> {
        let policy = self.policies.iter_mut().find(|p| p.cpu_id == cpu_id)?;

        // Enforce rate limit to avoid frequent MSR writes
        if now_us.saturating_sub(policy.last_freq_update_time_us) < policy.rate_limit_us {
            return None;
        }

        let target_mhz = match policy.governor {
            FreqGovernorMode::Performance => policy.max_freq_mhz,
            FreqGovernorMode::Powersave => policy.min_freq_mhz,
            FreqGovernorMode::Schedutil | FreqGovernorMode::Ondemand => {
                Self::compute_target_freq(policy, util, max_util)
            }
        };

        if target_mhz != policy.current_freq_mhz {
            policy.current_freq_mhz = target_mhz;
            policy.last_freq_update_time_us = now_us;
            Some(target_mhz)
        } else {
            None
        }
    }
}

impl Default for SchedutilGovernor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedutil_frequency_scaling() {
        let mut gov = SchedutilGovernor::new();
        let opps = [800, 1200, 1800, 2400, 3200];
        gov.register_cpu(0, 800, 3200, &opps);

        // 20% load: (5 * 200 * 3200) / (4 * 1024) = ~781 -> clamped to min 800
        let freq_low = SchedutilGovernor::compute_target_freq(&gov.policies[0], 200, 1024);
        assert_eq!(freq_low, 800);

        // 80% load: 1.25 * 3200 * 0.8 = 3200 -> max 3200
        let freq_high = SchedutilGovernor::compute_target_freq(&gov.policies[0], 820, 1024);
        assert_eq!(freq_high, 3200);

        // Dynamic update check with rate limiting
        assert_eq!(gov.update_cpu_util(0, 820, 1024, 20_000), Some(3200));
        // Under rate limit: should return None
        assert_eq!(gov.update_cpu_util(0, 200, 1024, 25_000), None);
    }
}
