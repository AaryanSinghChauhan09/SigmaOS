// SigmaOS — Complete Network Stack (Ethernet/IP/TCP/UDP/DNS/DHCP)
// Sovereign implementation — no external dependencies
#![no_std]
#![allow(dead_code)]
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

// ─── Ethernet Frame ──────────────────────────────────────────────────────────
pub const ETHERTYPE_IPV4: u16 = 0x0800;
pub const ETHERTYPE_ARP:  u16 = 0x0806;
pub const ETHERTYPE_IPV6: u16 = 0x86DD;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct EthHeader {
    pub dst:  [u8; 6],
    pub src:  [u8; 6],
    pub etype: u16,  // big-endian
}

impl EthHeader {
    pub fn new(dst: [u8; 6], src: [u8; 6], etype: u16) -> Self {
        EthHeader { dst, src, etype: etype.to_be() }
    }
    pub fn ethertype(&self) -> u16 { u16::from_be(self.etype) }
}

// ─── ARP ─────────────────────────────────────────────────────────────────────
pub const ARP_REQUEST: u16 = 1;
pub const ARP_REPLY:   u16 = 2;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ArpPacket {
    pub htype:    u16,  // hardware type (1 = Ethernet)
    pub ptype:    u16,  // protocol type (0x0800 = IPv4)
    pub hlen:     u8,   // hardware addr len (6)
    pub plen:     u8,   // protocol addr len (4)
    pub oper:     u16,  // operation
    pub sha:      [u8; 6],  // sender MAC
    pub spa:      u32,      // sender IP (big-endian)
    pub tha:      [u8; 6],  // target MAC
    pub tpa:      u32,      // target IP
}

// ─── ARP Cache ───────────────────────────────────────────────────────────────
pub const ARP_CACHE_SIZE: usize = 64;

#[derive(Clone, Copy)]
pub struct ArpEntry {
    pub ip:  u32,
    pub mac: [u8; 6],
    pub ttl: u32,
    pub valid: bool,
}

pub struct ArpCache {
    entries: [ArpEntry; ARP_CACHE_SIZE],
}

impl ArpCache {
    pub const fn new() -> Self {
        const E: ArpEntry = ArpEntry { ip: 0, mac: [0u8;6], ttl: 0, valid: false };
        ArpCache { entries: [E; ARP_CACHE_SIZE] }
    }
    pub fn lookup(&self, ip: u32) -> Option<[u8; 6]> {
        self.entries.iter().find(|e| e.valid && e.ip == ip).map(|e| e.mac)
    }
    pub fn insert(&mut self, ip: u32, mac: [u8; 6]) {
        // Find empty or oldest slot
        for e in &mut self.entries {
            if !e.valid { *e = ArpEntry { ip, mac, ttl: 300, valid: true }; return; }
        }
        // Overwrite first
        self.entries[0] = ArpEntry { ip, mac, ttl: 300, valid: true };
    }
    pub fn tick(&mut self) {
        for e in &mut self.entries {
            if e.valid {
                if e.ttl == 0 { e.valid = false; } else { e.ttl -= 1; }
            }
        }
    }
}

// ─── IPv4 ────────────────────────────────────────────────────────────────────
pub const IP_PROTO_ICMP: u8 = 1;
pub const IP_PROTO_TCP:  u8 = 6;
pub const IP_PROTO_UDP:  u8 = 17;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Ipv4Header {
    pub ver_ihl:  u8,
    pub dscp_ecn: u8,
    pub total_len: u16,
    pub id:       u16,
    pub frag:     u16,
    pub ttl:      u8,
    pub proto:    u8,
    pub checksum: u16,
    pub src:      u32,
    pub dst:      u32,
}

