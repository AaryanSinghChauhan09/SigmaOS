// SPDX-License-Identifier: MIT
// SigmaOS Network Manager 3 Engine (`src/system/sigmanetworkmanager3.rs`)

#![allow(dead_code)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::String;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceState {
    Down,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceType {
    Ethernet,
    Wifi,
    Loopback,
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub if_type: InterfaceType,
    pub state: InterfaceState,
}

#[derive(Debug, Default)]
pub struct SigmaNetworkManager3 {
    pub interfaces: BTreeMap<String, NetworkInterface>,
}

impl SigmaNetworkManager3 {
    pub fn new() -> Self {
        Self::default()
    }
}
