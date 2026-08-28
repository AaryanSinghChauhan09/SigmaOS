extern crate alloc;
// SigmaOS Dial-up 56K Modem Driver (S-Modem)
// Zero-dependency, #![no_std] compliant, emulating iconic telephone AT commands.


use alloc::string::String;
use alloc::vec::Vec;
use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

pub struct DialupModemDriver {
    pub current_power: PowerState,
    pub is_connected: bool,
    pub baud_rate: u32,
    pub tx_buffer: Vec<u8>,
    pub rx_buffer: Vec<u8>,
}

impl DialupModemDriver {
    pub fn new() -> Self {
        Self {
            current_power: PowerState::Off,
            is_connected: false,
            baud_rate: 56000,
            tx_buffer: Vec::new(),
            rx_buffer: Vec::new(),
        }
    }

    pub fn power_state(&self) -> PowerState {
        self.current_power
    }
}

impl PeripheralDevice for DialupModemDriver {
    fn name(&self) -> &'static str {
        "U.S. Robotics 56K Dial-up Faxmodem"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy // Authentic ancient peripheral
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.current_power = PowerState::On;
        self.is_connected = false;
        self.tx_buffer.clear();
        self.rx_buffer.clear();
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        if self.is_connected {
            // Send hang-up command sequence (ATH)
            self.tx_buffer.extend_from_slice(b"+++ATH\r");
            self.is_connected = false;
        }
        self.current_power = PowerState::Off;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if self.current_power == PowerState::Off {
            return Err("Modem is powered off");
        }
        if self.rx_buffer.is_empty() {
            return Ok(0);
        }
        let len = core::cmp::min(buffer.len(), self.rx_buffer.len());
        buffer[..len].copy_from_slice(&self.rx_buffer[..len]);
        self.rx_buffer.drain(..len);
        Ok(len)
    }

    fn write(&mut self, buffer: &[u8]) -> Result<usize, &'static str> {
        if self.current_power == PowerState::Off {
            return Err("Modem is powered off");
        }
        self.tx_buffer.extend_from_slice(buffer);

        // Parse AT commands
        let cmd = String::from_utf8_lossy(&self.tx_buffer);
        if cmd.contains("ATDT") {
            // Dialing command detected!
            self.is_connected = true;
            self.rx_buffer.extend_from_slice(b"CONNECT 56000\r\n");
            self.tx_buffer.clear();
        } else if cmd.contains("ATH") {
            // Hang up command!
            self.is_connected = false;
            self.rx_buffer.extend_from_slice(b"NO CARRIER\r\n");
            self.tx_buffer.clear();
        } else if cmd.contains("ATZ") {
            // Reset modem!
            self.is_connected = false;
            self.rx_buffer.extend_from_slice(b"OK\r\n");
            self.tx_buffer.clear();
        }

        Ok(buffer.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.current_power = state;
        Ok(())
    }
}

impl Default for DialupModemDriver {
    fn default() -> Self {
        Self::new()
    }
}
