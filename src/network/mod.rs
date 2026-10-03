// SigmaOS Network Stack Module
pub mod arp;
pub mod bluetooth;
pub mod device_discovery;
pub mod discovery;
pub mod icmp;
pub mod ip;
pub mod ring_buffer_stack;
pub mod routing;
pub mod security;
pub mod socket;
pub mod sovereign_remote_sharing;
pub mod tcp;
pub mod tcp_complete;
pub mod tcp_udp;
pub mod wireless_manager;
pub mod zenithnet;

pub use device_discovery::{
    DeviceDiscoverySyncEngine, DeviceType, DiscoveredPeerDevice, DiscoveryProtocol,
};
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
// tcp_complete has TcpConnection; tcp module has TcpSocket, TcpError, TcpSegment, TcpState
pub use tcp::{TcpError, TcpSegment, TcpState};
pub use tcp_complete::TcpConnection;
pub use wireless_manager::{BluetoothDevice, WifiProfile, WifiSecurity, WirelessManager};

// ZenithNet TCP/IP Stack
pub use routing::{ForwardingDecision, RouteEntry, RoutingEngine, RoutingTable};
pub use socket::{Socket, SocketError, SocketState, SocketType};
// AddressFamily, SocketOptions, SocketTable, SocketAddr come from network_stubs::* below
pub use arp::{
    ArpCache, ArpCacheEntry, ArpError, ArpHandler, ArpHardwareType, ArpOperation, ArpPacket,
    ArpStats,
};
pub use icmp::{
    IcmpError, IcmpHandler, IcmpHeader, IcmpPacket, IcmpStats, IcmpTimeExceededCode, IcmpType,
    IcmpUnreachableCode,
};
pub use zenithnet::{
    EthernetFrame, IpProtocol, Ipv4Addr, Ipv4Header, MacAddr, NetworkError, NetworkInterface,
    PacketType, TcpHeader, TcpState as ZenithTcpState, UdpHeader, ZenithNet,
};

#[path = "sovereign_async_io.rs"]
pub mod zero_copy_networking;
// IoCompletionEntry, IoCompletionQueue, XdpAction etc. come from network_stubs::* below

pub mod tc_qdisc_sovereign;
pub use tc_qdisc_sovereign::{
    FqCodelQdisc, HtbClass, HtbQdisc, Packet as QdiscPacket, PrioQdisc, TbfQdisc,
};

pub mod wireguard_sovereign;
pub use wireguard_sovereign::{SovereignWireGuardTunnel, WgPeer, WgSessionState};

// ─── Phase 1: Post-Quantum WireGuard Bridge ───────────────────────────────────
pub mod wireguard_pqc_bridge;

pub mod virtual_switch;
pub use virtual_switch::{
    BondingMode, FdbEntry, FlowAction, StpPortState, SwitchPort, SwitchPortMode,
    VirtualSwitchBridge, VirtualSwitchEngine,
};

pub mod quic;

// Re-export network stubs
pub use crate::stubs::network_stubs::*;
