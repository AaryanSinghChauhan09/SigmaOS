// # SigmaOS Sovereign Driver SDK
//
// Inspired by Linux's dynamic device driver framework (e.g. platform_driver, pci_driver),
// this SDK provides unified abstractions for device resource management (MMIO, IRQs),
// safe DMA-coherent buffer mappings, and dynamic driver-to-device probe matching.

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// Standard hardware resource classification matching Linux's struct resource flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    MemoryMappedIo, // MMIO region
    IoPort,         // Legacy I/O port address space
    InterruptLine,  // IRQ line index
}

/// Models physical resources allocated to a hardware device (e.g. BARs, IRQs)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceResource {
    pub resource_type: ResourceType,
    pub start: u64,
    pub length: u64,
    pub name: String,
}

impl DeviceResource {
    pub fn new(resource_type: ResourceType, start: u64, length: u64, name: &str) -> Self {
        Self {
            resource_type,
            start,
            length,
            name: String::from(name),
        }
    }
}

/// Unified hardware device descriptor matching standard Linux struct device
#[derive(Debug, Clone)]
pub struct SovereignDevice {
    pub id: usize,
    pub name: String,
    pub bus_type: String,
    pub vendor_id: u16,
    pub device_id: u16,
    pub resources: Vec<DeviceResource>,
}

impl SovereignDevice {
    pub fn new(id: usize, name: &str, bus: &str, vendor: u16, device: u16) -> Self {
        Self {
            id,
            name: String::from(name),
            bus_type: String::from(bus),
            vendor_id: vendor,
            device_id: device,
            resources: Vec::new(),
        }
    }

    pub fn add_resource(&mut self, res: DeviceResource) {
        self.resources.push(res);
    }
}

/// Standard driver power states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverPowerState {
    Active,
    Suspended,
    Off,
}

/// Standard, unified driver driver trait matching Linux pci_driver or platform_driver callbacks
pub trait SovereignDriver {
    /// Friendly identifier for the driver
    fn name(&self) -> &str;

    /// Checks if this driver supports and can probe/claim the target hardware device
    fn matches(&self, device: &SovereignDevice) -> bool;

    /// Initializes and binds the hardware device to the driver
    fn probe(&mut self, device: &SovereignDevice) -> Result<(), &'static str>;

    /// Transitions driver state to low-power suspend mode
    fn suspend(&mut self) -> Result<(), &'static str>;

    /// Transitions driver state back to fully active mode
    fn resume(&mut self) -> Result<(), &'static str>;

    /// Binds down and shuts the driver and device down completely
    fn remove(&mut self) -> Result<(), &'static str>;
}

/// Represents safe, contiguous DMA coherent buffers matching Linux's dma_alloc_coherent
pub struct DmaCoherentBuffer {
    pub physical_address: u64,
    pub length_bytes: usize,
    pub buffer: Vec<u8>,
}

impl DmaCoherentBuffer {
    pub fn allocate_coherent(physical_address: u64, length: usize) -> Self {
        let mut buffer = Vec::with_capacity(length);
        buffer.resize(length, 0u8);
        Self {
            physical_address,
            length_bytes: length,
            buffer,
        }
    }

    pub fn write_coherent(&mut self, offset: usize, data: &[u8]) -> Result<(), &'static str> {
        if offset + data.len() > self.length_bytes {
            return Err("DMA buffer boundary overflow write attempt");
        }
        self.buffer[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }

    pub fn read_coherent(&self, offset: usize, buffer: &mut [u8]) -> Result<(), &'static str> {
        if offset + buffer.len() > self.length_bytes {
            return Err("DMA buffer boundary overflow read attempt");
        }
        buffer.copy_from_slice(&self.buffer[offset..offset + buffer.len()]);
        Ok(())
    }
}

/// Dynamic SDK driver registry matching Linux driver matchmaker bus loops
pub struct SovereignDriverRegistry {
    pub registered_drivers: Vec<Box<dyn SovereignDriver>>,
    pub probed_devices: BTreeMap<usize, String>, // Device ID -> Driver Name
}

impl SovereignDriverRegistry {
    pub fn new() -> Self {
        Self {
            registered_drivers: Vec::new(),
            probed_devices: BTreeMap::new(),
        }
    }

    /// Register a sovereign device driver
    pub fn register_driver(&mut self, driver: Box<dyn SovereignDriver>) {
        self.registered_drivers.push(driver);
    }

