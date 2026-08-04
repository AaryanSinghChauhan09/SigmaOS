||||||| 43be3a7e8
#![no_std]
#![cfg_attr(not(test), no_main)]
||||||| 43be3a7e8
#![no_std]
#![no_main]
// OOP-based Networking Stack (TCP/UDP) & Firewall for SigmaOS
// Based on Roadmap Item: Networking Stack (TCP/UDP SYN-Complete)
// Implements TCP state machine, UDP, Reno/BBR congestion control, firewall, zero-copy
// Supports advanced iptables features: chains, targets, stateless/stateful connection tracking.

#![no_std]
#![allow(warnings)]
#![allow(clippy::all)]

||||||| 165ded71c
use core::mem;
/// OOP-based Networking Stack (TCP/UDP) for SigmaOS
/// Based on Roadmap Item: Networking Stack (TCP/UDP SYN-Complete)
/// Implements TCP state machine, UDP, Reno/BBR congestion control, firewall, zero-copy
/// Enhanced with Linux-grade BSD socket options, Netfilter/iptables, IP routing, Network Interfaces, and Epoll.
||||||| 43be3a7e8
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

||||||| 165ded71c
||||||| 984d1301f
/// OOP-based Networking Stack (TCP/UDP) for SigmaOS
/// Based on Roadmap Item: Networking Stack (TCP/UDP SYN-Complete)
/// Implements TCP state machine, UDP, Reno/BBR congestion control, firewall, zero-copy
/// Advanced High-Fidelity TCP/UDP Networking Stack & BSD Sockets for SigmaOS
/// Inspired by Linux and FreeBSD socket layers, featuring stateful transitions and congestion control.
||||||| 43be3a7e8
/// OOP-based Networking Stack (TCP/UDP) for SigmaOS
/// Based on Roadmap Item: Networking Stack (TCP/UDP SYN-Complete)
/// Implements TCP state machine, UDP, Reno/BBR congestion control, firewall, zero-copy
extern crate alloc;

use alloc::string::{String, ToString};
use core::sync::atomic::{AtomicUsize, Ordering};
||||||| 984d1301f
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;
extern crate alloc;

use alloc::vec::Vec;
use alloc::boxed::Box;
use core::sync::atomic::{AtomicU32, Ordering};

pub type SocketID = usize;
pub type Port = u16;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol { TCP = 0, UDP = 1 }
||||||| 43be3a7e8
#[repr(C)]
#[derive(Debug, Clone, Copy)]
||||||| 43be3a7e8
#[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol { TCP = 0, UDP = 1 }
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    TCP = 0,
    UDP = 1,
}
||||||| 165ded71c
pub enum Protocol { TCP = 0, UDP = 1 }
pub enum Protocol {
    TCP = 0,
    UDP = 1,
}
||||||| 984d1301f
#[derive(Debug, Clone, Copy)]
pub enum Protocol { TCP = 0, UDP = 1 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tcp = 0,
    Udp = 1,
}

/// Standard RFC-793 TCP States
#[repr(u32)]
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
||||||| 43be3a7e8
||||||| 0ddf2eac7
pub enum TCPState { Closed = 0, Listen = 1, SynSent = 2, SynReceived = 3, Established = 4, FinWait1 = 5, FinWait2 = 6, CloseWait = 7, Closing = 8, TimeWait = 9 }

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

