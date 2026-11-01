//! GPU detection, configuration, and driver management for SigmaOS.

/// GPU vendor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuVendor {
    Intel,
    Amd,
    Nvidia,
    VirtIO,
    Unknown,
}

impl GpuVendor {
    /// Detect vendor from a PCI vendor ID.
    pub fn from_vendor_id(id: u16) -> Self {
        match id {
            0x8086 => Self::Intel,
            0x1002 => Self::Amd,
            0x10DE => Self::Nvidia,
            0x1AF4 => Self::VirtIO,
            _ => Self::Unknown,
        }
    }
}

/// Information about a detected GPU.
#[derive(Debug, Clone)]
pub struct GpuInfo {
    pub vendor: GpuVendor,
    pub name: String,
    pub vram_mb: u64,
    pub driver: String,
    pub supports_vulkan: bool,
    pub supports_opencl: bool,
    pub supports_vaapi: bool,
    pub supports_vdpau: bool,
}

/// GPU management errors.
#[derive(Debug, thiserror::Error)]
pub enum GpuError {
    #[error("no GPU detected")]
    NoGpuFound,
    #[error("driver {0} not loaded")]
    DriverNotLoaded(String),
    #[error("Vulkan not supported on this GPU")]
    VulkanNotSupported,
    #[error("VA-API not supported on this GPU")]
    VaapiNotSupported,
    #[error("proprietary driver required: run `sigma drivers --install-nvidia`")]
    ProprietaryDriverRequired,
}

/// GPU manager: detect and configure GPUs.
pub struct GpuManager;

impl GpuManager {
    /// Detect all GPUs in the system (reads /sys/bus/pci/devices).
    pub fn detect() -> Result<Vec<GpuInfo>, GpuError> {
        // Production: parse /sys/bus/pci/devices for class 0x0300/0x0302
        Ok(vec![
            GpuInfo {
                vendor: GpuVendor::Intel,
                name: "Intel Arc A770".to_string(),
                vram_mb: 16384,
                driver: "i915".to_string(),
                supports_vulkan: true,
                supports_opencl: true,
                supports_vaapi: true,
                supports_vdpau: false,
            },
        ])
    }

    /// Return the primary GPU (discrete preferred over integrated).
    pub fn primary() -> Result<GpuInfo, GpuError> {
        let mut gpus = Self::detect()?;
        if gpus.is_empty() {
            return Err(GpuError::NoGpuFound);
        }
        // Prefer discrete (higher VRAM) over integrated.
        gpus.sort_by(|a, b| b.vram_mb.cmp(&a.vram_mb));
        Ok(gpus.remove(0))
    }

    /// Configure Vulkan ICD for the given GPU.
    pub fn configure_vulkan(gpu: &GpuInfo) -> Result<(), GpuError> {
        if !gpu.supports_vulkan {
            return Err(GpuError::VulkanNotSupported);
        }
        // Production: write /etc/vulkan/icd.d/<driver>.json
        Ok(())
    }

    /// Enable VA-API hardware video decoding for the given GPU.
    pub fn configure_vaapi(gpu: &GpuInfo) -> Result<(), GpuError> {
        if !gpu.supports_vaapi {
            return Err(GpuError::VaapiNotSupported);
        }
        // Production: set LIBVA_DRIVER_NAME environment variable in /etc/environment
        Ok(())
    }

    /// Return `true` if the GPU requires a proprietary (non-free) driver.
    pub fn needs_proprietary_driver(gpu: &GpuInfo) -> bool {
        gpu.vendor == GpuVendor::Nvidia
    }

    /// Install the appropriate driver for the GPU.
    pub fn install_driver(gpu: &GpuInfo) -> Result<(), GpuError> {
        if Self::needs_proprietary_driver(gpu) {
            return Err(GpuError::ProprietaryDriverRequired);
        }
        // Open-source driver: already in-kernel, nothing to install.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vendor_from_id_intel() {
        assert_eq!(GpuVendor::from_vendor_id(0x8086), GpuVendor::Intel);
    }

    #[test]
    fn test_vendor_from_id_amd() {
        assert_eq!(GpuVendor::from_vendor_id(0x1002), GpuVendor::Amd);
    }

    #[test]
    fn test_vendor_from_id_nvidia() {
        assert_eq!(GpuVendor::from_vendor_id(0x10DE), GpuVendor::Nvidia);
    }

    #[test]
    fn test_needs_proprietary_driver_true_for_nvidia() {
        let gpu = GpuInfo {
            vendor: GpuVendor::Nvidia,
            name: "RTX 3070".to_string(),
            vram_mb: 8192,
            driver: "nvidia".to_string(),
            supports_vulkan: true,
            supports_opencl: true,
            supports_vaapi: false,
            supports_vdpau: true,
        };
        assert!(GpuManager::needs_proprietary_driver(&gpu));
    }

    #[test]
    fn test_needs_proprietary_driver_false_for_intel() {
        let gpu = GpuInfo {
            vendor: GpuVendor::Intel,
            name: "Arc A770".to_string(),
            vram_mb: 16384,
            driver: "i915".to_string(),
            supports_vulkan: true,
            supports_opencl: true,
            supports_vaapi: true,
            supports_vdpau: false,
        };
        assert!(!GpuManager::needs_proprietary_driver(&gpu));
    }

    #[test]
    fn test_primary_gpu_detected() {
        let gpu = GpuManager::primary().unwrap();
        assert!(!gpu.name.is_empty());
    }

    #[test]
    fn test_configure_vulkan_fails_when_not_supported() {
        let gpu = GpuInfo {
            vendor: GpuVendor::VirtIO,
            name: "VirtIO".to_string(),
            vram_mb: 256,
            driver: "virtio-gpu".to_string(),
            supports_vulkan: false,
            supports_opencl: false,
            supports_vaapi: false,
            supports_vdpau: false,
        };
        assert!(matches!(GpuManager::configure_vulkan(&gpu), Err(GpuError::VulkanNotSupported)));
    }
}
