// SigmaOS Network Stack Module
pub mod analyzer;
pub mod enterprise;
pub mod tcp;
pub mod ring_buffer_stack;
pub use ring_buffer_stack::{
    IPv4Address, NetworkPacket, PacketRingBuffer, TcpSocket, TcpState as RingTcpState,
    ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN, compute_checksum,
};

pub use analyzer::{
    NetworkTrafficAnalyzer, TrafficPacket, Protocol, TrafficStatistics,
    ConnectionInfo, ConnectionState, TrafficAlert, AlertType, AlertSeverity,
    AnalysisStrategy, BandwidthAnalysis, SecurityAnalysis,
    AlpineZeroAllocCaptureBuffer, NixDeclarativeFilter,
    KaliPacketFingerprinter, KaliSnoopAnalysis, GentooUseFlagsDissector,
    ClearLinuxFlowLoadBalancer,
};
pub use enterprise::{EnterpriseNetworkError, IPv6Address, SecureVpnTunnel};
pub use tcp::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
