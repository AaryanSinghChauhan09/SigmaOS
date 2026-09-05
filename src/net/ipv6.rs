#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::vec;
use std::format;
// SigmaOS Network Protocol Layer

// IPv6 Stack - Linux-style IPv6 protocol implementation
// Supports IPv6 addressing, packet handling, and neighbor discovery

// (no_std only applicable at crate root - removed)

use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ipv6AddressType {
    Unicast,
    Multicast,
    Anycast,
}

#[derive(Debug, Clone)]
pub struct Ipv6Address {
    pub bytes: [u8; 16],
}

impl Ipv6Address {
    pub fn new(bytes: [u8; 16]) -> Self {
        Self { bytes }
    }

    pub fn loopback() -> Self {
        let mut bytes = [0u8; 16];
        bytes[15] = 1;
        Self { bytes }
    }

    pub fn unspecified() -> Self {
        Self { bytes: [0u8; 16] }
    }

    pub fn is_loopback(&self) -> bool {
        self.bytes == [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
    }

    pub fn is_unspecified(&self) -> bool {
        self.bytes == [0u8; 16]
    }

    pub fn address_type(&self) -> Ipv6AddressType {
        if self.bytes[0] == 0xff {
            Ipv6AddressType::Multicast
        } else {
            Ipv6AddressType::Unicast
        }
    }

    pub fn to_string(&self) -> String {
        // Simplified IPv6 string representation
        let mut parts = Vec::new();
        for i in (0..16).step_by(2) {
            let val = ((self.bytes[i] as u16) << 8) | (self.bytes[i + 1] as u16);
            parts.push(format!("{:x}", val));
        }
        parts.join(":")
    }
}

#[derive(Debug, Clone)]
pub struct Ipv6Header {
    pub version: u8,
    pub traffic_class: u8,
    pub flow_label: u32,
    pub payload_length: u16,
    pub next_header: u8,
    pub hop_limit: u8,
    pub source: Ipv6Address,
    pub destination: Ipv6Address,
}

impl Ipv6Header {
    pub fn new(source: Ipv6Address, destination: Ipv6Address, payload_length: u16, next_header: u8) -> Self {
        Self {
            version: 6,
            traffic_class: 0,
            flow_label: 0,
            payload_length,
            next_header,
            hop_limit: 64,
            source,
            destination,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        
        // Version (4 bits) + Traffic Class (8 bits) + Flow Label (20 bits)
        let version_tc_fl = ((self.version as u32) << 28) 
                          | ((self.traffic_class as u32) << 20) 
                          | (self.flow_label & 0xFFFFF);
        
        buffer.extend_from_slice(&version_tc_fl.to_be_bytes());
        buffer.extend_from_slice(&self.payload_length.to_be_bytes());
        buffer.push(self.next_header);
        buffer.push(self.hop_limit);
        buffer.extend_from_slice(&self.source.bytes);
        buffer.extend_from_slice(&self.destination.bytes);

        buffer
    }

    pub fn parse(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < 40 {
            return Err("IPv6 header too short");
        }

        let version_tc_fl = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        let version = (version_tc_fl >> 28) as u8;
        let traffic_class = ((version_tc_fl >> 20) & 0xFF) as u8;
        let flow_label = version_tc_fl & 0xFFFFF;

        let payload_length = u16::from_be_bytes([data[4], data[5]]);
        let next_header = data[6];
        let hop_limit = data[7];

        let mut source_bytes = [0u8; 16];
        source_bytes.copy_from_slice(&data[8..24]);

        let mut dest_bytes = [0u8; 16];
        dest_bytes.copy_from_slice(&data[24..40]);

        Ok(Self {
            version,
            traffic_class,
            flow_label,
            payload_length,
            next_header,
            hop_limit,
            source: Ipv6Address::new(source_bytes),
            destination: Ipv6Address::new(dest_bytes),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ipv6ExtensionHeader {
    HopByHopOptions,
    Routing,
    Fragment,
    EncapsulatingSecurityPayload,
    Authentication,
    DestinationOptions,
    NoNextHeader,
}

// =========================================================================
// ICMPv6 Core Protocol Handling
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icmpv6Type {
    EchoRequest = 128,
    EchoReply = 129,
    RouterSolicitation = 133,
    RouterAdvertisement = 134,
    NeighborSolicitation = 135,
    NeighborAdvertisement = 136,
}

#[derive(Debug, Clone)]
pub struct Icmpv6Packet {
    pub message_type: Icmpv6Type,
    pub code: u8,
    pub checksum: u16,
    pub payload: Vec<u8>,
}

impl Icmpv6Packet {
    pub fn new(message_type: Icmpv6Type, code: u8, payload: Vec<u8>) -> Self {
        let mut packet = Self {
            message_type,
            code,
            checksum: 0,
            payload,
        };
        packet.checksum = packet.calculate_checksum();
        packet
    }

    pub fn calculate_checksum(&self) -> u16 {
        let mut sum: u32 = (self.message_type as u32) + (self.code as u32);
        for chunk in self.payload.chunks(2) {
            let word = if chunk.len() == 2 {
                ((chunk[0] as u32) << 8) | (chunk[1] as u32)
            } else {
                (chunk[0] as u32) << 8
            };
            sum = sum.wrapping_add(word);
        }
        while (sum >> 16) != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        !(sum as u16)
    }

    /// Calculates ICMPv6 checksum using the IPv6 Pseudo-Header (RFC 4443)
    pub fn calculate_pseudo_header_checksum(&self, source: &Ipv6Address, destination: &Ipv6Address) -> u16 {
        let payload_bytes = self.serialize();
        let payload_len = payload_bytes.len() as u32;

        let mut sum: u32 = 0;
        for chunk in source.bytes.chunks(2) {
            sum = sum.wrapping_add(((chunk[0] as u32) << 8) | (chunk[1] as u32));
        }
        for chunk in destination.bytes.chunks(2) {
            sum = sum.wrapping_add(((chunk[0] as u32) << 8) | (chunk[1] as u32));
        }
        sum = sum.wrapping_add(payload_len >> 16);
        sum = sum.wrapping_add(payload_len & 0xFFFF);
        sum = sum.wrapping_add(58); // Next Header = 58 (ICMPv6)

        for chunk in payload_bytes.chunks(2) {
            let word = if chunk.len() == 2 {
                ((chunk[0] as u32) << 8) | (chunk[1] as u32)
            } else {
                (chunk[0] as u32) << 8
            };
            sum = sum.wrapping_add(word);
        }

        while (sum >> 16) != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        !(sum as u16)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(self.message_type as u8);
        buf.push(self.code);
        buf.extend_from_slice(&self.checksum.to_be_bytes());
        buf.extend_from_slice(&self.payload);
        buf
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 4 {
            return Err("ICMPv6 packet too short");
        }
        let msg_type = match bytes[0] {
            128 => Icmpv6Type::EchoRequest,
            129 => Icmpv6Type::EchoReply,
            133 => Icmpv6Type::RouterSolicitation,
            134 => Icmpv6Type::RouterAdvertisement,
            135 => Icmpv6Type::NeighborSolicitation,
            136 => Icmpv6Type::NeighborAdvertisement,
            _ => return Err("Unsupported ICMPv6 message type"),
        };
        let code = bytes[1];
        let checksum = u16::from_be_bytes([bytes[2], bytes[3]]);
        let payload = bytes[4..].to_vec();

        Ok(Self {
            message_type: msg_type,
            code,
            checksum,
            payload,
        })
    }
}

// =========================================================================
// Neighbor Discovery Option TLVs (RFC 4861)
// =========================================================================

#[derive(Debug, Clone)]
pub enum NdpOption {
    SourceLinkLayerAddress([u8; 6]),
    TargetLinkLayerAddress([u8; 6]),
    PrefixInformation {
        prefix_length: u8,
        on_link: bool,
        autonomous: bool,
        valid_lifetime: u32,
        preferred_lifetime: u32,
        prefix: Ipv6Address,
    },
}

impl NdpOption {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            NdpOption::SourceLinkLayerAddress(mac) => {
                buf.push(1); // Type 1
                buf.push(1); // Length in 8-octet units (1 = 8 bytes)
                buf.extend_from_slice(mac);
            }
            NdpOption::TargetLinkLayerAddress(mac) => {
                buf.push(2); // Type 2
                buf.push(1); // Length
                buf.extend_from_slice(mac);
            }
            NdpOption::PrefixInformation { prefix_length, on_link, autonomous, valid_lifetime, preferred_lifetime, prefix } => {
                buf.push(3); // Type 3
                buf.push(4); // Length (32 bytes)
                buf.push(*prefix_length);
                let flags = (if *on_link { 0x80 } else { 0 }) | (if *autonomous { 0x40 } else { 0 });
                buf.push(flags);
                buf.extend_from_slice(&valid_lifetime.to_be_bytes());
                buf.extend_from_slice(&preferred_lifetime.to_be_bytes());
                buf.extend_from_slice(&[0u8; 4]); // Reserved
                buf.extend_from_slice(&prefix.bytes);
            }
        }
        buf
    }
}

// =========================================================================
// DHCPv6 Option TLVs (RFC 8415)
// =========================================================================

#[derive(Debug, Clone)]
pub struct Dhcpv6Option {
    pub option_code: u16,
    pub option_data: Vec<u8>,
}

impl Dhcpv6Option {
    pub fn new(option_code: u16, option_data: Vec<u8>) -> Self {
        Self { option_code, option_data }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.option_code.to_be_bytes());
        buf.extend_from_slice(&(self.option_data.len() as u16).to_be_bytes());
        buf.extend_from_slice(&self.option_data);
        buf
    }
}

// =========================================================================
// DHCPv6 Client State Machine
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dhcpv6State {
    Init,
    Solicit,
    Request,
    Bound,
    Renew,
}

pub struct Dhcpv6Client {
    pub state: Dhcpv6State,
    pub transaction_id: u32,
    pub client_duid: Vec<u8>,
    pub assigned_address: Option<Ipv6Address>,
    pub preferred_lifetime: u32,
    pub valid_lifetime: u32,
}

impl Dhcpv6Client {
    pub fn new(client_duid: Vec<u8>) -> Self {
        Self {
            state: Dhcpv6State::Init,
            transaction_id: 0x123456,
            client_duid,
            assigned_address: None,
            preferred_lifetime: 3600,
            valid_lifetime: 7200,
        }
    }

    pub fn send_solicit(&mut self) -> Result<Vec<u8>, &'static str> {
        if self.state != Dhcpv6State::Init && self.state != Dhcpv6State::Solicit {
            return Err("DHCPv6 client not in valid state for Solicit");
        }
        self.state = Dhcpv6State::Solicit;
        let mut msg = Vec::new();
        msg.push(1); // SOLICIT message type
        msg.extend_from_slice(&self.transaction_id.to_be_bytes()[1..4]);
        msg.extend_from_slice(&self.client_duid);
        Ok(msg)
    }

    pub fn handle_advertise(&mut self, advertised_address: Ipv6Address) -> Result<Vec<u8>, &'static str> {
        if self.state != Dhcpv6State::Solicit {
            return Err("DHCPv6 client not expecting Advertise");
        }
        self.state = Dhcpv6State::Request;
        let mut msg = Vec::new();
        msg.push(3); // REQUEST message type
        msg.extend_from_slice(&self.transaction_id.to_be_bytes()[1..4]);
        msg.extend_from_slice(&advertised_address.bytes);
        Ok(msg)
    }

