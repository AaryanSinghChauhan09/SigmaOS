#![no_std]

extern crate alloc;
use alloc::vec::Vec;

use crate::kernel::vfs::inode::FsError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkError {
    InitFailed,
    SendFailed,
    ReceiveFailed,
    SocketError,
    CapabilityDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    AlreadyConnected,
}

pub struct SkBuff {
    pub head: *mut u8,
    pub data: *mut u8,
    pub tail: *mut u8,
    pub end: *mut u8,
    pub len: usize,
    pub mac_len: usize,
    pub network_header: usize,
    pub transport_header: usize,
    pub protocol: u16,
    pub dev: Option<*const NetDevice>,
    pub priority: u32,
    pub mark: u32,
    pub pkt_type: PktType,
    pub tstamp: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PktType {
    Unicast,
    Multicast,
    Broadcast,
    Others,
    Outgoing,
}

impl SkBuff {
    pub fn new() -> Self {
        SkBuff {
            head: core::ptr::null_mut(),
            data: core::ptr::null_mut(),
            tail: core::ptr::null_mut(),
            end: core::ptr::null_mut(),
            len: 0,
            mac_len: 0,
            network_header: 0,
            transport_header: 0,
            protocol: 0,
            dev: None,
            priority: 0,
            mark: 0,
            pkt_type: PktType::Unicast,
            tstamp: 0,
        }
    }
}

pub trait Socket: Send + Sync {
    fn connect(&mut self, remote_addr: &SocketAddr) -> Result<(), NetworkError>;
    fn bind(&mut self, local_addr: &SocketAddr) -> Result<(), NetworkError>;
    fn listen(&mut self, backlog: u32) -> Result<(), NetworkError>;
    fn accept(&mut self) -> Result<Box<dyn Socket>, NetworkError>;
    fn send(&mut self, buf: &[u8]) -> Result<usize, NetworkError>;
    fn recv(&mut self, buf: &mut [u8]) -> Result<usize, NetworkError>;
    fn send_to(&mut self, buf: &[u8], dest: &SocketAddr) -> Result<usize, NetworkError>;
    fn recv_from(&mut self, buf: &mut [u8]) -> Result<(usize, SocketAddr), NetworkError>;
    fn close(&mut self) -> Result<(), NetworkError>;
    fn get_state(&self) -> SocketState;
    fn get_socket_name(&self) -> Result<SocketAddr, NetworkError>;
    fn get_peer_name(&self) -> Result<SocketAddr, NetworkError>;
    fn set_nonblocking(&mut self, nonblocking: bool) -> Result<(), NetworkError>;
    fn poll(&self, events: u32) -> Result<u32, NetworkError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Uninit,
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    LastAck,
    Closing,
    TimeWait,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketAddr {
    pub family: u16,
    pub port: u16,
    pub addr: [u8; 16],
}

impl SocketAddr {
    pub fn new_ipv4(port: u16, addr: [u8; 4]) -> Self {
        let mut full_addr = [0u8; 16];
        full_addr[0..4].copy_from_slice(&addr);
        SocketAddr {
            family: 2,
            port,
            addr: full_addr,
        }
    }
}

pub trait NetDevice: Send + Sync {
    fn name(&self) -> &str;
    fn ifindex(&self) -> u32;
    fn flags(&self) -> u32;
    fn mtu(&self) -> u32;
    fn features(&self) -> u64;
    fn dev_type(&self) -> u16;
    fn operstate(&self) -> OperState;
    fn addr_len(&self) -> usize;
    fn dev_addr(&self) -> &[u8];
    fn broadcast(&self) -> &[u8];
    fn open(&mut self) -> Result<(), NetworkError>;
    fn stop(&mut self) -> Result<(), NetworkError>;
    fn start_xmit(&mut self, skb: SkBuff) -> Result<(), NetworkError>;
    fn do_ioctl(&mut self, cmd: u32, arg: usize) -> Result<(), NetworkError>;
    fn set_rx_mode(&mut self) -> Result<(), NetworkError>;
    fn change_mtu(&mut self, new_mtu: u32) -> Result<(), NetworkError>;
    fn validate_addr(&self) -> Result<(), NetworkError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperState {
    Unknown,
    NotPresent,
    Down,
    LowerLayerDown,
    Testing,
    Dormant,
    Up,
}

pub protocol TcpSk {
    snd_una: u32,
    snd_nxt: u32,
    rcv_nxt: u32,
    snd_wl1: u32,
    snd_wl2: u32,
    snd_wnd: u32,
    rcv_wnd: u32,
    cwnd: u32,
    ssthresh: u32,
    retransmits: u32,
    out_of_order: u32,
    rcv_tstamp: bool,
    snd_tstamp: bool,
}

pub trait CongestionControl: Send + Sync {
    fn update_cwnd(&mut self, acked: usize);
    fn on_loss(&mut self);
    fn get_cwnd(&self) -> usize;
    fn init(&mut self);
    fn name(&self) -> &str;
}

pub struct RenoCongestionControl {
    pub cwnd: usize,
    pub ssthresh: usize,
}

impl CongestionControl for RenoCongestionControl {
    fn update_cwnd(&mut self, acked: usize) {
        let cwnd = self.cwnd;
        let ssthresh = self.ssthresh;
        if cwnd < ssthresh {
            self.cwnd += acked;
        } else {
            self.cwnd += 1;
        }
    }
    fn on_loss(&mut self) {
        let cwnd = self.cwnd;
        self.ssthresh = cwnd / 2;
        self.cwnd = 1;
    }
    fn get_cwnd(&self) -> usize {
        self.cwnd
    }
    fn init(&mut self) {
        self.cwnd = 1;
        self.ssthresh = 65535;
    }
    fn name(&self) -> &str {
        "reno"
    }
}

pub struct BbrCongestionControl {
    pub cwnd: usize,
    pub pacing_gain: f64,
    pub cwnd_gain: f64,
    pub btlbw: usize,
    pub rtt: u64,
}

impl CongestionControl for BbrCongestionControl {
    fn update_cwnd(&mut self, acked: usize) {
        self.cwnd += acked;
    }
    fn on_loss(&mut self) {
        self.cwnd = self.cwnd / 2;
    }
    fn get_cwnd(&self) -> usize {
        self.cwnd
    }
    fn init(&mut self) {
        self.cwnd = 10;
        self.pacing_gain = 1.25;
        self.cwnd_gain = 2.0;
        self.btlbw = 0;
        self.rtt = 0;
    }
    fn name(&self) -> &str {
        "bbr"
    }
}

pub struct NetfilterHook {
    pub hook: fn(skb: &SkBuff) -> i32,
    pub priority: i32,
    pub dev: Option<String>,
}

pub struct NetfilterRule {
    pub chain: String,
    pub protocol: u8,
    pub src_addr: [u8; 16],
    pub dst_addr: [u8; 16],
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub action: NFAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NFAction {
    Accept,
    Drop,
    Reject,
    Queue,
    Stop,
}

pub struct Netfilter {
    pub rules: Vec<NetfilterRule>,
}

impl Netfilter {
    pub fn new() -> Self {
        Netfilter { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: NetfilterRule) {
        self.rules.push(rule);
    }

    pub fn is_allowed(&self, port: u16) -> bool {
        for rule in &self.rules {
            if let Some(rule_port) = rule.dst_port {
                if rule_port == port && rule.action == NFAction::Accept {
                    return true;
                }
            }
        }
        false
    }
}

pub trait Qdisc: Send + Sync {
    fn enqueue(&mut self, skb: SkBuff) -> Result<(), NetworkError>;
    fn dequeue(&mut self) -> Option<SkBuff>;
    fn peek(&self) -> Option<&SkBuff>;
    fn drop(&mut self) -> usize;
    fn reset(&mut self);
    fn qlen(&self) -> usize;
    fn name(&self) -> &str;
}

pub struct PfifoFast {
    pub queues: Vec<Vec<SkBuff>>,
    pub limit: usize,
}

impl Qdisc for PfifoFast {
    fn enqueue(&mut self, skb: SkBuff) -> Result<(), NetworkError> {
        if self.qlen() >= self.limit {
            return Err(NetworkError::IoError);
        }
        self.queues[0].push(skb);
        Ok(())
    }
    fn dequeue(&mut self) -> Option<SkBuff> {
        for q in &mut self.queues {
            if !q.is_empty() {
                return q.remove(0);
            }
        }
        None
    }
    fn peek(&self) -> Option<&SkBuff> {
        None
    }
    fn drop(&mut self) -> usize { 0 }
    fn reset(&mut self) {}
    fn qlen(&self) -> usize { self.queues.iter().map(|q| q.len()).sum() }
    fn name(&self) -> &str { "pfifo_fast" }
}

pub struct QdiscManager {
    pub qdiscs: Vec<Box<dyn Qdisc>>,
}

impl QdiscManager {
    pub fn new() -> Self {
        QdiscManager { qdiscs: Vec::new() }
    }

    pub fn register(&mut self, qdisc: Box<dyn Qdisc>) {
        self.qdiscs.push(qdisc);
    }
}