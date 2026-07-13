// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// net/sigma_ipv6.rs — IPv6 Network Stack
//
// Implements RFC 8200 (IPv6) header parsing/building, NDP Neighbor Cache,
// and ICMPv6 echo handling. Integrated with the existing sigma_net.rs IPv4
// stack for a dual-stack (IPv4+IPv6) network layer.
//
// Architecture (OOP):
//   • Ipv6Addr:          128-bit address wrapper with helper methods
//   • Ipv6Header:        Fixed 40-byte header (repr C, wire-compatible)
//   • Ipv6Packet:        Header + payload view with parse/build/validate
//   • NdpNeighborEntry:  Single NDP cache row (IP → MAC)
//   • NdpNeighborCache:  Fixed-size NDP table with LRU eviction
//   • Icmpv6Handler:     ICMPv6 echo request/reply processor
//   • Ipv6Stack:         Top-level encapsulation; dispatch by Next Header
//
// no_std, no alloc.

#![no_std]
#![allow(dead_code)]

use core::convert::TryInto;

// ─────────────────────────────────────────────────────────────────────────────
// Constants
// ─────────────────────────────────────────────────────────────────────────────

pub const IPV6_HEADER_LEN:    usize = 40;
pub const IPV6_MAX_PAYLOAD:   usize = 1500;   // practical MTU payload
pub const NDP_CACHE_SIZE:     usize = 64;
pub const NDP_ENTRY_TTL_SEC:  u64   = 300;    // 5 min reachability

// IPv6 Next Header values (RFC 2460 / IANA)
pub const NH_HOPBYHOP:  u8 = 0;
pub const NH_ICMPV6:    u8 = 58;
pub const NH_TCP:       u8 = 6;
pub const NH_UDP:       u8 = 17;
pub const NH_NO_NEXT:   u8 = 59;
pub const NH_DEST_OPT:  u8 = 60;

// ICMPv6 types (RFC 4443)
pub const ICMPV6_ECHO_REQUEST:  u8 = 128;
pub const ICMPV6_ECHO_REPLY:    u8 = 129;
pub const ICMPV6_ROUTER_SOL:    u8 = 133;
pub const ICMPV6_ROUTER_ADV:    u8 = 134;
pub const ICMPV6_NEIGHBOR_SOL:  u8 = 135;
pub const ICMPV6_NEIGHBOR_ADV:  u8 = 136;

// ─────────────────────────────────────────────────────────────────────────────
// IPv6 Address (128-bit)
// ─────────────────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Ipv6Addr {
    pub octets: [u8; 16],
}

impl Ipv6Addr {
    pub const fn new(o: [u8; 16]) -> Self { Self { octets: o } }

    pub const fn loopback() -> Self {
        Self { octets: [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,1] }
    }

    pub const fn link_local_all_nodes() -> Self {
        Self { octets: [0xff,0x02, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,1] }
    }

    pub const fn unspecified() -> Self {
        Self { octets: [0u8; 16] }
    }

    /// True if address is link-local (fe80::/10)
    pub fn is_link_local(&self) -> bool {
        self.octets[0] == 0xfe && (self.octets[1] & 0xc0) == 0x80
    }

    /// True if address is multicast (ff00::/8)
    pub fn is_multicast(&self) -> bool {
        self.octets[0] == 0xff
    }

    /// True if address is loopback (::1)
    pub fn is_loopback(&self) -> bool {
        *self == Self::loopback()
    }

    /// Solicited-node multicast: ff02::1:ff<last-3-octets>
    pub fn solicited_node_multicast(&self) -> Self {
        let mut m = [0u8; 16];
        m[0]  = 0xff; m[1]  = 0x02;
        m[11] = 0x01; m[12] = 0xff;
        m[13] = self.octets[13];
        m[14] = self.octets[14];
        m[15] = self.octets[15];
        Self { octets: m }
    }

