// SPDX-License-Identifier: MIT
// SigmaOS Filesystem Manager - Arch Linux Inspired
// Implements filesystem management similar to Arch's mkfs and mount tools

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Filesystem types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemType {
    Ext2,
    Ext3,
    Ext4,
    Xfs,
    Btrfs,
    F2fs,
    Ntfs,
    Vfat,
    Exfat,
    Swap,
    Tmpfs,
    Ramfs,
    Procfs,
    Sysfs,
    Devtmpfs,
    Debugfs,
    Securityfs,
    Pstorefs,
    Efivarfs,
    FUSE,
}

impl FilesystemType {
    pub fn as_str(&self) -> &str {
        match self {
            FilesystemType::Ext2 => "ext2",
            FilesystemType::Ext3 => "ext3",
            FilesystemType::Ext4 => "ext4",
            FilesystemType::Xfs => "xfs",
            FilesystemType::Btrfs => "btrfs",
            FilesystemType::F2fs => "f2fs",
            FilesystemType::Ntfs => "ntfs",
            FilesystemType::Vfat => "vfat",
            FilesystemType::Exfat => "exfat",
            FilesystemType::Swap => "swap",
            FilesystemType::Tmpfs => "tmpfs",
            FilesystemType::Ramfs => "ramfs",
            FilesystemType::Procfs => "proc",
            FilesystemType::Sysfs => "sysfs",
            FilesystemType::Devtmpfs => "devtmpfs",
            FilesystemType::Debugfs => "debugfs",
            FilesystemType::Securityfs => "securityfs",
            FilesystemType::Pstorefs => "pstore",
            FilesystemType::Efivarfs => "efivarfs",
            FilesystemType::FUSE => "fuse",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ext2" => Some(FilesystemType::Ext2),
            "ext3" => Some(FilesystemType::Ext3),
            "ext4" => Some(FilesystemType::Ext4),
            "xfs" => Some(FilesystemType::Xfs),
            "btrfs" => Some(FilesystemType::Btrfs),
            "f2fs" => Some(FilesystemType::F2fs),
            "ntfs" => Some(FilesystemType::Ntfs),
            "vfat" => Some(FilesystemType::Vfat),
            "exfat" => Some(FilesystemType::Exfat),
            "swap" => Some(FilesystemType::Swap),
            "tmpfs" => Some(FilesystemType::Tmpfs),
            "ramfs" => Some(FilesystemType::Ramfs),
            "proc" => Some(FilesystemType::Procfs),
            "sysfs" => Some(FilesystemType::Sysfs),
            "devtmpfs" => Some(FilesystemType::Devtmpfs),
            "debugfs" => Some(FilesystemType::Debugfs),
            "securityfs" => Some(FilesystemType::Securityfs),
            "pstore" => Some(FilesystemType::Pstorefs),
            "efivarfs" => Some(FilesystemType::Efivarfs),
            "fuse" => Some(FilesystemType::FUSE),
            _ => None,
        }
    }

    pub fn is_journaled(&self) -> bool {
        matches!(self, FilesystemType::Ext3 | FilesystemType::Ext4 | FilesystemType::Xfs | FilesystemType::Btrfs)
    }

    pub fn is_robust(&self) -> bool {
        matches!(self, FilesystemType::Ext4 | FilesystemType::Xfs | FilesystemType::Btrfs | FilesystemType::F2fs)
    }

    pub fn is_compressed(&self) -> bool {
        matches!(self, FilesystemType::Btrfs | FilesystemType::F2fs)
    }

    pub fn is_encrypted(&self) -> bool {
        false // Most filesystems don't have built-in encryption
    }

    pub fn supports_snapshots(&self) -> bool {
        matches!(self, FilesystemType::Btrfs | FilesystemType::Xfs)
    }

    pub fn supports_checksums(&self) -> bool {
        matches!(self, FilesystemType::Btrfs | FilesystemType::Xfs)
    }
}

/// Mount options
#[derive(Debug, Clone)]
pub struct MountOptions {
    pub ro: bool,
    pub rw: bool,
    pub noexec: bool,
    pub nosuid: bool,
    pub nodev: bool,
    pub noatime: bool,
    pub relatime: bool,
    pub strictatime: bool,
    pub dirsync: bool,
    pub sync: bool,
    pub async: bool,
    pub auto: bool,
    pub noauto: bool,
    pub user: bool,
    pub nouser: bool,
    pub users: Vec<String>,
    pub owner: bool,
    pub group: Option<String>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    pub umask: Option<u32>,
    pub fmask: Option<u32>,
    pub dmask: Option<u32>,
    pub io: Option<String>,
    pub discard: bool,
    pub nodiscard: bool,
    pub commit: Option<u32>,
    pub data: Option<String>,
    pub barrier: bool,
    pub nobarrier: bool,
    pub errors: Option<String>,
    pub quota: bool,
    pub usrquota: bool,
    pub grpquota: bool,
    pub aquota: bool,
    pub encryption: Option<String>,
    pub keyfile: Option<PathBuf>,
    pub keyslot: Option<String>,
    pub compression: Option<String>,
    pub compression_level: Option<u32>,
    pub subvol: Option<String>,
    pub subvolid: Option<u32>,
    pub snapshot: Option<String>,
    pub label: Option<String>,
    pub uuid: Option<String>,
    pub custom_options: BTreeMap<String, String>,
}

