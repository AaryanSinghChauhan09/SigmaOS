// Linux-inspired socket for network communication
// Provides socket abstraction for TCP/UDP protocols

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Socket domain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketDomain {
    Unix = 1,
    IPv4 = 2,
    IPv6 = 10,
}

/// Socket type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Stream = 1,    // TCP
    Datagram = 2,   // UDP
    Raw = 3,
}

/// Socket protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketProtocol {
    IP = 0,
    TCP = 6,
    UDP = 17,
}

/// Socket state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAck,
    TimeWait,
}

/// Socket instance
#[derive(Debug, Clone)]
pub struct Socket {
    pub id: u64,
    pub domain: SocketDomain,
    pub socket_type: SocketType,
    pub protocol: SocketProtocol,
    pub state: SocketState,
    pub local_port: u16,
    pub remote_port: u16,
    pub local_address: String,
    pub remote_address: String,
    pub backlog: u32,
}

impl Socket {
    pub fn new(id: u64, domain: SocketDomain, socket_type: SocketType, protocol: SocketProtocol) -> Self {
        Self {
            id,
            domain,
            socket_type,
            protocol,
            state: SocketState::Closed,
            local_port: 0,
            remote_port: 0,
            local_address: String::new(),
            remote_address: String::new(),
            backlog: 0,
        }
    }

    /// Bind to local address
    pub fn bind(&mut self, address: String, port: u16) -> Result<(), String> {
        if self.state != SocketState::Closed {
            return Err("Socket not in closed state".to_string());
        }

        self.local_address = address;
        self.local_port = port;
        Ok(())
    }

    /// Listen for connections
    pub fn listen(&mut self, backlog: u32) -> Result<(), String> {
        if self.socket_type != SocketType::Stream {
            return Err("Only stream sockets can listen".to_string());
        }

        if self.local_port == 0 {
            return Err("Socket not bound".to_string());
        }

        self.state = SocketState::Listen;
        self.backlog = backlog;
        Ok(())
    }

    /// Accept a connection
    pub fn accept(&mut self) -> Result<Socket, String> {
        if self.state != SocketState::Listen {
            return Err("Socket not in listen state".to_string());
        }

        // Simulate accepting a connection
        let mut new_socket = Socket::new(
            self.id + 1000,
            self.domain,
            self.socket_type,
            self.protocol,
        );
        new_socket.local_address = self.local_address.clone();
        new_socket.local_port = self.local_port;
        new_socket.state = SocketState::Established;

        Ok(new_socket)
    }

    /// Connect to remote address
    pub fn connect(&mut self, address: String, port: u16) -> Result<(), String> {
        if self.socket_type != SocketType::Stream {
            return Err("Only stream sockets can connect".to_string());
        }

        self.remote_address = address;
        self.remote_port = port;
        self.state = SocketState::Established;
        Ok(())
    }

    /// Send data
    pub fn send(&self, data: &[u8]) -> Result<usize, String> {
        if self.state != SocketState::Established {
            return Err("Socket not connected".to_string());
        }

        // Simulate sending data
        Ok(data.len())
    }

    /// Receive data
    pub fn recv(&self, _count: usize) -> Result<Vec<u8>, String> {
        if self.state != SocketState::Established {
            return Err("Socket not connected".to_string());
        }

        // Simulate receiving data
        Ok(vec![])
    }

    /// Send to address (for UDP)
    pub fn sendto(&self, data: &[u8], _address: String, _port: u16) -> Result<usize, String> {
        if self.socket_type != SocketType::Datagram {
            return Err("Only datagram sockets can use sendto".to_string());
        }

        Ok(data.len())
    }

