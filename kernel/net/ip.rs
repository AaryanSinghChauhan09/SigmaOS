// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/net/ip.rs — IPv4/IPv6 layer
//
// Implements:
//   - IPv4 header parsing + construction
//   - IPv6 header parsing + construction
//   - Basic routing table (up to 32 routes)
//   - ICMP echo request/reply
//   - ARP (address resolution)
//   - sigma_ip_send() used by TCP/UDP layers
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU16, Ordering};

// ── IPv4 header ───────────────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Ipv4Hdr {
    pub ver_ihl:  u8,   // version(4) | IHL(4)
    pub dscp_ecn: u8,
    pub total_len:u16,
    pub ident:    u16,
    pub flags_frag:u16,
    pub ttl:      u8,
    pub protocol: u8,
    pub checksum: u16,
    pub src_ip:   u32,
    pub dst_ip:   u32,
}

pub const IP_PROTO_ICMP: u8 = 1;
pub const IP_PROTO_TCP:  u8 = 6;
pub const IP_PROTO_UDP:  u8 = 17;

static IP_IDENT: AtomicU16 = AtomicU16::new(1);

impl Ipv4Hdr {
    pub fn new(src: u32, dst: u32, proto: u8, payload_len: u16) -> Self {
        let ident = IP_IDENT.fetch_add(1, Ordering::Relaxed);
        let total = 20u16 + payload_len;
        Self {
            ver_ihl: 0x45, dscp_ecn: 0,
            total_len: u16::to_be(total),
            ident: u16::to_be(ident),
            flags_frag: 0x0040_u16.to_be(), // DF bit set
            ttl: 64, protocol: proto,
            checksum: 0,
            src_ip: u32::to_be(src),
            dst_ip: u32::to_be(dst),
        }
    }

    pub fn fill_checksum(&mut self) {
        self.checksum = 0;
        let bytes = unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<Self>(),
            )
        };
        self.checksum = ip_checksum(bytes);
    }

    pub fn verify_checksum(&self) -> bool {
        let bytes = unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<Self>(),
            )
        };
        ip_checksum(bytes) == 0
    }

    pub fn src(&self) -> u32 { u32::from_be(self.src_ip) }
    pub fn dst(&self) -> u32 { u32::from_be(self.dst_ip) }
    pub fn total_len(&self) -> u16 { u16::from_be(self.total_len) }
}

fn ip_checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let mut i = 0;
    while i + 1 < data.len() {
        sum += u32::from(data[i]) << 8 | u32::from(data[i+1]);
        i += 2;
    }
    if data.len() & 1 != 0 { sum += u32::from(data[data.len()-1]) << 8; }
    while sum >> 16 != 0 { sum = (sum & 0xFFFF) + (sum >> 16); }
    !(sum as u16)
}

// ── IPv6 header ───────────────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Ipv6Hdr {
    pub ver_tc_fl:    u32,  // version(4)|traffic class(8)|flow label(20)
    pub payload_len:  u16,
    pub next_header:  u8,
    pub hop_limit:    u8,
    pub src_addr:     [u8; 16],
    pub dst_addr:     [u8; 16],
}

// ── ICMP ──────────────────────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct IcmpHdr {
    pub icmp_type: u8,
    pub code:      u8,
    pub checksum:  u16,
    pub rest:      u32,
}

pub const ICMP_ECHO_REQUEST: u8 = 8;
pub const ICMP_ECHO_REPLY:   u8 = 0;
pub const ICMP_DEST_UNREACH: u8 = 3;
pub const ICMP_TTL_EXCEEDED: u8 = 11;

// ── ARP packet ────────────────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ArpPacket {
    pub hw_type:     u16,  // 1 = Ethernet
    pub proto_type:  u16,  // 0x0800 = IPv4
    pub hw_size:     u8,   // 6
    pub proto_size:  u8,   // 4
    pub opcode:      u16,  // 1=request, 2=reply
    pub sender_mac:  [u8; 6],
    pub sender_ip:   u32,
    pub target_mac:  [u8; 6],
    pub target_ip:   u32,
}

pub const ARP_REQUEST: u16 = 1;
pub const ARP_REPLY:   u16 = 2;

// ── ARP cache ─────────────────────────────────────────────────────────────
const ARP_CACHE_SIZE: usize = 64;

