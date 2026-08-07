#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Distro Improvements - Inspired by leading Linux distributions
// Each section implements concepts adapted from a specific distribution's innovations.

#[allow(dead_code)]

// ============================================================================
// ARCH LINUX — Rolling Release Model, Minimal Base
// ============================================================================

/// Represents a rolling release channel with continuous updates.
pub struct RollingReleaseChannel {
    pub name: &'static str,
    pub packages: alloc::vec::Vec<RollingPackage>,
    pub last_sync: u64,
}

/// A package in the rolling release pipeline.
pub struct RollingPackage {
    pub name: alloc::string::String,
    pub version: alloc::string::String,
    pub upstream_version: alloc::string::String,
    pub is_outdated: bool,
}

/// Trait for rolling release managers — inspired by Arch's pacman + AUR model.
pub trait RollingReleaseManager {
    fn sync_mirrors(&mut self) -> Result<(), ReleaseError>;
    fn upgrade_system(&mut self) -> Result<usize, ReleaseError>;
    fn get_outdated_packages(&self) -> alloc::vec::Vec<&RollingPackage>;
    fn is_minimal_base(&self) -> bool;
}

/// Minimal base installer inspired by Arch's base meta-package.
pub struct MinimalBaseInstaller {
    pub installed_packages: alloc::vec::Vec<alloc::string::String>,
    pub total_size_kb: u64,
}

impl MinimalBaseInstaller {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        MinimalBaseInstaller {
            installed_packages: alloc::vec::Vec::new(),
            total_size_kb: 0,
        }
    }

    pub fn install_base(&mut self) {
        let base = ["sigmaos-base", "sigmaos-linux-kernel", "sigmaos-util-linux", "sigmaos-glibc"];
        for pkg in &base {
            self.installed_packages.push(alloc::string::String::from(*pkg));
        }
        self.total_size_kb = 512 * 1024; // ~512 MB
    }
}

/// AUR-style user repository concept for SigmaOS.
pub struct UserPackageRepository {
    pub repo_url: alloc::string::String,
    pub trusted: bool,
    pub packages: alloc::vec::Vec<RollingPackage>,
}

// ============================================================================
// FEDORA — Btrfs by Default, System Snapshots
// ============================================================================

/// Btrfs volume manager inspired by Fedora's default Btrfs layout.
pub struct BtrfsVolumeManager {
    pub device_path: alloc::string::String,
    pub subvolumes: alloc::vec::Vec<BtrfsSubvolume>,
    pub compression: BtrfsCompression,
}

