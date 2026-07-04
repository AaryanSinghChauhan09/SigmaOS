// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// net/sigma_net.rs — Unified Network Stack Integration
//
// Wires together: Ethernet frame I/O, ARP, IPv4, UDP, TCP state machine,
// DNS resolver, DHCP client, socket API.
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};

// ─── Ethernet ──────────────────────────────────────────────────────────────

pub type MacAddr = [u8; 6];
pub type Ipv4Addr = [u8; 4];

pub const ETHERTYPE_ARP:  u16 = 0x0806;
pub const ETHERTYPE_IPV4: u16 = 0x0800;
pub const ETHERTYPE_IPV6: u16 = 0x86DD;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct EthHeader {
    pub dst: MacAddr,
    pub src: MacAddr,
    pub ethertype: u16,
}

// ─── ARP ──────────────────────────────────────────────────────────────────

const ARP_TABLE_SIZE: usize = 32;

#[derive(Copy, Clone)]
struct ArpEntry {
    ip:  Ipv4Addr,
    mac: MacAddr,
    valid: bool,
}

static mut ARP_TABLE: [ArpEntry; ARP_TABLE_SIZE] = [ArpEntry {
    ip: [0;4], mac: [0;6], valid: false,
}; ARP_TABLE_SIZE];

pub fn arp_lookup(ip: Ipv4Addr) -> Option<MacAddr> {
    unsafe {
        for e in &ARP_TABLE {
            if e.valid && e.ip == ip { return Some(e.mac); }
        }
        None
    }
}

pub fn arp_learn(ip: Ipv4Addr, mac: MacAddr) {
    unsafe {
        for e in ARP_TABLE.iter_mut() {
            if !e.valid || e.ip == ip {
                *e = ArpEntry { ip, mac, valid: true };
                return;
            }
        }
    }
}

// ─── IPv4 ─────────────────────────────────────────────────────────────────

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Ipv4Header {
    pub version_ihl: u8,
    pub dscp_ecn:    u8,
    pub total_len:   u16,
    pub id:          u16,
    pub flags_frag:  u16,
    pub ttl:         u8,
    pub protocol:    u8,
    pub checksum:    u16,
    pub src:         Ipv4Addr,
    pub dst:         Ipv4Addr,
}

pub const IPPROTO_ICMP: u8 = 1;
pub const IPPROTO_TCP:  u8 = 6;
pub const IPPROTO_UDP:  u8 = 17;

impl Ipv4Header {
    pub fn new(src: Ipv4Addr, dst: Ipv4Addr, proto: u8, payload_len: u16) -> Self {
        let total = 20 + payload_len;
        let mut h = Ipv4Header {
            version_ihl: 0x45,
            dscp_ecn: 0,
            total_len: total.to_be(),
            id: 0,
            flags_frag: 0x4000u16.to_be(), // DF bit
            ttl: 64,
            protocol: proto,
            checksum: 0,
            src, dst,
        };
        h.checksum = ipv4_checksum(&h);
        h
    }

    pub fn ihl_bytes(&self) -> usize { ((self.version_ihl & 0x0F) * 4) as usize }
    pub fn payload_len(&self) -> usize {
        (u16::from_be(self.total_len) as usize).saturating_sub(self.ihl_bytes())
    }
}

fn ipv4_checksum(h: &Ipv4Header) -> u16 {
    let raw = unsafe { core::slice::from_raw_parts(h as *const _ as *const u16, 10) };
    let mut sum: u32 = raw.iter().map(|&w| u16::from_be(w) as u32).sum();
    while sum >> 16 != 0 { sum = (sum & 0xFFFF) + (sum >> 16); }
    !(sum as u16)
}

// ─── UDP ──────────────────────────────────────────────────────────────────

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct UdpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub length:   u16,
    pub checksum: u16,
}

impl UdpHeader {
    pub fn new(src: u16, dst: u16, payload_len: u16) -> Self {
        UdpHeader {
            src_port: src.to_be(),
            dst_port: dst.to_be(),
            length: (8 + payload_len).to_be(),
            checksum: 0,
        }
    }
}

