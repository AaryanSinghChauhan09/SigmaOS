// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/net/sigma_icmp.rs — ICMP + ICMPv6 (no_std, cleanroom)
// Language: Rust #![no_std] — OOP via IcmpHandler struct

#![no_std]

// ── ICMP Type/Code ────────────────────────────────────────────────────────────
pub const ICMP_ECHO_REPLY:    u8 = 0;
pub const ICMP_ECHO_REQUEST:  u8 = 8;
pub const ICMP_DEST_UNREACH:  u8 = 3;
pub const ICMP_TIME_EXCEEDED: u8 = 11;

pub const ICMPV6_ECHO_REQUEST: u8 = 128;
pub const ICMPV6_ECHO_REPLY:   u8 = 129;
pub const ICMPV6_NEIGHBOR_SOL: u8 = 135;
pub const ICMPV6_NEIGHBOR_ADV: u8 = 136;

// ── ICMP Header ───────────────────────────────────────────────────────────────
pub struct IcmpHeader<'a> { raw: &'a [u8] }

impl<'a> IcmpHeader<'a> {
    pub fn new(raw: &'a [u8]) -> Option<Self> {
        if raw.len() < 8 { None } else { Some(Self { raw }) }
    }
    pub fn msg_type(&self)  -> u8  { self.raw[0] }
    pub fn code(&self)      -> u8  { self.raw[1] }
    pub fn checksum(&self)  -> u16 { u16::from_be_bytes([self.raw[2], self.raw[3]]) }
    pub fn identifier(&self) -> u16 { u16::from_be_bytes([self.raw[4], self.raw[5]]) }
    pub fn sequence(&self)  -> u16  { u16::from_be_bytes([self.raw[6], self.raw[7]]) }
    pub fn payload(&self)   -> &[u8] { &self.raw[8..] }

    pub fn checksum_valid(&self) -> bool {
        let mut sum = 0u32;
        let mut i = 0;
        while i + 1 < self.raw.len() {
            sum += u16::from_be_bytes([self.raw[i], self.raw[i+1]]) as u32;
            i += 2;
        }
        if i < self.raw.len() { sum += (self.raw[i] as u32) << 8; }
        while sum >> 16 != 0 { sum = (sum & 0xFFFF) + (sum >> 16); }
        !(sum as u16) == 0
    }
}

// ── ICMP Reply Builder ────────────────────────────────────────────────────────

/// Build an ICMP Echo Reply from an Echo Request.
/// Returns the reply length written into `out`.
pub fn build_echo_reply(request: &[u8], out: &mut [u8; 1500]) -> usize {
    let hdr = match IcmpHeader::new(request) { Some(h) => h, None => return 0 };
    if hdr.msg_type() != ICMP_ECHO_REQUEST { return 0; }

    let payload = hdr.payload();
    let total = 8 + payload.len();
    if total > 1500 { return 0; }

    out[0] = ICMP_ECHO_REPLY;
    out[1] = 0; // code
    out[2] = 0; out[3] = 0; // checksum placeholder
    out[4] = hdr.raw[4]; out[5] = hdr.raw[5]; // identifier
    out[6] = hdr.raw[6]; out[7] = hdr.raw[7]; // sequence
    out[8..8+payload.len()].copy_from_slice(payload);

    // Compute checksum
    let chk = icmp_checksum(&out[..total]);
    out[2] = (chk >> 8) as u8;
    out[3] = (chk & 0xFF) as u8;
    total
}

/// Build an ICMP Destination Unreachable message
pub fn build_dest_unreach(original_ip_hdr: &[u8], code: u8, out: &mut [u8; 1500]) -> usize {
    let orig_len = original_ip_hdr.len().min(28); // IP header + 8 bytes of original
    let total = 8 + orig_len;
    if total > 1500 { return 0; }
    out[0] = ICMP_DEST_UNREACH;
    out[1] = code;
    out[2] = 0; out[3] = 0;  // checksum
    out[4] = 0; out[5] = 0; out[6] = 0; out[7] = 0; // unused
    out[8..8+orig_len].copy_from_slice(&original_ip_hdr[..orig_len]);
    let chk = icmp_checksum(&out[..total]);
    out[2] = (chk >> 8) as u8;
    out[3] = (chk & 0xFF) as u8;
    total
}

pub fn icmp_checksum(data: &[u8]) -> u16 {
    let mut sum = 0u32;
    let mut i = 0;
    while i + 1 < data.len() {
        sum += u16::from_be_bytes([data[i], data[i+1]]) as u32;
        i += 2;
    }
    if i < data.len() { sum += (data[i] as u32) << 8; }
    while sum >> 16 != 0 { sum = (sum & 0xFFFF) + (sum >> 16); }
    !(sum as u16)
}

// ── ICMP Statistics ───────────────────────────────────────────────────────────
#[derive(Default, Clone, Copy)]
pub struct IcmpStats {
    pub rx_echo_req:   u64,
    pub tx_echo_reply: u64,
    pub rx_echo_reply: u64,
    pub rx_unreach:    u64,
    pub rx_time_exc:   u64,
    pub bad_checksum:  u64,
}

// ── ICMP Handler ─────────────────────────────────────────────────────────────
pub struct IcmpHandler {
    pub stats: IcmpStats,
    reply_buf: [u8; 1500],
}

impl IcmpHandler {
    pub const fn new() -> Self {
        Self { stats: IcmpStats {
            rx_echo_req:0,tx_echo_reply:0,rx_echo_reply:0,
            rx_unreach:0,rx_time_exc:0,bad_checksum:0,
        }, reply_buf: [0u8; 1500] }
    }

    /// Process incoming ICMP packet. Returns reply length (>0) if a reply should be sent.
    pub fn process(&mut self, pkt: &[u8]) -> usize {
        let hdr = match IcmpHeader::new(pkt) { Some(h) => h, None => return 0 };
        if !hdr.checksum_valid() { self.stats.bad_checksum += 1; return 0; }
        match hdr.msg_type() {
            ICMP_ECHO_REQUEST => {
                self.stats.rx_echo_req += 1;
                let n = build_echo_reply(pkt, &mut self.reply_buf);
                if n > 0 { self.stats.tx_echo_reply += 1; }
                n
            }
            ICMP_ECHO_REPLY   => { self.stats.rx_echo_reply += 1; 0 }
            ICMP_DEST_UNREACH => { self.stats.rx_unreach    += 1; 0 }
            ICMP_TIME_EXCEEDED => { self.stats.rx_time_exc  += 1; 0 }
            _ => 0
        }
    }

    pub fn reply_buf(&self) -> &[u8] { &self.reply_buf }
}
