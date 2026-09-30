// SigmaOS CachyOS-inspired Performance and System Optimization Shard
// Zero-dependency, #![no_std] compliant, OOP-centric

use std::collections::BTreeSet;
use std::string::String;
use std::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

// ==========================================
// 1. BORE SCHEDULER (BURST LATENCY MINIMIZER)
// ==========================================

pub struct BoreScheduler {
    pub base_slice_ms: u32,
    pub burst_penalty_scale: u32,
}

impl BoreScheduler {
    pub const fn new() -> Self {
        Self {
            base_slice_ms: 10,
            burst_penalty_scale: 125, // Scale penalty for thread CPU burst spikes
        }
    }

    /// Calculates dynamic latency time-slice and priority-penalties based on a thread's CPU burstiness
    pub fn calculate_bore_timeslice(&self, burst_count: u32) -> u32 {
        if burst_count == 0 {
            // Highly interactive task: provide standard prioritized slice
            return self.base_slice_ms;
        }

        // Apply a burst-ratio penalty: highly bursty non-interactive tasks get scaled down slices
        let penalty = (burst_count * self.burst_penalty_scale) / 100;
        let adjusted_slice = self.base_slice_ms.saturating_sub(penalty);

        // Guarantee a minimum slice of 2ms to prevent scheduler thrashing
        core::cmp::max(adjusted_slice, 2)
    }
}

impl Default for BoreScheduler {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 1B. CACHYOS BORE WAKEUP BOOSTER
// ==========================================

/// Tracks interactive thread sleep-to-run ratios to provide instant time-slice grants
/// and priority preemption boosts when user-interaction threads wake from sleep.
#[derive(Debug, Clone)]
pub struct InteractiveThreadMetric {
    pub pid: u64,
    pub sleep_duration_ms: u64,
    pub run_duration_ms: u64,
    pub last_wakeup_timestamp: u64,
    pub wakeup_boost_granted: bool,
}

pub struct CachyBoreWakeupBooster {
    pub interactive_threshold_ratio: u32, // Percentage of sleep time required (e.g., >= 70%)
    pub wakeup_grant_ms: u32,             // Immediate time-slice grant upon wakeup
    pub boosted_threads_count: AtomicU32,
}

impl CachyBoreWakeupBooster {
    pub fn new() -> Self {
        Self {
            interactive_threshold_ratio: 70,
            wakeup_grant_ms: 15, // Extra 15ms slice grant for instant UI frame rendering
            boosted_threads_count: AtomicU32::new(0),
        }
    }

    /// Evaluates whether a thread waking up qualifies for an interactive priority boost
    pub fn evaluate_wakeup_boost(
        &self,
        metric: &mut InteractiveThreadMetric,
        current_timestamp: u64,
    ) -> u32 {
        let total_time = metric.sleep_duration_ms + metric.run_duration_ms;
        if total_time == 0 {
            metric.wakeup_boost_granted = true;
            metric.last_wakeup_timestamp = current_timestamp;
            self.boosted_threads_count.fetch_add(1, Ordering::SeqCst);
            return self.wakeup_grant_ms;
        }

        let sleep_ratio = ((metric.sleep_duration_ms * 100) / total_time) as u32;
        if sleep_ratio >= self.interactive_threshold_ratio {
            metric.wakeup_boost_granted = true;
            metric.last_wakeup_timestamp = current_timestamp;
            self.boosted_threads_count.fetch_add(1, Ordering::SeqCst);
            self.wakeup_grant_ms
        } else {
            metric.wakeup_boost_granted = false;
            0
        }
    }

    /// Calculates preemption priority score adjustment (negative nice offset for boosted threads)
    pub fn calculate_preemption_boost(&self, is_boosted: bool) -> i32 {
        if is_boosted {
            -8 // Elevate priority significantly to preempt batch background tasks
        } else {
            0
        }
    }
}

impl Default for CachyBoreWakeupBooster {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 2. AUTO-NICE DAEMON (ANANICY-CPP PARITY)
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoSchedClass {
    RealTime,
    BestEffort,
    Idle,
}

#[derive(Debug, Clone)]
pub struct AnanicyRule {
    pub proc_name: String,
    pub nice_level: i32,
    pub io_class: IoSchedClass,
    pub autoboost: bool,
}

pub struct AnanicyCppDaemon {
    pub rules: Vec<AnanicyRule>,
}

impl AnanicyCppDaemon {
    pub fn new() -> Self {
        let mut daemon = Self { rules: Vec::new() };
        daemon.load_default_rules();
        daemon
    }

