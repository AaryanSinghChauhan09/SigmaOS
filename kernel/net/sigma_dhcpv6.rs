// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/net/sigma_dhcpv6.rs — DHCPv6 + SLAAC (no_std, cleanroom)
// Language: Rust #![no_std]

#![no_std]

pub const DHCPV6_PORT_CLIENT: u16 = 546;
pub const DHCPV6_PORT_SERVER: u16 = 547;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dhcpv6MsgType {
    Solicit     = 1, Advertise = 2, Request  = 3, Confirm   = 4,
    Renew       = 5, Rebind    = 6, Reply    = 7, Release   = 8,
    Decline     = 9, Reconfigure=10, InfoReq = 11, RelayForw=12, RelayRepl=13,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dhcpv6State { Init, Soliciting, Requesting, Bound, Renewing, Rebinding }

#[derive(Clone, Copy, Default)]
pub struct Ipv6Addr(pub [u8; 16]);

impl Ipv6Addr {
    pub fn link_local(mac: &[u8; 6]) -> Self {
        let mut addr = [0u8; 16];
        addr[0] = 0xFE; addr[1] = 0x80;
        addr[8]  = mac[0] ^ 0x02;
        addr[9]  = mac[1]; addr[10] = mac[2];
        addr[11] = 0xFF; addr[12] = 0xFE;
        addr[13] = mac[3]; addr[14] = mac[4]; addr[15] = mac[5];
        Self(addr)
    }
    pub fn is_valid(&self) -> bool { self.0 != [0u8;16] }
}

#[derive(Clone, Copy, Default)]
pub struct Dhcpv6Lease {
    pub addr:          Ipv6Addr,
    pub prefix_len:    u8,
    pub gateway:       Ipv6Addr,
    pub dns:           [Ipv6Addr; 4],
    pub n_dns:         usize,
    pub preferred_lt:  u32,
    pub valid_lt:      u32,
}

pub struct Dhcpv6Client {
    pub state:    Dhcpv6State,
    pub lease:    Dhcpv6Lease,
    pub ll_addr:  Ipv6Addr,
    duid:         [u8; 18], // DUID-LL: 2-byte type + 2-byte hw-type + 6-byte MAC
    xid:          [u8; 3],
}

impl Dhcpv6Client {
    pub fn new(mac: &[u8;6]) -> Self {
        let mut duid = [0u8;18];
        duid[0] = 0; duid[1] = 3; // DUID-LL
        duid[2] = 0; duid[3] = 1; // Ethernet
        duid[4..10].copy_from_slice(mac);
        Self {
            state: Dhcpv6State::Init,
            lease: Dhcpv6Lease::default(),
            ll_addr: Ipv6Addr::link_local(mac),
            duid,
            xid: [0xAB, 0xCD, 0xEF],
        }
    }

    pub fn build_solicit(&self, buf: &mut [u8;128]) -> usize {
        buf[0] = Dhcpv6MsgType::Solicit as u8;
        buf[1..4].copy_from_slice(&self.xid);
        let mut off = 4;
        // Option 1: DUID
        buf[off] = 0; buf[off+1] = 1; off += 2;  // option type
        buf[off] = 0; buf[off+1] = 10; off += 2; // option len
        buf[off..off+10].copy_from_slice(&self.duid[..10]); off += 10;
        // Option 6: request list (DNS=23, prefix=26)
        buf[off] = 0; buf[off+1] = 6; off += 2;
        buf[off] = 0; buf[off+1] = 4; off += 2;
        buf[off] = 0; buf[off+1] = 23; off += 2;
        buf[off] = 0; buf[off+1] = 26; off += 2;
        off
    }

    pub fn process_reply(&mut self, pkt: &[u8]) -> bool {
        if pkt.len() < 4 { return false; }
        if pkt[0] != Dhcpv6MsgType::Reply as u8 { return false; }
        // Parse IA_NA (option 3) for IPv6 address
        let mut off = 4;
        while off + 4 <= pkt.len() {
            let opt = u16::from_be_bytes([pkt[off], pkt[off+1]]);
            let len = u16::from_be_bytes([pkt[off+2], pkt[off+3]]) as usize;
            off += 4;
            if off + len > pkt.len() { break; }
            match opt {
                3 => { // IA_NA
                    if len >= 16 {
                        let mut addr = [0u8;16];
                        addr.copy_from_slice(&pkt[off+8..off+24]);
                        self.lease.addr = Ipv6Addr(addr);
                        self.lease.valid_lt = u32::from_be_bytes(pkt[off+28..off+32].try_into().unwrap_or([0;4]));
                    }
                }
                23 => { // DNS servers
                    let mut i = 0;
                    while i + 16 <= len && self.lease.n_dns < 4 {
                        let mut a = [0u8;16];
                        a.copy_from_slice(&pkt[off+i..off+i+16]);
                        self.lease.dns[self.lease.n_dns] = Ipv6Addr(a);
                        self.lease.n_dns += 1;
                        i += 16;
                    }
                }
                _ => {}
            }
            off += len;
        }
        if self.lease.addr.is_valid() { self.state = Dhcpv6State::Bound; true } else { false }
    }
}
