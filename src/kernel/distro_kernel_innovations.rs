// SigmaOS Linux & BSD Inspired Kernel Innovations Subsystem
// Incorporates kernel-level features from Linux and BSD distributions:
// 1. Linux memfd_secret secret memory mapping engine (LinuxMemfdSecretEngine)
// 2. Linux Binder IPC transaction engine (LinuxBinderIpcEngine)
// 3. Linux Zswap compressed in-memory swap cache (LinuxZswapCompressedEngine)
// 4. Linux OverlayFS copy-up & whiteout engine (LinuxOverlayfsEngine)
// 5. FreeBSD Capsicum Casper capability daemon IPC engine (FreeBsdCasperDaemonEngine)

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec;
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ============================================================================
// 1. Linux memfd_secret Secret Memory Mapping Engine
// ============================================================================

#[derive(Debug, Clone)]
pub struct SecretMemoryArea {
    pub fd: i32,
    pub size_bytes: usize,
    pub is_locked: bool,
    pub encrypted_payload: Vec<u8>,
}

pub struct LinuxMemfdSecretEngine {
    pub secret_areas: BTreeMap<i32, SecretMemoryArea>,
    pub next_fd: i32,
}

impl LinuxMemfdSecretEngine {
    pub fn new() -> Self {
        Self {
            secret_areas: BTreeMap::new(),
            next_fd: 100,
        }
    }

    pub fn create_secret_memfd(&mut self, size: usize) -> Result<i32, &'static str> {
        if size == 0 || size % 4096 != 0 {
            return Err("MemfdSecret: Size must be a positive multiple of page size (4096)");
        }

        let fd = self.next_fd;
        self.next_fd += 1;

        let area = SecretMemoryArea {
            fd,
            size_bytes: size,
            is_locked: true,
            encrypted_payload: vec![0u8; size],
        };

        self.secret_areas.insert(fd, area);
        Ok(fd)
    }

    pub fn write_secret(&mut self, fd: i32, data: &[u8]) -> Result<(), &'static str> {
        let area = self
            .secret_areas
            .get_mut(&fd)
            .ok_or("MemfdSecret: File descriptor not found")?;

        if data.len() > area.size_bytes {
            return Err("MemfdSecret: Data exceeds secret buffer size");
        }

        for (i, &byte) in data.iter().enumerate() {
            area.encrypted_payload[i] = byte ^ 0xAA; // Simple obfuscation/encryption simulation
        }
        Ok(())
    }

    pub fn read_secret(&self, fd: i32, buf: &mut [u8]) -> Result<usize, &'static str> {
        let area = self
            .secret_areas
            .get(&fd)
            .ok_or("MemfdSecret: File descriptor not found")?;

        let read_len = buf.len().min(area.size_bytes);
        for i in 0..read_len {
            buf[i] = area.encrypted_payload[i] ^ 0xAA;
        }
        Ok(read_len)
    }
}

impl Default for LinuxMemfdSecretEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. Linux Binder IPC Transaction Engine
// ============================================================================

#[derive(Debug, Clone)]
pub struct BinderNode {
    pub handle: u32,
    pub name: String,
    pub owner_pid: u32,
}

#[derive(Debug, Clone)]
pub struct BinderTransaction {
    pub transaction_id: u64,
    pub sender_pid: u32,
    pub target_handle: u32,
    pub code: u32,
    pub payload: Vec<u8>,
}

pub struct LinuxBinderIpcEngine {
    pub nodes: BTreeMap<u32, BinderNode>,
    pub pending_transactions: Vec<BinderTransaction>,
    pub next_tx_id: u64,
}

