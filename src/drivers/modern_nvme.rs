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

// Modern high-performance NVMe PCIe block storage driver
use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

pub struct ModernNvmeDriver {
    is_initialized: bool,
    power_state: PowerState,
    lba_count: u64,
}

impl ModernNvmeDriver {
    pub fn new(lba_count: u64) -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
            lba_count,
        }
    }

    pub fn get_lba_count(&self) -> u64 {
        self.lba_count
    }
}

impl PeripheralDevice for ModernNvmeDriver {
    fn name(&self) -> &'static str {
        "PCIe NVMe Solid-State Block Driver"
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

        // Simulate high-speed sequential sector read
        for (i, byte) in buffer.iter_mut().enumerate() {
            *byte = (i % 256) as u8;
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

        // Simulate high-speed PCIe block write
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
    fn test_nvme_lifecycle() {
        let mut driver = ModernNvmeDriver::new(2048);
        assert!(driver.read(&mut [0; 10]).is_err());
        driver.initialize().unwrap();
        assert_eq!(driver.name(), "PCIe NVMe Solid-State Block Driver");
        assert_eq!(driver.generation(), DeviceGeneration::Modern);
        assert_eq!(driver.write(&[1, 2, 3]).unwrap(), 3);
        driver.shutdown().unwrap();
    }
}
