// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// net/dhcp/sigma_dhcp.rs — DHCP Client Implementation
//
// Implements DHCP client (RFC 2131) for automatic IP configuration.
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, Ordering};

// ─── DHCP Constants ───────────────────────────────────────────────────────────

pub const DHCP_SERVER_PORT: u16 = 67;
pub const DHCP_CLIENT_PORT: u16 = 68;
pub const DHCP_MAGIC_COOKIE: u32 = 0x63825363;

// DHCP Message Types
pub const DHCP_DISCOVER: u8 = 1;
pub const DHCP_OFFER: u8 = 2;
pub const DHCP_REQUEST: u8 = 3;
pub const DHCP_DECLINE: u8 = 4;
pub const DHCP_ACK: u8 = 5;
pub const DHCP_NAK: u8 = 6;
pub const DHCP_RELEASE: u8 = 7;
pub const DHCP_INFORM: u8 = 8;

// DHCP Options
pub const DHCP_OPT_SUBNET_MASK: u8 = 1;
pub const DHCP_OPT_ROUTER: u8 = 3;
pub const DHCP_OPT_DNS: u8 = 6;
pub const DHCP_OPT_REQUESTED_IP: u8 = 50;
pub const DHCP_OPT_LEASE_TIME: u8 = 51;
pub const DHCP_OPT_SERVER_ID: u8 = 54;
pub const DHCP_OPT_PARAM_REQ: u8 = 55;
pub const DHCP_OPT_MESSAGE_TYPE: u8 = 53;
pub const DHCP_OPT_END: u8 = 255;

// ─── DHCP State Machine ───────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq)]
pub enum DhcpState {
    Init,
    Selecting,
    Requesting,
    Bound,
    Renewing,
    Rebinding,
    InitReboot,
}

// ─── DHCP Packet Structure ────────────────────────────────────────────────────

#[repr(C, packed)]
pub struct DhcpPacket {
    pub op: u8,           // Message op code / message type
    pub htype: u8,        // Hardware address type
    pub hlen: u8,         // Hardware address length
    pub hops: u8,         // Hops
    pub xid: u32,         // Transaction ID
    pub secs: u16,        // Seconds elapsed
    pub flags: u16,       // Bootp flags
    pub ciaddr: [u8; 4],  // Client IP address
    pub yiaddr: [u8; 4],  // 'your' (client) IP address
    pub siaddr: [u8; 4],  // IP address of next server
    pub giaddr: [u8; 4],  // Relay agent IP address
    pub chaddr: [u8; 16], // Client hardware address
    pub sname: [u8; 64],  // Optional server host name
    pub file: [u8; 128],  // Boot file name
    pub options: [u8; 312], // Optional parameters field
}

impl DhcpPacket {
    pub const fn new() -> Self {
        DhcpPacket {
            op: 1, // BOOTREQUEST
            htype: 1, // Ethernet
            hlen: 6, // MAC length
            hops: 0,
            xid: 0,
            secs: 0,
            flags: 0x8000, // Broadcast flag
            ciaddr: [0; 4],
            yiaddr: [0; 4],
            siaddr: [0; 4],
            giaddr: [0; 4],
            chaddr: [0; 16],
            sname: [0; 64],
            file: [0; 128],
            options: [0; 312],
        }
    }
}

// ─── DHCP Client Configuration ────────────────────────────────────────────────

pub struct DhcpConfig {
    pub ip: [u8; 4],
    pub netmask: [u8; 4],
    pub gateway: [u8; 4],
    pub dns: [[u8; 4]; 2],
    pub lease_time: u32,
    pub server_ip: [u8; 4],
}

impl DhcpConfig {
    pub const fn new() -> Self {
        DhcpConfig {
            ip: [0; 4],
            netmask: [255, 255, 255, 0],
            gateway: [0; 4],
            dns: [[0; 4]; 2],
            lease_time: 0,
            server_ip: [0; 4],
        }
    }
}

// ─── DHCP Client ─────────────────────────────────────────────────────────────

pub struct DhcpClient {
    pub state: DhcpState,
    pub xid: u32,
    pub config: DhcpConfig,
    pub mac: [u8; 6],
    pub retry_count: u32,
    pub lease_expiry: u32,
}

impl DhcpClient {
    pub const fn new() -> Self {
        DhcpClient {
            state: DhcpState::Init,
            xid: 0,
            config: DhcpConfig::new(),
            mac: [0; 6],
            retry_count: 0,
            lease_expiry: 0,
        }
    }
    
    pub fn set_mac(&mut self, mac: [u8; 6]) {
        self.mac = mac;
    }
    
    pub fn generate_xid(&mut self) {
        self.xid = self.xid.wrapping_add(1);
    }
    
