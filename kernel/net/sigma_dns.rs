// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/net/sigma_dns.rs — DNS/DoH Resolver (no_std, cleanroom)
// Language: Rust #![no_std]
// Pattern: OOP via DnsResolver struct

#![no_std]

pub const MAX_NAME_LEN: usize = 253;
pub const MAX_RECORDS:  usize = 16;
pub const DNS_PORT:     u16   = 53;
pub const DNS_HDR_SIZE: usize = 12;

// ── DNS Header ────────────────────────────────────────────────────────────────

pub struct DnsHeader {
    pub id:      u16,
    pub flags:   u16, // QR|Opcode|AA|TC|RD|RA|Z|RCODE
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl DnsHeader {
    pub fn from_bytes(b: &[u8]) -> Option<Self> {
        if b.len() < DNS_HDR_SIZE { return None; }
        Some(Self {
            id:      u16::from_be_bytes([b[0],  b[1]]),
            flags:   u16::from_be_bytes([b[2],  b[3]]),
            qdcount: u16::from_be_bytes([b[4],  b[5]]),
            ancount: u16::from_be_bytes([b[6],  b[7]]),
            nscount: u16::from_be_bytes([b[8],  b[9]]),
            arcount: u16::from_be_bytes([b[10], b[11]]),
        })
    }

    pub fn to_bytes(&self, out: &mut [u8; DNS_HDR_SIZE]) {
        out[0..2].copy_from_slice(&self.id.to_be_bytes());
        out[2..4].copy_from_slice(&self.flags.to_be_bytes());
        out[4..6].copy_from_slice(&self.qdcount.to_be_bytes());
        out[6..8].copy_from_slice(&self.ancount.to_be_bytes());
        out[8..10].copy_from_slice(&self.nscount.to_be_bytes());
        out[10..12].copy_from_slice(&self.arcount.to_be_bytes());
    }

    pub fn is_response(&self) -> bool { self.flags & 0x8000 != 0 }
    pub fn rcode(&self)      -> u8   { (self.flags & 0x000F) as u8 }
}

// ── DNS Record Types ──────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum RrType { A = 1, NS = 2, CNAME = 5, SOA = 6, MX = 15, TXT = 16, AAAA = 28 }

#[derive(Clone, Copy)]
pub struct DnsRecord {
    pub name: [u8; MAX_NAME_LEN],
    pub name_len: usize,
    pub rr_type: u16,
    pub ttl:     u32,
    pub data:    [u8; 16], // IPv4 (4 bytes) or IPv6 (16 bytes)
    pub data_len: usize,
}

impl DnsRecord {
    pub const fn empty() -> Self {
        Self { name: [0u8; MAX_NAME_LEN], name_len: 0,
               rr_type: 0, ttl: 0, data: [0u8; 16], data_len: 0 }
    }
    pub fn ipv4(&self) -> Option<[u8; 4]> {
        if self.rr_type == RrType::A as u16 && self.data_len == 4 {
            Some(self.data[..4].try_into().unwrap_or([0;4]))
        } else { None }
    }
}

// ── DNS Cache ─────────────────────────────────────────────────────────────────

pub struct DnsCache {
    entries: [Option<DnsRecord>; MAX_RECORDS],
    count:   usize,
    tick:    u32,
}

impl DnsCache {
    pub const fn new() -> Self {
        Self { entries: [const { None }; MAX_RECORDS], count: 0, tick: 0 }
    }

    pub fn lookup(&self, name: &[u8], rr_type: u16) -> Option<&DnsRecord> {
        for e in self.entries.iter().flatten() {
            if e.rr_type == rr_type && &e.name[..e.name_len] == name {
                return Some(e);
            }
        }
        None
    }

    pub fn insert(&mut self, record: DnsRecord) {
        // Replace oldest (LRU approximation: overwrite slot count % MAX)
        let slot = self.count % MAX_RECORDS;
        self.entries[slot] = Some(record);
        self.count += 1;
    }

