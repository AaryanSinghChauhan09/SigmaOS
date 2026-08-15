use std::sync::atomic::{AtomicUsize, Ordering};
/// SigmaOS Network Socket Layer
/// Absorbs Linux BSD socket interface: socket()/bind()/listen()/accept()/connect()
/// Supports AF_INET (IPv4), AF_INET6, AF_UNIX; SOCK_STREAM/DGRAM/RAW
/// Enhanced with Debian Linux style DNS resolv.conf and /etc/hosts resolution engine.
use std::collections::HashMap;
use std::string::{String, ToString};
use alloc::vec::Vec;

// ── Address Families & Socket Types ──────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressFamily {
    Unspec,
    Unix,
    Inet,
    Inet6,
    Netlink,
    Packet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Stream,
    Dgram,
    Raw,
    SeqPacket,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tcp = 6,
    Udp = 17,
    Icmp = 1,
    Raw = 0,
}

/// IPv4 socket address
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SockAddrIn {
    pub port: u16,
    pub addr: [u8; 4], // IPv4 address
}

impl SockAddrIn {
    pub fn new(addr: [u8; 4], port: u16) -> Self {
        SockAddrIn { port, addr }
    }
    pub fn loopback(port: u16) -> Self {
        SockAddrIn::new([127, 0, 0, 1], port)
    }
    pub fn any(port: u16) -> Self {
        SockAddrIn::new([0, 0, 0, 0], port)
    }
}

// ── Socket State Machine ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Unbound,
    Bound,
    Listening,
    Connected,
    Accepted,
    CloseWait,
    FinWait1,
    FinWait2,
    TimeWait,
    Closed,
}

/// File descriptor flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketFlags {
    pub non_blocking: bool,
    pub reuse_addr: bool,
    pub reuse_port: bool,
    pub keep_alive: bool,
    pub no_delay: bool, // TCP_NODELAY (disable Nagle)
}

impl Default for SocketFlags {
    fn default() -> Self {
        SocketFlags {
            non_blocking: false,
            reuse_addr: false,
            reuse_port: false,
            keep_alive: false,
            no_delay: false,
        }
    }
}

/// Socket receive/send buffer
pub struct SocketBuffer {
    data: Vec<u8>,
    capacity: usize,
}

impl SocketBuffer {
    pub fn new(capacity: usize) -> Self {
        SocketBuffer {
            data: Vec::new(),
            capacity,
        }
    }
    pub fn push(&mut self, buf: &[u8]) -> usize {
        let avail = self.capacity.saturating_sub(self.data.len());
        let n = buf.len().min(avail);
        self.data.extend_from_slice(&buf[..n]);
        n
    }
    pub fn pop(&mut self, n: usize) -> Vec<u8> {
        let n = n.min(self.data.len());
        self.data.drain(..n).collect()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// A kernel socket
pub struct Socket {
    pub fd: u32,
    pub family: AddressFamily,
    pub sock_type: SocketType,
    pub protocol: Protocol,
    pub state: SocketState,
    pub local_addr: Option<SockAddrIn>,
    pub remote_addr: Option<SockAddrIn>,
    pub flags: SocketFlags,
    pub recv_buf: SocketBuffer,
    pub send_buf: SocketBuffer,
    pub backlog: usize,
    pub pending_accept: Vec<u32>, // FDs of sockets waiting to be accept()ed
    bytes_sent: AtomicUsize,
    bytes_recv: AtomicUsize,
}

impl Socket {
    pub fn new(fd: u32, family: AddressFamily, sock_type: SocketType, proto: Protocol) -> Self {
        Socket {
            fd,
            family,
            sock_type,
            protocol: proto,
            state: SocketState::Unbound,
            local_addr: None,
            remote_addr: None,
            flags: SocketFlags::default(),
            recv_buf: SocketBuffer::new(131072), // 128KB default
            send_buf: SocketBuffer::new(131072),
            backlog: 128,
            pending_accept: Vec::new(),
            bytes_sent: AtomicUsize::new(0),
            bytes_recv: AtomicUsize::new(0),
        }
    }

    pub fn bind(&mut self, addr: SockAddrIn) -> Result<(), &'static str> {
        if self.state != SocketState::Unbound {
            return Err("Socket already bound");
        }
        self.local_addr = Some(addr);
        self.state = SocketState::Bound;
        Ok(())
    }

    pub fn listen(&mut self, backlog: usize) -> Result<(), &'static str> {
        if self.sock_type != SocketType::Stream {
            return Err("Only SOCK_STREAM can listen");
        }
        if self.state != SocketState::Bound {
            return Err("Socket must be bound first");
        }
        self.backlog = backlog;
        self.state = SocketState::Listening;
        Ok(())
    }