impl MountOptions {
    /// Create new mount options
    pub fn new() -> Self {
        Self {
            ro: false,
            rw: true,
            noexec: false,
            nosuid: false,
            nodev: false,
            noatime: false,
            relatime: true,
            strictatime: false,
            dirsync: false,
            sync: false,
            async: true,
            auto: true,
            noauto: false,
            user: false,
            nouser: true,
            users: Vec::new(),
            owner: false,
            group: None,
            uid: None,
            gid: None,
            umask: None,
            fmask: None,
            dmask: None,
            io: None,
            discard: false,
            nodiscard: false,
            commit: None,
            data: None,
            barrier: true,
            nobarrier: false,
            errors: None,
            quota: false,
            usrquota: false,
            grpquota: false,
            aquota: false,
            encryption: None,
            keyfile: None,
            keyslot: None,
            compression: None,
            compression_level: None,
            subvol: None,
            subvolid: None,
            snapshot: None,
            label: None,
            uuid: None,
            custom_options: BTreeMap::new(),
        }
    }

    /// Set read-only
    pub fn ro(mut self) -> Self {
        self.ro = true;
        self.rw = false;
        self
    }

    /// Set read-write
    pub fn rw(mut self) -> Self {
        self.rw = true;
        self.ro = false;
        self
    }

    /// Set noexec
    pub fn noexec(mut self) -> Self {
        self.noexec = true;
        self
    }

    /// Set nosuid
    pub fn nosuid(mut self) -> Self {
        self.nosuid = true;
        self
    }

    /// Set nodev
    pub fn nodev(mut self) -> Self {
        self.nodev = true;
        self
    }

    /// Set noatime
    pub fn noatime(mut self) -> Self {
        self.noatime = true;
        self.relatime = false;
        self.strictatime = false;
        self
    }

    /// Set relatime
    pub fn relatime(mut self) -> Self {
        self.relatime = true;
        self.noatime = false;
        self.strictatime = false;
        self
    }

    /// Set strictatime
    pub fn strictatime(mut self) -> Self {
        self.strictatime = true;
        self.noatime = false;
        self.relatime = false;
        self
    }

    /// Set dirsync
    pub fn dirsync(mut self) -> Self {
        self.dirsync = true;
        self
    }

    /// Set sync
    pub fn sync(mut self) -> Self {
        self.sync = true;
        self.async = false;
        self
    }

    /// Set async
    pub fn async_(mut self) -> Self {
        self.async = true;
        self.sync = false;
        self
    }

    /// Set noauto
    pub fn noauto(mut self) -> Self {
        self.noauto = true;
        self.auto = false;
        self
    }

    /// Set auto
    pub fn auto(mut self) -> Self {
        self.auto = true;
        self.noauto = false;
        self
    }

    /// Set user
    pub fn user(mut self) -> Self {
        self.user = true;
        self.nouser = false;
        self
    }

    /// Set nouser
    pub fn nouser(mut self) -> Self {
        self.nouser = true;
        self.user = false;
        self
    }

    /// Add user
    pub fn add_user(mut self, user: impl Into<String>) -> Self {
        self.users.push(user.into());
        self
    }

    /// Set owner
    pub fn owner(mut self) -> Self {
        self.owner = true;
        self
    }

    /// Set group
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Set uid
    pub fn uid(mut self, uid: u32) -> Self {
        self.uid = Some(uid);
        self
    }

    /// Set gid
    pub fn gid(mut self, gid: u32) -> Self {
        self.gid = Some(gid);
        self
    }

    /// Set umask
    pub fn umask(mut self, mask: u32) -> Self {
        self.umask = Some(mask);
        self
    }

    /// Set fmask
    pub fn fmask(mut self, mask: u32) -> Self {
        self.fmask = Some(mask);
        self
    }

    /// Set dmask
    pub fn dmask(mut self, mask: u32) -> Self {
        self.dmask = Some(mask);
        self
    }

    /// Set io
    pub fn io(mut self, io: impl Into<String>) -> Self {
        self.io = Some(io.into());
        self
    }

    /// Set discard
    pub fn discard(mut self) -> Self {
        self.discard = true;
        self.nodiscard = false;
        self
    }

    /// Set nodiscard
    pub fn nodiscard(mut self) -> Self {
        self.nodiscard = true;
        self.discard = false;
        self
    }

    /// Set commit
    pub fn commit(mut self, commit: u32) -> Self {
        self.commit = Some(commit);
        self
    }

    /// Set data mode
    pub fn data(mut self, mode: impl Into<String>) -> Self {
        self.data = Some(mode.into());
        self
    }

    /// Set barrier
    pub fn barrier(mut self) -> Self {
        self.barrier = true;
        self.nobarrier = false;
        self
    }

