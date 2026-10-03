// SigmaOS CachyOS Compatibility & Performance Suite (CachyOS Parity)
// Implements x86-64-v3/v4 Microarchitecture detection, BORE CPU Scheduler Governor, CachyOS Kernel Variant Selector,
// Ananicy Process Rules, UKSM Deduplication, Gamescope/Proton Latency Governor, and Kernel Sysctl Tuning.

#[cfg(test_disabled)]
use std::format;
use std::string::String;
use std::vec::Vec;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap as HashMap;

#[cfg(any(feature = "standalone_test", test))]
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

/// CPU Energy Performance Preference (EPP) for CachyOS / auto-cpufreq
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnergyPerformancePreference {
    Performance,
    BalancePerformance,
    BalancePower,
    Power,
}

/// CachyOS Auto-CpuFreq Power Governor Engine
/// Dynamically adjusts CPU scaling governors and EPP parameters based on battery vs AC power states.
pub struct CachyOsAutoFreqEngine {
    pub is_on_ac_power: bool,
    pub current_governor: String,
    pub current_epp: EnergyPerformancePreference,
    pub turbo_boost_enabled: bool,
}

impl CachyOsAutoFreqEngine {
    pub fn new() -> Self {
        Self {
            is_on_ac_power: true,
            current_governor: String::from("performance"),
            current_epp: EnergyPerformancePreference::Performance,
            turbo_boost_enabled: true,
        }
    }

    pub fn set_power_state(&mut self, on_ac: bool) {
        self.is_on_ac_power = on_ac;
        if on_ac {
            self.current_governor = String::from("performance");
            self.current_epp = EnergyPerformancePreference::Performance;
            self.turbo_boost_enabled = true;
        } else {
            self.current_governor = String::from("powersave");
            self.current_epp = EnergyPerformancePreference::BalancePower;
            self.turbo_boost_enabled = false;
        }
    }
}

impl Default for CachyOsAutoFreqEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// CachyOS Hardware Detection & Microarch Driver Installer (chwd parity)
pub struct CachyOsChWDHardwareEngine {
    pub microarch_level: MicroArchLevel,
    pub installed_gpu_driver: String,
    pub optimized_mesa_v3: bool,
}

impl CachyOsChWDHardwareEngine {
    pub fn new(caps: CpuCapabilities) -> Self {
        let level = caps.detect_microarch_level();
        let mesa_v3 = level >= MicroArchLevel::V3;
        Self {
            microarch_level: level,
            installed_gpu_driver: String::from("chwd-video-linux"),
            optimized_mesa_v3: mesa_v3,
        }
    }

    pub fn auto_configure_chwd_drivers(&mut self, is_nvidia: bool) -> String {
        if is_nvidia {
            if self.microarch_level >= MicroArchLevel::V3 {
                self.installed_gpu_driver = String::from("chwd-nvidia-v3-dkms");
            } else {
                self.installed_gpu_driver = String::from("chwd-nvidia-dkms");
            }
        } else if self.microarch_level >= MicroArchLevel::V3 {
            self.installed_gpu_driver = String::from("chwd-mesa-v3-amdgpu-intel");
        } else {
            self.installed_gpu_driver = String::from("chwd-mesa-generic");
        }
        self.installed_gpu_driver.clone()
    }
}

/// Ananicy-cpp process priority rule
#[derive(Debug, Clone)]
pub struct AnanicyRule {
    pub name: String,
    pub nice: i8,
    pub ioclass: u8, // 1: Realtime, 2: BestEffort, 3: Idle
    pub ionice: u8,  // 0..7
    pub sched_policy: String,
}

/// CachyOS Ananicy-cpp Rule Engine for Auto-Nicing & Process Scheduling
pub struct CachyOsAnanicyPriorityEngine {
    pub rules: Vec<AnanicyRule>,
    pub default_nice: i8,
}

impl CachyOsAnanicyPriorityEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            rules: Vec::new(),
            default_nice: 0,
        };
        engine.load_cachyos_defaults();
        engine
    }

    pub fn load_cachyos_defaults(&mut self) {
        self.rules.push(AnanicyRule {
            name: String::from("gamescope"),
            nice: -15,
            ioclass: 1,
            ionice: 0,
            sched_policy: String::from("SCHED_FIFO"),
        });
        self.rules.push(AnanicyRule {
            name: String::from("steam"),
            nice: -5,
            ioclass: 2,
            ionice: 1,
            sched_policy: String::from("SCHED_OTHER"),
        });
        self.rules.push(AnanicyRule {
            name: String::from("obs"),
            nice: -8,
            ioclass: 2,
            ionice: 0,
            sched_policy: String::from("SCHED_OTHER"),
        });
    }

    pub fn evaluate_process(&self, process_name: &str) -> Option<AnanicyRule> {
        for rule in &self.rules {
            if rule.name == process_name {
                return Some(rule.clone());
            }
        }
        None
    }
}

impl Default for CachyOsAnanicyPriorityEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Ultra-fast Kernel Samepage Merging (UKSM) Memory Deduplication Governor
pub struct CachyOsUksmMemoryDeduplicationEngine {
    pub pages_scanned: u64,
    pub pages_shared: u64,
    pub cpu_use_limit_pct: u8,
    pub sleep_millisecs: u32,
    pub is_enabled: bool,
}

