// Fstab Configuration Manager for SigmaOS
// /etc/fstab configuration per Wiki 05-Filesystems.md
// Provides filesystem mount table management

use std::string::{String, ToString};

/// Filesystem type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsType {
    Ext4,
    Btrfs,
    Zfs,
    Xfs,
    Proc,
    Sysfs,
    Tmpfs,
    Devtmpfs,
    Auto,
}

impl FsType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "ext4" => FsType::Ext4,
            "btrfs" => FsType::Btrfs,
            "zfs" => FsType::Zfs,
            "xfs" => FsType::Xfs,
            "proc" => FsType::Proc,
            "sysfs" => FsType::Sysfs,
            "tmpfs" => FsType::Tmpfs,
            "devtmpfs" => FsType::Devtmpfs,
            "auto" => FsType::Auto,
            _ => FsType::Auto,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            FsType::Ext4 => "ext4",
            FsType::Btrfs => "btrfs",
            FsType::Zfs => "zfs",
            FsType::Xfs => "xfs",
            FsType::Proc => "proc",
            FsType::Sysfs => "sysfs",
            FsType::Tmpfs => "tmpfs",
            FsType::Devtmpfs => "devtmpfs",
            FsType::Auto => "auto",
        }
    }
}

/// Mount option
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MountOption {
    Defaults,
    Noatime,
    Nodiratime,
    Relatime,
    Strictatime,
    Nosuid,
    Nodev,
    Noexec,
    Ssd,
    Compress,
    Async,
    Sync,
    Rw,
    Ro,
    Mode,
    Custom(String),
}

impl MountOption {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "defaults" => MountOption::Defaults,
            "noatime" => MountOption::Noatime,
            "nodiratime" => MountOption::Nodiratime,
            "relatime" => MountOption::Relatime,
            "strictatime" => MountOption::Strictatime,
            "nosuid" => MountOption::Nosuid,
            "nodev" => MountOption::Nodev,
            "noexec" => MountOption::Noexec,
            "ssd" => MountOption::Ssd,
            "compress" => MountOption::Compress,
            "async" => MountOption::Async,
            "sync" => MountOption::Sync,
            "rw" => MountOption::Rw,
            "ro" => MountOption::Ro,
            "mode" => MountOption::Mode,
            _ => MountOption::Custom(String::from(s)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            MountOption::Defaults => "defaults",
            MountOption::Noatime => "noatime",
            MountOption::Nodiratime => "nodiratime",
            MountOption::Relatime => "relatime",
            MountOption::Strictatime => "strictatime",
            MountOption::Nosuid => "nosuid",
            MountOption::Nodev => "nodev",
            MountOption::Noexec => "noexec",
            MountOption::Ssd => "ssd",
            MountOption::Compress => "compress",
            MountOption::Async => "async",
            MountOption::Sync => "sync",
            MountOption::Rw => "rw",
            MountOption::Ro => "ro",
            MountOption::Mode => "mode",
            MountOption::Custom(s) => s,
        }
    }
}

/// Fstab entry
#[derive(Debug, Clone)]
pub struct FstabEntry {
    pub device: String,
    pub mount_point: String,
    pub fs_type: FsType,
    pub options: Vec<MountOption>,
    pub dump: u8,
    pub fsck_order: u8,
}

impl FstabEntry {
    pub fn new(device: String, mount_point: String, fs_type: FsType) -> Self {
        FstabEntry {
            device,
            mount_point,
            fs_type,
            options: vec![MountOption::Defaults],
            dump: 0,
            fsck_order: 0,
        }
    }

    pub fn with_options(mut self, options: Vec<MountOption>) -> Self {
        self.options = options;
        self
    }

    pub fn with_dump(mut self, dump: u8) -> Self {
        self.dump = dump;
        self
    }

    pub fn with_fsck_order(mut self, order: u8) -> Self {
        self.fsck_order = order;
        self
    }

    pub fn to_fstab_line(&self) -> String {
        let options_str: String = self.options.iter()
            .map(|o| o.as_str())
            .collect::<Vec<_>>()
            .join(",");

        format!(
            "{} {} {} {} {} {}",
            self.device,
            self.mount_point,
            self.fs_type.as_str(),
            options_str,
            self.dump,
            self.fsck_order
        )
    }

