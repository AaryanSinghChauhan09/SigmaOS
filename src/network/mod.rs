// SigmaOS Network Stack Module
pub mod ring_buffer_stack;
pub mod tcp;
pub mod legacy_net;
pub mod revival;

pub use ring_buffer_stack::{
    compute_checksum, IPv4Address, NetworkPacket, PacketRingBuffer, TcpSocket,
    TcpState as RingTcpState, ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
};
pub use tcp::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
pub use legacy_net::{
    LegacyProtocol, LegacyProtocolAdapter,
};
pub use revival::{
    RevivalProtocol, NetRevival,
};