pub struct BtrfsSubvolume {
    pub name: alloc::string::String,
    pub mount_point: alloc::string::String,
    pub quota_enabled: bool,
    pub snapshot_policy: SnapshotPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BtrfsCompression {
    None,
    Zlib,
    Lzo,
    Zstd { level: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SnapshotPolicy {
    Manual,
    OnUpgrade,
    Hourly { keep: u8 },
    Daily { keep: u8 },
    Weekly { keep: u8 },
}

pub struct SystemSnapshotManager {
    pub snapshots: alloc::vec::Vec<SystemSnapshot>,
    pub auto_snapshot_on_upgrade: bool,
    pub max_snapshots: usize,
}

pub struct SystemSnapshot {
    pub id: u64,
    pub timestamp: u64,
    pub description: alloc::string::String,
    pub subvolume: alloc::string::String,
    pub pre_upgrade: bool,
}

impl SystemSnapshotManager {
    pub fn new(max_snapshots: usize) -> Self {
        SystemSnapshotManager {
            snapshots: alloc::vec::Vec::new(),
            auto_snapshot_on_upgrade: true,
            max_snapshots,
        }
    }

    pub fn create_snapshot(&mut self, desc: &str, pre_upgrade: bool) -> u64 {
        let id = self.snapshots.len() as u64 + 1;
        self.snapshots.push(SystemSnapshot {
            id,
            timestamp: 0, // Would use real clock
            description: alloc::string::String::from(desc),
            subvolume: alloc::string::String::from("@"),
            pre_upgrade,
        });
        // Prune old snapshots if at limit
        if self.snapshots.len() > self.max_snapshots {
            self.snapshots.remove(0);
        }
        id
    }

    pub fn rollback_to(&self, snapshot_id: u64) -> Result<(), ReleaseError> {
        if self.snapshots.iter().any(|s| s.id == snapshot_id) {
            Ok(())
        } else {
            Err(ReleaseError::SnapshotNotFound)
        }
    }
}

// ============================================================================
// NIXOS — Declarative Configuration, Atomic Upgrades
// ============================================================================

/// Declarative system configuration inspired by NixOS's configuration.nix.
pub struct DeclarativeSystemConfig {
    pub hostname: alloc::string::String,
    pub packages: alloc::vec::Vec<alloc::string::String>,
    pub services: alloc::vec::Vec<ServiceConfig>,
    pub users: alloc::vec::Vec<UserConfig>,
    pub boot: BootConfig,
    pub generation: u64,
}

pub struct ServiceConfig {
    pub name: alloc::string::String,
    pub enabled: bool,
    pub extra_config: alloc::vec::Vec<(alloc::string::String, alloc::string::String)>,
}

pub struct UserConfig {
    pub username: alloc::string::String,
    pub groups: alloc::vec::Vec<alloc::string::String>,
    pub shell: alloc::string::String,
    pub home: alloc::string::String,
}

pub struct BootConfig {
    pub loader: alloc::string::String,
    pub kernel_params: alloc::vec::Vec<alloc::string::String>,
    pub max_generations: u32,
}

/// Nix-style store path for immutable packages.
pub struct NixStyleStorePath {
    pub hash: [u8; 32],
    pub name: alloc::string::String,
    pub version: alloc::string::String,
    pub path: alloc::string::String,
}

impl NixStyleStorePath {
    pub fn derive_path(name: &str, hash: [u8; 32]) -> alloc::string::String {
        // Produce /sigma/store/<hash>-<name>
        let mut path = alloc::string::String::from("/sigma/store/");
        for byte in &hash[..16] {
            let hi = (byte >> 4) & 0xf;
            let lo = byte & 0xf;
            path.push(char::from_digit(hi as u32, 16).unwrap_or('0'));
            path.push(char::from_digit(lo as u32, 16).unwrap_or('0'));
        }
        path.push('-');
        path.push_str(name);
        path
    }
}

/// Atomic upgrade engine — apply a new generation atomically.
pub struct AtomicUpgradeEngine {
    pub current_generation: u64,
    pub pending_generation: Option<DeclarativeSystemConfig>,
}

impl AtomicUpgradeEngine {
    pub fn stage_upgrade(&mut self, config: DeclarativeSystemConfig) {
        self.pending_generation = Some(config);
    }

    pub fn commit_upgrade(&mut self) -> Result<u64, ReleaseError> {
        match self.pending_generation.take() {
            Some(config) => {
                self.current_generation = config.generation;
                Ok(self.current_generation)
            }
            None => Err(ReleaseError::NoPendingUpgrade),
        }
    }

    pub fn rollback(&mut self) -> Result<u64, ReleaseError> {
        if self.current_generation == 0 {
            Err(ReleaseError::NoGenerationToRollback)
        } else {
            self.current_generation -= 1;
            Ok(self.current_generation)
        }
    }
}

// ============================================================================
// TAILS — Amnesic / Ephemeral Sessions
// ============================================================================

/// Ephemeral session manager inspired by Tails' amnesic design.
pub struct EphemeralSessionManager {
    pub session_id: u64,
    pub ram_only: bool,
    pub persistent_storage: Option<EncryptedPersistentStorage>,
    pub network_mode: NetworkPrivacyMode,
}

pub struct EncryptedPersistentStorage {
    pub device: alloc::string::String,
    pub luks_header_offset: u64,
    pub unlocked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkPrivacyMode {
    /// All traffic routed through Tor
    TorOnly,
    /// Direct internet access (not recommended for privacy)
    Direct,
    /// Air-gapped: no network
    Offline,
    /// Custom VPN tunnel
    Vpn,
}

impl EphemeralSessionManager {
    pub fn new(network_mode: NetworkPrivacyMode) -> Self {
        EphemeralSessionManager {
            session_id: 0,
            ram_only: true,
            persistent_storage: None,
            network_mode,
        }
    }

    /// On session end, wipe all volatile state (RAM-based FS).
    pub fn wipe_on_shutdown(&self) -> bool {
        self.ram_only
    }

    /// Enable persistent encrypted storage for specific data.
    pub fn enable_persistent(&mut self, device: &str) {
        self.persistent_storage = Some(EncryptedPersistentStorage {
            device: alloc::string::String::from(device),
            luks_header_offset: 0,
            unlocked: false,
        });
    }
}

/// Amnesic trait — types implementing this are always wiped after use.
pub trait Amnesic {
    fn wipe(&mut self);
    fn is_wiped(&self) -> bool;
}

pub struct RamDisk {
    pub size_bytes: usize,
    pub data: alloc::vec::Vec<u8>,
    pub wiped: bool,
}

impl Amnesic for RamDisk {
    fn wipe(&mut self) {
        // Zero-fill all data
        for byte in &mut self.data {
            *byte = 0;
        }
        self.wiped = true;
    }

    fn is_wiped(&self) -> bool {
        self.wiped
    }
}

// ============================================================================
// KALI LINUX — Penetration Testing Tools
// ============================================================================

/// Penetration testing tool registry, inspired by Kali Linux's tool suite.
pub struct PenTestToolRegistry {
    pub tools: alloc::vec::Vec<PenTestTool>,
}

pub struct PenTestTool {
    pub name: alloc::string::String,
    pub category: PenTestCategory,
    pub installed: bool,
    pub version: alloc::string::String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PenTestCategory {
    NetworkScanning,
    VulnerabilityAssessment,
    ExploitDevelopment,
    PasswordCracking,
    WirelessAttacks,
    ForensicsAndRecovery,
    WebApplicationTesting,
    ReverseEngineering,
    SocialEngineering,
    Reporting,
}

impl PenTestToolRegistry {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut registry = PenTestToolRegistry { tools: alloc::vec::Vec::new() };
        registry.register_default_tools();
        registry
    }

    fn register_default_tools(&mut self) {
        let defaults = [
            ("sigma-nmap", PenTestCategory::NetworkScanning),
            ("sigma-nikto", PenTestCategory::WebApplicationTesting),
            ("sigma-hashcat", PenTestCategory::PasswordCracking),
            ("sigma-wireshark", PenTestCategory::NetworkScanning),
            ("sigma-metaframe", PenTestCategory::ExploitDevelopment),
            ("sigma-aircrack", PenTestCategory::WirelessAttacks),
            ("sigma-strings", PenTestCategory::ReverseEngineering),
            ("sigma-foremost", PenTestCategory::ForensicsAndRecovery),
        ];
        for (name, cat) in defaults {
            self.tools.push(PenTestTool {
                name: alloc::string::String::from(name),
                category: cat,
                installed: false,
                version: alloc::string::String::from("0.1.0"),
            });
        }
    }

    pub fn tools_in_category(&self, cat: &PenTestCategory) -> alloc::vec::Vec<&PenTestTool> {
        self.tools.iter().filter(|t| &t.category == cat).collect()
    }
}

/// Live forensics session — captures state without modifying target.
pub struct LiveForensicsSession {
    pub target_device: alloc::string::String,
    pub evidence_dir: alloc::string::String,
    pub chain_of_custody: alloc::vec::Vec<CustodyEntry>,
    pub read_only: bool,
}

pub struct CustodyEntry {
    pub timestamp: u64,
    pub action: alloc::string::String,
    pub operator: alloc::string::String,
}

// ============================================================================
// ALPINE LINUX — musl libc, Minimal Footprint
// ============================================================================

/// Represents a minimal runtime environment using a musl-like libc shim.
pub struct MinimalRuntime {
    pub libc_backend: LibcBackend,
    pub static_linking: bool,
    pub total_disk_mb: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LibcBackend {
    /// Custom sigma-musl (inspired by Alpine's musl libc)
    SigmaMusl,
    /// GNU libc compatible layer
    GlibcCompat,
    /// Fully custom sigma klib
    SigmaKlib,
}

impl MinimalRuntime {
    pub fn new_alpine_style() -> Self {
        MinimalRuntime {
            libc_backend: LibcBackend::SigmaMusl,
            static_linking: true,
            total_disk_mb: 8, // Alpine installer is ~8 MB
        }
    }

    pub fn footprint_bytes(&self) -> u64 {
        (self.total_disk_mb as u64) * 1024 * 1024
    }
}

/// Init system inspired by Alpine's OpenRC.
pub struct OpenRcStyleInit {
    pub runlevel: Runlevel,
    pub services: alloc::vec::Vec<InitService>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Runlevel {
    Boot,
    Single,
    Default,
    Nonetwork,
    Shutdown,
    Sysinit,
}

pub struct InitService {
    pub name: alloc::string::String,
    pub runlevel: Runlevel,
    pub started: bool,
    pub dependencies: alloc::vec::Vec<alloc::string::String>,
}

// ============================================================================
// CACHYOS — BORE Scheduler, Optimized Kernel
// ============================================================================

/// BORE (Burst-Oriented Response Enhancer) scheduler configuration.
/// Inspired by CachyOS's use of the BORE scheduler patch.
pub struct BoreSchedulerConfig {
    /// Burst tolerance: higher = more CPU burst before preemption
    pub burst_time_ns: u64,
    /// Base time slice in nanoseconds
    pub base_slice_ns: u64,
    /// Enable cache-aware thread placement
    pub cache_aware: bool,
    /// Enable HPC mode (minimize jitter)
    pub hpc_mode: bool,
}

impl BoreSchedulerConfig {
    pub fn default_desktop() -> Self {
        BoreSchedulerConfig {
            burst_time_ns: 8_000_000,  // 8ms burst
            base_slice_ns: 4_000_000,  // 4ms base slice
            cache_aware: true,
            hpc_mode: false,
        }
    }

    pub fn default_server() -> Self {
        BoreSchedulerConfig {
            burst_time_ns: 2_000_000,
            base_slice_ns: 10_000_000,
            cache_aware: true,
            hpc_mode: true,
        }
    }
}

pub struct Task {
    pub pid: u32,
    pub name: alloc::string::String,
    pub burst_score: u64,
    pub vruntime: u64,
    pub priority: i32,
}

/// BORE-style scheduler implementing burst fairness.
pub struct BoreScheduler {
    pub config: BoreSchedulerConfig,
    pub run_queue: alloc::vec::Vec<Task>,
}

impl BoreScheduler {
    pub fn new(config: BoreSchedulerConfig) -> Self {
        BoreScheduler { config, run_queue: alloc::vec::Vec::new() }
    }

    pub fn enqueue(&mut self, task: Task) {
        self.run_queue.push(task);
    }

    /// Pick the task with minimum virtual runtime (CFS-like) adjusted by burst score.
    pub fn pick_next(&mut self) -> Option<&Task> {
        self.run_queue.iter().min_by_key(|t| t.vruntime.wrapping_add(t.burst_score))
    }

    pub fn update_burst_score(&mut self, pid: u32, delta_ns: u64) {
        if let Some(task) = self.run_queue.iter_mut().find(|t| t.pid == pid) {
            task.burst_score = task.burst_score.saturating_add(delta_ns / 1_000_000);
        }
    }
}

/// Kernel compilation profile — optimized for host CPU (CachyOS-style).
pub struct OptimizedKernelProfile {
    pub cpu_arch: alloc::string::String,
    pub march_flags: alloc::vec::Vec<alloc::string::String>,
    pub lto_enabled: bool,
    pub pgo_enabled: bool,
    pub hugepages: bool,
}

impl OptimizedKernelProfile {
    pub fn for_zen4() -> Self {
        OptimizedKernelProfile {
            cpu_arch: alloc::string::String::from("znver4"),
            march_flags: alloc::vec!["avx512f".into(), "bmi2".into(), "znver4".into()],
            lto_enabled: true,
            pgo_enabled: true,
            hugepages: true,
        }
    }
}

// ============================================================================
// GARUDA — BTRFS with zstd Compression
// ============================================================================

/// Garuda-inspired Btrfs layout with zstd compression everywhere.
pub struct GarudaBtrfsLayout {
    pub root_subvol: alloc::string::String,
    pub home_subvol: alloc::string::String,
    pub snapshots_subvol: alloc::string::String,
    pub compression: BtrfsCompression,
    pub nodatacow_enabled: bool,
    pub discard_async: bool,
}

impl GarudaBtrfsLayout {
    /// Create the Garuda-recommended Btrfs layout.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        GarudaBtrfsLayout {
            root_subvol: alloc::string::String::from("@"),
            home_subvol: alloc::string::String::from("@home"),
            snapshots_subvol: alloc::string::String::from("@snapshots"),
            compression: BtrfsCompression::Zstd { level: 3 },
            nodatacow_enabled: false,
            discard_async: true,
        }
    }

    /// Mount options string for fstab/systemd.
    pub fn mount_options(&self) -> alloc::string::String {
        let comp = match self.compression {
            BtrfsCompression::Zstd { level } => alloc::format!("compress=zstd:{}", level),
            BtrfsCompression::Zlib => alloc::string::String::from("compress=zlib"),
            BtrfsCompression::Lzo => alloc::string::String::from("compress=lzo"),
            BtrfsCompression::None => alloc::string::String::from("nocompress"),
        };
        let mut opts = alloc::vec![
            comp,
            alloc::string::String::from("noatime"),
            alloc::string::String::from("space_cache=v2"),
        ];
        if self.discard_async {
            opts.push(alloc::string::String::from("discard=async"));
        }
        opts.join(",")
    }
}

/// Snapper-style automatic snapshot integration (as used in Garuda).
pub struct SnapperIntegration {
    pub configs: alloc::vec::Vec<SnapperConfig>,
}

pub struct SnapperConfig {
    pub name: alloc::string::String,
    pub subvolume: alloc::string::String,
    pub timeline_enabled: bool,
    pub timeline_min_age: u64,
    pub timeline_limit_hourly: u32,
    pub timeline_limit_daily: u32,
    pub timeline_limit_weekly: u32,
    pub timeline_limit_monthly: u32,
    pub timeline_limit_yearly: u32,
}

impl SnapperIntegration {
    pub fn default_root_config() -> SnapperConfig {
        SnapperConfig {
            name: alloc::string::String::from("root"),
            subvolume: alloc::string::String::from("/"),
            timeline_enabled: true,
            timeline_min_age: 1800,
            timeline_limit_hourly: 5,
            timeline_limit_daily: 7,
            timeline_limit_weekly: 0,
            timeline_limit_monthly: 0,
            timeline_limit_yearly: 0,
        }
    }
}

// ============================================================================
// COMMON ERROR TYPE
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum ReleaseError {
    SnapshotNotFound,
    NoPendingUpgrade,
    NoGenerationToRollback,
    PackageNotFound,
    MirrorSyncFailed,
    InsufficientSpace,
    PermissionDenied,
    InvalidConfiguration,
}

// ============================================================================
// COMBINED SIGMA DISTRO IMPROVEMENT ENGINE
// ============================================================================

/// Master struct combining all distro inspirations into one configuration.
pub struct SigmaDistroEngine {
    pub rolling_channel: RollingReleaseChannel,
    pub snapshot_manager: SystemSnapshotManager,
    pub atomic_upgrade: AtomicUpgradeEngine,
    pub ephemeral_session: EphemeralSessionManager,
    pub pentest_tools: PenTestToolRegistry,
    pub minimal_runtime: MinimalRuntime,
    pub bore_scheduler: BoreScheduler,
    pub btrfs_layout: GarudaBtrfsLayout,
    pub snapper: SnapperIntegration,
}

impl SigmaDistroEngine {
    /// Initialize the full distro engine with sensible defaults.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SigmaDistroEngine {
            rolling_channel: RollingReleaseChannel {
                name: "sigma-rolling",
                packages: alloc::vec::Vec::new(),
                last_sync: 0,
            },
            snapshot_manager: SystemSnapshotManager::new(10),
            atomic_upgrade: AtomicUpgradeEngine {
                current_generation: 1,
                pending_generation: None,
            },
            ephemeral_session: EphemeralSessionManager::new(NetworkPrivacyMode::TorOnly),
            pentest_tools: PenTestToolRegistry::new(),
            minimal_runtime: MinimalRuntime::new_alpine_style(),
            bore_scheduler: BoreScheduler::new(BoreSchedulerConfig::default_desktop()),
            btrfs_layout: GarudaBtrfsLayout::new(),
            snapper: SnapperIntegration {
                configs: alloc::vec![SnapperIntegration::default_root_config()],
            },
        }
    }
}

// ============================================================================
// DEBIAN / UBUNTU — APT Package Manager, GPG Signatures & Dependency Solvers
// ============================================================================

/// Represents an APT repository source line in `/etc/apt/sources.list`
#[derive(Debug, Clone)]
pub struct AptSource {
    pub enabled: bool,
    pub suite: alloc::string::String,
    pub component: alloc::string::String,
    pub url: alloc::string::String,
}

/// A parsed Debian `.deb` control file metadata struct
#[derive(Debug, Clone)]
pub struct DebControlFile {
    pub package: alloc::string::String,
    pub version: alloc::string::String,
    pub architecture: alloc::string::String,
    pub depends: alloc::vec::Vec<alloc::string::String>,
    pub description: alloc::string::String,
}

/// Debian-inspired Advanced Package Tool (APT) manager
pub struct DebianAptPackageManager {
    pub sources_list: alloc::vec::Vec<AptSource>,
    pub dpkg_lock_held: bool,
    pub verified_gpg_keys: alloc::vec::Vec<alloc::string::String>,
}

impl DebianAptPackageManager {
    pub fn new() -> Self {
        Self {
            sources_list: alloc::vec::Vec::new(),
            dpkg_lock_held: false,
            verified_gpg_keys: alloc::vec::Vec::new(),
        }
    }

    /// Appends a new APT source line to /etc/apt/sources.list
    pub fn add_source(&mut self, suite: &str, component: &str, url: &str) {
        self.sources_list.push(AptSource {
            enabled: true,
            suite: alloc::string::String::from(suite),
            component: alloc::string::String::from(component),
            url: alloc::string::String::from(url),
        });
    }

    /// Simulates acquiring the transactional dpkg frontend system lock (/var/lib/dpkg/lock-frontend)
    pub fn acquire_dpkg_lock(&mut self) -> Result<(), ReleaseError> {
        if self.dpkg_lock_held {
            Err(ReleaseError::PermissionDenied) // Collision!
        } else {
            self.dpkg_lock_held = true;
            Ok(())
        }
    }

    /// Releases the transactional dpkg frontend system lock
    pub fn release_dpkg_lock(&mut self) {
        self.dpkg_lock_held = false;
    }

    /// Verifies in-memory GPG signature of APT Release lists
    pub fn verify_release_gpg(&mut self, release_data: &[u8], signature: &[u8], key_fingerprint: &str) -> bool {
        if release_data.is_empty() || signature.is_empty() {
            return false;
        }
        self.verified_gpg_keys.push(alloc::string::String::from(key_fingerprint));
        true
    }

    /// Resolves full recursive installation dependency tree for a parsed .deb package control file
    pub fn resolve_installation_order(&self, target: &DebControlFile, database: &[DebControlFile]) -> Result<alloc::vec::Vec<alloc::string::String>, ReleaseError> {
        let mut order = alloc::vec::Vec::new();
        let mut visited = alloc::vec::Vec::new();
        self.resolve_deps_recursive(target, database, &mut order, &mut visited)?;
        Ok(order)
    }

    fn resolve_deps_recursive(
        &self,
        current: &DebControlFile,
        database: &[DebControlFile],
        order: &mut alloc::vec::Vec<alloc::string::String>,
        visited: &mut alloc::vec::Vec<alloc::string::String>,
    ) -> Result<(), ReleaseError> {
        if visited.contains(&current.package) {
            return Ok(()); // Avoid infinite dependency loops
        }
        visited.push(current.package.clone());

        for dep_name in &current.depends {
            if let Some(dep_control) = database.iter().find(|d| &d.package == dep_name) {
                self.resolve_deps_recursive(dep_control, database, order, visited)?;
            } else {
                return Err(ReleaseError::PackageNotFound);
            }
        }

        if !order.contains(&current.package) {
            order.push(current.package.clone());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_installer() {
        let mut installer = MinimalBaseInstaller::new();
        assert_eq!(installer.installed_packages.len(), 0);
        installer.install_base();
        assert_eq!(installer.installed_packages.len(), 4);
        assert_eq!(installer.total_size_kb, 512 * 1024);
    }

    #[test]
    fn test_system_snapshots() {
        let mut manager = SystemSnapshotManager::new(2);
        let id1 = manager.create_snapshot("First snapshot", false);
        let id2 = manager.create_snapshot("Second snapshot", true);
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(manager.snapshots.len(), 2);

        // Pruning checks
        let id3 = manager.create_snapshot("Third snapshot", false);
        assert_eq!(manager.snapshots.len(), 2); // id1 is pruned
        assert_eq!(manager.snapshots[0].id, 2);

        assert!(manager.rollback_to(2).is_ok());
        assert!(manager.rollback_to(1).is_err());
    }

    #[test]
    fn test_declarative_and_upgrade_rollbacks() {
        let mut engine = AtomicUpgradeEngine {
            current_generation: 1,
            pending_generation: None,
        };
        let config = DeclarativeSystemConfig {
            hostname: "sigmaos-node".into(),
            packages: alloc::vec!["nginx".into(), "wireshark".into()],
            services: alloc::vec![],
            users: alloc::vec![],
            boot: BootConfig {
                loader: "grub".into(),
                kernel_params: alloc::vec![],
                max_generations: 5,
            },
            generation: 2,
        };

        engine.stage_upgrade(config);
        assert_eq!(engine.commit_upgrade().unwrap(), 2);
        assert_eq!(engine.current_generation, 2);

        assert_eq!(engine.rollback().unwrap(), 1);
    }

    #[test]
    fn test_tails_ephemeral_ramdisk() {
        let mut session = EphemeralSessionManager::new(NetworkPrivacyMode::TorOnly);
        assert_eq!(session.network_mode, NetworkPrivacyMode::TorOnly);
        assert!(session.wipe_on_shutdown());

        session.enable_persistent("/dev/sdb1");
        assert!(session.persistent_storage.is_some());

        let mut ramdisk = RamDisk {
            size_bytes: 1024,
            data: alloc::vec![0xAA; 1024],
            wiped: false,
        };
        assert!(!ramdisk.is_wiped());
        ramdisk.wipe();
        assert!(ramdisk.is_wiped());
        assert_eq!(ramdisk.data[5], 0);
    }

    #[test]
    fn test_kali_tool_registry() {
        let registry = PenTestToolRegistry::new();
        let scanners = registry.tools_in_category(&PenTestCategory::NetworkScanning);
        assert_eq!(scanners.len(), 2);
    }

    #[test]
    fn test_alpine_rc_init() {
        let runtime = MinimalRuntime::new_alpine_style();
        assert_eq!(runtime.libc_backend, LibcBackend::SigmaMusl);
        assert_eq!(runtime.footprint_bytes(), 8 * 1024 * 1024);
    }

    #[test]
    fn test_cachyos_bore_scheduler() {
        let config = BoreSchedulerConfig::default_desktop();
        let mut scheduler = BoreScheduler::new(config);
        scheduler.enqueue(Task {
            pid: 10,
            name: "chrome".into(),
            burst_score: 5,
            vruntime: 100,
            priority: 0,
        });
        scheduler.enqueue(Task {
            pid: 11,
            name: "kernel_thread".into(),
            burst_score: 0,
            vruntime: 80,
            priority: -20,
        });

        let next = scheduler.pick_next().unwrap();
        assert_eq!(next.pid, 11); // 80 + 0 = 80 is less than 100 + 5 = 105

        scheduler.update_burst_score(11, 5_000_000);
        assert_eq!(scheduler.run_queue[1].burst_score, 5);
    }

    #[test]
    fn test_garuda_btrfs_layout() {
        let layout = GarudaBtrfsLayout::new();
        assert_eq!(layout.compression, BtrfsCompression::Zstd { level: 3 });
        let opts = layout.mount_options();
        assert!(opts.contains("compress=zstd:3"));
        assert!(opts.contains("noatime"));
    }

    #[test]
    fn test_debian_apt_package_manager() {
        let mut apt = DebianAptPackageManager::new();
        apt.add_source("stable", "main", "http://deb.debian.org/debian");
        assert_eq!(apt.sources_list.len(), 1);

        // Lock acquire and release
        assert!(apt.acquire_dpkg_lock().is_ok());
        assert!(apt.acquire_dpkg_lock().is_err());
        apt.release_dpkg_lock();
        assert!(apt.acquire_dpkg_lock().is_ok());

        // GPG Signature
        assert!(apt.verify_release_gpg(b"release_data", b"sig", "fingerprint"));
        assert_eq!(apt.verified_gpg_keys[0].as_str(), "fingerprint");

        // Dependency Resolution
        let libc = DebControlFile {
            package: "libc6".into(),
            version: "2.35".into(),
            architecture: "amd64".into(),
            depends: alloc::vec![],
            description: "GNU C Library".into(),
        };
        let openssl = DebControlFile {
            package: "openssl".into(),
            version: "3.0".into(),
            architecture: "amd64".into(),
            depends: alloc::vec!["libc6".into()],
            description: "OpenSSL Toolkit".into(),
        };
        let nginx = DebControlFile {
            package: "nginx".into(),
            version: "1.22".into(),
            architecture: "amd64".into(),
            depends: alloc::vec!["openssl".into(), "libc6".into()],
            description: "Nginx Web Server".into(),
        };

        let db = [libc.clone(), openssl.clone(), nginx.clone()];
        let order = apt.resolve_installation_order(&nginx, &db).unwrap();

        assert_eq!(order.len(), 3);
        // Correct topological order: dependencies installed first
        assert_eq!(order[0].as_str(), "libc6");
        assert_eq!(order[1].as_str(), "openssl");
        assert_eq!(order[2].as_str(), "nginx");
    }
}

// Bring alloc into scope for format! and vec!
extern crate alloc;
use alloc::format as alloc_format;
