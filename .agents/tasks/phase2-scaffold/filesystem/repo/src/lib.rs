//! # sigmaos-filesystem
//!
//! SigmaOS Virtual Filesystem layer: VFS abstraction, ext4 driver stub, tmpfs,
//! devfs, mount manager, POSIX file operations, and inode operations.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │              User-space / syscall layer  │
//! └──────────────────────┬──────────────────┘
//!                        │ POSIX open/read/write/close/pipe
//! ┌──────────────────────▼──────────────────┐
//! │              VFS layer  (vfs.rs)         │
//! │  VfsManager · MountTable · PathResolver  │
//! └──────┬───────────────────────────┬───────┘
//!        │                           │
//! ┌──────▼──────┐           ┌────────▼──────────────┐
//! │  ext4 driver│           │  tmpfs / devfs drivers │
//! │  (ext4.rs)  │           │  (tmpfs.rs / devfs.rs) │
//! └─────────────┘           └────────────────────────┘
//!        │
//! ┌──────▼──────────────────────────────────┐
//! │  MountManager  (mount.rs)               │
//! │  InodeTable    (inode.rs)               │
//! │  FdTable       (posix.rs)               │
//! └─────────────────────────────────────────┘
//! ```
//!
//! ## Principles applied
//! - **Separation of concerns** — VFS abstracts concrete FS implementations.
//! - **Stable APIs** — public traits (`InodeOps`, `FileOps`, `SuperblockOps`)
//!   are versioned and backward-compatible.
//! - **Minimal kernel philosophy** — this crate carries no kernel dependency;
//!   the `kernel` feature gate (see below) can wire in kernel types via traits.
//! - **POSIX compliance** — file types, modes, fd semantics follow POSIX.
//! - **Memory safety** — no `unsafe` code in the public API surface.
//! - **Single responsibility** — one module per concern.
//!
//! ## Cross-crate kernel integration
//!
//! When compiled with `features = ["kernel"]` (not yet wired — placeholder),
//! this crate can depend on `sigmaos-kernel` through trait objects so the VFS
//! layer can request physical block I/O without a hard compile-time dependency.

// ---------------------------------------------------------------------------
// Modules
// ---------------------------------------------------------------------------

pub mod vfs;
pub mod mount;
pub mod tmpfs;
pub mod ext4;
pub mod devfs;
pub mod posix;
pub mod inode;

// ---------------------------------------------------------------------------
// Re-exports — stable public API surface
// ---------------------------------------------------------------------------

// VFS layer
pub use vfs::{
    FileType, FileMode, OpenFlags, SeekWhence,
    InodeMetadata, Inode,
    InodeOps, FileOps, SuperblockOps, VirtualFilesystem,
    FilesystemStats, VfsMountFlags,
    Dentry, MountPoint, MountTable,
    PathResolver, VfsManager,
    VfsError, FsError,
};

// Mount manager
pub use mount::{
    MountError, MountState, MountFlags, MountEntry, MountManager,
    mount, unmount,
};

// tmpfs
pub use tmpfs::{
    TmpfsError, InodeType as TmpfsInodeType, TmpfsInode,
    TmpfsFilesystem, TmpfsStats,
};

// ext4
pub use ext4::{
    EXT4_MAGIC,
    Ext4Error, Ext4Superblock, Ext4GroupDesc, Ext4Inode,
    Ext4ExtentHeader, Ext4Extent, Ext4Stats, Ext4Filesystem,
};

// devfs
pub use devfs::{
    DevfsError, DevNodeKind, DevNode, DevFs,
};

// POSIX file operations
pub use posix::{
    PosixError, PosixResult, OFlags, FdTable, OpenFile,
};

// Inode operations
pub use inode::{
    InodeError, StatInfo, InodeRecord, InodeTable,
};

// ── Extracted monorepo filesystem source ────────────────────────────────────
// All 65 source files from src/filesystem/ in the SigmaOS monorepo are exposed
// under `sigmaos_filesystem::extracted`. Stub modules satisfy cross-crate
// references that the monorepo resolved via other subsystem crates.

#[allow(clippy::all, unused, ambiguous_glob_reexports)]
pub mod extracted {
    extern crate alloc;

    // Companion: kernel library primitives (HashMap, BTreeMap, Vec stubs)
    pub mod klib {
        pub use std::collections::HashMap;
        pub use std::collections::BTreeMap;
        pub use std::collections::HashSet;
        pub use std::vec::Vec;
        pub mod btreemap {
            pub use std::collections::BTreeMap;
        }
        pub mod vec {
            pub use std::vec::Vec;
        }
    }

