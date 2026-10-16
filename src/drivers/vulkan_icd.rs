// SigmaOS Vulkan Installable Client Driver (ICD) Loader Subsystem
// Coordinates Vulkan API dispatch, physical device enumeration, and driver discovery.

#![allow(dead_code)]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub const VK_API_VERSION_1_3: u32 = (1 << 22) | (3 << 12);

/// Vulkan Physical Device Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkPhysicalDeviceType {
    IntegratedGpu,
    DiscreteGpu,
    VirtualGpu,
    CpuRasterizer,
}

/// Physical GPU Hardware Properties
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceProperties {
    pub device_name: String,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: VkPhysicalDeviceType,
    pub api_version: u32,
    pub driver_version: u32,
    pub vram_mb: usize,
}

/// Vulkan Queue Family Properties
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkQueueFamilyProperties {
    pub queue_flags: u32, // VK_QUEUE_GRAPHICS_BIT | VK_QUEUE_COMPUTE_BIT | VK_QUEUE_TRANSFER_BIT
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
}

pub const VK_QUEUE_GRAPHICS_BIT: u32 = 0x01;
pub const VK_QUEUE_COMPUTE_BIT: u32 = 0x02;
pub const VK_QUEUE_TRANSFER_BIT: u32 = 0x04;

/// Registered Vulkan ICD Driver
#[derive(Debug, Clone)]
pub struct VulkanIcdDriver {
    pub library_path: String,
    pub devices: Vec<VkPhysicalDeviceProperties>,
}

/// Vulkan ICD Loader Engine
#[derive(Debug, Clone, Default)]
pub struct VulkanIcdLoader {
    pub installed_icds: Vec<VulkanIcdDriver>,
}

impl VulkanIcdLoader {
    pub fn new() -> Self {
        Self {
            installed_icds: Vec::new(),
        }
    }

    /// Register a discovered Vulkan hardware driver
    pub fn register_icd(&mut self, library_path: &str, devices: &[VkPhysicalDeviceProperties]) {
        self.installed_icds.push(VulkanIcdDriver {
            library_path: String::from(library_path),
            devices: devices.to_vec(),
        });
    }

    /// Enumerate all available physical GPUs across all registered ICDs
    pub fn enumerate_physical_devices(&self) -> Vec<VkPhysicalDeviceProperties> {
        let mut list = Vec::new();
        for icd in &self.installed_icds {
            list.extend(icd.devices.clone());
        }
        list
    }

    /// Select the highest performance GPU (prefers discrete over integrated over CPU)
    pub fn select_best_gpu(&self) -> Option<VkPhysicalDeviceProperties> {
        let all_gpus = self.enumerate_physical_devices();
        all_gpus.into_iter().max_by_key(|g| match g.device_type {
            VkPhysicalDeviceType::DiscreteGpu => 3000 + g.vram_mb,
            VkPhysicalDeviceType::IntegratedGpu => 2000 + g.vram_mb,
            VkPhysicalDeviceType::VirtualGpu => 1000,
            VkPhysicalDeviceType::CpuRasterizer => 100,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vulkan_icd_enumeration_and_selection() {
        let mut loader = VulkanIcdLoader::new();
        let gpus = [
            VkPhysicalDeviceProperties {
                device_name: String::from("Intel Iris Xe"),
                vendor_id: 0x8086,
                device_id: 0x9A49,
                device_type: VkPhysicalDeviceType::IntegratedGpu,
                api_version: VK_API_VERSION_1_3,
                driver_version: 1,
                vram_mb: 2048,
            },
            VkPhysicalDeviceProperties {
                device_name: String::from("NVIDIA GeForce RTX 4090"),
                vendor_id: 0x10DE,
                device_id: 0x2684,
                device_type: VkPhysicalDeviceType::DiscreteGpu,
                api_version: VK_API_VERSION_1_3,
                driver_version: 550,
                vram_mb: 24576,
            },
        ];

        loader.register_icd("/usr/lib/libvulkan_intel.so", &gpus[0..1]);
        loader.register_icd("/usr/lib/libvulkan_nouveau.so", &gpus[1..2]);

        assert_eq!(loader.enumerate_physical_devices().len(), 2);
        let best = loader.select_best_gpu().expect("Best GPU");
        assert_eq!(best.device_name, "NVIDIA GeForce RTX 4090");
    }
}
