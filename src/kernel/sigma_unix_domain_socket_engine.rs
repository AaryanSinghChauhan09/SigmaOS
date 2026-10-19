/// SigmaOS Unix Domain Socket Engine
/// AF_UNIX streams, datagrams, abstract namespace, SCM_RIGHTS.
///
/// Standard library (std, libc) are fully authorized per SigmaOS policy.

use std::collections::HashMap;
use std::fmt;

/// Socket type
#[derive(Debug, Clone, PartialEq)]
pub enum UnixSocketType {
    Stream,
    Datagram,
    SeqPacket,
}

impl fmt::Display for UnixSocketType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stream => write!(f, "SOCK_STREAM"),
            Self::Datagram => write!(f, "SOCK_DGRAM"),
            Self::SeqPacket => write!(f, "SOCK_SEQPACKET"),
        }
    }
}

/// Socket address (filesystem or abstract)
#[derive(Debug, Clone, PartialEq)]
pub enum UnixSocketAddr {
    /// Filesystem path, e.g. "/run/sigma/control.sock"
    Path(String),
    /// Abstract namespace (Linux-specific), stored without null prefix
    Abstract(String),
    /// Unnamed (auto-bound)
    Unnamed,
}

impl fmt::Display for UnixSocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path(p) => write!(f, "{}", p),
            Self::Abstract(name) => write!(f, "@{}", name),
            Self::Unnamed => write!(f, "<unnamed>"),
        }
    }
}

/// Ancillary message type for sendmsg/recvmsg
#[derive(Debug, Clone)]
pub enum AncillaryData {
    /// SCM_RIGHTS: file descriptor passing
    Rights(Vec<i32>),
    /// SCM_CREDENTIALS: peer process credentials
    Credentials { pid: u32, uid: u32, gid: u32 },
}

/// Simulated Unix domain socket descriptor
#[derive(Debug, Clone)]
pub struct UnixSocket {
    pub fd: i32,
    pub sock_type: UnixSocketType,
    pub local_addr: UnixSocketAddr,
    pub peer_addr: Option<UnixSocketAddr>,
    pub rcvbuf_bytes: usize,
    pub sndbuf_bytes: usize,
    pub connected: bool,
    pub nonblocking: bool,
}

impl UnixSocket {
    pub fn new(fd: i32, sock_type: UnixSocketType) -> Self {
        Self {
            fd,
            sock_type,
            local_addr: UnixSocketAddr::Unnamed,
            peer_addr: None,
            rcvbuf_bytes: 212992,  // kernel default
            sndbuf_bytes: 212992,
            connected: false,
            nonblocking: false,
        }
    }

    pub fn bind(&mut self, addr: UnixSocketAddr) {
        self.local_addr = addr;
    }

    pub fn connect(&mut self, addr: UnixSocketAddr) -> Result<(), &'static str> {
        if self.sock_type == UnixSocketType::Stream || self.sock_type == UnixSocketType::SeqPacket {
            self.peer_addr = Some(addr);
            self.connected = true;
            Ok(())
        } else {
            // SOCK_DGRAM can "connect" to default destination
            self.peer_addr = Some(addr);
            Ok(())
        }
    }

    pub fn is_abstract(&self) -> bool {
        matches!(self.local_addr, UnixSocketAddr::Abstract(_))
    }
}

/// Unix socket registry
pub struct UnixSocketManager {
    sockets: HashMap<i32, UnixSocket>,
    next_fd: i32,
    pub path_registry: HashMap<String, i32>, // addr -> fd
}

impl UnixSocketManager {
    pub fn new() -> Self {
        Self {
            sockets: HashMap::new(),
            next_fd: 100,
            path_registry: HashMap::new(),
        }
    }

    pub fn create(&mut self, sock_type: UnixSocketType) -> i32 {
        let fd = self.next_fd;
        self.next_fd += 1;
        self.sockets.insert(fd, UnixSocket::new(fd, sock_type));
        fd
    }

    pub fn bind(&mut self, fd: i32, addr: UnixSocketAddr) -> Result<(), &'static str> {
        let addr_str = format!("{}", addr);
        if self.path_registry.contains_key(&addr_str) {
            return Err("address already in use");
        }
        let sock = self.sockets.get_mut(&fd).ok_or("bad file descriptor")?;
        sock.bind(addr.clone());
        self.path_registry.insert(addr_str, fd);
        Ok(())
    }

    pub fn connect(&mut self, fd: i32, addr: UnixSocketAddr) -> Result<(), &'static str> {
        let sock = self.sockets.get_mut(&fd).ok_or("bad file descriptor")?;
        sock.connect(addr)
    }

    pub fn pass_fds(&self, from_fd: i32, fds: Vec<i32>) -> AncillaryData {
        let _ = from_fd; // would validate in real impl
        AncillaryData::Rights(fds)
    }

    pub fn socket_count(&self) -> usize {
        self.sockets.len()
    }

    pub fn get(&self, fd: i32) -> Option<&UnixSocket> {
        self.sockets.get(&fd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_stream_socket() {
        let mut mgr = UnixSocketManager::new();
        let fd = mgr.create(UnixSocketType::Stream);
        assert!(fd >= 100);
        assert_eq!(mgr.socket_count(), 1);
    }

    #[test]
    fn test_bind_filesystem_path() {
        let mut mgr = UnixSocketManager::new();
        let fd = mgr.create(UnixSocketType::Stream);
        let result = mgr.bind(fd, UnixSocketAddr::Path("/run/sigma/test.sock".to_string()));
        assert!(result.is_ok());
        assert!(mgr.path_registry.contains_key("/run/sigma/test.sock"));
    }

    #[test]
    fn test_bind_abstract_namespace() {
        let mut mgr = UnixSocketManager::new();
        let fd = mgr.create(UnixSocketType::Datagram);
        let result = mgr.bind(fd, UnixSocketAddr::Abstract("sigma.test".to_string()));
        assert!(result.is_ok());
        assert!(mgr.get(fd).unwrap().is_abstract());
    }

    #[test]
    fn test_duplicate_bind_fails() {
        let mut mgr = UnixSocketManager::new();
        let fd1 = mgr.create(UnixSocketType::Stream);
        let fd2 = mgr.create(UnixSocketType::Stream);
        mgr.bind(fd1, UnixSocketAddr::Path("/run/dup.sock".to_string())).unwrap();
        let result = mgr.bind(fd2, UnixSocketAddr::Path("/run/dup.sock".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_fd_passing() {
        let mgr = UnixSocketManager::new();
        let ancillary = mgr.pass_fds(100, vec![3, 4, 5]);
        if let AncillaryData::Rights(fds) = ancillary {
            assert_eq!(fds, vec![3, 4, 5]);
        } else {
            panic!("expected Rights");
        }
    }
}
