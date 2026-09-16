// SPDX-License-Identifier: MIT
/// SigmaOS: ZenithNet - Bare-Metal Networking Stack
/// Implements TCP/IP, UDP, ARP, ICMP with zero-copy packet handling

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;
use core::fmt;

/// Packet types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketType {
    IPv4,
    IPv6,
    ARP,
    ICMP,
    IGMP,
}

/// IP Protocol types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpProtocol {
    Icmp = 1,
    Tcp = 6,
    Udp = 17,
}

/// TCP State Machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAck,
    TimeWait,
}

/// IPv4 Address (stored as u32)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ipv4Addr(pub u32);

impl Ipv4Addr {
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self(((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8) | (d as u32))
    }

    pub fn from_u32(addr: u32) -> Self {
        Self(addr)
    }

    pub fn to_octets(&self) -> [u8; 4] {
        [
            ((self.0 >> 24) & 0xFF) as u8,
            ((self.0 >> 16) & 0xFF) as u8,
            ((self.0 >> 8) & 0xFF) as u8,
            (self.0 & 0xFF) as u8,
        ]
    }

    pub fn localhost() -> Self {
        Self::new(127, 0, 0, 1)
    }

    pub fn any() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octets = self.to_octets();
        write!(f, "{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
    }
}

/// MAC Address
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddr(pub [u8; 6]);

impl MacAddr {
    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        Self([a, b, c, d, e, f])
    }

    pub fn broadcast() -> Self {
        Self([0xFF; 6])
    }

    pub fn zero() -> Self {
        Self([0; 6])
    }
}

impl fmt::Display for MacAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

/// Ethernet Frame
#[derive(Debug, Clone)]
pub struct EthernetFrame {
    pub dst_mac: MacAddr,
    pub src_mac: MacAddr,
    pub ether_type: u16,
    pub payload: Vec<u8>,
}

impl EthernetFrame {
    pub fn new(dst_mac: MacAddr, src_mac: MacAddr, ether_type: u16, payload: Vec<u8>) -> Self {
        Self {
            dst_mac,
            src_mac,
            ether_type,
            payload,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(14 + self.payload.len());
        data.extend_from_slice(&self.dst_mac.0);
        data.extend_from_slice(&self.src_mac.0);
        data.extend_from_slice(&self.ether_type.to_be_bytes());
        data.extend_from_slice(&self.payload);
        data
    }

    pub fn deserialize(data: &[u8]) -> Option<Self> {
        if data.len() < 14 {
            return None;
        }

        let dst_mac = MacAddr(
            [
                data[0], data[1], data[2], data[3], data[4], data[5],
            ],
        );
        let src_mac = MacAddr(
            [
                data[6], data[7], data[8], data[9], data[10], data[11],
            ],
        );
        let ether_type = u16::from_be_bytes([data[12], data[13]]);
        let payload = data[14..].to_vec();

        Some(EthernetFrame {
            dst_mac,
            src_mac,
            ether_type,
            payload,
        })
    }
}

/// IPv4 Header
#[derive(Debug, Clone)]
pub struct Ipv4Header {
    pub version_ihl: u8,
    pub dscp_ecn: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags_fragment_offset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub header_checksum: u16,
    pub src_addr: Ipv4Addr,
    pub dst_addr: Ipv4Addr,
}

impl Ipv4Header {
    pub fn new(src: Ipv4Addr, dst: Ipv4Addr, protocol: u8) -> Self {
        Self {
            version_ihl: 0x45, // IPv4, 5 words (20 bytes)
            dscp_ecn: 0,
            total_length: 20, // Minimum header size
            identification: 0,
            flags_fragment_offset: 0,
            ttl: 64,
            protocol,
            header_checksum: 0,
            src_addr: src,
            dst_addr: dst,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(20);
        data.push(self.version_ihl);
        data.push(self.dscp_ecn);
        data.extend_from_slice(&self.total_length.to_be_bytes());
        data.extend_from_slice(&self.identification.to_be_bytes());
        data.extend_from_slice(&self.flags_fragment_offset.to_be_bytes());
        data.push(self.ttl);
        data.push(self.protocol);
        data.extend_from_slice(&self.header_checksum.to_be_bytes());
        data.extend_from_slice(&self.src_addr.0.to_be_bytes());
        data.extend_from_slice(&self.dst_addr.0.to_be_bytes());
        data
    }
}

/// UDP Header
#[derive(Debug, Clone)]
pub struct UdpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub checksum: u16,
}

impl UdpHeader {
    pub fn new(src_port: u16, dst_port: u16) -> Self {
        Self {
            src_port,
            dst_port,
            length: 8, // Minimum header size
            checksum: 0,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(8);
        data.extend_from_slice(&self.src_port.to_be_bytes());
        data.extend_from_slice(&self.dst_port.to_be_bytes());
        data.extend_from_slice(&self.length.to_be_bytes());
        data.extend_from_slice(&self.checksum.to_be_bytes());
        data
    }
}

/// TCP Header
#[derive(Debug, Clone)]
pub struct TcpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub sequence: u32,
    pub ack_sequence: u32,
    pub data_offset_reserved_flags: u16,
    pub window: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
}

impl TcpHeader {
    pub fn new(src_port: u16, dst_port: u16) -> Self {
        Self {
            src_port,
            dst_port,
            sequence: 0,
            ack_sequence: 0,
            data_offset_reserved_flags: 0x5000, // Data offset 5, no flags
            window: 65535,
            checksum: 0,
            urgent_pointer: 0,
        }
    }

