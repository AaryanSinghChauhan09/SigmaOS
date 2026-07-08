// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/net/ethernet_device_base.rs — Base Device Trait for Ethernet Drivers
//
// Defines the OOP base class for all Ethernet devices using Rust traits.
// This provides a common interface for Ethernet operations.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── Ethernet Error Codes ─────────────────────────────────────────────────

pub const ETH_OK: I32 = 0;
pub const ETH_ERR_NO_DEVICE: I32 = -1;
pub const ETH_ERR_INIT_FAILED: I32 = -2;
pub const ETH_ERR_OUT_OF_MEM: I32 = -3;
pub const ETH_ERR_NOT_SUPPORTED: I32 = -4;
pub const ETH_ERR_IO: I32 = -5;
pub const ETH_ERR_TIMEOUT: I32 = -6;
pub const ETH_ERR_LINK_DOWN: I32 = -7;
pub const ETH_ERR_INVALID_PARAM: I32 = -8;

// ─── Ethernet MAC Address ─────────────────────────────────────────────

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EthernetAddress {
    pub bytes: [U8; 6],
}

impl EthernetAddress {
    pub const fn new() -> Self {
        EthernetAddress {
            bytes: [0; 6],
        }
    }

    pub const fn from_bytes(b0: U8, b1: U8, b2: U8, b3: U8, b4: U8, b5: U8) -> Self {
        EthernetAddress {
            bytes: [b0, b1, b2, b3, b4, b5],
        }
    }

    pub fn is_broadcast(&self) -> bool {
        self.bytes[0] == 0xFF && self.bytes[1] == 0xFF && self.bytes[2] == 0xFF &&
        self.bytes[3] == 0xFF && self.bytes[4] == 0xFF && self.bytes[5] == 0xFF
    }

    pub fn is_multicast(&self) -> bool {
        self.bytes[0] & 0x01 != 0
    }

    pub fn is_zero(&self) -> bool {
        self.bytes[0] == 0 && self.bytes[1] == 0 && self.bytes[2] == 0 &&
        self.bytes[3] == 0 && self.bytes[4] == 0 && self.bytes[5] == 0
    }
}

// ─── Ethernet Speed ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EthernetSpeed {
    Speed10,
    Speed100,
    Speed1000,
    Speed2500,
    Speed5000,
    Speed10000,
    Speed25000,
    Speed40000,
    Speed100000,
}

impl EthernetSpeed {
    pub fn to_mbps(&self) -> U32 {
        match self {
            EthernetSpeed::Speed10 => 10,
            EthernetSpeed::Speed100 => 100,
            EthernetSpeed::Speed1000 => 1000,
            EthernetSpeed::Speed2500 => 2500,
            EthernetSpeed::Speed5000 => 5000,
            EthernetSpeed::Speed10000 => 10000,
            EthernetSpeed::Speed25000 => 25000,
            EthernetSpeed::Speed40000 => 40000,
            EthernetSpeed::Speed100000 => 100000,
        }
    }
}

// ─── Ethernet Duplex Mode ─────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EthernetDuplex {
    Half,
    Full,
}

// ─── Ethernet Link Status ─────────────────────────────────────

#[repr(C)]
pub struct EthernetLinkStatus {
    pub link_up: bool,
    pub speed: EthernetSpeed,
    pub duplex: EthernetDuplex,
    pub autoneg: bool,
}

impl EthernetLinkStatus {
    pub const fn new() -> Self {
        EthernetLinkStatus {
            link_up: false,
            speed: EthernetSpeed::Speed1000,
            duplex: EthernetDuplex::Full,
            autoneg: true,
        }
    }
}

// ─── Ethernet Statistics ─────────────────────────────────────

#[repr(C)]
pub struct EthernetStats {
    pub rx_packets: U64,
    pub tx_packets: U64,
    pub rx_bytes: U64,
    pub tx_bytes: U64,
    pub rx_errors: U64,
    pub tx_errors: U64,
    pub rx_dropped: U64,
    pub tx_dropped: U64,
    pub multicast: U64,
    pub collisions: U64,
}

impl EthernetStats {
    pub const fn new() -> Self {
        EthernetStats {
            rx_packets: 0,
            tx_packets: 0,
            rx_bytes: 0,
            tx_bytes: 0,
            rx_errors: 0,
            tx_errors: 0,
            rx_dropped: 0,
            tx_dropped: 0,
            multicast: 0,
            collisions: 0,
        }
    }
}

// ─── Ethernet Device Trait ─────────────────────────────────────

/// Trait for Ethernet device operations
pub trait EthernetDevice {
    /// Initialize the Ethernet device
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32;
    
    /// Check if device is initialized
    fn is_initialized(&self) -> bool;
    
    /// Get device name
    fn get_device_name(&self) -> &'static str;
    
    /// Get MAC address
    fn get_mac_address(&self) -> EthernetAddress;
    
    /// Set MAC address
    fn set_mac_address(&mut self, mac: EthernetAddress) -> I32;
    
    /// Get MTU
    fn get_mtu(&self) -> U32;
    
    /// Set MTU
    fn set_mtu(&mut self, mtu: U32) -> I32;
    
    /// Get link status
    fn get_link_status(&self) -> EthernetLinkStatus;
    
    /// Set link speed/duplex
    fn set_link_config(&mut self, speed: EthernetSpeed, duplex: EthernetDuplex) -> I32;
    
    /// Enable autonegotiation
    fn set_autoneg(&mut self, enable: bool) -> I32;
    
    /// Enable/disable promiscuous mode
    fn set_promiscuous(&mut self, enable: bool) -> I32;
    
    /// Enable/disable multicast
    fn set_multicast(&mut self, enable: bool) -> I32;
    
    /// Add multicast address
    fn add_multicast_address(&mut self, mac: EthernetAddress) -> I32;
    
    /// Remove multicast address
    fn remove_multicast_address(&mut self, mac: EthernetAddress) -> I32;
    
    /// Enable/disable all-multicast
    fn set_all_multicast(&mut self, enable: bool) -> I32;
    
    /// Enable interface
    fn enable(&mut self) -> I32;
    
    /// Disable interface
    fn disable(&mut self) -> I32;
    
    /// Transmit packet
    fn transmit(&mut self, buffer: *const U8, length: U32) -> I32;
    
    /// Receive packet
    fn receive(&mut self, buffer: *mut U8, max_length: U32) -> I32;
    
    /// Get statistics
    fn get_stats(&self) -> EthernetStats;
    
    /// Reset statistics
    fn reset_stats(&mut self);
    
    /// Reset the device
    fn reset(&mut self) -> I32;
    
    /// Shutdown the device
    fn shutdown(&mut self) -> I32;
}

// ─── Ethernet PHY Trait ─────────────────────────────────────

/// Trait for PHY (Physical Layer) operations
pub trait EthernetPhy {
    /// Read PHY register
    fn read_phy(&self, phy_addr: U8, reg: U8) -> U16;
    
    /// Write PHY register
    fn write_phy(&mut self, phy_addr: U8, reg: U8, value: U16) -> I32;
    
    /// Get PHY ID
    fn get_phy_id(&self, phy_addr: U8) -> U32;
    
    /// Reset PHY
    fn reset_phy(&mut self, phy_addr: U8) -> I32;
    
    /// Get link status from PHY
    fn get_phy_link_status(&self, phy_addr: U8) -> bool;
    
    /// Get PHY speed/duplex
    fn get_phy_speed_duplex(&self, phy_addr: U8) -> (EthernetSpeed, EthernetDuplex);
}
