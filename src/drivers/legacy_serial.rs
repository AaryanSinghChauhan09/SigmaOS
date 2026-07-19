// Legacy Serial Port Driver (e.g., COM1)
// Demonstrates how legacy serial interface drivers implement the unified OOP architecture.

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

pub struct LegacySerialPort {
    is_initialized: bool,
    power_state: PowerState,
    port_base: u16,
}

impl LegacySerialPort {
    pub fn new(port_base: u16) -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
            port_base,
        }
    }

    pub fn get_port_base(&self) -> u16 {
        self.port_base
    }
}

impl PeripheralDevice for LegacySerialPort {
    fn name(&self) -> &'static str {
        "COM Legacy Serial Port"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        // Traditionally, COM1 is at port 0x3F8. We configure baud rate and line controls here.
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

        // Simulate reading a character from UART data register
        if !buffer.is_empty() {
            buffer[0] = b'S'; // Simulated incoming telemetry character
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
            return Err("Device is sleeping or off");
        }

        // Simulate writing each character to the UART transmitter holding register
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
    fn test_legacy_serial_creation() {
        let mut serial = LegacySerialPort::new(0x3F8);
        assert_eq!(serial.get_port_base(), 0x3F8);
        assert!(serial.initialize().is_ok());
    }

    #[test]
    fn test_legacy_serial_read_write() {
        let mut serial = LegacySerialPort::new(0x3F8);
        let mut buf = [0; 10];
        // Must fail before initialize
        assert!(serial.read(&mut buf).is_err());

        serial.initialize().unwrap();
        let bytes_read = serial.read(&mut buf).unwrap();
        assert_eq!(bytes_read, 1);
        assert_eq!(buf[0], b'S');

        let bytes_written = serial.write(b"HELLO").unwrap();
        assert_eq!(bytes_written, 5);
    }
}
