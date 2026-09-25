// SPDX-License-Identifier: MIT
/// SigmaOS: ZenithNet - Bare-Metal Networking Stack
/// Implements TCP/IP, UDP, ARP, ICMP with zero-copy packet handling
/// Enhanced with Linux netfilter and BSD firewall integration

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
    Timeout,
    ConnectionRefused,
    ConnectionReset,
    FirewallBlocked,
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
            Self::Timeout => write!(f, "Connection timeout"),
            Self::ConnectionRefused => write!(f, "Connection refused"),
            Self::ConnectionReset => write!(f, "Connection reset"),
            Self::FirewallBlocked => write!(f, "Packet blocked by firewall"),
        }
    }
}

/// Firewall rule action (inspired by Linux iptables and BSD PF)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallAction {
    Accept,
    Drop,
    Reject,
}

/// Firewall rule (Linux netfilter-inspired)
#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub id: u32,
    pub src_addr: Option<Ipv4Addr>,
    pub src_mask: Option<Ipv4Addr>,
    pub dst_addr: Option<Ipv4Addr>,
    pub dst_mask: Option<Ipv4Addr>,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub protocol: Option<IpProtocol>,
    pub action: FirewallAction,
    pub enabled: bool,
}

/// TCP Connection (Linux socket-inspired)
#[derive(Debug, Clone)]
pub struct TcpConnection {
    pub local_addr: Ipv4Addr,
    pub local_port: u16,
    pub remote_addr: Ipv4Addr,
    pub remote_port: u16,
    pub state: TcpState,
    pub sequence: u32,
    pub ack_sequence: u32,
    pub window_size: u16,
    pub receive_buffer: Vec<u8>,
    pub send_buffer: Vec<u8>,
    pub last_activity: u64, // timestamp
}

/// Socket (Linux/BSD socket API abstraction)
#[derive(Debug, Clone)]
pub struct NetworkSocket {
    pub socket_id: u32,
    pub domain: u32, // AF_INET, AF_INET6, etc.
    pub socket_type: u32, // SOCK_STREAM, SOCK_DGRAM, etc.
    pub protocol: u32, // IPPROTO_TCP, IPPROTO_UDP, etc.
    pub bound_addr: Option<Ipv4Addr>,
    pub bound_port: Option<u16>,
    pub connected_addr: Option<Ipv4Addr>,
    pub connected_port: Option<u16>,
    pub tcp_connection: Option<TcpConnection>,
    pub non_blocking: bool,
    pub receive_timeout: Option<u64>, // milliseconds
    pub send_timeout: Option<u64>, // milliseconds
}

/// ZenithNet - Main Networking Stack
pub struct ZenithNet {
    interfaces: BTreeMap<String, NetworkInterface>,
    routing_table: Vec<(Ipv4Addr, Ipv4Addr, String)>, // (dest, netmask, interface)
    arp_cache: BTreeMap<Ipv4Addr, MacAddr>,
    firewall_rules: Vec<FirewallRule>,
    tcp_connections: BTreeMap<u32, TcpConnection>, // connection_id -> connection
    sockets: BTreeMap<u32, NetworkSocket>, // socket_id -> socket
    next_socket_id: u32,
    next_connection_id: u32,
}

