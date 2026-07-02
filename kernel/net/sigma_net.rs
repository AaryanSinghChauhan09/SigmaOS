// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/net/sigma_net.rs — Network Stack (smoltcp-pattern, cleanroom)
// Replaces: SovereignNetStack.cpp (C++ stub, removed)
//
// Architecture: NIC HAL → Ethernet → IPv4/IPv6 → TCP/UDP → sockets
// Language: Rust #![no_std] — no libc, no alloc, no third-party crates
// Pattern: OOP via Traits (NicDevice, Protocol, Socket)

#![no_std]

// ── Constants ────────────────────────────────────────────────────────────────

pub const MTU:           usize = 1514;
pub const MAX_SOCKETS:   usize = 32;
pub const TCP_MSS:       usize = 1460;
pub const ARP_CACHE_SZ:  usize = 16;
pub const RX_RING_SZ:    usize = 32;
pub const TX_RING_SZ:    usize = 32;

// ── Address Types ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MacAddr(pub [u8; 6]);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Addr(pub [u8; 4]);

impl Ipv4Addr {
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Self { Self([a,b,c,d]) }
    pub fn as_u32(&self) -> u32 {
        ((self.0[0] as u32) << 24) | ((self.0[1] as u32) << 16) |
        ((self.0[2] as u32) <<  8) |  (self.0[3] as u32)
    }
    pub fn from_u32(v: u32) -> Self {
        Self([(v>>24) as u8, (v>>16) as u8, (v>>8) as u8, v as u8])
    }
}

// ── NIC Driver Trait (OOP) ────────────────────────────────────────────────────

pub trait NicDevice: Send + Sync {
    fn mac(&self)  -> MacAddr;
    /// Receive one frame into `buf`, returns byte count or 0
    fn recv(&mut self, buf: &mut [u8; MTU]) -> usize;
    /// Transmit `len` bytes from `buf`
    fn send(&mut self, buf: &[u8], len: usize);
    fn link_up(&self) -> bool;
}

// ── Ethernet Frame ────────────────────────────────────────────────────────────

#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EtherType {
    Ipv4 = 0x0800,
    Arp  = 0x0806,
    Ipv6 = 0x86DD,
}

pub struct EthFrame<'a> {
    raw: &'a [u8],
}

impl<'a> EthFrame<'a> {
    pub fn new(raw: &'a [u8]) -> Option<Self> {
        if raw.len() < 14 { None } else { Some(Self { raw }) }
    }
    pub fn dst(&self) -> MacAddr { MacAddr(self.raw[0..6].try_into().unwrap()) }
    pub fn src(&self) -> MacAddr { MacAddr(self.raw[6..12].try_into().unwrap()) }
    pub fn ether_type(&self) -> u16 {
        ((self.raw[12] as u16) << 8) | self.raw[13] as u16
    }
    pub fn payload(&self) -> &[u8] { &self.raw[14..] }
}

// ── ARP Cache ─────────────────────────────────────────────────────────────────

struct ArpEntry { ip: Ipv4Addr, mac: MacAddr, age: u32 }

pub struct ArpCache {
    entries: [Option<ArpEntry>; ARP_CACHE_SZ],
    count:   usize,
}

impl ArpCache {
    pub const fn new() -> Self {
        Self { entries: [const { None }; ARP_CACHE_SZ], count: 0 }
    }

    pub fn lookup(&self, ip: Ipv4Addr) -> Option<MacAddr> {
        for e in self.entries.iter().flatten() {
            if e.ip == ip { return Some(e.mac); }
        }
        None
    }

    pub fn insert(&mut self, ip: Ipv4Addr, mac: MacAddr) {
        for e in self.entries.iter_mut() {
            if let Some(ref mut entry) = e {
                if entry.ip == ip { entry.mac = mac; return; }
            }
        }
        let slot = self.count % ARP_CACHE_SZ;
        self.entries[slot] = Some(ArpEntry { ip, mac, age: 0 });
        self.count += 1;
    }
}

// ── IPv4 Header ───────────────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IpProto { Icmp = 1, Tcp = 6, Udp = 17 }

pub struct Ipv4Header<'a> { raw: &'a [u8] }

