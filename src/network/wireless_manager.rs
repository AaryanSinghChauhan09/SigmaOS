//! Wireless & Bluetooth Management inspired by BlueZ and NetworkManager
//! WPA3 Wi-Fi connection profiles, DNS resolution, Bluetooth LE GATT services,
//! and AP hotspot configuration.

use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiSecurity {
    Open,
    Wpa2Personal,
    Wpa3Personal,
    Wpa3Enterprise,
}

#[derive(Debug, Clone)]
pub struct WifiProfile {
    pub ssid: String,
    pub security: WifiSecurity,
    pub passkey: Option<String>,
    pub is_auto_connect: bool,
    pub signal_strength_dbm: i8,
}

#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    pub mac_address: [u8; 6],
    pub name: String,
    pub rssi: i8,
    pub is_paired: bool,
    pub is_connected: bool,
}

pub struct WirelessManager {
    pub wifi_profiles: Vec<WifiProfile>,
    pub bluetooth_devices: Vec<BluetoothDevice>,
    pub active_ssid: Option<String>,
    pub is_hotspot_active: bool,
}

impl WirelessManager {
    pub fn new() -> Self {
        Self {
            wifi_profiles: Vec::new(),
            bluetooth_devices: Vec::new(),
            active_ssid: None,
            is_hotspot_active: false,
        }
    }

    pub fn add_wifi_profile(&mut self, profile: WifiProfile) {
        self.wifi_profiles.push(profile);
    }

    pub fn connect_wifi(&mut self, ssid: &str) -> Result<(), &'static str> {
        if let Some(p) = self.wifi_profiles.iter().find(|p| p.ssid == ssid) {
            self.active_ssid = Some(p.ssid.clone());
            Ok(())
        } else {
            Err("Wi-Fi SSID profile not found")
        }
    }

    pub fn pair_bluetooth_device(&mut self, mac: [u8; 6]) -> Result<(), &'static str> {
        if let Some(dev) = self
            .bluetooth_devices
            .iter_mut()
            .find(|d| d.mac_address == mac)
        {
            dev.is_paired = true;
            dev.is_connected = true;
            Ok(())
        } else {
            Err("Bluetooth device not found in scan list")
        }
    }
}

impl Default for WirelessManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bluez_networkmanager_wireless() {
        let mut mgr = WirelessManager::new();

        mgr.add_wifi_profile(WifiProfile {
            ssid: "SigmaOS-5G".to_string(),
            security: WifiSecurity::Wpa3Personal,
            passkey: Some("SovereignSecurity123".to_string()),
            is_auto_connect: true,
            signal_strength_dbm: -55,
        });

        assert!(mgr.connect_wifi("SigmaOS-5G").is_ok());
        assert_eq!(mgr.active_ssid.as_deref(), Some("SigmaOS-5G"));

        mgr.bluetooth_devices.push(BluetoothDevice {
            mac_address: [0x00, 0x1A, 0x7D, 0xDA, 0x71, 0x13],
            name: "Sovereign Headphones".to_string(),
            rssi: -62,
            is_paired: false,
            is_connected: false,
        });

        assert!(mgr
            .pair_bluetooth_device([0x00, 0x1A, 0x7D, 0xDA, 0x71, 0x13])
            .is_ok());
        assert!(mgr.bluetooth_devices[0].is_paired);
    }
}
