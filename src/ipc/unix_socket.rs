//! High-performance BSD & Linux inspired Unix Domain Sockets for SigmaOS
//! Implements Stream and Datagram sockets, path-based binding, abstract namespaces,
//! socketpair creation, and capability-scoped sandboxing constraints.

#![allow(clippy::new_without_default)]
#![allow(dead_code)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use crate::ipc::ipc::{IPCError, IPCCapability};

/// Sockets can be Stream (connection-oriented) or Datagram (connectionless)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnixSocketType {
    Stream = 0,
    Datagram = 1,
}

/// Sockets can be bound to a pathname, an abstract name (Linux-like), or unbound
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnixSocketAddress {
    Unbound,
    Path(String),
    Abstract(String),
}

/// State of a connection-oriented Stream socket (inspired by TCP/Unix states)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnixSocketState {
    Closed,
    Unbound,
    Bound,
    Listening,
    Connecting,
    Connected,
}

/// Represents a single Unix Domain Socket endpoint.
pub struct UnixSocket {
    pub id: usize,
    pub socket_type: UnixSocketType,
    pub address: UnixSocketAddress,
    pub state: UnixSocketState,
    pub peer_id: Option<usize>,
    pub rx_buffer: Vec<u8>,
    pub capability: IPCCapability,
    // Tracks pending connection requests for a listening stream socket
    pub pending_connections: Vec<usize>,
}

impl UnixSocket {
    pub fn new(id: usize, socket_type: UnixSocketType, capability: IPCCapability) -> Self {
        Self {
            id,
            socket_type,
            address: UnixSocketAddress::Unbound,
            state: UnixSocketState::Unbound,
            peer_id: None,
            rx_buffer: Vec::new(),
            capability,
            pending_connections: Vec::new(),
        }
    }

    /// Bind a socket to an address
    pub fn bind(&mut self, address: UnixSocketAddress) -> Result<(), IPCError> {
        if !self.capability.allow_unix_sockets {
            return Err(IPCError::PermissionDenied);
        }
        if self.state != UnixSocketState::Unbound {
            return Err(IPCError::InvalidSize); // Already bound or connected
        }
        self.address = address;
        self.state = UnixSocketState::Bound;
        Ok(())
    }

    /// Listen for connections (Stream only)
    pub fn listen(&mut self) -> Result<(), IPCError> {
        if self.socket_type != UnixSocketType::Stream {
            return Err(IPCError::NotConnected); // Only Stream can listen
        }
        if self.state != UnixSocketState::Bound {
            return Err(IPCError::BufferEmpty); // Must be bound to listen
        }
        self.state = UnixSocketState::Listening;
        Ok(())
    }

    /// Close the socket and clear buffers
    pub fn close(&mut self) {
        self.state = UnixSocketState::Closed;
        self.peer_id = None;
        self.rx_buffer.clear();
        self.pending_connections.clear();
    }
}

/// Dynamic manager for Unix Domain Sockets binding and connection routing
pub struct UnixSocketManager {
    pub sockets: BTreeMap<usize, UnixSocket>,
    pub bindings: BTreeMap<String, usize>, // Path/Abstract address string -> Socket ID
    pub next_id: usize,
}

impl UnixSocketManager {
    pub fn new() -> Self {
        Self {
            sockets: BTreeMap::new(),
            bindings: BTreeMap::new(),
            next_id: 1,
        }
    }