    pub fn handle_reply(&mut self, assigned_address: Ipv6Address) -> Result<(), &'static str> {
        if self.state != Dhcpv6State::Request && self.state != Dhcpv6State::Renew {
            return Err("DHCPv6 client not expecting Reply");
        }
        self.assigned_address = Some(assigned_address);
        self.state = Dhcpv6State::Bound;
        Ok(())
    }
}

// =========================================================================
// IPv6 Multicast Routing & Group Membership Engine
// =========================================================================

#[derive(Debug, Clone)]
pub struct MulticastGroup {
    pub group_address: Ipv6Address,
    pub member_interfaces: Vec<String>,
}

pub struct Ipv6MulticastRouter {
    pub groups: Vec<MulticastGroup>,
}

impl Ipv6MulticastRouter {
    pub fn new() -> Self {
        Self { groups: Vec::new() }
    }

    pub fn join_group(&mut self, group_address: Ipv6Address, interface: &str) -> Result<(), &'static str> {
        if group_address.address_type() != Ipv6AddressType::Multicast {
            return Err("Provided address is not an IPv6 multicast address");
        }

        for group in self.groups.iter_mut() {
            if group.group_address.bytes == group_address.bytes {
                if !group.member_interfaces.iter().any(|i| i == interface) {
                    group.member_interfaces.push(interface.to_string());
                }
                return Ok(());
            }
        }

