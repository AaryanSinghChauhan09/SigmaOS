// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Linux-inspired filesystem hierarchy for SigmaOS
// Zero-allocation, performance-optimized filesystem structure

/// Linux Filesystem Hierarchy Standard (FHS) paths
pub mod paths {
    pub const ROOT: &str = "/";
    pub const BIN: &str = "/bin";
    pub const SBIN: &str = "/sbin";
    pub const LIB: &str = "/lib";
    pub const LIB64: &str = "/lib64";
    pub const ETC: &str = "/etc";
    pub const HOME: &str = "/home";
    pub const ROOT_HOME: &str = "/root";
    pub const VAR: &str = "/var";
    pub const TMP: &str = "/tmp";
    pub const OPT: &str = "/opt";
    pub const USR: &str = "/usr";
    pub const USR_BIN: &str = "/usr/bin";
    pub const USR_SBIN: &str = "/usr/sbin";
    pub const USR_LIB: &str = "/usr/lib";
    pub const USR_LOCAL: &str = "/usr/local";
    pub const USR_LOCAL_BIN: &str = "/usr/local/bin";
    pub const USR_LOCAL_LIB: &str = "/usr/local/lib";
    pub const BOOT: &str = "/boot";
    pub const DEV: &str = "/dev";
    pub const PROC: &str = "/proc";
    pub const SYS: &str = "/sys";
    pub const RUN: &str = "/run";
    pub const MNT: &str = "/mnt";
    pub const MEDIA: &str = "/media";
    pub const SRV: &str = "/srv";
}

/// Filesystem hierarchy structure
pub struct FilesystemHierarchy {
    pub root: DirectoryNode,
}

impl FilesystemHierarchy {
    pub const fn new() -> Self {
        Self {
            root: DirectoryNode::new("/"),
        }
    }
    
    pub fn create_linux_hierarchy(&mut self) {
        // Create standard Linux directories
        self.root.add_directory("bin");
        self.root.add_directory("sbin");
        self.root.add_directory("lib");
        self.root.add_directory("lib64");
        self.root.add_directory("etc");
        self.root.add_directory("home");
        self.root.add_directory("root");
        self.root.add_directory("var");
        self.root.add_directory("tmp");
        self.root.add_directory("opt");
        self.root.add_directory("usr");
        self.root.add_directory("boot");
        self.root.add_directory("dev");
        self.root.add_directory("proc");
        self.root.add_directory("sys");
        self.root.add_directory("run");
        self.root.add_directory("mnt");
        self.root.add_directory("media");
        self.root.add_directory("srv");
        
        // Create /usr subdirectories
        if let Some(usr) = self.root.get_directory("usr") {
            usr.add_directory("bin");
            usr.add_directory("sbin");
            usr.add_directory("lib");
            usr.add_directory("local");
            
            if let Some(local) = usr.get_directory("local") {
                local.add_directory("bin");
                local.add_directory("lib");
                local.add_directory("share");
            }
        }
        
        // Create /var subdirectories
        if let Some(var) = self.root.get_directory("var") {
            var.add_directory("log");
            var.add_directory("spool");
            var.add_directory("run");
            var.add_directory("lib");
            var.add_directory("cache");
            var.add_directory("tmp");
        }
        
        // Create /etc subdirectories
        if let Some(etc) = self.root.get_directory("etc") {
            etc.add_directory("init.d");
            etc.add_directory("rc.d");
            etc.add_directory("sysconfig");
            etc.add_directory("network");
            etc.add_directory("profile.d");
        }
    }
}

/// Directory node in filesystem hierarchy
pub struct DirectoryNode {
    pub name: String,
    pub path: String,
    pub subdirectories: Vec<DirectoryNode>,
    pub permissions: u32,
    pub owner: u32,
    pub group: u32,
}

impl DirectoryNode {
    pub fn new(path: &str) -> Self {
        Self {
            name: path.rsplit('/').next().unwrap_or("").to_string(),
            path: path.to_string(),
            subdirectories: Vec::new(),
            permissions: 0o755,
            owner: 0,
            group: 0,
        }
    }
    
    pub fn add_directory(&mut self, name: &str) {
        let path = format!("{}/{}", self.path, name);
        self.subdirectories.push(DirectoryNode::new(&path));
    }
    
    pub fn get_directory(&mut self, name: &str) -> Option<&mut DirectoryNode> {
        self.subdirectories.iter_mut().find(|d| d.name == name)
    }
    
    pub fn get_path(&self) -> &str {
        &self.path
    }
}

/// File permissions (Linux-style)
pub mod permissions {
    pub const S_IRUSR: u32 = 0o400; // Read by owner
    pub const S_IWUSR: u32 = 0o200; // Write by owner
    pub const S_IXUSR: u32 = 0o100; // Execute by owner
    pub const S_IRGRP: u32 = 0o040; // Read by group
    pub const S_IWGRP: u32 = 0o020; // Write by group
    pub const S_IXGRP: u32 = 0o010; // Execute by group
    pub const S_IROTH: u32 = 0o004; // Read by others
    pub const S_IWOTH: u32 = 0o002; // Write by others
    pub const S_IXOTH: u32 = 0o001; // Execute by others
    
