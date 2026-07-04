// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// net/sigma_udp.rs — UDP Protocol Implementation
//
// Implements UDP (RFC 768) for SigmaOS network stack.
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, Ordering};

// ─── UDP Header ───────────────────────────────────────────────────────────────

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct UdpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub checksum: u16,
}

impl UdpHeader {
    pub const fn new(src_port: u16, dst_port: u16, length: u16) -> Self {
        UdpHeader {
            src_port: src_port.to_be(),
            dst_port: dst_port.to_be(),
            length: length.to_be(),
            checksum: 0,
        }
    }
}

// ─── UDP Socket ────────────────────────────────────────────────────────────────

const UDP_RX_BUF_SIZE: usize = 65536;
const UDP_TX_BUF_SIZE: usize = 65536;

pub struct UdpSocket {
    pub local_port: u16,
    pub bound: bool,
    pub remote_ip: [u8; 4],
    pub remote_port: u16,
    pub connected: bool,
    
    rx_buf: [u8; UDP_RX_BUF_SIZE],
    rx_head: usize,
    rx_tail: usize,
    tx_buf: [u8; UDP_TX_BUF_SIZE],
    tx_head: usize,
    tx_tail: usize,
}

impl UdpSocket {
    pub const fn new() -> Self {
        UdpSocket {
            local_port: 0,
            bound: false,
            remote_ip: [0; 4],
            remote_port: 0,
            connected: false,
            rx_buf: [0; UDP_RX_BUF_SIZE],
            rx_head: 0,
            rx_tail: 0,
            tx_buf: [0; UDP_TX_BUF_SIZE],
            tx_head: 0,
            tx_tail: 0,
        }
    }
    
    pub fn rx_push(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &b in data {
            let next = (self.rx_tail + 1) % UDP_RX_BUF_SIZE;
            if next == self.rx_head { break; }
            self.rx_buf[self.rx_tail] = b;
            self.rx_tail = next;
            written += 1;
        }
        written
    }
    
    pub fn rx_pop(&mut self, buf: &mut [u8]) -> usize {
        let mut read = 0;
        while read < buf.len() && self.rx_head != self.rx_tail {
            buf[read] = self.rx_buf[self.rx_head];
            self.rx_head = (self.rx_head + 1) % UDP_RX_BUF_SIZE;
            read += 1;
        }
        read
    }
    
    pub fn rx_available(&self) -> usize {
        if self.rx_tail >= self.rx_head {
            self.rx_tail - self.rx_head
        } else {
            UDP_RX_BUF_SIZE - self.rx_head + self.rx_tail
        }
    }
    
    pub fn tx_push(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &b in data {
            let next = (self.tx_tail + 1) % UDP_TX_BUF_SIZE;
            if next == self.tx_head { break; }
            self.tx_buf[self.tx_tail] = b;
            self.tx_tail = next;
            written += 1;
        }
        written
    }
    
    pub fn tx_available(&self) -> usize {
        if self.tx_tail >= self.tx_head {
            UDP_TX_BUF_SIZE - (self.tx_tail - self.tx_head) - 1
        } else {
            self.tx_head - self.tx_tail - 1
        }
    }
}

// ─── UDP Socket Table ─────────────────────────────────────────────────────────

const MAX_UDP_SOCKETS: usize = 64;

static mut UDP_SOCKETS: [UdpSocket; MAX_UDP_SOCKETS] = [const { UdpSocket::new() }; MAX_UDP_SOCKETS];
static mut UDP_SOCKET_COUNT: usize = 0;

pub fn udp_alloc_socket() -> Option<usize> {
    unsafe {
        for i in 0..MAX_UDP_SOCKETS {
            if !UDP_SOCKETS[i].bound {
                UDP_SOCKETS[i] = UdpSocket::new();
                UDP_SOCKET_COUNT += 1;
                return Some(i);
            }
        }
        None
    }
}

pub fn udp_free_socket(idx: usize) {
    unsafe {
        if idx < MAX_UDP_SOCKETS {
            UDP_SOCKETS[idx] = UdpSocket::new();
            UDP_SOCKET_COUNT = UDP_SOCKET_COUNT.saturating_sub(1);
        }
    }
}

pub fn udp_get_socket(idx: usize) -> Option<&'static mut UdpSocket> {
    unsafe {
        if idx < MAX_UDP_SOCKETS && UDP_SOCKETS[idx].bound {
            Some(&mut UDP_SOCKETS[idx])
        } else {
            None
        }
    }
}

pub fn udp_bind_port(port: u16) -> bool {
    unsafe {
        for sock in UDP_SOCKETS.iter() {
            if sock.bound && sock.local_port == port {
                return false;
            }
        }
        true
    }
}

// ─── UDP Checksum Calculation ─────────────────────────────────────────────────

