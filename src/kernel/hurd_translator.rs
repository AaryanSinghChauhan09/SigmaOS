// SPDX-License-Identifier: GPL-2.0-or-later
// Principle: MICROKERNEL TRANSLATORS, SEPARATION OF MECHANISM AND POLICY, HURD VFS ABSTRACTION

use alloc::{vec::Vec, string::String, collections::BTreeMap, boxed::Box};

/// A mounted translator instance
#[derive(Debug, Clone)]
pub struct TranslatorMount {
    pub path: String,
    pub translator_name: String,
    pub flags: u32,
    pub active: bool,
}

impl TranslatorMount {
    pub fn new(path: String, translator_name: String, flags: u32) -> Self {
        Self { path, translator_name, flags, active: false }
    }
}

/// Request sent to a translator
#[derive(Debug, Clone)]
pub struct TranslatorRequest {
    pub op: TranslatorOp,
    pub path: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslatorOp {
    Lookup,
    Open,
    Read,
    Write,
    Stat,
    Readdir,
    Mkdir,
    Unlink,
    Rename,
}

/// Response from a translator
#[derive(Debug)]
pub struct TranslatorResponse {
    pub status: i32,
    pub data: Vec<u8>,
}

impl TranslatorResponse {
    pub fn ok(data: Vec<u8>) -> Self { Self { status: 0, data } }
    pub fn err(code: i32) -> Self { Self { status: code, data: Vec::new() } }
}

/// Trait for all Hurd filesystem translators
pub trait Translator {
    fn name(&self) -> &str;
    fn handle_request(&mut self, req: TranslatorRequest) -> TranslatorResponse;
    fn startup(&mut self) -> Result<(), &'static str>;
    fn shutdown(&mut self);
}

/// Ext2 translator (stub — real impl would use block I/O)
pub struct Ext2Translator {
    pub device: String,
    pub block_size: u32,
    pub inode_count: u32,
    pub mounted: bool,
}

impl Ext2Translator {
    pub fn new(device: String) -> Self {
        Self { device, block_size: 4096, inode_count: 0, mounted: false }
    }
}

impl Translator for Ext2Translator {
    fn name(&self) -> &str { "ext2fs" }

    fn startup(&mut self) -> Result<(), &'static str> {
        // Would parse superblock from block device
        self.mounted = true;
        Ok(())
    }

    fn shutdown(&mut self) {
        self.mounted = false;
    }

    fn handle_request(&mut self, req: TranslatorRequest) -> TranslatorResponse {
        if !self.mounted {
            return TranslatorResponse::err(-5); // EIO
        }
        match req.op {
            TranslatorOp::Lookup => TranslatorResponse::ok(alloc::vec![1]),
            TranslatorOp::Read => TranslatorResponse::ok(Vec::new()),
            _ => TranslatorResponse::err(-38), // ENOSYS
        }
    }
}

/// FAT translator
pub struct FatTranslator {
    pub device: String,
    pub fat_type: u8,  // 12, 16, or 32
    pub cluster_size: u32,
    pub mounted: bool,
}

impl FatTranslator {
    pub fn new(device: String, fat_type: u8) -> Self {
        Self { device, fat_type, cluster_size: 4096, mounted: false }
    }
}

impl Translator for FatTranslator {
    fn name(&self) -> &str { "fatfs" }

    fn startup(&mut self) -> Result<(), &'static str> {
        if self.fat_type != 12 && self.fat_type != 16 && self.fat_type != 32 {
            return Err("unsupported FAT type");
        }
        self.mounted = true;
        Ok(())
    }

    fn shutdown(&mut self) {
        self.mounted = false;
    }

    fn handle_request(&mut self, req: TranslatorRequest) -> TranslatorResponse {
        match req.op {
            TranslatorOp::Lookup => TranslatorResponse::ok(alloc::vec![0]),
            TranslatorOp::Readdir => TranslatorResponse::ok(Vec::new()),
            _ => TranslatorResponse::err(-38),
        }
    }
}

/// /proc translator
pub struct ProcTranslator {
    pub pid_entries: BTreeMap<u32, String>,
}

impl ProcTranslator {
    pub fn new() -> Self {
        Self { pid_entries: BTreeMap::new() }
    }

    pub fn register_pid(&mut self, pid: u32, name: String) {
        self.pid_entries.insert(pid, name);
    }
}

impl Translator for ProcTranslator {
    fn name(&self) -> &str { "procfs" }

    fn startup(&mut self) -> Result<(), &'static str> { Ok(()) }
    fn shutdown(&mut self) { self.pid_entries.clear(); }

    fn handle_request(&mut self, req: TranslatorRequest) -> TranslatorResponse {
        match req.op {
            TranslatorOp::Readdir => {
                let mut data = Vec::new();
                for pid in self.pid_entries.keys() {
                    let s = alloc::format!("{}\n", pid);
                    data.extend_from_slice(s.as_bytes());
                }
                TranslatorResponse::ok(data)
            }
            TranslatorOp::Lookup => {
                // Try parse pid from path
                if let Ok(pid) = req.path.trim_start_matches('/').parse::<u32>() {
                    if self.pid_entries.contains_key(&pid) {
                        return TranslatorResponse::ok(alloc::vec![1]);
                    }
                }
                TranslatorResponse::err(-2) // ENOENT
            }
            _ => TranslatorResponse::err(-38),
        }
    }
}

/// pfinet (TCP/IP) translator
pub struct PfinetTranslator {
    pub interface: String,
    pub ip_address: [u8; 4],
    pub running: bool,
}

impl PfinetTranslator {
    pub fn new(interface: String) -> Self {
        Self { interface, ip_address: [127, 0, 0, 1], running: false }
    }