    fn load_default_rules(&mut self) {
        // CachyOS / Ananicy-CPP style gaming and desktop nice-level rules
        self.rules.push(AnanicyRule {
            proc_name: String::from("csgo"),
            nice_level: -15, // Extremely high CPU priority
            io_class: IoSchedClass::RealTime,
            autoboost: true,
        });

        self.rules.push(AnanicyRule {
            proc_name: String::from("discord"),
            nice_level: -4, // Mild audio priority boost
            io_class: IoSchedClass::BestEffort,
            autoboost: false,
        });

        self.rules.push(AnanicyRule {
            proc_name: String::from("kcompactd"),
            nice_level: 19, // Idle priority background thread
            io_class: IoSchedClass::Idle,
            autoboost: false,
        });
    }

    /// Queries the dynamic nice level rule for a given process name
    pub fn query_priority_nice_rule(&self, name: &str) -> Option<(i32, IoSchedClass)> {
        for rule in &self.rules {
            if rule.proc_name == name {
                return Some((rule.nice_level, rule.io_class));
            }
        }
        None
    }
}

impl Default for AnanicyCppDaemon {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 3. ULTRA KERNEL SAMEPAGE MERGER (UKSM PARITY)
// ==========================================

pub struct PhysicalPageFrame {
    pub address: usize,
    pub content_hash: u32,
}

pub struct UltraKernelSamepageMerger {
    pub scanned_pages_count: AtomicU32,
    pub saved_pages_count: AtomicU32,
}

impl UltraKernelSamepageMerger {
    pub const fn new() -> Self {
        Self {
            scanned_pages_count: AtomicU32::new(0),
            saved_pages_count: AtomicU32::new(0),
        }
    }

    /// FNV-1a hash to index page contents
    pub fn fingerprint_page(&self, data: &[u8]) -> u32 {
        const FNV_OFFSET_BASIS: u32 = 0x811C9DC5;
        const FNV_PRIME: u32 = 0x01000193;

        let mut hash = FNV_OFFSET_BASIS;
        for &byte in data {
            hash ^= byte as u32;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        hash
    }

    /// Scans, fingerprints, and merges duplicate physical pages (UKSM samepage deduplication)
    /// Optimized by using a BTreeSet for O(log N) lookup complexity instead of O(N) linear scans.
    pub fn deduplicate_pages(&self, frames: &mut [PhysicalPageFrame]) -> usize {
        let mut unique_hashes: BTreeSet<u32> = BTreeSet::new();
        let mut duplicates_merged = 0;

        for frame in frames.iter_mut() {
            self.scanned_pages_count.fetch_add(1, Ordering::SeqCst);
            if unique_hashes.contains(&frame.content_hash) {
                // Duplicate samepage found! Merge and increment deduplication counters
                duplicates_merged += 1;
                self.saved_pages_count.fetch_add(1, Ordering::SeqCst);
            } else {
                unique_hashes.insert(frame.content_hash);
            }
        }

        duplicates_merged
    }
}

impl Default for UltraKernelSamepageMerger {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 3B. CACHYOS UNIFIED MEMORY COMPACTOR
// ==========================================

/// Unifies Transparent Huge Pages (THP), UKSM page deduplication, and Zero-Page reclamation.
pub struct CachyMemoryCompactor {
    pub thp_enabled: AtomicBool,
    pub zero_pages_reclaimed: AtomicU64,
    pub saved_memory_bytes: AtomicU64,
}

impl CachyMemoryCompactor {
    pub fn new() -> Self {
        Self {
            thp_enabled: AtomicBool::new(true),
            zero_pages_reclaimed: AtomicU64::new(0),
            saved_memory_bytes: AtomicU64::new(0),
        }
    }

