// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/net/sigma_dhcp.rs — DHCP Client (RFC 2131/2132, no_std)
// Language: Rust #![no_std]
// Pattern: OOP via DhcpClient struct with state machine

#![no_std]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DhcpState { Init, Selecting, Requesting, Bound, Renewing, Rebinding }

#[derive(Clone, Copy, Default)]
pub struct DhcpLease {
    pub ip:       [u8; 4],
    pub netmask:  [u8; 4],
    pub gateway:  [u8; 4],
    pub dns:      [u8; 4],
    pub lease_s:  u32, // lease duration seconds
    pub t1_s:     u32, // renewal time
    pub t2_s:     u32, // rebinding time
}

const DHCP_MAGIC: [u8; 4] = [99, 130, 83, 99];
const BOOT_REQUEST: u8  = 1;
const HTYPE_ETHERNET: u8 = 1;
const DHCPDISCOVER: u8  = 1;
const DHCPOFFER:    u8  = 2;
const DHCPREQUEST:  u8  = 3;
const DHCPACK:      u8  = 5;
const DHCPNAK:      u8  = 6;

pub struct DhcpClient {
    pub state:    DhcpState,
    pub lease:    DhcpLease,
    mac:          [u8; 6],
    xid:          u32,   // transaction ID
    server_ip:    [u8; 4],
    offered_ip:   [u8; 4],
    tick:         u64,
    timeout_tick: u64,
}

impl DhcpClient {
    pub const fn new(mac: [u8; 6]) -> Self {
        Self {
            state: DhcpState::Init, lease: DhcpLease {
                ip: [0;4], netmask: [0;4], gateway: [0;4],
                dns: [0;4], lease_s: 0, t1_s: 0, t2_s: 0,
            },
            mac, xid: 0xDEADBEEF,
            server_ip: [0;4], offered_ip: [0;4],
            tick: 0, timeout_tick: 0,
        }
    }

    /// Build a DHCPDISCOVER packet into `buf`. Returns packet length.
    pub fn build_discover(&self, buf: &mut [u8; 300]) -> usize {
        buf.fill(0);
        buf[0]  = BOOT_REQUEST;
        buf[1]  = HTYPE_ETHERNET;
        buf[2]  = 6; // hlen
        buf[3]  = 0; // hops
        buf[4..8].copy_from_slice(&self.xid.to_be_bytes());
        buf[28..34].copy_from_slice(&self.mac);
        buf[236..240].copy_from_slice(&DHCP_MAGIC);
        // Options: message type = DISCOVER
        buf[240] = 53; buf[241] = 1; buf[242] = DHCPDISCOVER;
        // Parameter request list
        buf[243] = 55; buf[244] = 4;
        buf[245] = 1; // subnet mask
        buf[246] = 3; // router
        buf[247] = 6; // DNS
        buf[248] = 15; // domain name
        buf[249] = 255; // end
        250
    }

    /// Build a DHCPREQUEST packet. Returns packet length.
    pub fn build_request(&self, buf: &mut [u8; 300]) -> usize {
        buf.fill(0);
        buf[0]  = BOOT_REQUEST;
        buf[1]  = HTYPE_ETHERNET;
        buf[2]  = 6;
        buf[4..8].copy_from_slice(&self.xid.to_be_bytes());
        buf[28..34].copy_from_slice(&self.mac);
        buf[236..240].copy_from_slice(&DHCP_MAGIC);
        let mut off = 240;
        // DHCPREQUEST
        buf[off] = 53; buf[off+1] = 1; buf[off+2] = DHCPREQUEST; off += 3;
        // Requested IP
        buf[off] = 50; buf[off+1] = 4;
        buf[off+2..off+6].copy_from_slice(&self.offered_ip); off += 6;
        // Server identifier
        buf[off] = 54; buf[off+1] = 4;
        buf[off+2..off+6].copy_from_slice(&self.server_ip); off += 6;
        buf[off] = 255; // end
        off + 1
    }

    /// Process an incoming DHCP packet. Returns true if a reply should be sent.
    pub fn process(&mut self, pkt: &[u8]) -> DhcpEvent {
        if pkt.len() < 240 { return DhcpEvent::None; }
        let xid = u32::from_be_bytes(pkt[4..8].try_into().unwrap_or([0;4]));
        if xid != self.xid { return DhcpEvent::None; }
        if &pkt[236..240] != &DHCP_MAGIC { return DhcpEvent::None; }

        // Parse options
        let mut msg_type = 0u8;
        let mut i = 240;
        while i < pkt.len() {
            match pkt[i] {
                255 => break,
                0   => { i += 1; continue; }
                code => {
                    if i + 1 >= pkt.len() { break; }
                    let len = pkt[i+1] as usize;
                    if i + 2 + len > pkt.len() { break; }
                    let data = &pkt[i+2..i+2+len];
                    match code {
                        53 if len == 1 => msg_type = data[0],
                        1  if len == 4 => self.lease.netmask.copy_from_slice(data),
                        3  if len >= 4 => self.lease.gateway.copy_from_slice(&data[..4]),
                        6  if len >= 4 => self.lease.dns.copy_from_slice(&data[..4]),
                        51 if len == 4 => self.lease.lease_s = u32::from_be_bytes(data.try_into().unwrap_or([0;4])),
                        54 if len == 4 => self.server_ip.copy_from_slice(data),
                        58 if len == 4 => self.lease.t1_s = u32::from_be_bytes(data.try_into().unwrap_or([0;4])),
                        59 if len == 4 => self.lease.t2_s = u32::from_be_bytes(data.try_into().unwrap_or([0;4])),
                        _ => {}
                    }
                    i += 2 + len;
                }
            }
        }

        // Offered/assigned IP is in yiaddr (offset 16)
        if pkt.len() >= 20 {
            self.offered_ip.copy_from_slice(&pkt[16..20]);
        }

        match (self.state, msg_type) {
            (DhcpState::Selecting, DHCPOFFER) => {
                self.state = DhcpState::Requesting;
                DhcpEvent::SendRequest
            }
            (DhcpState::Requesting, DHCPACK) => {
                self.lease.ip.copy_from_slice(&self.offered_ip);
                self.state = DhcpState::Bound;
                DhcpEvent::Bound(self.lease)
            }
            (DhcpState::Requesting, DHCPNAK) => {
                self.state = DhcpState::Init;
                DhcpEvent::Nak
            }
            _ => DhcpEvent::None,
        }
    }

    pub fn start_discover(&mut self) -> DhcpEvent {
        self.state = DhcpState::Selecting;
        DhcpEvent::SendDiscover
    }

    pub fn tick(&mut self, ticks: u64) {
        self.tick = ticks;
    }
}

#[derive(Clone, Copy)]
pub enum DhcpEvent {
    None,
    SendDiscover,
    SendRequest,
    Bound(DhcpLease),
    Nak,
}
