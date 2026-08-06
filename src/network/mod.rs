// SigmaOS Network Stack Module
pub mod analyzer;
pub mod enterprise;
pub mod tcp;
pub mod tcp_udp;
pub mod ring_buffer_stack;
pub mod pf_firewall;
pub mod nftables;

pub use enterprise::{
    EnterpriseNetworkError, IPv6Address, SecureVpnTunnel,
    IPv6Header, SlaacAutoconfig, IPv6Route, IPv6RoutingTable,
    AntiReplayWindow, VpnVirtualInterface,
    TlsState, TlsRecordType, SovereignSslEngine,
};
pub use tcp::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
pub use pf_firewall::{
    PfAction, PfAddress, PfDirection, PfFirewall, PfInterface, PfNatRule, PfPort, 
    PfProtocol, PfQueue, PfRule, PfRuleOptions, PfStateOption, PfStats, PfTable,
    ConnectionState, TcpState as PfConnectionState,
};
pub use nftables::{
    NftChain, NftChainPolicy, NftChainType, NftConnState, NftConntrack, NftConnection, NftCounter,
    NftCmpOp, NftDataType, NftExpression, NftFamily, NftHook, NftMap, NftMetaKey, NftPriority,
    NftQuota, NftRegister, NftRule, NftSet, NftTable, NftVerdict, NftPayloadProtocol,
    NftablesManager, NftablesStats,
};
