// Legacy AC97 Audio codec driver simulator
#[cfg(not(test))]
use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceGeneration { Legacy, Modern }

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState { Off, On }

#[cfg(test)]
pub trait PeripheralDevice {
    fn name(&self) -> &'static str;
    fn generation(&self) -> DeviceGeneration;
    fn initialize(&mut self) -> Result<(), &'static str>;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str>;
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str>;
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str>;
    fn shutdown(&mut self) -> Result<(), &'static str>;
}

pub struct LegacyAudioAc97 {
    is_initialized: bool,
    power_state: PowerState,
}

impl LegacyAudioAc97 {
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
        }
    }
}

impl PeripheralDevice for LegacyAudioAc97 {
    fn name(&self) -> &'static str {
        "AC97 Legacy Sound Blaster Driver"
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
        if !self.is_initialized {
            return Err("Device not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("Device is offline");
        }
        Ok(0) // legacy sound capture unsupported in fallback mode
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("Device not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("Device is offline");
        }
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
    fn test_ac97_lifecycle() {
        let mut driver = LegacyAudioAc97::new();
        driver.initialize().unwrap();
        assert_eq!(driver.name(), "AC97 Legacy Sound Blaster Driver");
        assert_eq!(driver.generation(), DeviceGeneration::Legacy);
        assert_eq!(driver.write(&[0]).unwrap(), 1);
        driver.shutdown().unwrap();
    }
}