impl CachyOsUksmMemoryDeduplicationEngine {
    pub fn new() -> Self {
        Self {
            pages_scanned: 0,
            pages_shared: 0,
            cpu_use_limit_pct: 20,
            sleep_millisecs: 20,
            is_enabled: true,
        }
    }

    pub fn run_deduplication_cycle(&mut self, candidate_pages: u64) -> u64 {
        if !self.is_enabled {
            return 0;
        }
        self.pages_scanned += candidate_pages;
        let merged = candidate_pages / 4; // Simulated 25% page merging ratio
        self.pages_shared += merged;
        merged
    }
}

impl Default for CachyOsUksmMemoryDeduplicationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// CachyOS Gamescope & Proton Game Latency Governor
pub struct CachyOsGamescopeProtonEngine {
    pub gamescope_fps_limit: u32,
    pub fsr_enabled: bool,
    pub low_latency_mode: bool,
    pub proton_wine_sync: String, // esync, fsync, ntsync
}

impl CachyOsGamescopeProtonEngine {
    pub fn new() -> Self {
        Self {
            gamescope_fps_limit: 144,
            fsr_enabled: true,
            low_latency_mode: true,
            proton_wine_sync: String::from("ntsync"),
        }
    }

    pub fn configure_game_overlay(&mut self, fps: u32, fsr: bool, sync_type: &str) {
        self.gamescope_fps_limit = fps;
        self.fsr_enabled = fsr;
        self.proton_wine_sync = String::from(sync_type);
    }
}

impl Default for CachyOsGamescopeProtonEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// CachyOS Kernel Sysctl Manager Engine
pub struct CachyOsKernelManagerEngine {
    pub sysctl_settings: HashMap<String, String>,
}

impl CachyOsKernelManagerEngine {
    pub fn new() -> Self {
        let mut settings = HashMap::new();
        settings.insert(String::from("vm.max_map_count"), String::from("1048576"));
        settings.insert(String::from("vm.swappiness"), String::from("10"));
        settings.insert(String::from("vm.vfs_cache_pressure"), String::from("50"));
        settings.insert(
            String::from("kernel.sched_cfs_bandwidth_slice_us"),
            String::from("3000"),
        );
        Self {
            sysctl_settings: settings,
        }
    }

    pub fn get_sysctl(&self, key: &str) -> Option<&String> {
        self.sysctl_settings.get(key)
    }
}

impl Default for CachyOsKernelManagerEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// CachyOS Master System Suite
pub struct CachyOsMasterSystemSuite {
    pub repo: CachyPackageRepo,
    pub bore_scheduler: BoreSchedulerGovernor,
    pub auto_freq: CachyOsAutoFreqEngine,
    pub chwd: CachyOsChWDHardwareEngine,
    pub ananicy: CachyOsAnanicyPriorityEngine,
    pub uksm: CachyOsUksmMemoryDeduplicationEngine,
    pub gamescope: CachyOsGamescopeProtonEngine,
    pub kernel_mgr: CachyOsKernelManagerEngine,
}

impl CachyOsMasterSystemSuite {
    pub fn new(caps: CpuCapabilities) -> Self {
        Self {
            repo: CachyPackageRepo::new(caps),
            bore_scheduler: BoreSchedulerGovernor::new(),
            auto_freq: CachyOsAutoFreqEngine::new(),
            chwd: CachyOsChWDHardwareEngine::new(caps),
            ananicy: CachyOsAnanicyPriorityEngine::new(),
            uksm: CachyOsUksmMemoryDeduplicationEngine::new(),
            gamescope: CachyOsGamescopeProtonEngine::new(),
            kernel_mgr: CachyOsKernelManagerEngine::new(),
        }
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

    #[test]
    fn test_cachy_autofreq_and_chwd() {
        let mut auto_freq = CachyOsAutoFreqEngine::new();
        assert!(auto_freq.is_on_ac_power);
        assert_eq!(auto_freq.current_governor, "performance");

        auto_freq.set_power_state(false); // Battery mode
        assert!(!auto_freq.is_on_ac_power);
        assert_eq!(auto_freq.current_governor, "powersave");
        assert_eq!(
            auto_freq.current_epp,
            EnergyPerformancePreference::BalancePower
        );

        let caps_v3 = CpuCapabilities::new_x86_64_v3_capable();
        let mut chwd = CachyOsChWDHardwareEngine::new(caps_v3);
        assert!(chwd.optimized_mesa_v3);

        let driver = chwd.auto_configure_chwd_drivers(true);
        assert_eq!(driver, "chwd-nvidia-v3-dkms");
    }

    #[test]
    fn test_cachyos_advancements_suite() {
        let caps = CpuCapabilities::new_x86_64_v3_capable();
        let suite = CachyOsMasterSystemSuite::new(caps);

        // Ananicy check
        let rule = suite.ananicy.evaluate_process("gamescope").unwrap();
        assert_eq!(rule.nice, -15);
        assert_eq!(rule.sched_policy, "SCHED_FIFO");

        // UKSM check
        let mut uksm = suite.uksm;
        let merged = uksm.run_deduplication_cycle(100);
        assert_eq!(merged, 25);

        // Gamescope check
        assert_eq!(suite.gamescope.gamescope_fps_limit, 144);
        assert_eq!(suite.gamescope.proton_wine_sync, "ntsync");

        // Kernel mgr check
        assert_eq!(
            suite.kernel_mgr.get_sysctl("vm.max_map_count").unwrap(),
            "1048576"
        );
    }
}
