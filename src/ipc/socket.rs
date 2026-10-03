//! SigmaOS — Unix Domain Sockets
//! SPDX-License-Identifier: MIT OR GPL-2.0
//! Inspired by Linux net/unix/
//!
//! Provides SOCK_STREAM, SOCK_DGRAM, and SOCK_SEQPACKET Unix sockets.
//! Supports: bind, connect, send, recv, socketpair, accept.
//! Also supports ancillary data: SCM_CREDENTIALS and SCM_RIGHTS.

#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::string::String;

// ── Errors ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketError {
    /// EADDRINUSE — path already bound
    AddrInUse,
    /// ENOENT — path not found in namespace
    NoEntry,
    /// ENOTCONN — socket not connected
    NotConnected,
    /// ECONNREFUSED — no listener at address
    ConnectionRefused,
    /// EAGAIN — buffer full / empty
    WouldBlock,
    /// EPIPE — peer closed
    BrokenPipe,
    /// EINVAL — invalid argument
    InvalidArg,
    /// EOPNOTSUPP — operation not supported for socket type
    OpNotSupported,
}

impl core::fmt::Display for SocketError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AddrInUse => write!(f, "EADDRINUSE"),
            Self::NoEntry => write!(f, "ENOENT"),
            Self::NotConnected => write!(f, "ENOTCONN"),
            Self::ConnectionRefused => write!(f, "ECONNREFUSED"),
            Self::WouldBlock => write!(f, "EAGAIN"),
            Self::BrokenPipe => write!(f, "EPIPE"),
            Self::InvalidArg => write!(f, "EINVAL"),
            Self::OpNotSupported => write!(f, "EOPNOTSUPP"),
        }
    }
}

// ── Socket type ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnixSocketType {
    /// Ordered, reliable, bidirectional byte stream
    Stream,
    /// Unreliable datagrams, connectionless
    Dgram,
    /// Ordered, reliable, bidirectional message-based
    Seqpacket,
}

// ── Ancillary data (cmsg) ─────────────────────────────────────────────────────

/// SCM_CREDENTIALS payload: process credentials
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScmCredentials {
    pub pid: u32,
    pub uid: u32,
    pub gid: u32,
}

/// SCM_RIGHTS payload: file descriptors being passed over the socket
#[derive(Debug, Clone)]
pub struct ScmRights {
    pub fds: Vec<i32>,
}

/// Ancillary control message
#[derive(Debug, Clone)]
pub enum CmsgData {
    Credentials(ScmCredentials),
    Rights(ScmRights),
}

/// A message in the socket receive buffer
#[derive(Debug, Clone)]
pub struct SocketMessage {
    /// Payload bytes
    pub data: Vec<u8>,
    /// Optional ancillary data
    pub cmsg: Option<CmsgData>,
    /// Sender path (for DGRAM)
    pub from: Option<String>,
}

// ── Socket state ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketState {
    Created,
    Bound,
    Listening,
    Connected,
    Closed,
}

// ── Global socket namespace ───────────────────────────────────────────────────

/// Abstract Unix socket namespace: maps path → socket handle
#[derive(Debug, Default)]
pub struct UnixNamespace {
    /// path → socket id
    bindings: HashMap<String, u64>,
    /// listener queues: socket_id → pending connections
    listen_queues: HashMap<u64, VecDeque<UnixSocket>>,
    next_id: u64,
}

impl UnixNamespace {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind(&mut self, path: &str, socket_id: u64) -> Result<(), SocketError> {
        if self.bindings.contains_key(path) {
            return Err(SocketError::AddrInUse);
        }
        self.bindings.insert(path.to_string(), socket_id);
        Ok(())
    }

    pub fn lookup(&self, path: &str) -> Option<u64> {
        self.bindings.get(path).copied()
    }

    pub fn unbind(&mut self, path: &str) {
        self.bindings.remove(path);
    }

    pub fn alloc_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Push an incoming connection to the listener's backlog
    pub fn push_connection(&mut self, listener_id: u64, conn: UnixSocket) {
        self.listen_queues.entry(listener_id).or_default().push_back(conn);
    }

    /// Pop the next pending connection
    pub fn pop_connection(&mut self, listener_id: u64) -> Option<UnixSocket> {
        self.listen_queues.get_mut(&listener_id)?.pop_front()
    }
}

// ── UnixSocket ────────────────────────────────────────────────────────────────

/// Kernel Unix domain socket
#[derive(Debug)]
pub struct UnixSocket {
    /// Unique socket ID within namespace
    pub id: u64,
    /// Socket type
    pub socket_type: UnixSocketType,
    /// Bound filesystem path (if any)
    pub path: Option<String>,
    /// Current state
    pub state: SocketState,
    /// Receive buffer
    recv_buf: VecDeque<SocketMessage>,
    /// Maximum receive buffer size (bytes)
    recv_buf_limit: usize,
    /// Peer socket id (for connected sockets)
    pub peer_id: Option<u64>,
    /// Process credentials
    pub credentials: Option<ScmCredentials>,
    /// Backlog limit for SOCK_STREAM listeners
    pub backlog: usize,
}

