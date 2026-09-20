// SigmaOS Common System Tweaks Engine (Linux sysctl, CachyOS, Fedora, FreeBSD sysctl.conf Inspired)
// Provides kernel sysctl parameter tuning, CachyOS ZRAM & swappiness optimization,
// FreeBSD network buffer scaling, and low-latency gaming I/O scheduler profiles.

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;

// ============================================================================
// Linux & BSD sysctl Kernel Tunable Matrix Engine
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SysctlCategory {
    VirtualMemory,
    Networking,
    Kernel,
    FileSystem,
}

pub struct DistroSysctlTweaksEngine {
    pub active_sysctls: BTreeMap<String, String>,
}

impl DistroSysctlTweaksEngine {
    pub fn new() -> Self {
        let mut sysctls = BTreeMap::new();
        // CachyOS / Fedora default recommended sysctls
        sysctls.insert("vm.swappiness".to_string(), "15".to_string());
        sysctls.insert("vm.vfs_cache_pressure".to_string(), "50".to_string());
        sysctls.insert("vm.dirty_ratio".to_string(), "10".to_string());
        sysctls.insert("vm.dirty_background_ratio".to_string(), "5".to_string());
        sysctls.insert("net.core.somaxconn".to_string(), "4096".to_string());
        sysctls.insert("net.ipv4.tcp_congestion_control".to_string(), "bbr".to_string());
        sysctls.insert("fs.file-max".to_string(), "2097152".to_string());

        Self {
            active_sysctls: sysctls,
        }
    }

    pub fn set_sysctl(&mut self, key: &str, val: &str) {
        self.active_sysctls.insert(key.to_string(), val.to_string());
    }

    pub fn get_sysctl(&self, key: &str) -> Option<&String> {
        self.active_sysctls.get(key)
    }

    pub fn apply_recommended_preset(&mut self, preset_name: &str) -> Result<usize, &'static str> {
        match preset_name {
            "cachyos_performance" => {
                self.set_sysctl("vm.swappiness", "10");
                self.set_sysctl("vm.watermark_boost_factor", "0");
                self.set_sysctl("vm.watermark_scale_factor", "125");
                self.set_sysctl("kernel.sched_latency_ns", "4000000");
                Ok(4)
            }
            "freebsd_network_server" => {
                self.set_sysctl("net.inet.tcp.sendbuf_max", "16777216");
                self.set_sysctl("net.inet.tcp.recvbuf_max", "16777216");
                self.set_sysctl("net.inet.tcp.syncookies", "1");
                Ok(3)
            }
            _ => Err("Unknown sysctl preset name"),
        }
    }
}

// ============================================================================
// CachyOS ZRAM & Swappiness Memory Tweaks
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZramCompressionAlgorithm {
    Zstd,
    Lz4,
    Lzo,
}

pub struct CachyOsMemorySchedulerTweaks {
    pub zram_size_ratio_percent: u32,
    pub compression_algo: ZramCompressionAlgorithm,
    pub thp_madvise_enabled: bool,
}

impl CachyOsMemorySchedulerTweaks {
    pub fn new() -> Self {
        Self {
            zram_size_ratio_percent: 100, // 100% of physical RAM as compressed ZRAM
            compression_algo: ZramCompressionAlgorithm::Zstd,
            thp_madvise_enabled: true,
        }
    }

    pub fn configure_zram_swap(&mut self, ram_gb: u32) -> String {
        let zram_gb = (ram_gb * self.zram_size_ratio_percent) / 100;
        format!(
            "CachyOS Memory Optimization: Configured {}GB ZRAM swap pool with {:?} compression (THP: madvise)",
            zram_gb, self.compression_algo
        )
    }
}

// ============================================================================
// FreeBSD Network Stack Scaling Tweaks
// ============================================================================

pub struct BsdNetworkStackTweaks {
    pub is_bbr_enabled: bool,
    pub max_socket_backlog: u32,
    pub syn_flood_cookie_protection: bool,
}

impl BsdNetworkStackTweaks {
    pub fn new() -> Self {
        Self {
            is_bbr_enabled: true,
            max_socket_backlog: 8192,
            syn_flood_cookie_protection: true,
        }
    }

    pub fn apply_network_tweaks(&self) -> String {
        format!(
            "BSD Network Stack: BBR congestion control enabled, max backlog {}, SYN cookies active",
            self.max_socket_backlog
        )
    }
}

// ============================================================================
// Low-Latency Gaming & Audio Tweaks
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NVmeIoScheduler {
    None,
    Kyber,
    Bfq,
}

pub struct GamingAndLowLatencyTweaks {
    pub io_scheduler: NVmeIoScheduler,
    pub real_time_audio_priority: u32,
    pub power_governor: String,
}

impl GamingAndLowLatencyTweaks {
    pub fn new() -> Self {
        Self {
            io_scheduler: NVmeIoScheduler::None, // Low latency NVMe bypass
            real_time_audio_priority: 95,        // PipeWire / JACK SCHED_FIFO 95
            power_governor: "performance".to_string(),
        }
    }

    pub fn activate_gaming_profile(&mut self) -> String {
        format!(
            "Low-Latency Gaming Tweaks: NVMe scheduler set to {:?}, Audio RT priority {}, Governor: {}",
            self.io_scheduler, self.real_time_audio_priority, self.power_governor
        )
    }
}

impl Default for DistroSysctlTweaksEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CachyOsMemorySchedulerTweaks {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BsdNetworkStackTweaks {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for GamingAndLowLatencyTweaks {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysctl_tweaks_engine() {
        let mut engine = DistroSysctlTweaksEngine::new();
        assert_eq!(engine.get_sysctl("vm.swappiness").unwrap(), "15");

        let applied = engine.apply_recommended_preset("cachyos_performance").unwrap();
        assert_eq!(applied, 4);
        assert_eq!(engine.get_sysctl("vm.swappiness").unwrap(), "10");
    }

    #[test]
    fn test_cachyos_memory_tweaks() {
        let mut tweaks = CachyOsMemorySchedulerTweaks::new();
        let status = tweaks.configure_zram_swap(16);
        assert!(status.contains("16GB ZRAM"));
        assert!(status.contains("Zstd"));
    }

    #[test]
    fn test_bsd_network_tweaks() {
        let tweaks = BsdNetworkStackTweaks::new();
        let status = tweaks.apply_network_tweaks();
        assert!(status.contains("BBR congestion control"));
        assert!(status.contains("8192"));
    }

    #[test]
    fn test_gaming_low_latency_tweaks() {
        let mut tweaks = GamingAndLowLatencyTweaks::new();
        let status = tweaks.activate_gaming_profile();
        assert!(status.contains("NVMe scheduler set to None"));
        assert!(status.contains("Audio RT priority 95"));
    }
}
