// Legacy Serial Port (UART 16550A) Driver
// Implements unified OOP peripheral interface for ancient communication terminals.

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice as PeripheralDeviceTrait, PowerState};

/// Represents an ancient 16550 UART Serial Port (e.g., COM1 at 0x3F8, COM2 at 0x2F8)
#[allow(dead_code)]
pub struct LegacySerialPort {
    base_port: u16,
    is_initialized: bool,
    power_state: PowerState,
    baud_rate: u32,
    device_id: u32,
}

impl LegacySerialPort {
    /// Creates a new LegacySerialPort instance
    pub fn new(base_port: u16) -> Self {
        Self {
            base_port,
            is_initialized: false,
            power_state: PowerState::Off,
            baud_rate: 9600,
            device_id: 2,
        }
    }

    /// Set baud rate by divisor (simulating DLAB logic)
    pub fn set_baud_rate(&mut self, baud_rate: u32) {
        self.baud_rate = baud_rate;
    }
}

impl PeripheralDeviceTrait for LegacySerialPort {
    fn device_id(&self) -> u32 {
        self.device_id
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }

    fn power_state(&self) -> PowerState {
        self.power_state
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }
}