    /// Set nobarrier
    pub fn nobarrier(mut self) -> Self {
        self.nobarrier = true;
        self.barrier = false;
        self
    }

    /// Set errors mode
    pub fn errors(mut self, mode: impl Into<String>) -> Self {
        self.errors = Some(mode.into());
        self
    }

    /// Enable quotas
    pub fn quota(mut self) -> Self {
        self.quota = true;
        self
    }

    /// Enable user quotas
    pub fn usrquota(mut self) -> Self {
        self.usrquota = true;
        self.quota = true;
        self
    }

    /// Enable group quotas
    pub fn grpquota(mut self) -> Self {
        self.grpquota = true;
        self.quota = true;
        self
    }

    /// Enable aquota
    pub fn aquota(mut self) -> Self {
        self.aquota = true;
        self.quota = true;
        self
    }

    /// Set encryption
    pub fn encryption(mut self, encryption: impl Into<String>) -> Self {
        self.encryption = Some(encryption.into());
        self
    }

    /// Set keyfile
    pub fn keyfile(mut self, keyfile: impl Into<PathBuf>) -> Self {
        self.keyfile = Some(keyfile.into());
        self
    }

    /// Set keyslot
    pub fn keyslot(mut self, keyslot: impl Into<String>) -> Self {
        self.keyslot = Some(keyslot.into());
        self
    }

    /// Set compression
    pub fn compression(mut self, compression: impl Into<String>) -> Self {
        self.compression = Some(compression.into());
        self
    }

    /// Set compression level
    pub fn compression_level(mut self, level: u32) -> Self {
        self.compression_level = Some(level);
        self
    }

    /// Set subvolume
    pub fn subvol(mut self, subvol: impl Into<String>) -> Self {
        self.subvol = Some(subvol.into());
        self
    }

    /// Set subvolume ID
    pub fn subvolid(mut self, id: u32) -> Self {
        self.subvolid = Some(id);
        self
    }

    /// Set snapshot
    pub fn snapshot(mut self, snapshot: impl Into<String>) -> Self {
        self.snapshot = Some(snapshot.into());
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set UUID
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    /// Set custom option
    pub fn custom_option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_options.insert(key.into(), value.into());
        self
    }

    /// Convert to mount command string
    pub fn to_mount_string(&self) -> String {
        let mut options = Vec::new();

        if self.ro { options.push("ro".to_string()); }
        if self.rw { options.push("rw".to_string()); }
        if self.noexec { options.push("noexec".to_string()); }
        if self.nosuid { options.push("nosuid".to_string()); }
        if self.nodev { options.push("nodev".to_string()); }
        if self.noatime { options.push("noatime".to_string()); }
        if self.relatime { options.push("relatime".to_string()); }
        if self.strictatime { options.push("strictatime".to_string()); }
        if self.dirsync { options.push("dirsync".to_string()); }
        if self.sync { options.push("sync".to_string()); }
        if self.async { options.push("async".to_string()); }
        if self.auto { options.push("auto".to_string()); }
        if self.noauto { options.push("noauto".to_string()); }
        if self.user { options.push("user".to_string()); }
        if self.nouser { options.push("nouser".to_string()); }
        if !self.users.is_empty() { options.push(format!("users={}", self.users.join(","))); }
        if self.owner { options.push("owner".to_string()); }
        if let Some(group) = &self.group { options.push(format!("group={}", group)); }
        if let Some(uid) = self.uid { options.push(format!("uid={}", uid)); }
        if let Some(gid) = self.gid { options.push(format!("gid={}", gid)); }
        if let Some(umask) = self.umask { options.push(format!("umask={:o}", umask)); }
        if let Some(fmask) = self.fmask { options.push(format!("fmask={:o}", fmask)); }
        if let Some(dmask) = self.dmask { options.push(format!("dmask={:o}", dmask)); }
        if let Some(io) = &self.io { options.push(format!("io={}", io)); }
        if self.discard { options.push("discard".to_string()); }
        if self.nodiscard { options.push("nodiscard".to_string()); }
        if let Some(commit) = self.commit { options.push(format!("commit={}", commit)); }
        if let Some(data) = &self.data { options.push(format!("data={}", data)); }
        if self.barrier { options.push("barrier".to_string()); }
        if self.nobarrier { options.push("nobarrier".to_string()); }
        if let Some(errors) = &self.errors { options.push(format!("errors={}", errors)); }
        if self.quota { options.push("quota".to_string()); }
        if self.usrquota { options.push("usrquota".to_string()); }
        if self.grpquota { options.push("grpquota".to_string()); }
        if self.aquota { options.push("aquota".to_string()); }
        if let Some(encryption) = &self.encryption { options.push(format!("encryption={}", encryption)); }
        if let Some(keyfile) = &self.keyfile { options.push(format!("keyfile={}", keyfile.display())); }
        if let Some(keyslot) = &self.keyslot { options.push(format!("keyslot={}", keyslot)); }
        if let Some(compression) = &self.compression { options.push(format!("compress={}", compression)); }
        if let Some(level) = self.compression_level { options.push(format!("compress-level={}", level)); }
        if let Some(subvol) = &self.subvol { options.push(format!("subvol={}", subvol)); }
        if let Some(subvolid) = self.subvolid { options.push(format!("subvolid={}", subvolid)); }
        if let Some(snapshot) = &self.snapshot { options.push(format!("snapshot={}", snapshot)); }
        if let Some(label) = &self.label { options.push(format!("label={}", label)); }
        if let Some(uuid) = &self.uuid { options.push(format!("uuid={}", uuid)); }

        for (key, value) in &self.custom_options {
            options.push(format!("{}={}", key, value));
        }

        options.join(",")
    }
}