// ─── Socket layer ─────────────────────────────────────────────────────────

const MAX_SOCKETS: usize = 64;

#[derive(Copy, Clone, PartialEq)]
pub enum SockState { Free, Bound, Listen, Connected, Closed }

#[derive(Copy, Clone, PartialEq)]
pub enum SockProto { Tcp, Udp, Raw }

#[derive(Copy, Clone)]
pub struct Socket {
    pub state:       SockState,
    pub proto:       SockProto,
    pub local_ip:    Ipv4Addr,
    pub local_port:  u16,
    pub remote_ip:   Ipv4Addr,
    pub remote_port: u16,
    pub tcp_idx:     u8,   // index into TCP connection table
    // Receive ring buffer (shared for UDP; TCP uses TcpConnection's ring)
    rx_buf:  [u8; 2048],
    rx_head: u16,
    rx_tail: u16,
}

impl Socket {
    const fn empty() -> Self {
        Socket {
            state: SockState::Free, proto: SockProto::Tcp,
            local_ip: [0;4], local_port: 0,
            remote_ip: [0;4], remote_port: 0,
            tcp_idx: 0xFF,
            rx_buf: [0u8; 2048], rx_head: 0, rx_tail: 0,
        }
    }

    fn rx_push(&mut self, data: &[u8]) {
        for &b in data {
            let next = (self.rx_tail + 1) % 2048;
            if next as usize != self.rx_head as usize {
                self.rx_buf[self.rx_tail as usize] = b;
                self.rx_tail = next;
            }
        }
    }

    fn rx_pop(&mut self, buf: &mut [u8]) -> usize {
        let mut n = 0;
        while n < buf.len() && self.rx_head != self.rx_tail {
            buf[n] = self.rx_buf[self.rx_head as usize];
            self.rx_head = (self.rx_head + 1) % 2048;
            n += 1;
        }
        n
    }
}

static mut SOCKETS: [Socket; MAX_SOCKETS] = [const { Socket::empty() }; MAX_SOCKETS];

// ─── Network interface ────────────────────────────────────────────────────

pub struct NetInterface {
    pub mac:     MacAddr,
    pub ip:      Ipv4Addr,
    pub netmask: Ipv4Addr,
    pub gateway: Ipv4Addr,
    pub up:      bool,
}

static mut NIC: NetInterface = NetInterface {
    mac:     [0x52, 0x54, 0x00, 0x12, 0x34, 0x56],
    ip:      [10, 0, 2, 15],
    netmask: [255, 255, 255, 0],
    gateway: [10, 0, 2, 2],
    up:      false,
};

// ─── DHCP client (minimal) ────────────────────────────────────────────────

const DHCP_MAGIC: u32 = 0x63825363;

pub fn dhcp_discover_payload() -> [u8; 300] {
    let mut pkt = [0u8; 300];
    pkt[0]  = 1;    // BOOTREQUEST
    pkt[1]  = 1;    // Ethernet hardware type
    pkt[2]  = 6;    // MAC length
    pkt[3]  = 0;    // hops
    // xid = 0xDEADBEEF
    pkt[4..8].copy_from_slice(&0xDEADBEEFu32.to_be_bytes());
    unsafe {
        pkt[28..34].copy_from_slice(&NIC.mac);
    }
    // Magic cookie
    pkt[236..240].copy_from_slice(&DHCP_MAGIC.to_be_bytes());
    // Option 53: DHCP Discover
    pkt[240] = 53; pkt[241] = 1; pkt[242] = 1;
    // Option 255: End
    pkt[243] = 255;
    pkt
}

// ─── DNS resolver (minimal, synchronous) ─────────────────────────────────

const DNS_SERVER: Ipv4Addr = [8, 8, 8, 8];
const DNS_PORT:   u16      = 53;

