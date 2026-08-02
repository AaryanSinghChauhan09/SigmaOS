#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Unified Peripheral Device Architecture
// Implements OOP principles for robust, low footprint device management

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;

/// Defines the generation of a peripheral device
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceGeneration {
    /// Older generation devices (e.g., PS/2, Serial, legacy ISA)
    Legacy,
    /// Modern generation devices (e.g., USB 3.0, PCIe)
    Modern,
    /// Unknown or generic fallback
    Unknown,
}

/// Current power state of the peripheral
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    On,
    Sleep,
    Off,
}

/// Unified Peripheral Device Trait
/// Any connected peripheral must implement this trait regardless of its generation.
/// This fulfills the OOP principles of abstraction and polymorphism.
pub trait PeripheralDevice {
    /// Returns the name or identifier of the device
    fn name(&self) -> &'static str;

    /// Returns the generation category of the device
    fn generation(&self) -> DeviceGeneration;

    /// Initializes the device, preparing it for I/O operations
    fn initialize(&mut self) -> Result<(), &'static str>;

    /// Reads data from the device into the buffer
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str>;

    /// Writes data to the device from the buffer
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str>;

    /// Sets the power state of the device to optimize energy consumption
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str>;

    /// Gracefully shuts down the device
    fn shutdown(&mut self) -> Result<(), &'static str>;
}

/// Centralized manager for peripheral devices.
/// Uses dynamic dispatch (`Box<dyn PeripheralDevice>`) to drastically reduce disk space
/// and kernel binary size by preventing excessive generic monomorphization.
pub struct PeripheralManager {
    devices: Vec<Box<dyn PeripheralDevice>>,
}

impl PeripheralManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    /// Registers a new peripheral device into the system.
    pub fn register_device(
        &mut self,
        mut device: Box<dyn PeripheralDevice>,
    ) -> Result<(), &'static str> {
        device.initialize()?;
        self.devices.push(device);
        Ok(())
    }

    /// Iterates over all devices and transitions them to a specific power state.
    pub fn broadcast_power_state(&mut self, state: PowerState) {
        for device in self.devices.iter_mut() {
            let _ = device.set_power_state(state);
        }
    }

    /// Returns the number of active managed devices.
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}

impl Default for PeripheralManager {
    fn default() -> Self {
        Self::new()
    }
}
