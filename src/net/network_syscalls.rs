//! Network System Calls Implementation
//!
//! Provides socket family syscalls with network namespace support (CLONE_NEWNET).
//! Implements socket(2), bind(2), listen(2), accept(2), connect(2) with per-namespace isolation.

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr as StdSocketAddr};
use std::sync::{Arc, Mutex, atomic::{AtomicU32, Ordering}};
use super::network_namespace::NetworkNamespaceId;

/// Socket file descriptor type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketFd(u32);

impl SocketFd {
    pub fn new(fd: u32) -> Self {
        SocketFd(fd)
    }

    pub fn raw(&self) -> u32 {
        self.0
    }
}

/// CLONE_NEWNET flag for namespace isolation
pub const CLONE_NEWNET: u32 = 0x40000000;

/// Socket domain constants (POSIX)
pub const AF_UNIX: u32 = 1;
pub const AF_INET: u32 = 2;
pub const AF_INET6: u32 = 10;

/// Socket type constants
pub const SOCK_STREAM: u32 = 1;  // TCP
pub const SOCK_DGRAM: u32 = 2;   // UDP
pub const SOCK_RAW: u32 = 3;     // Raw

/// Protocol constants
pub const IPPROTO_TCP: u32 = 6;
pub const IPPROTO_UDP: u32 = 17;
pub const IPPROTO_IP: u32 = 0;

/// Socket address structure for binding
#[derive(Debug, Clone)]
pub struct SockAddr {
    pub family: u32,
    pub address: IpAddr,
    pub port: u16,
}

impl SockAddr {
    pub fn new_ipv4(ip: Ipv4Addr, port: u16) -> Self {
        SockAddr {
            family: AF_INET,
            address: IpAddr::V4(ip),
            port,
        }
    }

    pub fn to_std(&self) -> Result<StdSocketAddr, String> {
        match self.address {
            IpAddr::V4(ip) => Ok(StdSocketAddr::new(IpAddr::V4(ip), self.port)),
            IpAddr::V6(ip) => Ok(StdSocketAddr::new(IpAddr::V6(ip), self.port)),
        }
    }
}

/// Socket state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Created,
    Bound,
    Listening,
    Connected,
    Closed,
}

/// Per-socket metadata
#[derive(Debug, Clone)]
pub struct SocketMetadata {
    pub fd: SocketFd,
    pub namespace_id: NetworkNamespaceId,
    pub domain: u32,
    pub socket_type: u32,
    pub protocol: u32,
    pub state: SocketState,
    pub local_addr: Option<SockAddr>,
    pub remote_addr: Option<SockAddr>,
    pub backlog: i32,
}

impl SocketMetadata {
    pub fn new(
        fd: SocketFd,
        namespace_id: NetworkNamespaceId,
        domain: u32,
        socket_type: u32,
        protocol: u32,
    ) -> Self {
        SocketMetadata {
            fd,
            namespace_id,
            domain,
            socket_type,
            protocol,
            state: SocketState::Created,
            local_addr: None,
            remote_addr: None,
            backlog: 0,
        }
    }
}

/// Per-namespace socket table
#[derive(Debug, Clone)]
pub struct NamespaceSocketTable {
    pub namespace_id: NetworkNamespaceId,
    pub sockets: Arc<Mutex<HashMap<u32, SocketMetadata>>>,
    pub fd_counter: Arc<AtomicU32>,
}

impl NamespaceSocketTable {
    pub fn new(namespace_id: NetworkNamespaceId) -> Self {
        NamespaceSocketTable {
            namespace_id,
            sockets: Arc::new(Mutex::new(HashMap::new())),
            fd_counter: Arc::new(AtomicU32::new(3)), // Start from 3 (0,1,2 are stdin/stdout/stderr)
        }
    }

    pub fn allocate_fd(&self) -> u32 {
        self.fd_counter.fetch_add(1, Ordering::SeqCst)
    }

    pub fn add_socket(&self, metadata: SocketMetadata) -> Result<(), String> {
        let mut sockets = self.sockets.lock().map_err(|e| e.to_string())?;
        if sockets.contains_key(&metadata.fd.raw()) {
            return Err(format!("Socket {} already exists", metadata.fd.raw()));
        }
        sockets.insert(metadata.fd.raw(), metadata);
        Ok(())
    }

