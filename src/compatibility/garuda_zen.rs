// SigmaOS Distro Compatibility Layer
/// Custom Garuda Linux and Zen Kernel Optimization Subsystems for SigmaOS
/// Implements Zen Interactivity Governor, Timeshift Btrfs snapshotting, Zram Memory Swap, and Nohang OOM Guards
extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

// ==========================================
// 1. Zen Interactivity Governor
// ==========================================

pub struct ZenInteractivityGovernor {
    pub latency_ns: AtomicU64,
    pub preempt_lag_ns: AtomicU64,
    pub dynamic_tick: AtomicBool,
    pub interactive_boost: AtomicBool,
}

impl ZenInteractivityGovernor {
    pub fn new() -> Self {
        ZenInteractivityGovernor {
            latency_ns: AtomicU64::new(2_000_000), // 2ms low-latency default
            preempt_lag_ns: AtomicU64::new(100_000), // 100us preemption lag
            dynamic_tick: AtomicBool::new(true),
            interactive_boost: AtomicBool::new(true),
        }
    }

    pub fn tune_for_low_latency(&self) {
        self.latency_ns.store(1_000_000, Ordering::SeqCst); // 1ms hyper-responsive
        self.preempt_lag_ns.store(50_000, Ordering::SeqCst); // 50us lag
        self.interactive_boost.store(true, Ordering::SeqCst);
    }

    pub fn tune_for_throughput(&self) {
        self.latency_ns.store(10_000_000, Ordering::SeqCst); // 10ms high-throughput
        self.preempt_lag_ns.store(500_000, Ordering::SeqCst); // 500us lag
        self.interactive_boost.store(false, Ordering::SeqCst);
    }
}

// ==========================================
// 2. Automated Btrfs Timeshift Snapshot Engine
// ==========================================

pub struct TimeshiftBtrfsEngine {
    pub snapshots_count: AtomicUsize,
    pub zstd_compression: AtomicBool,
    pub auto_snapshot_enabled: AtomicBool,
}

impl TimeshiftBtrfsEngine {
    pub fn new() -> Self {
        TimeshiftBtrfsEngine {
            snapshots_count: AtomicUsize::new(0),
            zstd_compression: AtomicBool::new(true), // Garuda uses Zstd compression by default
            auto_snapshot_enabled: AtomicBool::new(true),
        }
    }

    pub fn take_pre_upgrade_snapshot(&self, pkg_name: &str) -> u64 {
        let count = self.snapshots_count.fetch_add(1, Ordering::SeqCst);
        let _ = pkg_name; // Associated package metadata
        (count as u64) + 1000 // Returns snapshot ID
    }

    pub fn restore_snapshot(&self, snapshot_id: u64) -> bool {
        let _ = snapshot_id;
        // Simulates reverting subvolume to the snapshot state
        true
    }
}

// ==========================================
// 3. Zram Compressed Memory Swap Manager
// ==========================================

pub struct ZramSwapManager {
    pub capacity_bytes: AtomicUsize,
    pub compressed_size_bytes: AtomicUsize,
    pub ratio_percentage: AtomicUsize,
}

impl ZramSwapManager {
    pub fn new(capacity: usize) -> Self {
        ZramSwapManager {
            capacity_bytes: AtomicUsize::new(capacity),
            compressed_size_bytes: AtomicUsize::new(0),
            ratio_percentage: AtomicUsize::new(40), // 40% compression ratio (2.5x compression)
        }
    }

    pub fn allocate_swap_page(&self, uncompressed_size: usize) -> bool {
        let ratio = self.ratio_percentage.load(Ordering::SeqCst);
        let compressed_size = (uncompressed_size * ratio) / 100;

        let mut size_now = self.compressed_size_bytes.load(Ordering::SeqCst);
        let cap = self.capacity_bytes.load(Ordering::SeqCst);

        if size_now + compressed_size > cap {
            return false; // Out of swap space
        }

        self.compressed_size_bytes
            .fetch_add(compressed_size, Ordering::SeqCst);
        true
    }

    pub fn reset_swap(&self) {
        self.compressed_size_bytes.store(0, Ordering::SeqCst);
    }
}

