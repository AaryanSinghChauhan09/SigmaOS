// SigmaOS Network Stack Module
pub mod analyzer;
pub mod config;
pub mod enterprise;
pub mod ring_buffer_stack;
pub mod stack;
pub mod tcp;
pub mod legacy_net;
pub mod revival;
pub mod tcp_udp;
pub mod sync;
pub mod torrent;

pub use analyzer::{
    AlertSeverity, AlertType, AlpineZeroAllocCaptureBuffer, AnalysisStrategy, BandwidthAnalysis,
    ClearLinuxFlowLoadBalancer, ConnectionInfo, ConnectionState, GentooUseFlagsDissector,
    KaliPacketFingerprinter, KaliSnoopAnalysis, NetworkTrafficAnalyzer, NixDeclarativeFilter,
    Protocol, SecurityAnalysis, TrafficAlert, TrafficPacket, TrafficStatistics,
};
pub use config::{
    NetworkConfigManager, NetworkInterface, InterfaceType, InterfaceStatus,
    DnsConfig, RouteEntry, NetworkError,
};
pub use enterprise::{EnterpriseNetworkError, IPv6Address, SecureVpnTunnel};
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
pub use sync::{CloudSyncManager, ConflictResolution, SyncConfig, SyncCredentials, SyncError, SyncItem, SyncItemType, SyncProvider, SyncResult, SyncStatus};
pub use torrent::{PeerInfo, TorrentClient, TorrentError, TorrentState};