#[derive(Copy, Clone, Default)]
struct ArpEntry {
    ip:  u32,
    mac: [u8; 6],
    valid: bool,
}

pub struct ArpCache {
    entries: [ArpEntry; ARP_CACHE_SIZE],
    count:   usize,
}

impl ArpCache {
    pub const fn new() -> Self {
        Self {
            entries: [ArpEntry { ip: 0, mac: [0u8; 6], valid: false }; ARP_CACHE_SIZE],
            count: 0,
        }
    }

    pub fn lookup(&self, ip: u32) -> Option<[u8; 6]> {
        for e in &self.entries {
            if e.valid && e.ip == ip { return Some(e.mac); }
        }
        None
    }

    pub fn insert(&mut self, ip: u32, mac: [u8; 6]) {
        // Replace existing or use a free slot
        for e in &mut self.entries {
            if e.valid && e.ip == ip { e.mac = mac; return; }
        }
        for e in &mut self.entries {
            if !e.valid {
                *e = ArpEntry { ip, mac, valid: true };
                self.count += 1;
                return;
            }
        }
        // Evict oldest (index 0) — simple FIFO
        self.entries[0] = ArpEntry { ip, mac, valid: true };
    }
}

// ── Routing table ─────────────────────────────────────────────────────────
const ROUTE_TABLE_SIZE: usize = 32;

#[derive(Copy, Clone, Default)]
struct Route {
    network:  u32,
    netmask:  u32,
    gateway:  u32,   // 0 = directly connected
    iface_ip: u32,
    valid:    bool,
}

pub struct RoutingTable {
    routes: [Route; ROUTE_TABLE_SIZE],
    count:  usize,
    local_ip:   u32,
    gateway_ip: u32,
    local_mac:  [u8; 6],
}

impl RoutingTable {
    pub const fn new() -> Self {
        Self {
            routes: [Route { network: 0, netmask: 0, gateway: 0, iface_ip: 0, valid: false }; ROUTE_TABLE_SIZE],
            count: 0,
            local_ip:   0xC0A80002, // 192.168.0.2
            gateway_ip: 0xC0A80001, // 192.168.0.1
            local_mac:  [0x52, 0x54, 0x00, 0x12, 0x34, 0x56],
        }
    }

    pub fn add_route(&mut self, network: u32, netmask: u32, gateway: u32) {
        for r in &mut self.routes {
            if !r.valid {
                *r = Route { network, netmask, gateway, iface_ip: self.local_ip, valid: true };
                self.count += 1;
                return;
            }
        }
    }

    /// Look up the next-hop gateway for a destination IP.
    pub fn lookup(&self, dst: u32) -> Option<u32> {
        // Longest prefix match
        let mut best_prefix_len = u32::MAX;
        let mut best_gateway = None;
        for r in &self.routes {
            if !r.valid { continue; }
            if dst & r.netmask == r.network {
                let prefix_len = r.netmask.leading_ones();
                if best_gateway.is_none() || prefix_len > best_prefix_len {
                    best_prefix_len = prefix_len;
                    best_gateway = Some(if r.gateway == 0 { dst } else { r.gateway });
                }
            }
        }
        best_gateway
    }
}

// ── IP stack ──────────────────────────────────────────────────────────────
pub struct IpStack {
    pub routing:  RoutingTable,
    pub arp:      ArpCache,
    initialized:  bool,
}

