// System Information Tool
// Inspired by Linux Mint 22.2 System Information tool
// Provides detailed hardware and system information for troubleshooting

use std::collections::HashMap;

/// USB device information
#[derive(Debug, Clone, PartialEq)]
pub struct UsbDevice {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub vendor_id: String,
    pub product_id: String,
    pub controller: String,
    pub connection_speed: String,
    pub power_usage: String,
    pub max_speed: String,
    pub max_power: String,
}

impl UsbDevice {
    pub fn new(
        id: String,
        name: String,
        device_type: String,
        vendor_id: String,
        product_id: String,
    ) -> Self {
        Self {
            id,
            name,
            device_type,
            vendor_id,
            product_id,
            controller: String::new(),
            connection_speed: String::new(),
            power_usage: String::new(),
            max_speed: String::new(),
            max_power: String::new(),
        }
    }

    pub fn with_controller_info(
        mut self,
        controller: String,
        connection_speed: String,
        power_usage: String,
        max_speed: String,
        max_power: String,
    ) -> Self {
        self.controller = controller;
        self.connection_speed = connection_speed;
        self.power_usage = power_usage;
        self.max_speed = max_speed;
        self.max_power = max_power;
        self
    }
}

/// GPU information
#[derive(Debug, Clone, PartialEq)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: String,
    pub driver: String,
    pub hardware_acceleration: bool,
    pub memory_total: String,
    pub memory_used: String,
    pub opengl_version: String,
    pub vulkan_support: bool,
    pub compute_units: u32,
}

impl GpuInfo {
    pub fn new(name: String, vendor: String, driver: String) -> Self {
        Self {
            name,
            vendor,
            driver,
            hardware_acceleration: false,
            memory_total: String::new(),
            memory_used: String::new(),
            opengl_version: String::new(),
            vulkan_support: false,
            compute_units: 0,
        }
    }

    pub fn with_hardware_info(
        mut self,
        hardware_acceleration: bool,
        memory_total: String,
        memory_used: String,
        opengl_version: String,
        vulkan_support: bool,
        compute_units: u32,
    ) -> Self {
        self.hardware_acceleration = hardware_acceleration;
        self.memory_total = memory_total;
        self.memory_used = memory_used;
        self.opengl_version = opengl_version;
        self.vulkan_support = vulkan_support;
        self.compute_units = compute_units;
        self
    }
}

/// PCI device information
#[derive(Debug, Clone, PartialEq)]
pub struct PciDevice {
    pub slot: String,
    pub class: String,
    pub vendor: String,
    pub device: String,
    pub subsystem_vendor: String,
    pub subsystem_device: String,
    pub driver: String,
    pub pci_id: String,
}

impl PciDevice {
    pub fn new(
        slot: String,
        class: String,
        vendor: String,
        device: String,
        driver: String,
    ) -> Self {
        Self {
            slot,
            class,
            vendor,
            device,
            subsystem_vendor: String::new(),
            subsystem_device: String::new(),
            driver,
            pci_id: String::new(),
        }
    }

    pub fn with_subsystem_info(
        mut self,
        subsystem_vendor: String,
        subsystem_device: String,
        pci_id: String,
    ) -> Self {
        self.subsystem_vendor = subsystem_vendor;
        self.subsystem_device = subsystem_device;
        self.pci_id = pci_id;
        self
    }
}

/// BIOS/UEFI information
#[derive(Debug, Clone, PartialEq)]
pub struct BiosInfo {
    pub vendor: String,
    pub version: String,
    pub release_date: String,
    pub bios_revision: String,
    pub board_vendor: String,
    pub board_name: String,
    pub board_version: String,
    pub boot_mode: String,
    pub secure_boot: bool,
}

impl BiosInfo {
    pub fn new(
        vendor: String,
        version: String,
        release_date: String,
        board_vendor: String,
        board_name: String,
    ) -> Self {
        Self {
            vendor,
            version,
            release_date,
            bios_revision: String::new(),
            board_vendor,
            board_name,
            board_version: String::new(),
            boot_mode: String::new(),
            secure_boot: false,
        }
    }