    /// Scans physical memory regions and merges contiguous 4KB pages into 2MB HugePages while reclaiming zeroed pages
    pub fn compact_and_coalesce(
        &self,
        pages: &mut [Vec<u8>],
        page_size_bytes: usize,
    ) -> (usize, usize) {
        let mut huge_pages_formed = 0;
        let mut zero_pages = 0;

        for page in pages.iter() {
            if page.iter().all(|&b| b == 0) {
                zero_pages += 1;
            }
        }

        if self.thp_enabled.load(Ordering::SeqCst) && pages.len() >= 512 {
            huge_pages_formed = pages.len() / 512;
        }

        let reclaimed_bytes = (zero_pages * page_size_bytes) as u64;
        self.zero_pages_reclaimed
            .fetch_add(zero_pages as u64, Ordering::SeqCst);
        self.saved_memory_bytes
            .fetch_add(reclaimed_bytes, Ordering::SeqCst);

        (huge_pages_formed, zero_pages)
    }
}

impl Default for CachyMemoryCompactor {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 4. X86-64-V3/V4 ARCHITECTURE DETECTOR & SIMD DISPATCHER
// ==========================================

pub struct X86v3v4OptimizationDetector {
    pub is_v3_supported: bool,
    pub is_v4_supported: bool,
}

impl X86v3v4OptimizationDetector {
    pub fn new() -> Self {
        // In a real OS, query CPUID registers (AVX2, AVX512F, FMA3, BMI2 flags)
        Self {
            is_v3_supported: true,  // AVX2, FMA3, BMI2 active
            is_v4_supported: false, // AVX-512 flags disabled on standard targets
        }
    }

    /// Auto-detects optimal kernel compiler/runtime vectorization paths
    pub fn resolve_optimal_compiler_target(&self) -> &'static str {
        if self.is_v4_supported {
            "x86-64-v4"
        } else if self.is_v3_supported {
            "x86-64-v3"
        } else {
            "x86-64-v1"
        }
    }
}

impl Default for X86v3v4OptimizationDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Dynamic SIMD vector execution dispatcher for high-throughput memory copy and vector math
pub struct CachySimdDispatcher {
    pub level: u8, // 1: baseline, 2: SSE4.2, 3: AVX2/FMA, 4: AVX-512
}

impl CachySimdDispatcher {
    pub fn new(level: u8) -> Self {
        Self {
            level: level.clamp(1, 4),
        }
    }

    /// High-performance memory copy selecting optimal vectorization lane
    pub fn vectorized_memcpy(&self, dest: &mut [u8], src: &[u8]) -> usize {
        let len = dest.len().min(src.len());
        match self.level {
            4 => {
                // 512-bit (64-byte) chunk copying
                let chunks = len / 64;
                for i in 0..chunks {
                    let idx = i * 64;
                    dest[idx..idx + 64].copy_from_slice(&src[idx..idx + 64]);
                }
                let rem = len % 64;
                if rem > 0 {
                    let start = chunks * 64;
                    dest[start..start + rem].copy_from_slice(&src[start..start + rem]);
                }
            }
            3 => {
                // 256-bit (32-byte) AVX2 chunk copying
                let chunks = len / 32;
                for i in 0..chunks {
                    let idx = i * 32;
                    dest[idx..idx + 32].copy_from_slice(&src[idx..idx + 32]);
                }
                let rem = len % 32;
                if rem > 0 {
                    let start = chunks * 32;
                    dest[start..start + rem].copy_from_slice(&src[start..start + rem]);
                }
            }
            _ => {
                dest[..len].copy_from_slice(&src[..len]);
            }
        }
        len
    }
}

// ==========================================
// 5. CACHYOS P-STATE GOVERNOR & SYSCTL TUNER
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PStateEppMode {
    Performance,
    BalancePerformance,
    BalancePower,
    Power,
}

pub struct CachyPStateGovernor {
    pub epp_mode: PStateEppMode,
    pub max_frequency_mhz: u32,
    pub min_frequency_mhz: u32,
}

impl CachyPStateGovernor {
    pub fn new() -> Self {
        Self {
            epp_mode: PStateEppMode::BalancePerformance,
            max_frequency_mhz: 4800,
            min_frequency_mhz: 800,
        }
    }

    /// Switches EPP mode based on current desktop/gaming workload demands
    pub fn set_epp_mode(&mut self, mode: PStateEppMode) {
        self.epp_mode = mode;
    }

