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

// Legacy Keyboard Implementation (e.g., PS/2)
// Demonstrates how user-defined drivers implement the unified OOP architecture.

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

pub struct LegacyKeyboard {
    is_initialized: bool,
    power_state: PowerState,
}

impl Default for LegacyKeyboard {
    fn default() -> Self {
        Self::new()
    }
}

impl LegacyKeyboard {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
        }
    }
}

impl PeripheralDevice for LegacyKeyboard {
    fn name(&self) -> &'static str {
        "PS/2 Legacy Keyboard"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        // Here, one would traditionally map I/O ports 0x60 and 0x64
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

        // Dummy read: simulate reading a scancode
        if !buffer.is_empty() {
            buffer[0] = 0x1E; // Scancode for 'A'
            Ok(1)
        } else {
            Ok(0)
        }
    }

    fn write(&mut self, _data: &[u8]) -> Result<usize, &'static str> {
        // Writing to a legacy keyboard is rare (maybe setting LEDs)
        Ok(0)
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        // Legacy devices might not fully support sleep, but we simulate it
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.is_initialized = false;
        self.power_state = PowerState::Off;
        Ok(())
    }
}