    /// Automatically iterates over drivers, matchmaking and probing the device (Linux Bus Matchmaker loop)
    pub fn probe_device(&mut self, device: &SovereignDevice) -> Result<String, &'static str> {
        for driver in &mut self.registered_drivers {
            if driver.matches(device) {
                driver.probe(device)?;
                self.probed_devices.insert(device.id, String::from(driver.name()));
                return Ok(String::from(driver.name()));
            }
        }
        Err("No matching sovereign driver found for this device ID")
    }

    /// Safely shuts down and unbinds a device
    pub fn unbind_device(&mut self, device: &SovereignDevice) -> Result<(), &'static str> {
        if let Some(driver_name) = self.probed_devices.remove(&device.id) {
            if let Some(driver) = self.registered_drivers.iter_mut().find(|d| d.name() == driver_name) {
                driver.remove()?;
                return Ok(());
            }
        }
        Err("Device is not bound to any registered driver")
    }
}

impl Default for SovereignDriverRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Simple test implementation of an e1000 PCI Network driver
    struct E1000Driver {
        name: String,
        state: DriverPowerState,
        is_probed: bool,
    }

    impl E1000Driver {
        pub fn new() -> Self {
            Self {
                name: String::from("Sovereign-E1000-Nic"),
                state: DriverPowerState::Off,
                is_probed: false,
            }
        }
    }

    impl SovereignDriver for E1000Driver {
        fn name(&self) -> &str {
            &self.name
        }

        fn matches(&self, device: &SovereignDevice) -> bool {
            device.vendor_id == 0x8086 && device.device_id == 0x100E // e1000 PCI vendor/device ID
        }

        fn probe(&mut self, device: &SovereignDevice) -> Result<(), &'static str> {
            let _ = device;
            self.is_probed = true;
            self.state = DriverPowerState::Active;
            Ok(())
        }

        fn suspend(&mut self) -> Result<(), &'static str> {
            self.state = DriverPowerState::Suspended;
            Ok(())
        }

        fn resume(&mut self) -> Result<(), &'static str> {
            self.state = DriverPowerState::Active;
            Ok(())
        }

        fn remove(&mut self) -> Result<(), &'static str> {
            self.is_probed = false;
            self.state = DriverPowerState::Off;
            Ok(())
        }
    }

    #[test]
    fn test_device_resources() {
        let mut device = SovereignDevice::new(1, "Intel Gigabit Ethernet", "PCI", 0x8086, 0x100E);

        let bar0 = DeviceResource::new(ResourceType::MemoryMappedIo, 0xFEB00000, 128 * 1024, "bar0_mmio");
        let irq = DeviceResource::new(ResourceType::InterruptLine, 11, 1, "e1000_irq");

        device.add_resource(bar0);
        device.add_resource(irq);

        assert_eq!(device.resources.len(), 2);
        assert_eq!(device.resources[0].resource_type, ResourceType::MemoryMappedIo);
        assert_eq!(device.resources[1].start, 11);
    }

    #[test]
    fn test_driver_matchmaking_and_probing() {
        let mut registry = SovereignDriverRegistry::new();
        let driver = Box::new(E1000Driver::new());
        registry.register_driver(driver);

        let device = SovereignDevice::new(42, "Standard e1000 card", "PCI", 0x8086, 0x100E);

        // Run matching bus probe
        let bound_driver = registry.probe_device(&device).unwrap();
        assert_eq!(bound_driver, "Sovereign-E1000-Nic");
        assert_eq!(registry.probed_devices.get(&42).unwrap(), "Sovereign-E1000-Nic");

        // Unbind the device
        assert!(registry.unbind_device(&device).is_ok());
        assert!(registry.probed_devices.get(&42).is_none());
    }

    #[test]
    fn test_dma_coherent_allocations() {
        // Allocate a 4 KB coherent page
        let mut dma_page = DmaCoherentBuffer::allocate_coherent(0x3F000000, 4096);
        assert_eq!(dma_page.physical_address, 0x3F000000);

        // Write to DMA page
        let test_packet = [0xAA, 0xBB, 0xCC, 0xDD];
        assert!(dma_page.write_coherent(128, &test_packet).is_ok());

        // Read back from DMA page
        let mut read_back = [0u8; 4];
        assert!(dma_page.read_coherent(128, &mut read_back).is_ok());
        assert_eq!(read_back, test_packet);

        // Attempt writing out of bounds
        assert!(dma_page.write_coherent(4095, &test_packet).is_err());
    }
}
