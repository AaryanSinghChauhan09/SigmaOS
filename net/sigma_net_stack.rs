//! sigma_net_stack.rs — Zero-copy network stack skeleton (no_std Rust)
//! Provides Ethernet frame parsing and a minimal IPv4 + UDP handler.
//! No libc, no smoltcp, no lwIP — custom raw packet parsing only.

#![no_std]
#![allow(dead_code)]

pub const ETH_HDR_LEN: usize = 14;
pub const ETHERTYPE_IPV4: u16 = 0x0800;
pub const ETHERTYPE_ARP:  u16 = 0x0806;
pub const IP_PROTO_UDP:   u8  = 17;
pub const IP_PROTO_TCP:   u8  = 6;
pub const UDP_HDR_LEN:    usize = 8;
pub const IPV4_HDR_MIN:   usize = 20;

#[repr(C, packed)]
pub struct EthHeader {
    pub dst_mac:   [u8; 6],
    pub src_mac:   [u8; 6],
    pub ethertype: [u8; 2],
}

impl EthHeader {
    pub fn ethertype_u16(&self) -> u16 { u16::from_be_bytes(self.ethertype) }
}

pub fn parse_eth(frame: &[u8]) -> Option<(&EthHeader, &[u8])> {
    if frame.len() < ETH_HDR_LEN { return None; }
    let hdr = unsafe { &*(frame.as_ptr() as *const EthHeader) };
    Some((hdr, &frame[ETH_HDR_LEN..]))
}

#[repr(C, packed)]
pub struct Ipv4Header {
    pub ver_ihl:    u8,
    pub dscp_ecn:   u8,
    pub total_len:  [u8; 2],
    pub ident:      [u8; 2],
    pub flags_frag: [u8; 2],
    pub ttl:        u8,
    pub protocol:   u8,
    pub checksum:   [u8; 2],
    pub src_ip:     [u8; 4],
    pub dst_ip:     [u8; 4],
}

impl Ipv4Header {
    pub fn ihl_bytes(&self) -> usize { ((self.ver_ihl & 0x0F) as usize) * 4 }
}

pub fn parse_ipv4(payload: &[u8]) -> Option<(&Ipv4Header, &[u8])> {
    if payload.len() < IPV4_HDR_MIN { return None; }
    let hdr = unsafe { &*(payload.as_ptr() as *const Ipv4Header) };
    let ihl = hdr.ihl_bytes();
    if payload.len() < ihl { return None; }
    Some((hdr, &payload[ihl..]))
}

#[repr(C, packed)]
pub struct UdpHeader {
    pub src_port: [u8; 2],
    pub dst_port: [u8; 2],
    pub length:   [u8; 2],
    pub checksum: [u8; 2],
}

impl UdpHeader {
    pub fn src_port(&self) -> u16 { u16::from_be_bytes(self.src_port) }
    pub fn dst_port(&self) -> u16 { u16::from_be_bytes(self.dst_port) }
}

pub fn parse_udp(payload: &[u8]) -> Option<(&UdpHeader, &[u8])> {
    if payload.len() < UDP_HDR_LEN { return None; }
    let hdr = unsafe { &*(payload.as_ptr() as *const UdpHeader) };
    Some((hdr, &payload[UDP_HDR_LEN..]))
}

pub enum Packet<'a> {
    Udp { src_ip: [u8; 4], dst_ip: [u8; 4], src_port: u16, dst_port: u16, data: &'a [u8] },
    Arp,
    Unknown,
}

pub fn dispatch(frame: &[u8]) -> Packet<'_> {
    let Some((eth, rest)) = parse_eth(frame) else { return Packet::Unknown; };
    match eth.ethertype_u16() {
        ETHERTYPE_ARP  => Packet::Arp,
        ETHERTYPE_IPV4 => {
            let Some((ip, rest2)) = parse_ipv4(rest) else { return Packet::Unknown; };
            if ip.protocol == IP_PROTO_UDP {
                let Some((udp, data)) = parse_udp(rest2) else { return Packet::Unknown; };
                Packet::Udp { src_ip: ip.src_ip, dst_ip: ip.dst_ip,
                              src_port: udp.src_port(), dst_port: udp.dst_port(), data }
            } else { Packet::Unknown }
        }
        _ => Packet::Unknown,
    }
}
