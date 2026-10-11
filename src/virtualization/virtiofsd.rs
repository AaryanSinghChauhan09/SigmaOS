// SigmaOS Virtio-FS Host Daemon Engine (virtiofsd)
// Reference: Linux drivers/virtio/fs.c, qemu virtio-fs, rust-vmm virtio-fs

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// VirtIO-FS Configuration Space layout
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtioFsConfig {
    pub tag: [u8; 36],
    pub num_request_queues: u32,
}

impl VirtioFsConfig {
    pub fn new(tag_name: &str, num_queues: u32) -> Self {
        let mut tag = [0u8; 36];
        let bytes = tag_name.as_bytes();
        let len = bytes.len().min(36);
        tag[..len].copy_from_slice(&bytes[..len]);

        Self {
            tag,
            num_request_queues: num_queues,
        }
    }
}

/// VirtIO-FS Inode mapping between guest and host filesystem
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtioFsInodeMap {
    pub guest_inode: u64,
    pub host_path: String,
    pub ref_count: u64,
}

/// Virtiofsd Host Daemon Instance
pub struct VirtioFsDaemon {
    config: VirtioFsConfig,
    shared_dir: String,
    inodes: BTreeMap<u64, VirtioFsInodeMap>,
    next_inode: u64,
    is_active: bool,
}

impl VirtioFsDaemon {
    pub fn new(shared_dir: &str, tag_name: &str) -> Self {
        let mut daemon = Self {
            config: VirtioFsConfig::new(tag_name, 2),
            shared_dir: String::from(shared_dir),
            inodes: BTreeMap::new(),
            next_inode: 2,
            is_active: false,
        };

        // Root inode is 1
        daemon.inodes.insert(
            1,
            VirtioFsInodeMap {
                guest_inode: 1,
                host_path: String::from(shared_dir),
                ref_count: 1,
            },
        );

        daemon
    }

    pub fn start(&mut self) {
        self.is_active = true;
    }

    pub fn stop(&mut self) {
        self.is_active = false;
    }

    /// Allocates or increments ref count for a guest inode
    pub fn lookup_path(&mut self, relative_path: &str) -> u64 {
        let full_path = alloc::format!("{}/{}", self.shared_dir, relative_path);

        // Check if path already tracked
        for map in self.inodes.values_mut() {
            if map.host_path == full_path {
                map.ref_count += 1;
                return map.guest_inode;
            }
        }

        let ino = self.next_inode;
        self.next_inode += 1;

        self.inodes.insert(
            ino,
            VirtioFsInodeMap {
                guest_inode: ino,
                host_path: full_path,
                ref_count: 1,
            },
        );

        ino
    }

    /// Decrement inode ref count (forget)
    pub fn forget(&mut self, ino: u64, nlookup: u64) -> bool {
        if let Some(map) = self.inodes.get_mut(&ino) {
            if map.ref_count <= nlookup {
                self.inodes.remove(&ino);
            } else {
                map.ref_count -= nlookup;
            }
            true
        } else {
            false
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn tag_string(&self) -> String {
        let len = self.config.tag.iter().position(|&c| c == 0).unwrap_or(36);
        String::from_utf8_lossy(&self.config.tag[..len]).into_owned()
    }
}