    // Companion stubs: kernel IPC and namespace types
    pub mod kernel {
        pub mod msgqueue {
            use std::collections::{HashMap, VecDeque};
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct MsgQPerm { pub read: bool, pub write: bool }
            impl MsgQPerm {
                pub fn new() -> Self { Self { read: false, write: false } }
                pub fn with_read(mut self) -> Self { self.read = true; self }
                pub fn with_write(mut self) -> Self { self.write = true; self }
            }
            impl Default for MsgQPerm { fn default() -> Self { Self::new() } }
            #[derive(Debug, Clone)]
            pub struct Message { pub mtype: i64, pub data: Vec<u8> }
            impl Message { pub fn new(mtype: i64, data: Vec<u8>) -> Self { Self { mtype, data } } }
            pub struct MessageQueue { pub perm: MsgQPerm, pub messages: VecDeque<Message>, pub max_size: usize }
            impl MessageQueue {
                pub fn new(perm: MsgQPerm, max_size: usize) -> Self { Self { perm, messages: VecDeque::new(), max_size } }
                pub fn message_count(&self) -> usize { self.messages.len() }
                pub fn is_empty(&self) -> bool { self.messages.is_empty() }
                pub fn is_full(&self) -> bool { self.messages.len() >= self.max_size }
            }
            pub struct MessageQueueManager { queues: HashMap<u64, MessageQueue>, next_id: u64 }
            impl MessageQueueManager {
                pub fn new() -> Self { Self { queues: HashMap::new(), next_id: 1 } }
                pub fn create_queue(&mut self, _key: i32, perm: MsgQPerm, max_size: usize) -> u64 {
                    let id = self.next_id; self.next_id += 1;
                    self.queues.insert(id, MessageQueue::new(perm, max_size)); id
                }
                pub fn send(&mut self, qid: u64, msg: Message) -> Result<(), String> {
                    self.queues.get_mut(&qid).ok_or_else(|| "not found".to_string()).map(|q| q.messages.push_back(msg))
                }
                pub fn receive(&mut self, qid: u64, _mtype: i64) -> Result<Message, String> {
                    self.queues.get_mut(&qid).ok_or_else(|| "not found".to_string())?.messages.pop_front().ok_or_else(|| "empty".to_string())
                }
                pub fn remove_queue(&mut self, qid: u64) -> Result<(), String> {
                    self.queues.remove(&qid).map(|_| ()).ok_or_else(|| "not found".to_string())
                }
                pub fn get_queue(&self, qid: u64) -> Option<&MessageQueue> { self.queues.get(&qid) }
            }
        }
        pub mod namespaces {
            use std::sync::atomic::{AtomicU64, Ordering};
            static NS_CTR: AtomicU64 = AtomicU64::new(1);
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
            pub struct NamespaceId(pub u64);
            impl NamespaceId { pub fn raw(&self) -> u64 { self.0 } }
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum NamespaceType { Mount, Pid, Network, Ipc, Uts, User, Cgroup, Time }
            pub use NamespaceType as KernelNamespaceType;
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum NamespaceError { NotFound, PermissionDenied, MaxNamespacesReached, InvalidType, InvalidNamespaceId }
            pub trait KernelNamespace {
                fn namespace_id(&self) -> NamespaceId;
                fn namespace_type(&self) -> NamespaceType;
                fn ref_count(&self) -> u32 { 0 }
                fn increment_ref(&self) {}
                fn decrement_ref(&self) {}
                fn metadata(&self) -> String { String::new() }
            }
            pub fn next_namespace_id() -> NamespaceId { NamespaceId(NS_CTR.fetch_add(1, Ordering::Relaxed)) }
        }
    }

    // Stub: security::vault
    pub mod security {
        pub mod vault {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum EncryptionAlgorithm { Aes256Gcm, ChaCha20Poly1305, Kyber1024 }
        }
    }

    // Stub: stubs::vfs_stubs
    pub mod stubs {
        pub mod vfs_stubs {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum FsError { NotFound, PermissionDenied, AlreadyExists, IoError, InvalidInput }
            pub type Inode = u64;
            #[derive(Debug)]
            pub struct VirtualFilesystem { pub root_inode: Inode, pub last_written: usize }
            impl VirtualFilesystem { pub fn new() -> Self { Self { root_inode: 0, last_written: 0 } } }
        }
    }

