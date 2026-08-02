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

// JingOS tablet-centric capacitive touchscreen gestural input driver simulator
use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

pub struct TouchJingosDriver {
    is_initialized: bool,
    power_state: PowerState,
}

impl TouchJingosDriver {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
        }
    }
}

impl PeripheralDevice for TouchJingosDriver {
    fn name(&self) -> &'static str {
        "JingOS Capacitive Gestural Touchscreen Driver"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.is_initialized = true;
        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("Device not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("Device is offline");
        }

        // Simulate reading multi-touch pinch coordinates (X, Y, and Pressure)
        if buffer.len() >= 4 {
            buffer[0] = 100; // X high
            buffer[1] = 50; // X low
            buffer[2] = 200; // Y high
            buffer[3] = 10; // Y low
            Ok(4)
        } else {
            Ok(0)
        }
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("Device not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("Device is offline");
        }

        // Write sensitivity configuration commands to controller
        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.is_initialized = false;
        self.power_state = PowerState::Off;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touch_lifecycle() {
        let mut driver = TouchJingosDriver::new();
        driver.initialize().unwrap();
        assert_eq!(
            driver.name(),
            "JingOS Capacitive Gestural Touchscreen Driver"
        );
        assert_eq!(driver.generation(), DeviceGeneration::Modern);
        assert_eq!(driver.write(&[0]).unwrap(), 1);
        driver.shutdown().unwrap();
    }
}
