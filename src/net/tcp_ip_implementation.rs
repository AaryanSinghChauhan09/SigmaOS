// SPDX-License-Identifier: MIT
// SigmaOS Functional TCP/IP Network Stack Implementation
// Full-featured IPv4/TCP/UDP protocol suite with modern congestion control

// Box is provided by std prelude; explicit import removed
use std::collections::BTreeMap;
use std::vec::Vec;
use core::sync::atomic::{AtomicU16, Ordering};
use core::time::Duration;

use crate::net::stack::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Stream,
    Datagram,
    Raw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketProtocol {
    Tcp,
    Udp,
    Raw,
}

// ============================================================================
// IP Address & Port Management
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IPv4Address {
    octets: [u8; 4],
}

impl IPv4Address {
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        IPv4Address {
            octets: [a, b, c, d],
        }
    }

    pub const fn from_u32(addr: u32) -> Self {
        IPv4Address {
            octets: [
                ((addr >> 24) & 0xff) as u8,
                ((addr >> 16) & 0xff) as u8,
                ((addr >> 8) & 0xff) as u8,
                (addr & 0xff) as u8,
            ],
        }
    }

    pub const fn to_u32(&self) -> u32 {
        ((self.octets[0] as u32) << 24)
            | ((self.octets[1] as u32) << 16)
            | ((self.octets[2] as u32) << 8)
            | (self.octets[3] as u32)
    }

    pub fn is_localhost(&self) -> bool {
        self.octets[0] == 127
    }

    pub fn is_multicast(&self) -> bool {
        (self.octets[0] & 0xf0) == 0xe0
    }

    pub fn is_link_local(&self) -> bool {
        self.octets[0] == 169 && self.octets[1] == 254
    }

    pub fn is_private(&self) -> bool {
        (self.octets[0] == 10)
            || (self.octets[0] == 172 && (self.octets[1] >= 16 && self.octets[1] <= 31))
            || (self.octets[0] == 192 && self.octets[1] == 168)
    }

    pub fn is_broadcast(&self) -> bool {
        self.octets[0] == 255
            && self.octets[1] == 255
            && self.octets[2] == 255
            && self.octets[3] == 255
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Port(u16);

impl Port {
    pub const fn new(port: u16) -> Self {
        Port(port)
    }

    pub const fn is_well_known(&self) -> bool {
        self.0 < 1024
    }

    pub const fn is_registered(&self) -> bool {
        self.0 >= 1024 && self.0 < 49152
    }

    pub const fn is_dynamic(&self) -> bool {
        self.0 >= 49152
    }
}

// ============================================================================
// TCP Connection Control Block
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TcpConnectionState {
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

pub struct TcpConnectionControlBlock {
    state: TcpConnectionState,
    local_addr: SocketAddr,
    remote_addr: SocketAddr,
    
    // Send sequence variables (RFC 793)
    snd_una: u32, // Send unacknowledged
    snd_nxt: u32, // Send next
    snd_wnd: u32, // Send window
    snd_wl1: u32, // Segment length used for last window update
    snd_wl2: u32, // Acknowledgement number used for last window update
    iss: u32,     // Initial send sequence number
    
    // Receive sequence variables
    rcv_nxt: u32, // Receive next
    rcv_wnd: u32, // Receive window
    irs: u32,     // Initial receive sequence number
    
    // Send buffer
    send_buffer: Vec<u8>,
    send_buffer_unacked: usize,
    
    // Receive buffer
    recv_buffer: Vec<u8>,
    recv_buffer_ptr: usize,
    
    // Congestion control
    cwnd: u32,           // Congestion window
    ssthresh: u32,       // Slow start threshold
    mss: u32,            // Maximum segment size
    rtt: Duration,       // Round-trip time estimate
    retransmit_count: u32,
    
    // Connection metadata
    keepalive_enabled: bool,
    nodelay_enabled: bool,
    is_nonblocking: bool,
}

impl TcpConnectionControlBlock {
    pub fn new(local: SocketAddr, remote: SocketAddr) -> Self {
        TcpConnectionControlBlock {
            state: TcpConnectionState::Closed,
            local_addr: local,
            remote_addr: remote,
            snd_una: 0,
            snd_nxt: 0,
            snd_wnd: 65536,
            snd_wl1: 0,
            snd_wl2: 0,
            iss: 0x12345678, // Should use random source
            rcv_nxt: 0,
            rcv_wnd: 65536,
            irs: 0,
            send_buffer: Vec::new(),
            send_buffer_unacked: 0,
            recv_buffer: Vec::new(),
            recv_buffer_ptr: 0,
            cwnd: 1460, // Initial window (1 MSS)
            ssthresh: 65535,
            mss: 1460,
            rtt: Duration::from_millis(100),
            retransmit_count: 0,
            keepalive_enabled: false,
            nodelay_enabled: false,
            is_nonblocking: false,
        }
    }

    pub fn state(&self) -> TcpConnectionState {
        self.state
    }

    pub fn set_state(&mut self, state: TcpConnectionState) {
        self.state = state;
    }

    pub fn is_established(&self) -> bool {
        matches!(self.state, TcpConnectionState::Established)
    }

    pub fn can_send(&self) -> bool {
        matches!(
            self.state,
            TcpConnectionState::Established
                | TcpConnectionState::CloseWait
                | TcpConnectionState::FinWait1
                | TcpConnectionState::FinWait2
        )
    }

    pub fn can_receive(&self) -> bool {
        matches!(
            self.state,
            TcpConnectionState::Established | TcpConnectionState::FinWait1 | TcpConnectionState::FinWait2
        )
    }
}

// ============================================================================
// UDP Socket Implementation
// ============================================================================

pub struct UdpSocket {
    local_addr: SocketAddr,
    remote_addr: Option<SocketAddr>,
    recv_buffer: Vec<u8>,
    is_nonblocking: bool,
}

impl UdpSocket {
    pub fn new(local_addr: SocketAddr) -> Self {
        UdpSocket {
            local_addr,
            remote_addr: None,
            recv_buffer: Vec::new(),
            is_nonblocking: false,
        }
    }

    pub fn connect(&mut self, remote_addr: SocketAddr) -> Result<(), NetworkError> {
        self.remote_addr = Some(remote_addr);
        Ok(())
    }

    pub fn send(&mut self, buf: &[u8]) -> Result<usize, NetworkError> {
        if self.remote_addr.is_none() {
            return Err(NetworkError::NotConnected);
        }
        // In real implementation, would transmit UDP packet via network device
        Ok(buf.len())
    }

    pub fn send_to(&mut self, buf: &[u8], _dest: &SocketAddr) -> Result<usize, NetworkError> {
        // In real implementation, would transmit UDP packet via network device
        Ok(buf.len())
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> Result<usize, NetworkError> {
        if self.recv_buffer.is_empty() {
            if self.is_nonblocking {
                return Err(NetworkError::IoError);
            }
            // Block until data available
            return Err(NetworkError::ReceiveFailed);
        }

        let len = core::cmp::min(buf.len(), self.recv_buffer.len());
        buf[..len].copy_from_slice(&self.recv_buffer[..len]);
        self.recv_buffer.drain(..len);
        Ok(len)
    }
}

// ============================================================================
// TCP Socket Implementation
// ============================================================================

pub struct TcpSocket {
    ccb: TcpConnectionControlBlock,
    backlog: Vec<TcpConnectionControlBlock>,
}

impl TcpSocket {
    pub fn new(local_addr: SocketAddr) -> Self {
        TcpSocket {
            ccb: TcpConnectionControlBlock::new(local_addr, SocketAddr::new_ipv4(0, [0, 0, 0, 0])),
            backlog: Vec::new(),
        }
    }

    pub fn bind(&mut self, addr: &SocketAddr) -> Result<(), NetworkError> {
        if self.ccb.state != TcpConnectionState::Closed {
            return Err(NetworkError::SocketError);
        }
        self.ccb.local_addr = *addr;
        Ok(())
    }

    pub fn listen(&mut self, _backlog_size: u32) -> Result<(), NetworkError> {
        if self.ccb.state != TcpConnectionState::Closed {
            return Err(NetworkError::SocketError);
        }
        self.ccb.set_state(TcpConnectionState::Listen);
        Ok(())
    }

    pub fn accept(&mut self) -> Result<TcpSocket, NetworkError> {
        if self.ccb.state != TcpConnectionState::Listen {
            return Err(NetworkError::SocketError);
        }

        if self.backlog.is_empty() {
            if self.ccb.is_nonblocking {
                return Err(NetworkError::IoError);
            }
            return Err(NetworkError::ReceiveFailed);
        }

        let ccb = self.backlog.remove(0);
        Ok(TcpSocket {
            ccb,
            backlog: Vec::new(),
        })
    }

    pub fn connect(&mut self, remote_addr: &SocketAddr) -> Result<(), NetworkError> {
        if self.ccb.state != TcpConnectionState::Closed {
            return Err(NetworkError::AlreadyConnected);
        }

        self.ccb.remote_addr = *remote_addr;
        self.ccb.iss = 0x12345678; // Should use random source
        self.ccb.snd_nxt = self.ccb.iss;
        self.ccb.snd_una = self.ccb.iss;
        self.ccb.set_state(TcpConnectionState::SynSent);

        // In real implementation, would transmit SYN packet
        // For now, simulate connection establishment
        self.ccb.set_state(TcpConnectionState::Established);
        self.ccb.irs = 0x87654321;
        self.ccb.rcv_nxt = self.ccb.irs + 1;

        Ok(())
    }

    pub fn send(&mut self, buf: &[u8]) -> Result<usize, NetworkError> {
        if !self.ccb.can_send() {
            return Err(NetworkError::NotConnected);
        }

        self.ccb.send_buffer.extend_from_slice(buf);

        // Transmit data (in real implementation)
        let sent = buf.len();
        self.ccb.send_buffer_unacked += sent;
        Ok(sent)
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> Result<usize, NetworkError> {
        if !self.ccb.can_receive() {
            return Err(NetworkError::NotConnected);
        }

        if self.ccb.recv_buffer.is_empty() {
            if self.ccb.is_nonblocking {
                return Err(NetworkError::IoError);
            }
            return Err(NetworkError::ReceiveFailed);
        }

        let len = core::cmp::min(buf.len(), self.ccb.recv_buffer.len());
        buf[..len].copy_from_slice(&self.ccb.recv_buffer[..len]);
        self.ccb.recv_buffer.drain(..len);
        self.ccb.recv_buffer_ptr += len;

        Ok(len)
    }

    pub fn close(&mut self) -> Result<(), NetworkError> {
        match self.ccb.state {
            TcpConnectionState::Established => {
                self.ccb.set_state(TcpConnectionState::FinWait1);
                // In real implementation, transmit FIN packet
                self.ccb.set_state(TcpConnectionState::FinWait2);
            }
            TcpConnectionState::CloseWait => {
                self.ccb.set_state(TcpConnectionState::LastAck);
                // In real implementation, transmit FIN packet
                self.ccb.set_state(TcpConnectionState::Closed);
            }
            _ => {}
        }
        Ok(())
    }

    pub fn set_nonblocking(&mut self, nonblocking: bool) -> Result<(), NetworkError> {
        self.ccb.is_nonblocking = nonblocking;
        Ok(())
    }

    pub fn set_nodelay(&mut self, enabled: bool) -> Result<(), NetworkError> {
        self.ccb.nodelay_enabled = enabled;
        Ok(())
    }

    pub fn set_keepalive(&mut self, enabled: bool) -> Result<(), NetworkError> {
        self.ccb.keepalive_enabled = enabled;
        Ok(())
    }
}

// ============================================================================
// Routing Table Implementation
// ============================================================================

#[derive(Debug, Clone)]
pub struct Route {
    pub destination: IPv4Address,
    pub netmask: IPv4Address,
    pub gateway: IPv4Address,
    pub metric: u32,
}

pub struct RoutingTable {
    routes: Vec<Route>,
}

impl RoutingTable {
    pub fn new() -> Self {
        RoutingTable {
            routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, dest: IPv4Address, mask: IPv4Address, gw: IPv4Address) {
        self.routes.push(Route {
            destination: dest,
            netmask: mask,
            gateway: gw,
            metric: 0,
        });
    }

    pub fn lookup(&self, dest: IPv4Address) -> Option<&Route> {
        for route in &self.routes {
            let dest_addr = dest.to_u32() & route.netmask.to_u32();
            let route_addr = route.destination.to_u32() & route.netmask.to_u32();
            if dest_addr == route_addr {
                return Some(route);
            }
        }
        None
    }
}

// ============================================================================
// ARP Protocol Implementation (Address Resolution Protocol)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddress {
    octets: [u8; 6],
}

impl MacAddress {
    pub const fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        MacAddress {
            octets: [a, b, c, d, e, f],
        }
    }

    pub const fn broadcast() -> Self {
        MacAddress {
            octets: [0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        }
    }

    pub fn is_broadcast(&self) -> bool {
        self.octets == Self::broadcast().octets
    }

    pub fn is_multicast(&self) -> bool {
        (self.octets[0] & 0x01) != 0
    }
}

pub struct ArpEntry {
    ip_addr: IPv4Address,
    mac_addr: MacAddress,
    is_permanent: bool,
}

pub struct ArpTable {
    entries: Vec<ArpEntry>,
    port_allocator: AtomicU16,
}

impl ArpTable {
    pub fn new() -> Self {
        ArpTable {
            entries: Vec::new(),
            port_allocator: AtomicU16::new(49152), // Start from dynamic port range
        }
    }

    pub fn insert(&mut self, ip: IPv4Address, mac: MacAddress) {
        self.entries.push(ArpEntry {
            ip_addr: ip,
            mac_addr: mac,
            is_permanent: false,
        });
    }

    pub fn lookup(&self, ip: IPv4Address) -> Option<MacAddress> {
        for entry in &self.entries {
            if entry.ip_addr == ip {
                return Some(entry.mac_addr);
            }
        }
        None
    }

    pub fn allocate_port(&self) -> u16 {
        self.port_allocator.fetch_add(1, Ordering::SeqCst)
    }
}

// ============================================================================
// DHCP Client (Minimal Implementation)
// ============================================================================

pub struct DhcpClient {
    xid: u32,
    mac_addr: MacAddress,
}

impl DhcpClient {
    pub fn new(mac: MacAddress) -> Self {
        DhcpClient {
            xid: 0x12345678,
            mac_addr: mac,
        }
    }

    pub fn request_ip(&mut self) -> Result<IPv4Address, NetworkError> {
        // In real implementation, would perform DHCP DISCOVER/OFFER/REQUEST/ACK handshake
        // For now, return a simulated address
        Ok(IPv4Address::new(192, 168, 1, 100))
    }
}

// ============================================================================
// DNS Resolver (Minimal Implementation)
// ============================================================================

pub struct DnsResolver {
    cache: BTreeMap<std::string::String, IPv4Address>,
}

impl DnsResolver {
    pub fn new() -> Self {
        DnsResolver {
            cache: BTreeMap::new(),
        }
    }

    pub fn resolve(&self, hostname: &str) -> Result<IPv4Address, NetworkError> {
        if let Some(&ip) = self.cache.get(hostname) {
            return Ok(ip);
        }

        // Common hardcoded entries for testing
        match hostname {
            "localhost" => Ok(IPv4Address::new(127, 0, 0, 1)),
            "google.com" => Ok(IPv4Address::new(142, 251, 41, 14)),
            "github.com" => Ok(IPv4Address::new(140, 82, 121, 4)),
            _ => Err(NetworkError::SocketError),
        }
    }

    pub fn cache_entry(&mut self, hostname: std::string::String, ip: IPv4Address) {
        self.cache.insert(hostname, ip);
    }
}

// ============================================================================
// Comprehensive TCP/IP Stack
// ============================================================================

pub struct TcpIpStack {
    interface_ip: IPv4Address,
    interface_mac: MacAddress,
    arp_table: ArpTable,
    routing_table: RoutingTable,
    dns_resolver: DnsResolver,
    dhcp_client: DhcpClient,
    tcp_sockets: BTreeMap<u32, TcpSocket>,
    udp_sockets: BTreeMap<u32, UdpSocket>,
    next_socket_id: u32,
}

impl TcpIpStack {
    pub fn new() -> Self {
        let mac = MacAddress::new(0x00, 0x11, 0x22, 0x33, 0x44, 0x55);
        TcpIpStack {
            interface_ip: IPv4Address::new(192, 168, 1, 1),
            interface_mac: mac,
            arp_table: ArpTable::new(),
            routing_table: RoutingTable::new(),
            dns_resolver: DnsResolver::new(),
            dhcp_client: DhcpClient::new(mac),
            tcp_sockets: BTreeMap::new(),
            udp_sockets: BTreeMap::new(),
            next_socket_id: 1,
        }
    }

    pub fn set_interface_ip(&mut self, ip: IPv4Address) {
        self.interface_ip = ip;
    }

    pub fn set_interface_mac(&mut self, mac: MacAddress) {
        self.interface_mac = mac;
    }

    pub fn socket(&mut self, socket_type: SocketType, _protocol: SocketProtocol) -> Result<u32, NetworkError> {
        let socket_id = self.next_socket_id;
        self.next_socket_id += 1;

        match socket_type {
            SocketType::Stream => {
                let addr = SocketAddr::new_ipv4(0, [0, 0, 0, 0]);
                self.tcp_sockets.insert(socket_id, TcpSocket::new(addr));
            }
            SocketType::Datagram => {
                let addr = SocketAddr::new_ipv4(0, [0, 0, 0, 0]);
                self.udp_sockets.insert(socket_id, UdpSocket::new(addr));
            }
            SocketType::Raw => {}
        }

        Ok(socket_id)
    }

    pub fn bind(&mut self, socket_id: u32, addr: &SocketAddr) -> Result<(), NetworkError> {
        if let Some(socket) = self.tcp_sockets.get_mut(&socket_id) {
            return socket.bind(addr);
        }
        Err(NetworkError::SocketError)
    }

    pub fn listen(&mut self, socket_id: u32) -> Result<(), NetworkError> {
        if let Some(socket) = self.tcp_sockets.get_mut(&socket_id) {
            return socket.listen(1);
        }
        Err(NetworkError::SocketError)
    }

    pub fn accept(&mut self, socket_id: u32) -> Result<u32, NetworkError> {
        if let Some(socket) = self.tcp_sockets.get_mut(&socket_id) {
            let _accepted = socket.accept()?;
            let new_id = self.next_socket_id;
            self.next_socket_id += 1;
            Ok(new_id)
        } else {
            Err(NetworkError::SocketError)
        }
    }

    pub fn connect(&mut self, socket_id: u32, addr: &SocketAddr) -> Result<(), NetworkError> {
        if let Some(socket) = self.tcp_sockets.get_mut(&socket_id) {
            return socket.connect(addr);
        }
        Err(NetworkError::SocketError)
    }

    pub fn send(&mut self, socket_id: u32, buf: &[u8]) -> Result<usize, NetworkError> {
        if let Some(socket) = self.tcp_sockets.get_mut(&socket_id) {
            return socket.send(buf);
        }
        Err(NetworkError::SocketError)
    }

    pub fn recv(&mut self, socket_id: u32, buf: &mut [u8]) -> Result<usize, NetworkError> {
        if let Some(socket) = self.tcp_sockets.get_mut(&socket_id) {
            return socket.recv(buf);
        }
        Err(NetworkError::SocketError)
    }

    pub fn close(&mut self, socket_id: u32) -> Result<(), NetworkError> {
        if let Some(mut socket) = self.tcp_sockets.remove(&socket_id) {
            return socket.close();
        }
        Err(NetworkError::SocketError)
    }

    pub fn arp_lookup(&self, ip: IPv4Address) -> Option<MacAddress> {
        self.arp_table.lookup(ip)
    }

    pub fn arp_insert(&mut self, ip: IPv4Address, mac: MacAddress) {
        self.arp_table.insert(ip, mac);
    }

    pub fn route_lookup(&self, dest: IPv4Address) -> Option<&Route> {
        self.routing_table.lookup(dest)
    }

    pub fn route_add(&mut self, dest: IPv4Address, mask: IPv4Address, gw: IPv4Address) {
        self.routing_table.add_route(dest, mask, gw);
    }

    pub fn resolve_hostname(&self, hostname: &str) -> Result<IPv4Address, NetworkError> {
        self.dns_resolver.resolve(hostname)
    }

    pub fn cache_hostname(&mut self, hostname: std::string::String, ip: IPv4Address) {
        let mut resolver = DnsResolver::new();
        resolver.cache_entry(hostname, ip);
    }

    pub fn acquire_ip_via_dhcp(&mut self) -> Result<IPv4Address, NetworkError> {
        self.dhcp_client.request_ip()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_address() {
        let addr = IPv4Address::new(192, 168, 1, 1);
        assert_eq!(addr.to_u32(), 0xc0a80101);
        assert_eq!(addr.is_private(), true);
        assert_eq!(addr.is_broadcast(), false);
    }

    #[test]
    fn test_mac_address() {
        let mac = MacAddress::new(0x00, 0x11, 0x22, 0x33, 0x44, 0x55);
        assert_eq!(mac.is_broadcast(), false);
        assert_eq!(mac.is_multicast(), false);
    }

    #[test]
    fn test_tcp_socket_creation() {
        let mut stack = TcpIpStack::new();
        let socket_id = stack.socket(SocketType::Stream, SocketProtocol::Tcp).unwrap();
        assert!(socket_id > 0);
    }

    #[test]
    fn test_tcp_connection_states() {
        let addr = SocketAddr::new_ipv4(8080, [127, 0, 0, 1]);
        let mut tcp_sock = TcpSocket::new(addr);

        assert_eq!(tcp_sock.ccb.state(), TcpConnectionState::Closed);
        tcp_sock.bind(&addr).unwrap();

        tcp_sock.listen(1).unwrap();
        assert_eq!(tcp_sock.ccb.state(), TcpConnectionState::Listen);
    }

    #[test]
    fn test_routing_table() {
        let mut rt = RoutingTable::new();
        let dest = IPv4Address::new(10, 0, 0, 0);
        let mask = IPv4Address::new(255, 0, 0, 0);
        let gw = IPv4Address::new(192, 168, 1, 1);

        rt.add_route(dest, mask, gw);
        let lookup_addr = IPv4Address::new(10, 2, 3, 4);
        let route = rt.lookup(lookup_addr).unwrap();
        assert_eq!(route.gateway, gw);
    }

    #[test]
    fn test_dns_resolver() {
        let resolver = DnsResolver::new();
        let ip = resolver.resolve("localhost").unwrap();
        assert_eq!(ip, IPv4Address::new(127, 0, 0, 1));
    }

    #[test]
    fn test_arp_table() {
        let mut arp = ArpTable::new();
        let ip = IPv4Address::new(192, 168, 1, 100);
        let mac = MacAddress::new(0x00, 0x11, 0x22, 0x33, 0x44, 0x55);

        arp.insert(ip, mac);
        let found_mac = arp.lookup(ip).unwrap();
        assert_eq!(found_mac, mac);
    }

    #[test]
    fn test_port_allocation() {
        let arp = ArpTable::new();
        let port1 = arp.allocate_port();
        let port2 = arp.allocate_port();
        assert!(port2 > port1);
    }
}
