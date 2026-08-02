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

// Modern USB Controller Implementation (e.g., xHCI / USB 3.0)
// Demonstrates modern device handling using the same unified OOP architecture.

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

pub struct ModernUsbController {
    is_initialized: bool,
    power_state: PowerState,
    buffer: [u8; 64], // Simulated fast DMA buffer
}

impl Default for ModernUsbController {
    fn default() -> Self {
        Self::new()
    }
}

impl ModernUsbController {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
            buffer: [0; 64],
        }
    }
}

impl PeripheralDevice for ModernUsbController {
    fn name(&self) -> &'static str {
        "xHCI USB 3.0 Controller"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        // Here, one would traditionally map PCIe base address registers and set up rings
        self.is_initialized = true;
        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("Device not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("Device is sleeping or off");
        }

        // High-speed block read simulation
        let len = core::cmp::min(buffer.len(), self.buffer.len());
        buffer[..len].copy_from_slice(&self.buffer[..len]);
        Ok(len)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("Device not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("Device is sleeping or off");
        }

        let len = core::cmp::min(data.len(), self.buffer.len());
        self.buffer[..len].copy_from_slice(&data[..len]);
        Ok(len)
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        // Modern devices have rich ACPI/PCIe power management (D0, D3hot, etc.)
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.is_initialized = false;
        self.power_state = PowerState::Off;
        Ok(())
    }
}
