// SigmaOS Network Stack Module
pub mod stack;
pub mod tcp;
pub mod enterprise;
pub mod tcp_udp;
pub mod ring_buffer_stack;

pub use enterprise::{EnterpriseNetworkError, IPv6Address, SecureVpnTunnel};
pub use tcp::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
pub use ring_buffer_stack::{
    IPv4Address, NetworkPacket, PacketRingBuffer, TcpSocket, TcpState as RingTcpState,
    compute_checksum, ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
};
