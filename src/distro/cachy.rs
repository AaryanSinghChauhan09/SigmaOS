// SigmaOS CachyOS Compatibility & Performance Suite (CachyOS Parity)
// Implements x86-64-v3/v4 Microarchitecture detection, BORE CPU Scheduler Governor, and CachyOS Kernel Variant Selector.

#![no_std]

#[cfg(test)]
extern crate std;

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[cfg(not(test))]
use crate::klib::HashMap;

#[cfg(test)]
use std::collections::HashMap;

/// x86-64 Microarchitecture Level (CachyOS / Arch Linux parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MicroArchLevel {
    V1, // Generic x86-64
    V2, // CMPXCHG16B, LAHF-SAHF, POPCNT, SSE3, SSSE3, SSE4.1, SSE4.2
    V3, // AVX, AVX2, BMI1, BMI2, F16C, FMA, LZCNT, MOVBE, OSXSAVE
    V4, // AVX512F, AVX512BW, AVX512CD, AVX512DQ, AVX512VL
}

/// CachyOS Custom Kernel Variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CachyKernelVariant {
    CachyBore, // Burst-Oriented Response Enhancer scheduler
    CachyLto,  // Clang Full Link-Time Optimization
    CachyRt,   // Real-time PREEMPT_RT
    CachyBase, // Standard CachyOS kernel
}

/// CPU Hardware Capability Flags for Microarchitecture Detection
#[derive(Debug, Clone, Copy)]
pub struct CpuCapabilities {
    pub has_sse4_2: bool,
    pub has_avx2: bool,
    pub has_fma: bool,
    pub has_bmi2: bool,
    pub has_avx512f: bool,
    pub has_avx512bw: bool,
}

impl CpuCapabilities {
    pub fn new_x86_64_v3_capable() -> Self {
        Self {
            has_sse4_2: true,
            has_avx2: true,
            has_fma: true,
            has_bmi2: true,
            has_avx512f: false,
            has_avx512bw: false,
        }
    }

    pub fn new_x86_64_v4_capable() -> Self {
        Self {
            has_sse4_2: true,
            has_avx2: true,
            has_fma: true,
            has_bmi2: true,
            has_avx512f: true,
            has_avx512bw: true,
        }
    }

    /// Evaluates exact microarchitecture level based on detected instruction extensions
    pub fn detect_microarch_level(&self) -> MicroArchLevel {
        if self.has_avx512f && self.has_avx512bw {
            MicroArchLevel::V4
        } else if self.has_avx2 && self.has_fma && self.has_bmi2 {
            MicroArchLevel::V3
        } else if self.has_sse4_2 {
            MicroArchLevel::V2
        } else {
            MicroArchLevel::V1
        }
    }
}

/// BORE (Burst-Oriented Response Enhancer) CPU Scheduler Governor (CachyOS parity)
pub struct BoreSchedulerGovernor {
    pub burst_score_weight: u32,
    pub interactive_latency_ns: u64,
}

impl BoreSchedulerGovernor {
    pub fn new() -> Self {
        Self {
            burst_score_weight: 128,
            interactive_latency_ns: 2_000_000, // 2ms ultra-low latency for desktop interaction
        }
    }

    /// Calculates dynamic task burst score and adjusts time-slice allocation
    pub fn calculate_task_timeslice_ns(
        &self,
        task_burst_count: u32,
        base_timeslice_ns: u64,
    ) -> u64 {
        if task_burst_count < 10 {
            // High burst interactive task (mouse/UI/game input) -> grant low latency slice
            self.interactive_latency_ns
        } else {
            // Compute-bound background task -> scale timeslice up to prevent context switch thrashing
            base_timeslice_ns + (task_burst_count as u64 * 500_000)
        }
    }
}

impl Default for BoreSchedulerGovernor {
    fn default() -> Self {
        Self::new()
    }
}

/// CachyOS Microarchitecture-optimized package repository manager
pub struct CachyPackageRepo {
    pub active_level: MicroArchLevel,
    pub active_kernel: CachyKernelVariant,
    pub repository_urls: HashMap<MicroArchLevel, String>,
}

impl CachyPackageRepo {
    pub fn new(detected_caps: CpuCapabilities) -> Self {
        let level = detected_caps.detect_microarch_level();
        let mut repos = HashMap::new();
        repos.insert(
            MicroArchLevel::V1,
            "https://mirror.cachyos.org/repo/x86_64".to_string(),
        );
        repos.insert(
            MicroArchLevel::V3,
            "https://mirror.cachyos.org/repo/x86_64_v3".to_string(),
        );
        repos.insert(
            MicroArchLevel::V4,
            "https://mirror.cachyos.org/repo/x86_64_v4".to_string(),
        );

        Self {
            active_level: level,
            active_kernel: CachyKernelVariant::CachyBore,
            repository_urls: repos,
        }
    }

    pub fn get_active_repo_url(&self) -> String {
        self.repository_urls
            .get(&self.active_level)
            .cloned()
            .unwrap_or_else(|| "https://mirror.cachyos.org/repo/x86_64".to_string())
    }

    pub fn switch_kernel_variant(&mut self, variant: CachyKernelVariant) {
        self.active_kernel = variant;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microarch_detection() {
        let caps_v3 = CpuCapabilities::new_x86_64_v3_capable();
        assert_eq!(caps_v3.detect_microarch_level(), MicroArchLevel::V3);

        let caps_v4 = CpuCapabilities::new_x86_64_v4_capable();
        assert_eq!(caps_v4.detect_microarch_level(), MicroArchLevel::V4);
    }

    #[test]
    fn test_bore_scheduler_governor() {
        let bore = BoreSchedulerGovernor::new();

        // Interactive UI task (low burst count) gets ultra-low 2ms latency slice
        let interactive_slice = bore.calculate_task_timeslice_ns(3, 10_000_000);
        assert_eq!(interactive_slice, 2_000_000);

        // Heavy background compute task gets larger timeslice
        let compute_slice = bore.calculate_task_timeslice_ns(50, 10_000_000);
        assert_eq!(compute_slice, 35_000_000);
    }

    #[test]
    fn test_cachy_package_repo_selection() {
        let caps = CpuCapabilities::new_x86_64_v3_capable();
        let mut repo = CachyPackageRepo::new(caps);

        assert_eq!(repo.active_level, MicroArchLevel::V3);
        assert_eq!(
            repo.get_active_repo_url(),
            "https://mirror.cachyos.org/repo/x86_64_v3"
        );

        repo.switch_kernel_variant(CachyKernelVariant::CachyLto);
        assert_eq!(repo.active_kernel, CachyKernelVariant::CachyLto);
    }
}
