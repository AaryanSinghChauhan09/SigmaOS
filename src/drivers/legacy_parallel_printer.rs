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

// Legacy parallel port (Centronics LPT1) printer controller simulator
use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

pub struct LegacyParallelPrinter {
    is_initialized: bool,
    power_state: PowerState,
}

impl LegacyParallelPrinter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
        }
    }
}

impl PeripheralDevice for LegacyParallelPrinter {
    fn name(&self) -> &'static str {
        "Centronics LPT1 Parallel Printer"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
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

        // Parallel port printers are mostly write-only with basic status lines
        if !buffer.is_empty() {
            buffer[0] = 0xDF; // Status: Online, Selected, No Error
            Ok(1)
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

        // Simulate clock strobes transmitting characters bitwise
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
    fn test_parallel_printer_lifecycle() {
        let mut driver = LegacyParallelPrinter::new();
        driver.initialize().unwrap();
        assert_eq!(driver.name(), "Centronics LPT1 Parallel Printer");
        assert_eq!(driver.generation(), DeviceGeneration::Legacy);
        assert_eq!(driver.write(b"LPT1").unwrap(), 4);
        driver.shutdown().unwrap();
    }
}
