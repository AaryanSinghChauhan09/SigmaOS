// Linux-inspired socket abstraction
// Network communication interface for SigmaOS

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// socket address family
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressFamily {
    Unspec = 0,
    Unix = 1,
    Inet = 2,
    Inet6 = 10,
}

/// socket type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Stream = 1,    // TCP
    Datagram = 2,   // UDP
    Raw = 3,
}

/// socket protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketProtocol {
    Ip = 0,
    Tcp = 6,
    Udp = 17,
}

/// socket state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Unconnected,
    Connecting,
    Connected,
    Listening,
    Bound,
    Closed,
}

/// socket address
#[derive(Debug, Clone)]
pub struct SocketAddr {
    pub family: AddressFamily,
    pub port: u16,
    pub ip: String,
}

impl SocketAddr {
    pub fn new(family: AddressFamily, ip: String, port: u16) -> Self {
        SocketAddr { family, port, ip }
    }
}

/// socket instance
pub struct Socket {
    pub fd: i32,
    pub family: AddressFamily,
    pub socket_type: SocketType,
    pub protocol: SocketProtocol,
    pub state: SocketState,
    pub local_addr: Option<SocketAddr>,
    pub remote_addr: Option<SocketAddr>,
    pub backlog: i32,
    pub pending_connections: Vec<Arc<Mutex<Socket>>>,
}

impl Socket {
    pub fn new(fd: i32, family: AddressFamily, socket_type: SocketType, protocol: SocketProtocol) -> Self {
        Socket {
            fd,
            family,
            socket_type,
            protocol,
            state: SocketState::Unconnected,
            local_addr: None,
            remote_addr: None,
            backlog: 0,
            pending_connections: Vec::new(),
        }
    }

    /// Bind socket to address
    pub fn bind(&mut self, addr: SocketAddr) -> Result<(), String> {
        if self.state != SocketState::Unconnected {
            return Err("Socket already bound or connected".to_string());
        }

        self.local_addr = Some(addr);
        self.state = SocketState::Bound;
        Ok(())
    }

    /// Listen for connections
    pub fn listen(&mut self, backlog: i32) -> Result<(), String> {
        if self.state != SocketState::Bound {
            return Err("Socket not bound".to_string());
        }

        if self.socket_type != SocketType::Stream {
            return Err("Only stream sockets can listen".to_string());
        }

        self.backlog = backlog;
        self.state = SocketState::Listening;
        Ok(())
    }

    /// Accept a connection
    pub fn accept(&mut self) -> Result<Arc<Mutex<Socket>>, String> {
        if self.state != SocketState::Listening {
            return Err("Socket not listening".to_string());
        }

        if self.pending_connections.is_empty() {
            return Err("No pending connections".to_string());
        }

        Ok(self.pending_connections.remove(0))
    }

    /// Connect to remote address
    pub fn connect(&mut self, addr: SocketAddr) -> Result<(), String> {
        if self.state == SocketState::Connected {
            return Err("Socket already connected".to_string());
        }

        self.remote_addr = Some(addr);
        self.state = SocketState::Connected;
        Ok(())
    }

    /// Send data
    pub fn send(&self, data: &[u8]) -> Result<usize, String> {
        if self.state != SocketState::Connected {
            return Err("Socket not connected".to_string());
        }

        // Simulate sending
        Ok(data.len())
    }

    /// Receive data
    pub fn recv(&self, _count: usize) -> Result<Vec<u8>, String> {
        if self.state != SocketState::Connected {
            return Err("Socket not connected".to_string());
        }

        // Simulate receiving (would block in real implementation)
        Ok(Vec::new())
    }

    /// Send to address (for datagram sockets)
    pub fn sendto(&self, data: &[u8], _addr: SocketAddr) -> Result<usize, String> {
        if self.socket_type != SocketType::Datagram {
            return Err("Only datagram sockets can use sendto".to_string());
        }

        Ok(data.len())
    }

    /// Receive from address (for datagram sockets)
    pub fn recvfrom(&self, _count: usize) -> Result<(Vec<u8>, SocketAddr), String> {
        if self.socket_type != SocketType::Datagram {
            return Err("Only datagram sockets can use recvfrom".to_string());
        }

        // Simulate receiving
        Ok((Vec::new(), SocketAddr::new(AddressFamily::Inet, "0.0.0.0".to_string(), 0)))
    }

    /// Close socket
    pub fn close(&mut self) {
        self.state = SocketState::Closed;
    }

    /// Get socket state
    pub fn get_state(&self) -> SocketState {
        self.state
    }
}

/// socket manager
pub struct SocketManager {
    sockets: HashMap<i32, Arc<Mutex<Socket>>>,
    next_fd: i32,
}

impl SocketManager {
    pub fn new() -> Self {
        SocketManager {
            sockets: HashMap::new(),
            next_fd: 3, // Start after stdin, stdout, stderr
        }
    }