    pub fn get_socket(&self, fd: SocketFd) -> Result<SocketMetadata, String> {
        let sockets = self.sockets.lock().map_err(|e| e.to_string())?;
        sockets.get(&fd.raw())
            .cloned()
            .ok_or_else(|| format!("Socket {} not found", fd.raw()))
    }

    pub fn update_socket(&self, metadata: SocketMetadata) -> Result<(), String> {
        let mut sockets = self.sockets.lock().map_err(|e| e.to_string())?;
        sockets.insert(metadata.fd.raw(), metadata);
        Ok(())
    }

    pub fn remove_socket(&self, fd: SocketFd) -> Result<(), String> {
        let mut sockets = self.sockets.lock().map_err(|e| e.to_string())?;
        sockets.remove(&fd.raw());
        Ok(())
    }

    pub fn list_sockets(&self) -> Result<Vec<SocketMetadata>, String> {
        let sockets = self.sockets.lock().map_err(|e| e.to_string())?;
        Ok(sockets.values().cloned().collect())
    }

    pub fn count(&self) -> Result<usize, String> {
        let sockets = self.sockets.lock().map_err(|e| e.to_string())?;
        Ok(sockets.len())
    }
}

/// Global socket syscall manager
pub struct NetworkSyscalls {
    namespace_tables: Arc<Mutex<HashMap<NetworkNamespaceId, NamespaceSocketTable>>>,
}