    /// Receive from address (for UDP)
    pub fn recvfrom(&self, _count: usize) -> Result<(Vec<u8>, String, u16), String> {
        if self.socket_type != SocketType::Datagram {
            return Err("Only datagram sockets can use recvfrom".to_string());
        }

        Ok((vec![], String::new(), 0))
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

/// Socket manager for system-wide socket management
pub struct SocketManager {
    sockets: Arc<Mutex<HashMap<u64, Socket>>>,
    next_socket_id: Arc<Mutex<u64>>,
}

impl SocketManager {
    pub fn new() -> Self {
        Self {
            sockets: Arc::new(Mutex::new(HashMap::new())),
            next_socket_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new socket
    pub fn create_socket(&self, domain: SocketDomain, socket_type: SocketType, protocol: SocketProtocol) -> u64 {
        let mut next_id = self.next_socket_id.lock().unwrap();
        let socket_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let socket = Socket::new(socket_id, domain, socket_type, protocol);
        let mut sockets = self.sockets.lock().unwrap();
        sockets.insert(socket_id, socket);

        socket_id
    }

    /// Get a socket by ID
    pub fn get_socket(&self, socket_id: u64) -> Option<Socket> {
        let sockets = self.sockets.lock().unwrap();
        sockets.get(&socket_id).cloned()
    }

    /// Remove a socket
    pub fn remove_socket(&self, socket_id: u64) -> Result<(), String> {
        let mut sockets = self.sockets.lock().unwrap();
        match sockets.remove(&socket_id) {
            Some(_) => Ok(()),
            None => Err(format!("Socket {} not found", socket_id)),
        }
    }

    /// Bind socket
    pub fn bind(&self, socket_id: u64, address: String, port: u16) -> Result<(), String> {
        let mut sockets = self.sockets.lock().unwrap();
        match sockets.get_mut(&socket_id) {
            Some(socket) => socket.bind(address, port),
            None => Err(format!("Socket {} not found", socket_id)),
        }
    }

    /// Listen on socket
    pub fn listen(&self, socket_id: u64, backlog: u32) -> Result<(), String> {
        let mut sockets = self.sockets.lock().unwrap();
        match sockets.get_mut(&socket_id) {
            Some(socket) => socket.listen(backlog),
            None => Err(format!("Socket {} not found", socket_id)),
        }
    }

    /// Accept connection
    pub fn accept(&self, socket_id: u64) -> Result<Socket, String> {
        let mut sockets = self.sockets.lock().unwrap();
        match sockets.get_mut(&socket_id) {
            Some(socket) => {
                let new_socket = socket.accept()?;
                let new_id = new_socket.id;
                sockets.insert(new_id, new_socket.clone());
                Ok(new_socket)
            }
            None => Err(format!("Socket {} not found", socket_id)),
        }
    }

    /// Connect socket
    pub fn connect(&self, socket_id: u64, address: String, port: u16) -> Result<(), String> {
        let mut sockets = self.sockets.lock().unwrap();
        match sockets.get_mut(&socket_id) {
            Some(socket) => socket.connect(address, port),
            None => Err(format!("Socket {} not found", socket_id)),
        }
    }

    /// Send data
    pub fn send(&self, socket_id: u64, data: &[u8]) -> Result<usize, String> {
        let sockets = self.sockets.lock().unwrap();
        match sockets.get(&socket_id) {
            Some(socket) => socket.send(data),
            None => Err(format!("Socket {} not found", socket_id)),
        }
    }

    /// Receive data
    pub fn recv(&self, socket_id: u64, count: usize) -> Result<Vec<u8>, String> {
        let sockets = self.sockets.lock().unwrap();
        match sockets.get(&socket_id) {
            Some(socket) => socket.recv(count),
            None => Err(format!("Socket {} not found", socket_id)),
        }
    }

    /// Close socket
    pub fn close(&self, socket_id: u64) -> Result<(), String> {
        let mut sockets = self.sockets.lock().unwrap();
        match sockets.get_mut(&socket_id) {
            Some(socket) => {
                socket.close();
                Ok(())
            }
            None => Err(format!("Socket {} not found", socket_id)),
        }
    }

    /// Get socket count
    pub fn socket_count(&self) -> usize {
        let sockets = self.sockets.lock().unwrap();
        sockets.len()
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
    fn test_socket() {
        let socket = Socket::new(1, SocketDomain::IPv4, SocketType::Stream, SocketProtocol::TCP);
        assert_eq!(socket.id, 1);
        assert_eq!(socket.state, SocketState::Closed);
    }

    #[test]
    fn test_socket_bind() {
        let mut socket = Socket::new(1, SocketDomain::IPv4, SocketType::Stream, SocketProtocol::TCP);
        socket.bind("127.0.0.1".to_string(), 8080).unwrap();

        assert_eq!(socket.local_address, "127.0.0.1");
        assert_eq!(socket.local_port, 8080);
    }

    #[test]
    fn test_socket_bind_not_closed() {
        let mut socket = Socket::new(1, SocketDomain::IPv4, SocketType::Stream, SocketProtocol::TCP);
        socket.bind("127.0.0.1".to_string(), 8080).unwrap();
        assert!(socket.bind("127.0.0.1".to_string(), 8081).is_err());
    }

    #[test]
    fn test_socket_listen() {
        let mut socket = Socket::new(1, SocketDomain::IPv4, SocketType::Stream, SocketProtocol::TCP);
        socket.bind("127.0.0.1".to_string(), 8080).unwrap();
        socket.listen(128).unwrap();

        assert_eq!(socket.state, SocketState::Listen);
        assert_eq!(socket.backlog, 128);
    }

    #[test]
    fn test_socket_listen_not_stream() {
        let mut socket = Socket::new(1, SocketDomain::IPv4, SocketType::Datagram, SocketProtocol::UDP);
        assert!(socket.listen(128).is_err());
    }

    #[test]
    fn test_socket_connect() {
        let mut socket = Socket::new(1, SocketDomain::IPv4, SocketType::Stream, SocketProtocol::TCP);
        socket.connect("192.168.1.1".to_string(), 80).unwrap();

        assert_eq!(socket.remote_address, "192.168.1.1");
        assert_eq!(socket.remote_port, 80);
        assert_eq!(socket.state, SocketState::Established);
    }

    #[test]
    fn test_socket_send() {
        let mut socket = Socket::new(1, SocketDomain::IPv4, SocketType::Stream, SocketProtocol::TCP);
        socket.connect("192.168.1.1".to_string(), 80).unwrap();
        
        let data = b"Hello";
        let sent = socket.send(data).unwrap();
        assert_eq!(sent, data.len());
    }

    #[test]
    fn test_socket_send_not_connected() {
        let socket = Socket::new(1, SocketDomain::IPv4, SocketType::Stream, SocketProtocol::TCP);
        // Socket is in Closed state, not Established
        assert!(socket.send(b"Hello").is_err());
    }

    #[test]
    fn test_socket_close() {
        let mut socket = Socket::new(1, SocketDomain::IPv4, SocketType::Stream, SocketProtocol::TCP);
        socket.connect("192.168.1.1".to_string(), 80).unwrap();
        socket.close();

        assert_eq!(socket.state, SocketState::Closed);
    }

    #[test]
    fn test_socket_manager() {
        let manager = SocketManager::new();

        let socket_id = manager.create_socket(SocketDomain::IPv4, SocketType::Stream, SocketProtocol::TCP);
        assert_eq!(socket_id, 1);

        assert_eq!(manager.socket_count(), 1);
    }

    #[test]
    fn test_socket_manager_bind_listen() {
        let manager = SocketManager::new();

        let socket_id = manager.create_socket(SocketDomain::IPv4, SocketType::Stream, SocketProtocol::TCP);
        manager.bind(socket_id, "127.0.0.1".to_string(), 8080).unwrap();
        manager.listen(socket_id, 128).unwrap();

        let socket = manager.get_socket(socket_id).unwrap();
        assert_eq!(socket.state, SocketState::Listen);
    }

    #[test]
    fn test_socket_manager_connect() {
        let manager = SocketManager::new();

        let socket_id = manager.create_socket(SocketDomain::IPv4, SocketType::Stream, SocketProtocol::TCP);
        manager.connect(socket_id, "192.168.1.1".to_string(), 80).unwrap();

        let socket = manager.get_socket(socket_id).unwrap();
        assert_eq!(socket.state, SocketState::Established);
    }

    #[test]
    fn test_socket_manager_close() {
        let manager = SocketManager::new();

        let socket_id = manager.create_socket(SocketDomain::IPv4, SocketType::Stream, SocketProtocol::TCP);
        manager.close(socket_id).unwrap();

        let socket = manager.get_socket(socket_id).unwrap();
        assert_eq!(socket.state, SocketState::Closed);
    }

    #[test]
    fn test_socket_manager_remove() {
        let manager = SocketManager::new();

        let socket_id = manager.create_socket(SocketDomain::IPv4, SocketType::Stream, SocketProtocol::TCP);
        manager.remove_socket(socket_id).unwrap();

        assert_eq!(manager.socket_count(), 0);
    }

    #[test]
    fn test_socket_manager_invalid() {
        let manager = SocketManager::new();
        assert!(manager.bind(999, "127.0.0.1".to_string(), 8080).is_err());
        assert!(manager.listen(999, 128).is_err());
    }
}
