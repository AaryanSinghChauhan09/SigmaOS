// SPDX-License-Identifier: MIT
// SigmaNetworkManager3 - Advanced Network Interface & Connectivity System

#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceType {
    Ethernet,
    Wifi,
    Loopback,
    Bridge,
    Virtual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub if_type: InterfaceType,
    pub state: InterfaceState,
    pub mac_address: String,
    pub ip_address: Option<String>,
}

impl NetworkInterface {
    pub fn new(name: &str, if_type: InterfaceType, mac_address: &str) -> Self {
        Self {
            name: name.to_string(),
            if_type,
            state: InterfaceState::Down,
            mac_address: mac_address.to_string(),
            ip_address: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SigmaNetworkManager3 {
    pub interfaces: Vec<NetworkInterface>,
}

impl SigmaNetworkManager3 {
    pub fn new() -> Self {
        Self {
            interfaces: Vec::new(),
        }
    }

    pub fn add_interface(&mut self, iface: NetworkInterface) {
        self.interfaces.push(iface);
    }
}