    pub fn with_boot_info(
        mut self,
        bios_revision: String,
        board_version: String,
        boot_mode: String,
        secure_boot: bool,
    ) -> Self {
        self.bios_revision = bios_revision;
        self.board_version = board_version;
        self.boot_mode = boot_mode;
        self.secure_boot = secure_boot;
        self
    }
}

/// System Information Manager
#[derive(Debug, Clone)]
pub struct SystemInfoManager {
    usb_devices: Vec<UsbDevice>,
    gpu_info: Vec<GpuInfo>,
    pci_devices: Vec<PciDevice>,
    bios_info: Option<BiosInfo>,
}

impl SystemInfoManager {
    pub fn new() -> Self {
        Self {
            usb_devices: Vec::new(),
            gpu_info: Vec::new(),
            pci_devices: Vec::new(),
            bios_info: None,
        }
    }

    /// Add a USB device
    pub fn add_usb_device(&mut self, device: UsbDevice) {
        self.usb_devices.push(device);
    }

    /// Get all USB devices
    pub fn get_usb_devices(&self) -> &[UsbDevice] {
        &self.usb_devices
    }

    /// Get USB devices grouped by controller
    pub fn get_usb_by_controller(&self) -> HashMap<String, Vec<&UsbDevice>> {
        let mut grouped = HashMap::new();
        for device in &self.usb_devices {
            grouped
                .entry(device.controller.clone())
                .or_insert_with(Vec::new)
                .push(device);
        }
        grouped
    }

    /// Add GPU information
    pub fn add_gpu_info(&mut self, gpu: GpuInfo) {
        self.gpu_info.push(gpu);
    }

    /// Get all GPU information
    pub fn get_gpu_info(&self) -> &[GpuInfo] {
        &self.gpu_info
    }

    /// Get default GPU
    pub fn get_default_gpu(&self) -> Option<&GpuInfo> {
        self.gpu_info.first()
    }

    /// Add PCI device
    pub fn add_pci_device(&mut self, device: PciDevice) {
        self.pci_devices.push(device);
    }

    /// Get all PCI devices
    pub fn get_pci_devices(&self) -> &[PciDevice] {
        &self.pci_devices
    }

    /// Get PCI devices by class
    pub fn get_pci_by_class(&self, class: &str) -> Vec<&PciDevice> {
        self.pci_devices
            .iter()
            .filter(|d| d.class.contains(class))
            .collect()
    }

    /// Set BIOS information
    pub fn set_bios_info(&mut self, bios: BiosInfo) {
        self.bios_info = Some(bios);
    }

    /// Get BIOS information
    pub fn get_bios_info(&self) -> Option<&BiosInfo> {
        self.bios_info.as_ref()
    }

    /// Scan for potential USB issues
    pub fn diagnose_usb_issues(&self) -> Vec<String> {
        let mut issues = Vec::new();

        for device in &self.usb_devices {
            // Check for slow transfers
            if let (Some(speed), Some(max)) = (
                device.connection_speed.parse::<f64>().ok(),
                device.max_speed.parse::<f64>().ok(),
            ) {
                if speed < max * 0.5 {
                    issues.push(format!(
                        "{} running at {}% of max speed ({} vs {})",
                        device.name,
                        (speed / max * 100.0) as u32,
                        device.connection_speed,
                        device.max_speed
                    ));
                }
            }

            // Check for power issues
            if let (Some(usage), Some(max)) = (
                device.power_usage.parse::<f64>().ok(),
                device.max_power.parse::<f64>().ok(),
            ) {
                if usage > max * 0.9 {
                    issues.push(format!(
                        "{} near power limit ({}mA of {}mA)",
                        device.name,
                        usage as u32,
                        max as u32
                    ));
                }
            }
        }

        issues
    }

