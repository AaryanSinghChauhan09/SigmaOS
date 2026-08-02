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

// Modern Wireless Network Interface Driver
// Demonstrates how advanced modern wireless drivers implement the unified OOP architecture.

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

pub struct ModernWifiDriver {
    is_initialized: bool,
    power_state: PowerState,
    connected_ssid: bool,
}

impl ModernWifiDriver {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
            connected_ssid: false,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected_ssid
    }

    pub fn set_connected(&mut self, connected: bool) {
        self.connected_ssid = connected;
    }
}

impl PeripheralDevice for ModernWifiDriver {
    fn name(&self) -> &'static str {
        "802.11 Modern Wireless Driver"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        // Prepare descriptors, DMA queues, and load proprietary firmware blobs safely
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

        // Simulate reading wireless network packet payload
        if !buffer.is_empty() {
            buffer[0] = 0xAA; // Simulated start of frame byte
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

        // Simulate sending packet payload over DMA
        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.is_initialized = false;
        self.power_state = PowerState::Off;
        self.connected_ssid = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modern_wifi_creation() {
        let mut wifi = ModernWifiDriver::new();
        assert!(!wifi.is_connected());
        wifi.set_connected(true);
        assert!(wifi.is_connected());
    }

    #[test]
    fn test_modern_wifi_read_write() {
        let mut wifi = ModernWifiDriver::new();
        let mut buf = [0; 10];
        // Must fail before initialize
        assert!(wifi.read(&mut buf).is_err());

        wifi.initialize().unwrap();
        let bytes_read = wifi.read(&mut buf).unwrap();
        assert_eq!(bytes_read, 1);
        assert_eq!(buf[0], 0xAA);

        let bytes_written = wifi.write(b"PACKET").unwrap();
        assert_eq!(bytes_written, 6);
    }
}
