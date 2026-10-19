// SigmaOS Rump Kernel — NetBSD-Inspired Userland Driver Framework
// Allows running device drivers as isolated userland processes via a
// lightweight "rump" kernel providing the minimum kernel ABI.
// Also includes BFS (BeOS File System) live attribute queries.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicU32, AtomicU64, Ordering}};

// ─────────────────────────────────────────────────────────────────────────────
// Rump Kernel ABI (minimal kernel interface exposed to userland drivers)
// ─────────────────────────────────────────────────────────────────────────────

pub trait RumpKernelAbi: Send + Sync {
    fn kmem_alloc(&self, size: usize) -> Option<u64>; // returns virtual address
    fn kmem_free(&self, addr: u64, size: usize);
    fn kprintf(&self, msg: &str);
    fn mutex_init(&self) -> u32;  // returns mutex handle
    fn mutex_lock(&self, handle: u32) -> bool;
    fn mutex_unlock(&self, handle: u32);
    fn cv_signal(&self, handle: u32);
    fn cv_wait(&self, handle: u32);
    fn pci_read_config(&self, bus: u8, dev: u8, func: u8, offset: u8) -> u32;
    fn pci_write_config(&self, bus: u8, dev: u8, func: u8, offset: u8, val: u32);
    fn intr_establish(&self, irq: u32, handler: &str) -> bool;
}

/// Simulated Rump kernel ABI implementation
pub struct RumpKernelSim {
    heap: Arc<Mutex<HashMap<u64, usize>>>,
    next_addr: AtomicU64,
    next_mutex: AtomicU32,
    log: Arc<Mutex<Vec<String>>>,
}

