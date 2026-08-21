// Modern Wireless Network Interface Driver
// Demonstrates how advanced modern wireless drivers implement the unified OOP architecture.
// Enhanced with Linux/BSD-inspired wireless properties, security, scanning, and monitor mode support.

#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[cfg(not(test))]
use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceGeneration {
    Legacy,
    Modern,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    Off,
    On,
}

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

/// Linux/BSD Wireless Security/Encryption Standard Options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WirelessSecurity {
    Open,
    Wep,
    Wpa2Psk,
    Wpa2Enterprise,
    Wpa3Sae, // Modern SAE security
}

/// Discovered Access Point Representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessPoint {
    pub ssid: String,
    pub bssid: [u8; 6],
    pub channel: u8,
    pub rssi_dbm: i8,
    pub security: WirelessSecurity,
}

pub struct ModernWifiDriver {
    is_initialized: bool,
    power_state: PowerState,
    connected_ssid: Option<String>,

    // Linux/BSD wireless parameters
    pub current_security: WirelessSecurity,
    pub channel: u8,
    pub rssi_dbm: i8,
    pub tx_power_dbm: i8,
    pub active_scanning: bool,
    pub monitor_mode_enabled: bool, // Linux rfmon / mac80211 monitor mode
}

impl ModernWifiDriver {
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
            connected_ssid: None,
            current_security: WirelessSecurity::Open,
            channel: 1,
            rssi_dbm: -127,   // Unconnected / extremely weak default
            tx_power_dbm: 20, // 20 dBm (100mW) standard TX power
            active_scanning: true,
            monitor_mode_enabled: false,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected_ssid.is_some()
    }

    pub fn get_connected_ssid(&self) -> Option<&str> {
        self.connected_ssid.as_deref()
    }

    /// Set mock connected state (for compatibility)
    pub fn set_connected(&mut self, connected: bool) {
        if connected {
            self.connected_ssid = Some("SigmaOS_Wifi_6".to_string());
            self.rssi_dbm = -50; // Good signal strength
        } else {
            self.connected_ssid = None;
            self.rssi_dbm = -127;
        }
    }

    /// Set dynamic wireless channel (e.g. 1-14 for 2.4GHz, 36-165 for 5GHz)
    pub fn set_channel(&mut self, new_channel: u8) -> Result<(), &'static str> {
        if (new_channel >= 1 && new_channel <= 14) || (new_channel >= 36 && new_channel <= 165) {
            self.channel = new_channel;
            Ok(())
        } else {
            Err("802.11: Invalid Wi-Fi channel frequency")
        }
    }

    /// Set transmission power in dBm
    pub fn set_tx_power(&mut self, dbm: i8) -> Result<(), &'static str> {
        if dbm < -10 || dbm > 30 {
            return Err("802.11: TX power out of safe operational limit (-10 to 30 dBm)");
        }
        self.tx_power_dbm = dbm;
        Ok(())
    }

    /// Enable or disable monitor mode (Linux mac80211 `rfmon` capability)
    pub fn set_monitor_mode(&mut self, enabled: bool) {
        self.monitor_mode_enabled = enabled;
        if enabled {
            self.connected_ssid = None; // Disassociate when putting card into monitor mode
        }
    }

    /// Simulate active scanning for local BSSIDs
    pub fn scan_networks(&self) -> Vec<AccessPoint> {
        if self.power_state != PowerState::On || !self.is_initialized {
            return Vec::new();
        }

        // Return mock high-fidelity access points discovered
        let mut aps = Vec::new();
        aps.push(AccessPoint {
            ssid: "SigmaOS_Wifi_6".to_string(),
            bssid: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            channel: 6,
            rssi_dbm: -45,
            security: WirelessSecurity::Wpa3Sae,
        });
        aps.push(AccessPoint {
            ssid: "Ubuntu_Guest".to_string(),
            bssid: [0x00, 0x11, 0x22, 0x33, 0x44, 0xAA],
            channel: 11,
            rssi_dbm: -68,
            security: WirelessSecurity::Wpa2Psk,
        });
        aps.push(AccessPoint {
            ssid: "BSD_Secure_Corp".to_string(),
            bssid: [0x00, 0x11, 0x22, 0x33, 0x44, 0xFF],
            channel: 36,
            rssi_dbm: -55,
            security: WirelessSecurity::Wpa2Enterprise,
        });
        aps.push(AccessPoint {
            ssid: "Coffee_Shop_Free".to_string(),
            bssid: [0x00, 0x11, 0x22, 0x33, 0x44, 0x99],
            channel: 1,
            rssi_dbm: -80,
            security: WirelessSecurity::Open,
        });
        aps
    }

    /// Associate to a Wi-Fi Access Point with safety parameters
    pub fn associate(
        &mut self,
        ssid: &str,
        password: Option<&str>,
        security: WirelessSecurity,
    ) -> Result<(), &'static str> {
        if !self.is_initialized {
            return Err("802.11: Wi-Fi card must be initialized first");
        }
        if self.power_state != PowerState::On {
            return Err("802.11: Cannot associate while radio transceiver is powered off");
        }
        if self.monitor_mode_enabled {
            return Err("802.11: Cannot associate while in monitor mode");
        }

        // Enforce basic passphrase parameters
        match security {
            WirelessSecurity::Wpa2Psk | WirelessSecurity::Wpa3Sae => {
                if let Some(pass) = password {
                    if pass.len() < 8 {
                        return Err("802.11: Security configuration error: passphrase too short");
                    }
                } else {
                    return Err("802.11: Passphrase required for secure network association");
                }
            }
            WirelessSecurity::Wpa2Enterprise => {
                if password.is_none() {
                    return Err("802.11: Missing credentials for WPA2-Enterprise");
                }
            }
            WirelessSecurity::Wep => {
                if let Some(pass) = password {
                    if pass.len() != 5 && pass.len() != 13 {
                        return Err("802.11: WEP key must be 5 or 13 characters");
                    }
                } else {
                    return Err("802.11: WEP key is required");
                }
            }
            WirelessSecurity::Open => {}
        }

        self.connected_ssid = Some(ssid.to_string());
        self.current_security = security;
        self.rssi_dbm = -55; // Connected signal strength
        Ok(())
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
        self.connected_ssid = None;
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

    #[test]
    fn test_linux_bsd_wireless_parameters() {
        let mut wifi = ModernWifiDriver::new();

        // 1. Channel management
        assert_eq!(wifi.channel, 1);
        wifi.set_channel(6).unwrap();
        assert_eq!(wifi.channel, 6);
        wifi.set_channel(36).unwrap(); // 5GHz
        assert_eq!(wifi.channel, 36);
        assert!(wifi.set_channel(25).is_err()); // Invalid channel

        // 2. Transmit Power management
        assert_eq!(wifi.tx_power_dbm, 20);
        wifi.set_tx_power(15).unwrap();
        assert_eq!(wifi.tx_power_dbm, 15);
        assert!(wifi.set_tx_power(45).is_err()); // Unsafe

        // 3. Monitor mode toggle
        assert!(!wifi.monitor_mode_enabled);
        wifi.set_connected(true);
        wifi.set_monitor_mode(true);
        assert!(wifi.monitor_mode_enabled);
        assert!(!wifi.is_connected()); // Put into monitor mode disassociates active APs
    }

    #[test]
    fn test_wireless_scanning_and_association() {
        let mut wifi = ModernWifiDriver::new();
        wifi.initialize().unwrap();

        // 1. Scan results
        let aps = wifi.scan_networks();
        assert_eq!(aps.len(), 4);
        assert_eq!(aps[0].ssid, "SigmaOS_Wifi_6");
        assert_eq!(aps[0].security, WirelessSecurity::Wpa3Sae);
        assert_eq!(aps[0].rssi_dbm, -45);

        // 2. Association flow
        assert!(!wifi.is_connected());
        // Fail because of short password
        assert!(wifi
            .associate("SigmaOS_Wifi_6", Some("short"), WirelessSecurity::Wpa3Sae)
            .is_err());
        assert!(!wifi.is_connected());

        // Succeed association
        wifi.associate(
            "SigmaOS_Wifi_6",
            Some("strong_wpa3_password"),
            WirelessSecurity::Wpa3Sae,
        )
        .unwrap();
        assert!(wifi.is_connected());
        assert_eq!(wifi.get_connected_ssid(), Some("SigmaOS_Wifi_6"));
        assert_eq!(wifi.current_security, WirelessSecurity::Wpa3Sae);
        assert_eq!(wifi.rssi_dbm, -55);
    }
}