    /// Create a new socket
    pub fn create_socket(&mut self, socket_type: UnixSocketType, capability: IPCCapability) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let socket = UnixSocket::new(id, socket_type, capability);
        self.sockets.insert(id, socket);
        id
    }

    /// Bind socket to address
    pub fn bind(&mut self, id: usize, address: UnixSocketAddress) -> Result<(), IPCError> {
        let socket = self.sockets.get_mut(&id).ok_or(IPCError::NotConnected)?;
        if !socket.capability.allow_unix_sockets {
            return Err(IPCError::PermissionDenied);
        }

        let address_str = match &address {
            UnixSocketAddress::Path(p) => p.clone(),
            UnixSocketAddress::Abstract(a) => a.clone(),
            UnixSocketAddress::Unbound => return Err(IPCError::InvalidSize),
        };

        if self.bindings.contains_key(&address_str) {
            return Err(IPCError::BufferFull); // Address already in use
        }

        socket.bind(address)?;
        self.bindings.insert(address_str, id);
        Ok(())
    }

    /// Connect socket to bound address (Stream only)
    pub fn connect(&mut self, client_id: usize, address: UnixSocketAddress) -> Result<(), IPCError> {
        let address_str = match &address {
            UnixSocketAddress::Path(p) => p.clone(),
            UnixSocketAddress::Abstract(a) => a.clone(),
            UnixSocketAddress::Unbound => return Err(IPCError::InvalidSize),
        };

        let server_id = *self.bindings.get(&address_str).ok_or(IPCError::NotConnected)?;

        let (client_capability, client_type) = {
            let client_socket = self.sockets.get(&client_id).ok_or(IPCError::NotConnected)?;
            if client_socket.state == UnixSocketState::Connected {
                return Err(IPCError::InvalidSize);
            }
            (client_socket.capability, client_socket.socket_type)
        };

        if !client_capability.allow_unix_sockets {
            return Err(IPCError::PermissionDenied);
        }

        if client_type != UnixSocketType::Stream {
            return Err(IPCError::NotConnected);
        }

        let server_socket = self.sockets.get_mut(&server_id).ok_or(IPCError::NotConnected)?;
        if server_socket.state != UnixSocketState::Listening {
            return Err(IPCError::PermissionDenied);
        }

        server_socket.pending_connections.push(client_id);

        let client_socket = self.sockets.get_mut(&client_id).unwrap();
        client_socket.state = UnixSocketState::Connecting;
        client_socket.peer_id = Some(server_id);

        Ok(())
    }

    /// Accept incoming connection on listening socket (Stream only)
    pub fn accept(&mut self, server_id: usize) -> Result<usize, IPCError> {
        let server_socket = self.sockets.get_mut(&server_id).ok_or(IPCError::NotConnected)?;
        if server_socket.state != UnixSocketState::Listening {
            return Err(IPCError::PermissionDenied);
        }

        if server_socket.pending_connections.is_empty() {
            return Err(IPCError::BufferEmpty);
        }

        let client_id = server_socket.pending_connections.remove(0);

        let accepted_id = self.next_id;
        self.next_id += 1;

        let mut accepted_socket = UnixSocket::new(accepted_id, UnixSocketType::Stream, server_socket.capability);
        accepted_socket.state = UnixSocketState::Connected;
        accepted_socket.peer_id = Some(client_id);
        self.sockets.insert(accepted_id, accepted_socket);

        let client_socket = self.sockets.get_mut(&client_id).ok_or(IPCError::NotConnected)?;
        client_socket.state = UnixSocketState::Connected;
        client_socket.peer_id = Some(accepted_id);

        Ok(accepted_id)
    }

    /// Send data over socket
    pub fn send(&mut self, sender_id: usize, data: &[u8]) -> Result<(), IPCError> {
        let sender_socket = self.sockets.get(&sender_id).ok_or(IPCError::NotConnected)?;
        if !sender_socket.capability.can_send {
            return Err(IPCError::PermissionDenied);
        }

        match sender_socket.socket_type {
            UnixSocketType::Stream => {
                if sender_socket.state != UnixSocketState::Connected {
                    return Err(IPCError::NotConnected);
                }
                let peer_id = sender_socket.peer_id.ok_or(IPCError::NotConnected)?;
                let peer_socket = self.sockets.get_mut(&peer_id).ok_or(IPCError::NotConnected)?;
                peer_socket.rx_buffer.extend_from_slice(data);
                Ok(())
            }
            UnixSocketType::Datagram => {
                if sender_socket.peer_id.is_none() {
                    return Err(IPCError::NotConnected);
                }
                let peer_id = sender_socket.peer_id.unwrap();
                let peer_socket = self.sockets.get_mut(&peer_id).ok_or(IPCError::NotConnected)?;

                let mut packet = data.len().to_le_bytes().to_vec();
                packet.extend_from_slice(data);
                peer_socket.rx_buffer.extend(packet);
                Ok(())
            }
        }
    }

    /// Send datagram to target address (Datagram only, connectionless)
    pub fn send_to(&mut self, sender_id: usize, data: &[u8], target: UnixSocketAddress) -> Result<(), IPCError> {
        let sender_socket = self.sockets.get(&sender_id).ok_or(IPCError::NotConnected)?;
        if sender_socket.socket_type != UnixSocketType::Datagram {
            return Err(IPCError::NotConnected);
        }
        if !sender_socket.capability.can_send {
            return Err(IPCError::PermissionDenied);
        }

        let target_str = match &target {
            UnixSocketAddress::Path(p) => p.clone(),
            UnixSocketAddress::Abstract(a) => a.clone(),
            UnixSocketAddress::Unbound => return Err(IPCError::InvalidSize),
        };

        let target_id = *self.bindings.get(&target_str).ok_or(IPCError::NotConnected)?;
        let target_socket = self.sockets.get_mut(&target_id).ok_or(IPCError::NotConnected)?;

        let mut packet = sender_id.to_le_bytes().to_vec();
        packet.extend_from_slice(&data.len().to_le_bytes());
        packet.extend_from_slice(data);
        target_socket.rx_buffer.extend(packet);

        Ok(())
    }

    /// Receive data from socket (Stream)
    pub fn receive(&mut self, id: usize, buffer: &mut [u8]) -> Result<usize, IPCError> {
        let socket = self.sockets.get_mut(&id).ok_or(IPCError::NotConnected)?;
        if !socket.capability.can_receive {
            return Err(IPCError::PermissionDenied);
        }

        if socket.socket_type != UnixSocketType::Stream {
            return Err(IPCError::NotConnected);
        }

        if socket.rx_buffer.is_empty() {
            if socket.state == UnixSocketState::Connected {
                return Err(IPCError::BufferEmpty);
            } else {
                return Err(IPCError::NotConnected);
            }
        }

        let read_len = buffer.len().min(socket.rx_buffer.len());
        buffer[..read_len].copy_from_slice(&socket.rx_buffer[..read_len]);
        socket.rx_buffer.drain(..read_len);

        Ok(read_len)
    }

    /// Receive datagram from socket (Datagram)
    pub fn receive_from(&mut self, id: usize, buffer: &mut [u8]) -> Result<(usize, Option<usize>), IPCError> {
        let socket = self.sockets.get_mut(&id).ok_or(IPCError::NotConnected)?;
        if !socket.capability.can_receive {
            return Err(IPCError::PermissionDenied);
        }

        if socket.socket_type != UnixSocketType::Datagram {
            return Err(IPCError::NotConnected);
        }

        if socket.rx_buffer.is_empty() {
            return Err(IPCError::BufferEmpty);
        }

        if socket.peer_id.is_some() {
            let sz = core::mem::size_of::<usize>();
            if socket.rx_buffer.len() < sz {
                return Err(IPCError::BufferEmpty);
            }
            let mut len_bytes = [0; core::mem::size_of::<usize>()];
            len_bytes.copy_from_slice(&socket.rx_buffer[..sz]);
            let data_len = usize::from_le_bytes(len_bytes);

            if socket.rx_buffer.len() < sz + data_len {
                return Err(IPCError::BufferEmpty);
            }

            let read_len = buffer.len().min(data_len);
            buffer[..read_len].copy_from_slice(&socket.rx_buffer[sz..sz + read_len]);
            socket.rx_buffer.drain(..sz + data_len);

            Ok((read_len, socket.peer_id))
        } else {
            let sz = core::mem::size_of::<usize>();
            if socket.rx_buffer.len() < 2 * sz {
                return Err(IPCError::BufferEmpty);
            }
            let mut sender_bytes = [0; core::mem::size_of::<usize>()];
            sender_bytes.copy_from_slice(&socket.rx_buffer[..sz]);
            let sender_id = usize::from_le_bytes(sender_bytes);

            let mut len_bytes = [0; core::mem::size_of::<usize>()];
            len_bytes.copy_from_slice(&socket.rx_buffer[sz..2 * sz]);
            let data_len = usize::from_le_bytes(len_bytes);

            if socket.rx_buffer.len() < 2 * sz + data_len {
                return Err(IPCError::BufferEmpty);
            }

            let read_len = buffer.len().min(data_len);
            buffer[..read_len].copy_from_slice(&socket.rx_buffer[2 * sz..2 * sz + read_len]);
            socket.rx_buffer.drain(..2 * sz + data_len);

            Ok((read_len, Some(sender_id)))
        }
    }

    /// Create a pre-connected socketpair (Stream or Datagram)
    pub fn socketpair(&mut self, socket_type: UnixSocketType, capability: IPCCapability) -> Result<(usize, usize), IPCError> {
        let id1 = self.create_socket(socket_type, capability);
        let id2 = self.create_socket(socket_type, capability);

        {
            let s1 = self.sockets.get_mut(&id1).unwrap();
            s1.state = UnixSocketState::Connected;
            s1.peer_id = Some(id2);
        }

        {
            let s2 = self.sockets.get_mut(&id2).unwrap();
            s2.state = UnixSocketState::Connected;
            s2.peer_id = Some(id1);
        }

        Ok((id1, id2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_handshake() {
        let mut mgr = UnixSocketManager::new();
        let cap = IPCCapability::full();

        let client = mgr.create_socket(UnixSocketType::Stream, cap);
        let server = mgr.create_socket(UnixSocketType::Stream, cap);

        let addr = UnixSocketAddress::Path(String::from("/tmp/test.sock"));

        assert!(mgr.bind(server, addr.clone()).is_ok());
        assert!(mgr.sockets.get(&server).unwrap().state == UnixSocketState::Bound);

        assert!(mgr.sockets.get_mut(&server).unwrap().listen().is_ok());
        assert!(mgr.sockets.get(&server).unwrap().state == UnixSocketState::Listening);

        assert!(mgr.connect(client, addr).is_ok());
        assert!(mgr.sockets.get(&client).unwrap().state == UnixSocketState::Connecting);

        let accepted = mgr.accept(server).unwrap();
        assert!(mgr.sockets.get(&client).unwrap().state == UnixSocketState::Connected);
        assert!(mgr.sockets.get(&accepted).unwrap().state == UnixSocketState::Connected);
    }

    #[test]
    fn test_stream_read_write() {
        let mut mgr = UnixSocketManager::new();
        let cap = IPCCapability::full();

        let (c1, c2) = mgr.socketpair(UnixSocketType::Stream, cap).unwrap();

        let payload = b"Hello from Unix Domain Sockets!";
        assert!(mgr.send(c1, payload).is_ok());

        let mut buf = [0; 64];
        let bytes = mgr.receive(c2, &mut buf).unwrap();
        assert_eq!(bytes, payload.len());
        assert_eq!(&buf[..bytes], payload);
    }

    #[test]
    fn test_datagram_transmission() {
        let mut mgr = UnixSocketManager::new();
        let cap = IPCCapability::full();

        let s1 = mgr.create_socket(UnixSocketType::Datagram, cap);
        let s2 = mgr.create_socket(UnixSocketType::Datagram, cap);

        let addr = UnixSocketAddress::Abstract(String::from("test_abstract"));
        assert!(mgr.bind(s2, addr.clone()).is_ok());

        let payload = b"Datagram message payload";
        assert!(mgr.send_to(s1, payload, addr).is_ok());

        let mut buf = [0; 64];
        let (bytes, sender) = mgr.receive_from(s2, &mut buf).unwrap();
        assert_eq!(bytes, payload.len());
        assert_eq!(&buf[..bytes], payload);
        assert_eq!(sender, Some(s1));
    }

    #[test]
    fn test_socketpair_creation() {
        let mut mgr = UnixSocketManager::new();
        let cap = IPCCapability::full();

        let (s1, s2) = mgr.socketpair(UnixSocketType::Datagram, cap).unwrap();
        assert!(mgr.sockets.get(&s1).unwrap().state == UnixSocketState::Connected);
        assert!(mgr.sockets.get(&s2).unwrap().state == UnixSocketState::Connected);

        let payload = b"Socketpair msg";
        assert!(mgr.send(s1, payload).is_ok());

        let mut buf = [0; 64];
        let (bytes, sender) = mgr.receive_from(s2, &mut buf).unwrap();
        assert_eq!(bytes, payload.len());
        assert_eq!(&buf[..bytes], payload);
        assert_eq!(sender, Some(s1));
    }

    #[test]
    fn test_capability_enforcement() {
        let mut mgr = UnixSocketManager::new();
        let mut cap = IPCCapability::new(); // restrictive
        cap.can_send = false;
        cap.allow_unix_sockets = false;

        let s1 = mgr.create_socket(UnixSocketType::Stream, cap);
        let addr = UnixSocketAddress::Path(String::from("/tmp/perm.sock"));
        assert!(mgr.bind(s1, addr).is_err()); // No unix socket allow capability
    }
}
