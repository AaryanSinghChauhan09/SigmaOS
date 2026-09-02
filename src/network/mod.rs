// SigmaOS Network Stack Module
pub mod discovery;
pub mod ring_buffer_stack;
pub mod security;
pub mod sovereign_remote_sharing;
pub mod tcp;
pub mod tcp_udp;
pub mod wireless_manager;

pub use discovery::{
    DiscoveredNetworkService, DiscoveryProtocolType, Icmpv6NdpEntry, LlmnrNbnsResolver,
    NetworkDevicePeer, SovereignNetworkDiscoveryEngine, SsdpDiscoveryPacket,
};

pub use ring_buffer_stack::{
    compute_checksum, IPv4Address, NetworkPacket, PacketRingBuffer, TcpSocket,
    TcpState as RingTcpState, ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
};
pub use security::{
    Firewall, FirewallAction, FirewallRule, NetworkProtocol, TlsCipherSuite, TlsConfig, TlsVersion,
};
pub use sovereign_remote_sharing::{
    NfsClientLock, NfsCompoundOp, NfsExportRule, RsyncBlockChecksum, RsyncDeltaInstruction,
    ScpWireMessage, SmbDialect, SmbSession, SmbShareConfig, SovereignNfsEngine,
    SovereignRsyncEngine, SovereignSambaEngine, SovereignScpEngine, SovereignSshEngine,
    SshCertificate, SshMatchRule, SshMultiplexControlMaster,
};
pub use tcp::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
pub use wireless_manager::{BluetoothDevice, WifiProfile, WifiSecurity, WirelessManager};
