// Legacy Floppy Disk Controller (Intel 82077AA) Driver
// Implements unified OOP peripheral interface for ancient floppy diskette media (e.g., 3.5" 1.44MB floppy).

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

/// Represents an ancient Floppy Disk Controller (typically base I/O port 0x3F0)
pub struct LegacyFloppyDisk {
    base_port: u16,
    is_initialized: bool,
    power_state: PowerState,
    motor_on: bool,
    selected_drive: u8,
}

impl LegacyFloppyDisk {
    /// Creates a new LegacyFloppyDisk instance
    pub fn new(base_port: u16) -> Self {
        Self {
            base_port,
            is_initialized: false,
            power_state: PowerState::Off,
            motor_on: false,
            selected_drive: 0,
        }
    }

    /// Turn on/off the floppy drive spindle motor
    pub fn set_motor(&mut self, on: bool) {
        self.motor_on = on;
    }

    /// Select floppy drive (0-3)
    pub fn select_drive(&mut self, drive: u8) {
        self.selected_drive = drive & 0x03;
    }
}

impl PeripheralDevice for LegacyFloppyDisk {
    fn name(&self) -> &'static str {
        "Intel 82077AA Legacy Floppy Disk Controller"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        // Traditional floppy initialization:
        // 1. Reset the controller via Digital Output Register (DOR)
        // 2. Wait for IRQ 6 (floppy interrupt)
        // 3. Configure step rate, head unload time, head load time (SPECIFY command)
        // 4. Recalibrate drive head to cylinder 0 (RECALIBRATE command)
        self.is_initialized = true;
        self.power_state = PowerState::On;
        self.motor_on = false;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("Floppy: Device not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("Floppy: Device is asleep or powered off");
        }

        // Floppy operations require spindle motor to be on
        if !self.motor_on {
            return Err("Floppy: Spindle motor is off");
        }

        if buffer.is_empty() {
            return Ok(0);
        }

        // Simulate reading sectors from the ancient floppy media (1.44MB CHS format)
        // Filling the buffer with ancient diagnostics bytes
        let mut read_bytes = 0;
        for byte in buffer.iter_mut() {
            *byte = 0xE5; // Standard floppy format filler byte
            read_bytes += 1;
        }

        Ok(read_bytes)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("Floppy: Device not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("Floppy: Device is asleep or powered off");
        }
        if !self.motor_on {
            return Err("Floppy: Spindle motor is off");
        }

        // Simulate writing sectors to media
        let mut written_bytes = 0;
        for &byte in data {
            let _ = byte;
            written_bytes += 1;
        }

        Ok(written_bytes)
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        if state != PowerState::On {
            self.motor_on = false;
        }
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.motor_on = false;
        self.is_initialized = false;
        self.power_state = PowerState::Off;
        Ok(())
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_floppy_oop() {
        let mut floppy = LegacyFloppyDisk::new(0x3F0);
        assert_eq!(floppy.generation(), DeviceGeneration::Legacy);
        assert!(floppy.initialize().is_ok());

        let mut read_buf = [0u8; 512];
        // Must fail if motor is off
        assert!(floppy.read(&mut read_buf).is_err());

        floppy.set_motor(true);
        floppy.select_drive(1);
        assert_eq!(floppy.selected_drive, 1);

        let bytes_read = floppy.read(&mut read_buf).unwrap();
        assert_eq!(bytes_read, 512);
        assert_eq!(read_buf[0], 0xE5);

        let write_data = [0xAAu8; 256];
        let bytes_written = floppy.write(&write_data).unwrap();
        assert_eq!(bytes_written, 256);

        assert!(floppy.shutdown().is_ok());
    }
}
