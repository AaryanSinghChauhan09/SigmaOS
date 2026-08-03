// SigmaOS Network Module
// Network stack, browser core, and communication systems

pub mod browser_core;
pub mod ipv6;
pub mod routing;
pub mod sovereign_browser;
pub mod tls;
pub mod zenith;
pub mod signal_client;
pub mod tor_client;
pub mod torrent;

pub use signal_client::DoubleRatchetState;
pub use tor_client::{TorCircuit, TorRelay};
pub use torrent::{TorrentClient, TorrentMetadata};

pub use browser_core::{
    AdblockRule, BrowserCore, BrowserTab, BrowserTabState, SecurityLevel, TabCapabilities,
    TrackingProtection,
};
pub use ipv6::{
    Ipv6Address, Ipv6AddressType, Ipv6ExtensionHeader, Ipv6Header, Ipv6Interface, Ipv6Route,
    Ipv6Stack,
};
pub use routing::{RouteEntry, RouteKey, RouteProtocol, RouteType, RoutingTable};
pub use sovereign_browser::{
    AdBlockRule as SovereignAdBlockRule, BraveShield, BrowserTab as SovereignBrowserTab, BrowserError,
    SecurityProfile, SovereignBrowser, TabContainer, TabState,
};
pub use tls::{CipherSuite, TlsConfig, TlsEngine, TlsSession, TlsState, TlsVersion};
pub use zenith::{
    E1000NetworkDriver, NetworkDriverDevice, NetworkDriverManager, NetworkDriverType,
    NetworkError, NetworkPacketFrame, Rtl8139NetworkDriver, ZeroCopyPacketRing,
};
