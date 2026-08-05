// SigmaOS Network Stack Module
pub mod analyzer;
pub mod enterprise;
pub mod ring_buffer_stack;
pub mod stack;
pub mod tcp;
pub mod tcp_udp;

pub use analyzer::{
    NetworkTrafficAnalyzer, TrafficPacket, Protocol, TrafficStatistics,
    ConnectionInfo, ConnectionState, TrafficAlert, AlertType, AlertSeverity,
    AnalysisStrategy, BandwidthAnalysis, SecurityAnalysis,
    AlpineZeroAllocCaptureBuffer, NixDeclarativeFilter,
    KaliPacketFingerprinter, KaliSnoopAnalysis, GentooUseFlagsDissector,
    ClearLinuxFlowLoadBalancer,
};
pub use enterprise::{EnterpriseNetworkError, IPv6Address, SecureVpnTunnel};
pub use ring_buffer_stack::{
    compute_checksum, IPv4Address, NetworkPacket, PacketRingBuffer, TcpSocket,
    TcpState as RingTcpState, ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
};
pub use tcp::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
