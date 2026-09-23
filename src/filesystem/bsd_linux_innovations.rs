//! SigmaOS BSD & Linux Inspired Filesystem Innovations
//!
//! Provides clean-room implementations for:
//! 1. FreeBSD Soft Updates metadata dependency tracking (`BsdSoftUpdatesEngine`)
//! 2. OpenBSD securelevel lockdown and mount flags enforcement (`OpenBsdMountEnforcer`)
//! 3. Linux OverlayFS / Union Mount subsystem (`LinuxOverlayFsManager`)
//! 4. Linux ProcFS & SysFS dynamic telemetry virtual file system (`LinuxProcSysfsEmulator`)

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

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
        let _op2 = engine.register_operation(
            MetadataOp::DirAddEntry {
                parent_inode: 1,
                child_inode: 5,
            },
            Some(op1),
        );

        assert!(!engine.is_fully_committed());
        let sequence = engine.commit_flush_sequence();

        assert_eq!(sequence, std::vec![0, 1, 2]);
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

// ================= Comprehensive Linux & BSD Directory Classification Engine =================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SovereignDirectoryCategory {
    SystemBinaries,
    BootRelated,
    KernelRelated,
    ConfigFiles,
    UserRelated,
    SharedLibraries,
    MountPoints,
    Media,
    SystemInfo,
    MultiUserResources,
    TemporaryStorage,
}

pub struct SovereignComprehensiveDirectoryEngine {
    pub fhs_engine: SovereignFhsHierarchyEngine,
}

impl SovereignComprehensiveDirectoryEngine {
    pub fn new() -> Self {
        Self {
            fhs_engine: SovereignFhsHierarchyEngine::new(),
        }
    }

    pub fn classify_path(&self, raw_path: &str) -> SovereignDirectoryCategory {
        let path = self.fhs_engine.resolve_fhs_path(raw_path);

        if path.starts_with("/boot") || path.starts_with("/EFI") {
            SovereignDirectoryCategory::BootRelated
        } else if path.starts_with("/usr/lib/modules") || path.starts_with("/sys/kernel") {
            SovereignDirectoryCategory::KernelRelated
        } else if path.starts_with("/usr/bin") || path.starts_with("/usr/sbin") || path.starts_with("/usr/local/bin") || path.starts_with("/usr/local/sbin") {
            SovereignDirectoryCategory::SystemBinaries
        } else if path.starts_with("/usr/lib") || path.starts_with("/usr/lib64") || path.starts_with("/usr/local/lib") {
            SovereignDirectoryCategory::SharedLibraries
        } else if path.starts_with("/etc") || path.starts_with("/usr/local/etc") || path.starts_with("/state/etc") {
            SovereignDirectoryCategory::ConfigFiles
        } else if path.starts_with("/home") || path.starts_with("/root") || path.starts_with("/usr/home") {
            SovereignDirectoryCategory::UserRelated
        } else if path.starts_with("/media") || path.starts_with("/run/media") {
            SovereignDirectoryCategory::Media
        } else if path.starts_with("/mnt") || path.starts_with("/system/store") {
            SovereignDirectoryCategory::MountPoints
        } else if path.starts_with("/proc") || path.starts_with("/sys") || path.starts_with("/dev") {
            SovereignDirectoryCategory::SystemInfo
        } else if path.starts_with("/tmp") || path.starts_with("/var/tmp") || path.starts_with("/run/user") || path.starts_with("/dev/shm") {
            SovereignDirectoryCategory::TemporaryStorage
        } else {
            SovereignDirectoryCategory::MultiUserResources
        }
    }

    pub fn get_default_mount_flags(&self, category: SovereignDirectoryCategory) -> u32 {
        match category {
            SovereignDirectoryCategory::TemporaryStorage => MNT_NOEXEC | MNT_NOSUID | MNT_NODEV,
            SovereignDirectoryCategory::Media | SovereignDirectoryCategory::MountPoints => MNT_NOSUID | MNT_NODEV,
            SovereignDirectoryCategory::UserRelated => MNT_NOSUID | MNT_NODEV,
            SovereignDirectoryCategory::SystemInfo => MNT_NOEXEC | MNT_NOSUID,
            SovereignDirectoryCategory::ConfigFiles => MNT_NOSUID,
            SovereignDirectoryCategory::BootRelated => MNT_RDONLY | MNT_NOSUID | MNT_NODEV,
            _ => 0,
        }
    }
}

impl Default for SovereignComprehensiveDirectoryEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ================= GoboLinux Non-Hierarchical Package Directory Resolver =================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoboProgramEntry {
    pub program_name: String,
    pub version: String,
    pub binary_files: Vec<String>,
}

pub struct GoboLinuxPathResolver {
    pub programs: Vec<GoboProgramEntry>,
    pub system_index_path: String,
}

impl GoboLinuxPathResolver {
    pub fn new() -> Self {
        Self {
            programs: Vec::new(),
            system_index_path: "/System/Index".to_string(),
        }
    }

    pub fn register_program(&mut self, name: &str, version: &str, binaries: &[&str]) {
        self.programs.push(GoboProgramEntry {
            program_name: name.to_string(),
            version: version.to_string(),
            binary_files: binaries.iter().map(|s| s.to_string()).collect(),
        });
    }

    pub fn resolve_program_binary(&self, name: &str, binary: &str) -> Option<String> {
        let entry = self.programs.iter().find(|p| p.program_name == name)?;
        if entry.binary_files.iter().any(|b| b == binary) {
            Some(format!(
                "/Programs/{}/{}/bin/{}",
                entry.program_name, entry.version, binary
            ))
        } else {
            None
        }
    }

    pub fn generate_system_index_symlink(&self, name: &str, binary: &str) -> Option<String> {
        let target = self.resolve_program_binary(name, binary)?;
        Some(format!(
            "{}/bin/{} -> {}",
            self.system_index_path, binary, target
        ))
    }
}

impl Default for GoboLinuxPathResolver {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SOVEREIGN ACYCLIC GRAPH DIRECTORY ENGINE (DAG VFS)
// ============================================================================

/// Node type in the Acyclic Graph Directory Structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DagDirectoryNodeType {
    Root,
    Directory,
    RegularFile,
    Symlink,
    HardlinkRef,
}

/// A node in the Directed Acyclic Graph (DAG) directory structure
#[derive(Debug, Clone)]
pub struct DagDirectoryNode {
    pub id: u64,
    pub name: String,
    pub node_type: DagDirectoryNodeType,
    pub parents: Vec<u64>,
    pub children: Vec<u64>,
    pub content_hash: Option<String>,
}

/// Sovereign Acyclic Graph Directory Engine
/// Prevents cyclic references during hardlinks, symlink traversals,
/// and content-addressed storage references (Nix/Guix CAS & Plan 9 inspired).
pub struct SovereignAcyclicGraphDirectoryEngine {
    pub nodes: BTreeMap<u64, DagDirectoryNode>,
    pub next_node_id: u64,
}

impl SovereignAcyclicGraphDirectoryEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            nodes: BTreeMap::new(),
            next_node_id: 1,
        };
        // Create Root
        engine.nodes.insert(
            0,
            DagDirectoryNode {
                id: 0,
                name: "/".to_string(),
                node_type: DagDirectoryNodeType::Root,
                parents: Vec::new(),
                children: Vec::new(),
                content_hash: None,
            },
        );
        engine
    }

    /// Add a child directory node with cycle detection
    pub fn add_directory(&mut self, parent_id: u64, name: &str) -> Result<u64, &'static str> {
        if !self.nodes.contains_key(&parent_id) {
            return Err("Parent node not found");
        }
        let child_id = self.next_node_id;
        self.next_node_id += 1;

        let node = DagDirectoryNode {
            id: child_id,
            name: name.to_string(),
            node_type: DagDirectoryNodeType::Directory,
            parents: vec![parent_id],
            children: Vec::new(),
            content_hash: None,
        };

        self.nodes.insert(child_id, node);
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(child_id);
        }
        Ok(child_id)
    }

    /// Add a link (hardlink/symlink reference) with strict DAG cycle detection
    pub fn add_link(&mut self, parent_id: u64, target_id: u64) -> Result<(), &'static str> {
        if !self.nodes.contains_key(&parent_id) || !self.nodes.contains_key(&target_id) {
            return Err("Source or target node not found");
        }

        // Cycle check: verify if parent_id is reachable from target_id
        if self.is_reachable(target_id, parent_id) {
            return Err("Cycle detected: adding this link would create a cyclic directory loop");
        }

        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            if !parent.children.contains(&target_id) {
                parent.children.push(target_id);
            }
        }
        if let Some(target) = self.nodes.get_mut(&target_id) {
            if !target.parents.contains(&parent_id) {
                target.parents.push(parent_id);
            }
        }
        Ok(())
    }

    /// Check reachability using Breadth-First Search (BFS) to detect cycles
    pub fn is_reachable(&self, start_id: u64, target_id: u64) -> bool {
        if start_id == target_id {
            return true;
        }
        let mut queue = vec![start_id];
        let mut visited = Vec::new();

        while let Some(current) = queue.pop() {
            if current == target_id {
                return true;
            }
            if !visited.contains(&current) {
                visited.push(current);
                if let Some(node) = self.nodes.get(&current) {
                    for &child in &node.children {
                        if !visited.contains(&child) {
                            queue.push(child);
                        }
                    }
                }
            }
        }
        false
    }
}

impl Default for SovereignAcyclicGraphDirectoryEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SOVEREIGN ASYNC PROCEDURE CALL ENGINE (KERNEL & USERLAND APC QUEUE)
// ============================================================================

/// APC execution environment mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SovereignApcMode {
    KernelMode,
    UserMode,
    SpecialKernelMode,
}

/// APC item priority level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SovereignApcPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// An Asynchronous Procedure Call descriptor
#[derive(Debug, Clone)]
pub struct SovereignApcItem {
    pub apc_id: u64,
    pub target_thread_id: u64,
    pub mode: SovereignApcMode,
    pub priority: SovereignApcPriority,
    pub callback_address: u64,
    pub context_param: u64,
    pub is_executed: bool,
}

/// Sovereign Asynchronous Procedure Call Engine
/// Deferred I/O completion and thread interrupt delivery queue
/// (inspired by Linux io_uring completion callbacks, Windows/NT APC queues, and FreeBSD aio).
pub struct SovereignAsyncProcedureCallEngine {
    pub pending_apcs: Vec<SovereignApcItem>,
    pub next_apc_id: u64,
}

impl SovereignAsyncProcedureCallEngine {
    pub fn new() -> Self {
        Self {
            pending_apcs: Vec::new(),
            next_apc_id: 1,
        }
    }

    /// Queue a new Asynchronous Procedure Call for a target thread
    pub fn queue_apc(
        &mut self,
        thread_id: u64,
        mode: SovereignApcMode,
        priority: SovereignApcPriority,
        callback_address: u64,
        context_param: u64,
    ) -> u64 {
        let id = self.next_apc_id;
        self.next_apc_id += 1;

        let apc = SovereignApcItem {
            apc_id: id,
            target_thread_id: thread_id,
            mode,
            priority,
            callback_address,
            context_param,
            is_executed: false,
        };

        self.pending_apcs.push(apc);
        // Keep queue sorted by priority descending
        self.pending_apcs.sort_by(|a, b| b.priority.cmp(&a.priority));
        id
    }

    /// Dispatch and execute all pending APCs matching a thread ID and mode
    pub fn dispatch_apcs_for_thread(&mut self, thread_id: u64, mode: SovereignApcMode) -> usize {
        let mut count = 0;
        for apc in &mut self.pending_apcs {
            if apc.target_thread_id == thread_id && apc.mode == mode && !apc.is_executed {
                apc.is_executed = true;
                count += 1;
            }
        }
        self.pending_apcs.retain(|a| !a.is_executed);
        count
    }

    /// Count pending APCs for a target thread
    pub fn pending_count(&self, thread_id: u64) -> usize {
        self.pending_apcs
            .iter()
            .filter(|a| a.target_thread_id == thread_id && !a.is_executed)
            .count()
    }
}

impl Default for SovereignAsyncProcedureCallEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod gobo_tests {
    use super::*;