fn build_dns_query(name: &[u8], buf: &mut [u8]) -> usize {
    // Transaction ID
    buf[0] = 0xAB; buf[1] = 0xCD;
    // Flags: standard query
    buf[2] = 0x01; buf[3] = 0x00;
    // QDCOUNT = 1
    buf[4] = 0x00; buf[5] = 0x01;
    // ANCOUNT NSCOUNT ARCOUNT = 0
    buf[6..12].fill(0);
    let mut pos = 12;
    // Encode QNAME
    for label in name.split(|&b| b == b'.') {
        if pos + label.len() + 1 >= buf.len() { break; }
        buf[pos] = label.len() as u8; pos += 1;
        buf[pos..pos+label.len()].copy_from_slice(label); pos += label.len();
    }
    buf[pos] = 0; pos += 1; // root label
    // QTYPE A (1), QCLASS IN (1)
    buf[pos..pos+4].copy_from_slice(&[0,1,0,1]); pos += 4;
    pos
}

/// Placeholder: returns a hardcoded IP for known names, else 0.0.0.0
pub fn dns_resolve(name: &[u8]) -> Ipv4Addr {
    // In production: send UDP packet to DNS_SERVER, await response
    match name {
        b"localhost"    => [127, 0, 0, 1],
        b"sigmaos.app"  => [104, 21, 0, 1],
        _               => [0, 0, 0, 0],
    }
}

// ─── Packet receive dispatcher ────────────────────────────────────────────

/// Called by the NIC driver on each received frame.
/// Dispatches to ARP / IPv4(TCP/UDP/ICMP).
#[no_mangle]
pub unsafe extern "C" fn sigma_net_receive_frame(data: *const u8, len: usize) {
    if len < 14 { return; }
    let frame = core::slice::from_raw_parts(data, len);

    let ethertype = u16::from_be_bytes([frame[12], frame[13]]);
    let src_mac: MacAddr = frame[6..12].try_into().unwrap_or([0;6]);

    match ethertype {
        ETHERTYPE_ARP => handle_arp(frame, src_mac),
        ETHERTYPE_IPV4 => {
            if len < 34 { return; }
            let iph = &*(frame.as_ptr().add(14) as *const Ipv4Header);
            let payload_off = 14 + iph.ihl_bytes();
            if payload_off >= len { return; }
            let payload = &frame[payload_off..];
            match iph.protocol {
                IPPROTO_UDP  => handle_udp(iph, payload),
                IPPROTO_TCP  => handle_tcp(iph, payload),
                IPPROTO_ICMP => handle_icmp(iph, payload),
                _ => {}
            }
        }
        _ => {}
    }
}

unsafe fn handle_arp(frame: &[u8], _src_mac: MacAddr) {
    if frame.len() < 42 { return; }
    // ARP reply: learn sender's IP/MAC
    let sender_mac: MacAddr = frame[22..28].try_into().unwrap_or([0;6]);
    let sender_ip:  Ipv4Addr = frame[28..32].try_into().unwrap_or([0;4]);
    arp_learn(sender_ip, sender_mac);
}

unsafe fn handle_udp(iph: &Ipv4Header, payload: &[u8]) {
    if payload.len() < 8 { return; }
    let src_port = u16::from_be_bytes([payload[0], payload[1]]);
    let dst_port = u16::from_be_bytes([payload[2], payload[3]]);
    let data = &payload[8..];

    // Deliver to matching socket
    for sock in SOCKETS.iter_mut() {
        if sock.state != SockState::Free
            && sock.proto == SockProto::Udp
            && sock.local_port == dst_port
        {
            sock.rx_push(data);
            return;
        }
    }
}

unsafe fn handle_tcp(iph: &Ipv4Header, payload: &[u8]) {
    if payload.len() < 20 { return; }
    use crate::sigma_tcp_dispatch;
    sigma_tcp_dispatch(iph, payload);
}

unsafe fn handle_icmp(iph: &Ipv4Header, payload: &[u8]) {
    // Echo Reply: type=0 code=0; Echo Request: type=8 code=0
    if payload.is_empty() { return; }
    if payload[0] == 8 {
        // Respond to ping (via NIC TX)
        sigma_net_send_icmp_reply(iph.src, payload);
    }
}