    pub fn build_discover(&self) -> DhcpPacket {
        let mut pkt = DhcpPacket::new();
        pkt.op = 1; // BOOTREQUEST
        pkt.htype = 1; // Ethernet
        pkt.hlen = 6;
        pkt.xid = self.xid;
        pkt.chaddr[..6].copy_from_slice(&self.mac);
        
        // Add magic cookie
        pkt.options[0..4].copy_from_slice(&DHCP_MAGIC_COOKIE.to_be_bytes());
        
        // Add message type option
        pkt.options[4] = DHCP_OPT_MESSAGE_TYPE;
        pkt.options[5] = 1; // Length
        pkt.options[6] = DHCP_DISCOVER;
        
        // Add parameter request list
        pkt.options[7] = DHCP_OPT_PARAM_REQ;
        pkt.options[8] = 4; // Length
        pkt.options[9] = DHCP_OPT_SUBNET_MASK;
        pkt.options[10] = DHCP_OPT_ROUTER;
        pkt.options[11] = DHCP_OPT_DNS;
        pkt.options[12] = DHCP_OPT_LEASE_TIME;
        
        // End option
        pkt.options[13] = DHCP_OPT_END;
        
        pkt
    }
    
    pub fn build_request(&self, server_ip: [u8; 4]) -> DhcpPacket {
        let mut pkt = DhcpPacket::new();
        pkt.op = 1; // BOOTREQUEST
        pkt.htype = 1;
        pkt.hlen = 6;
        pkt.xid = self.xid;
        pkt.ciaddr = self.config.ip;
        pkt.chaddr[..6].copy_from_slice(&self.mac);
        
        // Add magic cookie
        pkt.options[0..4].copy_from_slice(&DHCP_MAGIC_COOKIE.to_be_bytes());
        
        // Add message type option
        pkt.options[4] = DHCP_OPT_MESSAGE_TYPE;
        pkt.options[5] = 1;
        pkt.options[6] = DHCP_REQUEST;
        
        // Add requested IP option
        pkt.options[7] = DHCP_OPT_REQUESTED_IP;
        pkt.options[8] = 4;
        pkt.options[9..13].copy_from_slice(&self.config.ip);
        
        // Add server identifier option
        pkt.options[13] = DHCP_OPT_SERVER_ID;
        pkt.options[14] = 4;
        pkt.options[15..19].copy_from_slice(&server_ip);
        
        // Add parameter request list
        pkt.options[19] = DHCP_OPT_PARAM_REQ;
        pkt.options[20] = 4;
        pkt.options[21] = DHCP_OPT_SUBNET_MASK;
        pkt.options[22] = DHCP_OPT_ROUTER;
        pkt.options[23] = DHCP_OPT_DNS;
        pkt.options[24] = DHCP_OPT_LEASE_TIME;
        
        // End option
        pkt.options[25] = DHCP_OPT_END;
        
        pkt
    }
    
    pub fn parse_options(&mut self, pkt: &DhcpPacket) {
        let options = &pkt.options;
        let mut pos = 4; // Skip magic cookie
        
        while pos < options.len() && options[pos] != DHCP_OPT_END {
            let opt_type = options[pos];
            if opt_type == 0 {
                pos += 1;
                continue;
            }
            
            if pos + 1 >= options.len() { break; }
            let opt_len = options[pos + 1] as usize;
            
            if pos + 2 + opt_len > options.len() { break; }
            let opt_data = &options[pos + 2..pos + 2 + opt_len];
            
            match opt_type {
                DHCP_OPT_MESSAGE_TYPE => {
                    if !opt_data.is_empty() {
                        let msg_type = opt_data[0];
                        self.handle_message_type(msg_type);
                    }
                }
                DHCP_OPT_SUBNET_MASK => {
                    if opt_len == 4 {
                        self.config.netmask.copy_from_slice(opt_data);
                    }
                }
                DHCP_OPT_ROUTER => {
                    if opt_len >= 4 {
                        self.config.gateway.copy_from_slice(&opt_data[..4]);
                    }
                }
                DHCP_OPT_DNS => {
                    if opt_len >= 4 {
                        self.config.dns[0].copy_from_slice(&opt_data[..4]);
                        if opt_len >= 8 {
                            self.config.dns[1].copy_from_slice(&opt_data[4..8]);
                        }
                    }
                }
                DHCP_OPT_LEASE_TIME => {
                    if opt_len == 4 {
                        self.config.lease_time = u32::from_be_bytes([
                            opt_data[0], opt_data[1], opt_data[2], opt_data[3],
                        ]);
                    }
                }
                DHCP_OPT_SERVER_ID => {
                    if opt_len == 4 {
                        self.config.server_ip.copy_from_slice(opt_data);
                    }
                }
                _ => {}
            }
            
            pos += 2 + opt_len;
        }
    }
    
    fn handle_message_type(&mut self, msg_type: u8) {
        match msg_type {
            DHCP_OFFER => {
                self.state = DhcpState::Selecting;
            }
            DHCP_ACK => {
                self.state = DhcpState::Bound;
                self.retry_count = 0;
            }
            DHCP_NAK => {
                self.state = DhcpState::Init;
                self.retry_count += 1;
            }
            _ => {}
        }
    }
    
