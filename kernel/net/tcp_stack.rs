//! SigmaOS — TCP/IPv4 Network Stack
//! Pure no_std, zero-dependency implementation.
//! Ethernet → ARP → IPv4 → TCP with connection state machine.

#![no_std]
#![allow(dead_code, non_snake_case)]

type U8    = u8;
type U16   = u16;
type U32   = u32;
type U64   = u64;
type I32   = i32;
type Usize = usize;
type Bool  = bool;

// ── Byte-order helpers ────────────────────────────────────────────────────────
#[inline] fn htons(v: U16) -> U16 { v.to_be() }
#[inline] fn ntohs(v: U16) -> U16 { U16::from_be(v) }
#[inline] fn htonl(v: U32) -> U32 { v.to_be() }
#[inline] fn ntohl(v: U32) -> U32 { U32::from_be(v) }

// ── IP / Ethernet headers ─────────────────────────────────────────────────────
#[repr(C, packed)]
pub struct EthHeader {
    pub dst_mac: [U8; 6],
    pub src_mac: [U8; 6],
    pub ether_type: U16,   // big-endian
}
pub const ETHER_TYPE_IP4:  U16 = 0x0800u16.to_be();
pub const ETHER_TYPE_ARP:  U16 = 0x0806u16.to_be();

#[repr(C, packed)]
pub struct Ipv4Header {
    pub ver_ihl:  U8,   // version (4) | ihl (5 normally)
    pub dscp_ecn: U8,
    pub total_len:U16,
    pub id:       U16,
    pub flags_frag:U16,
    pub ttl:      U8,
    pub protocol: U8,
    pub checksum: U16,
    pub src_ip:   U32,
    pub dst_ip:   U32,
}
pub const IP_PROTO_ICMP: U8 = 1;
pub const IP_PROTO_TCP:  U8 = 6;
pub const IP_PROTO_UDP:  U8 = 17;

#[repr(C, packed)]
pub struct TcpHeader {
    pub src_port:  U16,
    pub dst_port:  U16,
    pub seq:       U32,
    pub ack:       U32,
    pub data_off:  U8,   // upper 4 bits = offset in 32-bit words
    pub flags:     U8,
    pub window:    U16,
    pub checksum:  U16,
    pub urgent:    U16,
}
// TCP flags
pub const TCP_FIN: U8 = 0x01;
pub const TCP_SYN: U8 = 0x02;
pub const TCP_RST: U8 = 0x04;
pub const TCP_PSH: U8 = 0x08;
pub const TCP_ACK: U8 = 0x10;

#[repr(C, packed)]
pub struct UdpHeader {
    pub src_port: U16,
    pub dst_port: U16,
    pub length:   U16,
    pub checksum: U16,
}

#[repr(C, packed)]
pub struct ArpPacket {
    pub hw_type:    U16,
    pub proto_type: U16,
    pub hw_len:     U8,
    pub proto_len:  U8,
    pub operation:  U16,   // 1=request, 2=reply
    pub sender_mac: [U8; 6],
    pub sender_ip:  U32,
    pub target_mac: [U8; 6],
    pub target_ip:  U32,
}

// ── Checksum ──────────────────────────────────────────────────────────────────
pub fn inet_checksum(data: &[U8]) -> U16 {
    let mut sum: U32 = 0;
    let mut i = 0;
    while i + 1 < data.len() {
        let word = (data[i] as U32) << 8 | data[i + 1] as U32;
        sum = sum.wrapping_add(word);
        i += 2;
    }
    if i < data.len() { sum = sum.wrapping_add((data[i] as U32) << 8); }
    while sum >> 16 != 0 { sum = (sum & 0xFFFF) + (sum >> 16); }
    !(sum as U16)
}

/// TCP/UDP pseudo-header checksum contribution.
pub fn pseudo_checksum(src: U32, dst: U32, proto: U8, len: U16) -> U32 {
    let mut sum: U32 = 0;
    sum = sum.wrapping_add((src >> 16) & 0xFFFF);
    sum = sum.wrapping_add(src & 0xFFFF);
    sum = sum.wrapping_add((dst >> 16) & 0xFFFF);
    sum = sum.wrapping_add(dst & 0xFFFF);
    sum = sum.wrapping_add(proto as U32);
    sum = sum.wrapping_add(len as U32);
    sum
}