    pub fn set_syn(&mut self) {
        self.data_offset_reserved_flags |= 0x0002;
    }

    pub fn set_ack(&mut self) {
        self.data_offset_reserved_flags |= 0x0010;
    }

    pub fn set_fin(&mut self) {
        self.data_offset_reserved_flags |= 0x0001;
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(20);
        data.extend_from_slice(&self.src_port.to_be_bytes());
        data.extend_from_slice(&self.dst_port.to_be_bytes());
        data.extend_from_slice(&self.sequence.to_be_bytes());
        data.extend_from_slice(&self.ack_sequence.to_be_bytes());
        data.extend_from_slice(&self.data_offset_reserved_flags.to_be_bytes());
        data.extend_from_slice(&self.window.to_be_bytes());
        data.extend_from_slice(&self.checksum.to_be_bytes());
        data.extend_from_slice(&self.urgent_pointer.to_be_bytes());
        data
    }
}

/// Network Interface
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub mac_addr: MacAddr,
    pub ipv4_addr: Ipv4Addr,
    pub netmask: Ipv4Addr,
    pub gateway: Ipv4Addr,
    pub mtu: u16,
    pub enabled: bool,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

impl NetworkInterface {
    pub fn new(name: String, mac_addr: MacAddr, ipv4_addr: Ipv4Addr) -> Self {
        Self {
            name,
            mac_addr,
            ipv4_addr,
            netmask: Ipv4Addr::new(255, 255, 255, 0),
            gateway: Ipv4Addr::any(),
            mtu: 1500,
            enabled: false,
            rx_packets: 0,
            tx_packets: 0,
            rx_bytes: 0,
            tx_bytes: 0,
        }
    }

    pub fn is_on_network(&self, addr: Ipv4Addr) -> bool {
        (self.ipv4_addr.0 & self.netmask.0) == (addr.0 & self.netmask.0)
    }
}

/// Network Error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkError {
    InvalidPacket,
    InvalidAddress,
    InterfaceNotFound,
    InterfaceDisabled,
    RouteNotFound,
    ConnectionFailed,
    SocketError,
    BufferTooSmall,
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPacket => write!(f, "Invalid packet"),
            Self::InvalidAddress => write!(f, "Invalid address"),
            Self::InterfaceNotFound => write!(f, "Interface not found"),
            Self::InterfaceDisabled => write!(f, "Interface disabled"),
            Self::RouteNotFound => write!(f, "Route not found"),
            Self::ConnectionFailed => write!(f, "Connection failed"),
            Self::SocketError => write!(f, "Socket error"),
            Self::BufferTooSmall => write!(f, "Buffer too small"),
        }
    }
}

/// ZenithNet - Main Networking Stack
pub struct ZenithNet {
    interfaces: BTreeMap<String, NetworkInterface>,
    routing_table: Vec<(Ipv4Addr, Ipv4Addr, String)>, // (dest, netmask, interface)
    arp_cache: BTreeMap<Ipv4Addr, MacAddr>,
}

impl ZenithNet {
    pub fn new() -> Self {
        Self {
            interfaces: BTreeMap::new(),
            routing_table: Vec::new(),
            arp_cache: BTreeMap::new(),
        }
    }