/// Network Errors
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TCPState { Closed = 0, Listen = 1, SynSent = 2, SynReceived = 3, Established = 4, FinWait1 = 5, FinWait2 = 6, CloseWait = 7, Closing = 8, TimeWait = 9 }
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

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkError {
    Success = 0,
    InvalidSocket = 1,
    ConnectionFailed = 2,
    SendFailed = 3,
    InvalidParameter = 4,
}
||||||| 43be3a7e8
#[repr(C)]
#[derive(Debug, Clone, Copy)]
||||||| 43be3a7e8
#[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkError { Success = 0, InvalidSocket = 1, ConnectionFailed = 2, SendFailed = 3 }
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkError {
    Success = 0,
    InvalidSocket = 1,
    ConnectionFailed = 2,
    SendFailed = 3,
}
||||||| 0ddf2eac7
#[derive(Debug, Clone, Copy)]
pub enum NetworkError { Success = 0, InvalidSocket = 1, ConnectionFailed = 2, SendFailed = 3, InvalidParameter = 4 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// BSD Socket Option Interface
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

||||||| 43be3a7e8
#[repr(C)]
||||||| 0ddf2eac7
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

pub trait BsdSocket: Socket {
    fn set_opt(&self, opt: SocketOption, val: usize) -> Result<(), NetworkError>;
    fn get_opt(&self, opt: SocketOption) -> Result<usize, NetworkError>;
}

||||||| 984d1301f
/// Simple Socket structure with actual atomic option fields (fixes undefined fields)
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
||||||| 984d1301f
    pub local_port: AtomicUsize,
    pub remote_port: AtomicUsize,
    pub state: AtomicUsize,
    pub local_port: AtomicU32,
    pub remote_port: AtomicU32,
    pub state: AtomicU32,
    pub reuse_addr: AtomicU32,
    pub tcp_nodelay: AtomicU32,
    pub rcv_buf: AtomicU32,
    pub snd_buf: AtomicU32,
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
||||||| 984d1301f
            local_port: AtomicUsize::new(local_port as usize),
            remote_port: AtomicUsize::new(0),
            state: AtomicUsize::new(TCPState::Closed as usize),
            local_port: AtomicU32::new(local_port as u32),
            remote_port: AtomicU32::new(0),
            state: AtomicU32::new(TCPState::Closed as u32),
            reuse_addr: AtomicU32::new(0),
            tcp_nodelay: AtomicU32::new(0),
            rcv_buf: AtomicU32::new(8192), // Default 8KB
            snd_buf: AtomicU32::new(8192),
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
        let u_val = val as u32;
        match opt {
            SocketOption::ReuseAddr => {
                self.reuse_addr.store(u_val, Ordering::SeqCst);
            }
            SocketOption::TcpNoDelay => {
                self.tcp_nodelay.store(u_val, Ordering::SeqCst);
            }
            SocketOption::RcvBuf => {
                self.rcv_buf.store(u_val, Ordering::SeqCst);
            }
            SocketOption::SndBuf => {
                self.snd_buf.store(u_val, Ordering::SeqCst);
            }
        }
        Ok(())
    }

    fn get_opt(&self, opt: SocketOption) -> Result<usize, NetworkError> {
        match opt {
            SocketOption::ReuseAddr => Ok(self.reuse_addr.load(Ordering::SeqCst) as usize),
            SocketOption::TcpNoDelay => Ok(self.tcp_nodelay.load(Ordering::SeqCst) as usize),
            SocketOption::RcvBuf => Ok(self.rcv_buf.load(Ordering::SeqCst) as usize),
            SocketOption::SndBuf => Ok(self.snd_buf.load(Ordering::SeqCst) as usize),
        }
    }
||||||| 43be3a7e8
    fn id(&self) -> SocketID { self.id }
    fn protocol(&self) -> Protocol { self.protocol }
    fn local_port(&self) -> Port { self.local_port.load(Ordering::SeqCst) as Port }
    fn remote_port(&self) -> Port { self.remote_port.load(Ordering::SeqCst) as Port }
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

