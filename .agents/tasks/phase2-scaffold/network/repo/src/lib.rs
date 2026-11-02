//! # SigmaOS Network
//!
//! Standalone network stack crate extracted from the SigmaOS monorepo.
//!
//! ## Architecture (Layered Design)
//!
//! ```text
//! ┌──────────────────────────────────┐
//! │  Application / Policy layer       │  DnsResolver, MdnsAnnouncer
//! ├──────────────────────────────────┤
//! │  Session / TLS layer              │  TlsContext
//! ├──────────────────────────────────┤
//! │  Transport layer                  │  TcpConnection
//! ├──────────────────────────────────┤
//! │  Network / IP layer               │  TcpStack
//! └──────────────────────────────────┘
//! ```
//!
//! ## Principles
//! - Separation of mechanism (TCP/TLS) and policy (cert verification, DNS strategy)
//! - Zero-copy paths for receive buffers where applicable
//! - Race-free design: all shared state protected via ownership or explicit locking
//! - Memory safety: no unsafe blocks in public API surface
//! - Stable public API: semver-compatible across patch releases

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

use thiserror::Error;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors produced by the SigmaOS network stack.
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum NetworkError {
    /// The requested connection was refused by the remote end.
    #[error("connection refused")]
    ConnectionRefused,

    /// The connection was reset unexpectedly.
    #[error("connection reset")]
    ConnectionReset,

    /// A send operation failed.
    #[error("send failed: {0}")]
    SendFailed(String),

    /// A receive operation failed.
    #[error("receive failed: {0}")]
    ReceiveFailed(String),

    /// TLS handshake or crypto error.
    #[error("TLS error: {0}")]
    TlsError(String),

    /// DNS resolution failed.
    #[error("DNS resolution failed for {0}")]
    DnsResolutionFailed(String),

    /// mDNS announcement error.
    #[error("mDNS error: {0}")]
    MdnsError(String),

    /// Generic I/O error description.
    #[error("I/O error: {0}")]
    Io(String),
}

// ---------------------------------------------------------------------------
// TCP layer  (mechanism — no policy)
// ---------------------------------------------------------------------------

/// IPv4 or IPv6 socket address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SocketAddr {
    /// Dotted-decimal or colon-hex string representation of the address.
    pub host: String,
    /// TCP port number.
    pub port: u16,
}

impl SocketAddr {
    /// Construct a new socket address.
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        SocketAddr {
            host: host.into(),
            port,
        }
    }
}

/// State of a TCP connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpState {
    /// Initial state before `connect()`.
    Closed,
    /// Three-way handshake in progress.
    Connecting,
    /// Data transfer phase.
    Established,
    /// Graceful shutdown initiated.
    Closing,
    /// Both sides have closed.
    TimeWait,
}

/// A single TCP connection handle.
///
/// Mechanism only — TLS policy is handled by [`TlsContext`].
pub struct TcpConnection {
    local: SocketAddr,
    remote: SocketAddr,
    state: TcpState,
    /// Receive buffer (zero-copy: owned slice, no internal copy on enqueue).
    rx_buf: Vec<u8>,
}

impl TcpConnection {
    /// Establish a new outbound connection (stub: sets state to Established).
    pub fn connect(local: SocketAddr, remote: SocketAddr) -> Result<Self, NetworkError> {
        Ok(TcpConnection {
            local,
            remote,
            state: TcpState::Established,
            rx_buf: Vec::new(),
        })
    }

    /// Current connection state.
    pub fn state(&self) -> TcpState {
        self.state
    }

    /// Local bound address.
    pub fn local_addr(&self) -> &SocketAddr {
        &self.local
    }

    /// Remote peer address.
    pub fn remote_addr(&self) -> &SocketAddr {
        &self.remote
    }

    /// Send bytes over the connection.
    ///
    /// Returns the number of bytes enqueued (stub: always accepts all bytes).
    pub fn send(&mut self, data: &[u8]) -> Result<usize, NetworkError> {
        if self.state != TcpState::Established {
            return Err(NetworkError::SendFailed("not connected".into()));
        }
        Ok(data.len())
    }

    /// Receive bytes from the connection into `buf`.
    ///
    /// Returns the number of bytes written to `buf`.
    /// Zero-copy: data is moved out of the internal ring buffer, not copied twice.
    pub fn recv(&mut self, buf: &mut [u8]) -> Result<usize, NetworkError> {
        if self.state != TcpState::Established {
            return Err(NetworkError::ReceiveFailed("not connected".into()));
        }
        let n = self.rx_buf.len().min(buf.len());
        buf[..n].copy_from_slice(&self.rx_buf[..n]);
        self.rx_buf.drain(..n);
        Ok(n)
    }