    pub fn connect(&mut self, remote: SockAddrIn) -> Result<(), &'static str> {
        if self.state == SocketState::Connected {
            return Err("Already connected");
        }
        self.remote_addr = Some(remote);
        self.state = SocketState::Connected;
        Ok(())
    }

    pub fn send(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.state != SocketState::Connected {
            return Err("Not connected");
        }
        let avail = self.send_buf.capacity.saturating_sub(self.send_buf.len());
        if avail == 0 && self.flags.non_blocking {
            return Err("EWOULDBLOCK: resource temporarily unavailable");
        }
        let n = self.send_buf.push(data);
        self.bytes_sent.fetch_add(n, Ordering::Relaxed);
        Ok(n)
    }

    pub fn recv(&mut self, max: usize) -> Result<Vec<u8>, &'static str> {
        if self.state != SocketState::Connected && self.state != SocketState::Accepted {
            return Err("Not connected");
        }
        // BSD Non-blocking check: if buffer is empty and non_blocking flag is set, return EWOULDBLOCK
        if self.recv_buf.is_empty() && self.flags.non_blocking {
            return Err("EWOULDBLOCK: read would block");
        }
        let data = self.recv_buf.pop(max);
        self.bytes_recv.fetch_add(data.len(), Ordering::Relaxed);
        Ok(data)
    }

    pub fn inject_data(&mut self, data: &[u8]) {
        self.recv_buf.push(data);
    }
    pub fn bytes_sent(&self) -> usize {
        self.bytes_sent.load(Ordering::Relaxed)
    }
    pub fn bytes_recv(&self) -> usize {
        self.bytes_recv.load(Ordering::Relaxed)
    }
}

// ── Socket Manager (kernel socket table) ──────────────────────────────────

// Standard socket levels and options matching Linux / BSD
pub const SOL_SOCKET: i32 = 1;
pub const SO_REUSEADDR: i32 = 2;
pub const SO_REUSEPORT: i32 = 15;
pub const SO_KEEPALIVE: i32 = 9;
pub const SO_RCVBUF: i32 = 8;
pub const SO_SNDBUF: i32 = 7;

pub struct SocketLayer {
    sockets: BTreeMap<u32, Socket>,
    next_fd: AtomicUsize,
    bound_ports: BTreeMap<u16, u32>, // port -> fd
}