impl<'a> Ipv4Header<'a> {
    pub fn new(raw: &'a [u8]) -> Option<Self> {
        if raw.len() < 20 { None } else { Some(Self { raw }) }
    }
    pub fn ihl(&self)      -> usize { ((self.raw[0] & 0xF) * 4) as usize }
    pub fn total_len(&self) -> u16  { u16::from_be_bytes([self.raw[2], self.raw[3]]) }
    pub fn proto(&self)    -> u8    { self.raw[9] }
    pub fn src(&self)      -> Ipv4Addr { Ipv4Addr(self.raw[12..16].try_into().unwrap()) }
    pub fn dst(&self)      -> Ipv4Addr { Ipv4Addr(self.raw[16..20].try_into().unwrap()) }
    pub fn payload(&self)  -> &[u8]    { &self.raw[self.ihl()..] }
    pub fn checksum_ok(&self) -> bool  { Self::checksum(self.raw) == 0 }

    fn checksum(data: &[u8]) -> u16 {
        let mut sum: u32 = 0;
        let mut i = 0;
        while i + 1 < data.len() {
            sum += u16::from_be_bytes([data[i], data[i+1]]) as u32;
            i += 2;
        }
        if i < data.len() { sum += (data[i] as u32) << 8; }
        while sum >> 16 != 0 { sum = (sum & 0xFFFF) + (sum >> 16); }
        !(sum as u16)
    }
}

// ── UDP ───────────────────────────────────────────────────────────────────────

pub struct UdpDatagram<'a> { raw: &'a [u8] }

impl<'a> UdpDatagram<'a> {
    pub fn new(raw: &'a [u8]) -> Option<Self> {
        if raw.len() < 8 { None } else { Some(Self { raw }) }
    }
    pub fn src_port(&self) -> u16 { u16::from_be_bytes([self.raw[0], self.raw[1]]) }
    pub fn dst_port(&self) -> u16 { u16::from_be_bytes([self.raw[2], self.raw[3]]) }
    pub fn payload(&self)  -> &[u8] { &self.raw[8..] }
}

// ── Socket Trait ─────────────────────────────────────────────────────────────

pub trait Socket: Send + Sync {
    fn local_addr(&self) -> Ipv4Addr;
    fn local_port(&self) -> u16;
    fn recv(&mut self, buf: &mut [u8]) -> usize;
    fn send(&mut self, dst: Ipv4Addr, port: u16, data: &[u8]);
    fn close(&mut self);
}

// ── Network Interface ────────────────────────────────────────────────────────

pub struct NetIface {
    pub ip:      Ipv4Addr,
    pub netmask: Ipv4Addr,
    pub gateway: Ipv4Addr,
    pub mac:     MacAddr,
    arp:         ArpCache,
}

impl NetIface {
    pub fn new(ip: Ipv4Addr, netmask: Ipv4Addr, gateway: Ipv4Addr, mac: MacAddr) -> Self {
        Self { ip, netmask, gateway, mac, arp: ArpCache::new() }
    }

    /// Process one received Ethernet frame
    pub fn process_frame(&mut self, frame: &[u8]) {
        let eth = match EthFrame::new(frame) { Some(f) => f, None => return };
        match eth.ether_type() {
            0x0806 => self.process_arp(eth.payload()),
            0x0800 => self.process_ipv4(eth.payload()),
            _      => {} // drop unknown
        }
    }

    fn process_arp(&mut self, payload: &[u8]) {
        if payload.len() < 28 { return; }
        // ARP reply: opcode = 2
        let opcode = u16::from_be_bytes([payload[6], payload[7]]);
        if opcode == 2 {
            let sender_ip  = Ipv4Addr(payload[14..18].try_into().unwrap_or([0;4]));
            let sender_mac = MacAddr(payload[8..14].try_into().unwrap_or([0;6]));
            self.arp.insert(sender_ip, sender_mac);
        }
    }

    fn process_ipv4(&mut self, payload: &[u8]) {
        let hdr = match Ipv4Header::new(payload) { Some(h) => h, None => return };
        if !hdr.checksum_ok() { return; }
        if hdr.dst() != self.ip { return; }
        match hdr.proto() {
            1  => {} // ICMP — TODO
            6  => {} // TCP  — TODO
            17 => {} // UDP  — TODO
            _  => {}
        }
    }
}
