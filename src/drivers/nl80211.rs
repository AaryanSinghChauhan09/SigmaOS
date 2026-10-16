// SigmaOS nl80211 / cfg80211 Wireless Subsystem Interface
// Provides generic wireless device control, scan triggering, and interface management over Netlink.

#![allow(dead_code)]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// nl80211 Netlink Commands
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nl80211Cmd {
    GetWiphy,
    SetWiphy,
    NewInterface,
    DelInterface,
    GetInterface,
    TriggerScan,
    GetScanResults,
    Authenticate,
    Associate,
    Deauthenticate,
    Disassociate,
    Connect,
    Disconnect,
}

/// Supported Wi-Fi Radio Bands
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiBand {
    Band2Ghz,
    Band5Ghz,
    Band6Ghz,
}

/// Wireless PHY Device Info (Wiphy)
#[derive(Debug, Clone)]
pub struct WiphyDevice {
    pub wiphy_idx: u32,
    pub name: String,
    pub max_scan_ssids: u8,
    pub supported_bands: Vec<WifiBand>,
    pub max_tx_power_dbm: i32,
    pub is_scanning: bool,
}

/// Wireless Network Interface (wlan0)
#[derive(Debug, Clone)]
pub struct WifiInterface {
    pub if_index: u32,
    pub wiphy_idx: u32,
    pub name: String,
    pub mac_address: [u8; 6],
    pub is_up: bool,
}

/// Nl80211 Wireless Manager Subsystem
#[derive(Debug, Clone, Default)]
pub struct Nl80211Subsystem {
    pub wiphys: Vec<WiphyDevice>,
    pub interfaces: Vec<WifiInterface>,
}

impl Nl80211Subsystem {
    pub fn new() -> Self {
        Self {
            wiphys: Vec::new(),
            interfaces: Vec::new(),
        }
    }

    /// Register a physical wireless card PHY
    pub fn register_wiphy(&mut self, name: &str, bands: &[WifiBand], max_tx: i32) -> u32 {
        let idx = self.wiphys.len() as u32;
        self.wiphys.push(WiphyDevice {
            wiphy_idx: idx,
            name: String::from(name),
            max_scan_ssids: 4,
            supported_bands: bands.to_vec(),
            max_tx_power_dbm: max_tx,
            is_scanning: false,
        });
        idx
    }

    /// Add a logical network interface on top of a Wiphy
    pub fn add_interface(
        &mut self,
        wiphy_idx: u32,
        if_name: &str,
        mac: [u8; 6],
    ) -> Result<u32, &'static str> {
        if wiphy_idx >= self.wiphys.len() as u32 {
            return Err("Invalid wiphy index");
        }
        let if_idx = self.interfaces.len() as u32;
        self.interfaces.push(WifiInterface {
            if_index: if_idx,
            wiphy_idx,
            name: String::from(if_name),
            mac_address: mac,
            is_up: true,
        });
        Ok(if_idx)
    }

    /// Dispatch incoming nl80211 netlink command
    pub fn handle_command(&mut self, cmd: Nl80211Cmd, target_if: u32) -> Result<(), &'static str> {
        let iface = self
            .interfaces
            .iter_mut()
            .find(|i| i.if_index == target_if)
            .ok_or("Interface not found")?;

        let wiphy = self
            .wiphys
            .iter_mut()
            .find(|w| w.wiphy_idx == iface.wiphy_idx)
            .ok_or("Wiphy not found")?;

        match cmd {
            Nl80211Cmd::TriggerScan => {
                wiphy.is_scanning = true;
                Ok(())
            }
            Nl80211Cmd::Disconnect => {
                wiphy.is_scanning = false;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nl80211_wiphy_and_interface() {
        let mut nl = Nl80211Subsystem::new();
        let wiphy_idx = nl.register_wiphy("phy0", &[WifiBand::Band2Ghz, WifiBand::Band5Ghz], 20);

        let if_idx = nl
            .add_interface(wiphy_idx, "wlan0", [0x00, 0x11, 0x22, 0x33, 0x44, 0x55])
            .expect("Interface added");

        assert!(nl.handle_command(Nl80211Cmd::TriggerScan, if_idx).is_ok());
        assert!(nl.wiphys[0].is_scanning);
    }
}