impl Default for MountOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Mount entry
#[derive(Debug, Clone, PartialEq)]
pub struct MountEntry {
    pub device: String,
    pub mount_point: PathBuf,
    pub filesystem: FilesystemType,
    pub options: MountOptions,
    pub dump: u32,
    pub pass: u32,
    pub uuid: Option<String>,
    pub label: Option<String>,
    pub is_swap: bool,
    pub is_bind: bool,
    pub is_tmpfs: bool,
    pub is_virtual: bool,
}

impl MountEntry {
    /// Create a new mount entry
    pub fn new(device: impl Into<String>, mount_point: impl Into<PathBuf>, filesystem: FilesystemType) -> Self {
        Self {
            device: device.into(),
            mount_point: mount_point.into(),
            filesystem,
            options: MountOptions::new(),
            dump: 0,
            pass: 0,
            uuid: None,
            label: None,
            is_swap: false,
            is_bind: false,
            is_tmpfs: false,
            is_virtual: false,
        }
    }

    /// Set options
    pub fn options(mut self, options: MountOptions) -> Self {
        self.options = options;
        self
    }

    /// Set dump
    pub fn dump(mut self, dump: u32) -> Self {
        self.dump = dump;
        self
    }

    /// Set pass
    pub fn pass(mut self, pass: u32) -> Self {
        self.pass = pass;
        self
    }

    /// Set UUID
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Mark as swap
    pub fn swap(mut self) -> Self {
        self.is_swap = true;
        self.filesystem = FilesystemType::Swap;
        self
    }

    /// Mark as bind mount
    pub fn bind(mut self) -> Self {
        self.is_bind = true;
        self
    }

    /// Mark as tmpfs
    pub fn tmpfs(mut self) -> Self {
        self.is_tmpfs = true;
        self.filesystem = FilesystemType::Tmpfs;
        self
    }

    /// Mark as virtual
    pub fn virtual_(mut self) -> Self {
        self.is_virtual = true;
        self
    }

    /// Convert to fstab line
    pub fn to_fstab_line(&self) -> String {
        let device = if let Some(uuid) = &self.uuid {
            format!("UUID={}", uuid)
        } else if let Some(label) = &self.label {
            format!("LABEL={}", label)
        } else {
            self.device.clone()
        };

        let mount_point = self.mount_point.to_string_lossy();
        let fs_type = self.filesystem.as_str();
        let options = self.options.to_mount_string();

        format!("{} {} {} {} {} {}", device, mount_point, fs_type, options, self.dump, self.pass)
    }

    /// Convert to mount command
    pub fn to_mount_command(&self) -> String {
        let options = self.options.to_mount_string();
        format!("mount -t {} -o {} {} {}", self.filesystem.as_str(), options, self.device, self.mount_point.display())
    }
}

/// Filesystem manager
#[derive(Debug, Clone)]
pub struct FilesystemManager {
    pub fstab_path: PathBuf,
    pub mounts: Vec<MountEntry>,
    pub mounted_filesystems: Vec<MountEntry>,
    pub available_filesystems: Vec<FilesystemType>,
    pub default_mount_options: MountOptions,
}

impl FilesystemManager {
    /// Create a new filesystem manager
    pub fn new() -> Self {
        Self {
            fstab_path: PathBuf::from("/etc/fstab"),
            mounts: Vec::new(),
            mounted_filesystems: Vec::new(),
            available_filesystems: vec![
                FilesystemType::Ext4,
                FilesystemType::Xfs,
                FilesystemType::Btrfs,
                FilesystemType::F2fs,
                FilesystemType::Ntfs,
                FilesystemType::Vfat,
                FilesystemType::Exfat,
            ],
            default_mount_options: MountOptions::new(),
        }
    }

    /// Create filesystem manager with custom fstab path
    pub fn with_fstab(path: impl Into<PathBuf>) -> Self {
        Self {
            fstab_path: path.into(),
            mounts: Vec::new(),
            mounted_filesystems: Vec::new(),
            available_filesystems: vec![
                FilesystemType::Ext4,
                FilesystemType::Xfs,
                FilesystemType::Btrfs,
                FilesystemType::F2fs,
                FilesystemType::Ntfs,
                FilesystemType::Vfat,
                FilesystemType::Exfat,
            ],
            default_mount_options: MountOptions::new(),
        }
    }

