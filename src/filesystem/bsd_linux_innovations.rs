//! SigmaOS BSD & Linux Inspired Filesystem Innovations
//!
//! Provides clean-room implementations for:
//! 1. FreeBSD Soft Updates metadata dependency tracking (`BsdSoftUpdatesEngine`)
//! 2. OpenBSD securelevel lockdown and mount flags enforcement (`OpenBsdMountEnforcer`)
//! 3. Linux OverlayFS / Union Mount subsystem (`LinuxOverlayFsManager`)
//! 4. Linux ProcFS & SysFS dynamic telemetry virtual file system (`LinuxProcSysfsEmulator`)
use alloc::vec;
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Metadata update dependency types for FreeBSD Soft Updates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataOp {
    BlockAlloc { block_id: u64 },
    InodeAlloc { inode_id: u64, block_id: u64 },
    DirAddEntry { parent_inode: u64, child_inode: u64 },
}

/// Dependency rule for metadata writes
#[derive(Debug, Clone)]
pub struct MetadataDependency {
    pub op: MetadataOp,
    pub depends_on_op_id: Option<usize>,
    pub committed: bool,
}

/// FreeBSD-inspired Soft Updates metadata dependency engine
pub struct BsdSoftUpdatesEngine {
    pub pending_ops: Vec<MetadataDependency>,
}

impl BsdSoftUpdatesEngine {
    pub fn new() -> Self {
        Self {
            pending_ops: Vec::new(),
        }
    }

    /// Register a metadata operation with an optional prerequisite operation index
    pub fn register_operation(&mut self, op: MetadataOp, depends_on: Option<usize>) -> usize {
        let op_id = self.pending_ops.len();
        self.pending_ops.push(MetadataDependency {
            op,
            depends_on_op_id: depends_on,
            committed: false,
        });
        op_id
    }

    /// Flush metadata operations in strict dependency order to guarantee crash consistency
    pub fn commit_flush_sequence(&mut self) -> Vec<usize> {
        let mut flushed = Vec::new();
        let mut progress = true;

        while progress {
            progress = false;
            for i in 0..self.pending_ops.len() {
                if self.pending_ops[i].committed {
                    continue;
                }

                // Check if prerequisite operation has been committed
                let can_commit = match self.pending_ops[i].depends_on_op_id {
                    None => true,
                    Some(parent_id) => self.pending_ops[parent_id].committed,
                };

                if can_commit {
                    self.pending_ops[i].committed = true;
                    flushed.push(i);
                    progress = true;
                }
            }
        }

        flushed
    }

    /// Check if all registered metadata operations are committed
    pub fn is_fully_committed(&self) -> bool {
        self.pending_ops.iter().all(|d| d.committed)
    }
}

impl Default for BsdSoftUpdatesEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenBSD Mount Flags
pub const MNT_RDONLY: u32 = 0x0001;
pub const MNT_NOEXEC: u32 = 0x0002;
pub const MNT_NOSUID: u32 = 0x0004;
pub const MNT_NODEV: u32 = 0x0008;

/// OpenBSD-inspired Mount Flag and Securelevel Enforcer
pub struct OpenBsdMountEnforcer {
    pub mount_flags: BTreeMap<String, u32>, // mountpoint -> bitmask flags
}

impl OpenBsdMountEnforcer {
    pub fn new() -> Self {
        Self {
            mount_flags: BTreeMap::new(),
        }
    }

    pub fn set_mount_flags(&mut self, mountpoint: &str, flags: u32) {
        self.mount_flags.insert(mountpoint.to_string(), flags);
    }

    /// Validates file access against mount flags and system securelevel
    pub fn validate_access(
        &self,
        mountpoint: &str,
        path: &str,
        is_write: bool,
        is_exec: bool,
        is_dev: bool,
        securelevel: i32,
    ) -> Result<(), &'static str> {
        let flags = self.mount_flags.get(mountpoint).copied().unwrap_or(0);

        // Check MNT_RDONLY
        if is_write && (flags & MNT_RDONLY) != 0 {
            return Err("EROFS: Read-only file system");
        }

        // Check MNT_NOEXEC
        if is_exec && (flags & MNT_NOEXEC) != 0 {
            return Err("EACCES: Execution prohibited by noexec mount flag");
        }

        // Check MNT_NODEV
        if is_dev && (flags & MNT_NODEV) != 0 {
            return Err("EACCES: Device node access prohibited by nodev mount flag");
        }

        // OpenBSD Securelevel lockdown (securelevel > 0 blocks direct raw disk/dev write access)
        if securelevel > 0 && is_write && path.starts_with("/dev/raw") {
            return Err("EPERM: Raw disk write prohibited under active securelevel lockdown");
        }

        Ok(())
    }
}

impl Default for OpenBsdMountEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