impl NetworkSyscalls {
    pub fn new() -> Self {
        NetworkSyscalls {
            namespace_tables: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Ensure namespace has a socket table
    pub fn ensure_namespace(&self, ns_id: NetworkNamespaceId) -> Result<(), String> {
        let mut tables = self.namespace_tables.lock().map_err(|e| e.to_string())?;
        if !tables.contains_key(&ns_id) {
            tables.insert(ns_id, NamespaceSocketTable::new(ns_id));
        }
        Ok(())
    }

    /// Get socket table for namespace
    pub fn get_namespace_table(&self, ns_id: NetworkNamespaceId) -> Result<NamespaceSocketTable, String> {
        let tables = self.namespace_tables.lock().map_err(|e| e.to_string())?;
        tables.get(&ns_id)
            .cloned()
            .ok_or_else(|| format!("No socket table for namespace {:?}", ns_id))
    }

    /// socket(2) - Create a socket with namespace support
    pub fn sys_socket(
        &self,
        domain: u32,
        socket_type: u32,
        protocol: u32,
        namespace_id: NetworkNamespaceId,
    ) -> Result<SocketFd, String> {
        // Validate domain
        match domain {
            AF_INET | AF_INET6 | AF_UNIX => {}
            _ => return Err(format!("Invalid address family: {}", domain)),
        }

        // Validate socket type
        match socket_type {
            SOCK_STREAM | SOCK_DGRAM | SOCK_RAW => {}
            _ => return Err(format!("Invalid socket type: {}", socket_type)),
        }

        // Validate protocol
        match protocol {
            0 | IPPROTO_TCP | IPPROTO_UDP => {}
            _ => return Err(format!("Invalid protocol: {}", protocol)),
        }

        self.ensure_namespace(namespace_id)?;
        let table = self.get_namespace_table(namespace_id)?;

        let fd_raw = table.allocate_fd();
        let fd = SocketFd::new(fd_raw);
        let metadata = SocketMetadata::new(fd, namespace_id, domain, socket_type, protocol);
        table.add_socket(metadata)?;

        Ok(fd)
    }

    /// bind(2) - Bind socket to address with namespace isolation
    pub fn sys_bind(
        &self,
        fd: SocketFd,
        sockaddr: SockAddr,
        namespace_id: NetworkNamespaceId,
    ) -> Result<(), String> {
        let table = self.get_namespace_table(namespace_id)?;
        let mut metadata = table.get_socket(fd)?;

        // Verify socket belongs to this namespace
        if metadata.namespace_id != namespace_id {
            return Err("Socket does not belong to this namespace".to_string());
        }

        // Cannot bind twice
        if metadata.state != SocketState::Created {
            return Err("Socket already bound".to_string());
        }

        metadata.local_addr = Some(sockaddr);
        metadata.state = SocketState::Bound;
        table.update_socket(metadata)?;

        Ok(())
    }

    /// listen(2) - Listen for incoming connections
    pub fn sys_listen(
        &self,
        fd: SocketFd,
        backlog: i32,
        namespace_id: NetworkNamespaceId,
    ) -> Result<(), String> {
        let table = self.get_namespace_table(namespace_id)?;
        let mut metadata = table.get_socket(fd)?;

        // Must be bound before listening
        if metadata.state != SocketState::Bound {
            return Err("Socket not bound".to_string());
        }

        // Only stream sockets can listen
        if metadata.socket_type != SOCK_STREAM {
            return Err("Only SOCK_STREAM sockets can listen".to_string());
        }

        metadata.backlog = backlog;
        metadata.state = SocketState::Listening;
        table.update_socket(metadata)?;

        Ok(())
    }

    /// accept(2) - Accept incoming connection
    pub fn sys_accept(
        &self,
        fd: SocketFd,
        namespace_id: NetworkNamespaceId,
    ) -> Result<(SocketFd, SockAddr), String> {
        let table = self.get_namespace_table(namespace_id)?;
        let metadata = table.get_socket(fd)?;

        // Must be listening
        if metadata.state != SocketState::Listening {
            return Err("Socket not listening".to_string());
        }

        // Create new socket for connection
        let new_fd_raw = table.allocate_fd();
        let new_fd = SocketFd::new(new_fd_raw);

        let mut new_metadata = SocketMetadata::new(
            new_fd,
            namespace_id,
            metadata.domain,
            metadata.socket_type,
            metadata.protocol,
        );

        // Copy local address and set state
        new_metadata.local_addr = metadata.local_addr.clone();
        new_metadata.state = SocketState::Connected;

        // Create dummy remote address
        let remote = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 54321);
        new_metadata.remote_addr = Some(remote.clone());

        table.add_socket(new_metadata)?;

        Ok((new_fd, remote))
    }

    /// connect(2) - Connect to remote address
    pub fn sys_connect(
        &self,
        fd: SocketFd,
        sockaddr: SockAddr,
        namespace_id: NetworkNamespaceId,
    ) -> Result<(), String> {
        let table = self.get_namespace_table(namespace_id)?;
        let mut metadata = table.get_socket(fd)?;

        // Must be in Created or Bound state
        if metadata.state != SocketState::Created && metadata.state != SocketState::Bound {
            return Err(format!("Invalid socket state: {:?}", metadata.state));
        }

        metadata.remote_addr = Some(sockaddr);
        metadata.state = SocketState::Connected;
        table.update_socket(metadata)?;

        Ok(())
    }

    /// close(2) - Close socket
    pub fn sys_close(
        &self,
        fd: SocketFd,
        namespace_id: NetworkNamespaceId,
    ) -> Result<(), String> {
        let table = self.get_namespace_table(namespace_id)?;
        let mut metadata = table.get_socket(fd)?;

        metadata.state = SocketState::Closed;
        table.update_socket(metadata)?;

        Ok(())
    }

    /// Get socket information
    pub fn sys_getsockname(
        &self,
        fd: SocketFd,
        namespace_id: NetworkNamespaceId,
    ) -> Result<SockAddr, String> {
        let table = self.get_namespace_table(namespace_id)?;
        let metadata = table.get_socket(fd)?;

        metadata.local_addr.ok_or_else(|| "Socket not bound".to_string())
    }

    /// Get peer address
    pub fn sys_getpeername(
        &self,
        fd: SocketFd,
        namespace_id: NetworkNamespaceId,
    ) -> Result<SockAddr, String> {
        let table = self.get_namespace_table(namespace_id)?;
        let metadata = table.get_socket(fd)?;

        metadata.remote_addr.ok_or_else(|| "Socket not connected".to_string())
    }
}

impl Default for NetworkSyscalls {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_creation() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket");
        assert_ne!(fd.raw(), 0);
    }

    #[test]
    fn test_socket_bind() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket");