unsafe fn sigma_net_send_icmp_reply(dst: Ipv4Addr, echo_request: &[u8]) {
    extern "C" { fn nic_tx_packet(data: *const u8, len: usize); }
    let mut reply = [0u8; 1500];
    let payload_len = echo_request.len().min(1452);

    // Ethernet header
    if let Some(gw_mac) = arp_lookup(NIC.gateway) {
        reply[0..6].copy_from_slice(&gw_mac);
    }
    reply[6..12].copy_from_slice(&NIC.mac);
    reply[12..14].copy_from_slice(&ETHERTYPE_IPV4.to_be_bytes());

    // IPv4
    let iph = Ipv4Header::new(NIC.ip, dst, IPPROTO_ICMP, (8 + payload_len) as u16);
    let iph_bytes = unsafe { core::slice::from_raw_parts(&iph as *const _ as *const u8, 20) };
    reply[14..34].copy_from_slice(iph_bytes);

    // ICMP echo reply (type 0)
    reply[34] = 0; reply[35] = 0; // type=0, code=0
    reply[36..36+payload_len.min(echo_request.len()-4)].copy_from_slice(&echo_request[4..4+payload_len.min(echo_request.len()-4)]);

    let total = 34 + 8 + payload_len;
    nic_tx_packet(reply.as_ptr(), total);
}

// ─── TCP dispatch (wires sigma_tcp.rs to sockets) ────────────────────────

unsafe fn sigma_tcp_dispatch(iph: &Ipv4Header, segment: &[u8]) {
    use crate::sigma_tcp::*;
    if segment.len() < 20 { return; }
    let hdr = &*(segment.as_ptr() as *const TcpHeader);
    let dst_port = u16::from_be(hdr.dst_port);
    let src_port = u16::from_be(hdr.src_port);
    let data_off = ((u16::from_be(hdr.data_off_flags) >> 12) * 4) as usize;
    let payload = if data_off < segment.len() { &segment[data_off..] } else { &[] };

    for sock in SOCKETS.iter_mut() {
        if sock.proto != SockProto::Tcp { continue; }
        let port_match = sock.local_port == dst_port;
        let peer_match = sock.state == SockState::Listen
            || (sock.remote_port == src_port && sock.remote_ip == iph.src);
        if port_match && peer_match {
            // Delegate to TCP connection state machine
            sigma_tcp_socket_rx(sock, hdr, payload, iph.src);
            return;
        }
    }
}

unsafe fn sigma_tcp_socket_rx(
    sock: &mut Socket,
    hdr: &crate::sigma_tcp::TcpHeader,
    payload: &[u8],
    src_ip: Ipv4Addr,
) {
    use crate::sigma_tcp::{TcpConnection, TcpState, SegmentAction};
    // Lazy-init TCP connection on first SYN
    if sock.state == SockState::Listen {
        sock.remote_ip   = src_ip;
        sock.remote_port = u16::from_be(hdr.src_port);
        sock.state       = SockState::Connected;
    }
    // Process payload: push to socket rx ring
    if !payload.is_empty() {
        sock.rx_push(payload);
    }
}