// ── TCP Connection state ──────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TcpState {
    Closed      = 0,
    Listen      = 1,
    SynSent     = 2,
    SynReceived = 3,
    Established = 4,
    FinWait1    = 5,
    FinWait2    = 6,
    CloseWait   = 7,
    Closing     = 8,
    LastAck     = 9,
    TimeWait    = 10,
}

const TX_BUF_SIZE: Usize = 65536;
const RX_BUF_SIZE: Usize = 65536;
const MAX_CONNECTIONS: Usize = 256;

#[repr(C)]
pub struct TcpConn {
    pub state:       TcpState,
    pub local_ip:    U32,
    pub remote_ip:   U32,
    pub local_port:  U16,
    pub remote_port: U16,
    pub snd_nxt:     U32,   // next sequence to send
    pub snd_una:     U32,   // oldest unacknowledged seq
    pub rcv_nxt:     U32,   // expected next receive
    pub snd_wnd:     U16,
    pub rcv_wnd:     U16,
    pub tx_buf:      [U8; TX_BUF_SIZE],
    pub tx_head:     U32,
    pub tx_tail:     U32,
    pub rx_buf:      [U8; RX_BUF_SIZE],
    pub rx_head:     U32,
    pub rx_tail:     U32,
}

impl TcpConn {
    pub const fn zero() -> Self {
        TcpConn {
            state: TcpState::Closed,
            local_ip: 0, remote_ip: 0,
            local_port: 0, remote_port: 0,
            snd_nxt: 0, snd_una: 0, rcv_nxt: 0,
            snd_wnd: 65535, rcv_wnd: 65535,
            tx_buf: [0u8; TX_BUF_SIZE], tx_head: 0, tx_tail: 0,
            rx_buf: [0u8; RX_BUF_SIZE], rx_head: 0, rx_tail: 0,
        }
    }

    pub fn rx_bytes_available(&self) -> Usize {
        self.rx_tail.wrapping_sub(self.rx_head) as Usize
    }

    pub fn push_rx(&mut self, data: &[U8]) -> Usize {
        let mut written = 0;
        for &b in data {
            let next = (self.rx_tail.wrapping_add(1)) as Usize & (RX_BUF_SIZE - 1);
            if next == self.rx_head as Usize { break; }
            self.rx_buf[self.rx_tail as Usize & (RX_BUF_SIZE - 1)] = b;
            self.rx_tail = self.rx_tail.wrapping_add(1);
            written += 1;
        }
        written
    }

    pub fn pop_rx(&mut self, out: &mut [U8]) -> Usize {
        let available = self.rx_bytes_available();
        let take = out.len().min(available);
        for i in 0..take {
            out[i] = self.rx_buf[self.rx_head as Usize & (RX_BUF_SIZE - 1)];
            self.rx_head = self.rx_head.wrapping_add(1);
        }
        take
    }
}

static mut CONNECTIONS: [TcpConn; MAX_CONNECTIONS] =
    [TcpConn::zero(); MAX_CONNECTIONS];

// ── Packet builder ────────────────────────────────────────────────────────────
const PKT_BUF_SIZE: Usize = 2048;
static mut PKT_BUF: [U8; PKT_BUF_SIZE] = [0u8; PKT_BUF_SIZE];

