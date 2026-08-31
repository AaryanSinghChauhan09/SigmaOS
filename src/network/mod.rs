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
pub mod dhcp_dns;

pub use dhcp_dns::{CloudSyncEngine, DhcpClient, DhcpState, DnsResolver};

pub use enterprise::{EnterpriseNetworkError, IPv6Address, SecureVpnTunnel};
pub use analyzer::{
    TrafficPacket, Protocol, TrafficAlert, AlertType, AlertSeverity,
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