fn udp_checksum(
    src_ip: [u8; 4],
    dst_ip: [u8; 4],
    hdr: &UdpHeader,
    payload: &[u8],
) -> u16 {
    let mut sum: u32 = 0;
    
    // Pseudo-header
    sum += u32::from_be_bytes([src_ip[0], src_ip[1], 0, src_ip[2]]);
    sum += u32::from_be_bytes([src_ip[3], dst_ip[0], 0, dst_ip[1]]);
    sum += u32::from_be_bytes([dst_ip[2], dst_ip[3], 0, 17]); // Protocol UDP
    sum += u32::from_be(hdr.length);
    
    // UDP header
    let hdr_bytes = unsafe {
        core::slice::from_raw_parts(hdr as *const _ as *const u8, 8)
    };
    for chunk in hdr_bytes.chunks(2) {
        if chunk.len() == 2 {
            sum += u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
        }
    }
    
    // Payload
    for chunk in payload.chunks(2) {
        if chunk.len() == 2 {
            sum += u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
        } else if chunk.len() == 1 {
            sum += (chunk[0] as u32) << 8;
        }
    }
    
    while sum >> 16 != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    
    !sum as u16
}

// ─── C-ABI Exports ────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_udp_init() {
    UDP_SOCKET_COUNT = 0;
    for sock in UDP_SOCKETS.iter_mut() {
        *sock = UdpSocket::new();
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_udp_socket_alloc() -> i32 {
    match udp_alloc_socket() {
        Some(idx) => idx as i32,
        None => -1, // EMFILE
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_udp_socket_free(idx: i32) {
    if idx >= 0 {
        udp_free_socket(idx as usize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_udp_bind(idx: i32, port: u16) -> i32 {
    if idx < 0 { return -9; } // EBADF
    if !udp_bind_port(port) { return -98; } // EADDRINUSE
    
    if let Some(sock) = udp_get_socket(idx as usize) {
        sock.local_port = port;
        sock.bound = true;
        0
    } else {
        -9 // EBADF
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_udp_connect(
    idx: i32,
    dst_ip: u32,
    dst_port: u16,
) -> i32 {
    if idx < 0 { return -9; }
    
    if let Some(sock) = udp_get_socket(idx as usize) {
        sock.remote_ip = dst_ip.to_be_bytes();
        sock.remote_port = dst_port;
        sock.connected = true;
        0
    } else {
        -9
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_udp_sendto(
    idx: i32,
    buf: *const u8,
    len: usize,
    dst_ip: u32,
    dst_port: u16,
) -> i64 {
    if idx < 0 || buf.is_null() { return -9; }
    
    let data = core::slice::from_raw_parts(buf, len);
    
    if let Some(sock) = udp_get_socket(idx as usize) {
        let send_ip = if sock.connected {
            u32::from_be_bytes(sock.remote_ip)
        } else {
            dst_ip
        };
        let send_port = if sock.connected {
            sock.remote_port
        } else {
            dst_port
        };
        
        // Build UDP packet and transmit via NIC
        extern "C" { fn nic_tx_packet(data: *const u8, len: usize); }
        
        let mut packet = [0u8; 1514];
        let udp_len = (8 + len).min(1472) as u16;
        
        // Build UDP header
        let udp_hdr = UdpHeader::new(sock.local_port, send_port, udp_len);
        
        // Build IPv4 header (simplified, assumes NIC IP is set)
        extern "C" { fn sigma_net_get_ip() -> u32; }
        let src_ip = sigma_net_get_ip().to_be_bytes();
        let dst_ip_bytes = send_ip.to_be_bytes();
        
        // Calculate checksum
        let checksum = udp_checksum(src_ip, dst_ip_bytes, &udp_hdr, data);
        
        // Assemble packet (simplified - needs full Ethernet/IP framing)
        let mut offset = 0;
        // Ethernet header would go here
        offset += 14;
        // IPv4 header would go here
        offset += 20;
        
        // UDP header
        let hdr_bytes = core::slice::from_raw_parts(&udp_hdr as *const _ as *const u8, 8);
        packet[offset..offset+8].copy_from_slice(hdr_bytes);
        offset += 8;
        
        // Payload
        let copy_len = len.min(packet.len() - offset);
        packet[offset..offset+copy_len].copy_from_slice(&data[..copy_len]);
        
        let total = offset + copy_len;
        nic_tx_packet(packet.as_ptr(), total);
        
        copy_len as i64
    } else {
        -9
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_udp_recvfrom(
    idx: i32,
    buf: *mut u8,
    len: usize,
    src_ip: *mut u32,
    src_port: *mut u16,
) -> i64 {
    if idx < 0 || buf.is_null() { return -9; }
    
    if let Some(sock) = udp_get_socket(idx as usize) {
        let dst = core::slice::from_raw_parts_mut(buf, len);
        let received = sock.rx_pop(dst);
        
        if !src_ip.is_null() {
            *src_ip = u32::from_be_bytes(sock.remote_ip);
        }
        if !src_port.is_null() {
            *src_port = sock.remote_port;
        }
        
        received as i64
    } else {
        -9
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_udp_rx_dispatch(
    dst_port: u16,
    src_ip: [u8; 4],
    src_port: u16,
    data: *const u8,
    len: usize,
) {
    let payload = core::slice::from_raw_parts(data, len);
    
    for sock in UDP_SOCKETS.iter_mut() {
        if sock.bound && sock.local_port == dst_port {
            sock.remote_ip = src_ip;
            sock.remote_port = src_port;
            sock.rx_push(payload);
            break;
        }
    }
}
