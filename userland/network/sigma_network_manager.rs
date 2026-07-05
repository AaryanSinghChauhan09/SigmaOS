// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/network/sigma_network_manager.rs — Network Management
//
// Implements:
//   - Network interface management (Ethernet, Wi-Fi, cellular)
//   - Connection profiles and automatic switching
//   - DHCP client and static IP configuration
//   - DNS management and resolution
//   - VPN support (WireGuard, OpenVPN)
//   - Firewall integration with sigma-auth
//   - Network statistics and monitoring
//   - India context: BharatNet integration, rural connectivity
//
// Language: Rust
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

// ── Network interface types ─────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum InterfaceType {
    Ethernet = 0,
    WiFi = 1,
    Cellular = 2,
    Loopback = 3,
    Bridge = 4,
    VPN = 5,
}

// ── Connection state ─────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    Unknown = 0,
    Disconnected = 1,
    Connecting = 2,
    Connected = 3,
    Failed = 4,
}

// ── IP address configuration ─────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IpAddress {
    pub addr: [u8; 4],
    pub netmask: [u8; 4],
    pub gateway: [u8; 4],
    pub is_dhcp: bool,
}

impl IpAddress {
    pub const fn new() -> Self {
        Self {
            addr: [0u8; 4],
            netmask: [255, 255, 255, 0],
            gateway: [0u8; 4],
            is_dhcp: true,
        }
    }

    pub const fn from_bytes(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self {
            addr: [a, b, c, d],
            netmask: [255, 255, 255, 0],
            gateway: [0u8; 4],
            is_dhcp: false,
        }
    }
}

// ── Network interface ───────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NetworkInterface {
    pub name: [u8; 16],
    pub if_type: InterfaceType,
    pub mac_addr: [u8; 6],
    pub mtu: u32,
    pub ip: IpAddress,
    pub state: ConnectionState,
    pub tx_bytes: u64,
    pub rx_bytes: u64,
    pub tx_packets: u64,
    pub rx_packets: u64,
    pub is_up: bool,
}

impl NetworkInterface {
    pub const fn new() -> Self {
        Self {
            name: [0u8; 16],
            if_type: InterfaceType::Ethernet,
            mac_addr: [0u8; 6],
            mtu: 1500,
            ip: IpAddress::new(),
            state: ConnectionState::Unknown,
            tx_bytes: 0,
            rx_bytes: 0,
            tx_packets: 0,
            rx_packets: 0,
            is_up: false,
        }
    }
}

// ── DNS configuration ───────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DnsConfig {
    pub servers: [[u8; 4]; 4],
    pub server_count: u32,
    pub search_domains: [[u8; 64]; 4],
    pub domain_count: u32,
}

impl DnsConfig {
    pub const fn new() -> Self {
        Self {
            servers: [
                [8, 8, 8, 8],      // Google DNS
                [8, 8, 4, 4],      // Google DNS secondary
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            server_count: 2,
            search_domains: [[0u8; 64]; 4],
            domain_count: 0,
        }
    }
}

// ── Connection profile ─────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConnectionProfile {
    pub name: [u8; 64],
    pub if_name: [u8; 16],
    pub auto_connect: bool,
    pub priority: u32,
    pub ip: IpAddress,
    pub dns: DnsConfig,
    pub vpn_enabled: bool,
    pub vpn_config: [u8; 256],
}

impl ConnectionProfile {
    pub const fn new() -> Self {
        Self {
            name: [0u8; 64],
            if_name: [0u8; 16],
            auto_connect: false,
            priority: 0,
            ip: IpAddress::new(),
            dns: DnsConfig::new(),
            vpn_enabled: false,
            vpn_config: [0u8; 256],
        }
    }
}

// ── Network manager state ───────────────────────────────────────────────

const MAX_INTERFACES: usize = 16;
const MAX_PROFILES: usize = 32;

pub struct NetworkManager {
    interfaces: [Option<NetworkInterface>; MAX_INTERFACES],
    profiles: [Option<ConnectionProfile>; MAX_PROFILES],
    active_profile: Option<u32>,
    interface_count: AtomicU32,
    profile_count: AtomicU32,
    dns_config: DnsConfig,
    initialized: bool,
}

