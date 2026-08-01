// SigmaOS Network Stack Module
pub mod enterprise;
pub mod tcp;
pub mod legacy_net;
pub mod revival;

pub use enterprise::{EnterpriseNetworkError, IPv6Address, SecureVpnTunnel};
pub use tcp::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
pub use legacy_net::{
    LegacyProtocol, LegacyProtocolAdapter,
};
pub use revival::{
    RevivalProtocol, NetRevival,
};