    // Stub: access::control
    pub mod access {
        pub mod control {
            #[derive(Debug, Clone)]
            pub struct SecurityIdentifier { pub revision: u8, pub identifier_authority: [u8; 6], pub sub_authorities: Vec<u32> }
            impl Default for SecurityIdentifier { fn default() -> Self { Self { revision: 1, identifier_authority: [0;6], sub_authorities: vec![] } } }
            #[derive(Debug, Clone)]
            pub struct NtfsAce { pub sid: SecurityIdentifier, pub access_mask: u32 }
            impl NtfsAce { pub fn allow(sid: SecurityIdentifier, mask: u32, _f: u32) -> Self { Self { sid, access_mask: mask } } }
            #[derive(Debug, Clone)]
            pub struct NtfsDacl { pub aces: Vec<NtfsAce> }
            impl NtfsDacl { pub fn new() -> Self { Self { aces: vec![] } } pub fn add_ace(&mut self, a: NtfsAce) { self.aces.push(a); } }
            #[derive(Debug, Clone)]
            pub struct NtfsSacl { pub audit_aces: Vec<NtfsAce> }
            #[derive(Debug, Clone)]
            pub struct NtfsSecurityDescriptor { pub owner_sid: SecurityIdentifier, pub group_sid: SecurityIdentifier, pub dacl: NtfsDacl, pub sacl: NtfsSacl }
            impl NtfsSecurityDescriptor {
                pub fn evaluate_access(&self, _s: &SecurityIdentifier, _g: &[SecurityIdentifier], _r: u32) -> bool { true }
                pub fn add_dacl_ace(&mut self, a: NtfsAce) { self.dacl.add_ace(a); }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct Ext4InodeMode { pub user_read: bool, pub user_write: bool, pub user_exec: bool, pub group_read: bool, pub group_write: bool, pub group_exec: bool, pub other_read: bool, pub other_write: bool, pub other_exec: bool }
            impl Ext4InodeMode { pub fn new() -> Self { Self { user_read: true, user_write: true, user_exec: false, group_read: true, group_write: false, group_exec: false, other_read: true, other_write: false, other_exec: false } } }
            #[derive(Debug, Clone)]
            pub struct Ext4Xattr { pub name: Vec<u8>, pub value: Vec<u8> }
            impl Ext4Xattr { pub fn new(name: &[u8], value: &[u8]) -> Self { Self { name: name.to_vec(), value: value.to_vec() } } }
            #[derive(Debug, Clone)]
            pub struct PosixAclTable { pub entries: Vec<(u32, u32)> }
            impl PosixAclTable { pub fn new() -> Self { Self { entries: vec![] } } }
            #[derive(Debug, Clone, Copy)]
            pub struct Ext4AccessCheckEngine;
            impl Ext4AccessCheckEngine {
                pub fn new() -> Self { Self }
                pub fn check_access(&self, _m: &Ext4InodeMode, _u: u32, _g: u32, _a: Option<&PosixAclTable>, _r: u32) -> bool { true }
                pub fn evaluate_ext4_access(_su: u32, _sg: u32, _rm: u16, _ou: u32, _pg: u32, _mb: u16, _a: Option<&PosixAclTable>) -> bool { true }
            }
        }
    }

    // Stub: compatibility::legacy_adapters
    pub mod compatibility {
        pub mod legacy_adapters {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum KernelPersona { Linux, Linux6X, Linux2_6, Bsd, SigmaOs }
            impl KernelPersona {
                pub fn name(&self) -> &'static str {
                    match self { Self::Linux6X => "linux_6x", Self::Linux2_6 => "linux_2_6", Self::Linux => "linux", Self::Bsd => "bsd", Self::SigmaOs => "sigmaos" }
                }
                pub fn api_version(&self) -> u32 {
                    match self { Self::Linux6X => 60000, Self::Linux2_6 => 20006, Self::Linux => 50000, Self::Bsd => 0, Self::SigmaOs => 10000 }
                }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum SyscallAbi { Linux, Bsd, Native, Oabi32, Eabi32, Eabi64 }
        }
    }

    // Extracted monorepo filesystem modules (src/filesystem/ from SigmaOS monorepo)
    // Rust resolves #[path] relative to src/extracted/ for inline sub-modules,
    // so we go up one level to reach src/mod.rs.
    #[path = "../mod.rs"]
    pub mod filesystem;
}