        self.groups.push(MulticastGroup {
            group_address,
            member_interfaces: vec![interface.to_string()],
        });

        Ok(())
    }

    pub fn forward_multicast_packet(&self, group_address: &Ipv6Address, payload: &[u8]) -> Vec<String> {
        for group in &self.groups {
            if group.group_address.bytes == group_address.bytes {
                return group.member_interfaces.clone();
            }
        }
        Vec::new()
    }
}

pub struct Ipv6Stack {
    interfaces: Vec<Ipv6Interface>,
    routing_table: Vec<Ipv6Route>,
    pub icmpv6_enabled: bool,
    pub dhcpv6_client: Option<Dhcpv6Client>,
    pub multicast_router: Ipv6MulticastRouter,
}

#[derive(Debug, Clone)]
pub struct Ipv6Interface {
    pub name: String,
    pub address: Ipv6Address,
    pub prefix_length: u8,
    pub mtu: u16,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct Ipv6Route {
    pub destination: Ipv6Address,
    pub prefix_length: u8,
    pub gateway: Option<Ipv6Address>,
    pub interface: String,
    pub metric: u32,
}

impl Ipv6Stack {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            interfaces: Vec::new(),
            routing_table: Vec::new(),
            icmpv6_enabled: true,
            dhcpv6_client: None,
            multicast_router: Ipv6MulticastRouter::new(),
        }
    }

    /// Add an IPv6 interface
    pub fn add_interface(&mut self, interface: Ipv6Interface) -> Result<(), &'static str> {
        self.interfaces.push(interface);
        Ok(())
    }

    /// Add a route to the routing table
    pub fn add_route(&mut self, route: Ipv6Route) -> Result<(), &'static str> {
        self.routing_table.push(route);
        Ok(())
    }

    /// Find the best route for a destination
    pub fn find_route(&self, destination: &Ipv6Address) -> Option<&Ipv6Route> {
        let mut best_route = None;
        let mut best_metric = u32::MAX;

        for route in &self.routing_table {
            if self.matches_prefix(destination, &route.destination, route.prefix_length) {
                if route.metric < best_metric {
                    best_metric = route.metric;
                    best_route = Some(route);
                }
            }
        }

        best_route
    }

    /// Check if an address matches a prefix
    fn matches_prefix(&self, address: &Ipv6Address, prefix: &Ipv6Address, prefix_length: u8) -> bool {
        let full_bytes = (prefix_length / 8) as usize;
        let remaining_bits = prefix_length % 8;

        for i in 0..full_bytes {
            if address.bytes[i] != prefix.bytes[i] {
                return false;
            }
        }

        if remaining_bits > 0 && full_bytes < 16 {
            let mask = 0xFF << (8 - remaining_bits);
            if (address.bytes[full_bytes] & mask) != (prefix.bytes[full_bytes] & mask) {
                return false;
            }
        }

        true
    }

    /// Send an IPv6 packet
    pub fn send_packet(&self, destination: Ipv6Address, payload: Vec<u8>, next_header: u8) -> Result<(), &'static str> {
        let route = self.find_route(&destination)
            .ok_or("No route to destination")?;

        let header = Ipv6Header::new(
            route.gateway.clone().unwrap_or_else(|| route.destination.clone()),
            destination,
            payload.len() as u16,
            next_header,
        );

        // In a real implementation, this would send the packet
        Ok(())
    }

    /// Get interface count
    pub fn interface_count(&self) -> usize {
        self.interfaces.len()
    }

    /// Get route count
    pub fn route_count(&self) -> usize {
        self.routing_table.len()
    }
}