impl UnixSocket {
    const DEFAULT_BUF: usize = 212_992; // 208 KiB, same as Linux default

    pub fn new(id: u64, socket_type: UnixSocketType) -> Self {
        Self {
            id,
            socket_type,
            path: None,
            state: SocketState::Created,
            recv_buf: VecDeque::new(),
            recv_buf_limit: Self::DEFAULT_BUF,
            peer_id: None,
            credentials: None,
            backlog: 128,
        }
    }

    /// Total bytes currently in recv buffer
    pub fn recv_buf_used(&self) -> usize {
        self.recv_buf.iter().map(|m| m.data.len()).sum()
    }

    /// Enqueue a message (kernel delivers it here)
    pub fn enqueue(&mut self, msg: SocketMessage) -> Result<(), SocketError> {
        if self.state == SocketState::Closed {
            return Err(SocketError::BrokenPipe);
        }
        if self.recv_buf_used() + msg.data.len() > self.recv_buf_limit {
            return Err(SocketError::WouldBlock);
        }
        self.recv_buf.push_back(msg);
        Ok(())
    }

    /// Dequeue one message
    pub fn dequeue(&mut self) -> Option<SocketMessage> {
        self.recv_buf.pop_front()
    }
}

// ── Public API functions ──────────────────────────────────────────────────────

/// Create a new Unix domain socket
pub fn unix_socket_create(
    ns: &mut UnixNamespace,
    sock_type: UnixSocketType,
) -> UnixSocket {
    let id = ns.alloc_id();
    UnixSocket::new(id, sock_type)
}

/// Bind a socket to a filesystem path
pub fn unix_socket_bind(
    ns: &mut UnixNamespace,
    sock: &mut UnixSocket,
    path: &str,
) -> Result<(), SocketError> {
    if sock.state != SocketState::Created {
        return Err(SocketError::InvalidArg);
    }
    ns.bind(path, sock.id)?;
    sock.path = Some(path.to_string());
    sock.state = SocketState::Bound;
    Ok(())
}

/// Mark a SOCK_STREAM socket as listening for connections
pub fn unix_socket_listen(
    sock: &mut UnixSocket,
    backlog: usize,
) -> Result<(), SocketError> {
    if sock.socket_type != UnixSocketType::Stream
        && sock.socket_type != UnixSocketType::Seqpacket
    {
        return Err(SocketError::OpNotSupported);
    }
    if sock.state != SocketState::Bound {
        return Err(SocketError::InvalidArg);
    }
    sock.backlog = backlog;
    sock.state = SocketState::Listening;
    Ok(())
}

/// Connect a socket to a listening peer.
/// For SOCK_DGRAM: just record the target path, no handshake.
/// For SOCK_STREAM/SEQPACKET: push a new accepted socket to the listener queue.
pub fn unix_socket_connect(
    ns: &mut UnixNamespace,
    sock: &mut UnixSocket,
    path: &str,
) -> Result<(), SocketError> {
    let listener_id = ns.lookup(path).ok_or(SocketError::NoEntry)?;

    match sock.socket_type {
        UnixSocketType::Dgram => {
            // Just record target — no connection state
            sock.peer_id = Some(listener_id);
            sock.state = SocketState::Connected;
        }
        UnixSocketType::Stream | UnixSocketType::Seqpacket => {
            // Create an "accepted" socket pre-connected to the caller
            let peer_id = ns.alloc_id();
            let mut peer = UnixSocket::new(peer_id, sock.socket_type);
            peer.state = SocketState::Connected;
            peer.peer_id = Some(sock.id);

            sock.peer_id = Some(peer_id);
            sock.state = SocketState::Connected;

            ns.push_connection(listener_id, peer);
        }
    }
    Ok(())
}

/// Accept a connection on a listening socket. Returns the new connected peer.
pub fn unix_socket_accept(
    ns: &mut UnixNamespace,
    listener: &mut UnixSocket,
) -> Result<UnixSocket, SocketError> {
    if listener.state != SocketState::Listening {
        return Err(SocketError::InvalidArg);
    }
    ns.pop_connection(listener.id)
        .ok_or(SocketError::WouldBlock)
}

/// Send data over a connected socket (places message in peer's recv_buf).
/// For simplicity this takes the peer socket directly.
pub fn unix_socket_send(
    sock: &UnixSocket,
    peer: &mut UnixSocket,
    data: &[u8],
    cmsg: Option<CmsgData>,
) -> Result<usize, SocketError> {
    if sock.state != SocketState::Connected {
        return Err(SocketError::NotConnected);
    }
    let msg = SocketMessage {
        data: data.to_vec(),
        cmsg,
        from: sock.path.clone(),
    };
    peer.enqueue(msg)?;
    Ok(data.len())
}

