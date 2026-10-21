// SPDX-License-Identifier: MIT
// SigmaOS Network Manager 3
// (`src/system/sigmanetworkmanager3.rs`)

#![allow(dead_code)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceType {
    Ethernet,
    WiFi,
    Loopback,
    Bridge,
    Vlan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceState {
    Down,
    Up,
    Connecting,
    Connected,
    Disconnected,
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub mac_address: String,
    pub ip_address: Option<String>,
    pub if_type: InterfaceType,
    pub state: InterfaceState,
}

pub struct SigmaNetworkManager3 {
    pub interfaces: BTreeMap<String, NetworkInterface>,
}

impl SigmaNetworkManager3 {
    pub fn new() -> Self {
        let mut ifaces = BTreeMap::new();
        ifaces.insert(
            "eth0".to_string(),
            NetworkInterface {
                name: "eth0".to_string(),
                mac_address: "52:54:00:12:34:56".to_string(),
                ip_address: Some("192.168.1.100".to_string()),
                if_type: InterfaceType::Ethernet,
                state: InterfaceState::Connected,
            },
        );
        Self { interfaces: ifaces }
    }

    pub fn bring_up(&mut self, name: &str) -> bool {
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.state = InterfaceState::Up;
            true
        } else {
            false
        }
    }

    pub fn bring_down(&mut self, name: &str) -> bool {
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.state = InterfaceState::Down;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_manager3() {
        let mut mgr = SigmaNetworkManager3::new();
        assert!(mgr.bring_down("eth0"));
        assert_eq!(mgr.interfaces.get("eth0").unwrap().state, InterfaceState::Down);
        assert!(mgr.bring_up("eth0"));
        assert_eq!(mgr.interfaces.get("eth0").unwrap().state, InterfaceState::Up);
    }
}
