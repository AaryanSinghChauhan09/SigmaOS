// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - Ubuntu system integration (Netplan, Snap, Ubuntu Pro)
// Principles: OPEN STANDARD, MODULARITY, USER FREEDOM AND CONTROL
use alloc::{collections::BTreeMap, string::String, vec::Vec};
#[derive(Debug, Clone)]
pub struct EthernetConfig {
    pub dhcp: bool,
    pub addresses: Vec<String>,
    pub nameservers: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct WifiConfig {
    pub ssid: String,
    pub password: String,
    pub dhcp: bool,
}
#[derive(Debug, Default)]
pub struct NetplanConfig {
    pub ethernets: BTreeMap<String, EthernetConfig>,
    pub wifis: BTreeMap<String, WifiConfig>,
}
impl NetplanConfig {
    pub fn add_ethernet(&mut self, iface: &str, cfg: EthernetConfig) {
        self.ethernets.insert(iface.into(), cfg);
    }
    pub fn add_wifi(&mut self, iface: &str, cfg: WifiConfig) {
        self.wifis.insert(iface.into(), cfg);
    }
    pub fn apply(&self) -> Result<(), &'static str> {
        Ok(())
    }
    pub fn generate_yaml(&self) -> String {
        String::from(
            "network:
  version: 2
",
        )
    }
}
#[derive(Debug, Clone)]
pub struct SnapInfo {
    pub name: String,
    pub channel: String,
    pub revision: u32,
    pub confined: bool,
}
pub struct SnapSandbox {
    pub installed: BTreeMap<String, SnapInfo>,
}
impl SnapSandbox {
    pub fn new() -> Self {
        Self {
            installed: BTreeMap::new(),
        }
    }
    pub fn install_snap(&mut self, name: &str, channel: &str, confined: bool) {
        self.installed.insert(
            name.into(),
            SnapInfo {
                name: name.into(),
                channel: channel.into(),
                revision: 1,
                confined,
            },
        );
    }
    pub fn remove_snap(&mut self, name: &str) -> Result<(), &'static str> {
        self.installed
            .remove(name)
            .ok_or("not installed")
            .map(|_| ())
    }
    pub fn connect_interface(
        &self,
        _snap: &str,
        _plug: &str,
        _slot: &str,
    ) -> Result<(), &'static str> {
        Ok(())
    }
    pub fn list_snaps(&self) -> Vec<&SnapInfo> {
        self.installed.values().collect()
    }
}
pub struct UbuntuProHooks {
    pub subscribed: bool,
}
impl UbuntuProHooks {
    pub fn new() -> Self {
        Self { subscribed: false }
    }
    pub fn attach_subscription(&mut self, _token: &str) {
        self.subscribed = true;
    }
    pub fn enable_esm_infra(&self) -> Result<(), &'static str> {
        if self.subscribed {
            Ok(())
        } else {
            Err("not subscribed")
        }
    }
    pub fn enable_fips(&self) -> Result<(), &'static str> {
        if self.subscribed {
            Ok(())
        } else {
            Err("not subscribed")
        }
    }
    pub fn check_subscription_status(&self) -> bool {
        self.subscribed
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_netplan() {
        let mut n = NetplanConfig::default();
        n.add_ethernet(
            "eth0",
            EthernetConfig {
                dhcp: true,
                addresses: Vec::new(),
                nameservers: Vec::new(),
            },
        );
        assert!(n.ethernets.contains_key("eth0"));
    }
    #[test]
    fn test_snap() {
        let mut s = SnapSandbox::new();
        s.install_snap("firefox", "stable", true);
        assert_eq!(s.list_snaps().len(), 1);
        s.remove_snap("firefox").unwrap();
        assert!(s.installed.is_empty());
    }
    #[test]
    fn test_ubuntu_pro() {
        let mut p = UbuntuProHooks::new();
        assert!(p.enable_esm_infra().is_err());
        p.attach_subscription("token123");
        assert!(p.enable_esm_infra().is_ok());
    }
}
