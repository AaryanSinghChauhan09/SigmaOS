// Legacy Serial Port (UART 16550A) Driver
// Implements unified OOP peripheral interface for ancient communication terminals.

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

/// Represents an ancient 16550 UART Serial Port (e.g., COM1 at 0x3F8, COM2 at 0x2F8)
pub struct LegacySerialPort {
    base_port: u16,
    is_initialized: bool,
    power_state: PowerState,
    baud_rate: u32,
}

impl LegacySerialPort {
    /// Creates a new LegacySerialPort instance
    pub fn new(base_port: u16) -> Self {
        Self {
            base_port,
            is_initialized: false,
            power_state: PowerState::Off,
            baud_rate: 9600,
        }
    }

    /// Set baud rate by divisor (simulating DLAB logic)
    pub fn set_baud_rate(&mut self, baud_rate: u32) {
        self.baud_rate = baud_rate;
    }
}

impl PeripheralDevice for LegacySerialPort {
    fn name(&self) -> &'static str {
        "UART 16550A Legacy Serial Port"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        // Traditional x86 16550 initialization sequence:
        // 1. Disable all interrupts
        // 2. Set DLAB (Divisor Latch Access Bit) to configure baud rate
        // 3. Set divisor low and high bytes
        // 4. Clear DLAB and configure 8 bits, no parity, one stop bit (8N1)
        // 5. Enable FIFO, clear them, and set receiver trigger to 14 bytes
        // 6. Enable IRQs, RTS/DSR
        self.is_initialized = true;
        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("Serial port: Device not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("Serial port: Device is sleeping or powered off");
        }

        if buffer.is_empty() {
            return Ok(0);
        }

        // Simulate reading a byte from the Receiver Buffer Register (RBR)
        // Returning ancient ASCII Carriage Return or diagnostic data
        buffer[0] = b'S'; // Diagnostic ASCII startup char
        Ok(1)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("Serial port: Device not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("Serial port: Device is sleeping or powered off");
        }

        // Simulate writing bytes to the Transmitter Holding Register (THR)
        // In real hardware, we would poll Line Status Register (LSR) bit 5 (Empty THR) and outb
        let mut bytes_written = 0;
        for &byte in data {
            // Simulated volatile write to ancient COM port
            let _ = byte;
            bytes_written += 1;
        }
        Ok(bytes_written)
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
    fn test_legacy_serial_oop() {
        let mut serial = LegacySerialPort::new(0x3F8);
        assert_eq!(serial.generation(), DeviceGeneration::Legacy);
        assert!(serial.initialize().is_ok());

        let mut read_buf = [0u8; 10];
        let bytes_read = serial.read(&mut read_buf).unwrap();
        assert_eq!(bytes_read, 1);
        assert_eq!(read_buf[0], b'S');

        let write_data = b"Hello, SigmaOS!";
        let bytes_written = serial.write(write_data).unwrap();
        assert_eq!(bytes_written, write_data.len());

        assert!(serial.set_power_state(PowerState::Sleep).is_ok());
        assert!(serial.read(&mut read_buf).is_err()); // cannot read when asleep

        assert!(serial.shutdown().is_ok());
    }
}