    pub fn set_ip(&mut self, ip: [u8; 4]) {
        self.ip_address = ip;
    }
}

impl Translator for PfinetTranslator {
    fn name(&self) -> &str { "pfinet" }

    fn startup(&mut self) -> Result<(), &'static str> {
        self.running = true;
        Ok(())
    }

    fn shutdown(&mut self) { self.running = false; }

    fn handle_request(&mut self, req: TranslatorRequest) -> TranslatorResponse {
        if !self.running { return TranslatorResponse::err(-5); }
        TranslatorResponse::ok(self.ip_address.to_vec())
    }
}

/// Term translator (/dev/tty)
pub struct TermTranslator {
    pub tty_name: String,
    pub buffer: Vec<u8>,
    pub cols: u16,
    pub rows: u16,
}

impl TermTranslator {
    pub fn new(tty_name: String) -> Self {
        Self { tty_name, buffer: Vec::new(), cols: 80, rows: 24 }
    }
}

impl Translator for TermTranslator {
    fn name(&self) -> &str { "term" }

    fn startup(&mut self) -> Result<(), &'static str> { Ok(()) }
    fn shutdown(&mut self) { self.buffer.clear(); }

    fn handle_request(&mut self, req: TranslatorRequest) -> TranslatorResponse {
        match req.op {
            TranslatorOp::Write => {
                self.buffer.extend_from_slice(&req.data);
                TranslatorResponse::ok(alloc::vec![req.data.len() as u8])
            }
            TranslatorOp::Read => {
                let out = self.buffer.clone();
                self.buffer.clear();
                TranslatorResponse::ok(out)
            }
            _ => TranslatorResponse::err(-38),
        }
    }
}

/// Hurd Virtual Filesystem — dispatches to registered translators
pub struct HurdVfs {
    mounts: Vec<TranslatorMount>,
    translators: BTreeMap<String, Box<dyn Translator>>,
}

impl HurdVfs {
    pub fn new() -> Self {
        Self {
            mounts: Vec::new(),
            translators: BTreeMap::new(),
        }
    }

    pub fn register_translator(&mut self, mut t: Box<dyn Translator>) -> Result<(), &'static str> {
        t.startup()?;
        let name = String::from(t.name());
        self.translators.insert(name, t);
        Ok(())
    }

    pub fn mount(&mut self, path: String, translator_name: String, flags: u32) -> Result<(), &'static str> {
        if !self.translators.contains_key(&translator_name) {
            return Err("translator not registered");
        }
        let mut m = TranslatorMount::new(path, translator_name, flags);
        m.active = true;
        self.mounts.push(m);
        Ok(())
    }

    pub fn unmount(&mut self, path: &str) -> Result<(), &'static str> {
        let before = self.mounts.len();
        self.mounts.retain(|m| m.path != path);
        if self.mounts.len() < before { Ok(()) } else { Err("path not mounted") }
    }

    pub fn lookup_path(&self, path: &str) -> Option<&str> {
        // Find deepest matching mount
        let mut best: Option<&TranslatorMount> = None;
        for m in &self.mounts {
            if path.starts_with(m.path.as_str()) && m.active {
                if best.is_none() || m.path.len() > best.unwrap().path.len() {
                    best = Some(m);
                }
            }
        }
        best.map(|m| m.translator_name.as_str())
    }

    pub fn dispatch(&mut self, path: &str, req: TranslatorRequest) -> TranslatorResponse {
        let tname = self.lookup_path(path).map(|s| String::from(s));
        match tname {
            Some(name) => {
                if let Some(t) = self.translators.get_mut(&name) {
                    t.handle_request(req)
                } else {
                    TranslatorResponse::err(-2)
                }
            }
            None => TranslatorResponse::err(-2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext2_mount_lookup() {
        let mut vfs = HurdVfs::new();
        let ext2 = Box::new(Ext2Translator::new(String::from("/dev/sda1")));
        vfs.register_translator(ext2).unwrap();
        vfs.mount(String::from("/"), String::from("ext2fs"), 0).unwrap();
        let t = vfs.lookup_path("/home/user");
        assert_eq!(t, Some("ext2fs"));
    }

    #[test]
    fn test_unmount() {
        let mut vfs = HurdVfs::new();
        let ext2 = Box::new(Ext2Translator::new(String::from("/dev/sda1")));
        vfs.register_translator(ext2).unwrap();
        vfs.mount(String::from("/"), String::from("ext2fs"), 0).unwrap();
        vfs.unmount("/").unwrap();
        assert!(vfs.lookup_path("/").is_none());
    }

    #[test]
    fn test_proc_translator() {
        let mut proc = ProcTranslator::new();
        proc.register_pid(1, String::from("init"));
        proc.startup().unwrap();
        let resp = proc.handle_request(TranslatorRequest {
            op: TranslatorOp::Lookup,
            path: String::from("/1"),
            data: Vec::new(),
        });
        assert_eq!(resp.status, 0);
    }

    #[test]
    fn test_fat_bad_type() {
        let mut fat = FatTranslator::new(String::from("/dev/sdb"), 64);
        assert!(fat.startup().is_err());
    }

    #[test]
    fn test_term_write_read() {
        let mut term = TermTranslator::new(String::from("tty0"));
        term.startup().unwrap();
        let resp = term.handle_request(TranslatorRequest {
            op: TranslatorOp::Write,
            path: String::new(),
            data: alloc::vec![b'H', b'i'],
        });
        assert_eq!(resp.status, 0);
        let r = term.handle_request(TranslatorRequest {
            op: TranslatorOp::Read,
            path: String::new(),
            data: Vec::new(),
        });
        assert_eq!(r.data, alloc::vec![b'H', b'i']);
    }
}
