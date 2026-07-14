use crate::socket::{SigmaSocket, SocketAddr, SocketState};

/// Sovereign TCP Stack — absorbs Linux TCP/IP segment handling and Linux syscall ABI.
/// No high-level std::net, no Tokio, no async runtimes.
pub struct TcpStack {
    pub sockets: Vec<SigmaSocket>,
    next_id: u64,
}

impl Default for TcpStack {
    fn default() -> Self {
        Self::new()
    }
}

impl TcpStack {
    pub fn new() -> Self {
        Self { sockets: Vec::new(), next_id: 1 }
    }

    pub fn bind(&mut self, ip: [u8; 4], port: u16) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let mut sock = SigmaSocket::new(id, SocketAddr::new(ip, port));
        sock.listen();
        self.sockets.push(sock);
        id
    }

    pub fn get_socket(&self, id: u64) -> Option<&SigmaSocket> {
        self.sockets.iter().find(|s| s.id == id)
    }
}
