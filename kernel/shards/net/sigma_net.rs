#![no_std]
#![allow(dead_code)]

/// SigmaOS Network Driver Abstraction (SigmaNet)
/// Provides a zero-allocation TCP/IP stack foundation using static socket buffers.

use core::sync::atomic::{AtomicUsize, Ordering};

const MAX_SOCKETS: usize = 256;
const MTU_SIZE: usize = 1500;
const SK_BUFF_COUNT: usize = 512;

#[derive(Copy, Clone, PartialEq)]
pub enum Protocol {
    TCP,
    UDP,
    ICMP,
    RAW,
}

#[derive(Copy, Clone, PartialEq)]
pub enum SocketState {
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

#[derive(Copy, Clone)]
pub struct Socket {
    pub active: bool,
    pub protocol: Protocol,
    pub state: SocketState,
    pub local_port: u16,
    pub remote_port: u16,
    pub remote_ip: u32,
    pub tx_bytes: u64,
    pub rx_bytes: u64,
}

/// A static buffer for packet data, similar to Linux sk_buff
#[derive(Copy, Clone)]
pub struct SkBuff {
    pub in_use: bool,
    pub len: usize,
    pub data: [u8; MTU_SIZE],
}

pub struct SigmaNetworkStack {
    sockets: [Socket; MAX_SOCKETS],
    sk_buffs: [SkBuff; SK_BUFF_COUNT],
    next_skb: AtomicUsize,
}

impl SigmaNetworkStack {
    pub const fn new() -> Self {
        let empty_socket = Socket {
            active: false,
            protocol: Protocol::TCP,
            state: SocketState::Closed,
            local_port: 0,
            remote_port: 0,
            remote_ip: 0,
            tx_bytes: 0,
            rx_bytes: 0,
        };
        
        let empty_skb = SkBuff {
            in_use: false,
            len: 0,
            data: [0; MTU_SIZE],
        };

        Self {
            sockets: [empty_socket; MAX_SOCKETS],
            sk_buffs: [empty_skb; SK_BUFF_COUNT],
            next_skb: AtomicUsize::new(0),
        }
    }

    pub fn socket_create(&mut self, protocol: Protocol) -> Result<usize, &'static str> {
        for (i, sock) in self.sockets.iter_mut().enumerate() {
            if !sock.active {
                sock.active = true;
                sock.protocol = protocol;
                sock.state = SocketState::Closed;
                return Ok(i);
            }
        }
        Err("No free sockets")
    }

    pub fn socket_bind(&mut self, fd: usize, port: u16) -> Result<(), &'static str> {
        if fd >= MAX_SOCKETS || !self.sockets[fd].active {
            return Err("Invalid socket");
        }
        
        // Basic collision check
        for sock in self.sockets.iter() {
            if sock.active && sock.local_port == port {
                return Err("Port already in use");
            }
        }
        
        self.sockets[fd].local_port = port;
        Ok(())
    }

    pub fn socket_listen(&mut self, fd: usize) -> Result<(), &'static str> {
        if fd >= MAX_SOCKETS || !self.sockets[fd].active {
            return Err("Invalid socket");
        }
        
        self.sockets[fd].state = SocketState::Listen;
        Ok(())
    }
    
    /// Allocate an SkBuff for transmitting data
    pub fn alloc_skb(&mut self) -> Result<usize, &'static str> {
        // Simple linear scan, could be optimized with a bitmap
        for (i, skb) in self.sk_buffs.iter_mut().enumerate() {
            if !skb.in_use {
                skb.in_use = true;
                skb.len = 0;
                return Ok(i);
            }
        }
        Err("OOM: No free sk_buffs")
    }
    
    pub fn free_skb(&mut self, idx: usize) {
        if idx < SK_BUFF_COUNT {
            self.sk_buffs[idx].in_use = false;
        }
    }
}

static mut G_SIGMA_NET: SigmaNetworkStack = SigmaNetworkStack::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_net_init() {
    // Hardware init goes here (e.g., E1000 PCI probe)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_net_socket(domain: i32, type_: i32, protocol: i32) -> i32 {
    let proto = match type_ {
        1 => Protocol::TCP, // SOCK_STREAM
        2 => Protocol::UDP, // SOCK_DGRAM
        3 => Protocol::RAW, // SOCK_RAW
        _ => return -1,
    };
    
    match G_SIGMA_NET.socket_create(proto) {
        Ok(fd) => fd as i32,
        Err(_) => -1,
    }
}
