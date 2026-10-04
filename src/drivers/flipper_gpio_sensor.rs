#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// Flipper Devices style GPIO sensor and TRNG (True Random Number Generator) driver
use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

pub struct FlipperGpioSensor {
    is_initialized: bool,
    power_state: PowerState,
}

impl FlipperGpioSensor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
        }
    }
}

impl PeripheralDevice for FlipperGpioSensor {
    fn name(&self) -> &'static str {
        "Flipper GPIO Multi-Sensor TRNG Driver"
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

        // Simulate reading True Random bytes or sensor signals
        for byte in buffer.iter_mut() {
            *byte = 42; // standard sensor value representation
        }
        Ok(buffer.len())
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("Device not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("Device is offline");
        }

        // Writing commands to GPIO multiplexers
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
    fn test_flipper_lifecycle() {
        let mut driver = FlipperGpioSensor::new();
        driver.initialize().unwrap();
        assert_eq!(driver.name(), "Flipper GPIO Multi-Sensor TRNG Driver");
        assert_eq!(driver.generation(), DeviceGeneration::Modern);
        assert_eq!(driver.write(&[0xFF]).unwrap(), 1);
        driver.shutdown().unwrap();
    }
}
