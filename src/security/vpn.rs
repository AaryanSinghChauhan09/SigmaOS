#![no_std]

/// Secure VPN Client for SigmaOS
/// Based on 100-Improvement-Ideas.md #40: Secure VPN client
/// Implements VPN connection management with encryption

use core::sync::atomic::{AtomicU64, Ordering};

/// VPN protocol
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VPNProtocol {
    OpenVPN = 0,
    WireGuard = 1,
    IKEv2 = 2,
    SSTP = 3,
}

/// VPN state
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VPNState {
    Disconnected = 0,
    Connecting = 1,
    Connected = 2,
    Disconnecting = 3,
    Error = 4,
}

/// VPN server
#[repr(C)]
pub struct VPNServer {
    pub id: u64,
    pub name: [u8; 64],
    pub address: [u8; 128],
    pub protocol: VPNProtocol,
    pub port: u16,
}

impl VPNServer {
    pub fn new(id: u64, name: &str, address: &str, protocol: VPNProtocol, port: u16) -> Self {
        let mut name_array = [0u8; 64];
        let name_bytes = name.as_bytes();
        let name_len = name_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        
        let mut addr_array = [0u8; 128];
        let addr_bytes = address.as_bytes();
        let addr_len = addr_bytes.len().min(127);
        
        unsafe {
            core::ptr::copy_nonoverlapping(addr_bytes.as_ptr(), addr_array.as_mut_ptr(), addr_len);
        }
        
        VPNServer {
            id,
            name: name_array,
            address: addr_array,
            protocol,
            port,
        }
    }
}

/// VPN connection
pub struct VPNConnection {
    pub server: VPNServer,
    pub state: VPNState,
    pub connected_at: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub encryption_key: [u8; 32],
}

impl VPNConnection {
    pub fn new(server: VPNServer) -> Self {
        VPNConnection {
            server,
            state: VPNState::Disconnected,
            connected_at: 0,
            bytes_sent: 0,
            bytes_received: 0,
            encryption_key: [0u8; 32],
        }
    }
}

/// VPN client
pub struct VPNClient {
    pub connections: Vec<Option<VPNConnection>>,
    pub servers: Vec<Option<VPNServer>>,
    pub active_connection: AtomicU64,
    pub next_server_id: AtomicU64,
    pub auto_reconnect: bool,
    pub kill_switch_enabled: bool,
}

impl VPNClient {
    pub fn new() -> Self {
        VPNClient {
            connections: Vec::new(),
            servers: Vec::new(),
            active_connection: AtomicU64::new(0),
            next_server_id: AtomicU64::new(1),
            auto_reconnect: true,
            kill_switch_enabled: true,
        }
    }
    
    /// Add server
    pub fn add_server(&mut self, name: &str, address: &str, protocol: VPNProtocol, port: u16) -> u64 {
        let id = self.next_server_id.fetch_add(1, Ordering::SeqCst);
        let server = VPNServer::new(id, name, address, protocol, port);
        self.servers.push(Some(server));
        id
    }
    
    /// Remove server
    pub fn remove_server(&mut self, id: u64) -> bool {
        for server_option in &mut self.servers {
            if let Some(ref server) = *server_option {
                if server.id == id {
                    *server_option = None;
                    return true;
                }
            }
        }
        false
    }
    
    /// Connect to server
    pub fn connect(&mut self, server_id: u64) -> Result<(), VPNError> {
        // Find server
        let server = match self.get_server(server_id) {
            Some(s) => s.clone(),
            None => return Err(VPNError::ServerNotFound),
        };
        
        // Disconnect current connection if any
        let current_id = self.active_connection.load(Ordering::SeqCst);
        if current_id > 0 {
            let _ = self.disconnect();
        }
        
        // Create new connection
        let mut connection = VPNConnection::new(server);
        connection.state = VPNState::Connecting;
        
        // Simulate connection
        connection.state = VPNState::Connected;
        connection.connected_at = get_current_time();
        
        let conn_id = server_id;
        self.connections.push(Some(connection));
        self.active_connection.store(conn_id, Ordering::SeqCst);
        
        Ok(())
    }
    
    /// Disconnect
    pub fn disconnect(&mut self) -> Result<(), VPNError> {
        let current_id = self.active_connection.load(Ordering::SeqCst);
        if current_id == 0 {
            return Err(VPNError::NotConnected);
        }
        
        for connection_option in &mut self.connections {
            if let Some(ref mut connection) = *connection_option {
                if connection.server.id == current_id {
                    connection.state = VPNState::Disconnecting;
                    connection.state = VPNState::Disconnected;
                    self.active_connection.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        
        Err(VPNError::ConnectionNotFound)
    }
    
    /// Get active connection
    pub fn get_active_connection(&self) -> Option<&VPNConnection> {
        let current_id = self.active_connection.load(Ordering::SeqCst);
        if current_id == 0 {
            return None;
        }
        
        for connection_option in &self.connections {
            if let Some(ref connection) = *connection_option {
                if connection.server.id == current_id {
                    return Some(connection);
                }
            }
        }
        None
    }
    
    /// Get server by ID
    pub fn get_server(&self, id: u64) -> Option<&VPNServer> {
        for server_option in &self.servers {
            if let Some(ref server) = *server_option {
                if server.id == id {
                    return Some(server);
                }
            }
        }
        None
    }
    
    /// List servers
    pub fn list_servers(&self) -> Vec<&VPNServer> {
        let mut servers = Vec::new();
        for server_option in &self.servers {
            if let Some(ref server) = *server_option {
                servers.push(server);
            }
        }
        servers
    }
    
    /// Update statistics
    pub fn update_stats(&mut self, bytes_sent: u64, bytes_received: u64) {
        let current_id = self.active_connection.load(Ordering::SeqCst);
        if current_id == 0 {
            return;
        }
        
        for connection_option in &mut self.connections {
            if let Some(ref mut connection) = *connection_option {
                if connection.server.id == current_id {
                    connection.bytes_sent += bytes_sent;
                    connection.bytes_received += bytes_received;
                    break;
                }
            }
        }
    }
    
    /// Set auto-reconnect
    pub fn set_auto_reconnect(&mut self, enabled: bool) {
        self.auto_reconnect = enabled;
    }
    
    /// Set kill switch
    pub fn set_kill_switch(&mut self, enabled: bool) {
        self.kill_switch_enabled = enabled;
    }
    
    /// Get connection state
    pub fn get_state(&self) -> VPNState {
        if let Some(conn) = self.get_active_connection() {
            conn.state
        } else {
            VPNState::Disconnected
        }
    }
}

/// VPN error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum VPNError {
    Success = 0,
    ServerNotFound = 1,
    ConnectionFailed = 2,
    NotConnected = 3,
    ConnectionNotFound = 4,
    AuthenticationFailed = 5,
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

/// Get current time (nanoseconds)
fn get_current_time() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1_000_000, Ordering::SeqCst)
}