    pub fn tick(&mut self) { self.tick += 1; }
}

// ── Query Builder ─────────────────────────────────────────────────────────────

/// Build a DNS A-record query packet into `buf`.
/// Returns the packet length or 0 on error.
pub fn build_query(name: &[u8], id: u16, buf: &mut [u8; 512]) -> usize {
    buf.fill(0);
    // Header: standard query, recursion desired
    let hdr = DnsHeader { id, flags: 0x0100, qdcount: 1,
                          ancount: 0, nscount: 0, arcount: 0 };
    let mut h = [0u8; DNS_HDR_SIZE];
    hdr.to_bytes(&mut h);
    buf[..DNS_HDR_SIZE].copy_from_slice(&h);

    // Encode QNAME as labels
    let mut off = DNS_HDR_SIZE;
    let mut label_start = 0;
    for (i, &b) in name.iter().enumerate() {
        if b == b'.' {
            let len = i - label_start;
            if off + len + 1 >= 512 { return 0; }
            buf[off] = len as u8; off += 1;
            buf[off..off+len].copy_from_slice(&name[label_start..i]);
            off += len;
            label_start = i + 1;
        }
    }
    // Last label
    let len = name.len() - label_start;
    if len > 0 && off + len + 1 < 512 {
        buf[off] = len as u8; off += 1;
        buf[off..off+len].copy_from_slice(&name[label_start..]);
        off += len;
    }
    if off + 5 >= 512 { return 0; }
    buf[off] = 0; off += 1;         // root label
    buf[off..off+2].copy_from_slice(&(RrType::A as u16).to_be_bytes()); off += 2;
    buf[off..off+2].copy_from_slice(&1u16.to_be_bytes()); off += 2; // QCLASS=IN
    off
}

// ── Response Parser ───────────────────────────────────────────────────────────

/// Parse DNS response and extract A records into `out`. Returns count.
pub fn parse_response(pkt: &[u8], out: &mut [DnsRecord; MAX_RECORDS]) -> usize {
    let hdr = match DnsHeader::from_bytes(pkt) { Some(h) => h, None => return 0 };
    if !hdr.is_response() || hdr.rcode() != 0 { return 0; }

    let mut off = DNS_HDR_SIZE;
    // Skip questions
    for _ in 0..hdr.qdcount {
        off = skip_name(pkt, off);
        off += 4; // QTYPE + QCLASS
        if off >= pkt.len() { return 0; }
    }
    // Parse answers
    let mut count = 0usize;
    for _ in 0..hdr.ancount {
        if off >= pkt.len() || count >= MAX_RECORDS { break; }
        let mut rec = DnsRecord::empty();
        // Read name (may be pointer)
        let name_end = read_name(pkt, off, &mut rec.name);
        rec.name_len = name_end.1;
        off = name_end.0;
        if off + 10 > pkt.len() { break; }
        rec.rr_type = u16::from_be_bytes([pkt[off], pkt[off+1]]); off += 2;
        let _class = u16::from_be_bytes([pkt[off], pkt[off+1]]); off += 2;
        rec.ttl     = u32::from_be_bytes(pkt[off..off+4].try_into().unwrap_or([0;4])); off += 4;
        let rdlen   = u16::from_be_bytes([pkt[off], pkt[off+1]]) as usize; off += 2;
        if off + rdlen > pkt.len() { break; }
        let n = rdlen.min(16);
        rec.data[..n].copy_from_slice(&pkt[off..off+n]);
        rec.data_len = n;
        off += rdlen;
        out[count] = rec;
        count += 1;
    }
    count
}

fn skip_name(pkt: &[u8], mut off: usize) -> usize {
    loop {
        if off >= pkt.len() { return off; }
        let len = pkt[off] as usize;
        if len == 0 { return off + 1; }
        if len & 0xC0 == 0xC0 { return off + 2; } // pointer
        off += 1 + len;
    }
}

/// Returns (new_offset, name_length_written)
fn read_name(pkt: &[u8], mut off: usize, out: &mut [u8; MAX_NAME_LEN]) -> (usize, usize) {
    let mut name_len = 0;
    let mut jumped   = false;
    let mut ret_off  = 0;
    loop {
        if off >= pkt.len() { break; }
        let len = pkt[off] as usize;
        if len == 0 { if !jumped { ret_off = off + 1; } break; }
        if len & 0xC0 == 0xC0 {
            if !jumped { ret_off = off + 2; jumped = true; }
            let ptr = ((len & 0x3F) << 8 | pkt[off+1] as usize);
            off = ptr;
            continue;
        }
        off += 1;
        if name_len + len < MAX_NAME_LEN {
            if name_len > 0 { out[name_len] = b'.'; name_len += 1; }
            out[name_len..name_len+len].copy_from_slice(&pkt[off..off+len]);
            name_len += len;
        }
        off += len;
    }
    (if jumped { ret_off } else { off }, name_len)
}
