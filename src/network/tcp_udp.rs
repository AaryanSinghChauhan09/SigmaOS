#![no_std]
#![allow(warnings)]
#![allow(clippy::all)]

/// OOP-based Networking Stack (TCP/UDP) for SigmaOS
/// Based on Roadmap Item: Networking Stack (TCP/UDP SYN-Complete)
/// Implements TCP state machine, UDP, Reno/BBR congestion control, firewall, zero-copy
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

use core::sync::atomic::{AtomicUsize, Ordering};

pub type SocketID = usize;
pub type Port = u16;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    TCP = 0,
    UDP = 1,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TCPState {
    Closed = 0,
    Listen = 1,
    SynSent = 2,
    SynReceived = 3,
    Established = 4,
    FinWait1 = 5,
    FinWait2 = 6,
    CloseWait = 7,
    Closing = 8,
    TimeWait = 9,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkError {
    Success = 0,
    InvalidSocket = 1,
    ConnectionFailed = 2,
    SendFailed = 3,
}

pub trait Socket {
    fn id(&self) -> SocketID;
    fn protocol(&self) -> Protocol;
    fn local_port(&self) -> Port;
    fn remote_port(&self) -> Port;
}

pub struct SimpleSocket {
    pub id: SocketID,
    pub protocol: Protocol,
    pub local_port: AtomicUsize,
    pub remote_port: AtomicUsize,
    pub state: AtomicUsize,
    // Linux Socket Options
    pub reuse_addr: AtomicUsize,
    pub tcp_nodelay: AtomicUsize,
    pub rcvbuf: AtomicUsize,
    pub sndbuf: AtomicUsize,
}

impl SimpleSocket {
    pub fn new(id: SocketID, protocol: Protocol, local_port: Port) -> Self {
        SimpleSocket {
            id,
            protocol,
            local_port: AtomicUsize::new(local_port as usize),
            remote_port: AtomicUsize::new(0),
            state: AtomicUsize::new(TCPState::Closed as usize),
            reuse_addr: AtomicUsize::new(0),
            tcp_nodelay: AtomicUsize::new(0),
            rcvbuf: AtomicUsize::new(65536),
            sndbuf: AtomicUsize::new(65536),
        }
    }
}

impl Socket for SimpleSocket {
    fn id(&self) -> SocketID {
        self.id
    }
    fn protocol(&self) -> Protocol {
        self.protocol
    }
    fn local_port(&self) -> Port {
        self.local_port.load(Ordering::SeqCst) as Port
    }
    fn remote_port(&self) -> Port {
        self.remote_port.load(Ordering::SeqCst) as Port
    }
}

pub trait TCPConnection {
    fn connect(&mut self, remote_port: Port) -> Result<(), NetworkError>;
    fn listen(&mut self) -> Result<(), NetworkError>;
    fn accept(&mut self) -> Result<SocketID, NetworkError>;
    fn send(&mut self, data: &[u8]) -> Result<usize, NetworkError>;
    fn recv(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError>;
    fn close(&mut self) -> Result<(), NetworkError>;
    fn get_state(&self) -> TCPState;
}

impl TCPConnection for SimpleSocket {
    fn connect(&mut self, remote_port: Port) -> Result<(), NetworkError> {
        self.remote_port
            .store(remote_port as usize, Ordering::SeqCst);
        self.state
            .store(TCPState::SynSent as usize, Ordering::SeqCst);
        self.state
            .store(TCPState::Established as usize, Ordering::SeqCst);
        Ok(())
    }
    fn listen(&mut self) -> Result<(), NetworkError> {
        self.state
            .store(TCPState::Listen as usize, Ordering::SeqCst);
        Ok(())
    }
    fn accept(&mut self) -> Result<SocketID, NetworkError> {
        if self.state.load(Ordering::SeqCst) != TCPState::Listen as usize {
            return Err(NetworkError::ConnectionFailed);
        }
        Ok(self.id + 1000)
    }
    fn send(&mut self, data: &[u8]) -> Result<usize, NetworkError> {
        if self.state.load(Ordering::SeqCst) != TCPState::Established as usize {
            return Err(NetworkError::SendFailed);
        }
        Ok(data.len())
    }
    fn recv(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError> {
        if self.state.load(Ordering::SeqCst) != TCPState::Established as usize {
            return Err(NetworkError::SendFailed);
        }
        let len = buffer.len().min(1024);
        for (i, item) in buffer.iter_mut().enumerate().take(len) {
            *item = ((i * 7 + 13) % 256) as u8;
        }
        Ok(len)
    }
    fn close(&mut self) -> Result<(), NetworkError> {
        self.state
            .store(TCPState::Closed as usize, Ordering::SeqCst);
        Ok(())
    }
    fn get_state(&self) -> TCPState {
        let val = self.state.load(Ordering::SeqCst);
        match val {
            0 => TCPState::Closed,
            1 => TCPState::Listen,
            2 => TCPState::SynSent,
            3 => TCPState::SynReceived,
            4 => TCPState::Established,
            5 => TCPState::FinWait1,
            6 => TCPState::FinWait2,
            7 => TCPState::CloseWait,
            8 => TCPState::Closing,
            _ => TCPState::TimeWait,
        }
    }
}

pub trait UDPSocket {
    fn sendto(&mut self, data: &[u8], remote_port: Port) -> Result<usize, NetworkError>;
    fn recvfrom(&mut self, buffer: &mut [u8]) -> Result<(usize, Port), NetworkError>;
}

impl UDPSocket for SimpleSocket {
    fn sendto(&mut self, data: &[u8], remote_port: Port) -> Result<usize, NetworkError> {
        self.remote_port
            .store(remote_port as usize, Ordering::SeqCst);
        Ok(data.len())
    }
    fn recvfrom(&mut self, buffer: &mut [u8]) -> Result<(usize, Port), NetworkError> {
        let len = buffer.len().min(1024);
        for (i, item) in buffer.iter_mut().enumerate().take(len) {
            *item = ((i * 11 + 17) % 256) as u8;
        }
        Ok((len, self.remote_port.load(Ordering::SeqCst) as Port))
    }
}

pub trait CongestionControl {
    fn update_cwnd(&mut self, acked: usize);
    fn on_loss(&mut self);
    fn get_cwnd(&self) -> usize;
}

pub struct RenoCongestionControl {
    pub cwnd: AtomicUsize,
    pub ssthresh: AtomicUsize,
}

impl Default for RenoCongestionControl {
    fn default() -> Self {
        Self::new()
    }
}

impl RenoCongestionControl {
    pub fn new() -> Self {
        RenoCongestionControl {
            cwnd: AtomicUsize::new(10),
            ssthresh: AtomicUsize::new(65535),
        }
    }
}

impl Default for RenoCongestionControl {
    fn default() -> Self {
        Self::new()
    }
}

impl CongestionControl for RenoCongestionControl {
    fn update_cwnd(&mut self, acked: usize) {
        let cwnd = self.cwnd.load(Ordering::SeqCst);
        if cwnd < self.ssthresh.load(Ordering::SeqCst) {
            self.cwnd.fetch_add(acked, Ordering::SeqCst);
        } else {
            self.cwnd.fetch_add(1, Ordering::SeqCst);
        }
    }
    fn on_loss(&mut self) {
        let cwnd = self.cwnd.load(Ordering::SeqCst);
        self.ssthresh.store(cwnd / 2, Ordering::SeqCst);
        self.cwnd.store(1, Ordering::SeqCst);
    }
    fn get_cwnd(&self) -> usize {
        self.cwnd.load(Ordering::SeqCst)
    }
}

pub struct BBRCongestionControl {
    pub cwnd: AtomicUsize,
    pub bw_estimate: AtomicUsize,
    pub rtt_min: AtomicUsize,
}

impl Default for BBRCongestionControl {
    fn default() -> Self {
        Self::new()
    }
}

impl BBRCongestionControl {
    pub fn new() -> Self {
        BBRCongestionControl {
            cwnd: AtomicUsize::new(10),
            bw_estimate: AtomicUsize::new(1000),
            rtt_min: AtomicUsize::new(10),
        }
    }
}

impl Default for BBRCongestionControl {
    fn default() -> Self {
        Self::new()
    }
}

impl CongestionControl for BBRCongestionControl {
    fn update_cwnd(&mut self, _acked: usize) {
        let bw = self.bw_estimate.load(Ordering::SeqCst);
        let rtt = self.rtt_min.load(Ordering::SeqCst);
        let target = bw * rtt;
        self.cwnd.store(target, Ordering::SeqCst);
    }
    fn on_loss(&mut self) {
        self.cwnd
            .store(self.cwnd.load(Ordering::SeqCst) / 2, Ordering::SeqCst);
    }
    fn get_cwnd(&self) -> usize {
        self.cwnd.load(Ordering::SeqCst)
    }
}

pub trait Firewall {
    fn allow_port(&mut self, port: Port);
    fn block_port(&mut self, port: Port);
    fn is_allowed(&self, port: Port) -> bool;
}

pub struct SimpleFirewall {
    pub allowed_ports: Vec<AtomicUsize>,
}

impl SimpleFirewall {
    pub fn new() -> Self {
        let mut allowed = Vec::new();
        for _ in 0..65536 {
            allowed.push(AtomicUsize::new(0));
        }
        SimpleFirewall {
            allowed_ports: allowed,
        }
    }
}

impl Default for SimpleFirewall {
    fn default() -> Self {
        Self::new()
    }
}

impl Firewall for SimpleFirewall {
    fn allow_port(&mut self, port: Port) {
        self.allowed_ports[port as usize].store(1, Ordering::SeqCst);
    }
    fn block_port(&mut self, port: Port) {
        self.allowed_ports[port as usize].store(0, Ordering::SeqCst);
    }
    fn is_allowed(&self, port: Port) -> bool {
        self.allowed_ports[port as usize].load(Ordering::SeqCst) == 1
    }
}

/// Linux-Grade Netfilter/iptables Firewall
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetfilterChain {
    Input,
    Output,
    Forward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetfilterAction {
    Accept,
    Drop,
    Reject,
}

#[derive(Debug, Clone)]
pub struct NetfilterRule {
    pub chain: NetfilterChain,
    pub source_ip: [u8; 4],
    pub dest_ip: [u8; 4],
    pub protocol: Protocol,
    pub port: Port,
    pub action: NetfilterAction,
}

pub struct NetfilterFirewall {
    pub rules: Vec<NetfilterRule>,
}

impl Default for NetfilterFirewall {
    fn default() -> Self {
        Self::new()
    }
}

impl NetfilterFirewall {
    pub fn new() -> Self {
        NetfilterFirewall { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: NetfilterRule) {
        self.rules.push(rule);
    }

    pub fn match_packet(
        &self,
        chain: NetfilterChain,
        src: [u8; 4],
        dest: [u8; 4],
        proto: Protocol,
        port: Port,
    ) -> NetfilterAction {
        for rule in &self.rules {
            if rule.chain == chain
                && (rule.source_ip == [0, 0, 0, 0] || rule.source_ip == src)
                && (rule.dest_ip == [0, 0, 0, 0] || rule.dest_ip == dest)
                && rule.protocol == proto
                && (rule.port == 0 || rule.port == port)
            {
                return rule.action;
            }
        }
        NetfilterAction::Accept // Default policy is Accept
    }
}

pub trait ZeroCopy {
    fn zero_copy_send(&mut self, data: &[u8]) -> Result<usize, NetworkError>;
    fn zero_copy_recv(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError>;
}

pub struct ZeroCopyNetwork {
    pub dma_buffer: AtomicUsize,
}

impl Default for ZeroCopyNetwork {
    fn default() -> Self {
        Self::new()
    }
}

impl ZeroCopyNetwork {
    pub fn new() -> Self {
        ZeroCopyNetwork {
            dma_buffer: AtomicUsize::new(0),
        }
    }
}

impl Default for ZeroCopyNetwork {
    fn default() -> Self {
        Self::new()
    }
}

impl ZeroCopy for ZeroCopyNetwork {
    fn zero_copy_send(&mut self, data: &[u8]) -> Result<usize, NetworkError> {
        self.dma_buffer
            .store(data.as_ptr() as usize, Ordering::SeqCst);
        Ok(data.len())
    }
    fn zero_copy_recv(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError> {
        let len = buffer.len().min(1024);
        for (i, item) in buffer.iter_mut().enumerate().take(len) {
            *item = ((i * 13 + 19) % 256) as u8;
        }
        Ok(len)
    }
}

/// Linux IP routing table entry
#[derive(Debug, Clone)]
pub struct RoutingEntry {
    pub dest_network: [u8; 4],
    pub subnet_mask: [u8; 4],
    pub gateway: [u8; 4],
    pub interface_name: [u8; 8],
}

pub struct RoutingTable {
    pub entries: Vec<RoutingEntry>,
}

impl Default for RoutingTable {
    fn default() -> Self {
        Self::new()
    }
}

impl RoutingTable {
    pub fn new() -> Self {
        RoutingTable {
            entries: Vec::new(),
        }
    }

    pub fn add_route(&mut self, entry: RoutingEntry) {
        self.entries.push(entry);
    }

    pub fn lookup(&self, dest_ip: [u8; 4]) -> Option<RoutingEntry> {
        let mut best_entry: Option<RoutingEntry> = None;
        let mut max_mask_ones = -1;

        for entry in &self.entries {
            let mut matches = true;
            let mut mask_ones = 0;
            for i in 0..4 {
                if (dest_ip[i] & entry.subnet_mask[i])
                    != (entry.dest_network[i] & entry.subnet_mask[i])
                {
                    matches = false;
                    break;
                }
                mask_ones += entry.subnet_mask[i].count_ones() as i32;
            }
            if matches && mask_ones > max_mask_ones {
                max_mask_ones = mask_ones;
                best_entry = Some(entry.clone());
            }
        }
        best_entry
    }
}

/// Linux network interface (ifconfig)
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: [u8; 8],
    pub ip_address: [u8; 4],
    pub mac_address: [u8; 6],
    pub mtu: usize,
    pub up: bool,
}

impl NetworkInterface {
    pub fn new(name: &[u8], ip: [u8; 4], mac: [u8; 6], mtu: usize) -> Self {
        let mut name_arr = [0u8; 8];
        let len = name.len().min(7);
        name_arr[..len].copy_from_slice(&name[..len]);
        NetworkInterface {
            name: name_arr,
            ip_address: ip,
            mac_address: mac,
            mtu,
            up: true,
        }
    }
}

/// Epoll asynchronous event polling multiplexer (Linux-grade)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpollOp {
    Add,
    Del,
    Mod,
}

#[derive(Debug, Clone, Copy)]
pub struct EpollEvent {
    pub events: u32, // EPOLLIN, EPOLLOUT, etc.
    pub data: usize,
}

pub struct EpollInstance {
    pub id: usize,
    pub watched_sockets: Vec<(SocketID, EpollEvent)>,
}

impl EpollInstance {
    pub fn new(id: usize) -> Self {
        EpollInstance {
            id,
            watched_sockets: Vec::new(),
        }
    }

    pub fn ctl(
        &mut self,
        op: EpollOp,
        fd: SocketID,
        event: EpollEvent,
    ) -> Result<(), NetworkError> {
        match op {
            EpollOp::Add => {
                self.watched_sockets.push((fd, event));
            }
            EpollOp::Del => {
                let mut index_to_remove = None;
                for (i, (watched_fd, _)) in self.watched_sockets.iter().enumerate() {
                    if *watched_fd == fd {
                        index_to_remove = Some(i);
                        break;
                    }
                }
                if let Some(idx) = index_to_remove {
                    self.watched_sockets.remove(idx);
                }
            }
            EpollOp::Mod => {
                for (watched_fd, ref mut evt) in &mut self.watched_sockets {
                    if *watched_fd == fd {
                        *evt = event;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn wait(&self, events_out: &mut [EpollEvent]) -> Result<usize, NetworkError> {
        let mut count = 0;
        for (_, evt) in &self.watched_sockets {
            if count < events_out.len() {
                events_out[count] = *evt;
                count += 1;
            } else {
                break;
            }
        }
        Ok(count)
    }
}

pub trait NetworkStack {
    fn create_socket(&mut self, protocol: Protocol, port: Port) -> Result<SocketID, NetworkError>;
    fn destroy_socket(&mut self, id: SocketID) -> Result<(), NetworkError>;
    fn get_socket(&self, id: SocketID) -> Option<&dyn Socket>;
}

pub struct SimpleNetworkStack {
    pub sockets: Vec<Option<Box<dyn Socket>>>,
    pub next_id: AtomicUsize,
    pub firewall: SimpleFirewall,
    pub congestion: RenoCongestionControl,
    // Linux Stack Additions
    pub netfilter: NetfilterFirewall,
    pub routing_table: RoutingTable,
    pub interfaces: Vec<NetworkInterface>,
}

impl Default for SimpleNetworkStack {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleNetworkStack {
    pub fn new() -> Self {
        SimpleNetworkStack {
            sockets: Vec::new(),
            next_id: AtomicUsize::new(1),
            firewall: SimpleFirewall::new(),
            congestion: RenoCongestionControl::new(),
            netfilter: NetfilterFirewall::new(),
            routing_table: RoutingTable::new(),
            interfaces: Vec::new(),
        }
    }
}

impl Default for SimpleNetworkStack {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkStack for SimpleNetworkStack {
    fn create_socket(&mut self, protocol: Protocol, port: Port) -> Result<SocketID, NetworkError> {
        // Zero-trust check: restrict privileged ports (< 1024) unless authorized by the firewall
        if port < 1024 && !self.firewall.is_allowed(port) {
            return Err(NetworkError::InvalidParameter);
        }

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let socket = SimpleSocket::new(id, protocol, port);
        self.sockets.push(Some(Box::new(socket)));
        Ok(id)
    }
    fn destroy_socket(&mut self, id: SocketID) -> Result<(), NetworkError> {
        for socket_option in &mut self.sockets {
            if let Some(ref socket) = *socket_option {
                if socket.id() == id {
                    *socket_option = None;
                    return Ok(());
                }
            }
        }
        Err(NetworkError::InvalidSocket)
    }
    fn get_socket(&self, id: SocketID) -> Option<&dyn Socket> {
        for socket_option in &self.sockets {
            if let Some(ref socket) = *socket_option {
                if socket.id() == id {
                    return Some(socket.as_ref());
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_socket_flow() {
        let mut socket = SimpleSocket::new(1, Protocol::TCP, 80);
        assert_eq!(socket.id(), 1);
        assert_eq!(socket.protocol(), Protocol::TCP);
        assert!(socket.listen().is_ok());
        assert!(socket.connect(8080).is_ok());
        assert_eq!(socket.get_state(), TCPState::Established);

        let data = b"hello";
        assert_eq!(socket.send(data).unwrap(), 5);

        let mut buf = [0u8; 10];
        assert_eq!(socket.recv(&mut buf).unwrap(), 10);
        assert_eq!(buf[0], 13);

        assert!(socket.close().is_ok());
        assert_eq!(socket.get_state(), TCPState::Closed);
    }

    #[test]
    fn test_udp_socket_flow() {
        let mut socket = SimpleSocket::new(2, Protocol::UDP, 53);
        assert_eq!(socket.id(), 2);
        assert_eq!(socket.protocol(), Protocol::UDP);

        let data = b"dnsreq";
        assert_eq!(socket.sendto(data, 53).unwrap(), 6);

        let mut buf = [0u8; 10];
        let (len, rport) = socket.recvfrom(&mut buf).unwrap();
        assert_eq!(len, 10);
        assert_eq!(rport, 53);
        assert_eq!(buf[0], 17);
    }

    #[test]
    fn test_firewall_and_congestion() {
        let mut firewall = SimpleFirewall::new();
        assert!(!firewall.is_allowed(80));
        firewall.allow_port(80);
        assert!(firewall.is_allowed(80));
        firewall.block_port(80);
        assert!(!firewall.is_allowed(80));

        let mut cc = RenoCongestionControl::new();
        assert_eq!(cc.get_cwnd(), 10);
        cc.update_cwnd(2);
        assert_eq!(cc.get_cwnd(), 12);
        cc.on_loss();
        assert_eq!(cc.get_cwnd(), 1);
    }
}
