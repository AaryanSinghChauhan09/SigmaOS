//! Sigma Network Manager 3
//!
//! Advanced multi-interface network routing, state management, and configuration.

use alloc::string::String;
use alloc::vec::Vec;

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
    Up,
    Down,
    Configuring,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkInterface {
    pub name: String,
    pub if_type: InterfaceType,
    pub state: InterfaceState,
    pub ip_address: Option<String>,
}

#[derive(Debug, Default)]
pub struct SigmaNetworkManager3 {
    pub interfaces: Vec<NetworkInterface>,
}

impl SigmaNetworkManager3 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_interface(&mut self, iface: NetworkInterface) {
        self.interfaces.push(iface);
    }
}