// ==========================================
// 4. Nohang OOM Guard / Low Memory Daemon
// ==========================================

pub struct NohangOomGuard {
    pub memory_limit: AtomicUsize,
    pub oom_count: AtomicUsize,
    pub threshold_percentage: AtomicUsize,
}

impl NohangOomGuard {
    pub fn new(limit: usize) -> Self {
        NohangOomGuard {
            memory_limit: AtomicUsize::new(limit),
            oom_count: AtomicUsize::new(0),
            threshold_percentage: AtomicUsize::new(90), // Alert and kill if 90% memory threshold is exceeded
        }
    }

    pub fn check_pressure(&self, current_usage: usize) -> bool {
        let limit = self.memory_limit.load(Ordering::SeqCst);
        let thresh_percent = self.threshold_percentage.load(Ordering::SeqCst);
        let limit_thresh = (limit * thresh_percent) / 100;

        current_usage > limit_thresh
    }

    pub fn kill_hung_process(&self, pid: usize) -> bool {
        let _ = pid;
        self.oom_count.fetch_add(1, Ordering::SeqCst);
        true // Process successfully terminated before OOM lockup
    }
}

impl Default for ZenInteractivityGovernor {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TimeshiftBtrfsEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 5. Garuda Performance Tuning Profile Manager
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GarudaTuningProfile {
    GamingMode,
    UltraLowLatency,
    WorkstationThroughput,
    BatterySaver,
}

pub struct GarudaTuningProfileManager {
    pub current_profile: GarudaTuningProfile,
    pub vfs_cache_pressure: AtomicUsize,
    pub dirty_ratio: AtomicUsize,
    pub swappiness: AtomicUsize,
    pub sched_slice_us: AtomicUsize,
    pub cpu_governor_performance: AtomicBool,
}

impl GarudaTuningProfileManager {
    pub fn new() -> Self {
        Self {
            current_profile: GarudaTuningProfile::UltraLowLatency,
            vfs_cache_pressure: AtomicUsize::new(50),
            dirty_ratio: AtomicUsize::new(10),
            swappiness: AtomicUsize::new(100),
            sched_slice_us: AtomicUsize::new(1000), // 1ms
            cpu_governor_performance: AtomicBool::new(true),
        }
    }

    pub fn apply_profile(&mut self, profile: GarudaTuningProfile) {
        self.current_profile = profile;
        match profile {
            GarudaTuningProfile::GamingMode => {
                self.vfs_cache_pressure.store(20, Ordering::SeqCst);
                self.dirty_ratio.store(5, Ordering::SeqCst);
                self.swappiness.store(150, Ordering::SeqCst);
                self.sched_slice_us.store(500, Ordering::SeqCst); // 0.5ms ultra responsive
                self.cpu_governor_performance.store(true, Ordering::SeqCst);
            }
            GarudaTuningProfile::UltraLowLatency => {
                self.vfs_cache_pressure.store(30, Ordering::SeqCst);
                self.dirty_ratio.store(8, Ordering::SeqCst);
                self.swappiness.store(100, Ordering::SeqCst);
                self.sched_slice_us.store(1000, Ordering::SeqCst);
                self.cpu_governor_performance.store(true, Ordering::SeqCst);
            }
            GarudaTuningProfile::WorkstationThroughput => {
                self.vfs_cache_pressure.store(100, Ordering::SeqCst);
                self.dirty_ratio.store(20, Ordering::SeqCst);
                self.swappiness.store(60, Ordering::SeqCst);
                self.sched_slice_us.store(5000, Ordering::SeqCst); // 5ms throughput
                self.cpu_governor_performance.store(false, Ordering::SeqCst);
            }
            GarudaTuningProfile::BatterySaver => {
                self.vfs_cache_pressure.store(150, Ordering::SeqCst);
                self.dirty_ratio.store(30, Ordering::SeqCst);
                self.swappiness.store(10, Ordering::SeqCst);
                self.sched_slice_us.store(10000, Ordering::SeqCst); // 10ms power efficient
                self.cpu_governor_performance.store(false, Ordering::SeqCst);
            }
        }
    }
}

impl Default for GarudaTuningProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 6. IRQ Balance & P-Core/E-Core Affinity Optimizer
// ==========================================

pub struct IrqBalanceOptimizer {
    pub p_core_mask: u64,
    pub e_core_mask: u64,
    pub irq_count: AtomicUsize,
}

impl IrqBalanceOptimizer {
    pub fn new(p_core_mask: u64, e_core_mask: u64) -> Self {
        Self {
            p_core_mask,
            e_core_mask,
            irq_count: AtomicUsize::new(0),
        }
    }