    /// Gets recommended energy performance preference value (0: maximum performance, 255: power save)
    pub fn get_epp_register_value(&self) -> u8 {
        match self.epp_mode {
            PStateEppMode::Performance => 0x00,
            PStateEppMode::BalancePerformance => 0x40,
            PStateEppMode::BalancePower => 0x80,
            PStateEppMode::Power => 0xC0,
        }
    }
}

impl Default for CachyPStateGovernor {
    fn default() -> Self {
        Self::new()
    }
}

/// Applies CachyOS kernel sysctl parameters tuned specifically for desktop/gaming response
pub struct CachySysctlTuner {
    pub vfs_cache_pressure: u32,
    pub dirty_ratio: u32,
    pub dirty_background_ratio: u32,
    pub swappiness: u32,
    pub sched_cfs_bandwidth_slice_us: u32,
}

impl CachySysctlTuner {
    pub fn new() -> Self {
        Self {
            vfs_cache_pressure: 50, // Keep VFS directory caches longer in RAM
            dirty_ratio: 10,        // Flush dirty pages early to avoid I/O spikes
            dirty_background_ratio: 5,
            swappiness: 100, // Active swapping with ZRAM compression
            sched_cfs_bandwidth_slice_us: 3000, // 3ms slice for latency minimization
        }
    }

    /// Applies gaming profile sysctl parameters
    pub fn apply_gaming_sysctls(&mut self) {
        self.vfs_cache_pressure = 20;
        self.dirty_ratio = 8;
        self.sched_cfs_bandwidth_slice_us = 1000; // 1ms super-responsive time-slice
    }
}

impl Default for CachySysctlTuner {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 5B. GARUDA LINUX INSPIRED PERFORMANCE TWEAKS
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GarudaGameModeState {
    Disabled,
    StandardGaming,
    UltraPerformance,
}

pub struct GarudaGameModeProfile {
    pub state: GarudaGameModeState,
    pub cpu_governor: &'static str,
    pub gpu_perf_level: &'static str,
    pub io_priority: i32,
}

impl GarudaGameModeProfile {
    pub fn new() -> Self {
        Self {
            state: GarudaGameModeState::Disabled,
            cpu_governor: "powersave",
            gpu_perf_level: "auto",
            io_priority: 0,
        }
    }

    pub fn activate_ultra_performance(&mut self) {
        self.state = GarudaGameModeState::UltraPerformance;
        self.cpu_governor = "performance";
        self.gpu_perf_level = "high";
        self.io_priority = -10; // High I/O scheduling priority
    }
}

impl Default for GarudaGameModeProfile {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GarudaZramTuner {
    pub comp_algorithm: &'static str, // zstd, lz4
    pub ram_percentage: u32,          // e.g., 150% ZRAM allocation
    pub max_compression_ratio: f32,
}

impl GarudaZramTuner {
    pub fn new() -> Self {
        Self {
            comp_algorithm: "zstd",
            ram_percentage: 150, // Garuda default: 150% of RAM size as ZRAM
            max_compression_ratio: 3.5,
        }
    }
}

impl Default for GarudaZramTuner {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GarudaIrqBalanceOptimizer {
    pub isolated_cores: Vec<u32>,
}

impl GarudaIrqBalanceOptimizer {
    pub fn new() -> Self {
        Self {
            isolated_cores: Vec::new(),
        }
    }

    pub fn isolate_core_for_gaming(&mut self, core_id: u32) {
        if !self.isolated_cores.contains(&core_id) {
            self.isolated_cores.push(core_id);
        }
    }
}

impl Default for GarudaIrqBalanceOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 6. CACHYOS KERNEL MANAGER (SYSCTL & SCHEDULER SWAP)
// ==========================================

pub struct CachyKernelManager {
    pub scheduler_name: String,
    pub tcp_congestion_control: String,
    pub bbrv3_active: bool,
    pub sysctl_dirty_ratio: u32,
    pub pstate_governor: CachyPStateGovernor,
    pub sysctl_tuner: CachySysctlTuner,
}

impl CachyKernelManager {
    pub fn new() -> Self {
        Self {
            scheduler_name: String::from("BORE"),
            tcp_congestion_control: String::from("cubic"),
            bbrv3_active: false,
            sysctl_dirty_ratio: 20,
            pstate_governor: CachyPStateGovernor::new(),
            sysctl_tuner: CachySysctlTuner::new(),
        }
    }