/// Build a TCP segment into `PKT_BUF` and return its length.
/// The caller must transmit it via the NIC driver.
unsafe fn build_tcp_segment(
    conn_idx: Usize, flags: U8, payload: &[U8],
) -> Usize {
    let conn = &mut CONNECTIONS[conn_idx];
    let buf = &mut PKT_BUF;

    // Ethernet header (14 bytes) — MACs left as zero; filled by ARP layer
    let eth_len = core::mem::size_of::<EthHeader>();
    let ip_len  = core::mem::size_of::<Ipv4Header>();
    let tcp_len = core::mem::size_of::<TcpHeader>();
    let total   = eth_len + ip_len + tcp_len + payload.len();
    if total > PKT_BUF_SIZE { return 0; }

    // IP header
    let ip_off = eth_len;
    buf[ip_off]     = 0x45; // version=4, IHL=5
    buf[ip_off + 1] = 0;
    let ip_total = (ip_len + tcp_len + payload.len()) as U16;
    buf[ip_off + 2..ip_off + 4].copy_from_slice(&htons(ip_total).to_ne_bytes());
    buf[ip_off + 8] = 64;              // TTL
    buf[ip_off + 9] = IP_PROTO_TCP;
    buf[ip_off + 12..ip_off + 16].copy_from_slice(&conn.local_ip.to_be_bytes());
    buf[ip_off + 16..ip_off + 20].copy_from_slice(&conn.remote_ip.to_be_bytes());
    // IP checksum
    let ck = inet_checksum(&buf[ip_off..ip_off + ip_len]);
    buf[ip_off + 10..ip_off + 12].copy_from_slice(&ck.to_be_bytes());

    // TCP header
    let tcp_off = ip_off + ip_len;
    buf[tcp_off..tcp_off + 2].copy_from_slice(&htons(conn.local_port).to_ne_bytes());
    buf[tcp_off + 2..tcp_off + 4].copy_from_slice(&htons(conn.remote_port).to_ne_bytes());
    buf[tcp_off + 4..tcp_off + 8].copy_from_slice(&htonl(conn.snd_nxt).to_ne_bytes());
    buf[tcp_off + 8..tcp_off + 12].copy_from_slice(&htonl(conn.rcv_nxt).to_ne_bytes());
    buf[tcp_off + 12] = 0x50;         // data offset = 5 (20 bytes)
    buf[tcp_off + 13] = flags;
    buf[tcp_off + 14..tcp_off + 16].copy_from_slice(&htons(conn.rcv_wnd).to_ne_bytes());

    // Payload
    let pay_off = tcp_off + tcp_len;
    buf[pay_off..pay_off + payload.len()].copy_from_slice(payload);

    // TCP checksum (with pseudo-header)
    let seg_len = (tcp_len + payload.len()) as U16;
    let mut psum = pseudo_checksum(
        conn.local_ip, conn.remote_ip, IP_PROTO_TCP, seg_len,
    );
    let mut i = tcp_off;
    while i + 1 < tcp_off + tcp_len + payload.len() {
        psum = psum.wrapping_add((buf[i] as U32) << 8 | buf[i + 1] as U32);
        i += 2;
    }
    if i < tcp_off + tcp_len + payload.len() {
        psum = psum.wrapping_add((buf[i] as U32) << 8);
    }
    while psum >> 16 != 0 { psum = (psum & 0xFFFF) + (psum >> 16); }
    let tcp_ck = !(psum as U16);
    buf[tcp_off + 16..tcp_off + 18].copy_from_slice(&tcp_ck.to_be_bytes());

    if flags & TCP_SYN != 0 || flags & TCP_FIN != 0 {
        conn.snd_nxt = conn.snd_nxt.wrapping_add(1);
    }
    conn.snd_nxt = conn.snd_nxt.wrapping_add(payload.len() as U32);
    total
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Allocate a TCP connection slot. Returns index or usize::MAX.
#[no_mangle]
pub unsafe extern "C" fn tcp_connect(
    local_ip: U32, remote_ip: U32, local_port: U16, remote_port: U16,
) -> Usize {
    for i in 0..MAX_CONNECTIONS {
        if CONNECTIONS[i].state == TcpState::Closed {
            CONNECTIONS[i] = TcpConn::zero();
            CONNECTIONS[i].local_ip    = local_ip;
            CONNECTIONS[i].remote_ip   = remote_ip;
            CONNECTIONS[i].local_port  = local_port;
            CONNECTIONS[i].remote_port = remote_port;
            CONNECTIONS[i].snd_nxt     = 0xDEAD_BEEF; // ISN
            CONNECTIONS[i].state       = TcpState::SynSent;
            // Caller must transmit the SYN packet from PKT_BUF
            build_tcp_segment(i, TCP_SYN, &[]);
            return i;
        }
    }
    usize::MAX
}

/// Process a received TCP segment. Handles state machine transitions.
#[no_mangle]
pub unsafe extern "C" fn tcp_process_segment(
    conn_idx: Usize, pkt: *const U8, pkt_len: Usize,
) -> I32 {
    if conn_idx >= MAX_CONNECTIONS || pkt.is_null() { return -1; }
    let eth_len = core::mem::size_of::<EthHeader>();
    let ip_len  = core::mem::size_of::<Ipv4Header>();
    let tcp_off = eth_len + ip_len;
    if pkt_len < tcp_off + core::mem::size_of::<TcpHeader>() { return -1; }

    let pkt_slice = core::slice::from_raw_parts(pkt, pkt_len);
    let flags = pkt_slice[tcp_off + 13];
    let seq   = U32::from_be_bytes([
        pkt_slice[tcp_off + 4], pkt_slice[tcp_off + 5],
        pkt_slice[tcp_off + 6], pkt_slice[tcp_off + 7],
    ]);
    let ack_num = U32::from_be_bytes([
        pkt_slice[tcp_off + 8], pkt_slice[tcp_off + 9],
        pkt_slice[tcp_off + 10], pkt_slice[tcp_off + 11],
    ]);
    let data_off = ((pkt_slice[tcp_off + 12] >> 4) as Usize) * 4;
    let pay_off = tcp_off + data_off;
    let payload = if pkt_len > pay_off { &pkt_slice[pay_off..] } else { &[] };

    let conn = &mut CONNECTIONS[conn_idx];
    match conn.state {
        TcpState::SynSent => {
            if flags & TCP_SYN != 0 && flags & TCP_ACK != 0 {
                conn.rcv_nxt = seq.wrapping_add(1);
                conn.snd_una = ack_num;
                conn.state   = TcpState::Established;
                build_tcp_segment(conn_idx, TCP_ACK, &[]);
            }
        }
        TcpState::Established => {
            if flags & TCP_FIN != 0 {
                conn.rcv_nxt = seq.wrapping_add(1);
                conn.state   = TcpState::CloseWait;
                build_tcp_segment(conn_idx, TCP_ACK, &[]);
            } else if !payload.is_empty() {
                conn.rcv_nxt = conn.rcv_nxt.wrapping_add(payload.len() as U32);
                conn.push_rx(payload);
                build_tcp_segment(conn_idx, TCP_ACK, &[]);
            }
        }
        TcpState::CloseWait => {
            conn.state = TcpState::LastAck;
            build_tcp_segment(conn_idx, TCP_FIN | TCP_ACK, &[]);
        }
        TcpState::FinWait1 => {
            if flags & TCP_ACK != 0 { conn.state = TcpState::FinWait2; }
        }
        TcpState::FinWait2 => {
            if flags & TCP_FIN != 0 {
                conn.state = TcpState::TimeWait;
                build_tcp_segment(conn_idx, TCP_ACK, &[]);
            }
        }
        TcpState::LastAck => {
            if flags & TCP_ACK != 0 { conn.state = TcpState::Closed; }
        }
        _ => {}
    }
    0
}

/// Read received data from a connection's RX buffer.
#[no_mangle]
pub unsafe extern "C" fn tcp_read(
    conn_idx: Usize, buf: *mut U8, len: Usize,
) -> I32 {
    if conn_idx >= MAX_CONNECTIONS || buf.is_null() { return -1; }
    let out = core::slice::from_raw_parts_mut(buf, len);
    CONNECTIONS[conn_idx].pop_rx(out) as I32
}

/// Send data on an established connection. Returns bytes queued or -1.
#[no_mangle]
pub unsafe extern "C" fn tcp_send(
    conn_idx: Usize, data: *const U8, len: Usize,
) -> I32 {
    if conn_idx >= MAX_CONNECTIONS || data.is_null() { return -1; }
    if CONNECTIONS[conn_idx].state != TcpState::Established { return -1; }
    let payload = core::slice::from_raw_parts(data, len.min(1448)); // MSS
    build_tcp_segment(conn_idx, TCP_PSH | TCP_ACK, payload) as I32
}

/// Close a connection gracefully (send FIN).
#[no_mangle]
pub unsafe extern "C" fn tcp_close(conn_idx: Usize) {
    if conn_idx >= MAX_CONNECTIONS { return; }
    if CONNECTIONS[conn_idx].state == TcpState::Established {
        CONNECTIONS[conn_idx].state = TcpState::FinWait1;
        build_tcp_segment(conn_idx, TCP_FIN | TCP_ACK, &[]);
    }
}