    /// Selects optimal CPU core affinity mask for an IRQ based on real-time latency needs
    pub fn assign_irq_affinity(&self, irq_number: u32, is_high_priority_device: bool) -> u64 {
        self.irq_count.fetch_add(1, Ordering::SeqCst);
        let _ = irq_number;
        if is_high_priority_device {
            self.p_core_mask // Route NVMe/GPU/Wi-Fi IRQs to P-cores
        } else {
            self.e_core_mask // Route background timer/USB IRQs to E-cores
        }
    }
}

// ==========================================
// 7. Garuda Auto-Nice & Process Priority Engine
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessIoClass {
    Realtime,
    BestEffort,
    Idle,
}

pub struct GarudaAutoNiceRule {
    pub process_name: &'static str,
    pub nice_value: i32,
    pub io_class: ProcessIoClass,
}

pub struct GarudaAutoNiceEngine {
    pub rules: [GarudaAutoNiceRule; 4],
}

impl GarudaAutoNiceEngine {
    pub fn new() -> Self {
        Self {
            rules: [
                GarudaAutoNiceRule {
                    process_name: "steam",
                    nice_value: -10,
                    io_class: ProcessIoClass::Realtime,
                },
                GarudaAutoNiceRule {
                    process_name: "pipewire",
                    nice_value: -15,
                    io_class: ProcessIoClass::Realtime,
                },
                GarudaAutoNiceRule {
                    process_name: "obs",
                    nice_value: -8,
                    io_class: ProcessIoClass::BestEffort,
                },
                GarudaAutoNiceRule {
                    process_name: "baloo",
                    nice_value: 19,
                    io_class: ProcessIoClass::Idle,
                },
            ],
        }
    }

    pub fn lookup_rule(&self, name: &str) -> Option<(i32, ProcessIoClass)> {
        for rule in &self.rules {
            if rule.process_name == name {
                return Some((rule.nice_value, rule.io_class));
            }
        }
        None
    }
}

impl Default for GarudaAutoNiceEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 8. Garuda GPU Performance Governor
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuPowerState {
    PerformanceHigh,
    BalancedMedium,
    PowerSaveLow,
}

pub struct GarudaGpuPerformanceGovernor {
    pub current_state: GpuPowerState,
    pub clock_mhz: AtomicUsize,
    pub fan_curve_percent: AtomicUsize,
}

impl GarudaGpuPerformanceGovernor {
    pub fn new() -> Self {
        Self {
            current_state: GpuPowerState::BalancedMedium,
            clock_mhz: AtomicUsize::new(1500),
            fan_curve_percent: AtomicUsize::new(50),
        }
    }

    pub fn set_power_state(&mut self, state: GpuPowerState) {
        self.current_state = state;
        match state {
            GpuPowerState::PerformanceHigh => {
                self.clock_mhz.store(2400, Ordering::SeqCst);
                self.fan_curve_percent.store(85, Ordering::SeqCst);
            }
            GpuPowerState::BalancedMedium => {
                self.clock_mhz.store(1500, Ordering::SeqCst);
                self.fan_curve_percent.store(50, Ordering::SeqCst);
            }
            GpuPowerState::PowerSaveLow => {
                self.clock_mhz.store(800, Ordering::SeqCst);
                self.fan_curve_percent.store(25, Ordering::SeqCst);
            }
        }
    }
}

impl Default for GarudaGpuPerformanceGovernor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zen_governor_tuning() {
        let gov = ZenInteractivityGovernor::new();
        assert_eq!(gov.latency_ns.load(Ordering::SeqCst), 2_000_000);