    /// Close the connection gracefully.
    pub fn close(&mut self) {
        self.state = TcpState::Closing;
    }

    /// Inject bytes into the receive buffer (used by the stack and tests).
    #[doc(hidden)]
    pub fn inject_rx(&mut self, data: &[u8]) {
        self.rx_buf.extend_from_slice(data);
    }
}

// ---------------------------------------------------------------------------
// TCP stack  (owns device + connection table)
// ---------------------------------------------------------------------------

/// Configuration for the TCP/IP stack.
#[derive(Debug, Clone)]
pub struct TcpStackConfig {
    /// Maximum number of simultaneous open connections.
    pub max_connections: usize,
    /// Receive window size in bytes.
    pub recv_window: usize,
    /// Enable TCP Fast Open.
    pub tcp_fast_open: bool,
}

impl Default for TcpStackConfig {
    fn default() -> Self {
        TcpStackConfig {
            max_connections: 1024,
            recv_window: 65536,
            tcp_fast_open: false,
        }
    }
}

/// The SigmaOS TCP/IP stack.
///
/// Owns the connection table and drives protocol state machines.
/// Policy decisions (TLS, DNS strategy) are injected through separate types
/// following the separation-of-mechanism-and-policy principle.
pub struct TcpStack {
    config: TcpStackConfig,
    connections: Vec<TcpConnection>,
}

impl TcpStack {
    /// Create a new TCP stack with the given configuration.
    pub fn new(config: TcpStackConfig) -> Self {
        TcpStack {
            config,
            connections: Vec::new(),
        }
    }

    /// Open a new outbound TCP connection.
    pub fn connect(
        &mut self,
        local: SocketAddr,
        remote: SocketAddr,
    ) -> Result<usize, NetworkError> {
        if self.connections.len() >= self.config.max_connections {
            return Err(NetworkError::Io("connection table full".into()));
        }
        let conn = TcpConnection::connect(local, remote)?;
        self.connections.push(conn);
        Ok(self.connections.len() - 1)
    }

    /// Get a mutable reference to a connection by index.
    pub fn connection_mut(&mut self, idx: usize) -> Option<&mut TcpConnection> {
        self.connections.get_mut(idx)
    }

    /// Number of active connections.
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}

// ---------------------------------------------------------------------------
// TLS layer  (policy: cert verification, cipher suite selection)
// ---------------------------------------------------------------------------

/// Supported TLS protocol versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsVersion {
    /// TLS 1.2 (legacy, still supported for compatibility).
    Tls12,
    /// TLS 1.3 (preferred — forward secrecy by default).
    Tls13,
}

/// TLS handshake state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsHandshakeState {
    /// Not yet started.
    Initial,
    /// ClientHello sent / ServerHello expected.
    ClientHello,
    /// Certificate and key exchange in progress.
    Handshake,
    /// Handshake complete; application data flows.
    Established,
    /// Session closed.
    Closed,
}

/// TLS context — encapsulates policy (version, cert verification, ALPN).
///
/// Mechanism (TCP framing) stays in [`TcpConnection`]; only policy lives here.
pub struct TlsContext {
    version: TlsVersion,
    server_name: Option<String>,
    verify_certs: bool,
    state: TlsHandshakeState,
}

impl TlsContext {
    /// Create a new TLS context for the given server name.
    ///
    /// Certificate verification is **enabled by default** (secure default).
    pub fn new(server_name: impl Into<String>) -> Self {
        TlsContext {
            version: TlsVersion::Tls13,
            server_name: Some(server_name.into()),
            verify_certs: true,
            state: TlsHandshakeState::Initial,
        }
    }

    /// Create a TLS context without SNI (e.g. IP-address connections).
    pub fn new_no_sni() -> Self {
        TlsContext {
            version: TlsVersion::Tls13,
            server_name: None,
            verify_certs: true,
            state: TlsHandshakeState::Initial,
        }
    }

    /// Override the minimum TLS version (policy decision).
    pub fn with_version(mut self, version: TlsVersion) -> Self {
        self.version = version;
        self
    }

    /// Disable certificate verification (insecure — for testing only).
    pub fn with_cert_verification(mut self, verify: bool) -> Self {
        self.verify_certs = verify;
        self
    }

    /// Perform a TLS handshake over `conn` (stub: transitions to Established).
    pub fn handshake(&mut self, _conn: &mut TcpConnection) -> Result<(), NetworkError> {
        self.state = TlsHandshakeState::ClientHello;
        // Stub: skip actual crypto; go straight to Established.
        self.state = TlsHandshakeState::Established;
        Ok(())
    }

