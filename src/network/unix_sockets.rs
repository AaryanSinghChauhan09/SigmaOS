// UNIX Domain Sockets (AF_UNIX / PF_LOCAL IPC)
// Native path-based and abstract inter-process socket communication inspired by Linux and BSD.

use crate::klib::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnixSocketState {
    Unbound,
    Listening,
    Connecting,
    Connected,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnixSocketAddress {
    Path(String),
    Abstract(String),
}

pub struct UnixSocketConn {
    pub address: UnixSocketAddress,
    pub state: UnixSocketState,
    pub tx_buffer: Vec<u8>,
    pub rx_buffer: Vec<u8>,
    pub peer_address: Option<UnixSocketAddress>,
}

impl UnixSocketConn {
    pub fn new(address: UnixSocketAddress) -> Self {
        Self {
            address,
            state: UnixSocketState::Unbound,
            tx_buffer: Vec::new(),
            rx_buffer: Vec::new(),
            peer_address: None,
        }
    }

    pub fn listen(&mut self) -> Result<(), &'static str> {
        if self.state != UnixSocketState::Unbound {
            return Err("Socket is already bound or active");
        }
        self.state = UnixSocketState::Listening;
        Ok(())
    }

    pub fn connect_to(&mut self, peer_addr: UnixSocketAddress) -> Result<(), &'static str> {
        self.peer_address = Some(peer_addr);
        self.state = UnixSocketState::Connected;
        Ok(())
    }

    pub fn write_data(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.state != UnixSocketState::Connected {
            return Err("Socket is not connected");
        }
        self.tx_buffer.extend_from_slice(data);
        Ok(data.len())
    }

    pub fn read_data(&mut self, limit: usize) -> Result<Vec<u8>, &'static str> {
        if self.state != UnixSocketState::Connected {
            return Err("Socket is not connected");
        }
        let read_len = self.rx_buffer.len().min(limit);
        let data = self.rx_buffer.drain(0..read_len).collect();
        Ok(data)
    }

    pub fn close(&mut self) {
        self.state = UnixSocketState::Closed;
        self.tx_buffer.clear();
        self.rx_buffer.clear();
    }
}

pub struct UnixSocketRegistry {
    pub sockets: HashMap<UnixSocketAddress, UnixSocketConn>,
}

impl UnixSocketRegistry {
    pub fn new() -> Self {
        Self {
            sockets: HashMap::new(),
        }
    }

    /// Bind a new socket to an address
    pub fn bind(&mut self, address: UnixSocketAddress) -> Result<(), &'static str> {
        if self.sockets.contains_key(&address) {
            return Err("Address already in use");
        }
        let socket = UnixSocketConn::new(address.clone());
        self.sockets.insert(address, socket);
        Ok(())
    }

    /// Listen on a bound socket
    pub fn listen(&mut self, address: &UnixSocketAddress) -> Result<(), &'static str> {
        let socket = self.sockets.get_mut(address).ok_or("Socket not found")?;
        socket.listen()
    }

    /// Connect a client socket to a listening server socket
    pub fn connect(&mut self, client_addr: UnixSocketAddress, server_addr: UnixSocketAddress) -> Result<(), &'static str> {
        // 1. Ensure server is listening
        {
            let server = self.sockets.get(&server_addr).ok_or("Server socket not found")?;
            if server.state != UnixSocketState::Listening {
                return Err("Server socket is not listening");
            }
        }

        // 2. Bind and connect client
        self.bind(client_addr.clone())?;
        let client = self.sockets.get_mut(&client_addr).unwrap();
        client.connect_to(server_addr.clone())?;

        // 3. Spawn a connected endpoint on server representing client peer
        let mut server_peer = UnixSocketConn::new(server_addr.clone());
        server_peer.state = UnixSocketState::Connected;
        server_peer.peer_address = Some(client_addr);

        // Register the server-side endpoint with a unique abstract address
        let peer_address = UnixSocketAddress::Abstract(format!("peer-{:?}", server_addr));
        self.sockets.insert(peer_address, server_peer);

        Ok(())
    }

    /// Helper to bridge / pipe data packets from client TX buffer to server peer RX buffer
    pub fn pipe_packets(&mut self, sender_addr: &UnixSocketAddress, receiver_addr: &UnixSocketAddress) -> Result<usize, &'static str> {
        let mut data = Vec::new();
        if let Some(sender) = self.sockets.get_mut(sender_addr) {
            data = sender.tx_buffer.drain(..).collect();
        }

        let len = data.len();
        if len > 0 {
            if let Some(receiver) = self.sockets.get_mut(receiver_addr) {
                receiver.rx_buffer.extend_from_slice(&data);
            }
        }

        Ok(len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_unbound_lifecycle() {
        let address = UnixSocketAddress::Path("/var/run/test.sock".to_string());
        let mut sock = UnixSocketConn::new(address);
        assert_eq!(sock.state, UnixSocketState::Unbound);

        sock.listen().unwrap();
        assert_eq!(sock.state, UnixSocketState::Listening);
    }

    #[test]
    fn test_socket_registry_binding_errors() {
        let mut registry = UnixSocketRegistry::new();
        let address = UnixSocketAddress::Path("/var/run/sigma.sock".to_string());

        assert!(registry.bind(address.clone()).is_ok());
        // Duplicate bind must fail
        assert!(registry.bind(address).is_err());
    }

    #[test]
    fn test_local_connection_packet_pipe() {
        let mut registry = UnixSocketRegistry::new();
        let server_addr = UnixSocketAddress::Path("/var/run/server.sock".to_string());
        let client_addr = UnixSocketAddress::Path("/var/run/client.sock".to_string());

        // 1. Bind and listen server
        registry.bind(server_addr.clone()).unwrap();
        registry.listen(&server_addr).unwrap();

        // 2. Connect client to server
        registry.connect(client_addr.clone(), server_addr.clone()).unwrap();

        // Check client is connected
        let client = registry.sockets.get(&client_addr).unwrap();
        assert_eq!(client.state, UnixSocketState::Connected);
        assert_eq!(client.peer_address, Some(server_addr.clone()));

        // 3. Write data from client
        let mut client_mut = registry.sockets.get_mut(&client_addr).unwrap();
        client_mut.write_data(b"Hello Local Server!").unwrap();

        // 4. Pipe packets to server peer
        let server_peer_addr = UnixSocketAddress::Abstract(format!("peer-{:?}", server_addr));
        registry.pipe_packets(&client_addr, &server_peer_addr).unwrap();

        // 5. Read data on server peer
        let server_peer = registry.sockets.get_mut(&server_peer_addr).unwrap();
        let data = server_peer.read_data(100).unwrap();
        assert_eq!(data, b"Hello Local Server!");
    }
}