    pub fn process_offer(&mut self, pkt: &DhcpPacket) {
        self.config.ip = pkt.yiaddr;
        self.parse_options(pkt);
        self.state = DhcpState::Requesting;
    }
    
    pub fn process_ack(&mut self, pkt: &DhcpPacket) {
        self.config.ip = pkt.yiaddr;
        self.parse_options(pkt);
        self.state = DhcpState::Bound;
        self.retry_count = 0;
    }
    
    pub fn should_renew(&self, current_time: u32) -> bool {
        if self.config.lease_time == 0 { return false; }
        let renew_time = self.config.lease_time / 2; // T1 = 50% of lease
        current_time >= renew_time
    }
    
    pub fn should_rebind(&self, current_time: u32) -> bool {
        if self.config.lease_time == 0 { return false; }
        let rebind_time = (self.config.lease_time * 7) / 8; // T2 = 87.5% of lease
        current_time >= rebind_time
    }
}

// ─── Global DHCP Client Instance ───────────────────────────────────────────────

static mut DHCP_CLIENT: DhcpClient = DhcpClient::new();

// ─── C-ABI Exports ────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_dhcp_init() {
    DHCP_CLIENT = DhcpClient::new();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_dhcp_set_mac(mac: *const u8) {
    let mut mac_bytes = [0u8; 6];
    if !mac.is_null() {
        let mac_slice = core::slice::from_raw_parts(mac, 6);
        mac_bytes.copy_from_slice(mac_slice);
    }
    DHCP_CLIENT.set_mac(mac_bytes);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_dhcp_discover(buf: *mut u8, len: usize) -> i32 {
    DHCP_CLIENT.generate_xid();
    let pkt = DHCP_CLIENT.build_discover();
    
    let pkt_bytes = core::slice::from_raw_parts(&pkt as *const _ as *const u8, core::mem::size_of::<DhcpPacket>());
    let copy_len = pkt_bytes.len().min(len);
    
    if !buf.is_null() {
        let dst = core::slice::from_raw_parts_mut(buf, len);
        dst[..copy_len].copy_from_slice(&pkt_bytes[..copy_len]);
    }
    
    copy_len as i32
}

#[no_mangle]
pub unsafe extern "C" fn sigma_dhcp_request(buf: *mut u8, len: usize, server_ip: u32) -> i32 {
    let pkt = DHCP_CLIENT.build_request(server_ip.to_be_bytes());
    
    let pkt_bytes = core::slice::from_raw_parts(&pkt as *const _ as *const u8, core::mem::size_of::<DhcpPacket>());
    let copy_len = pkt_bytes.len().min(len);
    
    if !buf.is_null() {
        let dst = core::slice::from_raw_parts_mut(buf, len);
        dst[..copy_len].copy_from_slice(&pkt_bytes[..copy_len]);
    }
    
    copy_len as i32
}

#[no_mangle]
pub unsafe extern "C" fn sigma_dhcp_process_offer(pkt: *const u8, len: usize) -> i32 {
    if pkt.is_null() || len < core::mem::size_of::<DhcpPacket>() {
        return -1;
    }
    
    let dhcp_pkt = &*(pkt as *const DhcpPacket);
    if dhcp_pkt.xid != DHCP_CLIENT.xid {
        return -2; // Wrong transaction ID
    }
    
    DHCP_CLIENT.process_offer(dhcp_pkt);
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_dhcp_process_ack(pkt: *const u8, len: usize) -> i32 {
    if pkt.is_null() || len < core::mem::size_of::<DhcpPacket>() {
        return -1;
    }
    
    let dhcp_pkt = &*(pkt as *const DhcpPacket);
    if dhcp_pkt.xid != DHCP_CLIENT.xid {
        return -2;
    }
    
    DHCP_CLIENT.process_ack(dhcp_pkt);
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_dhcp_get_ip() -> u32 {
    u32::from_be_bytes(DHCP_CLIENT.config.ip)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_dhcp_get_netmask() -> u32 {
    u32::from_be_bytes(DHCP_CLIENT.config.netmask)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_dhcp_get_gateway() -> u32 {
    u32::from_be_bytes(DHCP_CLIENT.config.gateway)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_dhcp_get_dns(idx: i32) -> u32 {
    if idx >= 0 && idx < 2 {
        u32::from_be_bytes(DHCP_CLIENT.config.dns[idx as usize])
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_dhcp_get_lease_time() -> u32 {
    DHCP_CLIENT.config.lease_time
}

#[no_mangle]
pub unsafe extern "C" fn sigma_dhcp_get_state() -> i32 {
    match DHCP_CLIENT.state {
        DhcpState::Init => 0,
        DhcpState::Selecting => 1,
        DhcpState::Requesting => 2,
        DhcpState::Bound => 3,
        DhcpState::Renewing => 4,
        DhcpState::Rebinding => 5,
        DhcpState::InitReboot => 6,
    }
}