    pub fn from_fstab_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            return None;
        }

        let device = String::from(parts[0]);
        let mount_point = String::from(parts[1]);
        let fs_type = FsType::from_str(parts[2]);

        let options: Vec<MountOption> = parts[3].split(',').map(MountOption::from_str).collect();
        let options: Vec<MountOption> = parts[3]
            .split(',')
            .map(MountOption::from_str)
            .collect();

        let dump = if parts.len() > 4 {
            parts[4].parse().unwrap_or(0)
        } else {
            0
        };

        let fsck_order = if parts.len() > 5 {
            parts[5].parse().unwrap_or(0)
        } else {
            0
        };

        Some(FstabEntry {
            device,
            mount_point,
            fs_type,
            options,
            dump,
            fsck_order,
        })
    }
}

/// Fstab manager
#[derive(Debug, Clone)]
pub struct FstabManager {
    pub entries: Vec<FstabEntry>,
}

impl FstabManager {
    pub fn new() -> Self {
        FstabManager {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: FstabEntry) {
        self.entries.push(entry);
    }

    pub fn remove_entry(&mut self, mount_point: &str) -> bool {
        let original_len = self.entries.len();
        self.entries.retain(|e| e.mount_point != mount_point);
        self.entries.len() < original_len
    }

    pub fn get_entry(&self, mount_point: &str) -> Option<&FstabEntry> {
        self.entries.iter().find(|e| e.mount_point == mount_point)
    }

    pub fn parse_fstab(fstab_content: &str) -> Self {
        let mut manager = FstabManager::new();

        for line in fstab_content.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(entry) = FstabEntry::from_fstab_line(line) {
                manager.add_entry(entry);
            }
        }

        manager
    }

    pub fn to_fstab_string(&self) -> String {
        let mut result = String::new();

        for entry in &self.entries {
            result.push_str(&entry.to_fstab_line());
            result.push('\n');
        }

        result
    }

    pub fn add_standard_entries(&mut self) {
        // Add standard proc filesystem
        self.add_entry(
            FstabEntry::new(String::from("proc"), String::from("/proc"), FsType::Proc)
                .with_options(vec![MountOption::Nosuid, MountOption::Noexec, MountOption::Nodev])
                .with_dump(0)
                .with_fsck_order(0)
        );

        // Add standard sysfs
        self.add_entry(
            FstabEntry::new(String::from("sysfs"), String::from("/sys"), FsType::Sysfs)
                .with_options(vec![MountOption::Nosuid, MountOption::Noexec, MountOption::Nodev])
                .with_dump(0)
                .with_fsck_order(0)
        );

        // Add standard devtmpfs
        self.add_entry(
            FstabEntry::new(String::from("devtmpfs"), String::from("/dev"), FsType::Devtmpfs)
                .with_options(vec![MountOption::Nosuid, MountOption::Mode, MountOption::Noexec])
                .with_dump(0)
                .with_fsck_order(0)
        );

        // Add standard tmpfs for /tmp
        self.add_entry(
            FstabEntry::new(String::from("tmpfs"), String::from("/tmp"), FsType::Tmpfs)
                .with_options(vec![MountOption::Nosuid, MountOption::Nodev, MountOption::Noexec])
                .with_dump(0)
                .with_fsck_order(0)
        );
    }
}