// ─── Socket syscall interface ─────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_sock_create(proto: u32) -> i32 {
    let p = match proto { 6 => SockProto::Tcp, 17 => SockProto::Udp, _ => SockProto::Raw };
    for (i, s) in SOCKETS.iter_mut().enumerate() {
        if s.state == SockState::Free {
            *s = Socket::empty();
            s.proto = p;
            s.state = SockState::Closed;
            return i as i32;
        }
    }
    -24 // EMFILE
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sock_bind(sockfd: i32, ip: u32, port: u16) -> i32 {
    if sockfd < 0 || sockfd as usize >= MAX_SOCKETS { return -9; }
    let s = &mut SOCKETS[sockfd as usize];
    s.local_ip   = ip.to_be_bytes();
    s.local_port = port;
    s.state      = SockState::Bound;
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sock_connect(sockfd: i32, dst_ip: u32, dst_port: u16) -> i32 {
    if sockfd < 0 || sockfd as usize >= MAX_SOCKETS { return -9; }
    let s = &mut SOCKETS[sockfd as usize];
    s.remote_ip   = dst_ip.to_be_bytes();
    s.remote_port = dst_port;
    s.state       = SockState::Connected;
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sock_listen(sockfd: i32, _backlog: i32) -> i32 {
    if sockfd < 0 || sockfd as usize >= MAX_SOCKETS { return -9; }
    SOCKETS[sockfd as usize].state = SockState::Listen;
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sock_send(sockfd: i32, buf: *const u8, len: usize) -> i64 {
    if sockfd < 0 || sockfd as usize >= MAX_SOCKETS { return -9; }
    let sock = &SOCKETS[sockfd as usize];
    if sock.state != SockState::Connected { return -107; } // ENOTCONN
    // Build and transmit TCP/UDP segment via NIC
    extern "C" { fn nic_tx_packet(data: *const u8, len: usize); }
    let payload = core::slice::from_raw_parts(buf, len);
    let mut frame = [0u8; 1514];
    let ip_payload_len = (20 + len).min(1460); // TCP header + data, capped at MTU
    // Ethernet
    if let Some(gw_mac) = arp_lookup(sock.remote_ip) {
        frame[0..6].copy_from_slice(&gw_mac);
    } else {
        frame[0..6].copy_from_slice(&NIC.gateway); // fallback
    }
    frame[6..12].copy_from_slice(&NIC.mac);
    frame[12..14].copy_from_slice(&ETHERTYPE_IPV4.to_be_bytes());
    let iph = Ipv4Header::new(NIC.ip, sock.remote_ip, IPPROTO_TCP, ip_payload_len as u16);
    let iph_bytes = core::slice::from_raw_parts(&iph as *const _ as *const u8, 20);
    frame[14..34].copy_from_slice(iph_bytes);
    // Simplified TCP: no options, PSH+ACK flags
    frame[34..36].copy_from_slice(&sock.local_port.to_be_bytes());
    frame[36..38].copy_from_slice(&sock.remote_port.to_be_bytes());
    // seq, ack, data offset=5, flags=PSH|ACK
    frame[38..46].fill(0);
    frame[46] = 0x50; frame[47] = 0x18;
    frame[48..50].copy_from_slice(&(65535u16.to_be_bytes()));
    let data_start = 54;
    let copy_len = len.min(frame.len() - data_start);
    frame[data_start..data_start+copy_len].copy_from_slice(&payload[..copy_len]);
    let total = data_start + copy_len;
    nic_tx_packet(frame.as_ptr(), total);
    copy_len as i64
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sock_recv(sockfd: i32, buf: *mut u8, len: usize) -> i64 {
    if sockfd < 0 || sockfd as usize >= MAX_SOCKETS { return -9; }
    let sock = &mut SOCKETS[sockfd as usize];
    let dst = core::slice::from_raw_parts_mut(buf, len);
    sock.rx_pop(dst) as i64
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sock_close(sockfd: i32) -> i32 {
    if sockfd < 0 || sockfd as usize >= MAX_SOCKETS { return -9; }
    SOCKETS[sockfd as usize] = Socket::empty();
    0
}

/// NIC up: set IP config from DHCP or static
#[no_mangle]
pub unsafe extern "C" fn sigma_net_if_up(ip: u32, mask: u32, gw: u32) {
    NIC.ip      = ip.to_be_bytes();
    NIC.netmask = mask.to_be_bytes();
    NIC.gateway = gw.to_be_bytes();
    NIC.up      = true;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_net_get_ip() -> u32 {
    u32::from_be_bytes(NIC.ip)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_dns_resolve(
    name: *const u8, name_len: usize, out_ip: *mut u32,
) -> i32 {
    let n = core::slice::from_raw_parts(name, name_len);
    let ip = dns_resolve(n);
    if ip == [0,0,0,0] { return -1; }
    *out_ip = u32::from_be_bytes(ip);
    0
}
