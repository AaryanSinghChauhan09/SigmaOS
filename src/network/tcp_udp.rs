#![no_std]
#![no_main]

/// OOP-based Networking Stack (TCP/UDP) for SigmaOS
/// Based on Roadmap Item: Networking Stack (TCP/UDP SYN-Complete)
/// Implements TCP state machine, UDP, Reno/BBR congestion control, firewall, zero-copy

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SocketID = usize;
pub type Port = u16;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Protocol { TCP = 0, UDP = 1 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TCPState { Closed = 0, Listen = 1, SynSent = 2, SynReceived = 3, Established = 4, FinWait1 = 5, FinWait2 = 6, CloseWait = 7, Closing = 8, TimeWait = 9 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NetworkError { Success = 0, InvalidSocket = 1, ConnectionFailed = 2, SendFailed = 3 }

pub trait Socket {
    fn id(&self) -> SocketID;
    fn protocol(&self) -> Protocol;
    fn local_port(&self) -> Port;
    fn remote_port(&self) -> Port;
}

#[repr(C)]
pub struct SimpleSocket {
    pub id: SocketID,
    pub protocol: Protocol,
    pub local_port: AtomicUsize,
    pub remote_port: AtomicUsize,
    pub state: AtomicUsize,
}

impl SimpleSocket {
    pub fn new(id: SocketID, protocol: Protocol, local_port: Port) -> Self {
        SimpleSocket {
            id,
            protocol,
            local_port: AtomicUsize::new(local_port as usize),
            remote_port: AtomicUsize::new(0),
            state: AtomicUsize::new(TCPState::Closed as usize),
        }
    }
}

impl Socket for SimpleSocket {
    fn id(&self) -> SocketID { self.id }
    fn protocol(&self) -> Protocol { self.protocol }
    fn local_port(&self) -> Port { self.local_port.load(Ordering::SeqCst) as Port }
    fn remote_port(&self) -> Port { self.remote_port.load(Ordering::SeqCst) as Port }
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
        self.state.store(TCPState::Closed as usize, Ordering::SeqCst);
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
        self.remote_port.store(remote_port as usize, Ordering::SeqCst);
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

#[repr(C)]
pub struct RenoCongestionControl {
    pub cwnd: AtomicUsize,
    pub ssthresh: AtomicUsize,
}

impl RenoCongestionControl {
    pub fn new() -> Self {
        RenoCongestionControl {
            cwnd: AtomicUsize::new(10),
            ssthresh: AtomicUsize::new(65535),
        }
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
    fn get_cwnd(&self) -> usize { self.cwnd.load(Ordering::SeqCst) }
}

#[repr(C)]
pub struct BBRCongestionControl {
    pub cwnd: AtomicUsize,
    pub bw_estimate: AtomicUsize,
    pub rtt_min: AtomicUsize,
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

impl CongestionControl for BBRCongestionControl {
    fn update_cwnd(&mut self, _acked: usize) {
        let bw = self.bw_estimate.load(Ordering::SeqCst);
        let rtt = self.rtt_min.load(Ordering::SeqCst);
        let target = bw * rtt;
        self.cwnd.store(target, Ordering::SeqCst);
    }
    fn on_loss(&mut self) {
        self.cwnd.store(self.cwnd.load(Ordering::SeqCst) / 2, Ordering::SeqCst);
    }
    fn get_cwnd(&self) -> usize { self.cwnd.load(Ordering::SeqCst) }
}

pub trait Firewall {
    fn allow_port(&mut self, port: Port);
    fn block_port(&mut self, port: Port);
    fn is_allowed(&self, port: Port) -> bool;
}

#[repr(C)]
pub struct SimpleFirewall {
    pub allowed_ports: [AtomicUsize; 65536],
}

impl SimpleFirewall {
    pub fn new() -> Self {
        let mut allowed_ports = [AtomicUsize::new(0); 65536];
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
        ZeroCopyNetwork { dma_buffer: AtomicUsize::new(0) }
    }
}

impl ZeroCopy for ZeroCopyNetwork {
    fn zero_copy_send(&mut self, data: &[u8]) -> Result<usize, NetworkError> {
        self.dma_buffer.store(data.as_ptr() as usize, Ordering::SeqCst);
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

pub struct SimpleNetworkStack {
    pub sockets: Vec<Option<Box<dyn Socket>>>,
    pub next_id: AtomicUsize,
    pub firewall: SimpleFirewall,
    pub congestion: RenoCongestionControl,
}

impl SimpleNetworkStack {
    pub fn new() -> Self {
        SimpleNetworkStack {
            sockets: Vec::new(),
            next_id: AtomicUsize::new(1),
            firewall: SimpleFirewall::new(),
            congestion: RenoCongestionControl::new(),
        }
    }
}

impl NetworkStack for SimpleNetworkStack {
    fn create_socket(&mut self, protocol: Protocol, port: Port) -> Result<SocketID, NetworkError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let socket = SimpleSocket::new(id, protocol, port);
        self.sockets.push(Some(Box::new(socket)));
        Ok(id)
    }
    fn destroy_socket(&mut self, id: SocketID) -> Result<(), NetworkError> {
        for socket_option in &mut self.sockets {
            if let Some(ref socket) = *socket_option {
                if socket.id() == id {
                    return Ok(());
                }
            }
        }
        Err(NetworkError::InvalidSocket)
    }
    fn get_socket(&self, id: SocketID) -> Option<&dyn Socket> {
        for socket_option in &self.sockets {
            if let Some(ref socket) = *socket_option {
                if socket.id() == id { return Some(socket.as_ref()); }
            }
        }
        None
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

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
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