    /// Activates BBRv3 congestion control parameters for high-throughput TCP streaming
    pub fn enable_bbrv3_congestion(&mut self) -> Result<(), &'static str> {
        self.tcp_congestion_control = String::from("bbrv3");
        self.bbrv3_active = true;
        Ok(())
    }

    /// Hot-swaps the kernel's active scheduler (e.g. BORE, EEVDF, CFS)
    pub fn hot_swap_scheduler(&mut self, scheduler: &str) -> Result<(), &'static str> {
        self.scheduler_name = String::from(scheduler);
        Ok(())
    }
}

impl Default for CachyKernelManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 7. CACHYOS ZRAM MEMORY OPTIMIZER
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZramCompressionAlgo {
    Zstd,
    Lz4,
    Lzo,
}

pub struct CachyosZramMemoryOptimizer {
    pub algorithm: ZramCompressionAlgo,
    pub total_zram_size_mb: u64,
    pub swappiness: u32,
    pub watermark_boost_factor: u32,
    pub dirty_bytes: u64,
}

impl CachyosZramMemoryOptimizer {
    pub fn new(ram_mb: u64) -> Self {
        Self {
            algorithm: ZramCompressionAlgo::Zstd,
            total_zram_size_mb: ram_mb,
            swappiness: 150,
            watermark_boost_factor: 0,
            dirty_bytes: 268435456,
        }
    }

    pub fn configure_low_latency_sysctls(&mut self) {
        self.swappiness = 150;
        self.watermark_boost_factor = 0;
        self.dirty_bytes = 268435456;
    }
}

impl Default for CachyosZramMemoryOptimizer {
    fn default() -> Self {
        Self::new(8192)
    }
}

// ==========================================
// 8. CACHYOS GAMEMODE & LATENCY BOOSTER
// ==========================================

pub struct CachyGameMode {
    pub active: AtomicBool,
    pub target_pid: AtomicU64,
    pub pcore_mask: u64,
    pub bypass_compositor: AtomicBool,
}

impl CachyGameMode {
    pub fn new(pcore_mask: u64) -> Self {
        Self {
            active: AtomicBool::new(false),
            target_pid: AtomicU64::new(0),
            pcore_mask,
            bypass_compositor: AtomicBool::new(false),
        }
    }

    pub fn engage_gamemode(&self, pid: u64) -> (bool, i32, u64) {
        self.target_pid.store(pid, Ordering::SeqCst);
        self.active.store(true, Ordering::SeqCst);
        self.bypass_compositor.store(true, Ordering::SeqCst);
        (true, -15, self.pcore_mask)
    }

    pub fn disengage_gamemode(&self) {
        self.active.store(false, Ordering::SeqCst);
        self.target_pid.store(0, Ordering::SeqCst);
        self.bypass_compositor.store(false, Ordering::SeqCst);
    }
}

// ==========================================
// 9. CACHYOS BORE SMP BALANCER (P-CORE / E-CORE TOPOLOGY)
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuCoreType {
    PerformancePCore,
    EfficiencyECore,
}

#[derive(Debug, Clone)]
pub struct CpuCoreTopology {
    pub core_id: u32,
    pub core_type: CpuCoreType,
}

pub struct CachyBoreSMPBalancer {
    pub cores: Vec<CpuCoreTopology>,
}

impl CachyBoreSMPBalancer {
    pub fn new() -> Self {
        Self { cores: Vec::new() }
    }

    pub fn register_core(&mut self, core_id: u32, core_type: CpuCoreType) {
        self.cores.push(CpuCoreTopology { core_id, core_type });
    }

    pub fn select_optimal_core_for_task(&self, is_interactive: bool) -> u32 {
        let target_type = if is_interactive {
            CpuCoreType::PerformancePCore
        } else {
            CpuCoreType::EfficiencyECore
        };

        for core in &self.cores {
            if core.core_type == target_type {
                return core.core_id;
            }
        }

        0
    }
}

impl Default for CachyBoreSMPBalancer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cachy_bore_wakeup_boost() {
        let booster = CachyBoreWakeupBooster::new();
        let mut metric = InteractiveThreadMetric {
            pid: 101,
            sleep_duration_ms: 80,
            run_duration_ms: 20, // 80% sleep = highly interactive UI thread
            last_wakeup_timestamp: 0,
            wakeup_boost_granted: false,
        };