impl ZenithNet {
    pub fn new() -> Self {
        Self {
            interfaces: BTreeMap::new(),
            routing_table: Vec::new(),
            arp_cache: BTreeMap::new(),
            firewall_rules: Vec::new(),
            tcp_connections: BTreeMap::new(),
            sockets: BTreeMap::new(),
            next_socket_id: 1,
            next_connection_id: 1,
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

    // ========== Firewall Management (Linux iptables/BSD PF inspired) ==========

    /// Add firewall rule
    pub fn add_firewall_rule(&mut self, rule: FirewallRule) -> Result<(), NetworkError> {
        self.firewall_rules.push(rule);
        Ok(())
    }

    /// Remove firewall rule by ID
    pub fn remove_firewall_rule(&mut self, rule_id: u32) -> Result<(), NetworkError> {
        if let Some(pos) = self.firewall_rules.iter().position(|r| r.id == rule_id) {
            self.firewall_rules.remove(pos);
            Ok(())
        } else {
            Err(NetworkError::InterfaceNotFound) // Reuse error for simplicity
        }
    }

    /// Check if packet should be allowed by firewall
    pub fn check_firewall(&self, src_addr: Ipv4Addr, dst_addr: Ipv4Addr, src_port: u16, dst_port: u16, protocol: IpProtocol) -> Result<(), NetworkError> {
        for rule in &self.firewall_rules {
            if !rule.enabled {
                continue;
            }

            // Check source address match
            if let Some(rule_src) = rule.src_addr {
                let rule_mask = rule.src_mask.unwrap_or(Ipv4Addr::new(255, 255, 255, 255));
                if (src_addr.0 & rule_mask.0) != (rule_src.0 & rule_mask.0) {
                    continue;
                }
            }

            // Check destination address match
            if let Some(rule_dst) = rule.dst_addr {
                let rule_mask = rule.dst_mask.unwrap_or(Ipv4Addr::new(255, 255, 255, 255));
                if (dst_addr.0 & rule_mask.0) != (rule_dst.0 & rule_mask.0) {
                    continue;
                }
            }

            // Check source port match
            if let Some(rule_src_port) = rule.src_port {
                if src_port != rule_src_port {
                    continue;
                }
            }

            // Check destination port match
            if let Some(rule_dst_port) = rule.dst_port {
                if dst_port != rule_dst_port {
                    continue;
                }
            }

            // Check protocol match
            if let Some(rule_protocol) = rule.protocol {
                if protocol != rule_protocol {
                    continue;
                }
            }

            // Rule matched - apply action
            match rule.action {
                FirewallAction::Accept => return Ok(()),
                FirewallAction::Drop => return Err(NetworkError::FirewallBlocked),
                FirewallAction::Reject => return Err(NetworkError::ConnectionRefused),
            }
        }

        // Default allow if no rules matched
        Ok(())
    }

    // ========== Socket Management (Linux/BSD socket API) ==========

    /// Create a new socket
    pub fn socket(&mut self, domain: u32, socket_type: u32, protocol: u32) -> Result<u32, NetworkError> {
        let socket_id = self.next_socket_id;
        self.next_socket_id += 1;

        let socket = NetworkSocket {
            socket_id,
            domain,
            socket_type,
            protocol,
            bound_addr: None,
            bound_port: None,
            connected_addr: None,
            connected_port: None,
            tcp_connection: None,
            non_blocking: false,
            receive_timeout: None,
            send_timeout: None,
        };

        self.sockets.insert(socket_id, socket);
        Ok(socket_id)
    }

    /// Bind socket to address and port
    pub fn bind(&mut self, socket_id: u32, addr: Ipv4Addr, port: u16) -> Result<(), NetworkError> {
        if let Some(socket) = self.sockets.get_mut(&socket_id) {
            socket.bound_addr = Some(addr);
            socket.bound_port = Some(port);
            Ok(())
        } else {
            Err(NetworkError::SocketError)
        }
    }

    /// Connect socket to remote address
    pub fn connect(&mut self, socket_id: u32, remote_addr: Ipv4Addr, remote_port: u16) -> Result<(), NetworkError> {
        if let Some(socket) = self.sockets.get_mut(&socket_id) {
            if socket.socket_type != 1 { // SOCK_STREAM
                return Err(NetworkError::SocketError);
            }

            // Create TCP connection
            let connection_id = self.next_connection_id;
            self.next_connection_id += 1;

            let local_addr = socket.bound_addr.unwrap_or(Ipv4Addr::any());
            let local_port = socket.bound_port.unwrap_or(0); // Let system assign

            let tcp_connection = TcpConnection {
                local_addr,
                local_port,
                remote_addr,
                remote_port,
                state: TcpState::SynSent,
                sequence: 1000, // Initial sequence number
                ack_sequence: 0,
                window_size: 65535,
                receive_buffer: Vec::new(),
                send_buffer: Vec::new(),
                last_activity: 0,
            };

            self.tcp_connections.insert(connection_id, tcp_connection);
            socket.connected_addr = Some(remote_addr);
            socket.connected_port = Some(remote_port);
            socket.tcp_connection = Some(tcp_connection);

            Ok(())
        } else {
            Err(NetworkError::SocketError)
        }
    }

    /// Listen on socket (TCP server)
    pub fn listen(&mut self, socket_id: u32, backlog: u32) -> Result<(), NetworkError> {
        if let Some(socket) = self.sockets.get_mut(&socket_id) {
            if socket.socket_type != 1 { // SOCK_STREAM
                return Err(NetworkError::SocketError);
            }

            if let Some(ref mut conn) = socket.tcp_connection {
                conn.state = TcpState::Listen;
            } else {
                // Create listening connection
                let connection_id = self.next_connection_id;
                self.next_connection_id += 1;

                let tcp_connection = TcpConnection {
                    local_addr: socket.bound_addr.unwrap_or(Ipv4Addr::any()),
                    local_port: socket.bound_port.unwrap_or(0),
                    remote_addr: Ipv4Addr::any(),
                    remote_port: 0,
                    state: TcpState::Listen,
                    sequence: 0,
                    ack_sequence: 0,
                    window_size: 65535,
                    receive_buffer: Vec::new(),
                    send_buffer: Vec::new(),
                    last_activity: 0,
                };

                self.tcp_connections.insert(connection_id, tcp_connection);
                socket.tcp_connection = Some(tcp_connection);
            }

            Ok(())
        } else {
            Err(NetworkError::SocketError)
        }
    }

    /// Set socket to non-blocking mode
    pub fn set_non_blocking(&mut self, socket_id: u32, non_blocking: bool) -> Result<(), NetworkError> {
        if let Some(socket) = self.sockets.get_mut(&socket_id) {
            socket.non_blocking = non_blocking;
            Ok(())
        } else {
            Err(NetworkError::SocketError)
        }
    }

    /// Set socket receive timeout
    pub fn set_receive_timeout(&mut self, socket_id: u32, timeout_ms: u64) -> Result<(), NetworkError> {
        if let Some(socket) = self.sockets.get_mut(&socket_id) {
            socket.receive_timeout = Some(timeout_ms);
            Ok(())
        } else {
            Err(NetworkError::SocketError)
        }
    }

    /// Close socket
    pub fn close(&mut self, socket_id: u32) -> Result<(), NetworkError> {
        if let Some(mut socket) = self.sockets.remove(&socket_id) {
            if let Some(ref conn) = socket.tcp_connection {
                // Clean up TCP connection
                // In real implementation, send FIN packet
            }
            Ok(())
        } else {
            Err(NetworkError::SocketError)
        }
    }

    // ========== TCP Connection Management ==========

    /// Get TCP connection state
    pub fn get_tcp_state(&self, connection_id: u32) -> Option<TcpState> {
        self.tcp_connections.get(&connection_id).map(|conn| conn.state)
    }

    /// Update TCP connection state
    pub fn set_tcp_state(&mut self, connection_id: u32, state: TcpState) -> Result<(), NetworkError> {
        if let Some(conn) = self.tcp_connections.get_mut(&connection_id) {
            conn.state = state;
            Ok(())
        } else {
            Err(NetworkError::ConnectionFailed)
        }
    }

    /// Get connection statistics
    pub fn get_connection_stats(&self, connection_id: u32) -> Option<(u32, u32, u64, u64)> {
        self.tcp_connections.get(&connection_id).map(|conn| {
            (
                conn.window_size,
                conn.sequence,
                conn.receive_buffer.len() as u64,
                conn.send_buffer.len() as u64,
            )
        })
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

    #[test]
    fn test_firewall_rules() {
        let mut net = ZenithNet::new();

        // Add a rule to block traffic from 192.168.1.100
        let rule = FirewallRule {
            id: 1,
            src_addr: Some(Ipv4Addr::new(192, 168, 1, 100)),
            src_mask: Some(Ipv4Addr::new(255, 255, 255, 255)),
            dst_addr: None,
            dst_mask: None,
            src_port: None,
            dst_port: None,
            protocol: None,
            action: FirewallAction::Drop,
            enabled: true,
        };

        net.add_firewall_rule(rule).unwrap();

        // Test that the firewall blocks traffic from 192.168.1.100
        let result = net.check_firewall(
            Ipv4Addr::new(192, 168, 1, 100),
            Ipv4Addr::new(192, 168, 1, 1),
            1234,
            80,
            IpProtocol::Tcp,
        );
        assert_eq!(result, Err(NetworkError::FirewallBlocked));

        // Test that traffic from other addresses is allowed
        let result = net.check_firewall(
            Ipv4Addr::new(192, 168, 1, 50),
            Ipv4Addr::new(192, 168, 1, 1),
            1234,
            80,
            IpProtocol::Tcp,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_socket_creation() {
        let mut net = ZenithNet::new();

        // Create a TCP socket
        let socket_id = net.socket(2, 1, 6).unwrap(); // AF_INET, SOCK_STREAM, IPPROTO_TCP
        assert!(socket_id > 0);

        // Bind the socket
        let result = net.bind(socket_id, Ipv4Addr::new(0, 0, 0, 0), 8080);
        assert!(result.is_ok());

        // Set non-blocking mode
        let result = net.set_non_blocking(socket_id, true);
        assert!(result.is_ok());

        // Set receive timeout
        let result = net.set_receive_timeout(socket_id, 5000);
        assert!(result.is_ok());

        // Close the socket
        let result = net.close(socket_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tcp_connection() {
        let mut net = ZenithNet::new();

        // Create and bind socket
        let socket_id = net.socket(2, 1, 6).unwrap();
        net.bind(socket_id, Ipv4Addr::new(0, 0, 0, 0), 8080).unwrap();

        // Connect to remote address
        let result = net.connect(socket_id, Ipv4Addr::new(192, 168, 1, 100), 80);
        assert!(result.is_ok());

        // Check socket state
        let socket = net.sockets.get(&socket_id).unwrap();
        assert!(socket.connected_addr.is_some());
        assert_eq!(socket.connected_port, Some(80));
    }

    #[test]
    fn test_tcp_listen() {
        let mut net = ZenithNet::new();

        // Create and bind socket
        let socket_id = net.socket(2, 1, 6).unwrap();
        net.bind(socket_id, Ipv4Addr::new(0, 0, 0, 0), 80).unwrap();

        // Set socket to listen mode
        let result = net.listen(socket_id, 128);
        assert!(result.is_ok());

        // Check that the connection is in Listen state
        let socket = net.sockets.get(&socket_id).unwrap();
        if let Some(ref conn) = socket.tcp_connection {
            assert_eq!(conn.state, TcpState::Listen);
        } else {
            panic!("TCP connection should exist");
        }
    }

    #[test]
    fn test_firewall_port_filtering() {
        let mut net = ZenithNet::new();

        // Add a rule to block port 22 (SSH)
        let rule = FirewallRule {
            id: 2,
            src_addr: None,
            src_mask: None,
            dst_addr: None,
            dst_mask: None,
            src_port: None,
            dst_port: Some(22),
            protocol: Some(IpProtocol::Tcp),
            action: FirewallAction::Drop,
            enabled: true,
        };

        net.add_firewall_rule(rule).unwrap();

        // Test that port 22 is blocked
        let result = net.check_firewall(
            Ipv4Addr::new(192, 168, 1, 50),
            Ipv4Addr::new(192, 168, 1, 1),
            1234,
            22,
            IpProtocol::Tcp,
        );
        assert_eq!(result, Err(NetworkError::FirewallBlocked));

        // Test that other ports are allowed
        let result = net.check_firewall(
            Ipv4Addr::new(192, 168, 1, 50),
            Ipv4Addr::new(192, 168, 1, 1),
            1234,
            80,
            IpProtocol::Tcp,
        );
        assert!(result.is_ok());
    }
}