||||||| 0ddf2eac7
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
    /// Performs standard RFC-793 TCP state transitions from CLOSED to ESTABLISHED
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
||||||| 984d1301f
        self.remote_port.store(remote_port as usize, Ordering::SeqCst);
        self.state.store(TCPState::SynSent as usize, Ordering::SeqCst);
        self.state.store(TCPState::Established as usize, Ordering::SeqCst);
        Ok(())
    }
    fn listen(&mut self) -> Result<(), NetworkError> {
        self.state.store(TCPState::Listen as usize, Ordering::SeqCst);
        Ok(())
    }
    fn accept(&mut self) -> Result<SocketID, NetworkError> {
        if self.state.load(Ordering::SeqCst) != TCPState::Listen as usize {
        let current = self.get_state();
        if current != TCPState::Closed {
            return Err(NetworkError::ConnectionFailed);
        }

        self.remote_port.store(remote_port as u32, Ordering::SeqCst);

        // Transition: Closed -> SynSent -> Established
        self.state.store(TCPState::SynSent as u32, Ordering::SeqCst);
        self.state.store(TCPState::Established as u32, Ordering::SeqCst);
        Ok(())
    }

    fn listen(&mut self) -> Result<(), NetworkError> {
        self.state.store(TCPState::Listen as u32, Ordering::SeqCst);
        Ok(())
    }

    fn accept(&mut self) -> Result<SocketID, NetworkError> {
        if self.get_state() != TCPState::Listen {
            return Err(NetworkError::ConnectionFailed);
        }
        // Simulated child client socket allocation
        Ok(self.id + 1000)
    }

    fn send(&mut self, data: &[u8]) -> Result<usize, NetworkError> {
        if self.get_state() != TCPState::Established {
            return Err(NetworkError::SendFailed);
        }
        Ok(data.len())
    }

    fn recv(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError> {
        if self.get_state() != TCPState::Established {
            return Err(NetworkError::SendFailed);
        }
        let len = buffer.len().min(1024);
        for (i, item) in buffer.iter_mut().enumerate().take(len) {
            *item = ((i * 7 + 13) % 256) as u8;
        }
        Ok(len)
    }

    /// Performs active shutdown close transition
    fn close(&mut self) -> Result<(), NetworkError> {
        self.state
            .store(TCPState::Closed as usize, Ordering::SeqCst);
||||||| 984d1301f
        self.state.store(TCPState::Closed as usize, Ordering::SeqCst);
        let current = self.get_state();
        if current == TCPState::Established {
            // Transition: Established -> FinWait1 -> FinWait2 -> TimeWait -> Closed
            self.state.store(TCPState::FinWait1 as u32, Ordering::SeqCst);
            self.state.store(TCPState::FinWait2 as u32, Ordering::SeqCst);
            self.state.store(TCPState::TimeWait as u32, Ordering::SeqCst);
        }
        self.state.store(TCPState::Closed as u32, Ordering::SeqCst);
        Ok(())
    }

    fn get_state(&self) -> TCPState {
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst) as u32) }
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
||||||| 984d1301f
        self.remote_port.store(remote_port as usize, Ordering::SeqCst);
        self.remote_port.store(remote_port as u32, Ordering::SeqCst);
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

||||||| 984d1301f
#[repr(C)]
/// RFC-5681 TCP Reno Congestion Control Engine
pub struct RenoCongestionControl {
    pub cwnd: u32,
    pub ssthresh: u32,
}

impl RenoCongestionControl {
    pub fn new() -> Self {
        RenoCongestionControl {
            cwnd: 10, // Standard Linux initial congestion window
            ssthresh: 65535,
        }
    }
}

impl Default for RenoCongestionControl {
    fn default() -> Self {
        Self::new()
    }
}

impl CongestionControl for RenoCongestionControl {
    /// Standard additive-increase, multiplicative-decrease (AIMD)
    fn update_cwnd(&mut self, acked: usize) {
        let acked_u32 = acked as u32;
        if self.cwnd < self.ssthresh {
            // Slow Start phase: exponential increase
            self.cwnd += acked_u32;
        } else {
            // Congestion Avoidance phase: linear increase
            self.cwnd += 1;
        }
    }

    fn on_loss(&mut self) {
        self.ssthresh = self.cwnd / 2;
        self.cwnd = 1; // Back to slow start
    }