/// Receive data from the socket's receive buffer.
pub fn unix_socket_recv(
    sock: &mut UnixSocket,
    buf: &mut [u8],
) -> Result<usize, SocketError> {
    let msg = sock.dequeue().ok_or(SocketError::WouldBlock)?;
    let n = msg.data.len().min(buf.len());
    buf[..n].copy_from_slice(&msg.data[..n]);
    Ok(n)
}

/// socketpair(2): create a pair of connected, anonymous sockets
pub fn unix_socket_pair(
    ns: &mut UnixNamespace,
    sock_type: UnixSocketType,
) -> (UnixSocket, UnixSocket) {
    let id_a = ns.alloc_id();
    let id_b = ns.alloc_id();

    let mut a = UnixSocket::new(id_a, sock_type);
    let mut b = UnixSocket::new(id_b, sock_type);

    a.state = SocketState::Connected;
    a.peer_id = Some(id_b);

    b.state = SocketState::Connected;
    b.peer_id = Some(id_a);

    (a, b)
}

/// Close and remove a bound socket from the namespace
pub fn unix_socket_close(
    ns: &mut UnixNamespace,
    sock: &mut UnixSocket,
) {
    if let Some(ref path) = sock.path {
        ns.unbind(path);
    }
    sock.state = SocketState::Closed;
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socketpair_send_recv() {
        let mut ns = UnixNamespace::new();
        let (mut a, mut b) = unix_socket_pair(&mut ns, UnixSocketType::Stream);

        // a sends to b
        unix_socket_send(&a, &mut b, b"hello", None).unwrap();

        let mut buf = [0u8; 64];
        let n = unix_socket_recv(&mut b, &mut buf).unwrap();
        assert_eq!(&buf[..n], b"hello");
    }

    #[test]
    fn test_connect_accept_roundtrip() {
        let mut ns = UnixNamespace::new();

        // Server
        let mut server = unix_socket_create(&mut ns, UnixSocketType::Stream);
        unix_socket_bind(&mut ns, &mut server, "/tmp/test.sock").unwrap();
        unix_socket_listen(&mut server, 5).unwrap();

        // Client
        let mut client = unix_socket_create(&mut ns, UnixSocketType::Stream);
        unix_socket_connect(&mut ns, &mut client, "/tmp/test.sock").unwrap();
        assert_eq!(client.state, SocketState::Connected);

        // Accept on server side
        let mut peer = unix_socket_accept(&mut ns, &mut server).unwrap();
        assert_eq!(peer.state, SocketState::Connected);

        // client sends
        unix_socket_send(&client, &mut peer, b"world", None).unwrap();
        let mut buf = [0u8; 64];
        let n = unix_socket_recv(&mut peer, &mut buf).unwrap();
        assert_eq!(&buf[..n], b"world");
    }

    #[test]
    fn test_bind_addr_in_use() {
        let mut ns = UnixNamespace::new();
        let mut s1 = unix_socket_create(&mut ns, UnixSocketType::Stream);
        let mut s2 = unix_socket_create(&mut ns, UnixSocketType::Stream);
        unix_socket_bind(&mut ns, &mut s1, "/tmp/dup.sock").unwrap();
        let err = unix_socket_bind(&mut ns, &mut s2, "/tmp/dup.sock");
        assert_eq!(err, Err(SocketError::AddrInUse));
    }

    #[test]
    fn test_dgram_send_recv() {
        let mut ns = UnixNamespace::new();
        let mut server = unix_socket_create(&mut ns, UnixSocketType::Dgram);
        unix_socket_bind(&mut ns, &mut server, "/tmp/dgram.sock").unwrap();

        let mut client = unix_socket_create(&mut ns, UnixSocketType::Dgram);
        unix_socket_connect(&mut ns, &mut client, "/tmp/dgram.sock").unwrap();

        unix_socket_send(&client, &mut server, b"datagram", None).unwrap();
        let mut buf = [0u8; 64];
        let n = unix_socket_recv(&mut server, &mut buf).unwrap();
        assert_eq!(&buf[..n], b"datagram");
    }

    #[test]
    fn test_scm_credentials() {
        let mut ns = UnixNamespace::new();
        let (mut a, mut b) = unix_socket_pair(&mut ns, UnixSocketType::Stream);
        let creds = CmsgData::Credentials(ScmCredentials { pid: 100, uid: 1000, gid: 1000 });
        unix_socket_send(&a, &mut b, b"auth", Some(creds)).unwrap();
        let msg = b.dequeue().unwrap();
        if let Some(CmsgData::Credentials(c)) = msg.cmsg {
            assert_eq!(c.pid, 100);
        } else {
            panic!("expected SCM_CREDENTIALS");
        }
    }
}