    #[test]
    fn test_gobolinux_path_resolver() {
        let mut gobo = GoboLinuxPathResolver::new();
        gobo.register_program("Bash", "5.2.21", &["bash", "sh"]);

        let binary_path = gobo.resolve_program_binary("Bash", "bash").unwrap();
        assert_eq!(binary_path, "/Programs/Bash/5.2.21/bin/bash");

        let symlink = gobo.generate_system_index_symlink("Bash", "bash").unwrap();
        assert_eq!(
            symlink,
            "/System/Index/bin/bash -> /Programs/Bash/5.2.21/bin/bash"
        );
    }

    #[test]
    fn test_comprehensive_directory_classification() {
        let engine = SovereignComprehensiveDirectoryEngine::new();

        assert_eq!(engine.classify_path("/bin/sh"), SovereignDirectoryCategory::SystemBinaries);
        assert_eq!(engine.classify_path("/boot/vmlinuz"), SovereignDirectoryCategory::BootRelated);
        assert_eq!(engine.classify_path("/usr/lib/modules/6.8.0/kernel"), SovereignDirectoryCategory::KernelRelated);
        assert_eq!(engine.classify_path("/etc/fstab"), SovereignDirectoryCategory::ConfigFiles);
        assert_eq!(engine.classify_path("/home/user"), SovereignDirectoryCategory::UserRelated);
        assert_eq!(engine.classify_path("/lib/libc.so.6"), SovereignDirectoryCategory::SharedLibraries);
        assert_eq!(engine.classify_path("/mnt/data"), SovereignDirectoryCategory::MountPoints);
        assert_eq!(engine.classify_path("/media/usb"), SovereignDirectoryCategory::Media);
        assert_eq!(engine.classify_path("/proc/cpuinfo"), SovereignDirectoryCategory::SystemInfo);
        assert_eq!(engine.classify_path("/usr/share/doc"), SovereignDirectoryCategory::MultiUserResources);
        assert_eq!(engine.classify_path("/tmp/test.tmp"), SovereignDirectoryCategory::TemporaryStorage);

        // Mount security flags test
        let tmp_flags = engine.get_default_mount_flags(SovereignDirectoryCategory::TemporaryStorage);
        assert_ne!(tmp_flags & MNT_NOEXEC, 0);
        assert_ne!(tmp_flags & MNT_NOSUID, 0);
        assert_ne!(tmp_flags & MNT_NODEV, 0);
    }

    #[test]
    fn test_sovereign_acyclic_graph_directory_engine() {
        let mut dag = SovereignAcyclicGraphDirectoryEngine::new();
        let usr = dag.add_directory(0, "usr").unwrap();
        let bin = dag.add_directory(usr, "bin").unwrap();

        // Adding a link from usr -> bin is fine
        assert!(dag.add_link(usr, bin).is_ok());

        // Adding a link from bin -> usr would create a cycle (usr -> bin -> usr)
        let cycle_res = dag.add_link(bin, usr);
        assert!(cycle_res.is_err());
        assert_eq!(
            cycle_res.unwrap_err(),
            "Cycle detected: adding this link would create a cyclic directory loop"
        );
    }

    #[test]
    fn test_sovereign_async_procedure_call_engine() {
        let mut apc_engine = SovereignAsyncProcedureCallEngine::new();

        let id1 = apc_engine.queue_apc(
            1001,
            SovereignApcMode::KernelMode,
            SovereignApcPriority::Normal,
            0x8000_1000,
            0xDEAD_BEEF,
        );
        let id2 = apc_engine.queue_apc(
            1001,
            SovereignApcMode::KernelMode,
            SovereignApcPriority::Critical,
            0x8000_2000,
            0xCAFE_BABE,
        );

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(apc_engine.pending_count(1001), 2);

        // Highest priority (Critical) should be at front
        assert_eq!(apc_engine.pending_apcs[0].priority, SovereignApcPriority::Critical);

        let dispatched = apc_engine.dispatch_apcs_for_thread(1001, SovereignApcMode::KernelMode);
        assert_eq!(dispatched, 2);
        assert_eq!(apc_engine.pending_count(1001), 0);
    }
}