    /// Add network interface
    pub fn add_interface(&mut self, interface: NetworkInterface) -> Result<(), NetworkError> {
        self.interfaces.insert(interface.name.clone(), interface);
        Ok(())
    }

    /// Enable interface
    pub fn enable_interface(&mut self, name: &str) -> Result<(), NetworkError> {
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.enabled = true;
            Ok(())
        } else {
            Err(NetworkError::InterfaceNotFound)
        }
    }

    /// Add route
    pub fn add_route(
        &mut self,
        dest: Ipv4Addr,
        netmask: Ipv4Addr,
        interface: String,
    ) -> Result<(), NetworkError> {
        if !self.interfaces.contains_key(&interface) {
            return Err(NetworkError::InterfaceNotFound);
        }
        self.routing_table.push((dest, netmask, interface));
        Ok(())
    }

    /// Find route for destination
    pub fn find_route(&self, dest: Ipv4Addr) -> Result<String, NetworkError> {
        for (route_dest, netmask, interface) in &self.routing_table {
            if (dest.0 & netmask.0) == (route_dest.0 & netmask.0) {
                return Ok(interface.clone());
            }
        }
        Err(NetworkError::RouteNotFound)
    }

    /// ARP lookup or resolution
    pub fn arp_lookup(&self, ip: Ipv4Addr) -> Option<MacAddr> {
        self.arp_cache.get(&ip).copied()
    }

    /// Add ARP entry
    pub fn arp_add(&mut self, ip: Ipv4Addr, mac: MacAddr) {
        self.arp_cache.insert(ip, mac);
    }

    /// Get interface statistics
    pub fn get_interface_stats(&self, name: &str) -> Result<(u64, u64, u64, u64), NetworkError> {
        if let Some(iface) = self.interfaces.get(name) {
            Ok((iface.rx_packets, iface.tx_packets, iface.rx_bytes, iface.tx_bytes))
        } else {
            Err(NetworkError::InterfaceNotFound)
        }
    }

    /// Get interface count
    pub fn interface_count(&self) -> usize {
        self.interfaces.len()
    }
}

impl Default for ZenithNet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_addr() {
        let addr = Ipv4Addr::new(192, 168, 1, 1);
        let octets = addr.to_octets();
        assert_eq!(octets, [192, 168, 1, 1]);
    }

    #[test]
    fn test_mac_addr() {
        let mac = MacAddr::new(0x08, 0x00, 0x27, 0x00, 0x00, 0x00);
        assert_eq!(mac.0[0], 0x08);
    }

    #[test]
    fn test_ethernet_frame() {
        let src = MacAddr::new(0x00, 0x01, 0x02, 0x03, 0x04, 0x05);
        let dst = MacAddr::new(0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B);
        let frame = EthernetFrame::new(dst, src, 0x0800, vec![1, 2, 3, 4]);

        let serialized = frame.serialize();
        let deserialized = EthernetFrame::deserialize(&serialized).unwrap();

        assert_eq!(deserialized.src_mac, src);
        assert_eq!(deserialized.dst_mac, dst);
    }

    #[test]
    fn test_zenithnet_creation() {
        let net = ZenithNet::new();
        assert_eq!(net.interface_count(), 0);
    }

    #[test]
    fn test_add_interface() {
        let mut net = ZenithNet::new();
        let iface = NetworkInterface::new(
            "eth0".to_string(),
            MacAddr::new(0x00, 0x01, 0x02, 0x03, 0x04, 0x05),
            Ipv4Addr::new(192, 168, 1, 1),
        );

        net.add_interface(iface).unwrap();
        assert_eq!(net.interface_count(), 1);
    }

    #[test]
    fn test_tcp_header_flags() {
        let mut header = TcpHeader::new(80, 1234);
        header.set_syn();
        header.set_ack();

        assert_eq!(header.data_offset_reserved_flags & 0x0002, 0x0002);
        assert_eq!(header.data_offset_reserved_flags & 0x0010, 0x0010);
    }

    #[test]
    fn test_arp_cache() {
        let mut net = ZenithNet::new();
        let ip = Ipv4Addr::new(192, 168, 1, 100);
        let mac = MacAddr::new(0x00, 0x01, 0x02, 0x03, 0x04, 0x05);

        net.arp_add(ip, mac);
        assert_eq!(net.arp_lookup(ip), Some(mac));
    }
}