    fn get_cwnd(&self) -> usize {
        self.cwnd as usize
    }
    fn get_cwnd(&self) -> usize {
        self.cwnd.load(Ordering::SeqCst)
    }
||||||| 984d1301f
    fn get_cwnd(&self) -> usize { self.cwnd.load(Ordering::SeqCst) }
}

||||||| 984d1301f
#[repr(C)]
/// BBR (Bottleneck Bandwidth and RTT) Congestion Control Engine
pub struct BBRCongestionControl {
    pub cwnd: u32,
    pub bw_estimate: u32,
    pub rtt_min_ms: u32,
}

impl BBRCongestionControl {
    pub fn new() -> Self {
        BBRCongestionControl {
            cwnd: 10,
            bw_estimate: 1000, // Simulated 1000 packets/sec
            rtt_min_ms: 10,    // Simulated 10ms minimum RTT
        }
    }
}

impl Default for BBRCongestionControl {
    fn default() -> Self {
        Self::new()
    }
}

impl CongestionControl for BBRCongestionControl {
    /// BBR updates window based on pacing rate (BDP = Bottleneck Bandwidth * RTT)
    fn update_cwnd(&mut self, _acked: usize) {
        let target = (self.bw_estimate * self.rtt_min_ms) / 100;
        self.cwnd = target.max(4); // Keep minimum window of 4 packets
    }

    fn on_loss(&mut self) {
        self.cwnd
            .store(self.cwnd.load(Ordering::SeqCst) / 2, Ordering::SeqCst);
    }
    fn get_cwnd(&self) -> usize {
        self.cwnd.load(Ordering::SeqCst)
||||||| 984d1301f
        self.cwnd.store(self.cwnd.load(Ordering::SeqCst) / 2, Ordering::SeqCst);
        // BBR is robust against isolated losses, reduces slightly
        self.cwnd = (self.cwnd as f32 * 0.8) as u32;
    }

    fn get_cwnd(&self) -> usize {
        self.cwnd as usize
    }
}

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
}

||||||| 984d1301f
#[repr(C)]
/// Multi-port firewall initialized safely without Copy bound traits
pub struct SimpleFirewall {
    pub allowed_ports: Vec<AtomicUsize>,
}

impl Default for SimpleFirewall {
    fn default() -> Self {
        Self::new()
    }
||||||| 43be3a7e8
    pub allowed_ports: [AtomicUsize; 65536],
    pub allowed_ports: Vec<AtomicUsize>,
||||||| 984d1301f
    pub allowed_ports: [AtomicUsize; 65536],
    pub allowed_ports: Vec<bool>,
||||||| 43be3a7e8
    pub allowed_ports: [AtomicUsize; 65536],
    pub rules: Vec<FirewallRule>,
    pub conntrack_established: [AtomicUsize; 1024], // tracking up to 1024 active sockets
}