    /// Get system summary
    pub fn get_summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();
        summary.insert("usb_devices".to_string(), self.usb_devices.len().to_string());
        summary.insert("gpu_count".to_string(), self.gpu_info.len().to_string());
        summary.insert("pci_devices".to_string(), self.pci_devices.len().to_string());
        summary.insert(
            "bios_vendor".to_string(),
            self.bios_info
                .as_ref()
                .map(|b| b.vendor.clone())
                .unwrap_or_else(|| "Unknown".to_string()),
        );
        summary.insert(
            "boot_mode".to_string(),
            self.bios_info
                .as_ref()
                .map(|b| b.boot_mode.clone())
                .unwrap_or_else(|| "Unknown".to_string()),
        );
        summary.insert(
            "secure_boot".to_string(),
            self.bios_info
                .as_ref()
                .map(|b| b.secure_boot.to_string())
                .unwrap_or_else(|| "Unknown".to_string()),
        );
        summary
    }
}

impl Default for SystemInfoManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_manager_creation() {
        let manager = SystemInfoManager::new();
        assert_eq!(manager.get_usb_devices().len(), 0);
        assert_eq!(manager.get_gpu_info().len(), 0);
        assert_eq!(manager.get_pci_devices().len(), 0);
        assert!(manager.get_bios_info().is_none());
    }

    #[test]
    fn test_usb_device_addition() {
        let mut manager = SystemInfoManager::new();
        let device = UsbDevice::new(
            "1-1".to_string(),
            "Logitech Mouse".to_string(),
            "Mouse".to_string(),
            "046d".to_string(),
            "c52b".to_string(),
        );
        manager.add_usb_device(device);
        assert_eq!(manager.get_usb_devices().len(), 1);
    }

    #[test]
    fn test_usb_device_with_controller() {
        let device = UsbDevice::new(
            "1-1".to_string(),
            "USB Keyboard".to_string(),
            "Keyboard".to_string(),
            "046d".to_string(),
            "c31c".to_string(),
        )
        .with_controller_info(
            "usb1".to_string(),
            "480 Mbps".to_string(),
            "100 mA".to_string(),
            "480 Mbps".to_string(),
            "500 mA".to_string(),
        );
        assert_eq!(device.controller, "usb1");
        assert_eq!(device.connection_speed, "480 Mbps");
    }

    #[test]
    fn test_usb_grouping() {
        let mut manager = SystemInfoManager::new();
        let device1 = UsbDevice::new(
            "1-1".to_string(),
            "Device 1".to_string(),
            "Storage".to_string(),
            "1234".to_string(),
            "5678".to_string(),
        )
        .with_controller_info("usb1".to_string(), String::new(), String::new(), String::new(), String::new());
        let device2 = UsbDevice::new(
            "1-2".to_string(),
            "Device 2".to_string(),
            "Storage".to_string(),
            "1234".to_string(),
            "5679".to_string(),
        )
        .with_controller_info("usb1".to_string(), String::new(), String::new(), String::new(), String::new());
        let device3 = UsbDevice::new(
            "2-1".to_string(),
            "Device 3".to_string(),
            "Storage".to_string(),
            "1234".to_string(),
            "5680".to_string(),
        )
        .with_controller_info("usb2".to_string(), String::new(), String::new(), String::new(), String::new());

        manager.add_usb_device(device1);
        manager.add_usb_device(device2);
        manager.add_usb_device(device3);

        let grouped = manager.get_usb_by_controller();
        assert_eq!(grouped.get("usb1").map(|v| v.len()), Some(2));
        assert_eq!(grouped.get("usb2").map(|v| v.len()), Some(1));
    }

    #[test]
    fn test_gpu_info() {
        let mut manager = SystemInfoManager::new();
        let gpu = GpuInfo::new(
            "NVIDIA GeForce RTX 3080".to_string(),
            "NVIDIA".to_string(),
            "nvidia".to_string(),
        )
        .with_hardware_info(
            true,
            "10 GB".to_string(),
            "2 GB".to_string(),
            "4.6".to_string(),
            true,
            8704,
        );
        manager.add_gpu_info(gpu);
        assert_eq!(manager.get_gpu_info().len(), 1);
        assert!(manager.get_default_gpu().is_some());
    }

    #[test]
    fn test_pci_device() {
        let mut manager = SystemInfoManager::new();
        let device = PciDevice::new(
            "00:00.0".to_string(),
            "Host bridge".to_string(),
            "Intel Corporation".to_string(),
            "Device 1234".to_string(),
            "pci-root".to_string(),
        )
        .with_subsystem_info(
            "Intel Corporation".to_string(),
            "Device 5678".to_string(),
            "8086:1234".to_string(),
        );
        manager.add_pci_device(device);
        assert_eq!(manager.get_pci_devices().len(), 1);
    }

    #[test]
    fn test_pci_by_class() {
        let mut manager = SystemInfoManager::new();
        let device1 = PciDevice::new(
            "00:02.0".to_string(),
            "VGA controller".to_string(),
            "Intel".to_string(),
            "Device".to_string(),
            "i915".to_string(),
        );
        let device2 = PciDevice::new(
            "00:1f.0".to_string(),
            "Audio device".to_string(),
            "Intel".to_string(),
            "Device".to_string(),
            "snd_hda_intel".to_string(),
        );
        manager.add_pci_device(device1);
        manager.add_pci_device(device2);

        let vga_devices = manager.get_pci_by_class("VGA");
        assert_eq!(vga_devices.len(), 1);
    }

    #[test]
    fn test_bios_info() {
        let mut manager = SystemInfoManager::new();
        let bios = BiosInfo::new(
            "American Megatrends Inc.".to_string(),
            "F2".to_string(),
            "01/15/2024".to_string(),
            "ASUS".to_string(),
            "ROG STRIX B550-F GAMING".to_string(),
        )
        .with_boot_info("1.0".to_string(), "Rev X.0".to_string(), "UEFI".to_string(), true);
        manager.set_bios_info(bios);
        assert!(manager.get_bios_info().is_some());
        assert_eq!(manager.get_bios_info().unwrap().secure_boot, true);
    }

    #[test]
    fn test_usb_diagnosis() {
        let mut manager = SystemInfoManager::new();
        let device = UsbDevice::new(
            "1-1".to_string(),
            "Slow Device".to_string(),
            "Storage".to_string(),
            "1234".to_string(),
            "5678".to_string(),
        )
        .with_controller_info(
            "usb1".to_string(),
            "12.0".to_string(),
            "100.0".to_string(),
            "480.0".to_string(),
            "500.0".to_string(),
        );
        manager.add_usb_device(device);

        let issues = manager.diagnose_usb_issues();
        assert!(!issues.is_empty());
        assert!(issues[0].contains("%"));
    }

    #[test]
    fn test_system_summary() {
        let mut manager = SystemInfoManager::new();
        manager.add_usb_device(UsbDevice::new(
            "1-1".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            "1234".to_string(),
            "5678".to_string(),
        ));
        manager.add_gpu_info(GpuInfo::new(
            "Test GPU".to_string(),
            "Test".to_string(),
            "test".to_string(),
        ));
        manager.add_pci_device(PciDevice::new(
            "00:00.0".to_string(),
            "Host bridge".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            "test".to_string(),
        ));
        manager.set_bios_info(BiosInfo::new(
            "Test".to_string(),
            "1.0".to_string(),
            "2024".to_string(),
            "Test".to_string(),
            "Test".to_string(),
        ));

        let summary = manager.get_summary();
        assert_eq!(summary.get("usb_devices"), Some(&"1".to_string()));
        assert_eq!(summary.get("gpu_count"), Some(&"1".to_string()));
        assert_eq!(summary.get("pci_devices"), Some(&"1".to_string()));
    }
}
