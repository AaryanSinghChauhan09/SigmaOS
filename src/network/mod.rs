// SigmaOS Network Stack Module
pub mod stack;
pub mod enterprise;
pub mod analyzer;
pub mod tcp;
pub mod tcp_udp;
pub mod wireless;
pub mod zero_trust;
pub mod legacy_net;
pub mod revival;

pub use enterprise::{EnterpriseNetworkError, IPv6Address, SecureVpnTunnel};
pub use analyzer::{
    NetworkTrafficAnalyzer, TrafficPacket, Protocol, TrafficStatistics,
    ConnectionInfo, ConnectionState, TrafficAlert, AlertType, AlertSeverity,
    AnalysisStrategy, BandwidthAnalysis, SecurityAnalysis,
    AlpineZeroAllocCaptureBuffer, NixDeclarativeFilter,
    KaliPacketFingerprinter, KaliSnoopAnalysis, GentooUseFlagsDissector,
    ClearLinuxFlowLoadBalancer,
};
pub use tcp::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
pub use legacy_net::{
    LegacyProtocol, LegacyProtocolAdapter,
};
pub use revival::{
    RevivalProtocol, NetRevival,
};