        let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
        assert!(syscalls.sys_bind(fd, addr, ns_id).is_ok());
    }

    #[test]
    fn test_socket_listen() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket");

        let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
        syscalls.sys_bind(fd, addr, ns_id).expect("Failed to bind");

        assert!(syscalls.sys_listen(fd, 5, ns_id).is_ok());
    }

    #[test]
    fn test_socket_accept() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket");

        let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
        syscalls.sys_bind(fd, addr, ns_id).expect("Failed to bind");
        syscalls.sys_listen(fd, 5, ns_id).expect("Failed to listen");

        let (conn_fd, _peer_addr) = syscalls.sys_accept(fd, ns_id)
            .expect("Failed to accept");
        assert_ne!(conn_fd.raw(), fd.raw());
    }

    #[test]
    fn test_socket_connect() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket");

        let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
        assert!(syscalls.sys_connect(fd, addr, ns_id).is_ok());
    }

    #[test]
    fn test_namespace_isolation() {
        let syscalls = NetworkSyscalls::new();
        let ns1 = NetworkNamespaceId::new(1);
        let ns2 = NetworkNamespaceId::new(2);

        let fd1 = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns1)
            .expect("Failed to create socket in ns1");
        let fd2 = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns2)
            .expect("Failed to create socket in ns2");

        // Same FD number in different namespaces should be allowed
        assert_eq!(fd1.raw(), fd2.raw());

        // But they should be different sockets in their respective namespaces
        let addr1 = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
        let addr2 = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 2), 8081);

        syscalls.sys_bind(fd1, addr1, ns1).expect("Failed to bind in ns1");
        syscalls.sys_bind(fd2, addr2, ns2).expect("Failed to bind in ns2");

        let retrieved1 = syscalls.sys_getsockname(fd1, ns1).expect("Failed to get name in ns1");
        let retrieved2 = syscalls.sys_getsockname(fd2, ns2).expect("Failed to get name in ns2");

        assert_eq!(retrieved1.port, 8080);
        assert_eq!(retrieved2.port, 8081);
    }

    #[test]
    fn test_socket_close() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket");

        assert!(syscalls.sys_close(fd, ns_id).is_ok());
    }

    #[test]
    fn test_invalid_domain() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let result = syscalls.sys_socket(999, SOCK_STREAM, IPPROTO_TCP, ns_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_socket_type() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let result = syscalls.sys_socket(AF_INET, 999, IPPROTO_TCP, ns_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_getpeername() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket");

        let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
        syscalls.sys_connect(fd, addr.clone(), ns_id).expect("Failed to connect");

        let peer = syscalls.sys_getpeername(fd, ns_id).expect("Failed to get peer name");
        assert_eq!(peer.port, 8080);
    }

    #[test]
    fn test_socket_state_transitions() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket");

        // Verify initial state
        let metadata = {
            let table = syscalls.get_namespace_table(ns_id).unwrap();
            table.get_socket(fd).unwrap()
        };
        assert_eq!(metadata.state, SocketState::Created);

        // Bind changes state to Bound
        let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
        syscalls.sys_bind(fd, addr, ns_id).expect("Failed to bind");

        let metadata = {
            let table = syscalls.get_namespace_table(ns_id).unwrap();
            table.get_socket(fd).unwrap()
        };
        assert_eq!(metadata.state, SocketState::Bound);

        // Listen changes state to Listening
        syscalls.sys_listen(fd, 5, ns_id).expect("Failed to listen");

        let metadata = {
            let table = syscalls.get_namespace_table(ns_id).unwrap();
            table.get_socket(fd).unwrap()
        };
        assert_eq!(metadata.state, SocketState::Listening);
    }

    #[test]
    fn test_multiple_sockets_per_namespace() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd1 = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket 1");
        let fd2 = syscalls.sys_socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP, ns_id)
            .expect("Failed to create socket 2");
        let fd3 = syscalls.sys_socket(AF_INET6, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket 3");

        assert_ne!(fd1.raw(), fd2.raw());
        assert_ne!(fd2.raw(), fd3.raw());

        let table = syscalls.get_namespace_table(ns_id).expect("Failed to get table");
        let count = table.count().expect("Failed to count");
        assert_eq!(count, 3);
    }
}
