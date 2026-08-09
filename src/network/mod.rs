// SigmaOS Network Stack Module
pub mod ring_buffer_stack;
pub mod tcp;
pub mod tcp_udp;
pub mod wireshark_parity;

pub use ring_buffer_stack::{
    compute_checksum, IPv4Address, NetworkPacket, PacketRingBuffer, TcpSocket,
    TcpState as RingTcpState, ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
};
pub use tcp::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
pub use tcp_udp::{
    Protocol, TCPState as UdpTcpState, NetworkError as UdpNetworkError, Socket, SimpleSocket,
    TCPConnection as UdpTCPConnection, UDPSocket, RenoCongestionControl, BBRCongestionControl,
    FirewallTarget, FirewallChain, ConntrackState, FirewallRule, Firewall as IptablesFirewall,
    SimpleFirewall, ZeroCopy, ZeroCopyNetwork, NetworkStack as CoreNetworkStack, SimpleNetworkStack,
};
pub use wireshark_parity::{PacketCapture, ProtocolType, WiresharkPacket, ProtocolDissector, NetworkStatistics};
