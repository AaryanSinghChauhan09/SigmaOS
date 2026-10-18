/// SigmaOS Swap & Hibernation Image Engine
/// Manages ZRAM swap, swsusp hibernate, and systemd sleep targets.
///
/// Standard library (std, libc) are fully authorized per SigmaOS policy.
use std::collections::HashMap;
use std::fmt;

/// Hibernation methods available
#[derive(Debug, Clone, PartialEq)]
pub enum HibernateMethod {
    SuspendToRam,  // S2RAM (sleep)
    SuspendToDisk, // S2Disk (hibernate, swsusp)
    HybridSleep,   // RAM + Disk
    SuspendThenHibernate,
}

impl fmt::Display for HibernateMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SuspendToRam => write!(f, "mem"),
            Self::SuspendToDisk => write!(f, "disk"),
            Self::HybridSleep => write!(f, "hybrid-sleep"),
            Self::SuspendThenHibernate => write!(f, "suspend-then-hibernate"),
        }
    }
}

/// Swap device descriptor
#[derive(Debug, Clone)]
pub struct SwapDevice {
    pub path: String,
    pub size_bytes: u64,
    pub priority: i32,
    pub is_zram: bool,
    pub compression: String,
}

/// ZRAM device configuration
#[derive(Debug, Clone)]
pub struct ZramConfig {
    pub device_id: u32,
    pub size_bytes: u64,
    pub algorithm: String, // lz4, zstd, lzo-rle
    pub streams: u32,
}

impl ZramConfig {
    pub fn new(device_id: u32, size_mb: u64, algorithm: &str) -> Self {
        Self {
            device_id,
            size_bytes: size_mb * 1024 * 1024,
            algorithm: algorithm.to_string(),
            streams: num_cpus_approx(),
        }
    }
}

fn num_cpus_approx() -> u32 {
    // Approximation without sys call in pure Rust context
    std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(4)
}

/// Hibernation image metadata
#[derive(Debug, Clone)]
pub struct HibernationImage {
    pub timestamp: u64,
    pub kernel_version: String,
    pub image_size_bytes: u64,
    pub compression: String,
    pub checksum: [u8; 32], // SHA-256
    pub resume_device: String,
    pub resume_offset: u64,
}

/// Swap & Hibernation Manager
pub struct SwapHibernationManager {
    pub swap_devices: Vec<SwapDevice>,
    pub zram_configs: Vec<ZramConfig>,
    pub hibernate_method: HibernateMethod,
    pub compression: String,
    pub stats: HashMap<String, u64>,
}

impl SwapHibernationManager {
    pub fn new() -> Self {
        Self {
            swap_devices: Vec::new(),
            zram_configs: Vec::new(),
            hibernate_method: HibernateMethod::SuspendToDisk,
            compression: "lz4".to_string(),
            stats: HashMap::new(),
        }
    }

    /// Register a ZRAM device for swap
    pub fn add_zram_swap(&mut self, size_mb: u64, algorithm: &str) -> u32 {
        let id = self.zram_configs.len() as u32;
        let cfg = ZramConfig::new(id, size_mb, algorithm);
        self.swap_devices.push(SwapDevice {
            path: format!("/dev/zram{}", id),
            size_bytes: cfg.size_bytes,
            priority: 100 + id as i32,
            is_zram: true,
            compression: algorithm.to_string(),
        });
        self.zram_configs.push(cfg);
        id
    }

    /// Register a physical swap partition or file
    pub fn add_swap_device(&mut self, path: &str, size_bytes: u64, priority: i32) {
        self.swap_devices.push(SwapDevice {
            path: path.to_string(),
            size_bytes,
            priority,
            is_zram: false,
            compression: String::new(),
        });
    }

    /// Compute total swap capacity
    pub fn total_swap_bytes(&self) -> u64 {
        self.swap_devices.iter().map(|d| d.size_bytes).sum()
    }

    /// Prepare a hibernation image descriptor (does not touch disk — kernel handles actual write)
    pub fn prepare_hibernate_image(
        &self,
        kernel_version: &str,
        resume_device: &str,
    ) -> Result<HibernationImage, &'static str> {
        if self.swap_devices.is_empty() {
            return Err("no swap device available for hibernation");
        }
        let primary_swap = &self.swap_devices[0];
        Ok(HibernationImage {
            timestamp: 0, // would be system time in production
            kernel_version: kernel_version.to_string(),
            image_size_bytes: 0, // kernel fills this in
            compression: self.compression.clone(),
            checksum: [0u8; 32],
            resume_device: resume_device.to_string(),
            resume_offset: if primary_swap.is_zram { 0 } else { 4096 }, // typical swap file offset
        })
    }

    /// Simulate hibernate transition (returns cmdline hint for kernel)
    pub fn hibernate_cmdline(&self, image: &HibernationImage) -> String {
        format!(
            "resume={} resume_offset={} mem_sleep_default={}",
            image.resume_device, image.resume_offset, self.hibernate_method
        )
    }

    pub fn record_stat(&mut self, key: &str, value: u64) {
        self.stats.insert(key.to_string(), value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zram_device_registration() {
        let mut mgr = SwapHibernationManager::new();
        let id = mgr.add_zram_swap(4096, "lz4");
        assert_eq!(id, 0);
        assert_eq!(mgr.zram_configs[0].size_bytes, 4096 * 1024 * 1024);
        assert_eq!(mgr.swap_devices[0].path, "/dev/zram0");
    }

    #[test]
    fn test_total_swap_calculation() {
        let mut mgr = SwapHibernationManager::new();
        mgr.add_zram_swap(4096, "lz4");
        mgr.add_swap_device("/dev/nvme0n1p3", 8 * 1024 * 1024 * 1024, 0);
        let total = mgr.total_swap_bytes();
        assert_eq!(total, 4096u64 * 1024 * 1024 + 8 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_hibernate_fails_without_swap() {
        let mgr = SwapHibernationManager::new();
        let result = mgr.prepare_hibernate_image("6.6.0-sigma", "/dev/nvme0n1p3");
        assert!(result.is_err());
    }

    #[test]
    fn test_hibernate_image_creation() {
        let mut mgr = SwapHibernationManager::new();
        mgr.add_swap_device("/dev/nvme0n1p3", 8 * 1024 * 1024 * 1024, 0);
        let img = mgr
            .prepare_hibernate_image("6.6.0-sigma", "/dev/nvme0n1p3")
            .unwrap();
        assert_eq!(img.resume_device, "/dev/nvme0n1p3");
        assert_eq!(img.compression, "lz4");
    }

    #[test]
    fn test_cmdline_generation() {
        let mut mgr = SwapHibernationManager::new();
        mgr.add_swap_device("/dev/nvme0n1p3", 8 * 1024 * 1024 * 1024, 0);
        let img = mgr
            .prepare_hibernate_image("6.6.0-sigma", "/dev/nvme0n1p3")
            .unwrap();
        let cmdline = mgr.hibernate_cmdline(&img);
        assert!(cmdline.contains("resume=/dev/nvme0n1p3"));
    }
}