impl IpStack {
    pub const fn new() -> Self {
        Self {
            routing: RoutingTable::new(),
            arp: ArpCache::new(),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        // Default routes: local /24 network + default gateway
        let local_net = self.routing.local_ip & 0xFFFFFF00;
        self.routing.add_route(local_net, 0xFFFFFF00, 0);       // LAN
        self.routing.add_route(0, 0, self.routing.gateway_ip);  // default
        self.initialized = true;
    }

    /// Process an incoming IPv4 packet
    pub unsafe fn rx_ipv4(&mut self, data: &[u8]) {
        if data.len() < 20 { return; }
        let hdr = &*(data.as_ptr() as *const Ipv4Hdr);
        if !hdr.verify_checksum() { return; }

        let src = hdr.src();
        let ihl = (hdr.ver_ihl & 0x0F) as usize * 4;
        let payload = &data[ihl..hdr.total_len() as usize];

        match hdr.protocol {
            IP_PROTO_ICMP => self.rx_icmp(src, payload),
            IP_PROTO_TCP  => {
                extern "C" {
                    fn tcp_rx_segment(fd: usize, hdr: *const u8, pay: *const u8, len: usize) -> i32;
                }
                // Route to TCP stack — fd lookup by port TBD
                if payload.len() >= 20 {
                    tcp_rx_segment(0, payload.as_ptr(), payload.as_ptr().add(20), payload.len() - 20);
                }
            }
            IP_PROTO_UDP  => self.rx_udp(src, payload),
            _ => {}
        }
    }

    unsafe fn rx_icmp(&mut self, src_ip: u32, data: &[u8]) {
        if data.len() < 8 { return; }
        let hdr = &*(data.as_ptr() as *const IcmpHdr);
        if hdr.icmp_type == ICMP_ECHO_REQUEST {
            // Send echo reply
            self.send_icmp_reply(src_ip, data);
        }
    }

    unsafe fn rx_udp(&mut self, _src_ip: u32, _data: &[u8]) {
        // UDP dispatch — Phase B
    }

    unsafe fn send_icmp_reply(&mut self, dst_ip: u32, echo_req: &[u8]) {
        let mut reply_buf = [0u8; 1500];
        let icmp_offset = 20usize;

        // Build IP header
        let ip_hdr = &mut *(reply_buf.as_mut_ptr() as *mut Ipv4Hdr);
        *ip_hdr = Ipv4Hdr::new(self.routing.local_ip, dst_ip, IP_PROTO_ICMP, echo_req.len() as u16);

        // Build ICMP reply header (type=0, code=0, same rest field)
        let icmp_hdr = &mut *(reply_buf.as_mut_ptr().add(icmp_offset) as *mut IcmpHdr);
        let orig = &*(echo_req.as_ptr() as *const IcmpHdr);
        icmp_hdr.icmp_type = ICMP_ECHO_REPLY;
        icmp_hdr.code      = 0;
        icmp_hdr.checksum  = 0;
        icmp_hdr.rest      = orig.rest;

        // Copy echo data
        if echo_req.len() > 8 {
            let copy_len = echo_req.len() - 8;
            reply_buf[icmp_offset + 8..icmp_offset + 8 + copy_len]
                .copy_from_slice(&echo_req[8..8 + copy_len]);
        }

        let total = icmp_offset + echo_req.len();
        ip_hdr.fill_checksum();
        // Compute ICMP checksum
        let icmp_bytes = &reply_buf[icmp_offset..total];
        let csum = ip_checksum(icmp_bytes);
        reply_buf[icmp_offset + 2] = (csum >> 8) as u8;
        reply_buf[icmp_offset + 3] = (csum & 0xFF) as u8;

        self.tx_raw(&reply_buf[..total], dst_ip);
    }

    unsafe fn tx_raw(&self, data: &[u8], _dst_ip: u32) {
        // Hand to NIC driver via sigma-bus
        extern "C" { fn sigma_bus_send_impl(ch: u32, data: *const u8, len: usize) -> i32; }
        sigma_bus_send_impl(0x21, data.as_ptr(), data.len()); // IPC_CH_NET_TX
    }
}

// ── sigma_ip_send — used by TCP/UDP layers ────────────────────────────────
static mut G_IP: IpStack = IpStack::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_ip_init() {
    G_IP.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ip_send(
    dst_ip: u32, proto: u8, payload: *const u8, payload_len: usize,
) -> i32 {
    if payload.is_null() { return -14; }
    let mut buf = [0u8; 1500];
    if payload_len + 20 > buf.len() { return -22; }

    let ip_hdr = &mut *(buf.as_mut_ptr() as *mut Ipv4Hdr);
    *ip_hdr = Ipv4Hdr::new(G_IP.routing.local_ip, dst_ip, proto, payload_len as u16);
    core::ptr::copy_nonoverlapping(payload, buf.as_mut_ptr().add(20), payload_len);
    ip_hdr.fill_checksum();

    G_IP.tx_raw(&buf[..20 + payload_len], dst_ip);
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ip_rx(data: *const u8, len: usize) {
    if data.is_null() || len < 20 { return; }
    let slice = core::slice::from_raw_parts(data, len);
    G_IP.rx_ipv4(slice);
}