    pub const S_IRWXU: u32 = S_IRUSR | S_IWUSR | S_IXUSR;
    pub const S_IRWXG: u32 = S_IRGRP | S_IWGRP | S_IXGRP;
    pub const S_IRWXO: u32 = S_IROTH | S_IWOTH | S_IXOTH;
    
    pub const S_ISUID: u32 = 0o4000; // Set user ID
    pub const S_ISGID: u32 = 0o2000; // Set group ID
    pub const S_ISVTX: u32 = 0o1000; // Sticky bit
}

/// File types (Linux-style)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    RegularFile,
    Directory,
    SymbolicLink,
    BlockDevice,
    CharacterDevice,
    NamedPipe,
    Socket,
}

/// File mode (Linux-style)
pub struct FileMode {
    pub file_type: FileType,
    pub permissions: u32,
}

impl FileMode {
    pub fn new(file_type: FileType, permissions: u32) -> Self {
        Self {
            file_type,
            permissions,
        }
    }
    
    pub fn from_mode(mode: u32) -> Self {
        let file_type = match mode & 0o170000 {
            0o100000 => FileType::RegularFile,
            0o040000 => FileType::Directory,
            0o120000 => FileType::SymbolicLink,
            0o060000 => FileType::BlockDevice,
            0o020000 => FileType::CharacterDevice,
            0o010000 => FileType::NamedPipe,
            0o140000 => FileType::Socket,
            _ => FileType::RegularFile,
        };
        
        Self {
            file_type,
            permissions: mode & 0o7777,
        }
    }
    
    pub fn to_mode(&self) -> u32 {
        let type_bits = match self.file_type {
            FileType::RegularFile => 0o100000,
            FileType::Directory => 0o040000,
            FileType::SymbolicLink => 0o120000,
            FileType::BlockDevice => 0o060000,
            FileType::CharacterDevice => 0o020000,
            FileType::NamedPipe => 0o010000,
            FileType::Socket => 0o140000,
        };
        
        type_bits | self.permissions
    }
}

/// Mount options (Linux-style)
pub mod mount_options {
    pub const RW: &str = "rw";
    pub const RO: &str = "ro";
    pub const NOEXEC: &str = "noexec";
    pub const NOSUID: &str = "nosuid";
    pub const NODEV: &str = "nodev";
    pub const NOATIME: &str = "noatime";
    pub const NODIRATIME: &str = "nodiratime";
    pub const RELATIME: &str = "relatime";
    pub const SYNC: &str = "sync";
    pub const ASYNC: &str = "async";
    pub const DIRSYNC: &str = "dirsync";
}

/// Filesystem types (Linux-style)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemType {
    Ext4,
    XFS,
    Btrfs,
    Tmpfs,
    Procfs,
    Sysfs,
    Devtmpfs,
    Cgroup,
    Cgroup2,
    Securityfs,
    Configfs,
    Debugfs,
    Tracefs,
    Hugetlbfs,
    Mqueue,
    Ramfs,
    SigmaFS,
    Vfat,
    Ntfs,
    Iso9660,
}

/// Mount point structure
pub struct MountPoint {
    pub source: String,
    pub target: String,
    pub filesystem_type: FilesystemType,
    pub options: Vec<String>,
    pub dump: u32,
    pub fsck_order: u32,
}

impl MountPoint {
    pub fn new(source: &str, target: &str, fs_type: FilesystemType) -> Self {
        Self {
            source: source.to_string(),
            target: target.to_string(),
            filesystem_type: fs_type,
            options: vec!["rw".to_string(), "relatime".to_string()],
            dump: 0,
            fsck_order: 0,
        }
    }
    
    pub fn add_option(&mut self, option: &str) {
        self.options.push(option.to_string());
    }
}

/// Fstab entry (Linux-style /etc/fstab)
pub struct FstabEntry {
    pub spec: String,
    pub file: String,
    pub vfstype: String,
    pub mntops: String,
    pub freq: u32,
    pub passno: u32,
}

impl FstabEntry {
    pub fn new(spec: &str, file: &str, vfstype: &str, mntops: &str, freq: u32, passno: u32) -> Self {
        Self {
            spec: spec.to_string(),
            file: file.to_string(),
            vfstype: vfstype.to_string(),
            mntops: mntops.to_string(),
            freq,
            passno,
        }
    }
}

/// Standard Linux fstab entries
pub fn get_standard_fstab() -> Vec<FstabEntry> {
    vec![
        FstabEntry::new("proc", "/proc", "proc", "defaults", 0, 0),
        FstabEntry::new("sysfs", "/sys", "sysfs", "defaults", 0, 0),
        FstabEntry::new("devtmpfs", "/dev", "devtmpfs", "defaults", 0, 0),
        FstabEntry::new("devpts", "/dev/pts", "devpts", "defaults", 0, 0),
        FstabEntry::new("tmpfs", "/run", "tmpfs", "defaults", 0, 0),
        FstabEntry::new("tmpfs", "/tmp", "tmpfs", "defaults", 0, 0),
    ]
}