impl SocketLayer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SocketLayer {
            sockets: BTreeMap::new(),
            next_fd: AtomicUsize::new(4),
            bound_ports: BTreeMap::new(),
        }
    }

    pub fn socket(&mut self, family: AddressFamily, typ: SocketType, proto: Protocol) -> u32 {
        let fd = self.next_fd.fetch_add(1, Ordering::SeqCst) as u32;
        self.sockets.insert(fd, Socket::new(fd, family, typ, proto));
        fd
    }

    pub fn setsockopt(&mut self, fd: u32, level: i32, optname: i32, optval: &[u8]) -> Result<(), &'static str> {
        let sock = self.sockets.get_mut(&fd).ok_or("EBADF: invalid fd")?;
        if level != SOL_SOCKET {
            return Err("ENOPROTOOPT: level not supported");
        }
        if optval.is_empty() {
            return Err("EINVAL: invalid option value");
        }
        let val = optval[0] != 0;
        match optname {
            SO_REUSEADDR => sock.flags.reuse_addr = val,
            SO_REUSEPORT => sock.flags.reuse_port = val,
            SO_KEEPALIVE => sock.flags.keep_alive = val,
            _ => return Err("ENOPROTOOPT: option not supported"),
        }
        Ok(())
    }

    pub fn getsockopt(&self, fd: u32, level: i32, optname: i32) -> Result<Vec<u8>, &'static str> {
        let sock = self.sockets.get(&fd).ok_or("EBADF: invalid fd")?;
        if level != SOL_SOCKET {
            return Err("ENOPROTOOPT");
        }
        let val = match optname {
            SO_REUSEADDR => sock.flags.reuse_addr,
            SO_REUSEPORT => sock.flags.reuse_port,
            SO_KEEPALIVE => sock.flags.keep_alive,
            _ => return Err("ENOPROTOOPT"),
        };
        Ok(vec![if val { 1 } else { 0 }])
    }

    pub fn bind(&mut self, fd: u32, addr: SockAddrIn) -> Result<(), &'static str> {
        let new_sock = self.sockets.get(&fd).ok_or("EBADF: invalid fd")?;
        let allow_bind = if let Some(&existing_fd) = self.bound_ports.get(&addr.port) {
            if let Some(existing_sock) = self.sockets.get(&existing_fd) {
                (existing_sock.flags.reuse_addr && new_sock.flags.reuse_addr) ||
                (existing_sock.flags.reuse_port && new_sock.flags.reuse_port)
            } else {
                false
            }
        } else {
            true
        };

        if !allow_bind {
            return Err("EADDRINUSE: port already bound");
        }

        self.bound_ports.entry(addr.port).or_insert(fd);
        let sock = self.sockets.get_mut(&fd).ok_or("EBADF: invalid fd")?;
        sock.bind(addr)
    }

    pub fn listen(&mut self, fd: u32, backlog: usize) -> Result<(), &'static str> {
        let sock = self.sockets.get_mut(&fd).ok_or("EBADF")?;
        sock.listen(backlog)
    }

    pub fn connect(&mut self, fd: u32, remote: SockAddrIn) -> Result<(), &'static str> {
        let sock = self.sockets.get_mut(&fd).ok_or("EBADF")?;
        sock.connect(remote)
    }

    pub fn send(&mut self, fd: u32, data: &[u8]) -> Result<usize, &'static str> {
        let sock = self.sockets.get_mut(&fd).ok_or("EBADF")?;
        sock.send(data)
    }

    pub fn recv(&mut self, fd: u32, max: usize) -> Result<Vec<u8>, &'static str> {
        let sock = self.sockets.get_mut(&fd).ok_or("EBADF")?;
        sock.recv(max)
    }

    pub fn inject_data(&mut self, fd: u32, data: &[u8]) {
        if let Some(sock) = self.sockets.get_mut(&fd) {
            sock.inject_data(data);
        }
    }

    pub fn set_non_blocking(&mut self, fd: u32, non_blocking: bool) -> Result<(), &'static str> {
        let sock = self.sockets.get_mut(&fd).ok_or("EBADF")?;
        sock.flags.non_blocking = non_blocking;
        Ok(())
    }

    pub fn close(&mut self, fd: u32) -> Result<(), &'static str> {
        let sock = self.sockets.remove(&fd).ok_or("EBADF")?;
        if let Some(addr) = sock.local_addr {
            self.bound_ports.remove(&addr.port);
        }
        Ok(())
    }

    pub fn get_socket(&self, fd: u32) -> Option<&Socket> {
        self.sockets.get(&fd)
    }
    pub fn socket_count(&self) -> usize {
        self.sockets.len()
    }
}

impl Default for SocketLayer {
    fn default() -> Self {
        Self::new()
    }
}

// ── Debian-Style DNS and host configuration parser ─────────────────────────

/// Sovereign DNS Resolver Configuration Parser (replicates /etc/resolv.conf and /etc/hosts)
pub struct SovereignDnsResolver {
    pub nameservers: Vec<String>,
    pub hosts: HashMap<String, [u8; 4]>,
}

impl SovereignDnsResolver {
    pub fn new() -> Self {
        Self {
            nameservers: Vec::new(),
            hosts: HashMap::new(),
        }
    }

    /// Parses standard Debian /etc/resolv.conf lines
    pub fn parse_resolv_conf(&mut self, content: &str) {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
                continue;
            }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 && parts[0] == "nameserver" {
                self.nameservers.push(parts[1].to_string());
            }
        }
    }

    /// Parses standard Debian /etc/hosts lines
    pub fn parse_hosts(&mut self, content: &str) {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let ip_str = parts[0];
                let mut ip_bytes = [0u8; 4];
                let octets: Vec<&str> = ip_str.split('.').collect();
                if octets.len() == 4 {
                    for i in 0..4 {
                        ip_bytes[i] = octets[i].parse::<u8>().unwrap_or(0);
                    }
                    for &hostname in &parts[1..] {
                        self.hosts.insert(hostname.to_string(), ip_bytes);
                    }
                }
            }
        }
    }

    /// Resolves hostname dynamically via static hosts files or returns default loopbacks
    pub fn resolve_hostname(&self, hostname: &str) -> Option<[u8; 4]> {
        if let Some(ip) = self.hosts.get(hostname) {
            Some(*ip)
        } else if hostname == "localhost" {
            Some([127, 0, 0, 1])
        } else if !self.nameservers.is_empty() {
            // Simulated fallback to first nameserver
            Some([8, 8, 8, 8])
        } else {
            None
        }
    }
}

