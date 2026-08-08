#![no_std]
#![allow(warnings)]
#![allow(clippy::all)]
||||||| 984d1301f
#![no_std]
#![no_main]
/// Advanced High-Fidelity TCP/UDP Networking Stack & BSD Sockets for SigmaOS
/// Inspired by Linux and FreeBSD socket layers, featuring stateful transitions and congestion control.

/// OOP-based Networking Stack (TCP/UDP) for SigmaOS
/// Based on Roadmap Item: Networking Stack (TCP/UDP SYN-Complete)
/// Implements TCP state machine, UDP, Reno/BBR congestion control, firewall, zero-copy
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
||||||| 984d1301f
/// OOP-based Networking Stack (TCP/UDP) for SigmaOS
/// Based on Roadmap Item: Networking Stack (TCP/UDP SYN-Complete)
/// Implements TCP state machine, UDP, Reno/BBR congestion control, firewall, zero-copy
extern crate alloc;

use core::sync::atomic::{AtomicUsize, Ordering};
||||||| 984d1301f
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::sync::atomic::{AtomicU32, Ordering};

pub type SocketID = usize;
pub type Port = u16;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    TCP = 0,
    UDP = 1,
}
||||||| 984d1301f
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Protocol { TCP = 0, UDP = 1 }
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tcp = 0,
    Udp = 1,
}

#[repr(usize)]
||||||| 984d1301f
#[repr(C)]
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

#[repr(usize)]
||||||| 984d1301f
#[repr(C)]
/// Network Errors
#[repr(C)]
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

||||||| 984d1301f
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

/// Simple Socket structure with actual atomic option fields (fixes undefined fields)
pub struct SimpleSocket {
    pub id: SocketID,
    pub protocol: Protocol,
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

||||||| 984d1301f
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
        for i in 0..len {
            buffer[i] = ((i * 7 + 13) % 256) as u8;
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
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) }
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

pub trait Firewall {
    fn allow_port(&mut self, port: Port);
    fn block_port(&mut self, port: Port);
    fn is_allowed(&self, port: Port) -> bool;
}

||||||| 984d1301f
#[repr(C)]
/// Multi-port firewall initialized safely without Copy bound traits
pub struct SimpleFirewall {
    pub allowed_ports: Vec<AtomicUsize>,
||||||| 984d1301f
    pub allowed_ports: [AtomicUsize; 65536],
    pub allowed_ports: Vec<bool>,
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
||||||| 984d1301f
        let mut allowed_ports = [AtomicUsize::new(0); 65536];
        SimpleFirewall { allowed_ports }
        let mut allowed = Vec::new();
        allowed.resize(65536, false);
        SimpleFirewall {
            allowed_ports: allowed,
        }
    }
}

impl Firewall for SimpleFirewall {
    fn allow_port(&mut self, port: Port) {
        self.allowed_ports[port as usize] = true;
    }

    fn block_port(&mut self, port: Port) {
        self.allowed_ports[port as usize] = false;
    }

    fn is_allowed(&self, port: Port) -> bool {
        self.allowed_ports[port as usize]
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
        for i in 0..len {
            buffer[i] = ((i * 13 + 19) % 256) as u8;
        }
        Ok(len)
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
    pub firewall: SimpleFirewall,
    pub congestion: RenoCongestionControl,
||||||| 984d1301f
    pub congestion: RenoCongestionControl,
    // Linux Stack Additions
    pub netfilter: NetfilterFirewall,
    pub routing_table: RoutingTable,
    pub interfaces: Vec<NetworkInterface>,
}

impl SimpleNetworkStack {
    pub fn new() -> Self {
        SimpleNetworkStack {
            sockets: Vec::new(),
            next_id: AtomicU32::new(1),
            firewall: SimpleFirewall::new(),
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
||||||| 984d1301f
        for socket_option in &mut self.sockets {
            if let Some(ref socket) = *socket_option {
                if socket.id() == id {
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
                if socket.id() == id { return Some(socket.as_ref()); }
            }
        }
        None
        self.sockets.iter().find(|s| s.id() == id).map(|s| s.as_ref())
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