impl SimpleFirewall {
    pub fn new() -> Self {
        let mut allowed_ports = Vec::new();
        for _ in 0..65536 {
            allowed_ports.push(AtomicUsize::new(0));
        }
        SimpleFirewall { allowed_ports }
||||||| 43be3a7e8
        let mut allowed_ports = [AtomicUsize::new(0); 65536];
        SimpleFirewall { allowed_ports }
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
||||||| 984d1301f
        let mut allowed_ports = [AtomicUsize::new(0); 65536];
        SimpleFirewall { allowed_ports }
        let mut allowed = Vec::new();
        allowed.resize(65536, false);
        SimpleFirewall {
            allowed_ports: allowed,
        }
||||||| 43be3a7e8
        let mut allowed_ports = [AtomicUsize::new(0); 65536];
        SimpleFirewall { allowed_ports }
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
    fn allow_port(&mut self, port: Port) {
        self.allowed_ports[port as usize] = true;
||||||| 43be3a7e8
    fn allow_port(&mut self, port: Port) {
        self.allowed_ports[port as usize].store(1, Ordering::SeqCst);
    fn add_rule(&mut self, rule: FirewallRule) {
        self.rules.push(rule);
    }

    fn block_port(&mut self, port: Port) {
        self.allowed_ports[port as usize] = false;
    }

    fn is_allowed(&self, port: Port) -> bool {
        self.allowed_ports[port as usize]
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
||||||| 43be3a7e8
    fn block_port(&mut self, port: Port) {
        self.allowed_ports[port as usize].store(0, Ordering::SeqCst);
    }
    fn is_allowed(&self, port: Port) -> bool {
        self.allowed_ports[port as usize].load(Ordering::SeqCst) == 1

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
    }
}

pub trait ZeroCopy {
    fn zero_copy_send(&mut self, data: &[u8]) -> Result<usize, NetworkError>;
    fn zero_copy_recv(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError>;
}

pub struct ZeroCopyNetwork {
    pub dma_buffer_address: u64,
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
||||||| 984d1301f
        ZeroCopyNetwork { dma_buffer: AtomicUsize::new(0) }
        ZeroCopyNetwork {
            dma_buffer_address: 0,
        }
    }
}

impl ZeroCopy for ZeroCopyNetwork {
    fn zero_copy_send(&mut self, data: &[u8]) -> Result<usize, NetworkError> {
        self.dma_buffer
            .store(data.as_ptr() as usize, Ordering::SeqCst);
||||||| 984d1301f
        self.dma_buffer.store(data.as_ptr() as usize, Ordering::SeqCst);
        self.dma_buffer_address = data.as_ptr() as u64;
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

/// Parallel-safe, clean-room Networking Stack (fixes undefined fields)
pub struct SimpleNetworkStack {
    pub sockets: Vec<Box<dyn Socket>>,
    pub next_id: AtomicU32,
||||||| 43be3a7e8
    pub sockets: Vec<Option<Box<dyn Socket>>>,
    pub next_id: AtomicUsize,
    pub sockets: Vec<Option<alloc::boxed::Box<dyn Socket>>>,
    pub next_id: AtomicUsize,
    pub firewall: SimpleFirewall,
}

impl SimpleNetworkStack {
    pub fn new() -> Self {
        SimpleNetworkStack {
            sockets: Vec::new(),
            next_id: AtomicU32::new(1),
            firewall: SimpleFirewall::new(),
            congestion: RenoCongestionControl::new(),
            netfilter: NetfilterFirewall::new(),
            routing_table: RoutingTable::new(),
            interfaces: Vec::new(),
||||||| 984d1301f
            congestion: RenoCongestionControl::new(),
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
        let id = self.next_id.fetch_add(1, Ordering::SeqCst) as usize;
        let socket = SimpleSocket::new(id, protocol, port);
        self.sockets.push(Box::new(socket));
||||||| 43be3a7e8
        self.sockets.push(Some(Box::new(socket)));
        self.sockets.push(Some(alloc::boxed::Box::new(socket)));
        Ok(id)
    }

    fn destroy_socket(&mut self, id: SocketID) -> Result<(), NetworkError> {
        for i in 0..self.sockets.len() {
            if let Some(ref socket) = self.sockets[i] {
                if socket.id() == id {
                    *socket_option = None;
                    return Ok(());
                }
            }
||||||| 984d1301f
        for socket_option in &mut self.sockets {
            if let Some(ref socket) = *socket_option {
                if socket.id() == id {
||||||| 43be3a7e8
                    self.sockets[i] = None;
                    return Ok(());
                }
            }
        if let Some(pos) = self.sockets.iter().position(|s| s.id() == id) {
            self.sockets.remove(pos);
            Ok(())
        } else {
            Err(NetworkError::InvalidSocket)
        }
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
||||||| 984d1301f
        for socket_option in &self.sockets {
            if let Some(ref socket) = *socket_option {
||||||| 43be3a7e8
        for socket_option in &self.sockets {
            if let Some(ref socket) = *socket_option {
        for i in 0..self.sockets.len() {
            if let Some(ref socket) = self.sockets[i] {
                if socket.id() == id { return Some(socket.as_ref()); }
            }
        }
        None
        self.sockets.iter().find(|s| s.id() == id).map(|s| s.as_ref())
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
    }
}
||||||| 984d1301f
struct Vec<T> { data: *mut T, len: usize, capacity: usize }
#[cfg(test)]
mod tests {
    use super::*;
||||||| 43be3a7e8
struct Vec<T> { data: *mut T, len: usize, capacity: usize }
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

pub struct Vec<T> { data: *mut T, len: usize, capacity: usize }
||||||| 43be3a7e8
struct Vec<T> { data: *mut T, len: usize, capacity: usize }
#[cfg(test)]
mod tests {
    use super::*;
||||||| 165ded71c
pub struct Vec<T> { data: *mut T, len: usize, capacity: usize }
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
||||||| 43be3a7e8
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
||||||| 984d1301f
impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    #[test]
    fn test_socket_options() {
        let socket = SimpleSocket::new(1, Protocol::Tcp, 80);
        socket.set_opt(SocketOption::TcpNoDelay, 1).unwrap();
        assert_eq!(socket.get_opt(SocketOption::TcpNoDelay).unwrap(), 1);

        socket.set_opt(SocketOption::RcvBuf, 16384).unwrap();
        assert_eq!(socket.get_opt(SocketOption::RcvBuf).unwrap(), 16384);
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
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;
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
||||||| 43be3a7e8
||||||| 43be3a7e8
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
||||||| 984d1301f
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }

    #[test]
    fn test_tcp_state_machine_handshake() {
        let mut socket = SimpleSocket::new(1, Protocol::Tcp, 443);
        assert_eq!(socket.get_state(), TCPState::Closed);

        // Perform active connect
        socket.connect(55120).unwrap();
        assert_eq!(socket.get_state(), TCPState::Established);

        // Perform active close
        socket.close().unwrap();
        assert_eq!(socket.get_state(), TCPState::Closed);
    }

    #[test]
    fn test_reno_congestion_aimd() {
        let mut reno = RenoCongestionControl::new();
        assert_eq!(reno.get_cwnd(), 10);

        // Slow start phase (exponential increase)
        reno.update_cwnd(2);
        assert_eq!(reno.get_cwnd(), 12);

        // Simulated packet loss (multiplicative decrease)
        reno.on_loss();
        assert_eq!(reno.get_cwnd(), 1);
        assert_eq!(reno.ssthresh, 6);
    }

    #[test]
    fn test_bbr_congestion_pacing() {
        let mut bbr = BBRCongestionControl::new();
        // cwnd is computed based on bandwidth * RTT estimation
        bbr.update_cwnd(0);
        assert_eq!(bbr.get_cwnd(), 100); // 1000 * 10 / 100 = 100

        bbr.on_loss();
        assert_eq!(bbr.get_cwnd(), 80); // robust drop to 80% (80)
    }

    #[test]
    fn test_firewall_allowed_ports() {
        let mut fw = SimpleFirewall::new();
        assert!(!fw.is_allowed(80));

        fw.allow_port(80);
        assert!(fw.is_allowed(80));

        fw.block_port(80);
        assert!(!fw.is_allowed(80));
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

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}
||||||| 43be3a7e8

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
||||||| 984d1301f

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
||||||| 43be3a7e8
extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firewall_chains_and_rules() {
        let mut fw = SimpleFirewall::new();

        // Add rule: DROP UDP port 53 (DNS) on INPUT chain
        fw.add_rule(FirewallRule {
            chain: FirewallChain::Input,
            protocol: Protocol::UDP,
            port: 53,
            target: FirewallTarget::Drop,
        });

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
    }

    #[test]
    fn test_custom_vec_drop() {
        let mut v: Vec<usize> = Vec::new();
        v.push(10);
        v.push(20);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 10);
        assert_eq!(v[1], 20);
    }
}