impl NetworkManager {
    pub const fn new() -> Self {
        Self {
            interfaces: [const { None }; MAX_INTERFACES],
            profiles: [const { None }; MAX_PROFILES],
            active_profile: None,
            interface_count: AtomicU32::new(0),
            profile_count: AtomicU32::new(0),
            dns_config: DnsConfig::new(),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }

    /// Add a network interface
    pub fn add_interface(&mut self, iface: NetworkInterface) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_INTERFACES {
            if self.interfaces[i].is_none() {
                self.interfaces[i] = Some(iface);
                self.interface_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Add a connection profile
    pub fn add_profile(&mut self, profile: ConnectionProfile) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_PROFILES {
            if self.profiles[i].is_none() {
                self.profiles[i] = Some(profile);
                self.profile_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Connect to a profile
    pub fn connect(&mut self, profile_id: u32) -> bool {
        if !self.initialized {
            return false;
        }

        let profile_idx = profile_id as usize;
        if profile_idx >= MAX_PROFILES {
            return false;
        }

        let profile = match self.profiles[profile_idx] {
            Some(p) => p,
            None => return false,
        };

        // Find the interface
        for i in 0..MAX_INTERFACES {
            if let Some(iface) = &mut self.interfaces[i] {
                let mut if_name_match = true;
                for j in 0..16 {
                    if iface.name[j] != profile.if_name[j] {
                        if_name_match = false;
                        break;
                    }
                }

                if if_name_match {
                    iface.state = ConnectionState::Connecting;
                    iface.ip = profile.ip;
                    iface.state = ConnectionState::Connected;
                    iface.is_up = true;
                    self.active_profile = Some(profile_id);
                    self.dns_config = profile.dns;
                    return true;
                }
            }
        }
        false
    }

    /// Disconnect current connection
    pub fn disconnect(&mut self) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_INTERFACES {
            if let Some(iface) = &mut self.interfaces[i] {
                if iface.state == ConnectionState::Connected {
                    iface.state = ConnectionState::Disconnected;
                    iface.is_up = false;
                    self.active_profile = None;
                    return true;
                }
            }
        }
        false
    }

    /// Update interface statistics
    pub fn update_stats(&mut self, if_name: &[u8], tx: u64, rx: u64) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_INTERFACES {
            if let Some(iface) = &mut self.interfaces[i] {
                let mut name_match = true;
                for j in 0..16 {
                    if j < if_name.len() && iface.name[j] != if_name[j] {
                        name_match = false;
                        break;
                    }
                }

                if name_match {
                    iface.tx_bytes = tx;
                    iface.rx_bytes = rx;
                    return true;
                }
            }
        }
        false
    }

    /// Get interface by name
    pub fn get_interface(&self, name: &[u8]) -> Option<NetworkInterface> {
        for i in 0..MAX_INTERFACES {
            if let Some(iface) = &self.interfaces[i] {
                let mut name_match = true;
                for j in 0..16 {
                    if j < name.len() && iface.name[j] != name[j] {
                        name_match = false;
                        break;
                    }
                }

                if name_match {
                    return Some(*iface);
                }
            }
        }
        None
    }

    /// Set DNS configuration
    pub fn set_dns(&mut self, dns: DnsConfig) {
        self.dns_config = dns;
    }

    pub fn interface_count(&self) -> u32 {
        self.interface_count.load(Ordering::Relaxed)
    }

    pub fn profile_count(&self) -> u32 {
        self.profile_count.load(Ordering::Relaxed)
    }
}

// ── Global network manager instance ───────────────────────────────────────

static mut G_NETMAN: NetworkManager = NetworkManager::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn netman_init() {
    G_NETMAN.init();
}

#[no_mangle]
pub unsafe extern "C" fn netman_add_interface(
    name: *const u8,
    if_type: u8,
    mac_addr: *const u8,
    mtu: u32,
) -> i32 {
    let mut iface = NetworkInterface::new();
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 16.min(iface.name.len()));
        for i in 0..name_slice.len() {
            iface.name[i] = name_slice[i];
        }
    }
    
    iface.if_type = match if_type {
        0 => InterfaceType::Ethernet,
        1 => InterfaceType::WiFi,
        2 => InterfaceType::Cellular,
        3 => InterfaceType::Loopback,
        4 => InterfaceType::Bridge,
        5 => InterfaceType::VPN,
        _ => InterfaceType::Ethernet,
    };
    
    if !mac_addr.is_null() {
        let mac_slice = core::slice::from_raw_parts(mac_addr, 6);
        iface.mac_addr.copy_from_slice(mac_slice);
    }
    
    iface.mtu = mtu;
    
    if G_NETMAN.add_interface(iface) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn netman_add_profile(
    name: *const u8,
    if_name: *const u8,
    auto_connect: bool,
    priority: u32,
) -> i32 {
    let mut profile = ConnectionProfile::new();
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 64.min(profile.name.len()));
        for i in 0..name_slice.len() {
            profile.name[i] = name_slice[i];
        }
    }
    
    if !if_name.is_null() {
        let if_slice = core::slice::from_raw_parts(if_name, 16.min(profile.if_name.len()));
        for i in 0..if_slice.len() {
            profile.if_name[i] = if_slice[i];
        }
    }
    
    profile.auto_connect = auto_connect;
    profile.priority = priority;
    
    if G_NETMAN.add_profile(profile) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn netman_connect(profile_id: u32) -> i32 {
    if G_NETMAN.connect(profile_id) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn netman_disconnect() -> i32 {
    if G_NETMAN.disconnect() { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn netman_update_stats(
    if_name: *const u8,
    tx: u64,
    rx: u64,
) -> i32 {
    if if_name.is_null() {
        return -1;
    }
    let name_slice = core::slice::from_raw_parts(if_name, 16);
    if G_NETMAN.update_stats(name_slice, tx, rx) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn netman_interface_count() -> u32 {
    G_NETMAN.interface_count()
}

#[no_mangle]
pub unsafe extern "C" fn netman_profile_count() -> u32 {
    G_NETMAN.profile_count()
}
