//! SigmaOS built-in IRC and chat client.
//!
//! Provides multi-server connections, channel management, and notifications.

#![warn(missing_docs)]

/// State of a server connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Connected and authenticated.
    Connected,
    /// Connection in progress.
    Connecting,
    /// Disconnected.
    Disconnected,
}

/// A chat server the client connects to.
#[derive(Debug, Clone)]
pub struct Server {
    /// Hostname of the server.
    pub host: String,
    /// Port number.
    pub port: u16,
    /// Whether TLS is used.
    pub tls: bool,
    /// Current connection state.
    pub state: ConnectionState,
}

/// Trait for the chat client.
pub trait ChatClient {
    /// Connect to a server.
    fn connect(&mut self, server: Server) -> Result<(), ChatError>;
    /// Join a channel on the connected server.
    fn join(&mut self, channel: &str) -> Result<(), ChatError>;
    /// Send a message to a channel.
    fn send(&mut self, channel: &str, message: &str) -> Result<(), ChatError>;
}

/// Errors returned by the chat client.
#[derive(Debug)]
pub enum ChatError {
    /// The connection to the server failed.
    ConnectionFailed(String),
    /// The nickname was rejected.
    NickInUse,
    /// The channel does not exist.
    NoSuchChannel,
}

impl std::fmt::Display for ChatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatError::ConnectionFailed(msg) => write!(f, "connection failed: {msg}"),
            ChatError::NickInUse => write!(f, "nickname in use"),
            ChatError::NoSuchChannel => write!(f, "no such channel"),
        }
    }
}

impl std::error::Error for ChatError {}