    /// Load fstab
    pub fn load_fstab(&mut self) -> io::Result<()> {
        if !self.fstab_path.exists() {
            return Ok(());
        }

        let file = fs::File::open(&self.fstab_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(entry) = self.parse_fstab_line(line) {
                self.mounts.push(entry);
            }
        }

        Ok(())
    }

    /// Parse fstab line
    pub fn parse_fstab_line(&self, line: &str) -> Option<MountEntry> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            return None;
        }

        let device = parts[0].to_string();
        let mount_point = PathBuf::from(parts[1]);
        let fs_type = FilesystemType::from_str(parts[2])?;
        let options_str = parts[3];
        let dump = parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);
        let pass = parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(0);

        // Parse UUID/LABEL
        let mut uuid = None;
        let mut label = None;

        if device.starts_with("UUID=") {
            uuid = Some(device[5..].to_string());
        } else if device.starts_with("LABEL=") {
            label = Some(device[6..].to_string());
        }

        // Parse options
        let options = self.parse_mount_options(options_str);

        let mut entry = MountEntry::new(device, mount_point, fs_type)
            .options(options)
            .dump(dump)
            .pass(pass);

        if let Some(uuid) = uuid {
            entry = entry.uuid(uuid);
        }
        if let Some(label) = label {
            entry = entry.label(label);
        }

        Some(entry)
    }

    /// Parse mount options
    pub fn parse_mount_options(&self, options_str: &str) -> MountOptions {
        let mut options = MountOptions::new();

        for opt in options_str.split(',') {
            let opt = opt.trim();
            match opt {
                "ro" => options = options.ro(),
                "rw" => options = options.rw(),
                "noexec" => options = options.noexec(),
                "nosuid" => options = options.nosuid(),
                "nodev" => options = options.nodev(),
                "noatime" => options = options.noatime(),
                "relatime" => options = options.relatime(),
                "strictatime" => options = options.strictatime(),
                "dirsync" => options = options.dirsync(),
                "sync" => options = options.sync(),
                "async" => options = options.async_(),
                "auto" => options = options.auto(),
                "noauto" => options = options.noauto(),
                "user" => options = options.user(),
                "nouser" => options = options.nouser(),
                "owner" => options = options.owner(),
                "discard" => options = options.discard(),
                "nodiscard" => options = options.nodiscard(),
                "barrier" => options = options.barrier(),
                "nobarrier" => options = options.nobarrier(),
                "quota" => options = options.quota(),
                "usrquota" => options = options.usrquota(),
                "grpquota" => options = options.grpquota(),
                "aquota" => options = options.aquota(),
                _ => {
                    if let Some((key, value)) = opt.split_once('=') {
                        options = options.custom_option(key.to_string(), value.to_string());
                    }
                }
            }
        }

        options
    }

    /// Save fstab
    pub fn save_fstab(&self) -> io::Result<()> {
        let mut content = String::new();
        content.push_str("# SigmaOS /etc/fstab\n");
        content.push_str("# Generated by SigmaOS Filesystem Manager\n\n");

        for mount in &self.mounts {
            content.push_str(&mount.to_fstab_line());
            content.push('\n');
        }

        fs::write(&self.fstab_path, content)
    }

    /// Add mount entry
    pub fn add_mount(&mut self, entry: MountEntry) {
        self.mounts.push(entry);
    }

    /// Remove mount entry
    pub fn remove_mount(&mut self, mount_point: &Path) -> bool {
        if let Some(pos) = self.mounts.iter().position(|m| m.mount_point == mount_point) {
            self.mounts.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get mount entry by mount point
    pub fn get_mount(&self, mount_point: &Path) -> Option<&MountEntry> {
        self.mounts.iter().find(|m| m.mount_point == mount_point)
    }

    /// Get mount entry by device
    pub fn get_mount_by_device(&self, device: &str) -> Option<&MountEntry> {
        self.mounts.iter().find(|m| m.device == device)
    }

    /// Get mount entry by UUID
    pub fn get_mount_by_uuid(&self, uuid: &str) -> Option<&MountEntry> {
        self.mounts.iter().find(|m| m.uuid.as_deref() == Some(uuid))
    }

    /// Get mount entry by label
    pub fn get_mount_by_label(&self, label: &str) -> Option<&MountEntry> {
        self.mounts.iter().find(|m| m.label.as_deref() == Some(label))
    }

    /// Mount filesystem
    pub fn mount(&mut self, entry: &MountEntry) -> Result<(), String> {
        // Create mount point if it doesn't exist
        if !entry.mount_point.exists() {
            fs::create_dir_all(&entry.mount_point)
                .map_err(|e| format!("Failed to create mount point: {}", e))?;
        }

        // Execute mount command
        let mount_cmd = entry.to_mount_command();
        let output = Command::new("sh")
            .arg("-c")
            .arg(&mount_cmd)
            .output()
            .map_err(|e| format!("Failed to execute mount: {}", e))?;

        if !output.status.success() {
            return Err(format!("Mount failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        // Add to mounted filesystems
        self.mounted_filesystems.push(entry.clone());

        Ok(())
    }

    /// Unmount filesystem
    pub fn unmount(&mut self, mount_point: &Path) -> Result<(), String> {
        let mount_point_str = mount_point.to_string_lossy();

        let output = Command::new("umount")
            .arg(&*mount_point_str)
            .output()
            .map_err(|e| format!("Failed to execute umount: {}", e))?;

        if !output.status.success() {
            return Err(format!("Unmount failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        // Remove from mounted filesystems
        if let Some(pos) = self.mounted_filesystems.iter().position(|m| m.mount_point == *mount_point) {
            self.mounted_filesystems.remove(pos);
        }

        Ok(())
    }

    /// Mount all filesystems from fstab
    pub fn mount_all(&mut self) -> Result<Vec<String>, String> {
        let mut mounted = Vec::new();
        let mut errors = Vec::new();

        for entry in &self.mounts {
            if !entry.options.auto {
                continue;
            }

            if entry.is_swap {
                // Handle swap
                if let Err(e) = self.enable_swap(entry) {
                    errors.push(format!("Failed to enable swap {}: {}", entry.device, e));
                } else {
                    mounted.push(entry.device.clone());
                }
            } else if entry.is_virtual {
                // Handle virtual filesystems
                if let Err(e) = self.mount_virtual(entry) {
                    errors.push(format!("Failed to mount virtual {}: {}", entry.mount_point.display(), e));
                } else {
                    mounted.push(entry.mount_point.to_string_lossy().to_string());
                }
            } else {
                // Regular mount
                if let Err(e) = self.mount(entry) {
                    errors.push(format!("Failed to mount {}: {}", entry.device, e));
                } else {
                    mounted.push(entry.device.clone());
                }
            }
        }

        if errors.is_empty() {
            Ok(mounted)
        } else {
            Err(errors.join("; "))
        }
    }

    /// Unmount all filesystems
    pub fn unmount_all(&mut self) -> Result<Vec<String>, String> {
        let mut unmounted = Vec::new();
        let mut errors = Vec::new();

        // Unmount in reverse order
        for entry in self.mounted_filesystems.iter().rev() {
            if entry.is_virtual {
                continue; // Skip virtual filesystems
            }

            if let Err(e) = self.unmount(&entry.mount_point) {
                errors.push(format!("Failed to unmount {}: {}", entry.mount_point.display(), e));
            } else {
                unmounted.push(entry.mount_point.to_string_lossy().to_string());
            }
        }

        if errors.is_empty() {
            Ok(unmounted)
        } else {
            Err(errors.join("; "))
        }
    }

    /// Enable swap
    pub fn enable_swap(&mut self, entry: &MountEntry) -> Result<(), String> {
        let output = Command::new("swapon")
            .arg(&entry.device)
            .output()
            .map_err(|e| format!("Failed to enable swap: {}", e))?;

        if !output.status.success() {
            return Err(format!("swapon failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        self.mounted_filesystems.push(entry.clone());
        Ok(())
    }

    /// Disable swap
    pub fn disable_swap(&mut self, device: &str) -> Result<(), String> {
        let output = Command::new("swapoff")
            .arg(device)
            .output()
            .map_err(|e| format!("Failed to disable swap: {}", e))?;

        if !output.status.success() {
            return Err(format!("swapoff failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        // Remove from mounted filesystems
        if let Some(pos) = self.mounted_filesystems.iter().position(|m| m.device == device && m.is_swap) {
            self.mounted_filesystems.remove(pos);
        }

        Ok(())
    }

    /// Mount virtual filesystem
    pub fn mount_virtual(&mut self, entry: &MountEntry) -> Result<(), String> {
        let fs_type = entry.filesystem.as_str();
        let mount_point = entry.mount_point.to_string_lossy();

        let output = Command::new("mount")
            .arg("-t")
            .arg(fs_type)
            .arg(&mount_point)
            .output()
            .map_err(|e| format!("Failed to mount virtual filesystem: {}", e))?;

        if !output.status.success() {
            return Err(format!("Virtual mount failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        self.mounted_filesystems.push(entry.clone());
        Ok(())
    }

    /// Create filesystem
    pub fn create_filesystem(&self, device: &str, fs_type: FilesystemType, label: Option<&str>) -> Result<(), String> {
        let mut cmd = vec![self.get_mkfs_command(fs_type), device.to_string()];

        if let Some(label) = label {
            cmd.push("-L".to_string());
            cmd.push(label.to_string());
        }

        let output = Command::new(&cmd[0])
            .args(&cmd[1..])
            .output()
            .map_err(|e| format!("Failed to create filesystem: {}", e))?;

        if !output.status.success() {
            return Err(format!("mkfs failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        Ok(())
    }

    /// Get mkfs command for filesystem type
    fn get_mkfs_command(&self, fs_type: FilesystemType) -> String {
        match fs_type {
            FilesystemType::Ext2 => "mkfs.ext2".to_string(),
            FilesystemType::Ext3 => "mkfs.ext3".to_string(),
            FilesystemType::Ext4 => "mkfs.ext4".to_string(),
            FilesystemType::Xfs => "mkfs.xfs".to_string(),
            FilesystemType::Btrfs => "mkfs.btrfs".to_string(),
            FilesystemType::F2fs => "mkfs.f2fs".to_string(),
            FilesystemType::Ntfs => "mkfs.ntfs".to_string(),
            FilesystemType::Vfat => "mkfs.vfat".to_string(),
            FilesystemType::Exfat => "mkfs.exfat".to_string(),
            FilesystemType::Swap => "mkswap".to_string(),
            _ => "mkfs".to_string(),
        }
    }

    /// Check filesystem
    pub fn check_filesystem(&self, device: &str, fs_type: FilesystemType) -> Result<(), String> {
        let fsck_cmd = match fs_type {
            FilesystemType::Ext2 | FilesystemType::Ext3 | FilesystemType::Ext4 => "fsck.ext4",
            FilesystemType::Xfs => "fsck.xfs",
            FilesystemType::Btrfs => "btrfs-check",
            FilesystemType::F2fs => "fsck.f2fs",
            FilesystemType::Ntfs => "ntfsfix",
            FilesystemType::Vfat => "fsck.vfat",
            FilesystemType::Exfat => "fsck.exfat",
            FilesystemType::Swap => "fsck.swap",
            _ => "fsck",
        };

        let output = Command::new(fsck_cmd)
            .arg("-f")
            .arg(device)
            .output()
            .map_err(|e| format!("Failed to check filesystem: {}", e))?;

        if !output.status.success() {
            return Err(format!("fsck failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        Ok(())
    }

    /// Resize filesystem
    pub fn resize_filesystem(&self, device: &str, fs_type: FilesystemType, size: Option<&str>) -> Result<(), String> {
        let resize_cmd = match fs_type {
            FilesystemType::Ext2 | FilesystemType::Ext3 | FilesystemType::Ext4 => "resize2fs",
            FilesystemType::Xfs => "xfs_growfs",
            FilesystemType::Btrfs => "btrfs filesystem resize",
            FilesystemType::F2fs => "resize.f2fs",
            FilesystemType::Vfat => "fatresize",
            FilesystemType::Exfat => "exfatprogs",
            _ => return Err(format!("Resize not supported for {}", fs_type.as_str())),
        };

        let mut cmd = vec![resize_cmd.to_string(), device.to_string()];
        if let Some(size) = size {
            cmd.push(size.to_string());
        }

        let output = Command::new(&cmd[0])
            .args(&cmd[1..])
            .output()
            .map_err(|e| format!("Failed to resize filesystem: {}", e))?;

        if !output.status.success() {
            return Err(format!("Resize failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        Ok(())
    }

    /// Get filesystem usage
    pub fn get_filesystem_usage(&self, mount_point: &Path) -> Result<FilesystemUsage, String> {
        let mount_point_str = mount_point.to_string_lossy();

        let output = Command::new("df")
            .arg("-h")
            .arg("--output=source,size,used,avail,pcent,target")
            .arg(&*mount_point_str)
            .output()
            .map_err(|e| format!("Failed to get filesystem usage: {}", e))?;

        if !output.status.success() {
            return Err(format!("df failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();

        if lines.len() < 2 {
            return Err("Invalid df output".to_string());
        }

        let parts: Vec<&str> = lines[1].split_whitespace().collect();
        if parts.len() < 6 {
            return Err("Invalid df output format".to_string());
        }

        let usage = FilesystemUsage {
            source: parts[0].to_string(),
            size: parts[1].to_string(),
            used: parts[2].to_string(),
            available: parts[3].to_string(),
            percent: parts[4].to_string(),
            mount_point: parts[5].to_string(),
        };

        Ok(usage)
    }

    /// Get mounted filesystems
    pub fn get_mounted_filesystems(&self) -> &[MountEntry] {
        &self.mounted_filesystems
    }

    /// Get all mounts from fstab
    pub fn get_all_mounts(&self) -> &[MountEntry] {
        &self.mounts
    }

    /// Check if filesystem is mounted
    pub fn is_mounted(&self, mount_point: &Path) -> bool {
        self.mounted_filesystems.iter().any(|m| m.mount_point == *mount_point)
    }

    /// Check if device is mounted
    pub fn is_device_mounted(&self, device: &str) -> bool {
        self.mounted_filesystems.iter().any(|m| m.device == device)
    }

    /// Get filesystem type for device
    pub fn get_filesystem_type(&self, device: &str) -> Option<FilesystemType> {
        self.mounts.iter()
            .find(|m| m.device == device)
            .map(|m| m.filesystem)
    }

    /// Add default virtual filesystems
    pub fn add_virtual_filesystems(&mut self) {
        let virtual_mounts = vec![
            MountEntry::new("proc", "/proc", FilesystemType::Procfs).virtual_(),
            MountEntry::new("sysfs", "/sys", FilesystemType::Sysfs).virtual_(),
            MountEntry::new("devtmpfs", "/dev", FilesystemType::Devtmpfs).virtual_(),
            MountEntry::new("tmpfs", "/run", FilesystemType::Tmpfs).tmpfs().virtual_(),
            MountEntry::new("tmpfs", "/tmp", FilesystemType::Tmpfs).tmpfs().virtual_(),
        ];

        for mount in virtual_mounts {
            self.mounts.push(mount);
        }
    }

    /// Export fstab configuration
    pub fn export_fstab(&self) -> String {
        let mut content = String::new();
        content.push_str("# SigmaOS /etc/fstab\n");
        content.push_str("# Generated by SigmaOS Filesystem Manager\n\n");

        for mount in &self.mounts {
            content.push_str(&mount.to_fstab_line());
            content.push('\n');
        }

        content
    }

    /// Import fstab configuration
    pub fn import_fstab(&mut self, content: &str) -> Result<(), String> {
        self.mounts.clear();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(entry) = self.parse_fstab_line(line) {
                self.mounts.push(entry);
            }
        }

        Ok(())
    }
}

impl Default for FilesystemManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Filesystem usage information
#[derive(Debug, Clone)]
pub struct FilesystemUsage {
    pub source: String,
    pub size: String,
    pub used: String,
    pub available: String,
    pub percent: String,
    pub mount_point: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filesystem_type() {
        assert_eq!(FilesystemType::Ext4.as_str(), "ext4");
        assert_eq!(FilesystemType::from_str("xfs"), Some(FilesystemType::Xfs));
        assert!(FilesystemType::Ext4.is_journaled());
        assert!(FilesystemType::Btrfs.is_compressed());
    }

    #[test]
    fn test_mount_options() {
        let options = MountOptions::new()
            .rw()
            .noexec()
            .nosuid()
            .nodev()
            .relatime()
            .discard()
            .commit(60)
            .label("test".to_string());

        let options_str = options.to_mount_string();
        assert!(options_str.contains("rw"));
        assert!(options_str.contains("noexec"));
        assert!(options_str.contains("nosuid"));
        assert!(options_str.contains("nodev"));
        assert!(options_str.contains("relatime"));
        assert!(options_str.contains("discard"));
        assert!(options_str.contains("commit=60"));
        assert!(options_str.contains("label=test"));
    }

    #[test]
    fn test_mount_entry() {
        let entry = MountEntry::new("/dev/sda1", "/", FilesystemType::Ext4)
            .uuid("12345678-1234-1234-1234-123456789012".to_string())
            .options(MountOptions::new().rw().relatime().discard())
            .dump(0)
            .pass(1);

        let fstab_line = entry.to_fstab_line();
        assert!(fstab_line.contains("UUID=12345678-1234-1234-1234-123456789012"));
        assert!(fstab_line.contains("/dev/sda1"));
        assert!(fstab_line.contains("ext4"));
        assert!(fstab_line.contains("rw,relatime,discard"));
    }

    #[test]
    fn test_filesystem_manager() {
        let mut manager = FilesystemManager::new();

        let entry = MountEntry::new("/dev/sda1", "/", FilesystemType::Ext4)
            .uuid("12345678-1234-1234-1234-123456789012".to_string())
            .options(MountOptions::new().rw().relatime());

        manager.add_mount(entry);
        assert_eq!(manager.get_all_mounts().len(), 1);

        let mount = manager.get_mount(&PathBuf::from("/"));
        assert!(mount.is_some());
        assert_eq!(mount.unwrap().device, "/dev/sda1");
    }

    #[test]
    fn test_fstab_parsing() {
        let manager = FilesystemManager::new();

        let fstab_line = "UUID=12345678-1234-1234-1234-123456789012 / ext4 rw,relatime,discard 0 1";
        let entry = manager.parse_fstab_line(fstab_line).unwrap();

        assert_eq!(entry.device, "UUID=12345678-1234-1234-1234-123456789012");
        assert_eq!(entry.mount_point, PathBuf::from("/"));
        assert_eq!(entry.filesystem, FilesystemType::Ext4);
        assert_eq!(entry.uuid, Some("12345678-1234-1234-1234-123456789012".to_string()));
    }

    #[test]
    fn test_fstab_export() {
        let mut manager = FilesystemManager::new();

        let entry = MountEntry::new("/dev/sda1", "/", FilesystemType::Ext4)
            .uuid("12345678-1234-1234-1234-123456789012".to_string())
            .options(MountOptions::new().rw().relatime());

        manager.add_mount(entry);
        let fstab = manager.export_fstab();

        assert!(fstab.contains("UUID=12345678-1234-1234-1234-123456789012"));
        assert!(fstab.contains("/dev/sda1"));
        assert!(fstab.contains("ext4"));
    }

    #[test]
    fn test_filesystem_usage() {
        let usage = FilesystemUsage {
            source: "/dev/sda1".to_string(),
            size: "100G".to_string(),
            used: "50G".to_string(),
            available: "50G".to_string(),
            percent: "50%".to_string(),
            mount_point: "/".to_string(),
        };

        assert_eq!(usage.source, "/dev/sda1");
        assert_eq!(usage.percent, "50%");
    }
}