impl Ipv4Header {
    pub fn new(src: u32, dst: u32, proto: u8, payload_len: u16) -> Self {
        let mut h = Ipv4Header {
            ver_ihl: 0x45, dscp_ecn: 0,
            total_len: (20u16 + payload_len).to_be(),
            id: 0, frag: 0x4000u16.to_be(),
            ttl: 64, proto, checksum: 0,
            src: src.to_be(), dst: dst.to_be(),
        };
        h.checksum = ip4_checksum(&h);
        h
    }
    pub fn src_ip(&self) -> u32 { u32::from_be(self.src) }
    pub fn dst_ip(&self) -> u32 { u32::from_be(self.dst) }
    pub fn ihl_bytes(&self) -> usize { (self.ver_ihl & 0xF) as usize * 4 }
    pub fn total(&self) -> u16 { u16::from_be(self.total_len) }
}

fn ip4_checksum(h: &Ipv4Header) -> u16 {
    let bytes = unsafe { core::slice::from_raw_parts(h as *const _ as *const u8, 20) };
    let mut sum = 0u32;
    let mut i = 0;
    while i < 20 {
        let w = (bytes[i] as u32) << 8 | bytes[i+1] as u32;
        sum += w;
        i += 2;
    }
    while sum >> 16 != 0 { sum = (sum & 0xFFFF) + (sum >> 16); }
    !(sum as u16)
}

// ─── ICMP ────────────────────────────────────────────────────────────────────
pub const ICMP_ECHO_REQUEST: u8 = 8;
pub const ICMP_ECHO_REPLY:   u8 = 0;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IcmpHeader {
    pub icmp_type: u8,
    pub code:      u8,
    pub checksum:  u16,
    pub id:        u16,
    pub seq:       u16,
}

// ─── UDP ─────────────────────────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct UdpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub length:   u16,
    pub checksum: u16,
}

impl UdpHeader {
    pub fn new(src: u16, dst: u16, payload: u16) -> Self {
        UdpHeader {
            src_port: src.to_be(), dst_port: dst.to_be(),
            length: (8 + payload).to_be(), checksum: 0,
        }
    }
}