impl LinuxBinderIpcEngine {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            pending_transactions: Vec::new(),
            next_tx_id: 1,
        }
    }

    pub fn register_binder_node(&mut self, handle: u32, name: &str, owner_pid: u32) {
        let node = BinderNode {
            handle,
            name: name.to_string(),
            owner_pid,
        };
        self.nodes.insert(handle, node);
    }

    pub fn send_transaction(
        &mut self,
        sender_pid: u32,
        target_handle: u32,
        code: u32,
        payload: &[u8],
    ) -> Result<u64, &'static str> {
        if !self.nodes.contains_key(&target_handle) {
            return Err("Binder: Target node handle not found");
        }

        let tx_id = self.next_tx_id;
        self.next_tx_id += 1;

        self.pending_transactions.push(BinderTransaction {
            transaction_id: tx_id,
            sender_pid,
            target_handle,
            code,
            payload: payload.to_vec(),
        });

        Ok(tx_id)
    }

    pub fn receive_transaction(&mut self, target_handle: u32) -> Option<BinderTransaction> {
        if let Some(pos) = self
            .pending_transactions
            .iter()
            .position(|tx| tx.target_handle == target_handle)
        {
            Some(self.pending_transactions.remove(pos))
        } else {
            None
        }
    }
}

impl Default for LinuxBinderIpcEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. Linux Zswap Compressed In-Memory Swap Cache
// ============================================================================

#[derive(Debug, Clone)]
pub struct ZswapCompressedPage {
    pub page_index: u64,
    pub original_size: usize,
    pub compressed_data: Vec<u8>,
}

pub struct LinuxZswapCompressedEngine {
    pub stored_pages: BTreeMap<u64, ZswapCompressedPage>,
    pub pool_size_bytes: usize,
    pub max_pool_size_bytes: usize,
}

impl LinuxZswapCompressedEngine {
    pub fn new(max_pool_bytes: usize) -> Self {
        Self {
            stored_pages: BTreeMap::new(),
            pool_size_bytes: 0,
            max_pool_size_bytes: max_pool_bytes,
        }
    }

    pub fn compress_and_store_page(&mut self, page_idx: u64, page_data: &[u8]) -> Result<usize, &'static str> {
        if page_data.len() != 4096 {
            return Err("Zswap: Input page size must be exactly 4096 bytes");
        }

        // RLE compression simulation
        let mut compressed = Vec::new();
        let mut i = 0;
        while i < page_data.len() {
            let byte = page_data[i];
            let mut count = 1;
            while i + count < page_data.len() && page_data[i + count] == byte && count < 255 {
                count += 1;
            }
            compressed.push(count as u8);
            compressed.push(byte);
            i += count;
        }

        if self.pool_size_bytes + compressed.len() > self.max_pool_size_bytes {
            return Err("Zswap: Pool capacity exceeded, rejecting compression");
        }

        let compressed_size = compressed.len();
        self.pool_size_bytes += compressed_size;

        self.stored_pages.insert(
            page_idx,
            ZswapCompressedPage {
                page_index: page_idx,
                original_size: 4096,
                compressed_data: compressed,
            },
        );

        Ok(compressed_size)
    }

    pub fn decompress_page(&mut self, page_idx: u64) -> Result<Vec<u8>, &'static str> {
        let entry = self
            .stored_pages
            .remove(&page_idx)
            .ok_or("Zswap: Page index not found in cache")?;

        self.pool_size_bytes = self.pool_size_bytes.saturating_sub(entry.compressed_data.len());

        let mut decompressed = Vec::with_capacity(4096);
        let mut i = 0;
        while i < entry.compressed_data.len() {
            let count = entry.compressed_data[i] as usize;
            let byte = entry.compressed_data[i + 1];
            for _ in 0..count {
                decompressed.push(byte);
            }
            i += 2;
        }

        Ok(decompressed)
    }

    pub fn compression_ratio(&self) -> f32 {
        if self.stored_pages.is_empty() {
            return 1.0;
        }

        let total_uncompressed = self.stored_pages.len() * 4096;
        total_uncompressed as f32 / self.pool_size_bytes as f32
    }
}

impl Default for LinuxZswapCompressedEngine {
    fn default() -> Self {
        Self::new(1024 * 1024 * 64) // 64 MB default max pool
    }
}

