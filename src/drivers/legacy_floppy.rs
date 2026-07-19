// Legacy floppy disk controller simulator driver
use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

pub struct LegacyFloppyDriver {
    is_initialized: bool,
    power_state: PowerState,
}

impl LegacyFloppyDriver {
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
        }
    }
}

impl PeripheralDevice for LegacyFloppyDriver {
    fn name(&self) -> &'static str {
        "3.5-inch Legacy Floppy Disk Controller"
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

        // Simulate reading slow magnetic sectors (e.g. all zeroes)
        for byte in buffer.iter_mut() {
            *byte = 0;
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

        // Simulate slow write sectors
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
    fn test_floppy_lifecycle() {
        let mut driver = LegacyFloppyDriver::new();
        assert!(driver.read(&mut [0; 10]).is_err());
        driver.initialize().unwrap();
        assert_eq!(driver.name(), "3.5-inch Legacy Floppy Disk Controller");
        assert_eq!(driver.generation(), DeviceGeneration::Legacy);
        assert_eq!(driver.write(&[0, 1]).unwrap(), 2);
        driver.shutdown().unwrap();
    }
}