/// Linux OverlayFS / Union Mount Subsystem
pub struct LinuxOverlayFsManager {
    pub lower_layer: BTreeMap<String, Vec<u8>>, // Read-only lower layer
    pub upper_layer: BTreeMap<String, Vec<u8>>, // Read-write upper layer
    pub whiteouts: Vec<String>,                 // Deleted lower-layer entries
}

impl LinuxOverlayFsManager {
    pub fn new() -> Self {
        Self {
            lower_layer: BTreeMap::new(),
            upper_layer: BTreeMap::new(),
            whiteouts: Vec::new(),
        }
    }

    pub fn add_lower_file(&mut self, path: &str, content: &[u8]) {
        self.lower_layer.insert(path.to_string(), content.to_vec());
    }

    /// Read file prioritizing upper layer over lower layer unless whited out
    pub fn read_file(&self, path: &str) -> Option<&[u8]> {
        if self.whiteouts.iter().any(|w| w == path) {
            return None; // File was whited out (deleted in overlay)
        }

        if let Some(data) = self.upper_layer.get(path) {
            return Some(data.as_slice());
        }

        self.lower_layer.get(path).map(|v| v.as_slice())
    }

    /// Write file to upper layer (copy-up from lower if needed)
    pub fn write_file(&mut self, path: &str, content: &[u8]) {
        // Remove whiteout if recreating file
        self.whiteouts.retain(|w| w != path);
        self.upper_layer.insert(path.to_string(), content.to_vec());
    }

    /// Delete file via OverlayFS whiteout
    pub fn delete_file(&mut self, path: &str) -> bool {
        let in_upper = self.upper_layer.remove(path).is_some();
        let in_lower = self.lower_layer.contains_key(path);

        if in_lower {
            if !self.whiteouts.contains(&path.to_string()) {
                self.whiteouts.push(path.to_string());
            }
            return true;
        }

        in_upper
    }
}

impl Default for LinuxOverlayFsManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Linux ProcFS & SysFS Telemetry Emulator
pub struct LinuxProcSysfsEmulator {
    pub uptime_seconds: u64,
    pub total_memory_kb: u64,
    pub free_memory_kb: u64,
    pub cpu_count: usize,
}

impl LinuxProcSysfsEmulator {
    pub fn new(
        uptime_seconds: u64,
        total_memory_kb: u64,
        free_memory_kb: u64,
        cpu_count: usize,
    ) -> Self {
        Self {
            uptime_seconds,
            total_memory_kb,
            free_memory_kb,
            cpu_count,
        }
    }

