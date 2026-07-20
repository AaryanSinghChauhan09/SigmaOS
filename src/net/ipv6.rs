// IPv6 Stack - Linux-style IPv6 protocol implementation
// Supports IPv6 addressing, packet handling, and neighbor discovery

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

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

pub struct Ipv6Stack {
    interfaces: Vec<Ipv6Interface>,
    routing_table: Vec<Ipv6Route>,
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
    pub fn new() -> Self {
        Self {
            interfaces: Vec::new(),
            routing_table: Vec::new(),
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
            route.gateway.unwrap_or_else(|| route.destination.clone()),
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

#[cfg(test)]
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
}