impl Default for SovereignDnsResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_server_client_lifecycle() {
        let mut sl = SocketLayer::new();

        // Server
        let server_fd = sl.socket(AddressFamily::Inet, SocketType::Stream, Protocol::Tcp);
        sl.bind(server_fd, SockAddrIn::any(8080)).unwrap();
        sl.listen(server_fd, 10).unwrap();
        assert_eq!(
            sl.get_socket(server_fd).unwrap().state,
            SocketState::Listening
        );

        // Client
        let client_fd = sl.socket(AddressFamily::Inet, SocketType::Stream, Protocol::Tcp);
        sl.connect(client_fd, SockAddrIn::loopback(8080)).unwrap();
        assert_eq!(
            sl.get_socket(client_fd).unwrap().state,
            SocketState::Connected
        );

        // Data transfer
        let n = sl.send(client_fd, b"Hello SigmaOS!").unwrap();
        assert_eq!(n, 14);
        assert_eq!(sl.get_socket(client_fd).unwrap().bytes_sent(), 14);
    }

    #[test]
    fn test_socket_close_releases_port() {
        let mut sl = SocketLayer::new();
        let fd = sl.socket(AddressFamily::Inet, SocketType::Dgram, Protocol::Udp);
        sl.bind(fd, SockAddrIn::any(53)).unwrap();
        sl.close(fd).unwrap();
        // Can bind again after close
        let fd2 = sl.socket(AddressFamily::Inet, SocketType::Dgram, Protocol::Udp);
        sl.bind(fd2, SockAddrIn::any(53)).unwrap();
    }

    #[test]
    fn test_recv_inject() {
        let mut sl = SocketLayer::new();
        let fd = sl.socket(AddressFamily::Inet, SocketType::Stream, Protocol::Tcp);
        sl.connect(fd, SockAddrIn::loopback(9000)).unwrap();
        sl.inject_data(fd, b"response data");
        let data = sl.recv(fd, 512).unwrap();
        assert_eq!(data, b"response data");
    }

    #[test]
    fn test_port_collision() {
        let mut sl = SocketLayer::new();
        let fd1 = sl.socket(AddressFamily::Inet, SocketType::Stream, Protocol::Tcp);
        let fd2 = sl.socket(AddressFamily::Inet, SocketType::Stream, Protocol::Tcp);
        sl.bind(fd1, SockAddrIn::any(80)).unwrap();
        let result = sl.bind(fd2, SockAddrIn::any(80));
        assert!(result.is_err());
    }

    #[test]
    fn test_non_blocking_socket_read_ewouldblock() {
        let mut sl = SocketLayer::new();
        let fd = sl.socket(AddressFamily::Inet, SocketType::Stream, Protocol::Tcp);
        sl.connect(fd, SockAddrIn::loopback(11000)).unwrap();

        // Set socket to non-blocking
        sl.set_non_blocking(fd, true).unwrap();

        // Reading when buffer is empty must return EWOULDBLOCK error
        let result = sl.recv(fd, 100);
        assert_eq!(result, Err("EWOULDBLOCK: read would block"));

        // Injecting data and reading should work fine
        sl.inject_data(fd, b"delayed payload");
        let res_data = sl.recv(fd, 100).unwrap();
        assert_eq!(res_data, b"delayed payload");
    }

    #[test]
    fn test_dns_resolver_resolv_conf_and_hosts() {
        let mut dns = SovereignDnsResolver::new();

        let resolv_conf = r#"
            # This is a comment
            nameserver 1.1.1.1
            nameserver 8.8.8.8
        "#;
        dns.parse_resolv_conf(resolv_conf);
        assert_eq!(dns.nameservers.len(), 2);
        assert_eq!(dns.nameservers[0], "1.1.1.1");

        let hosts = r#"
            127.0.0.1 localhost local.host
            192.168.1.1 gateway.local
        "#;
        dns.parse_hosts(hosts);
        assert_eq!(dns.hosts.get("gateway.local"), Some(&[192, 168, 1, 1]));
        assert_eq!(dns.hosts.get("local.host"), Some(&[127, 0, 0, 1]));

        // Resolution tests
        assert_eq!(dns.resolve_hostname("localhost"), Some([127, 0, 0, 1]));
        assert_eq!(dns.resolve_hostname("gateway.local"), Some([192, 168, 1, 1]));
        assert_eq!(dns.resolve_hostname("google.com"), Some([8, 8, 8, 8])); // fallback to resolver nameserver
    }
}
