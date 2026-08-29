use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
// SigmaOS Hardware Detection and Device Manager
// Inspired by Linux udev, DevFS, and BSD device management
// Supports automatic device detection, hot-plug events, and device abstraction

use crate::klib::HashMap;
// std::fs not in no_std
// Path/PathBuf not in no_std

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    PCI,
    USB,
    SATA,
    NVMe,
    Network,
    Audio,
    Video,
    Input,
    Storage,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub device_type: DeviceType,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_id: u8,
    pub subclass_id: u8,
    pub prog_if: u8,
    pub name: String,
    pub description: String,
    pub driver: Option<String>,
    pub status: DeviceStatus,
    pub resources: DeviceResources,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceStatus {
    Disconnected,
    Connected,
    Initialized,
    Active,
    Error,
}

#[derive(Debug, Clone)]
pub struct DeviceResources {
    pub io_ports: Vec<(u16, u16)>, // (base, size)
    pub memory_ranges: Vec<(u64, u64)>, // (base, size)
    pub irq: Option<u32>,
    pub dma_channel: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct DriverInfo {
    pub name: String,
    pub version: String,
    pub supported_devices: Vec<(u16, u16)>, // (vendor_id, device_id)
    pub supported_classes: Vec<(u8, u8)>, // (class_id, subclass_id)
    pub priority: u32,
    pub load_status: DriverLoadStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverLoadStatus {
    NotLoaded,
    Loading,
    Loaded,
    Failed,
}

pub struct HardwareManager {
    devices: HashMap<String, DeviceInfo>,
    drivers: HashMap<String, DriverInfo>,
    device_tree: DeviceTree,
    hotplug_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct DeviceTree {
    pub root: DeviceNode,
}

#[derive(Debug, Clone)]
pub struct DeviceNode {
    pub name: String,
    pub device: Option<DeviceInfo>,
    pub children: Vec<DeviceNode>,
}

impl HardwareManager {
    pub fn new() -> Self {
        HardwareManager {
            devices: HashMap::new(),
            drivers: HashMap::new(),
            device_tree: DeviceTree {
                root: DeviceNode {
                    name: "root".to_string(),
                    device: None,
                    children: vec![],
                },
            },
            hotplug_enabled: true,
        }
    }

    pub fn scan_hardware(&mut self) -> Result<(), String> {
        println!("Scanning hardware devices...");
        
        // Scan PCI bus
        self.scan_pci_bus()?;
        
        // Scan USB bus
        self.scan_usb_bus()?;
        
        // Scan storage devices
        self.scan_storage_devices()?;
        
        // Build device tree
        self.build_device_tree()?;
        
        println!("Hardware scan complete. Found {} devices.", self.devices.len());
        Ok(())
    }

    fn scan_pci_bus(&mut self) -> Result<(), String> {
        println!("Scanning PCI bus...");
        
        // Simulate PCI device discovery
        // In real implementation, would read PCI configuration space
        
        // Add example PCI devices
        self.add_device(DeviceInfo {
            device_type: DeviceType::PCI,
            vendor_id: 0x8086, // Intel
            device_id: 0x1af4, // Example device
            class_id: 0x01, // Mass storage
            subclass_id: 0x06, // SATA
            prog_if: 0x01,
            name: "intel_sata_controller".to_string(),
            description: "Intel SATA Controller".to_string(),
            driver: Some("ahci".to_string()),
            status: DeviceStatus::Connected,
            resources: DeviceResources {
                io_ports: vec![(0x1F0, 8), (0x3F6, 1)],
                memory_ranges: vec![(0xFE000000, 0x1000)],
                irq: Some(14),
                dma_channel: None,
            },
        });
        
        self.add_device(DeviceInfo {
            device_type: DeviceType::PCI,
            vendor_id: 0x10DE, // NVIDIA
            device_id: 0x1B80, // Example GPU
            class_id: 0x03, // Display
            subclass_id: 0x00, // VGA
            prog_if: 0x00,
            name: "nvidia_gpu".to_string(),
            description: "NVIDIA Graphics Card".to_string(),
            driver: Some("nouveau".to_string()),
            status: DeviceStatus::Connected,
            resources: DeviceResources {
                io_ports: vec![],
                memory_ranges: vec![(0xFD000000, 0x1000000)],
                irq: Some(16),
                dma_channel: None,
            },
        });
        
        Ok(())
    }

    fn scan_usb_bus(&mut self) -> Result<(), String> {
        println!("Scanning USB bus...");
        
        // Simulate USB device discovery
        self.add_device(DeviceInfo {
            device_type: DeviceType::USB,
            vendor_id: 0x046D, // Logitech
            device_id: 0xC52B, // USB Receiver
            class_id: 0x03, // HID
            subclass_id: 0x01, // Keyboard
            prog_if: 0x01,
            name: "logitech_usb_receiver".to_string(),
            description: "Logitech USB Receiver".to_string(),
            driver: Some("usbhid".to_string()),
            status: DeviceStatus::Connected,
            resources: DeviceResources {
                io_ports: vec![],
                memory_ranges: vec![],
                irq: Some(12),
                dma_channel: None,
            },
        });
        
        Ok(())
    }

    fn scan_storage_devices(&mut self) -> Result<(), String> {
        println!("Scanning storage devices...");
        
        // Simulate storage device discovery
        self.add_device(DeviceInfo {
            device_type: DeviceType::SATA,
            vendor_id: 0x1AF4, // VirtIO
            device_id: 0x1001, // Block device
            class_id: 0x01, // Mass storage
            subclass_id: 0x01, // SCSI
            prog_if: 0x00,
            name: "virtio_block".to_string(),
            description: "VirtIO Block Device".to_string(),
            driver: Some("virtio_blk".to_string()),
            status: DeviceStatus::Connected,
            resources: DeviceResources {
                io_ports: vec![],
                memory_ranges: vec![(0xFE000000, 0x1000)],
                irq: Some(15),
                dma_channel: None,
            },
        });
        
        Ok(())
    }

    fn add_device(&mut self, device: DeviceInfo) {
        let device_id = device.name.clone();
        self.devices.insert(device_id.clone(), device);
    }

    fn build_device_tree(&mut self) -> Result<(), String> {
        println!("Building device tree...");
        
        // Build hierarchical device tree
        // Root -> Buses -> Devices
        
        let mut pci_node = DeviceNode {
            name: "pci".to_string(),
            device: None,
            children: vec![],
        };
        
        let mut usb_node = DeviceNode {
            name: "usb".to_string(),
            device: None,
            children: vec![],
        };
        
        let mut storage_node = DeviceNode {
            name: "storage".to_string(),
            device: None,
            children: vec![],
        };
        
        for (name, device) in &self.devices {
            let node = DeviceNode {
                name: name.clone(),
                device: Some(device.clone()),
                children: vec![],
            };
            
            match device.device_type {
                DeviceType::PCI => pci_node.children.push(node),
                DeviceType::USB => usb_node.children.push(node),
                DeviceType::SATA | DeviceType::NVMe => storage_node.children.push(node),
                _ => {}
            }
        }
        
        self.device_tree.root.children = vec![pci_node, usb_node, storage_node];
        
        Ok(())
    }

    pub fn load_drivers(&mut self) -> Result<(), String> {
        println!("Loading device drivers...");
        
        // Load available drivers
        self.load_builtin_drivers()?;
        
        // Match devices to drivers
        self.match_drivers_to_devices()?;
        
        println!("Driver loading complete.");
        Ok(())
    }

    fn load_builtin_drivers(&mut self) -> Result<(), String> {
        // Register built-in drivers
        self.register_driver(DriverInfo {
            name: "ahci".to_string(),
            version: "1.0.0".to_string(),
            supported_devices: vec![(0x8086, 0x1af4)],
            supported_classes: vec![(0x01, 0x06)],
            priority: 100,
            load_status: DriverLoadStatus::NotLoaded,
        });
        
        self.register_driver(DriverInfo {
            name: "nouveau".to_string(),
            version: "1.0.0".to_string(),
            supported_devices: vec![(0x10DE, 0x1B80)],
            supported_classes: vec![(0x03, 0x00)],
            priority: 90,
            load_status: DriverLoadStatus::NotLoaded,
        });
        
        self.register_driver(DriverInfo {
            name: "usbhid".to_string(),
            version: "1.0.0".to_string(),
            supported_devices: vec![(0x046D, 0xC52B)],
            supported_classes: vec![(0x03, 0x01)],
            priority: 80,
            load_status: DriverLoadStatus::NotLoaded,
        });
        
        self.register_driver(DriverInfo {
            name: "virtio_blk".to_string(),
            version: "1.0.0".to_string(),
            supported_devices: vec![(0x1AF4, 0x1001)],
            supported_classes: vec![(0x01, 0x01)],
            priority: 95,
            load_status: DriverLoadStatus::NotLoaded,
        });
        
        Ok(())
    }

    fn register_driver(&mut self, driver: DriverInfo) {
        self.drivers.insert(driver.name.clone(), driver);
    }

    fn match_drivers_to_devices(&mut self) -> Result<(), String> {
        for (device_name, device) in self.devices.iter_mut() {
            let best_driver = self.find_best_driver(device);
            
            if let Some(driver_name) = best_driver {
                println!("Loading driver '{}' for device '{}'", driver_name, device_name);
                device.driver = Some(driver_name.clone());
                device.status = DeviceStatus::Initialized;
                
                if let Some(driver) = self.drivers.get_mut(&driver_name) {
                    driver.load_status = DriverLoadStatus::Loaded;
                }
            }
        }
        
        Ok(())
    }

    fn find_best_driver(&self, device: &DeviceInfo) -> Option<String> {
        let mut best_driver = None;
        let mut best_priority = 0;
        
        for (name, driver) in &self.drivers {
            // Check for exact device match
            if driver.supported_devices.contains(&(device.vendor_id, device.device_id)) {
                if driver.priority > best_priority {
                    best_priority = driver.priority;
                    best_driver = Some(name.clone());
                }
            }
            
            // Check for class match
            if driver.supported_classes.contains(&(device.class_id, device.subclass_id)) {
                if driver.priority > best_priority {
                    best_priority = driver.priority;
                    best_driver = Some(name.clone());
                }
            }
        }
        
        best_driver
    }

    pub fn handle_hotplug_event(&mut self, device: DeviceInfo) -> Result<(), String> {
        println!("Handling hotplug event for device: {}", device.name);
        
        if !self.hotplug_enabled {
            println!("Hotplug is disabled. Ignoring event.");
            return Ok(());
        }
        
        // Add device to registry
        self.add_device(device.clone());
        
        // Find and load driver
        let best_driver = self.find_best_driver(&device);
        if let Some(driver_name) = best_driver {
            println!("Auto-loading driver '{}' for hotplugged device", driver_name);
            if let Some(driver) = self.drivers.get_mut(&driver_name) {
                driver.load_status = DriverLoadStatus::Loaded;
            }
        }
        
        // Update device tree
        self.build_device_tree()?;
        
        Ok(())
    }

    pub fn list_devices(&self) -> Vec<&DeviceInfo> {
        self.devices.values().collect()
    }

    pub fn get_device(&self, name: &str) -> Option<&DeviceInfo> {
        self.devices.get(name)
    }

    pub fn list_drivers(&self) -> Vec<&DriverInfo> {
        self.drivers.values().collect()
    }

    pub fn get_driver(&self, name: &str) -> Option<&DriverInfo> {
        self.drivers.get(name)
    }

    pub fn enable_hotplug(&mut self) {
        self.hotplug_enabled = true;
        println!("Hotplug enabled.");
    }

    pub fn disable_hotplug(&mut self) {
        self.hotplug_enabled = false;
        println!("Hotplug disabled.");
    }

    pub fn generate_device_report(&self) -> String {
        let mut report = String::new();
        report.push_str("SigmaOS Hardware Report\n");
        report.push_str("======================\n\n");
        
        report.push_str(&format!("Total Devices: {}\n", self.devices.len()));
        report.push_str(&format!("Total Drivers: {}\n", self.drivers.len()));
        report.push_str(&format!("Hotplug: {}\n\n", if self.hotplug_enabled { "Enabled" } else { "Disabled" }));
        
        report.push_str("Devices:\n");
        for device in self.devices.values() {
            report.push_str(&format!("  - {} ({:?}): {}\n", 
                device.name, device.device_type, device.description));
            report.push_str(&format!("    Status: {:?}\n", device.status));
            if let Some(driver) = &device.driver {
                report.push_str(&format!("    Driver: {}\n", driver));
            }
            report.push('\n');
        }
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_manager_creation() {
        let manager = HardwareManager::new();
        assert_eq!(manager.devices.len(), 0);
        assert_eq!(manager.drivers.len(), 0);
    }

    #[test]
    fn test_device_addition() {
        let mut manager = HardwareManager::new();
        let device = DeviceInfo {
            device_type: DeviceType::PCI,
            vendor_id: 0x8086,
            device_id: 0x1af4,
            class_id: 0x01,
            subclass_id: 0x06,
            prog_if: 0x01,
            name: "test_device".to_string(),
            description: "Test Device".to_string(),
            driver: None,
            status: DeviceStatus::Connected,
            resources: DeviceResources {
                io_ports: vec![],
                memory_ranges: vec![],
                irq: None,
                dma_channel: None,
            },
        };
        
        manager.add_device(device);
        assert_eq!(manager.devices.len(), 1);
    }

    #[test]
    fn test_driver_registration() {
        let mut manager = HardwareManager::new();
        let driver = DriverInfo {
            name: "test_driver".to_string(),
            version: "1.0.0".to_string(),
            supported_devices: vec![(0x8086, 0x1af4)],
            supported_classes: vec![],
            priority: 100,
            load_status: DriverLoadStatus::NotLoaded,
        };
        
        manager.register_driver(driver);
        assert_eq!(manager.drivers.len(), 1);
    }
}