    /// Read from big-endian bytes at a given offset in a buffer.
    pub fn from_bytes(buf: &[u8], offset: usize) -> Option<Self> {
        if offset + 16 > buf.len() { return None; }
        let mut o = [0u8; 16];
        o.copy_from_slice(&buf[offset..offset+16]);
        Some(Self { octets: o })
    }

    /// Write to buffer at offset.
    pub fn write_to(&self, buf: &mut [u8], offset: usize) -> bool {
        if offset + 16 > buf.len() { return false; }
        buf[offset..offset+16].copy_from_slice(&self.octets);
        true
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// IPv6 Fixed Header (RFC 8200 §3, 40 bytes)
// ─────────────────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ipv6Header {
    /// Version(4b) + Traffic Class(8b) + Flow Label(20b) packed into u32 BE
    pub vtcfl:       u32,
    pub payload_len: u16,   // length of payload (not including this header)
    pub next_header: u8,
    pub hop_limit:   u8,
    pub src:         Ipv6Addr,
    pub dst:         Ipv6Addr,
}

impl Ipv6Header {
    pub const fn new(src: Ipv6Addr, dst: Ipv6Addr, next: u8, hop: u8) -> Self {
        Self {
            vtcfl:       0x6000_0000u32.to_be(), // version=6, TC=0, FL=0
            payload_len: 0,
            next_header: next,
            hop_limit:   hop,
            src,
            dst,
        }
    }

    pub fn version(&self) -> u8 {
        ((u32::from_be(self.vtcfl) >> 28) & 0xf) as u8
    }

    pub fn traffic_class(&self) -> u8 {
        ((u32::from_be(self.vtcfl) >> 20) & 0xff) as u8
    }

    pub fn flow_label(&self) -> u32 {
        u32::from_be(self.vtcfl) & 0x000f_ffff
    }

    /// Parse from raw bytes. Returns None if buffer too short or version != 6.
    pub fn parse(buf: &[u8]) -> Option<Self> {
        if buf.len() < IPV6_HEADER_LEN { return None; }
        let vtcfl = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        if (vtcfl >> 28) != 6 { return None; }  // not IPv6
        let payload_len = u16::from_be_bytes([buf[4], buf[5]]);
        let next_header = buf[6];
        let hop_limit   = buf[7];
        let src = Ipv6Addr::from_bytes(buf, 8)?;
        let dst = Ipv6Addr::from_bytes(buf, 24)?;
        Some(Self {
            vtcfl: vtcfl.to_be(),
            payload_len,
            next_header,
            hop_limit,
            src,
            dst,
        })
    }

    /// Serialize into buffer. Returns false if buffer too small.
    pub fn write_to(&self, buf: &mut [u8]) -> bool {
        if buf.len() < IPV6_HEADER_LEN { return false; }
        let vtcfl_be = u32::from_be(self.vtcfl).to_be_bytes();
        buf[0..4].copy_from_slice(&vtcfl_be);
        buf[4..6].copy_from_slice(&self.payload_len.to_be_bytes());
        buf[6] = self.next_header;
        buf[7] = self.hop_limit;
        self.src.write_to(buf, 8);
        self.dst.write_to(buf, 24);
        true
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// IPv6 Packet View (header + payload slice references)
// ─────────────────────────────────────────────────────────────────────────────

pub struct Ipv6Packet<'a> {
    pub header:  Ipv6Header,
    pub payload: &'a [u8],
}

impl<'a> Ipv6Packet<'a> {
    /// Parse from raw ethernet payload.
    pub fn parse(buf: &'a [u8]) -> Option<Self> {
        let header = Ipv6Header::parse(buf)?;
        let plen   = u16::from_be(header.payload_len) as usize;
        if IPV6_HEADER_LEN + plen > buf.len() { return None; }
        Some(Self { header, payload: &buf[IPV6_HEADER_LEN..IPV6_HEADER_LEN + plen] })
    }