// ============================================================================
// 4. Linux OverlayFS Copy-Up & Whiteout Engine
// ============================================================================

pub struct LinuxOverlayfsEngine {
    pub lower_layers: Vec<String>,
    pub upper_layer: String,
    pub work_dir: String,
    pub whiteout_entries: Vec<String>,
    pub upper_overrides: BTreeMap<String, Vec<u8>>,
}

impl LinuxOverlayfsEngine {
    pub fn new(lower: &[&str], upper: &str, work: &str) -> Self {
        Self {
            lower_layers: lower.iter().map(|s| s.to_string()).collect(),
            upper_layer: upper.to_string(),
            work_dir: work.to_string(),
            whiteout_entries: Vec::new(),
            upper_overrides: BTreeMap::new(),
        }
    }

    pub fn create_whiteout(&mut self, rel_path: &str) {
        if !self.whiteout_entries.contains(&rel_path.to_string()) {
            self.whiteout_entries.push(rel_path.to_string());
        }
        self.upper_overrides.remove(rel_path);
    }

    pub fn copy_up_file(&mut self, rel_path: &str, lower_data: &[u8]) -> Result<(), &'static str> {
        if self.whiteout_entries.contains(&rel_path.to_string()) {
            self.whiteout_entries.retain(|p| p != rel_path);
        }
        self.upper_overrides
            .insert(rel_path.to_string(), lower_data.to_vec());
        Ok(())
    }

    pub fn resolve_path(&self, rel_path: &str) -> Option<String> {
        if self.whiteout_entries.contains(&rel_path.to_string()) {
            return None; // Whiteout hides lower file
        }

        if self.upper_overrides.contains_key(rel_path) {
            return Some(format!("{}/{}", self.upper_layer, rel_path));
        }

        if let Some(lower) = self.lower_layers.first() {
            return Some(format!("{}/{}", lower, rel_path));
        }

        None
    }
}

impl Default for LinuxOverlayfsEngine {
    fn default() -> Self {
        Self::new(&["/lower"], "/upper", "/work")
    }
}

// ============================================================================
// 5. FreeBSD Capsicum Casper Capability Daemon IPC Engine
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CasperServiceType {
    CasperDns,
    CasperGrp,
    CasperPwd,
    CasperSysctl,
}

#[derive(Debug, Clone)]
pub struct CasperChannel {
    pub channel_id: u32,
    pub service_type: CasperServiceType,
    pub is_closed: bool,
}

pub struct FreeBsdCasperDaemonEngine {
    pub active_channels: BTreeMap<u32, CasperChannel>,
    pub next_channel_id: u32,
}

impl FreeBsdCasperDaemonEngine {
    pub fn new() -> Self {
        Self {
            active_channels: BTreeMap::new(),
            next_channel_id: 1,
        }
    }

    pub fn open_casper_service(&mut self, service: CasperServiceType) -> u32 {
        let channel_id = self.next_channel_id;
        self.next_channel_id += 1;

        let channel = CasperChannel {
            channel_id,
            service_type: service,
            is_closed: false,
        };

        self.active_channels.insert(channel_id, channel);
        channel_id
    }

    pub fn dispatch_casper_request(&self, channel_id: u32, query: &str) -> Result<String, &'static str> {
        let channel = self
            .active_channels
            .get(&channel_id)
            .ok_or("Casper: Channel ID not found")?;

        if channel.is_closed {
            return Err("Casper: Channel is closed");
        }

        match channel.service_type {
            CasperServiceType::CasperDns => Ok(format!("casper_dns_resolve({}) -> 127.0.0.1", query)),
            CasperServiceType::CasperGrp => Ok(format!("casper_grp_lookup({}) -> gid=1000", query)),
            CasperServiceType::CasperPwd => Ok(format!("casper_pwd_lookup({}) -> uid=1000", query)),
            CasperServiceType::CasperSysctl => Ok(format!("casper_sysctl_get({}) -> val=1", query)),
        }
    }

    pub fn close_casper_service(&mut self, channel_id: u32) -> Result<(), &'static str> {
        let channel = self
            .active_channels
            .get_mut(&channel_id)
            .ok_or("Casper: Channel ID not found")?;

        channel.is_closed = true;
        Ok(())
    }
}

impl Default for FreeBsdCasperDaemonEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 6. Master Suite Coordinator
// ============================================================================

pub struct SovereignDistroKernelInnovationsSuite {
    pub memfd_secret: LinuxMemfdSecretEngine,
    pub binder: LinuxBinderIpcEngine,
    pub zswap: LinuxZswapCompressedEngine,
    pub overlayfs: LinuxOverlayfsEngine,
    pub casper: FreeBsdCasperDaemonEngine,
}

impl SovereignDistroKernelInnovationsSuite {
    pub fn new() -> Self {
        Self {
            memfd_secret: LinuxMemfdSecretEngine::default(),
            binder: LinuxBinderIpcEngine::default(),
            zswap: LinuxZswapCompressedEngine::default(),
            overlayfs: LinuxOverlayfsEngine::default(),
            casper: FreeBsdCasperDaemonEngine::default(),
        }
    }
}

impl Default for SovereignDistroKernelInnovationsSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_memfd_secret_engine() {
        let mut memfd = LinuxMemfdSecretEngine::new();
        let fd = memfd.create_secret_memfd(4096).unwrap();

        let secret_data = b"TOP_SECRET_KERNEL_KEY";
        assert!(memfd.write_secret(fd, secret_data).is_ok());

        let mut read_buf = [0u8; 21];
        let bytes_read = memfd.read_secret(fd, &mut read_buf).unwrap();

        assert_eq!(bytes_read, 21);
        assert_eq!(&read_buf, secret_data);
    }

    #[test]
    fn test_linux_binder_ipc_engine() {
        let mut binder = LinuxBinderIpcEngine::new();
        binder.register_binder_node(1, "surfaceflinger", 100);

        let tx_id = binder
            .send_transaction(101, 1, 10, b"TRANSACTION_PAYLOAD")
            .unwrap();
        assert_eq!(tx_id, 1);

        let tx = binder.receive_transaction(1).unwrap();
        assert_eq!(tx.sender_pid, 101);
        assert_eq!(tx.payload, b"TRANSACTION_PAYLOAD");
    }

    #[test]
    fn test_linux_zswap_compressed_engine() {
        let mut zswap = LinuxZswapCompressedEngine::new(1024 * 1024);
        let dummy_page = [0x41u8; 4096];

        let compressed_size = zswap.compress_and_store_page(100, &dummy_page).unwrap();
        assert!(compressed_size < 4096);
        assert!(zswap.compression_ratio() > 10.0);

        let decompressed = zswap.decompress_page(100).unwrap();
        assert_eq!(decompressed.len(), 4096);
        assert_eq!(decompressed[0], 0x41);
    }

    #[test]
    fn test_linux_overlayfs_engine() {
        let mut overlay = LinuxOverlayfsEngine::new(&["/lower"], "/upper", "/work");

        overlay.copy_up_file("etc/nginx.conf", b"worker_processes 4;").unwrap();
        assert_eq!(
            overlay.resolve_path("etc/nginx.conf"),
            Some("/upper/etc/nginx.conf".to_string())
        );

        overlay.create_whiteout("etc/nginx.conf");
        assert_eq!(overlay.resolve_path("etc/nginx.conf"), None);
    }

    #[test]
    fn test_freebsd_casper_daemon_engine() {
        let mut casper = FreeBsdCasperDaemonEngine::new();
        let channel = casper.open_casper_service(CasperServiceType::CasperDns);

        let resp = casper.dispatch_casper_request(channel, "example.com").unwrap();
        assert!(resp.contains("casper_dns_resolve"));

        assert!(casper.close_casper_service(channel).is_ok());
        assert!(casper.dispatch_casper_request(channel, "example.com").is_err());
    }
}
