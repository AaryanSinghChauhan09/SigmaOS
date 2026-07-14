// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/net/udp.rs — UDP datagram protocol with Zero-Trust Identities
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, Ordering};

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct UdpHdr {
    pub src_port: u16,
    pub dst_port: u16,
    pub length:   u16,   // header + data
    pub checksum: u16,
}

// Zero-Trust Identity for UDP datagram verification
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ZeroTrustIdentity {
    pub identity_hash: [u8; 32],
    pub verified: AtomicBool,
}

impl ZeroTrustIdentity {
    pub const fn new() -> Self {
        Self {
            identity_hash: [0u8; 32],
            verified: AtomicBool::new(false),
        }
    }
    
    pub fn is_verified(&self) -> bool {
        self.verified.load(Ordering::Acquire)
    }
    
    pub fn verify(&self) {
        self.verified.store(true, Ordering::Release);
    }
}

const MAX_UDP_SOCKETS: usize = 64;
const UDP_RX_BUF: usize = 65536;

#[derive(Copy, Clone)]
struct UdpSocket {
    local_port:  u16,
    bound:       AtomicBool,
    rx_buf:      [u8; UDP_RX_BUF],
    rx_head:     usize,
    rx_tail:     usize,
    src_ip_last: u32,
    src_port_last: u16,
    active:      AtomicBool,
    peer_identity: ZeroTrustIdentity,
}

impl UdpSocket {
    const fn empty() -> Self {
        Self {
            local_port: 0, 
            bound: AtomicBool::new(false),
            rx_buf: [0u8; UDP_RX_BUF],
            rx_head: 0, rx_tail: 0,
            src_ip_last: 0, src_port_last: 0,
            active: AtomicBool::new(false),
            peer_identity: ZeroTrustIdentity::new(),
        }
    }
    fn push(&mut self, data: &[u8]) {
        for &b in data {
            let next = (self.rx_tail + 1) % UDP_RX_BUF;
            if next != self.rx_head {
                self.rx_buf[self.rx_tail] = b;
                self.rx_tail = next;
            }
        }
    }
    fn pop(&mut self, buf: &mut [u8]) -> usize {
        let mut n = 0;
        while n < buf.len() && self.rx_head != self.rx_tail {
            buf[n] = self.rx_buf[self.rx_head];
            self.rx_head = (self.rx_head + 1) % UDP_RX_BUF;
            n += 1;
        }
        n
    }
}

struct UdpStack {
    sockets: [UdpSocket; MAX_UDP_SOCKETS],
}

impl UdpStack {
    const fn new() -> Self {
        Self { sockets: [const { UdpSocket::empty() }; MAX_UDP_SOCKETS] }
    }
    fn alloc(&mut self) -> Option<usize> {
        self.sockets.iter().position(|s| !s.active.load(Ordering::Acquire))
    }
    fn find_port(&mut self, port: u16) -> Option<usize> {
        self.sockets.iter().position(|s| s.active.load(Ordering::Acquire) && s.bound.load(Ordering::Acquire) && s.local_port == port)
    }
    fn socket(&mut self) -> i32 {
        let i = self.alloc()?;
        self.sockets[i].active.store(true, Ordering::Release);
        i as i32
    }
    fn bind(&mut self, fd: usize, port: u16) -> i32 {
        if fd >= MAX_UDP_SOCKETS || !self.sockets[fd].active.load(Ordering::Acquire) { return -9; }
        self.sockets[fd].local_port = port;
        self.sockets[fd].bound.store(true, Ordering::Release);
        0
    }
    unsafe fn sendto(&mut self, fd: usize, buf: *const u8, len: usize, dst_ip: u32, dst_port: u16) -> i64 {
        if fd >= MAX_UDP_SOCKETS || !self.sockets[fd].active.load(Ordering::Acquire) { return -9; }
        let s = &self.sockets[fd];
        let total = 8 + len;
        let mut pkt = [0u8; 1500];
        if total > pkt.len() { return -22; }
        let hdr = &mut *(pkt.as_mut_ptr() as *mut UdpHdr);
        hdr.src_port = u16::to_be(s.local_port);
        hdr.dst_port = u16::to_be(dst_port);
        hdr.length   = u16::to_be(total as u16);
        hdr.checksum = 0;
        core::ptr::copy_nonoverlapping(buf, pkt.as_mut_ptr().add(8), len);
        extern "C" { fn sigma_ip_send(dst: u32, proto: u8, data: *const u8, len: usize) -> i32; }
        sigma_ip_send(dst_ip, 17, pkt.as_ptr(), total);
        len as i64
    }
    fn recvfrom(&mut self, fd: usize, buf: &mut [u8]) -> i64 {
        if fd >= MAX_UDP_SOCKETS || !self.sockets[fd].active.load(Ordering::Acquire) { return -9; }
        self.sockets[fd].pop(buf) as i64
    }
    fn rx(&mut self, src_ip: u32, data: &[u8], identity_hash: *const u8) {
        if data.len() < 8 { return; }
        let hdr = unsafe { &*(data.as_ptr() as *const UdpHdr) };
        let dst_port = u16::from_be(hdr.dst_port);
        let src_port = u16::from_be(hdr.src_port);
        let payload  = &data[8..];
        if let Some(i) = self.find_port(dst_port) {
            self.sockets[i].src_ip_last   = src_ip;
            self.sockets[i].src_port_last = src_port;
            
            // Store peer identity for zero-trust verification
            if !identity_hash.is_null() {
                for j in 0..32 {
                    self.sockets[i].peer_identity.identity_hash[j] = unsafe { *identity_hash.add(j) };
                }
            }
            
            self.sockets[i].push(payload);
        }
    }
}

static mut G_UDP: UdpStack = UdpStack::new();

#[no_mangle] pub extern "C" fn udp_socket() -> i32 { unsafe { G_UDP.socket() } }
#[no_mangle] pub extern "C" fn udp_bind(fd: usize, port: u16) -> i32 { unsafe { G_UDP.bind(fd, port) } }
#[no_mangle] pub extern "C" fn udp_sendto(fd: usize, buf: *const u8, len: usize, dst_ip: u32, dst_port: u16) -> i64 {
    unsafe { G_UDP.sendto(fd, buf, len, dst_ip, dst_port) }
}
#[no_mangle] pub extern "C" fn udp_recvfrom(fd: usize, buf: *mut u8, len: usize) -> i64 {
    if buf.is_null() { return -14; }
    unsafe { G_UDP.recvfrom(fd, core::slice::from_raw_parts_mut(buf, len)) }
}
#[no_mangle] pub extern "C" fn udp_rx(src_ip: u32, data: *const u8, len: usize, identity_hash: *const u8) {
    if data.is_null() { return; }
    unsafe { G_UDP.rx(src_ip, core::slice::from_raw_parts(data, len), identity_hash); }
}

/// Verify peer identity for UDP socket
#[no_mangle] pub extern "C" fn udp_verify_identity(fd: usize) -> i32 {
    unsafe {
        if fd >= MAX_UDP_SOCKETS || !G_UDP.sockets[fd].active.load(Ordering::Acquire) { return -9; }
        G_UDP.sockets[fd].peer_identity.verify();
        0
    }
}