    /// Compute ICMPv6 pseudo-header checksum (RFC 4443 §2.3)
    pub fn icmpv6_checksum(&self) -> u16 {
        let mut sum: u32 = 0;

        // Pseudo-header: src, dst, upper-layer length (u32), zero(u8), NH(u8)
        for chunk in self.header.src.octets.chunks(2) {
            sum += u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
        }
        for chunk in self.header.dst.octets.chunks(2) {
            sum += u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
        }
        let upper_len = self.payload.len() as u32;
        sum += (upper_len >> 16) as u16 as u32;
        sum += (upper_len & 0xffff) as u16 as u32;
        sum += self.header.next_header as u32;

        // ICMPv6 body
        let mut i = 0;
        while i + 1 < self.payload.len() {
            sum += u16::from_be_bytes([self.payload[i], self.payload[i+1]]) as u32;
            i += 2;
        }
        if i < self.payload.len() {
            sum += (self.payload[i] as u32) << 8;
        }

        // Fold carry bits
        while sum >> 16 != 0 { sum = (sum & 0xffff) + (sum >> 16); }
        !(sum as u16)
    }

    /// Validate ICMPv6 checksum. Returns true if valid.
    pub fn validate_icmpv6_checksum(&self) -> bool {
        self.icmpv6_checksum() == 0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// NDP Neighbor Cache (Neighbor Discovery Protocol, RFC 4861)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq)]
pub enum NdpEntryState {
    Empty,
    Incomplete,
    Reachable,
    Stale,
    Delay,
    Probe,
}

#[derive(Copy, Clone)]
pub struct NdpNeighborEntry {
    pub ip:        Ipv6Addr,
    pub mac:       [u8; 6],
    pub state:     NdpEntryState,
    pub timestamp: u64,  // seconds since boot (updated on use)
    pub age:       u64,  // seconds since entry created/refreshed
}

impl NdpNeighborEntry {
    pub const fn empty() -> Self {
        Self {
            ip:        Ipv6Addr::unspecified(),
            mac:       [0u8; 6],
            state:     NdpEntryState::Empty,
            timestamp: 0,
            age:       0,
        }
    }
}

pub struct NdpNeighborCache {
    entries: [NdpNeighborEntry; NDP_CACHE_SIZE],
    count:   usize,
    clock:   u64,  // current time in seconds (updated externally)
}

impl NdpNeighborCache {
    pub const fn new() -> Self {
        Self {
            entries: [NdpNeighborEntry::empty(); NDP_CACHE_SIZE],
            count:   0,
            clock:   0,
        }
    }

    /// Update internal clock (called from timer tick).
    pub fn tick(&mut self, now_sec: u64) {
        self.clock = now_sec;
        // Age entries; mark stale after TTL
        for i in 0..NDP_CACHE_SIZE {
            if self.entries[i].state == NdpEntryState::Reachable {
                let age = now_sec.saturating_sub(self.entries[i].timestamp);
                if age > NDP_ENTRY_TTL_SEC {
                    self.entries[i].state = NdpEntryState::Stale;
                }
            }
        }
    }

    /// Look up MAC for an IPv6 address. Returns None if not cached.
    pub fn lookup(&self, ip: &Ipv6Addr) -> Option<[u8; 6]> {
        for i in 0..NDP_CACHE_SIZE {
            let e = &self.entries[i];
            if e.state != NdpEntryState::Empty && &e.ip == ip {
                return Some(e.mac);
            }
        }
        None
    }

    /// Insert or update an NDP entry (IP → MAC).
    pub fn insert(&mut self, ip: Ipv6Addr, mac: [u8; 6]) {
        // Check if entry already exists; update in-place
        for i in 0..NDP_CACHE_SIZE {
            if self.entries[i].ip == ip && self.entries[i].state != NdpEntryState::Empty {
                self.entries[i].mac       = mac;
                self.entries[i].state     = NdpEntryState::Reachable;
                self.entries[i].timestamp = self.clock;
                return;
            }
        }

        // Find empty slot
        for i in 0..NDP_CACHE_SIZE {
            if self.entries[i].state == NdpEntryState::Empty {
                self.entries[i] = NdpNeighborEntry {
                    ip, mac,
                    state:     NdpEntryState::Reachable,
                    timestamp: self.clock,
                    age:       0,
                };
                if self.count < NDP_CACHE_SIZE { self.count += 1; }
                return;
            }
        }

        // Cache full: evict oldest Stale entry (LRU-lite)
        let mut oldest_idx = 0;
        let mut oldest_ts  = u64::MAX;
        for i in 0..NDP_CACHE_SIZE {
            if self.entries[i].state == NdpEntryState::Stale
                && self.entries[i].timestamp < oldest_ts
            {
                oldest_ts  = self.entries[i].timestamp;
                oldest_idx = i;
            }
        }
        self.entries[oldest_idx] = NdpNeighborEntry {
            ip, mac,
            state:     NdpEntryState::Reachable,
            timestamp: self.clock,
            age:       0,
        };
    }

    /// Invalidate a cache entry.
    pub fn remove(&mut self, ip: &Ipv6Addr) {
        for i in 0..NDP_CACHE_SIZE {
            if &self.entries[i].ip == ip {
                self.entries[i] = NdpNeighborEntry::empty();
                if self.count > 0 { self.count -= 1; }
                break;
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ICMPv6 Handler (Echo Request/Reply + Neighbor Solicitation/Advertisement)
// ─────────────────────────────────────────────────────────────────────────────

pub struct Icmpv6Handler;

impl Icmpv6Handler {
    /// Process an incoming ICMPv6 packet.
    /// `out_buf` receives the reply (if any). Returns reply length or 0.
    pub fn process(
        pkt:     &Ipv6Packet,
        out_buf: &mut [u8; IPV6_HEADER_LEN + 64],
        our_ip:  &Ipv6Addr,
    ) -> usize {
        if pkt.payload.is_empty() { return 0; }
        let icmp_type = pkt.payload[0];

        match icmp_type {
            ICMPV6_ECHO_REQUEST => Self::handle_echo_request(pkt, out_buf, our_ip),
            ICMPV6_NEIGHBOR_SOL => Self::handle_neighbor_sol(pkt, out_buf, our_ip),
            _ => 0,
        }
    }

    fn handle_echo_request(
        pkt:     &Ipv6Packet,
        out_buf: &mut [u8; IPV6_HEADER_LEN + 64],
        our_ip:  &Ipv6Addr,
    ) -> usize {
        if pkt.payload.len() < 8 { return 0; }

        // Build reply header: swap src/dst
        let mut reply_hdr = pkt.header;
        reply_hdr.src = *our_ip;
        reply_hdr.dst = pkt.header.src;

        let body_len = pkt.payload.len().min(64);
        reply_hdr.payload_len = (body_len as u16).to_be();

        if !reply_hdr.write_to(&mut out_buf[..IPV6_HEADER_LEN]) { return 0; }

        // Copy ICMPv6 body; change type to Echo Reply
        out_buf[IPV6_HEADER_LEN] = ICMPV6_ECHO_REPLY;
        out_buf[IPV6_HEADER_LEN + 1] = 0; // code = 0
        out_buf[IPV6_HEADER_LEN + 2] = 0; // checksum placeholder
        out_buf[IPV6_HEADER_LEN + 3] = 0;
        let copy_len = (body_len - 4).min(64 - 4);
        for i in 0..copy_len {
            out_buf[IPV6_HEADER_LEN + 4 + i] = pkt.payload[4 + i];
        }
        IPV6_HEADER_LEN + body_len
    }

    fn handle_neighbor_sol(
        pkt:     &Ipv6Packet,
        out_buf: &mut [u8; IPV6_HEADER_LEN + 64],
        our_ip:  &Ipv6Addr,
    ) -> usize {
        if pkt.payload.len() < 24 { return 0; }
        // Target address starts at offset 8 (after type/code/cksum/reserved)
        let target = Ipv6Addr::from_bytes(pkt.payload, 8);
        if target.as_ref() != Some(our_ip) { return 0; }

        // Build Neighbor Advertisement reply
        let mut hdr = pkt.header;
        hdr.src = *our_ip;
        hdr.dst = pkt.header.src;
        hdr.next_header = NH_ICMPV6;
        hdr.payload_len = 32u16.to_be(); // 8 header + 16 target + 8 option

        if !hdr.write_to(&mut out_buf[..IPV6_HEADER_LEN]) { return 0; }

        let b = &mut out_buf[IPV6_HEADER_LEN..];
        b[0] = ICMPV6_NEIGHBOR_ADV;
        b[1] = 0;          // code
        b[2] = 0; b[3] = 0; // checksum (to be filled by caller)
        b[4] = 0x60;        // S=1 (solicited), O=1 (override)
        b[5] = 0; b[6] = 0; b[7] = 0;
        our_ip.write_to(&mut b[8..], 0);
        // TODO: add Target Link-Layer Address option (type=2, len=1, MAC)

        IPV6_HEADER_LEN + 24
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Dual-Stack IPv6 dispatcher
// ─────────────────────────────────────────────────────────────────────────────

pub struct Ipv6Stack {
    pub our_addr:  Ipv6Addr,
    pub ndp_cache: NdpNeighborCache,
}

impl Ipv6Stack {
    pub const fn new(our_addr: Ipv6Addr) -> Self {
        Self {
            our_addr,
            ndp_cache: NdpNeighborCache::new(),
        }
    }

    /// Dispatch an incoming raw IPv6 frame.
    /// Returns number of bytes written to out_buf (0 = no reply).
    pub fn rx_frame(
        &mut self,
        frame:   &[u8],
        out_buf: &mut [u8; IPV6_HEADER_LEN + 64],
    ) -> usize {
        let pkt = match Ipv6Packet::parse(frame) {
            Some(p) => p,
            None    => return 0,
        };

        // Drop if not addressed to us or multicast-for-us
        let is_for_us = pkt.header.dst == self.our_addr
            || pkt.header.dst == self.our_addr.solicited_node_multicast()
            || pkt.header.dst == Ipv6Addr::link_local_all_nodes();
        if !is_for_us { return 0; }

        match pkt.header.next_header {
            NH_ICMPV6 => Icmpv6Handler::process(&pkt, out_buf, &self.our_addr),
            NH_TCP    => 0, // Production: hand off to sigma_tcp
            NH_UDP    => 0, // Production: hand off to sigma_udp
            _         => 0,
        }
    }

    /// Update NDP cache from a timer tick.
    pub fn tick(&mut self, now_sec: u64) {
        self.ndp_cache.tick(now_sec);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Global singleton + C ABI
// ─────────────────────────────────────────────────────────────────────────────

static mut IPV6_STACK: Ipv6Stack = Ipv6Stack::new(Ipv6Addr::unspecified());

#[no_mangle]
pub unsafe extern "C" fn sigma_ipv6_init(our_addr: *const u8) {
    if our_addr.is_null() { return; }
    let mut o = [0u8; 16];
    for i in 0..16 { o[i] = *our_addr.add(i); }
    IPV6_STACK.our_addr = Ipv6Addr::new(o);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ipv6_rx(
    frame:    *const u8,
    frame_len: usize,
    out_buf:  *mut u8,
) -> usize {
    if frame.is_null() || out_buf.is_null() { return 0; }
    let frame_slice = core::slice::from_raw_parts(frame, frame_len);
    let mut buf = [0u8; IPV6_HEADER_LEN + 64];
    let n = IPV6_STACK.rx_frame(frame_slice, &mut buf);
    for i in 0..n { *out_buf.add(i) = buf[i]; }
    n
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ipv6_ndp_insert(ip: *const u8, mac: *const u8) {
    if ip.is_null() || mac.is_null() { return; }
    let mut o = [0u8; 16]; for i in 0..16 { o[i] = *ip.add(i); }
    let mut m = [0u8; 6];  for i in 0..6  { m[i] = *mac.add(i); }
    IPV6_STACK.ndp_cache.insert(Ipv6Addr::new(o), m);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ipv6_tick(now_sec: u64) {
    IPV6_STACK.tick(now_sec);
}