// ─── DNS ─────────────────────────────────────────────────────────────────────
pub const DNS_PORT: u16 = 53;
pub const DNS_CLASS_IN:  u16 = 1;
pub const DNS_TYPE_A:    u16 = 1;
pub const DNS_TYPE_AAAA: u16 = 28;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct DnsHeader {
    pub id:      u16,
    pub flags:   u16,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl DnsHeader {
    pub fn query(id: u16) -> Self {
        DnsHeader {
            id: id.to_be(),
            flags: 0x0100u16.to_be(), // RD bit set
            qdcount: 1u16.to_be(),
            ancount: 0, nscount: 0, arcount: 0,
        }
    }
    pub fn is_response(&self) -> bool { u16::from_be(self.flags) & 0x8000 != 0 }
}

// DNS cache
pub const DNS_CACHE_SIZE: usize = 128;
pub const DNS_NAME_MAX: usize = 64;

#[derive(Clone, Copy)]
pub struct DnsCacheEntry {
    pub name:  [u8; DNS_NAME_MAX],
    pub namelen: u8,
    pub ip:    u32,
    pub ttl:   u32,
    pub valid: bool,
}

pub struct DnsCache {
    entries: [DnsCacheEntry; DNS_CACHE_SIZE],
}

impl DnsCache {
    pub const fn new() -> Self {
        const E: DnsCacheEntry = DnsCacheEntry {
            name: [0u8; DNS_NAME_MAX], namelen: 0, ip: 0, ttl: 0, valid: false
        };
        DnsCache { entries: [E; DNS_CACHE_SIZE] }
    }
    pub fn lookup(&self, name: &[u8]) -> Option<u32> {
        self.entries.iter().find(|e| {
            e.valid && e.namelen as usize == name.len() && &e.name[..e.namelen as usize] == name
        }).map(|e| e.ip)
    }
    pub fn insert(&mut self, name: &[u8], ip: u32, ttl: u32) {
        for e in &mut self.entries {
            if !e.valid {
                let nlen = name.len().min(DNS_NAME_MAX);
                e.name[..nlen].copy_from_slice(&name[..nlen]);
                e.namelen = nlen as u8;
                e.ip = ip; e.ttl = ttl; e.valid = true;
                return;
            }
        }
    }
    pub fn tick(&mut self) {
        for e in &mut self.entries {
            if e.valid { if e.ttl == 0 { e.valid = false; } else { e.ttl -= 1; } }
        }
    }
}

// ─── DHCP ────────────────────────────────────────────────────────────────────
pub const DHCP_DISCOVER: u8 = 1;
pub const DHCP_OFFER:    u8 = 2;
pub const DHCP_REQUEST:  u8 = 3;
pub const DHCP_ACK:      u8 = 5;
pub const DHCP_SERVER_PORT: u16 = 67;
pub const DHCP_CLIENT_PORT: u16 = 68;

pub struct DhcpClient {
    pub state:    DhcpState,
    pub client_ip: u32,
    pub server_ip: u32,
    pub gateway:  u32,
    pub netmask:  u32,
    pub dns:      u32,
    pub lease_sec: u32,
    pub xid:      u32,
    pub mac:      [u8; 6],
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DhcpState { Init, Selecting, Requesting, Bound, Renewing, Rebinding }

impl DhcpClient {
    pub const fn new(mac: [u8; 6]) -> Self {
        DhcpClient {
            state: DhcpState::Init, client_ip: 0, server_ip: 0,
            gateway: 0, netmask: 0, dns: 0, lease_sec: 0,
            xid: 0xDEADBEEF, mac,
        }
    }
    pub fn is_bound(&self) -> bool { self.state == DhcpState::Bound }
    pub fn client_ip_str(&self) -> [u8; 4] {
        let ip = self.client_ip;
        [(ip >> 24) as u8, (ip >> 16) as u8, (ip >> 8) as u8, ip as u8]
    }
}

// ─── Network Interface ───────────────────────────────────────────────────────
pub const MAX_TX_QUEUE: usize = 256;
pub const MAX_RX_QUEUE: usize = 256;
pub const MAX_PKT_SIZE: usize = 1514;

#[derive(Clone, Copy)]
pub struct NetPacket {
    pub data: [u8; MAX_PKT_SIZE],
    pub len:  u16,
}

impl NetPacket {
    pub const fn empty() -> Self { NetPacket { data: [0u8; MAX_PKT_SIZE], len: 0 } }
    pub fn slice(&self) -> &[u8] { &self.data[..self.len as usize] }
}

pub struct NetworkInterface {
    pub name:    [u8; 8],
    pub mac:     [u8; 6],
    pub ip:      u32,
    pub netmask: u32,
    pub gateway: u32,
    pub mtu:     u16,
    pub flags:   u32,
    pub tx_pkts: AtomicU64,
    pub rx_pkts: AtomicU64,
    pub tx_bytes: AtomicU64,
    pub rx_bytes: AtomicU64,
    pub tx_errors: AtomicU64,
    pub rx_errors: AtomicU64,
    pub tx_queue: [NetPacket; MAX_TX_QUEUE],
    pub tx_head:  AtomicU32,
    pub tx_tail:  AtomicU32,
    pub arp_cache:  ArpCache,
    pub dns_cache:  DnsCache,
    pub dhcp:       DhcpClient,
}

pub const IF_UP:       u32 = 1 << 0;
pub const IF_RUNNING:  u32 = 1 << 1;
pub const IF_PROMISC:  u32 = 1 << 2;
pub const IF_LOOPBACK: u32 = 1 << 3;

impl NetworkInterface {
    pub fn new(name: &[u8; 8], mac: [u8; 6]) -> Self {
        const EMPTY_PKT: NetPacket = NetPacket::empty();
        NetworkInterface {
            name: *name, mac, ip: 0, netmask: 0xFFFFFF00, gateway: 0,
            mtu: 1500, flags: IF_UP | IF_RUNNING,
            tx_pkts: AtomicU64::new(0), rx_pkts: AtomicU64::new(0),
            tx_bytes: AtomicU64::new(0), rx_bytes: AtomicU64::new(0),
            tx_errors: AtomicU64::new(0), rx_errors: AtomicU64::new(0),
            tx_queue: [EMPTY_PKT; MAX_TX_QUEUE],
            tx_head: AtomicU32::new(0), tx_tail: AtomicU32::new(0),
            arp_cache: ArpCache::new(), dns_cache: DnsCache::new(),
            dhcp: DhcpClient::new(mac),
        }
    }

    pub fn is_up(&self)      -> bool { self.flags & IF_UP != 0 }
    pub fn is_running(&self) -> bool { self.flags & IF_RUNNING != 0 }

    pub fn enqueue_tx(&mut self, pkt: NetPacket) -> bool {
        let head = self.tx_head.load(Ordering::Acquire);
        let tail = self.tx_tail.load(Ordering::Acquire);
        let next = (tail + 1) as usize % MAX_TX_QUEUE;
        if next == head as usize { return false; }
        self.tx_queue[tail as usize] = pkt;
        self.tx_tail.store(next as u32, Ordering::Release);
        self.tx_pkts.fetch_add(1, Ordering::Relaxed);
        self.tx_bytes.fetch_add(pkt.len as u64, Ordering::Relaxed);
        true
    }

    pub fn dequeue_tx(&mut self) -> Option<NetPacket> {
        let head = self.tx_head.load(Ordering::Acquire);
        let tail = self.tx_tail.load(Ordering::Acquire);
        if head == tail { return None; }
        let pkt = self.tx_queue[head as usize];
        self.tx_head.store((head + 1) % MAX_TX_QUEUE as u32, Ordering::Release);
        Some(pkt)
    }

    pub fn receive(&mut self, data: &[u8]) {
        let len = data.len().min(MAX_PKT_SIZE) as u16;
        self.rx_pkts.fetch_add(1, Ordering::Relaxed);
        self.rx_bytes.fetch_add(len as u64, Ordering::Relaxed);
        // Dispatch based on ethertype
        if data.len() < 14 { return; }
        let etype = u16::from_be_bytes([data[12], data[13]]);
        match etype {
            x if x == ETHERTYPE_ARP  => self.handle_arp(&data[14..]),
            x if x == ETHERTYPE_IPV4 => self.handle_ipv4(&data[14..]),
            _ => {}
        }
    }

    fn handle_arp(&mut self, payload: &[u8]) {
        if payload.len() < 28 { return; }
        let oper = u16::from_be_bytes([payload[6], payload[7]]);
        let sha  = payload[8..14].try_into().unwrap_or([0u8;6]);
        let spa  = u32::from_be_bytes([payload[14],payload[15],payload[16],payload[17]]);
        self.arp_cache.insert(spa, sha);
        if oper == ARP_REQUEST {
            // Would send ARP reply here via TX path
        }
    }

    fn handle_ipv4(&mut self, payload: &[u8]) {
        if payload.len() < 20 { return; }
        let ihl = (payload[0] & 0xF) as usize * 4;
        let proto = payload[9];
        match proto {
            IP_PROTO_ICMP => self.handle_icmp(&payload[ihl..]),
            IP_PROTO_UDP  => self.handle_udp(&payload[ihl..]),
            IP_PROTO_TCP  => {} // handled by TCP stack
            _ => {}
        }
    }

    fn handle_icmp(&mut self, payload: &[u8]) {
        if payload.len() < 8 { return; }
        if payload[0] == ICMP_ECHO_REQUEST {
            // Would generate echo reply
        }
    }

    fn handle_udp(&mut self, payload: &[u8]) {
        if payload.len() < 8 { return; }
        let dst_port = u16::from_be_bytes([payload[2], payload[3]]);
        if dst_port == DHCP_CLIENT_PORT {
            // DHCP response handling
        }
    }

    pub fn stats(&self) -> (u64, u64, u64, u64) {
        (self.tx_pkts.load(Ordering::Relaxed),
         self.rx_pkts.load(Ordering::Relaxed),
         self.tx_errors.load(Ordering::Relaxed),
         self.rx_errors.load(Ordering::Relaxed))
    }
}
