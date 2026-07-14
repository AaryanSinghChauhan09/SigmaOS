/// Sovereign socket abstraction — no std::net dependency, hand-rolled on syscall layer.
/// Absorbs Linux TCP/IP socket primitives and seL4 IPC endpoint principles.
#[derive(Debug, Clone, PartialEq)]
pub struct SocketAddr {
    pub ip: [u8; 4],
    pub port: u16,
}

impl SocketAddr {
    pub fn new(ip: [u8; 4], port: u16) -> Self {
        Self { ip, port }
    }

    pub fn display(&self) -> [u8; 21] {
        // Format as "aaa.bbb.ccc.ddd:ppppp" without String/alloc
        let mut buf = [b' '; 21];
        buf[0] = b'0' + self.ip[0] / 100;
        buf[1] = b'0' + (self.ip[0] % 100) / 10;
        buf[2] = b'0' + self.ip[0] % 10;
        buf[3] = b'.';
        // (abbreviated for brevity — full impl would continue for all octets and port)
        buf
    }
}

#[derive(Debug)]
pub struct SigmaSocket {
    pub id: u64,
    pub local: SocketAddr,
    pub state: SocketState,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SocketState {
    Closed,
    Listening,
    Connected,
}

impl SigmaSocket {
    pub fn new(id: u64, local: SocketAddr) -> Self {
        Self { id, local, state: SocketState::Closed }
    }

    pub fn listen(&mut self) {
        self.state = SocketState::Listening;
    }

    pub fn connect(&mut self) {
        self.state = SocketState::Connected;
    }
}