impl Default for Ipv6Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv6_address_loopback() {
        let addr = Ipv6Address::loopback();
        assert!(addr.is_loopback());
        assert!(!addr.is_unspecified());
    }

    #[test]
    fn test_ipv6_address_unspecified() {
        let addr = Ipv6Address::unspecified();
        assert!(addr.is_unspecified());
        assert!(!addr.is_loopback());
    }

    #[test]
    fn test_ipv6_header_serialization() {
        let source = Ipv6Address::loopback();
        let dest = Ipv6Address::unspecified();
        
        let header = Ipv6Header::new(source, dest, 0, 58);
        let serialized = header.serialize();
        
        assert_eq!(serialized.len(), 40);
    }

    #[test]
    fn test_ipv6_header_parsing() {
        let source = Ipv6Address::loopback();
        let dest = Ipv6Address::unspecified();
        
        let header = Ipv6Header::new(source, dest, 0, 58);
        let serialized = header.serialize();
        
        let parsed = Ipv6Header::parse(&serialized).unwrap();
        assert_eq!(parsed.version, 6);
        assert!(parsed.source.is_loopback());
    }

    #[test]
    fn test_ipv6_stack() {
        let mut stack = Ipv6Stack::new();
        
        let interface = Ipv6Interface {
            name: "eth0".to_string(),
            address: Ipv6Address::loopback(),
            prefix_length: 128,
            mtu: 1500,
            enabled: true,
        };
        
        stack.add_interface(interface).unwrap();
        assert_eq!(stack.interface_count(), 1);
    }

    #[test]
    fn test_ipv6_routing() {
        let mut stack = Ipv6Stack::new();
        
        let route = Ipv6Route {
            destination: Ipv6Address::loopback(),
            prefix_length: 128,
            gateway: None,
            interface: "lo".to_string(),
            metric: 1,
        };
        
        stack.add_route(route).unwrap();
        assert_eq!(stack.route_count(), 1);
    }

    #[test]
    fn test_prefix_matching() {
        let stack = Ipv6Stack::new();
        
        let addr1 = Ipv6Address::loopback();
        let addr2 = Ipv6Address::loopback();
        
        assert!(stack.matches_prefix(&addr1, &addr2, 128));
    }

    #[test]
    fn test_icmpv6_packet_serialization_and_parsing() {
        let pkt = Icmpv6Packet::new(Icmpv6Type::EchoRequest, 0, vec![1, 2, 3, 4, 5, 6]);
        let serialized = pkt.serialize();
        let parsed = Icmpv6Packet::parse(&serialized).unwrap();
        assert_eq!(parsed.message_type, Icmpv6Type::EchoRequest);
        assert_eq!(parsed.code, 0);
        assert_eq!(parsed.payload, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_dhcpv6_client_flow() {
        let mut client = Dhcpv6Client::new(vec![0x00, 0x01, 0x02, 0x03]);
        assert_eq!(client.state, Dhcpv6State::Init);

        let solicit_bytes = client.send_solicit().unwrap();
        assert_eq!(client.state, Dhcpv6State::Solicit);
        assert_eq!(solicit_bytes[0], 1); // Solicit msg type

        let adv_addr = Ipv6Address::new([0xff, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        let req_bytes = client.handle_advertise(adv_addr).unwrap();
        assert_eq!(client.state, Dhcpv6State::Request);
        assert_eq!(req_bytes[0], 3); // Request msg type

        let assigned = Ipv6Address::new([0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100]);
        client.handle_reply(assigned.clone()).unwrap();
        assert_eq!(client.state, Dhcpv6State::Bound);
        assert_eq!(client.assigned_address.unwrap().bytes, assigned.bytes);
    }

    #[test]
    fn test_multicast_router() {
        let mut router = Ipv6MulticastRouter::new();
        let mc_addr = Ipv6Address::new([0xff, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);

        router.join_group(mc_addr.clone(), "eth0").unwrap();
        router.join_group(mc_addr.clone(), "eth1").unwrap();

        let interfaces = router.forward_multicast_packet(&mc_addr, &[0xAA, 0xBB]);
        assert_eq!(interfaces.len(), 2);
        assert_eq!(interfaces[0], "eth0");
        assert_eq!(interfaces[1], "eth1");
    }

    #[test]
    fn test_icmpv6_pseudo_header_checksum() {
        let src = Ipv6Address::loopback();
        let dst = Ipv6Address::loopback();
        let pkt = Icmpv6Packet::new(Icmpv6Type::EchoRequest, 0, vec![1, 2, 3, 4]);
        let csum = pkt.calculate_pseudo_header_checksum(&src, &dst);
        assert_ne!(csum, 0);
    }

    #[test]
    fn test_ndp_and_dhcpv6_option_tlvs() {
        let mac_opt = NdpOption::SourceLinkLayerAddress([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        let mac_serialized = mac_opt.serialize();
        assert_eq!(mac_serialized.len(), 8);
        assert_eq!(mac_serialized[0], 1); // Type 1

        let dhcp_opt = Dhcpv6Option::new(1, vec![0xDE, 0xAD, 0xBE, 0xEF]);
        let dhcp_serialized = dhcp_opt.serialize();
        assert_eq!(dhcp_serialized.len(), 8); // 2 bytes code + 2 bytes len + 4 bytes data
    }
}
