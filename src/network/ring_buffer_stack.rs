#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Asynchronous Ring-Buffer TCP/UDP Networking Stack Layer
// Zero-dependency, // #![no_std]  // crate-root only compliant, and allocation-free networking stack.

pub const ETHERNET_HEADER_LEN: usize = 14;
pub const IPV4_HEADER_LEN: usize = 20;
pub const TCP_HEADER_LEN: usize = 20;
pub const UDP_HEADER_LEN: usize = 8;

pub const PACKET_BUFFER_SIZE: usize = 2048;
pub const RING_BUFFER_CAPACITY: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAck,
    TimeWait,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IPv4Address(pub [u8; 4]);

#[derive(Debug, Clone, Copy)]
pub struct NetworkPacket {
    pub buffer: [u8; PACKET_BUFFER_SIZE],
    pub length: usize,
}

pub struct PacketRingBuffer {
    packets: [Option<NetworkPacket>; RING_BUFFER_CAPACITY],
    head: usize,
    tail: usize,
    count: usize,
}

impl PacketRingBuffer {
    pub const fn new() -> Self {
        Self {
            packets: [None; RING_BUFFER_CAPACITY],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn push(&mut self, packet: NetworkPacket) -> Result<(), &'static str> {
        if self.count >= RING_BUFFER_CAPACITY {
            return Err("Ring buffer overflow");
        }
        self.packets[self.tail] = Some(packet);
        self.tail = (self.tail + 1) % RING_BUFFER_CAPACITY;
        self.count += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<NetworkPacket> {
        if self.count == 0 {
            return None;
        }
        let packet = self.packets[self.head].take();
        self.head = (self.head + 1) % RING_BUFFER_CAPACITY;
        self.count -= 1;
        packet
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

/// Compute standard Internet Checksum over header slices
pub fn compute_checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let mut i = 0;
    while i < data.len() - 1 {
        let word = ((data[i] as u16) << 8) | (data[i + 1] as u16);
        sum += word as u32;
        i += 2;
    }
    if i < data.len() {
        sum += (data[i] as u32) << 8;
    }
    while (sum >> 16) != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !(sum as u16)
}

pub struct TcpSocket {
    pub local_port: u16,
    pub remote_port: u16,
    pub local_ip: IPv4Address,
    pub remote_ip: IPv4Address,
    pub state: TcpState,
    pub seq_number: u32,
    pub ack_number: u32,
}

impl TcpSocket {
    pub fn new(local_port: u16, local_ip: IPv4Address) -> Self {
        Self {
            local_port,
            remote_port: 0,
            local_ip,
            remote_ip: IPv4Address([0; 4]),
            state: TcpState::Closed,
            seq_number: 1000,
            ack_number: 0,
        }
    }

    /// Process incoming segment to drive TCP state machine asynchronously
    pub fn process_segment(
        &mut self,
        flags: u8,
        seq: u32,
        ack: u32,
        payload_len: usize,
    ) -> Option<NetworkPacket> {
        match self.state {
            TcpState::Closed => None,
            TcpState::Listen => {
                if flags & 0x02 != 0 {
                    // SYN
                    self.state = TcpState::SynReceived;
                    self.ack_number = seq + 1;
                    // Prepare SYN-ACK response
                    self.send_packet(0x12) // SYN-ACK
                } else {
                    None
                }
            }
            TcpState::SynSent => {
                if flags & 0x12 == 0x12 {
                    // SYN-ACK
                    self.state = TcpState::Established;
                    self.ack_number = seq + 1;
                    self.seq_number = ack;
                    self.send_packet(0x10) // ACK
                } else {
                    None
                }
            }
            TcpState::Established => {
                if flags & 0x01 != 0 {
                    // FIN
                    self.state = TcpState::CloseWait;
                    self.ack_number = seq + 1;
                    self.send_packet(0x10) // ACK
                } else {
                    self.ack_number += payload_len as u32;
                    None
                }
            }
            _ => None,
        }
    }

    fn send_packet(&self, flags: u8) -> Option<NetworkPacket> {
        let mut packet = NetworkPacket {
            buffer: [0; PACKET_BUFFER_SIZE],
            length: ETHERNET_HEADER_LEN + IPV4_HEADER_LEN + TCP_HEADER_LEN,
        };
        // Fill TCP Header flags
        packet.buffer[ETHERNET_HEADER_LEN + IPV4_HEADER_LEN + 13] = flags;
        // Local/Remote ports
        packet.buffer[ETHERNET_HEADER_LEN + IPV4_HEADER_LEN + 0] = (self.local_port >> 8) as u8;
        packet.buffer[ETHERNET_HEADER_LEN + IPV4_HEADER_LEN + 1] = (self.local_port & 0xFF) as u8;
        packet.buffer[ETHERNET_HEADER_LEN + IPV4_HEADER_LEN + 2] = (self.remote_port >> 8) as u8;
        packet.buffer[ETHERNET_HEADER_LEN + IPV4_HEADER_LEN + 3] = (self.remote_port & 0xFF) as u8;

        Some(packet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer_networking_stack() {
        let mut ring = PacketRingBuffer::new();
        assert_eq!(ring.count(), 0);

        let packet = NetworkPacket {
            buffer: [0; PACKET_BUFFER_SIZE],
            length: 100,
        };
        assert!(ring.push(packet).is_ok());
        assert_eq!(ring.count(), 1);

        let popped = ring.pop().unwrap();
        assert_eq!(popped.length, 100);
        assert_eq!(ring.count(), 0);
    }

    #[test]
    fn test_checksum_and_tcp_state_machine() {
        let data = [
            0x45u8, 0x00, 0x00, 0x28, 0x1A, 0x2B, 0x40, 0x00, 0x40, 0x06, 0x00, 0x00,
        ];
        let csum = compute_checksum(&data);
        assert_ne!(csum, 0);

        let local_ip = IPv4Address([192, 168, 1, 1]);
        let mut socket = TcpSocket::new(80, local_ip);
        socket.state = TcpState::Listen;

        // Process incoming SYN
        let response = socket.process_segment(0x02, 500, 0, 0).unwrap();
        assert_eq!(socket.state, TcpState::SynReceived);
        assert_eq!(socket.ack_number, 501);
        assert_eq!(
            response.buffer[ETHERNET_HEADER_LEN + IPV4_HEADER_LEN + 13],
            0x12
        ); // SYN-ACK
    }
}
