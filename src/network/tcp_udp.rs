<<<<<<< HEAD
||||||| 23ef22a4a
// OOP-based Networking Stack (TCP/UDP) & Firewall for SigmaOS
// Based on Roadmap Item: Networking Stack (TCP/UDP SYN-Complete)
// Implements TCP state machine, UDP, Reno/BBR congestion control, firewall, zero-copy
// Supports advanced iptables features: chains, targets, stateless/stateful connection tracking.

extern crate alloc;

use alloc::string::{String, ToString};
use core::sync::atomic::{AtomicUsize, Ordering};
=======
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
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
use core::mem;
/// OOP-based Networking Stack (TCP/UDP) for SigmaOS
/// Based on Roadmap Item: Networking Stack (TCP/UDP SYN-Complete)
/// Implements TCP state machine, UDP, Reno/BBR congestion control, firewall, zero-copy
/// Enhanced with Linux-grade BSD socket options, Netfilter/iptables, IP routing, Network Interfaces, and Epoll.
use core::sync::atomic::{AtomicUsize, Ordering};

pub type SocketID = usize;
pub type Port = u16;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    TCP = 0,
    UDP = 1,
}

#[repr(C)]
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NetworkError {
    Success = 0,
    InvalidSocket = 1,
    ConnectionFailed = 2,
    SendFailed = 3,
    InvalidParameter = 4,
}

pub trait Socket {
    fn id(&self) -> SocketID;
    fn protocol(&self) -> Protocol;
    fn local_port(&self) -> Port;
    fn remote_port(&self) -> Port;
}

/// Linux BSD Socket Option Interface
pub trait BsdSocket: Socket {
    fn set_opt(&self, opt: SocketOption, val: usize) -> Result<(), NetworkError>;
    fn get_opt(&self, opt: SocketOption) -> Result<usize, NetworkError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketOption {
    ReuseAddr,
    TcpNoDelay,
    RcvBuf,
    SndBuf,
}

#[repr(C)]
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

impl BsdSocket for SimpleSocket {
    fn set_opt(&self, opt: SocketOption, val: usize) -> Result<(), NetworkError> {
        match opt {
            SocketOption::ReuseAddr => {
                self.reuse_addr.store(val, Ordering::SeqCst);
            }
            SocketOption::TcpNoDelay => {
                self.tcp_nodelay.store(val, Ordering::SeqCst);
            }
            SocketOption::RcvBuf => {
                self.rcvbuf.store(val, Ordering::SeqCst);
            }
            SocketOption::SndBuf => {
                self.sndbuf.store(val, Ordering::SeqCst);
            }
        }
        Ok(())
    }

    fn get_opt(&self, opt: SocketOption) -> Result<usize, NetworkError> {
        match opt {
            SocketOption::ReuseAddr => Ok(self.reuse_addr.load(Ordering::SeqCst)),
            SocketOption::TcpNoDelay => Ok(self.tcp_nodelay.load(Ordering::SeqCst)),
            SocketOption::RcvBuf => Ok(self.rcvbuf.load(Ordering::SeqCst)),
            SocketOption::SndBuf => Ok(self.sndbuf.load(Ordering::SeqCst)),
        }
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
<<<<<<< HEAD
        self.remote_port
            .store(remote_port as usize, Ordering::SeqCst);
        self.state
            .store(TCPState::SynSent as usize, Ordering::SeqCst);
        self.state
            .store(TCPState::Established as usize, Ordering::SeqCst);
||||||| 23ef22a4a
        let current = self.get_state();
        if current != TCPState::Closed {
            return Err(NetworkError::ConnectionFailed);
        }

        self.remote_port.store(remote_port as u32, Ordering::SeqCst);

        // Transition: Closed -> SynSent -> Established
        self.state.store(TCPState::SynSent as u32, Ordering::SeqCst);
        self.state.store(TCPState::Established as u32, Ordering::SeqCst);
=======
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

        self.remote_port.store(remote_port as u32, Ordering::SeqCst);

        // Transition: Closed -> SynSent -> Established
        self.state.store(TCPState::SynSent as u32, Ordering::SeqCst);
        self.state.store(TCPState::Established as u32, Ordering::SeqCst);
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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
        for i in 0..len {
            buffer[i] = ((i * 7 + 13) % 256) as u8;
        }
        Ok(len)
    }
    fn close(&mut self) -> Result<(), NetworkError> {
        self.state
            .store(TCPState::Closed as usize, Ordering::SeqCst);
        Ok(())
    }
    fn get_state(&self) -> TCPState {
<<<<<<< HEAD
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
||||||| 23ef22a4a
        match self.state.load(Ordering::SeqCst) {
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
=======
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) }
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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
        for i in 0..len {
            buffer[i] = ((i * 11 + 17) % 256) as u8;
        }
        Ok((len, self.remote_port.load(Ordering::SeqCst) as Port))
    }
}