impl Default for FstabManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_fstab_entry_creation() {
        let entry = FstabEntry::new(
            String::from("/dev/sda1"),
            String::from("/mnt/data"),
            FsType::Ext4,
        );
        assert_eq!(entry.device, "/dev/sda1");
        assert_eq!(entry.mount_point, "/mnt/data");
        assert_eq!(entry.fs_type, FsType::Ext4);
    }

    #[test]
    fn test_fstab_entry_with_options() {
        let entry = FstabEntry::new(
            String::from("/dev/sda1"),
            String::from("/mnt/data"),
            FsType::Ext4,
        )
        .with_options(vec![MountOption::Noatime, MountOption::Ssd])
        .with_dump(0)
        .with_fsck_order(2);

        assert_eq!(entry.options.len(), 2);
        assert_eq!(entry.dump, 0);
        assert_eq!(entry.fsck_order, 2);
    }

    #[test]
    fn test_fstab_entry_to_line() {
        let entry = FstabEntry::new(
            String::from("/dev/sda1"),
            String::from("/mnt/data"),
            FsType::Ext4,
        )
        .with_options(vec![MountOption::Defaults])
        .with_dump(0)
        .with_fsck_order(2);

        let line = entry.to_fstab_line();
        assert!(line.contains("/dev/sda1"));
        assert!(line.contains("/mnt/data"));
        assert!(line.contains("ext4"));
        assert!(line.contains("defaults"));
    }

    #[test]
    fn test_fstab_entry_from_line() {
        let line = "/dev/sda1 /mnt/data ext4 defaults,noatime 0 2";
        let entry = FstabEntry::from_fstab_line(line).unwrap();

        assert_eq!(entry.device, "/dev/sda1");
        assert_eq!(entry.mount_point, "/mnt/data");
        assert_eq!(entry.fs_type, FsType::Ext4);
        assert_eq!(entry.dump, 0);
        assert_eq!(entry.fsck_order, 2);
    }

    #[test]
    fn test_fstab_manager_creation() {
        let manager = FstabManager::new();
        assert_eq!(manager.entries.len(), 0);
    }

    #[test]
    fn test_fstab_manager_add_entry() {
        let mut manager = FstabManager::new();
        let entry = FstabEntry::new(
            String::from("/dev/sda1"),
            String::from("/mnt/data"),
            FsType::Ext4,
        );
        manager.add_entry(entry);
        assert_eq!(manager.entries.len(), 1);
    }

    #[test]
    fn test_fstab_manager_remove_entry() {
        let mut manager = FstabManager::new();
        let entry = FstabEntry::new(
            String::from("/dev/sda1"),
            String::from("/mnt/data"),
            FsType::Ext4,
        );
        manager.add_entry(entry);
        assert!(manager.remove_entry("/mnt/data"));
        assert_eq!(manager.entries.len(), 0);
    }

    #[test]
    fn test_fstab_manager_get_entry() {
        let mut manager = FstabManager::new();
        let entry = FstabEntry::new(
            String::from("/dev/sda1"),
            String::from("/mnt/data"),
            FsType::Ext4,
        );
        manager.add_entry(entry);

        let found = manager.get_entry("/mnt/data");
        assert!(found.is_some());
        assert_eq!(found.unwrap().device, "/dev/sda1");
    }

    #[test]
    fn test_parse_fstab() {
        let fstab_content = r#"
# Comment line
/dev/sda1 /mnt/data ext4 defaults,noatime 0 2
/dev/sda2 /mnt/backup btrfs defaults,compress 0 2
"#;

        let manager = FstabManager::parse_fstab(fstab_content);
        assert_eq!(manager.entries.len(), 2);
    }

    #[test]
    fn test_to_fstab_string() {
        let mut manager = FstabManager::new();
        manager.add_entry(
            FstabEntry::new(
                String::from("/dev/sda1"),
                String::from("/mnt/data"),
                FsType::Ext4,
            ).with_options(vec![MountOption::Defaults])
             .with_dump(0)
             .with_fsck_order(2)
        );

        let fstab_str = manager.to_fstab_string();
        assert!(fstab_str.contains("/dev/sda1"));
        assert!(fstab_str.contains("/mnt/data"));
    }

    #[test]
    fn test_add_standard_entries() {
        let mut manager = FstabManager::new();
        manager.add_standard_entries();

        assert!(manager.get_entry("/proc").is_some());
        assert!(manager.get_entry("/sys").is_some());
        assert!(manager.get_entry("/dev").is_some());
        assert!(manager.get_entry("/tmp").is_some());
    }

    #[test]
    fn test_fs_type_from_str() {
        assert_eq!(FsType::from_str("ext4"), FsType::Ext4);
        assert_eq!(FsType::from_str("btrfs"), FsType::Btrfs);
        assert_eq!(FsType::from_str("zfs"), FsType::Zfs);
        assert_eq!(FsType::from_str("unknown"), FsType::Auto);
    }

    #[test]
    fn test_mount_option_from_str() {
        assert_eq!(MountOption::from_str("defaults"), MountOption::Defaults);
        assert_eq!(MountOption::from_str("noatime"), MountOption::Noatime);
        assert_eq!(MountOption::from_str("compress"), MountOption::Compress);
        assert_eq!(MountOption::from_str("custom"), MountOption::Custom(String::from("custom")));
    }

    #[test]
    fn test_round_trip() {
        let original = FstabEntry::new(
            String::from("/dev/sda1"),
            String::from("/mnt/data"),
            FsType::Ext4,
        )
        .with_options(vec![MountOption::Noatime, MountOption::Ssd])
        .with_dump(0)
        .with_fsck_order(2);

        let line = original.to_fstab_line();
        let parsed = FstabEntry::from_fstab_line(&line).unwrap();

        assert_eq!(original.device, parsed.device);
        assert_eq!(original.mount_point, parsed.mount_point);
        assert_eq!(original.fs_type, parsed.fs_type);
        assert_eq!(original.dump, parsed.dump);
        assert_eq!(original.fsck_order, parsed.fsck_order);
    }
}