    /// Current handshake state.
    pub fn state(&self) -> TlsHandshakeState {
        self.state
    }

    /// Configured TLS version.
    pub fn version(&self) -> TlsVersion {
        self.version
    }

    /// Whether certificate verification is enabled.
    pub fn verify_certs(&self) -> bool {
        self.verify_certs
    }
}

// ---------------------------------------------------------------------------
// DNS resolver  (policy: server selection, caching, retry strategy)
// ---------------------------------------------------------------------------

/// A resolved DNS record (A/AAAA).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsRecord {
    /// Hostname queried.
    pub name: String,
    /// Resolved address (IPv4 octets for now; IPv6 extension is additive).
    pub addr: [u8; 4],
    /// Time-to-live in seconds.
    pub ttl: u32,
}

/// DNS resolver — owns the server list and an in-memory cache.
///
/// Policy (which servers, TTL policy, retry count) is configurable.
/// Mechanism (wire format encoding/decoding) is separated into a future
/// `dns::codec` module following the separation-of-concerns principle.
pub struct DnsResolver {
    servers: Vec<[u8; 4]>,
    cache: Vec<DnsRecord>,
}

impl DnsResolver {
    /// Create a resolver that queries the given DNS servers in order.
    pub fn new(servers: Vec<[u8; 4]>) -> Self {
        DnsResolver {
            servers,
            cache: Vec::new(),
        }
    }

    /// Resolve `hostname` to an IPv4 address.
    ///
    /// Checks the in-memory cache first; falls back to a stub network query.
    pub fn resolve(&mut self, hostname: &str) -> Result<[u8; 4], NetworkError> {
        // Cache hit (policy: honour TTL — stub ignores TTL for simplicity).
        if let Some(rec) = self.cache.iter().find(|r| r.name == hostname) {
            return Ok(rec.addr);
        }
        if self.servers.is_empty() {
            return Err(NetworkError::DnsResolutionFailed(hostname.to_string()));
        }
        // Stub: return loopback for any name when a server is configured.
        let record = DnsRecord {
            name: hostname.to_string(),
            addr: [127, 0, 0, 1],
            ttl: 60,
        };
        self.cache.push(record.clone());
        Ok(record.addr)
    }

    /// Pre-populate the cache (useful for testing and static /etc/hosts entries).
    pub fn seed(&mut self, record: DnsRecord) {
        self.cache.push(record);
    }

    /// Number of entries currently in the cache.
    pub fn cache_len(&self) -> usize {
        self.cache.len()
    }
}

// ---------------------------------------------------------------------------
// mDNS announcer  (service discovery — zero-configuration networking)
// ---------------------------------------------------------------------------

/// A service record to announce via mDNS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MdnsService {
    /// Service instance name (e.g. "My Printer").
    pub instance: String,
    /// Service type (e.g. "_http._tcp").
    pub service_type: String,
    /// Port the service is listening on.
    pub port: u16,
}

impl MdnsService {
    /// Construct a new service record.
    pub fn new(instance: impl Into<String>, service_type: impl Into<String>, port: u16) -> Self {
        MdnsService {
            instance: instance.into(),
            service_type: service_type.into(),
            port,
        }
    }
}

/// mDNS announcer — broadcasts service records on the local link.
///
/// Discoverability principle: services announce themselves so peers can find
/// them without central coordination (zero-configuration networking).
pub struct MdnsAnnouncer {
    services: Vec<MdnsService>,
    running: bool,
}

impl MdnsAnnouncer {
    /// Create a new announcer (initially stopped).
    pub fn new() -> Self {
        MdnsAnnouncer {
            services: Vec::new(),
            running: false,
        }
    }

    /// Register a service to be announced.
    pub fn register(&mut self, svc: MdnsService) {
        self.services.push(svc);
    }

    /// Start broadcasting announcements (stub: sets running flag).
    pub fn start(&mut self) -> Result<(), NetworkError> {
        if self.services.is_empty() {
            return Err(NetworkError::MdnsError("no services registered".into()));
        }
        self.running = true;
        Ok(())
    }

    /// Stop broadcasting.
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Whether the announcer is currently active.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Registered services.
    pub fn services(&self) -> &[MdnsService] {
        &self.services
    }
}

impl Default for MdnsAnnouncer {
    fn default() -> Self {
        Self::new()
    }
}

// All types above are already `pub` at crate root — no re-export block needed.
