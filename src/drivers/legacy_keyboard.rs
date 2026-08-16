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

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice as PeripheralDeviceTrait, PowerState};

pub struct LegacyKeyboard {
    is_initialized: bool,
    power_state: PowerState,
    device_id: u32,
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
            device_id: 1,
        }
    }
}

impl PeripheralDeviceTrait for LegacyKeyboard {
    fn name(&self) -> &'static str {
        "Legacy PS/2 Keyboard"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.is_initialized = true;
        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, _buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::Off;
        self.is_initialized = false;
        Ok(())
    }
}