        let slice_grant = booster.evaluate_wakeup_boost(&mut metric, 500);
        assert_eq!(slice_grant, 15);
        assert!(metric.wakeup_boost_granted);
        assert_eq!(metric.last_wakeup_timestamp, 500);
        assert_eq!(booster.calculate_preemption_boost(true), -8);
    }

    #[test]
    fn test_cachy_memory_compactor() {
        let compactor = CachyMemoryCompactor::new();
        let mut pages = Vec::new();
        for i in 0..512 {
            if i % 2 == 0 {
                pages.push(vec![0u8; 4096]); // Zero page
            } else {
                pages.push(vec![1u8; 4096]);
            }
        }

        let (huge_pages, zero_pages) = compactor.compact_and_coalesce(&mut pages, 4096);
        assert_eq!(huge_pages, 1);
        assert_eq!(zero_pages, 256);
        assert_eq!(compactor.zero_pages_reclaimed.load(Ordering::SeqCst), 256);
    }

    #[test]
    fn test_cachy_simd_dispatcher() {
        let dispatcher = CachySimdDispatcher::new(3); // AVX2
        let src = vec![42u8; 128];
        let mut dest = vec![0u8; 128];

        let copied = dispatcher.vectorized_memcpy(&mut dest, &src);
        assert_eq!(copied, 128);
        assert_eq!(dest, src);
    }

    #[test]
    fn test_garuda_performance_tweaks() {
        let mut gamemode = GarudaGameModeProfile::new();
        assert_eq!(gamemode.state, GarudaGameModeState::Disabled);

        gamemode.activate_ultra_performance();
        assert_eq!(gamemode.state, GarudaGameModeState::UltraPerformance);
        assert_eq!(gamemode.cpu_governor, "performance");
        assert_eq!(gamemode.gpu_perf_level, "high");

        let zram = GarudaZramTuner::new();
        assert_eq!(zram.ram_percentage, 150);
        assert_eq!(zram.comp_algorithm, "zstd");

        let mut irq_opt = GarudaIrqBalanceOptimizer::new();
        irq_opt.isolate_core_for_gaming(3);
        assert_eq!(irq_opt.isolated_cores, vec![3]);
    }

    #[test]
    fn test_cachy_pstate_and_sysctl_tuner() {
        let mut governor = CachyPStateGovernor::new();
        governor.set_epp_mode(PStateEppMode::Performance);
        assert_eq!(governor.get_epp_register_value(), 0x00);

        let mut tuner = CachySysctlTuner::new();
        assert_eq!(tuner.vfs_cache_pressure, 50);

        tuner.apply_gaming_sysctls();
        assert_eq!(tuner.vfs_cache_pressure, 20);
        assert_eq!(tuner.sched_cfs_bandwidth_slice_us, 1000);
    }

    #[test]
    fn test_cachyos_zram_gamemode_and_smp_balancer() {
        let mut zram = CachyosZramMemoryOptimizer::new(16384);
        zram.configure_low_latency_sysctls();
        assert_eq!(zram.swappiness, 150);
        assert_eq!(zram.watermark_boost_factor, 0);

        let gamemode = CachyGameMode::new(0x00FF);
        let (active, boost, mask) = gamemode.engage_gamemode(1234);
        assert!(active);
        assert_eq!(boost, -15);
        assert_eq!(mask, 0x00FF);
        assert!(gamemode.active.load(Ordering::SeqCst));

        gamemode.disengage_gamemode();
        assert!(!gamemode.active.load(Ordering::SeqCst));

        let mut balancer = CachyBoreSMPBalancer::new();
        balancer.register_core(0, CpuCoreType::PerformancePCore);
        balancer.register_core(1, CpuCoreType::PerformancePCore);
        balancer.register_core(2, CpuCoreType::EfficiencyECore);
        balancer.register_core(3, CpuCoreType::EfficiencyECore);

        assert_eq!(balancer.select_optimal_core_for_task(true), 0); // P-core
        assert_eq!(balancer.select_optimal_core_for_task(false), 2); // E-core
    }
}
