#![no_std]
use alloc::vec::Vec;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::String;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::cmp;

// -----------------------------------------------------------------------------
// TCP State Machine and Control Block
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpState {
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
    TimeWait
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CongestionControlAlgo {
    Reno,
    Cubic
}

#[derive(Debug, Clone, Copy)]
pub struct TcpOptions {
    pub mss: u16,
    pub window_scaling: u8,
    pub timestamps: bool,
    pub sack: bool,
    pub ts_val: u32,
    pub ts_ecr: u32,
}

pub struct TcpControlBlock {
    pub state: TcpState,
    pub local_ip: u32,
    pub local_port: u16,
    pub remote_ip: u32,
    pub remote_port: u16,
    
    // Send sequence variables
    pub snd_una: u32,
    pub snd_nxt: u32,
    pub snd_wnd: u32,
    pub snd_up: u32,
    pub snd_wl1: u32,
    pub snd_wl2: u32,
    pub iss: u32,
    
    // Receive sequence variables
    pub rcv_nxt: u32,
    pub rcv_wnd: u32,
    pub rcv_up: u32,
    pub irs: u32,
    
    // Congestion control
    pub cwnd: u32,
    pub ssthresh: u32,
    pub cc_algo: CongestionControlAlgo,
    pub dup_acks: u8,
    
    // Timers
    pub rto: u32,
    pub srtt: u32,
    pub rttvar: u32,
    
    // Buffers
    pub tx_buffer: VecDeque<u8>,
    pub rx_buffer: VecDeque<u8>,
    
    pub options: TcpOptions,
}

impl TcpControlBlock {
    pub fn new(local_ip: u32, local_port: u16, remote_ip: u32, remote_port: u16) -> Self {
        Self {
            state: TcpState::Closed,
            local_ip,
            local_port,
            remote_ip,
            remote_port,
            snd_una: 0,
            snd_nxt: 0,
            snd_wnd: 65535,
            snd_up: 0,
            snd_wl1: 0,
            snd_wl2: 0,
            iss: 0, // Should be randomized
            rcv_nxt: 0,
            rcv_wnd: 65535,
            rcv_up: 0,
            irs: 0,
            cwnd: 10 * 1460, // Initial window size
            ssthresh: 65535,
            cc_algo: CongestionControlAlgo::Cubic,
            dup_acks: 0,
            rto: 1000,
            srtt: 0,
            rttvar: 0,
            tx_buffer: VecDeque::new(),
            rx_buffer: VecDeque::new(),
            options: TcpOptions { mss: 1460, window_scaling: 7, timestamps: true, sack: true, ts_val: 0, ts_ecr: 0 },
        }
    }

    pub fn handle_incoming(&mut self, seq: u32, ack: u32, flags: u8, window: u16, payload: &[u8]) {
        // Implement TCP state transitions based on RFC 793
        let syn = (flags & 0x02) != 0;
        let ack_flag = (flags & 0x10) != 0;
        let fin = (flags & 0x01) != 0;
        let rst = (flags & 0x04) != 0;

        match self.state {
            TcpState::Closed => {
                // Closed state handling
                if syn {
                    self.state = TcpState::SynReceived;
                    self.rcv_nxt = seq + 1;
                    self.irs = seq;
                    // Send SYN-ACK
                }
            },
            TcpState::Listen => {
                if syn {
                    self.state = TcpState::SynReceived;
                    self.rcv_nxt = seq + 1;
                    self.irs = seq;
                }
            },
            TcpState::SynSent => {
                if syn && ack_flag {
                    self.state = TcpState::Established;
                    self.rcv_nxt = seq + 1;
                    self.snd_una = ack;
                    // Send ACK
                } else if syn {
                    self.state = TcpState::SynReceived;
                    self.rcv_nxt = seq + 1;
                    // Send SYN-ACK
                }
            },
            TcpState::SynReceived => {
                if ack_flag && ack == self.snd_nxt {
                    self.state = TcpState::Established;
                }
            },
            TcpState::Established => {
                if ack_flag {
                    if ack > self.snd_una && ack <= self.snd_nxt {
                        self.snd_una = ack;
                        self.dup_acks = 0;
                        // Congestion avoidance
                        if self.cwnd < self.ssthresh {
                            self.cwnd += self.options.mss as u32; // Slow start
                        } else {
                            self.cwnd += (self.options.mss as u32 * self.options.mss as u32) / self.cwnd; // Congestion avoidance
                        }
                    } else if ack == self.snd_una {
                        self.dup_acks += 1;
                        if self.dup_acks == 3 {
                            // Fast retransmit
                            self.ssthresh = core::cmp::max(self.cwnd / 2, 2 * self.options.mss as u32);
                            self.cwnd = self.ssthresh + 3 * self.options.mss as u32;
                        }
                    }
                }
                
                if payload.len() > 0 {
                    if seq == self.rcv_nxt {
                        self.rx_buffer.extend(payload.iter());
                        self.rcv_nxt += payload.len() as u32;
                        // Send ACK
                    } else {
                        // Handle out of order or SACK
                    }
                }
                
                if fin {
                    self.state = TcpState::CloseWait;
                    self.rcv_nxt += 1;
                    // Send ACK
                }
            },
            TcpState::FinWait1 => {
                if ack_flag { self.state = TcpState::FinWait2; }
                if fin {
                    if self.state == TcpState::FinWait2 {
                        self.state = TcpState::TimeWait;
                    } else {
                        self.state = TcpState::Closing;
                    }
                }
            },
            TcpState::FinWait2 => {
                if fin {
                    self.state = TcpState::TimeWait;
                    // Start TimeWait timer
                }
            },
            TcpState::CloseWait => {
                // Application should call close()
            },
            TcpState::Closing => {
                if ack_flag {
                    self.state = TcpState::TimeWait;
                }
            },
            TcpState::LastAck => {
                if ack_flag {
                    self.state = TcpState::Closed;
                }
            },
            TcpState::TimeWait => {
                // Wait for 2*MSL then transition to Closed
            }
        }
    }
}

// -----------------------------------------------------------------------------
// IP Layer (Fragmentation and Reassembly)
// -----------------------------------------------------------------------------

pub struct IpFragment {
    pub offset: u16,
    pub more_fragments: bool,
    pub payload: Vec<u8>,
}

pub struct IpReassemblyBuffer {
    pub src_ip: u32,
    pub dst_ip: u32,
    pub identification: u16,
    pub fragments: BTreeMap<u16, IpFragment>,
    pub total_length: Option<u16>,
    pub timer: u32,
}

pub struct IpLayer {
    pub reassembly_buffers: Vec<IpReassemblyBuffer>,
}

impl IpLayer {
    pub fn new() -> Self {
        Self { reassembly_buffers: Vec::new() }
    }
    
    pub fn handle_fragment(&mut self, src: u32, dst: u32, id: u16, offset: u16, mf: bool, payload: &[u8]) -> Option<Vec<u8>> {
        let buffer = self.reassembly_buffers.iter_mut().find(|b| b.src_ip == src && b.dst_ip == dst && b.identification == id);
        
        let buffer = match buffer {
            Some(b) => b,
            None => {
                self.reassembly_buffers.push(IpReassemblyBuffer {
                    src_ip: src, dst_ip: dst, identification: id,
                    fragments: BTreeMap::new(),
                    total_length: None,
                    timer: 60,
                });
                self.reassembly_buffers.last_mut().unwrap()
            }
        };
        
        buffer.fragments.insert(offset, IpFragment { offset, more_fragments: mf, payload: payload.to_vec() });
        
        if !mf {
            buffer.total_length = Some(offset + payload.len() as u16);
        }
        
        // Check if fully reassembled
        let mut current_offset = 0;
        let mut fully_reassembled = false;
        
        for (off, frag) in &buffer.fragments {
            if *off != current_offset { break; }
            current_offset += frag.payload.len() as u16;
            if !frag.more_fragments {
                fully_reassembled = true;
            }
        }
        
        if fully_reassembled {
            let mut complete_payload = Vec::new();
            for (_, frag) in &buffer.fragments {
                complete_payload.extend(&frag.payload);
            }
            // Remove buffer
            self.reassembly_buffers.retain(|b| !(b.src_ip == src && b.dst_ip == dst && b.identification == id));
            return Some(complete_payload);
        }
        
        None
    }
}

// -----------------------------------------------------------------------------
// UDP and Sockets
// -----------------------------------------------------------------------------

pub struct UdpSocket {
    pub local_port: u16,
    pub remote_ip: Option<u32>,
    pub remote_port: Option<u16>,
    pub rx_queue: VecDeque<Vec<u8>>,
}

pub struct RoutingEntry {
    pub dest: u32,
    pub mask: u32,
    pub gateway: u32,
    pub iface: String,
}

pub struct ArpEntry {
    pub ip: u32,
    pub mac: [u8; 6],
    pub expiry: u64,
}

pub struct NetworkStack {
    pub tcp_connections: BTreeMap<u16, TcpControlBlock>,
    pub udp_sockets: BTreeMap<u16, UdpSocket>,
    pub arp_cache: BTreeMap<u32, ArpEntry>,
    pub routing_table: Vec<RoutingEntry>,
    pub ip_layer: IpLayer,
}

impl NetworkStack {
    pub fn new() -> Self {
        Self {
            tcp_connections: BTreeMap::new(),
            udp_sockets: BTreeMap::new(),
            arp_cache: BTreeMap::new(),
            routing_table: Vec::new(),
            ip_layer: IpLayer::new(),
        }
    }
    
    pub fn resolve_route(&self, ip: u32) -> Option<&RoutingEntry> {
        let mut best_match = None;
        let mut max_prefix = 0;
        
        for route in &self.routing_table {
            if (ip & route.mask) == (route.dest & route.mask) {
                let prefix_len = route.mask.count_ones();
                if prefix_len >= max_prefix {
                    max_prefix = prefix_len;
                    best_match = Some(route);
                }
            }
        }
        best_match
    }
    
    pub fn update_arp(&mut self, ip: u32, mac: [u8; 6]) {
        self.arp_cache.insert(ip, ArpEntry { ip, mac, expiry: 0xFFFFFFFF });
    }
}
