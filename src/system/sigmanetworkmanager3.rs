// SPDX-License-Identifier: MIT
// SigmaOS Network Manager 3

#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceState {
    Down,
    Up,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceType {
    Ethernet,
    Wifi,
    Loopback,
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub state: InterfaceState,
    pub if_type: InterfaceType,
}

#[derive(Debug, Clone, Default)]
pub struct SigmaNetworkManager3 {
    pub interfaces: Vec<NetworkInterface>,
}

impl SigmaNetworkManager3 {
    pub fn new() -> Self {
        Self::default()
    }
}