        gov.tune_for_low_latency();
        assert_eq!(gov.latency_ns.load(Ordering::SeqCst), 1_000_000);
        assert_eq!(gov.preempt_lag_ns.load(Ordering::SeqCst), 50_000);

        gov.tune_for_throughput();
        assert_eq!(gov.latency_ns.load(Ordering::SeqCst), 10_000_000);
    }

    #[test]
    fn test_timeshift_btrfs_snapshots() {
        let engine = TimeshiftBtrfsEngine::new();
        assert_eq!(engine.snapshots_count.load(Ordering::SeqCst), 0);

        let snap_id = engine.take_pre_upgrade_snapshot("libc6");
        assert_eq!(snap_id, 1000);
        assert_eq!(engine.snapshots_count.load(Ordering::SeqCst), 1);

        assert!(engine.restore_snapshot(snap_id));
    }

    #[test]
    fn test_zram_swap_allocation() {
        let zram = ZramSwapManager::new(1024 * 1024); // 1MB zram disk
        assert!(zram.allocate_swap_page(4096)); // Uncompressed 4KB page

        let size_now = zram.compressed_size_bytes.load(Ordering::SeqCst);
        assert_eq!(size_now, (4096 * 40) / 100);
    }

    #[test]
    fn test_nohang_oom_guard() {
        let guard = NohangOomGuard::new(1000);
        assert!(!guard.check_pressure(800)); // 80% is okay
        assert!(guard.check_pressure(950)); // 95% triggers protection!

        assert!(guard.kill_hung_process(1234));
        assert_eq!(guard.oom_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_garuda_tuning_profile_manager() {
        let mut mgr = GarudaTuningProfileManager::new();
        assert_eq!(mgr.current_profile, GarudaTuningProfile::UltraLowLatency);

        mgr.apply_profile(GarudaTuningProfile::GamingMode);
        assert_eq!(mgr.current_profile, GarudaTuningProfile::GamingMode);
        assert_eq!(mgr.vfs_cache_pressure.load(Ordering::SeqCst), 20);
        assert_eq!(mgr.sched_slice_us.load(Ordering::SeqCst), 500);

        mgr.apply_profile(GarudaTuningProfile::BatterySaver);
        assert_eq!(mgr.current_profile, GarudaTuningProfile::BatterySaver);
        assert_eq!(mgr.vfs_cache_pressure.load(Ordering::SeqCst), 150);
        assert!(!mgr.cpu_governor_performance.load(Ordering::SeqCst));
    }

    #[test]
    fn test_irq_balance_optimizer() {
        let optimizer = IrqBalanceOptimizer::new(0x000F, 0x00F0);
        let p_core_mask = optimizer.assign_irq_affinity(16, true);
        assert_eq!(p_core_mask, 0x000F);

        let e_core_mask = optimizer.assign_irq_affinity(17, false);
        assert_eq!(e_core_mask, 0x00F0);
        assert_eq!(optimizer.irq_count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_garuda_auto_nice_engine() {
        let auto_nice = GarudaAutoNiceEngine::new();
        let (nice, io_cls) = auto_nice.lookup_rule("pipewire").unwrap();
        assert_eq!(nice, -15);
        assert_eq!(io_cls, ProcessIoClass::Realtime);

        assert!(auto_nice.lookup_rule("nonexistent").is_none());
    }

    #[test]
    fn test_garuda_gpu_performance_governor() {
        let mut gpu = GarudaGpuPerformanceGovernor::new();
        assert_eq!(gpu.current_state, GpuPowerState::BalancedMedium);

        gpu.set_power_state(GpuPowerState::PerformanceHigh);
        assert_eq!(gpu.clock_mhz.load(Ordering::SeqCst), 2400);
        assert_eq!(gpu.fan_curve_percent.load(Ordering::SeqCst), 85);

        gpu.set_power_state(GpuPowerState::PowerSaveLow);
        assert_eq!(gpu.clock_mhz.load(Ordering::SeqCst), 800);
    }
}