impl RumpKernelSim {
    pub fn new() -> Self {
        RumpKernelSim {
            heap: Arc::new(Mutex::new(HashMap::new())),
            next_addr: AtomicU64::new(0x1000_0000),
            next_mutex: AtomicU32::new(1),
            log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn alloc_log(&self) -> Vec<String> { self.log.lock().unwrap().clone() }
}

impl RumpKernelAbi for RumpKernelSim {
    fn kmem_alloc(&self, size: usize) -> Option<u64> {
        let addr = self.next_addr.fetch_add((size + 0xF & !0xF) as u64, Ordering::SeqCst);
        self.heap.lock().unwrap().insert(addr, size);
        Some(addr)
    }

    fn kmem_free(&self, addr: u64, _size: usize) {
        self.heap.lock().unwrap().remove(&addr);
    }

    fn kprintf(&self, msg: &str) {
        self.log.lock().unwrap().push(msg.into());
    }

    fn mutex_init(&self) -> u32 { self.next_mutex.fetch_add(1, Ordering::SeqCst) }
    fn mutex_lock(&self, _: u32) -> bool { true }
    fn mutex_unlock(&self, _: u32) {}
    fn cv_signal(&self, _: u32) {}
    fn cv_wait(&self, _: u32) {}

    fn pci_read_config(&self, bus: u8, dev: u8, func: u8, offset: u8) -> u32 {
        // Simulated PCI config space read
        match (bus, dev, func, offset) {
            (0, 0, 0, 0) => 0x8086_1234, // Vendor/Device ID
            (0, 0, 0, 8) => 0x0200_0000, // Class code: Ethernet
            _ => 0xFFFF_FFFF,
        }
    }

    fn pci_write_config(&self, _: u8, _: u8, _: u8, _: u8, _: u32) {}

    fn intr_establish(&self, irq: u32, handler: &str) -> bool {
        self.log.lock().unwrap().push(format!("irq {} → {}", irq, handler));
        true
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Rump Driver Process
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RumpDriverState { Loaded, Init, Running, Faulted, Unloaded }

pub struct RumpDriverProcess {
    pub name: String,
    pub pid: u32,
    pub state: RumpDriverState,
    pub device_class: String,
    pub irq: Option<u32>,
    pub iobase: Option<u64>,
    pub mem_allocations: Vec<u64>,
    kernel: Arc<dyn RumpKernelAbi>,
}

impl RumpDriverProcess {
    pub fn new(name: &str, class: &str, kernel: Arc<dyn RumpKernelAbi>) -> Self {
        static PID: AtomicU32 = AtomicU32::new(200);
        RumpDriverProcess {
            name: name.into(),
            pid: PID.fetch_add(1, Ordering::SeqCst),
            state: RumpDriverState::Loaded,
            device_class: class.into(),
            irq: None,
            iobase: None,
            mem_allocations: Vec::new(),
            kernel,
        }
    }

    /// Initialize the driver (calls driver_attach equivalent)
    pub fn init(&mut self) -> Result<(), &'static str> {
        self.kernel.kprintf(&format!("rump: loading driver {}", self.name));

        // Probe PCI (simulated)
        let vendor_device = self.kernel.pci_read_config(0, 0, 0, 0);
        self.kernel.kprintf(&format!("rump: pci probe 0x{:08X}", vendor_device));

        // Allocate DMA-capable memory
        if let Some(addr) = self.kernel.kmem_alloc(65536) {
            self.mem_allocations.push(addr);
            self.kernel.kprintf(&format!("rump: dma buf @ 0x{:016X}", addr));
        }

        // Establish IRQ handler
        let irq = 11;
        if self.kernel.intr_establish(irq, &format!("{}_intr", self.name)) {
            self.irq = Some(irq);
        }

        self.state = RumpDriverState::Running;
        Ok(())
    }

    /// Detach the driver (frees all resources)
    pub fn detach(&mut self) {
        for addr in self.mem_allocations.drain(..) {
            self.kernel.kmem_free(addr, 65536);
        }
        self.state = RumpDriverState::Unloaded;
        self.kernel.kprintf(&format!("rump: driver {} detached", self.name));
    }

    /// Simulate a driver fault (triggers rump sandbox crash isolation)
    pub fn simulate_fault(&mut self) {
        self.state = RumpDriverState::Faulted;
        self.kernel.kprintf(&format!("rump: driver {} faulted (isolated)", self.name));
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Rump Host Bridge (manages multiple isolated drivers)
// ─────────────────────────────────────────────────────────────────────────────

pub struct RumpHostBridge {
    pub drivers: Vec<RumpDriverProcess>,
    kernel: Arc<dyn RumpKernelAbi>,
}

impl RumpHostBridge {
    pub fn new(kernel: Arc<dyn RumpKernelAbi>) -> Self {
        RumpHostBridge { drivers: Vec::new(), kernel }
    }

    pub fn load_driver(&mut self, name: &str, class: &str) -> usize {
        let mut proc = RumpDriverProcess::new(name, class, Arc::clone(&self.kernel));
        let _ = proc.init();
        self.drivers.push(proc);
        self.drivers.len() - 1
    }

    pub fn fault_driver(&mut self, idx: usize) {
        if let Some(drv) = self.drivers.get_mut(idx) {
            drv.simulate_fault();
        }
    }

    pub fn running_count(&self) -> usize {
        self.drivers.iter().filter(|d| d.state == RumpDriverState::Running).count()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BFS Attributed File System (BeOS-inspired)
// File attributes as key-value pairs + live query indexing
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum AttrValue {
    Int32(i32),
    Int64(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Raw(Vec<u8>),
}

impl AttrValue {
    pub fn as_str(&self) -> Option<&str> { if let AttrValue::Str(s) = self { Some(s) } else { None } }
    pub fn as_int64(&self) -> Option<i64> { if let AttrValue::Int64(n) = self { Some(*n) } else { None } }
    pub fn as_float(&self) -> Option<f64> { if let AttrValue::Float(f) = self { Some(*f) } else { None } }
}

#[derive(Debug, Clone)]
pub struct BfsInode {
    pub ino: u64,
    pub name: String,
    pub size: u64,
    pub mime_type: String,
    /// Named attributes (BeOS-style extended attributes)
    pub attrs: HashMap<String, AttrValue>,
}

impl BfsInode {
    pub fn new(ino: u64, name: &str) -> Self {
        BfsInode { ino, name: name.into(), size: 0, mime_type: "application/octet-stream".into(), attrs: HashMap::new() }
    }

    pub fn set_attr(&mut self, key: &str, val: AttrValue) { self.attrs.insert(key.into(), val); }
    pub fn get_attr(&self, key: &str) -> Option<&AttrValue> { self.attrs.get(key) }
    pub fn remove_attr(&mut self, key: &str) { self.attrs.remove(key); }
}

/// BFS query predicate
#[derive(Debug, Clone)]
pub enum QueryPredicate {
    Eq(String, AttrValue),
    Gt(String, AttrValue),
    Lt(String, AttrValue),
    Contains(String, String),   // attr_name, substring
    HasAttr(String),
    And(Box<QueryPredicate>, Box<QueryPredicate>),
    Or(Box<QueryPredicate>,  Box<QueryPredicate>),
    Not(Box<QueryPredicate>),
}

impl QueryPredicate {
    pub fn matches(&self, inode: &BfsInode) -> bool {
        match self {
            QueryPredicate::Eq(attr, val) => {
                inode.get_attr(attr).map(|v| format!("{:?}", v) == format!("{:?}", val)).unwrap_or(false)
            }
            QueryPredicate::Gt(attr, val) => {
                match (inode.get_attr(attr), val) {
                    (Some(AttrValue::Int64(a)), AttrValue::Int64(b)) => a > b,
                    (Some(AttrValue::Float(a)), AttrValue::Float(b)) => a > b,
                    _ => false,
                }
            }
            QueryPredicate::Lt(attr, val) => {
                match (inode.get_attr(attr), val) {
                    (Some(AttrValue::Int64(a)), AttrValue::Int64(b)) => a < b,
                    (Some(AttrValue::Float(a)), AttrValue::Float(b)) => a < b,
                    _ => false,
                }
            }
            QueryPredicate::Contains(attr, substr) => {
                inode.get_attr(attr).and_then(|v| v.as_str()).map(|s| s.contains(substr.as_str())).unwrap_or(false)
            }
            QueryPredicate::HasAttr(attr) => inode.attrs.contains_key(attr),
            QueryPredicate::And(a, b) => a.matches(inode) && b.matches(inode),
            QueryPredicate::Or(a, b)  => a.matches(inode) || b.matches(inode),
            QueryPredicate::Not(p)    => !p.matches(inode),
        }
    }
}

pub struct BfsVolume {
    pub name: String,
    pub inodes: HashMap<u64, BfsInode>,
    next_ino: AtomicU64,
}

impl BfsVolume {
    pub fn new(name: &str) -> Self {
        BfsVolume { name: name.into(), inodes: HashMap::new(), next_ino: AtomicU64::new(2) }
    }

    pub fn create_file(&mut self, name: &str) -> u64 {
        let ino = self.next_ino.fetch_add(1, Ordering::SeqCst);
        self.inodes.insert(ino, BfsInode::new(ino, name));
        ino
    }

    pub fn set_attr(&mut self, ino: u64, key: &str, val: AttrValue) -> bool {
        self.inodes.get_mut(&ino).map(|i| i.set_attr(key, val)).is_some()
    }

    /// Live query: find all files matching the predicate
    /// In real BFS, attributes are indexed in a separate B-Tree for O(log n) queries
    pub fn query(&self, predicate: &QueryPredicate) -> Vec<&BfsInode> {
        self.inodes.values().filter(|i| predicate.matches(i)).collect()
    }

    pub fn query_count(&self, predicate: &QueryPredicate) -> usize {
        self.inodes.values().filter(|i| predicate.matches(i)).count()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rump_kernel_alloc_free() {
        let kernel = Arc::new(RumpKernelSim::new());
        let addr = kernel.kmem_alloc(4096).unwrap();
        assert!(addr >= 0x1000_0000);
        kernel.kmem_free(addr, 4096);
        let log = kernel.alloc_log();
        assert!(log.is_empty());
    }

    #[test]
    fn test_rump_driver_init() {
        let kernel: Arc<dyn RumpKernelAbi> = Arc::new(RumpKernelSim::new());
        let mut drv = RumpDriverProcess::new("wm8960", "audio", Arc::clone(&kernel));
        drv.init().unwrap();
        assert_eq!(drv.state, RumpDriverState::Running);
        assert!(drv.irq.is_some());
        assert!(!drv.mem_allocations.is_empty());
    }

    #[test]
    fn test_rump_driver_detach() {
        let kernel: Arc<dyn RumpKernelAbi> = Arc::new(RumpKernelSim::new());
        let mut drv = RumpDriverProcess::new("rtl8169", "net", Arc::clone(&kernel));
        drv.init().unwrap();
        drv.detach();
        assert_eq!(drv.state, RumpDriverState::Unloaded);
        assert!(drv.mem_allocations.is_empty());
    }

    #[test]
    fn test_rump_fault_isolation() {
        let kernel: Arc<dyn RumpKernelAbi> = Arc::new(RumpKernelSim::new());
        let mut bridge = RumpHostBridge::new(Arc::clone(&kernel));
        bridge.load_driver("e1000", "net");
        bridge.load_driver("ahci", "block");
        assert_eq!(bridge.running_count(), 2);
        bridge.fault_driver(0);
        assert_eq!(bridge.running_count(), 1); // only ahci still running
    }

    #[test]
    fn test_bfs_attribute_set_get() {
        let mut vol = BfsVolume::new("home");
        let ino = vol.create_file("document.pdf");
        vol.set_attr(ino, "BEOS:TYPE", AttrValue::Str("application/pdf".into()));
        vol.set_attr(ino, "META:rating", AttrValue::Int64(5));
        vol.set_attr(ino, "META:author", AttrValue::Str("Alice".into()));

        let inode = vol.inodes.get(&ino).unwrap();
        assert_eq!(inode.get_attr("META:rating").and_then(|v| v.as_int64()), Some(5));
        assert_eq!(inode.get_attr("META:author").and_then(|v| v.as_str()), Some("Alice"));
    }

    #[test]
    fn test_bfs_live_query_by_type() {
        let mut vol = BfsVolume::new("media");
        let ino1 = vol.create_file("song.mp3");
        let ino2 = vol.create_file("photo.jpg");
        let ino3 = vol.create_file("video.mp4");

        vol.set_attr(ino1, "BEOS:TYPE", AttrValue::Str("audio/mpeg".into()));
        vol.set_attr(ino2, "BEOS:TYPE", AttrValue::Str("image/jpeg".into()));
        vol.set_attr(ino3, "BEOS:TYPE", AttrValue::Str("video/mp4".into()));

        let query = QueryPredicate::Eq("BEOS:TYPE".into(), AttrValue::Str("audio/mpeg".into()));
        let results = vol.query(&query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "song.mp3");
    }

    #[test]
    fn test_bfs_complex_query() {
        let mut vol = BfsVolume::new("docs");
        for i in 1..=5 {
            let ino = vol.create_file(&format!("file{}.rs", i));
            vol.set_attr(ino, "META:size", AttrValue::Int64(i * 1000));
            vol.set_attr(ino, "META:lang", AttrValue::Str("Rust".into()));
        }

        // Query: lang == "Rust" AND size > 3000
        let q = QueryPredicate::And(
            Box::new(QueryPredicate::Eq("META:lang".into(), AttrValue::Str("Rust".into()))),
            Box::new(QueryPredicate::Gt("META:size".into(), AttrValue::Int64(3000))),
        );
        let count = vol.query_count(&q);
        assert_eq!(count, 2); // files 4 and 5 (4000, 5000)
    }

    #[test]
    fn test_bfs_contains_query() {
        let mut vol = BfsVolume::new("email");
        let ino = vol.create_file("email001.eml");
        vol.set_attr(ino, "MAIL:subject", AttrValue::Str("Meeting notes for Q4".into()));
        let q = QueryPredicate::Contains("MAIL:subject".into(), "Q4".into());
        assert_eq!(vol.query_count(&q), 1);
    }
}