    /// Dynamic path resolution for /proc and /sys system files
    pub fn read_virtual_path(&self, path: &str) -> Option<String> {
        match path {
            "/proc/meminfo" => Some(format!(
                "MemTotal:        {} kB\nMemFree:         {} kB\n",
                self.total_memory_kb, self.free_memory_kb
            )),
            "/proc/uptime" => Some(format!("{}.00 0.00\n", self.uptime_seconds)),
            "/proc/version" => Some(
                "Linux version 6.8.0-sigmaos (gcc 13.2.0) #1 SMP PREEMPT_DYNAMIC\n".to_string(),
            ),
            "/sys/devices/system/cpu/online" => {
                Some(format!("0-{}\n", self.cpu_count.saturating_sub(1)))
            }
            "/sys/kernel/debug" => {
                Some("debugfs /sys/kernel/debug debugfs rw,relatime 0 0\n".to_string())
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsd_soft_updates_ordering() {
        let mut engine = BsdSoftUpdatesEngine::new();

        // Register 1: BlockAlloc
        let op0 = engine.register_operation(MetadataOp::BlockAlloc { block_id: 100 }, None);
        // Register 2: InodeAlloc depending on BlockAlloc
        let op1 = engine.register_operation(
            MetadataOp::InodeAlloc {
                inode_id: 5,
                block_id: 100,
            },
            Some(op0),
        );
        // Register 3: DirAddEntry depending on InodeAlloc
        let op2 = engine.register_operation(
            MetadataOp::DirAddEntry {
                parent_inode: 1,
                child_inode: 5,
            },
            Some(op1),
        );

        assert!(!engine.is_fully_committed());
        let sequence = engine.commit_flush_sequence();

        assert_eq!(sequence, alloc::vec![0, 1, 2]);
        assert!(engine.is_fully_committed());
    }

    #[test]
    fn test_openbsd_mount_enforcer() {
        let mut enforcer = OpenBsdMountEnforcer::new();
        enforcer.set_mount_flags("/var", MNT_NOEXEC | MNT_NOSUID);

        // Read/Write on /var should succeed
        assert!(enforcer
            .validate_access("/var", "/var/log/syslog", true, false, false, 0)
            .is_ok());

        // Execution on /var should fail due to MNT_NOEXEC
        assert!(enforcer
            .validate_access("/var", "/var/tmp/script.sh", false, true, false, 0)
            .is_err());

        // Raw disk write with securelevel > 0 should fail
        assert!(enforcer
            .validate_access("/", "/dev/raw/disk0", true, false, false, 1)
            .is_err());
    }

    #[test]
    fn test_linux_overlayfs() {
        let mut overlay = LinuxOverlayFsManager::new();
        overlay.add_lower_file("/etc/hosts", b"127.0.0.1 localhost");

        // Read from lower layer
        assert_eq!(
            overlay.read_file("/etc/hosts").unwrap(),
            b"127.0.0.1 localhost"
        );

        // Copy-up write to upper layer
        overlay.write_file("/etc/hosts", b"127.0.0.1 localhost sigmaos");
        assert_eq!(
            overlay.read_file("/etc/hosts").unwrap(),
            b"127.0.0.1 localhost sigmaos"
        );

        // Delete (creates whiteout)
        assert!(overlay.delete_file("/etc/hosts"));
        assert!(overlay.read_file("/etc/hosts").is_none());
    }

    #[test]
    fn test_linux_proc_sysfs_emulator() {
        let emu = LinuxProcSysfsEmulator::new(120, 16384000, 8192000, 8);

        let meminfo = emu.read_virtual_path("/proc/meminfo").unwrap();
        assert!(meminfo.contains("16384000 kB"));

        let cpu = emu
            .read_virtual_path("/sys/devices/system/cpu/online")
            .unwrap();
        assert_eq!(cpu, "0-7\n");
    }
}

// ================= Linux FHS 3.0 & FreeBSD hier(7) Filesystem Hierarchy Engine =================

/// Sovereign Linux FHS 3.0 & FreeBSD hier(7) unified filesystem hierarchy manager
pub struct SovereignFhsHierarchyEngine {
    pub merged_usr: bool,
    pub var_run_redirect: bool,
    pub bsd_hier_mode: bool,
}

impl SovereignFhsHierarchyEngine {
    pub fn new() -> Self {
        Self {
            merged_usr: true,
            var_run_redirect: true,
            bsd_hier_mode: true,
        }
    }

    /// Resolves raw paths into canonical Linux FHS 3.0 and FreeBSD hier(7) paths
    pub fn resolve_fhs_path(&self, raw_path: &str) -> String {
        let mut path = raw_path.to_string();

        // 1. Linux merged-usr resolution (/bin -> /usr/bin, /sbin -> /usr/sbin, /lib -> /usr/lib)
        if self.merged_usr {
            if path == "/bin" || path.starts_with("/bin/") {
                path = format!("/usr{}", path);
            } else if path == "/sbin" || path.starts_with("/sbin/") {
                path = format!("/usr{}", path);
            } else if path == "/lib" || path.starts_with("/lib/") {
                path = format!("/usr{}", path);
            } else if path == "/lib64" || path.starts_with("/lib64/") {
                path = format!("/usr{}", path);
            }
        }

        // 2. /var/run -> /run and /var/lock -> /run/lock symlink redirection
        if self.var_run_redirect {
            if path == "/var/run" || path.starts_with("/var/run/") {
                path = format!("/run{}", &path[8..]);
            } else if path == "/var/lock" || path.starts_with("/var/lock/") {
                path = format!("/run/lock{}", &path[9..]);
            }
        }

        // 3. FreeBSD hier(7) /usr/local ports directory hierarchy resolution
        if self.bsd_hier_mode && (path == "/usr/local" || path.starts_with("/usr/local/")) {
            // Keep /usr/local hierarchy intact as top-class BSD ports prefix
            return path;
        }

        path
    }
}

impl Default for SovereignFhsHierarchyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod fhs_tests {
    use super::*;

    #[test]
    fn test_fhs_hierarchy_resolution() {
        let engine = SovereignFhsHierarchyEngine::new();

        // Merged-usr test
        assert_eq!(engine.resolve_fhs_path("/bin/sh"), "/usr/bin/sh");
        assert_eq!(engine.resolve_fhs_path("/sbin/init"), "/usr/sbin/init");
        assert_eq!(
            engine.resolve_fhs_path("/lib/libc.so.6"),
            "/usr/lib/libc.so.6"
        );

        // /var/run redirection
        assert_eq!(
            engine.resolve_fhs_path("/var/run/sshd.pid"),
            "/run/sshd.pid"
        );
        assert_eq!(
            engine.resolve_fhs_path("/var/lock/subsys"),
            "/run/lock/subsys"
        );

        // FreeBSD hier(7) /usr/local
        assert_eq!(
            engine.resolve_fhs_path("/usr/local/etc/nginx.conf"),
            "/usr/local/etc/nginx.conf"
        );
    }
}