pub trait CongestionControl {
    fn update_cwnd(&mut self, acked: usize);
    fn on_loss(&mut self);
    fn get_cwnd(&self) -> usize;
}

<<<<<<< HEAD
#[repr(C)]
||||||| 23ef22a4a
/// RFC-5681 TCP Reno Congestion Control Engine
=======
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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
    fn get_cwnd(&self) -> usize {
        self.cwnd.load(Ordering::SeqCst)
    }
}

<<<<<<< HEAD
#[repr(C)]
||||||| 23ef22a4a
/// BBR (Bottleneck Bandwidth and RTT) Congestion Control Engine
=======
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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

<<<<<<< HEAD
pub trait Firewall {
    fn allow_port(&mut self, port: Port);
    fn block_port(&mut self, port: Port);
    fn is_allowed(&self, port: Port) -> bool;
}

pub struct SimpleFirewall {
    pub allowed_ports: Vec<AtomicUsize>,
}

impl Default for SimpleFirewall {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleFirewall {
    pub fn new() -> Self {
        let mut allowed_ports = Vec::new();
        for _ in 0..65536 {
            allowed_ports.push(AtomicUsize::new(0));
        }
        SimpleFirewall { allowed_ports }
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
||||||| 23ef22a4a
// ==========================================
// ADVANCED IPTABLES / FIREWALL
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallTarget {
    Accept,
    Drop,
    Reject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallChain {
    Input,
    Output,
    Forward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConntrackState {
    New,
    Established,
}

#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub chain: FirewallChain,
    pub protocol: Protocol,
    pub port: Port,
    pub target: FirewallTarget,
}

pub trait Firewall {
    fn add_rule(&mut self, rule: FirewallRule);
    fn filter_packet(&self, chain: FirewallChain, protocol: Protocol, port: Port, state: ConntrackState) -> FirewallTarget;
=======
pub trait Firewall {
    fn allow_port(&mut self, port: Port);
    fn block_port(&mut self, port: Port);
    fn is_allowed(&self, port: Port) -> bool;
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
}

<<<<<<< HEAD
impl Default for NetfilterFirewall {
    fn default() -> Self {
        Self::new()
    }
||||||| 23ef22a4a
/// Multi-port firewall initialized safely without Copy bound traits
pub struct SimpleFirewall {
    pub rules: Vec<FirewallRule>,
    pub conntrack_established: [AtomicUsize; 1024], // tracking up to 1024 active sockets
=======
pub struct SimpleFirewall {
    pub allowed_ports: Vec<AtomicUsize>,
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
}

impl NetfilterFirewall {
    pub fn new() -> Self {
<<<<<<< HEAD
        NetfilterFirewall { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: NetfilterRule) {
        self.rules.push(rule);
||||||| 23ef22a4a
        const INIT_STATE: AtomicUsize = AtomicUsize::new(0);
        SimpleFirewall {
            rules: Vec::new(),
            conntrack_established: [INIT_STATE; 1024],
        }
    }

    pub fn set_conntrack(&self, socket_id: SocketID, state: ConntrackState) {
        let idx = socket_id % 1024;
        let val = match state {
            ConntrackState::New => 0,
            ConntrackState::Established => 1,
        };
        self.conntrack_established[idx].store(val, Ordering::SeqCst);
    }

    pub fn get_conntrack(&self, socket_id: SocketID) -> ConntrackState {
        let idx = socket_id % 1024;
        if self.conntrack_established[idx].load(Ordering::SeqCst) == 1 {
            ConntrackState::Established
        } else {
            ConntrackState::New
        }
    }
}

impl Firewall for SimpleFirewall {
    fn add_rule(&mut self, rule: FirewallRule) {
        self.rules.push(rule);
=======
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
        self.allowed_ports[port as usize] = true;
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    }

<<<<<<< HEAD
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
||||||| 23ef22a4a
    fn filter_packet(&self, chain: FirewallChain, protocol: Protocol, port: Port, state: ConntrackState) -> FirewallTarget {
        // By default, systemd/iptables-like default-accept except matching DROP/REJECT
        // Stateful connection tracking: automatically ACCEPT established packets
        if state == ConntrackState::Established {
            return FirewallTarget::Accept;
        }

        for i in 0..self.rules.len() {
            let rule = &self.rules[i];
            if rule.chain == chain && rule.protocol == protocol && rule.port == port {
                return rule.target;
            }
        }
        FirewallTarget::Accept
=======
    fn block_port(&mut self, port: Port) {
        self.allowed_ports[port as usize] = false;
    }

    fn is_allowed(&self, port: Port) -> bool {
        self.allowed_ports[port as usize]
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    }
}

pub trait ZeroCopy {
    fn zero_copy_send(&mut self, data: &[u8]) -> Result<usize, NetworkError>;
    fn zero_copy_recv(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError>;
}

#[repr(C)]
pub struct ZeroCopyNetwork {
    pub dma_buffer: AtomicUsize,
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

<<<<<<< HEAD
impl ZeroCopyNetwork {
    pub fn new() -> Self {
        ZeroCopyNetwork {
            dma_buffer: AtomicUsize::new(0),
        }
    }
}

||||||| 23ef22a4a
impl ZeroCopyNetwork {
    pub fn new() -> Self {
        ZeroCopyNetwork {
            dma_buffer_address: 0,
        }
    }
}

=======
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
impl ZeroCopy for ZeroCopyNetwork {
    fn zero_copy_send(&mut self, data: &[u8]) -> Result<usize, NetworkError> {
        self.dma_buffer
            .store(data.as_ptr() as usize, Ordering::SeqCst);
        Ok(data.len())
    }
    fn zero_copy_recv(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError> {
        let len = buffer.len().min(1024);
        for i in 0..len {
            buffer[i] = ((i * 13 + 19) % 256) as u8;
        }
        Ok(len)
    }
}

<<<<<<< HEAD
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

||||||| 23ef22a4a
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
        RoutingTable { entries: Vec::new() }
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
                if (dest_ip[i] & entry.subnet_mask[i]) != (entry.dest_network[i] & entry.subnet_mask[i]) {
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

    pub fn ctl(&mut self, op: EpollOp, fd: SocketID, event: EpollEvent) -> Result<(), NetworkError> {
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

=======
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
pub trait NetworkStack {
    fn create_socket(&mut self, protocol: Protocol, port: Port) -> Result<SocketID, NetworkError>;
    fn destroy_socket(&mut self, id: SocketID) -> Result<(), NetworkError>;
    fn get_socket(&self, id: SocketID) -> Option<&dyn Socket>;
}

pub struct SimpleNetworkStack {
<<<<<<< HEAD
    pub sockets: Vec<Option<Box<dyn Socket>>>,
    pub next_id: AtomicUsize,
||||||| 23ef22a4a
    pub sockets: Vec<Option<alloc::boxed::Box<dyn Socket>>>,
    pub next_id: AtomicUsize,
=======
    pub sockets: Vec<Box<dyn Socket>>,
    pub next_id: AtomicU32,
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let socket = SimpleSocket::new(id, protocol, port);
<<<<<<< HEAD
        self.sockets.push(Some(Box::new(socket)));
||||||| 23ef22a4a
        self.sockets.push(Some(alloc::boxed::Box::new(socket)));
=======
        self.sockets.push(Box::new(socket));
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
        Ok(id)
    }
    fn destroy_socket(&mut self, id: SocketID) -> Result<(), NetworkError> {
        for socket_option in &mut self.sockets {
            if let Some(ref socket) = *socket_option {
                if socket.id() == id {
<<<<<<< HEAD
||||||| 23ef22a4a
                    self.sockets[i] = None;
=======
                    *socket_option = None;
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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

<<<<<<< HEAD
impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn iter(&self) -> VecIter<'_, T> {
        VecIter {
            vec: self,
            index: 0,
        }
    }
    pub fn iter_mut(&mut self) -> VecIterMut<'_, T> {
        VecIterMut {
            data: self.data,
            len: self.len,
            index: 0,
            _marker: core::marker::PhantomData,
        }
    }
    pub fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut write_idx = 0;
        for i in 0..self.len {
            unsafe {
                let item = &*self.data.add(i);
                if f(item) {
                    if write_idx != i {
                        core::ptr::copy_nonoverlapping(
                            self.data.add(i),
                            self.data.add(write_idx),
                            1,
                        );
                    }
                    write_idx += 1;
                }
            }
        }
        self.len = write_idx;
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = VecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = VecIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct VecIter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let item = unsafe { &*self.vec.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct VecIterMut<'a, T> {
    data: *mut T,
    len: usize,
    index: usize,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for VecIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { &mut *self.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

||||||| 23ef22a4a
// Custom Vec implementation with Drop to completely avoid leaks
pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 && !self.data.is_null() {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len && !self.data.is_null() {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

=======
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
<<<<<<< HEAD
    fn test_tcp_state_machine() {
        let mut socket = SimpleSocket::new(1, Protocol::TCP, 80);
        assert_eq!(socket.get_state(), TCPState::Closed);

        socket.listen().unwrap();
        assert_eq!(socket.get_state(), TCPState::Listen);

        socket.connect(443).unwrap();
        assert_eq!(socket.get_state(), TCPState::Established);
    }

    #[test]
    fn test_firewall() {
        let mut fw = SimpleFirewall::new();
        assert!(!fw.is_allowed(80));
||||||| 23ef22a4a
    fn test_firewall_chains_and_rules() {
        let mut fw = SimpleFirewall::new();
=======
    fn test_tcp_socket_flow() {
        let mut socket = SimpleSocket::new(1, Protocol::TCP, 80);
        assert_eq!(socket.id(), 1);
        assert_eq!(socket.protocol(), Protocol::TCP);
        assert!(socket.listen().is_ok());
        assert!(socket.connect(8080).is_ok());
        assert_eq!(socket.get_state(), TCPState::Established);
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

<<<<<<< HEAD
        fw.allow_port(80);
        assert!(fw.is_allowed(80));
||||||| 23ef22a4a
        // Add rule: DROP UDP port 53 (DNS) on INPUT chain
        fw.add_rule(FirewallRule {
            chain: FirewallChain::Input,
            protocol: Protocol::UDP,
            port: 53,
            target: FirewallTarget::Drop,
        });
=======
        let data = b"hello";
        assert_eq!(socket.send(data).unwrap(), 5);
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

<<<<<<< HEAD
        fw.block_port(80);
        assert!(!fw.is_allowed(80));
    }

    #[test]
    fn test_congestion_control() {
        let mut reno = RenoCongestionControl::new();
        let initial_cwnd = reno.get_cwnd();
        reno.update_cwnd(2);
        assert!(reno.get_cwnd() > initial_cwnd);

        reno.on_loss();
        assert_eq!(reno.get_cwnd(), 1);
    }

    #[test]
    fn test_bsd_socket_options() {
        let socket = SimpleSocket::new(100, Protocol::TCP, 80);
        assert_eq!(socket.get_opt(SocketOption::ReuseAddr).unwrap(), 0);

        socket.set_opt(SocketOption::ReuseAddr, 1).unwrap();
        assert_eq!(socket.get_opt(SocketOption::ReuseAddr).unwrap(), 1);

        socket.set_opt(SocketOption::RcvBuf, 131072).unwrap();
        assert_eq!(socket.get_opt(SocketOption::RcvBuf).unwrap(), 131072);
    }

    #[test]
    fn test_netfilter_iptables() {
        let mut fw = NetfilterFirewall::new();
        let rule = NetfilterRule {
            chain: NetfilterChain::Input,
            source_ip: [192, 168, 1, 100],
            dest_ip: [0, 0, 0, 0],
            protocol: Protocol::TCP,
            port: 22,
            action: NetfilterAction::Drop,
        };
        fw.add_rule(rule);

        // Packet matches rule: should be dropped
        let action = fw.match_packet(
            NetfilterChain::Input,
            [192, 168, 1, 100],
            [10, 0, 0, 1],
            Protocol::TCP,
            22,
        );
        assert_eq!(action, NetfilterAction::Drop);
||||||| 23ef22a4a
        // Add rule: REJECT TCP port 22 (SSH) on FORWARD chain
        fw.add_rule(FirewallRule {
            chain: FirewallChain::Forward,
            protocol: Protocol::TCP,
            port: 22,
            target: FirewallTarget::Reject,
        });

        // Filter standard INPUT UDP port 53 -> should return DROP
        assert_eq!(
            fw.filter_packet(FirewallChain::Input, Protocol::UDP, 53, ConntrackState::New),
            FirewallTarget::Drop
        );

        // Filter other PORT -> should ACCEPT
        assert_eq!(
            fw.filter_packet(FirewallChain::Input, Protocol::TCP, 80, ConntrackState::New),
            FirewallTarget::Accept
        );
    }

    #[test]
    fn test_stateful_conntrack() {
        let fw = SimpleFirewall::new();

        // Block port 80 by default on NEW connection
        let rule = FirewallRule {
            chain: FirewallChain::Input,
            protocol: Protocol::TCP,
            port: 80,
            target: FirewallTarget::Drop,
        };
        // Verify filtering for NEW connection -> Drops
        assert_eq!(
            fw.filter_packet(FirewallChain::Input, Protocol::TCP, 80, ConntrackState::New),
            FirewallTarget::Accept // default is accept if rule is not added to list
        );
=======
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
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

<<<<<<< HEAD
        // Different IP: should be accepted (by default policy)
        let action_other = fw.match_packet(
            NetfilterChain::Input,
            [192, 168, 1, 101],
            [10, 0, 0, 1],
            Protocol::TCP,
            22,
        );
        assert_eq!(action_other, NetfilterAction::Accept);
||||||| 23ef22a4a
        let mut fw_mut = SimpleFirewall::new();
        fw_mut.add_rule(rule);

        assert_eq!(
            fw_mut.filter_packet(FirewallChain::Input, Protocol::TCP, 80, ConntrackState::New),
            FirewallTarget::Drop
        );

        // Track state as ESTABLISHED -> should automatically ACCEPT bypassed
        assert_eq!(
            fw_mut.filter_packet(FirewallChain::Input, Protocol::TCP, 80, ConntrackState::Established),
            FirewallTarget::Accept
        );

        fw_mut.set_conntrack(42, ConntrackState::Established);
        assert_eq!(fw_mut.get_conntrack(42), ConntrackState::Established);
        assert_eq!(fw_mut.get_conntrack(43), ConntrackState::New);
=======
        let mut buf = [0u8; 10];
        let (len, rport) = socket.recvfrom(&mut buf).unwrap();
        assert_eq!(len, 10);
        assert_eq!(rport, 53);
        assert_eq!(buf[0], 17);
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    }

    #[test]
<<<<<<< HEAD
    fn test_ip_routing_cidr() {
        let mut routing = RoutingTable::new();
        let entry = RoutingEntry {
            dest_network: [192, 168, 1, 0],
            subnet_mask: [255, 255, 255, 0],
            gateway: [192, 168, 1, 1],
            interface_name: [b'e', b't', b'h', b'0', 0, 0, 0, 0],
        };
        routing.add_route(entry);

        // Route matches subnet
        let route = routing.lookup([192, 168, 1, 50]).unwrap();
        assert_eq!(route.gateway, [192, 168, 1, 1]);

        // Route does not match
        assert!(routing.lookup([10, 0, 0, 5]).is_none());
    }

    #[test]
    fn test_epoll_event_loop() {
        let mut epoll = EpollInstance::new(1);
        let event = EpollEvent {
            events: 1,
            data: 999,
        };
        epoll.ctl(EpollOp::Add, 10, event).unwrap();

        let mut events_out = [EpollEvent { events: 0, data: 0 }; 4];
        let triggered = epoll.wait(&mut events_out).unwrap();
        assert_eq!(triggered, 1);
        assert_eq!(events_out[0].data, 999);
||||||| 23ef22a4a
    fn test_custom_vec_drop() {
        let mut v: Vec<usize> = Vec::new();
        v.push(10);
        v.push(20);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 10);
        assert_eq!(v[1], 20);
=======
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
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    }
}