    /// Create a new socket
    pub fn socket(&mut self, family: AddressFamily, socket_type: SocketType, protocol: SocketProtocol) -> Result<i32, String> {
        let fd = self.next_fd;
        self.next_fd += 1;

        let socket = Arc::new(Mutex::new(Socket::new(fd, family, socket_type, protocol)));
        self.sockets.insert(fd, socket);

        Ok(fd)
    }

    /// Get a socket by file descriptor
    pub fn get_socket(&self, fd: i32) -> Option<Arc<Mutex<Socket>>> {
        self.sockets.get(&fd).cloned()
    }

    /// Close a socket
    pub fn close(&mut self, fd: i32) -> Result<(), String> {
        let socket = self.sockets.remove(&fd)
            .ok_or_else(|| format!("Socket not found: {}", fd))?;
        
        let mut socket_guard = socket.lock().unwrap();
        socket_guard.close();
        Ok(())
    }

    /// Get socket count
    pub fn socket_count(&self) -> usize {
        self.sockets.len()
    }
}

impl Default for SocketManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_creation() {
        let socket = Socket::new(3, AddressFamily::Inet, SocketType::Stream, SocketProtocol::Tcp);
        assert_eq!(socket.fd, 3);
        assert_eq!(socket.state, SocketState::Unconnected);
    }

    #[test]
    fn test_socket_bind() {
        let mut socket = Socket::new(3, AddressFamily::Inet, SocketType::Stream, SocketProtocol::Tcp);
        let addr = SocketAddr::new(AddressFamily::Inet, "127.0.0.1".to_string(), 8080);
        
        socket.bind(addr).unwrap();
        assert_eq!(socket.state, SocketState::Bound);
        assert!(socket.local_addr.is_some());
    }

    #[test]
    fn test_socket_listen() {
        let mut socket = Socket::new(3, AddressFamily::Inet, SocketType::Stream, SocketProtocol::Tcp);
        let addr = SocketAddr::new(AddressFamily::Inet, "127.0.0.1".to_string(), 8080);
        
        socket.bind(addr).unwrap();
        socket.listen(10).unwrap();
        
        assert_eq!(socket.state, SocketState::Listening);
        assert_eq!(socket.backlog, 10);
    }

    #[test]
    fn test_socket_connect() {
        let mut socket = Socket::new(3, AddressFamily::Inet, SocketType::Stream, SocketProtocol::Tcp);
        let addr = SocketAddr::new(AddressFamily::Inet, "127.0.0.1".to_string(), 8080);
        
        socket.connect(addr).unwrap();
        assert_eq!(socket.state, SocketState::Connected);
    }

    #[test]
    fn test_socket_send_recv() {
        let mut socket = Socket::new(3, AddressFamily::Inet, SocketType::Stream, SocketProtocol::Tcp);
        let addr = SocketAddr::new(AddressFamily::Inet, "127.0.0.1".to_string(), 8080);
        
        socket.connect(addr).unwrap();
        
        let sent = socket.send(b"hello").unwrap();
        assert_eq!(sent, 5);
        
        let _received = socket.recv(10).unwrap();
    }

    #[test]
    fn test_socket_close() {
        let mut socket = Socket::new(3, AddressFamily::Inet, SocketType::Stream, SocketProtocol::Tcp);
        socket.close();
        
        assert_eq!(socket.state, SocketState::Closed);
    }

    #[test]
    fn test_socket_manager_creation() {
        let mut manager = SocketManager::new();
        
        let fd = manager.socket(AddressFamily::Inet, SocketType::Stream, SocketProtocol::Tcp).unwrap();
        assert_eq!(fd, 3);
        assert_eq!(manager.socket_count(), 1);
    }

    #[test]
    fn test_socket_manager_get() {
        let mut manager = SocketManager::new();
        
        let fd = manager.socket(AddressFamily::Inet, SocketType::Stream, SocketProtocol::Tcp).unwrap();
        let socket = manager.get_socket(fd);
        
        assert!(socket.is_some());
    }

    #[test]
    fn test_socket_manager_close() {
        let mut manager = SocketManager::new();
        
        let fd = manager.socket(AddressFamily::Inet, SocketType::Stream, SocketProtocol::Tcp).unwrap();
        manager.close(fd).unwrap();
        
        assert_eq!(manager.socket_count(), 0);
    }

    #[test]
    fn test_socket_datagram_sendto() {
        let socket = Socket::new(3, AddressFamily::Inet, SocketType::Datagram, SocketProtocol::Udp);
        let addr = SocketAddr::new(AddressFamily::Inet, "127.0.0.1".to_string(), 8080);
        
        let sent = socket.sendto(b"hello", addr).unwrap();
        assert_eq!(sent, 5);
    }

    #[test]
    fn test_socket_datagram_recvfrom() {
        let socket = Socket::new(3, AddressFamily::Inet, SocketType::Datagram, SocketProtocol::Udp);
        
        let (data, addr) = socket.recvfrom(10).unwrap();
        assert_eq!(data.len(), 0);
        assert_eq!(addr.family, AddressFamily::Inet);
    }
}
