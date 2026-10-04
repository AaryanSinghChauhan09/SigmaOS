// SigmaOS Reincarnation Server (Minix 3-inspired)
// Self-healing driver supervisor: monitors driver processes,
// detects crashes, restarts them transparently without system panic.
// Also includes Redox-inspired URL scheme dispatch architecture.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicU32, AtomicU64, Ordering}};
use std::time::Duration;

// ─────────────────────────────────────────────────────────────────────────────
// Driver Process Descriptor
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverState {
    Starting,
    Running,
    Unresponsive,
    Crashed,
    Restarting,
    Stopped,
    PermanentlyFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverKind {
    FileSystem,
    Network,
    Block,
    Char,
    Input,
    Audio,
    Usb,
    Pci,
    Ipc,
    Clock,
    Process,
}

#[derive(Debug, Clone)]
pub struct DriverSpec {
    pub name: String,
    pub kind: DriverKind,
    pub binary_path: String,
    pub max_restarts: u32,
    pub restart_delay_ms: u64,
    pub critical: bool, // if true, system cannot run without it
    pub dependencies: Vec<String>,
}

impl DriverSpec {
    pub fn new(name: &str, kind: DriverKind, path: &str) -> Self {
        DriverSpec {
            name: name.into(),
            kind,
            binary_path: path.into(),
            max_restarts: 5,
            restart_delay_ms: 100,
            critical: false,
            dependencies: Vec::new(),
        }
    }

    pub fn critical(mut self) -> Self { self.critical = true; self }
    pub fn max_restarts(mut self, n: u32) -> Self { self.max_restarts = n; self }
    pub fn depends_on(mut self, dep: &str) -> Self { self.dependencies.push(dep.into()); self }
}

#[derive(Debug)]
pub struct DriverProcess {
    pub spec: DriverSpec,
    pub pid: Option<u32>,
    pub state: DriverState,
    pub restart_count: u32,
    pub last_heartbeat_tick: u64,
    pub uptime_ticks: u64,
}

impl DriverProcess {
    pub fn new(spec: DriverSpec) -> Self {
        DriverProcess {
            spec,
            pid: None,
            state: DriverState::Stopped,
            restart_count: 0,
            last_heartbeat_tick: 0,
            uptime_ticks: 0,
        }
    }

    pub fn is_alive(&self) -> bool {
        matches!(self.state, DriverState::Running)
    }

    pub fn can_restart(&self) -> bool {
        self.restart_count < self.spec.max_restarts
            && !matches!(self.state, DriverState::PermanentlyFailed)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Reincarnation Server
// ─────────────────────────────────────────────────────────────────────────────

static NEXT_PID: AtomicU32 = AtomicU32::new(100);

pub struct ReincarnationServer {
    drivers: Arc<Mutex<HashMap<String, DriverProcess>>>,
    tick: AtomicU64,
    heartbeat_timeout_ticks: u64,
    pub log: Arc<Mutex<Vec<RSEvent>>>,
}

#[derive(Debug, Clone)]
pub struct RSEvent {
    pub tick: u64,
    pub driver: String,
    pub event: RSEventKind,
}

#[derive(Debug, Clone)]
pub enum RSEventKind {
    Started { pid: u32 },
    Crashed { pid: u32 },
    Restarted { new_pid: u32, attempt: u32 },
    Unresponsive,
    PermanentlyFailed,
    DependencyFailed { dep: String },
}

impl ReincarnationServer {
    pub fn new(heartbeat_timeout_ticks: u64) -> Self {
        ReincarnationServer {
            drivers: Arc::new(Mutex::new(HashMap::new())),
            tick: AtomicU64::new(0),
            heartbeat_timeout_ticks,
            log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Register a driver to be managed by RS
    pub fn register(&self, spec: DriverSpec) {
        let name = spec.name.clone();
        self.drivers.lock().unwrap().insert(name, DriverProcess::new(spec));
    }

    /// Start a driver (simulate exec())
    pub fn start(&self, name: &str) -> Result<u32, String> {
        // Check dependencies first (separate lock scope)
        {
            let drivers = self.drivers.lock().unwrap();
            let proc = drivers.get(name).ok_or_else(|| format!("Unknown driver: {}", name))?;
            for dep in &proc.spec.dependencies {
                match drivers.get(dep) {
                    Some(dep_proc) if !dep_proc.is_alive() => {
                        return Err(format!("Dependency {} not running", dep));
                    }
                    None => return Err(format!("Dependency {} not registered", dep)),
                    _ => {}
                }
            }
        }

        // Now mutably start the driver
        let pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);
        {
            let mut drivers = self.drivers.lock().unwrap();
            let proc = drivers.get_mut(name).ok_or_else(|| format!("Unknown driver: {}", name))?;
            proc.pid = Some(pid);
            proc.state = DriverState::Running;
            proc.last_heartbeat_tick = self.tick.load(Ordering::Relaxed);
        }

        self.log.lock().unwrap().push(RSEvent {
            tick: self.tick.load(Ordering::Relaxed),
            driver: name.into(),
            event: RSEventKind::Started { pid },
        });
        Ok(pid)
    }

    /// Simulate a driver crash (called by kernel on signal/segfault)
    pub fn notify_crash(&self, name: &str, dead_pid: u32) {
        let tick = self.tick.load(Ordering::Relaxed);
        self.log.lock().unwrap().push(RSEvent {
            tick,
            driver: name.into(),
            event: RSEventKind::Crashed { pid: dead_pid },
        });
        let mut drivers = self.drivers.lock().unwrap();
        if let Some(proc) = drivers.get_mut(name) {
            proc.state = DriverState::Crashed;
            proc.pid = None;
        }
    }

    /// Heartbeat from a running driver (must call periodically)
    pub fn heartbeat(&self, name: &str) {
        let tick = self.tick.load(Ordering::Relaxed);
        if let Some(proc) = self.drivers.lock().unwrap().get_mut(name) {
            proc.last_heartbeat_tick = tick;
        }
    }

    /// Main RS tick — called by the kernel watchdog timer
    /// Returns list of drivers that were restarted
    pub fn tick(&self) -> Vec<String> {
        let current_tick = self.tick.fetch_add(1, Ordering::SeqCst) + 1;
        let mut restarted = Vec::new();
        let names: Vec<String> = self.drivers.lock().unwrap().keys().cloned().collect();

        for name in names {
            let (should_restart, is_unresponsive) = {
                let drivers = self.drivers.lock().unwrap();
                let proc = match drivers.get(&name) { Some(p) => p, None => continue };
                let unresponsive = proc.state == DriverState::Running
                    && current_tick - proc.last_heartbeat_tick > self.heartbeat_timeout_ticks;
                let crashed = proc.state == DriverState::Crashed;
                (crashed || unresponsive, unresponsive)
            };

            if is_unresponsive {
                let tick = self.tick.load(Ordering::Relaxed);
                self.log.lock().unwrap().push(RSEvent { tick, driver: name.clone(), event: RSEventKind::Unresponsive });
                self.drivers.lock().unwrap().get_mut(&name).map(|p| p.state = DriverState::Crashed);
            }

            if should_restart {
                let can_restart = {
                    let drivers = self.drivers.lock().unwrap();
                    drivers.get(&name).map(|p| p.can_restart()).unwrap_or(false)
                };

                if can_restart {
                    // Simulate restart
                    let new_pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);
                    let attempt = {
                        let mut drivers = self.drivers.lock().unwrap();
                        let proc = drivers.get_mut(&name).unwrap();
                        proc.restart_count += 1;
                        proc.pid = Some(new_pid);
                        proc.state = DriverState::Running;
                        proc.last_heartbeat_tick = current_tick;
                        proc.restart_count
                    };
                    let tick = self.tick.load(Ordering::Relaxed);
                    self.log.lock().unwrap().push(RSEvent {
                        tick,
                        driver: name.clone(),
                        event: RSEventKind::Restarted { new_pid, attempt },
                    });
                    restarted.push(name.clone());
                } else {
                    // Permanently failed
                    let tick = self.tick.load(Ordering::Relaxed);
                    self.log.lock().unwrap().push(RSEvent {
                        tick,
                        driver: name.clone(),
                        event: RSEventKind::PermanentlyFailed,
                    });
                    self.drivers.lock().unwrap().get_mut(&name)
                        .map(|p| p.state = DriverState::PermanentlyFailed);
                }
            }
        }
        restarted
    }

    pub fn status(&self, name: &str) -> Option<DriverState> {
        self.drivers.lock().unwrap().get(name).map(|p| p.state.clone())
    }

    pub fn all_status(&self) -> Vec<(String, DriverState, u32)> {
        self.drivers.lock().unwrap().iter()
            .map(|(name, proc)| (name.clone(), proc.state.clone(), proc.restart_count))
            .collect()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Redox-inspired URL Scheme Dispatch
// ─────────────────────────────────────────────────────────────────────────────

/// A Redox-style URL scheme handler (e.g. `file:`, `tcp:`, `display:`, `time:`)
pub trait SchemeHandler: Send + Sync {
    fn open(&self, path: &str, flags: u32) -> Result<u64, SchemeError>;
    fn read(&self, fd: u64, buf_size: usize) -> Result<Vec<u8>, SchemeError>;
    fn write(&self, fd: u64, data: &[u8]) -> Result<usize, SchemeError>;
    fn close(&self, fd: u64) -> Result<(), SchemeError>;
    fn stat(&self, path: &str) -> Result<SchemeStat, SchemeError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemeError {
    NotFound,
    PermissionDenied,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    BrokenPipe,
    Utf8Error,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct SchemeStat {
    pub size: u64,
    pub mode: u32,
    pub mtime: u64,
}

pub struct SchemeNamespace {
    handlers: HashMap<String, Arc<dyn SchemeHandler>>,
    next_fd: AtomicU64,
}

impl SchemeNamespace {
    pub fn new() -> Self {
        SchemeNamespace { handlers: HashMap::new(), next_fd: AtomicU64::new(1) }
    }

    /// Register a scheme (e.g. `file`, `tcp`, `display`)
    pub fn register(&mut self, scheme: &str, handler: Arc<dyn SchemeHandler>) {
        self.handlers.insert(scheme.into(), handler);
    }

    /// Parse scheme from URL like `tcp:127.0.0.1:80`
    fn parse_scheme(url: &str) -> Option<(&str, &str)> {
        url.find(':').map(|i| (&url[..i], &url[i+1..]))
    }

    /// Open a URL — dispatches to the correct scheme handler
    pub fn open(&self, url: &str, flags: u32) -> Result<u64, SchemeError> {
        let (scheme, path) = Self::parse_scheme(url).ok_or(SchemeError::InvalidInput)?;
        let handler = self.handlers.get(scheme).ok_or(SchemeError::NotFound)?;
        let inner_fd = handler.open(path, flags)?;
        // Return a namespace-level FD
        Ok(inner_fd)
    }

    pub fn read(&self, url: &str, fd: u64, size: usize) -> Result<Vec<u8>, SchemeError> {
        let (scheme, _) = Self::parse_scheme(url).ok_or(SchemeError::InvalidInput)?;
        let handler = self.handlers.get(scheme).ok_or(SchemeError::NotFound)?;
        handler.read(fd, size)
    }

    pub fn write(&self, url: &str, fd: u64, data: &[u8]) -> Result<usize, SchemeError> {
        let (scheme, _) = Self::parse_scheme(url).ok_or(SchemeError::InvalidInput)?;
        let handler = self.handlers.get(scheme).ok_or(SchemeError::NotFound)?;
        handler.write(fd, data)
    }

    pub fn registered_schemes(&self) -> Vec<&String> { self.handlers.keys().collect() }
}

// Built-in scheme handlers

pub struct FileScheme {
    files: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    open_fds: Arc<Mutex<HashMap<u64, String>>>,
    next_fd: AtomicU64,
}

impl FileScheme {
    pub fn new() -> Self {
        FileScheme {
            files: Arc::new(Mutex::new(HashMap::new())),
            open_fds: Arc::new(Mutex::new(HashMap::new())),
            next_fd: AtomicU64::new(1),
        }
    }

    pub fn create_file(&self, path: &str, content: Vec<u8>) {
        self.files.lock().unwrap().insert(path.into(), content);
    }
}

impl SchemeHandler for FileScheme {
    fn open(&self, path: &str, _flags: u32) -> Result<u64, SchemeError> {
        if self.files.lock().unwrap().contains_key(path) {
            let fd = self.next_fd.fetch_add(1, Ordering::SeqCst);
            self.open_fds.lock().unwrap().insert(fd, path.into());
            Ok(fd)
        } else {
            Err(SchemeError::NotFound)
        }
    }

    fn read(&self, fd: u64, size: usize) -> Result<Vec<u8>, SchemeError> {
        let fds = self.open_fds.lock().unwrap();
        let path = fds.get(&fd).ok_or(SchemeError::NotFound)?;
        let files = self.files.lock().unwrap();
        match files.get(path.as_str()) {
            Some(data) => Ok(data[..data.len().min(size)].to_vec()),
            None => Err(SchemeError::NotFound),
        }
    }

    fn write(&self, fd: u64, data: &[u8]) -> Result<usize, SchemeError> {
        let fds = self.open_fds.lock().unwrap();
        let path = fds.get(&fd).ok_or(SchemeError::NotFound)?;
        let mut files = self.files.lock().unwrap();
        files.insert(path.clone(), data.to_vec());
        Ok(data.len())
    }

    fn close(&self, fd: u64) -> Result<(), SchemeError> {
        self.open_fds.lock().unwrap().remove(&fd);
        Ok(())
    }

    fn stat(&self, path: &str) -> Result<SchemeStat, SchemeError> {
        let files = self.files.lock().unwrap();
        match files.get(path) {
            Some(data) => Ok(SchemeStat { size: data.len() as u64, mode: 0o644, mtime: 0 }),
            None => Err(SchemeError::NotFound),
        }
    }
}

pub struct TcpScheme;
impl SchemeHandler for TcpScheme {
    fn open(&self, addr: &str, _: u32) -> Result<u64, SchemeError> {
        // Validate addr format: host:port
        if addr.contains(':') { Ok(42) } else { Err(SchemeError::InvalidInput) }
    }
    fn read(&self, _: u64, size: usize) -> Result<Vec<u8>, SchemeError> { Ok(vec![0u8; size.min(4096)]) }
    fn write(&self, _: u64, data: &[u8]) -> Result<usize, SchemeError> { Ok(data.len()) }
    fn close(&self, _: u64) -> Result<(), SchemeError> { Ok(()) }
    fn stat(&self, _: &str) -> Result<SchemeStat, SchemeError> { Ok(SchemeStat { size: 0, mode: 0, mtime: 0 }) }
}

pub struct TimeScheme;
impl SchemeHandler for TimeScheme {
    fn open(&self, _: &str, _: u32) -> Result<u64, SchemeError> { Ok(1) }
    fn read(&self, _: u64, _: usize) -> Result<Vec<u8>, SchemeError> {
        Ok(b"1727771400000000000".to_vec()) // nanoseconds
    }
    fn write(&self, _: u64, _: &[u8]) -> Result<usize, SchemeError> { Err(SchemeError::PermissionDenied) }
    fn close(&self, _: u64) -> Result<(), SchemeError> { Ok(()) }
    fn stat(&self, _: &str) -> Result<SchemeStat, SchemeError> { Ok(SchemeStat { size: 8, mode: 0o444, mtime: 0 }) }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    fn make_rs() -> ReincarnationServer {
        let rs = ReincarnationServer::new(5); // 5 ticks without heartbeat = unresponsive
        rs.register(DriverSpec::new("ext2fs", DriverKind::FileSystem, "/sbin/ext2fs").critical().max_restarts(3));
        rs.register(DriverSpec::new("inet", DriverKind::Network, "/sbin/inet").max_restarts(5));
        rs.register(DriverSpec::new("ahci", DriverKind::Block, "/sbin/ahci").max_restarts(3));
        rs
    }

    #[test]
    fn test_rs_start_driver() {
        let rs = make_rs();
        let pid = rs.start("ext2fs").unwrap();
        assert!(pid >= 100);
        assert_eq!(rs.status("ext2fs"), Some(DriverState::Running));
    }

    #[test]
    fn test_rs_crash_and_restart() {
        let rs = make_rs();
        let pid = rs.start("inet").unwrap();
        rs.notify_crash("inet", pid);
        assert_eq!(rs.status("inet"), Some(DriverState::Crashed));

        // Tick should restart it
        let restarted = rs.tick();
        assert!(restarted.contains(&"inet".to_string()));
        assert_eq!(rs.status("inet"), Some(DriverState::Running));
    }

    #[test]
    fn test_rs_max_restarts_permanent_failure() {
        let rs = make_rs();
        rs.start("ext2fs").unwrap();

        // Crash 3 times (max_restarts = 3)
        for i in 0..4 {
            rs.notify_crash("ext2fs", 100 + i);
            rs.tick();
        }
        // After 4 crashes, should be permanently failed
        let state = rs.status("ext2fs");
        assert!(matches!(state, Some(DriverState::PermanentlyFailed) | Some(DriverState::Crashed)));
    }

    #[test]
    fn test_rs_heartbeat_prevents_restart() {
        let rs = make_rs();
        rs.start("ahci").unwrap();
        // Send heartbeats every tick
        for _ in 0..10 {
            rs.heartbeat("ahci");
            rs.tick();
        }
        assert_eq!(rs.status("ahci"), Some(DriverState::Running));
    }

    #[test]
    fn test_redox_file_scheme() {
        let fs = Arc::new(FileScheme::new());
        fs.create_file("etc/hostname", b"sigmaos".to_vec());

        let mut ns = SchemeNamespace::new();
        ns.register("file", fs);

        let fd = ns.open("file:etc/hostname", 0).unwrap();
        let data = ns.read("file:etc/hostname", fd, 64).unwrap();
        assert_eq!(data, b"sigmaos");
    }

    #[test]
    fn test_redox_tcp_scheme() {
        let mut ns = SchemeNamespace::new();
        ns.register("tcp", Arc::new(TcpScheme));

        let fd = ns.open("tcp:127.0.0.1:8080", 0).unwrap();
        let written = ns.write("tcp:127.0.0.1:8080", fd, b"GET / HTTP/1.1\r\n").unwrap();
        assert_eq!(written, 16);
    }

    #[test]
    fn test_redox_time_scheme() {
        let mut ns = SchemeNamespace::new();
        ns.register("time", Arc::new(TimeScheme));
        let fd = ns.open("time:0", 0).unwrap();
        let ts = ns.read("time:0", fd, 32).unwrap();
        assert!(!ts.is_empty());
    }

    #[test]
    fn test_scheme_not_found() {
        let ns = SchemeNamespace::new();
        assert_eq!(ns.open("unknown:path", 0), Err(SchemeError::NotFound));
    }

    #[test]
    fn test_rs_all_status() {
        let rs = make_rs();
        rs.start("ext2fs").unwrap();
        rs.start("inet").unwrap();
        let status = rs.all_status();
        assert_eq!(status.len(), 3);
    }
}
