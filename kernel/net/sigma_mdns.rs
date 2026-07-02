// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/net/sigma_mdns.rs — mDNS/DNS-SD (no_std, cleanroom)
// Language: Rust #![no_std] — OOP via MdnsResponder struct

#![no_std]

pub const MDNS_PORT:      u16  = 5353;
pub const MDNS_ADDR:      [u8;4] = [224,0,0,251]; // 224.0.0.251
pub const MAX_RECORDS:    usize = 32;
pub const MAX_NAME:       usize = 64;
pub const DNS_TYPE_A:     u16  = 1;
pub const DNS_TYPE_PTR:   u16  = 12;
pub const DNS_TYPE_TXT:   u16  = 16;
pub const DNS_TYPE_SRV:   u16  = 33;
pub const DNS_CLASS_IN:   u16  = 1;
pub const MDNS_CACHE_FLUSH: u16 = 0x8000;

// ── mDNS Record ───────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct MdnsRecord {
    pub name:     [u8; MAX_NAME],
    pub name_len: usize,
    pub rr_type:  u16,
    pub ttl:      u32,
    pub data:     [u8; 64],
    pub data_len: usize,
}

impl MdnsRecord {
    pub const fn empty() -> Self {
        Self { name: [0u8;MAX_NAME], name_len:0, rr_type:0, ttl:120,
               data: [0u8;64], data_len:0 }
    }
    pub fn set_name(&mut self, n: &[u8]) {
        let l = n.len().min(MAX_NAME);
        self.name[..l].copy_from_slice(&n[..l]); self.name_len = l;
    }
    pub fn set_ipv4(&mut self, ip: [u8;4]) {
        self.rr_type = DNS_TYPE_A;
        self.data[..4].copy_from_slice(&ip);
        self.data_len = 4;
    }
}

// ── mDNS Packet Builder ───────────────────────────────────────────────────────
pub fn encode_name(name: &[u8], buf: &mut [u8], off: usize) -> usize {
    let mut pos = off;
    let mut rem = name;
    while !rem.is_empty() {
        let dot = rem.iter().position(|&b| b == b'.').unwrap_or(rem.len());
        let label = &rem[..dot];
        if pos + 1 + label.len() >= buf.len() { break; }
        buf[pos] = label.len() as u8; pos += 1;
        buf[pos..pos+label.len()].copy_from_slice(label); pos += label.len();
        if dot < rem.len() { rem = &rem[dot+1..]; } else { break; }
    }
    if pos < buf.len() { buf[pos] = 0; pos += 1; }
    pos
}

/// Build an mDNS announcement packet for a single A record
pub fn build_announce(hostname: &[u8], ip: [u8;4], buf: &mut [u8;512]) -> usize {
    // DNS header: QR=1 (response), AA=1 (authoritative), ANCOUNT=1
    buf[0] = 0; buf[1] = 0; // ID = 0 (mDNS)
    buf[2] = 0x84; buf[3] = 0x00; // QR=1, AA=1
    buf[4] = 0; buf[5] = 0; // QDCOUNT=0
    buf[6] = 0; buf[7] = 1; // ANCOUNT=1
    buf[8] = 0; buf[9] = 0; // NSCOUNT=0
    buf[10]= 0; buf[11]= 0; // ARCOUNT=0

    let mut off = 12;
    off = encode_name(hostname, buf, off);
    // TYPE A
    buf[off]   = 0; buf[off+1] = DNS_TYPE_A as u8; off += 2;
    // CLASS IN + cache-flush
    let cls = DNS_CLASS_IN | MDNS_CACHE_FLUSH;
    buf[off]   = (cls >> 8) as u8; buf[off+1] = (cls & 0xFF) as u8; off += 2;
    // TTL
    buf[off..off+4].copy_from_slice(&120u32.to_be_bytes()); off += 4;
    // RDLENGTH = 4
    buf[off] = 0; buf[off+1] = 4; off += 2;
    // RDATA = IPv4
    buf[off..off+4].copy_from_slice(&ip); off += 4;
    off
}

/// Parse question from mDNS query, return queried name length
pub fn parse_query_name<'a>(pkt: &'a [u8], off: usize, out: &mut [u8;MAX_NAME]) -> (usize, usize) {
    let mut pos = off; let mut len = 0;
    loop {
        if pos >= pkt.len() { break; }
        let label_len = pkt[pos] as usize; pos += 1;
        if label_len == 0 { break; }
        if label_len & 0xC0 == 0xC0 { pos += 1; break; } // pointer
        if len + label_len + 1 < MAX_NAME {
            if len > 0 { out[len] = b'.'; len += 1; }
            out[len..len+label_len].copy_from_slice(&pkt[pos..pos+label_len]);
            len += label_len;
        }
        pos += label_len;
    }
    (pos, len)
}

// ── mDNS Responder ────────────────────────────────────────────────────────────
pub struct MdnsResponder {
    hostname:   [u8; MAX_NAME],
    host_len:   usize,
    ip:         [u8; 4],
    records:    [MdnsRecord; MAX_RECORDS],
    n_records:  usize,
    buf:        [u8; 512],
}

impl MdnsResponder {
    pub fn new(hostname: &[u8], ip: [u8;4]) -> Self {
        let mut r = Self {
            hostname: [0u8;MAX_NAME], host_len: hostname.len().min(MAX_NAME),
            ip, records: [MdnsRecord::empty(); MAX_RECORDS], n_records: 0,
            buf: [0u8;512],
        };
        r.hostname[..r.host_len].copy_from_slice(&hostname[..r.host_len]);
        r
    }

    pub fn add_record(&mut self, rec: MdnsRecord) -> bool {
        if self.n_records >= MAX_RECORDS { return false; }
        self.records[self.n_records] = rec; self.n_records += 1; true
    }

    /// Process incoming mDNS packet; if it's a query for our hostname, return reply length
    pub fn process(&mut self, pkt: &[u8]) -> Option<usize> {
        if pkt.len() < 12 { return None; }
        let qr = (pkt[2] >> 7) & 1;
        if qr == 1 { return None; } // skip responses
        let qdcount = u16::from_be_bytes([pkt[4], pkt[5]]) as usize;
        let mut off = 12;
        for _ in 0..qdcount {
            let mut name = [0u8; MAX_NAME];
            let (new_off, name_len) = parse_query_name(pkt, off, &mut name);
            off = new_off;
            if off + 4 > pkt.len() { break; }
            let qtype = u16::from_be_bytes([pkt[off], pkt[off+1]]); off += 4;
            // Check if queried name matches our hostname
            if name_len == self.host_len && name[..name_len] == self.hostname[..self.host_len] {
                if qtype == DNS_TYPE_A || qtype == 255 {
                    let len = build_announce(&self.hostname[..self.host_len], self.ip, &mut self.buf);
                    return Some(len);
                }
            }
        }
        None
    }

    pub fn reply_buf(&self) -> &[u8] { &self.buf }
}
