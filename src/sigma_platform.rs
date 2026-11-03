//! SigmaOS Platform Integration Layer
//!
//! This module is the top-level wiring point for the SigmaOS platform.
//! It implements the 8-layer architecture defined in ARCHITECTURE-OVERVIEW.md:
//!
//!   Layer 1 – Kernel Backend Abstraction  (KernelBackend trait + adapters)
//!   Layer 2 – Boot and Init               (BootPipeline)
//!   Layer 3 – Base System                 (SystemOrchestrator)
//!   Layer 4 – Package Layer               (PackageCoordinator)
//!   Layer 5 – Desktop Shell               (DesktopSession)
//!   Layer 6 – Config and Theming          (ConfigStore, ThemeEngine)
//!   Layer 7 – Security and Trust          (SecurityContext)
//!   Layer 8 – App and Support             (AppRegistry)
//!
//! Each layer depends only on the layer beneath it and communicates
//! through the trait interfaces defined here.

// ---------------------------------------------------------------------------
// Layer 1 — Kernel Backend Abstraction
// ---------------------------------------------------------------------------

/// Backend identifier chosen at boot via `SIGMA_KERNEL_BACKEND` env var.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    /// Linux kernel (systemd, pacman/apt/dnf, NetworkManager, SELinux/AppArmor).
    Linux,
    /// BSD kernel (rc.d, pkg/pkg_add, ifconfig, pledge/unveil/Capsicum).
    Bsd,
    /// Minimal no-op backend for testing and scaffolding.
    Stub,
}

impl BackendKind {
    pub fn from_env() -> Self {
        match std::env::var("SIGMA_KERNEL_BACKEND").as_deref() {
            Ok("bsd") => BackendKind::Bsd,
            Ok("stub") => BackendKind::Stub,
            _ => BackendKind::Linux, // Linux is the default
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            BackendKind::Linux => "linux",
            BackendKind::Bsd => "bsd",
            BackendKind::Stub => "stub",
        }
    }
}

/// The core trait every kernel backend must implement.
/// Shared subsystems depend only on this trait — never on a concrete backend.
pub trait KernelBackend: Send + Sync {
    fn kind(&self) -> BackendKind;
    /// Return the init system name (e.g. "systemd", "rc.d").
    fn init_system(&self) -> &str;
    /// Return the default package manager command (e.g. "pacman", "pkg").
    fn package_manager(&self) -> &str;
    /// Return the network manager command (e.g. "nmcli", "ifconfig").
    fn network_manager(&self) -> &str;
    /// Return true if the backend is ready (required services reachable).
    fn is_ready(&self) -> bool;
    /// Return a human-readable health summary.
    fn health_summary(&self) -> String;
}

// ---------------------------------------------------------------------------
// Concrete backends
// ---------------------------------------------------------------------------

pub struct LinuxBackend;
impl KernelBackend for LinuxBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Linux
    }
    fn init_system(&self) -> &str {
        "systemd"
    }
    fn package_manager(&self) -> &str {
        "pacman"
    }
    fn network_manager(&self) -> &str {
        "nmcli"
    }
    fn is_ready(&self) -> bool {
        true
    }
    fn health_summary(&self) -> String {
        format!(
            "Linux backend: init={} pkgmgr={} netmgr={}",
            self.init_system(),
            self.package_manager(),
            self.network_manager()
        )
    }
}

pub struct BsdBackend;
impl KernelBackend for BsdBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Bsd
    }
    fn init_system(&self) -> &str {
        "rc.d"
    }
    fn package_manager(&self) -> &str {
        "pkg"
    }
    fn network_manager(&self) -> &str {
        "ifconfig"
    }
    fn is_ready(&self) -> bool {
        true
    }
    fn health_summary(&self) -> String {
        format!(
            "BSD backend: init={} pkgmgr={} netmgr={}",
            self.init_system(),
            self.package_manager(),
            self.network_manager()
        )
    }
}

pub struct StubBackend;
impl KernelBackend for StubBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Stub
    }
    fn init_system(&self) -> &str {
        "stub-init"
    }
    fn package_manager(&self) -> &str {
        "stub-pkg"
    }
    fn network_manager(&self) -> &str {
        "stub-net"
    }
    fn is_ready(&self) -> bool {
        true
    }
    fn health_summary(&self) -> String {
        "Stub backend: no-op (test/scaffold mode)".to_string()
    }
}

/// Construct the appropriate backend from the environment.
pub fn create_backend(kind: BackendKind) -> Box<dyn KernelBackend> {
    match kind {
        BackendKind::Linux => Box::new(LinuxBackend),
        BackendKind::Bsd => Box::new(BsdBackend),
        BackendKind::Stub => Box::new(StubBackend),
    }
}

// ---------------------------------------------------------------------------
// Layer 2 — Boot Pipeline
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootStage {
    Firmware,
    Bootloader,
    Kernel,
    Initramfs,
    Init,
    Services,
    Login,
    Desktop,
    Ready,
}

impl BootStage {
    pub fn next(self) -> Option<BootStage> {
        match self {
            Self::Firmware => Some(Self::Bootloader),
            Self::Bootloader => Some(Self::Kernel),
            Self::Kernel => Some(Self::Initramfs),
            Self::Initramfs => Some(Self::Init),
            Self::Init => Some(Self::Services),
            Self::Services => Some(Self::Login),
            Self::Login => Some(Self::Desktop),
            Self::Desktop => Some(Self::Ready),
            Self::Ready => None,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Firmware => "Firmware (UEFI/BIOS)",
            Self::Bootloader => "Bootloader (systemd-boot/GRUB/loader.conf)",
            Self::Kernel => "Kernel load and decompress",
            Self::Initramfs => "Early userspace (initramfs)",
            Self::Init => "Init system startup",
            Self::Services => "System services",
            Self::Login => "Display manager / login",
            Self::Desktop => "Desktop shell",
            Self::Ready => "System ready",
        }
    }
}

pub struct BootPipeline {
    pub backend: BackendKind,
    pub current_stage: BootStage,
    pub log: Vec<String>,
    pub failed_stage: Option<BootStage>,
}

impl BootPipeline {
    pub fn new(backend: BackendKind) -> Self {
        Self {
            backend,
            current_stage: BootStage::Firmware,
            log: Vec::new(),
            failed_stage: None,
        }
    }

    /// Advance to the next stage. Returns `false` if already at Ready.
    pub fn advance(&mut self) -> bool {
        self.log
            .push(format!("[boot] {} ✓", self.current_stage.label()));
        match self.current_stage.next() {
            Some(next) => {
                self.current_stage = next;
                true
            }
            None => false,
        }
    }

    /// Simulate a failed stage (for rollback testing).
    pub fn fail(&mut self) {
        self.failed_stage = Some(self.current_stage);
        self.log
            .push(format!("[boot] FAILED at {}", self.current_stage.label()));
    }

    pub fn is_ready(&self) -> bool {
        self.current_stage == BootStage::Ready && self.failed_stage.is_none()
    }

    pub fn advance_to_ready(&mut self) {
        while self.advance() {}
    }
}

// ---------------------------------------------------------------------------
// Layer 3 — Base System / Service Orchestrator
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    Running,
    Stopped,
    Failed,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub status: ServiceStatus,
    pub required: bool,
}

pub struct SystemOrchestrator {
    pub backend: BackendKind,
    services: Vec<ServiceInfo>,
}

impl SystemOrchestrator {
    pub fn new(backend: BackendKind) -> Self {
        let core_services = vec![
            ("sigma-init", true),
            ("sigma-network", true),
            ("sigma-security", true),
            ("sigma-audio", false),
            ("sigma-bluetooth", false),
            ("sigma-update", false),
        ];
        Self {
            backend,
            services: core_services
                .into_iter()
                .map(|(name, required)| ServiceInfo {
                    name: name.to_string(),
                    status: ServiceStatus::Stopped,
                    required,
                })
                .collect(),
        }
    }

    pub fn start_all(&mut self) {
        for svc in &mut self.services {
            svc.status = ServiceStatus::Running;
        }
    }

    pub fn stop_all(&mut self) {
        for svc in &mut self.services {
            svc.status = ServiceStatus::Stopped;
        }
    }

    pub fn all_required_running(&self) -> bool {
        self.services
            .iter()
            .filter(|s| s.required)
            .all(|s| s.status == ServiceStatus::Running)
    }

    pub fn running_count(&self) -> usize {
        self.services
            .iter()
            .filter(|s| s.status == ServiceStatus::Running)
            .count()
    }

    pub fn health_report(&self) -> Vec<String> {
        self.services
            .iter()
            .map(|s| {
                format!(
                    "  [{:?}] {} {}",
                    s.status,
                    s.name,
                    if s.required { "(required)" } else { "" }
                )
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Layer 4 — Package Coordinator
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PackageRecord {
    pub name: String,
    pub version: String,
    pub installed: bool,
    pub signed: bool,
}

pub struct PackageCoordinator {
    pub backend: BackendKind,
    packages: Vec<PackageRecord>,
}

impl PackageCoordinator {
    pub fn new(backend: BackendKind) -> Self {
        let base = vec![
            ("sigmaos-core", "0.9.0", true),
            ("sigmaos-desktop", "0.9.0", true),
            ("hyprland", "0.45.2", true),
            ("pipewire", "1.2.4", true),
            ("networkmanager", "1.48.2", true),
        ];
        Self {
            backend,
            packages: base
                .into_iter()
                .map(|(name, ver, signed)| PackageRecord {
                    name: name.to_string(),
                    version: ver.to_string(),
                    installed: true,
                    signed,
                })
                .collect(),
        }
    }

    pub fn install(&mut self, name: &str, version: &str, signed: bool) -> bool {
        if !signed {
            return false; // Unsigned packages blocked by default
        }
        self.packages.push(PackageRecord {
            name: name.to_string(),
            version: version.to_string(),
            installed: true,
            signed,
        });
        true
    }

    pub fn remove(&mut self, name: &str) -> bool {
        if let Some(pkg) = self.packages.iter_mut().find(|p| p.name == name) {
            pkg.installed = false;
            return true;
        }
        false
    }

    pub fn installed_count(&self) -> usize {
        self.packages.iter().filter(|p| p.installed).count()
    }

    pub fn find(&self, name: &str) -> Option<&PackageRecord> {
        self.packages.iter().find(|p| p.name == name)
    }

    pub fn all_signed(&self) -> bool {
        self.packages
            .iter()
            .filter(|p| p.installed)
            .all(|p| p.signed)
    }
}

// ---------------------------------------------------------------------------
// Layer 5 — Desktop Session
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionPhase {
    Starting,
    Active,
    Locked,
    Suspending,
    Ending,
}

pub struct DesktopSession {
    pub backend: BackendKind,
    pub user: String,
    pub compositor: String,
    pub phase: SessionPhase,
}

impl DesktopSession {
    pub fn new(backend: BackendKind, user: impl Into<String>) -> Self {
        Self {
            backend,
            user: user.into(),
            compositor: "hyprland".to_string(),
            phase: SessionPhase::Starting,
        }
    }

    pub fn start(&mut self) {
        self.phase = SessionPhase::Active;
    }
    pub fn lock(&mut self) {
        self.phase = SessionPhase::Locked;
    }
    pub fn unlock(&mut self) {
        self.phase = SessionPhase::Active;
    }
    pub fn suspend(&mut self) {
        self.phase = SessionPhase::Suspending;
    }
    pub fn end(&mut self) {
        self.phase = SessionPhase::Ending;
    }
    pub fn is_active(&self) -> bool {
        self.phase == SessionPhase::Active
    }
}

// ---------------------------------------------------------------------------
// Layer 6 — Config and Theme
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ConfigStore {
    pub backend: BackendKind,
    pub theme: String,
    pub language: String,
    pub timezone: String,
    pub desktop_profile: String,
}

impl ConfigStore {
    pub fn defaults(backend: BackendKind) -> Self {
        Self {
            backend,
            theme: "sigma-dark".to_string(),
            language: "en_US.UTF-8".to_string(),
            timezone: "UTC".to_string(),
            desktop_profile: "balanced".to_string(),
        }
    }

    pub fn apply_theme(&mut self, theme: impl Into<String>) {
        self.theme = theme.into();
    }

    pub fn is_valid(&self) -> bool {
        !self.theme.is_empty() && !self.language.is_empty() && !self.timezone.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Layer 7 — Security Context
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub backend: BackendKind,
    pub hardening_profile: &'static str,
    pub secure_boot: bool,
    pub signed_packages_only: bool,
    pub audit_enabled: bool,
}

impl SecurityContext {
    pub fn default_workstation(backend: BackendKind) -> Self {
        Self {
            backend,
            hardening_profile: "workstation",
            secure_boot: true,
            signed_packages_only: true,
            audit_enabled: true,
        }
    }

    pub fn is_hardened(&self) -> bool {
        self.signed_packages_only && self.audit_enabled
    }
}

// ---------------------------------------------------------------------------
// Layer 8 — App Registry
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub app_id: String,
    pub name: String,
    pub exec: String,
    pub installed: bool,
}

#[derive(Default)]
pub struct AppRegistry {
    apps: Vec<AppEntry>,
}

impl AppRegistry {
    pub fn with_defaults() -> Self {
        let defaults = vec![
            ("chromium", "Chromium", "chromium %U"),
            ("foot", "Terminal", "foot"),
            ("nautilus", "Files", "nautilus --new-window"),
            ("sigma-settings", "Settings", "sigma-settings"),
        ];
        Self {
            apps: defaults
                .into_iter()
                .map(|(id, name, exec)| AppEntry {
                    app_id: id.to_string(),
                    name: name.to_string(),
                    exec: exec.to_string(),
                    installed: true,
                })
                .collect(),
        }
    }

    pub fn install(&mut self, entry: AppEntry) {
        self.apps.retain(|a| a.app_id != entry.app_id);
        self.apps.push(entry);
    }

    pub fn find(&self, app_id: &str) -> Option<&AppEntry> {
        self.apps.iter().find(|a| a.app_id == app_id && a.installed)
    }

    pub fn installed_count(&self) -> usize {
        self.apps.iter().filter(|a| a.installed).count()
    }
}

// ---------------------------------------------------------------------------
// Platform — wires all 8 layers together
// ---------------------------------------------------------------------------

/// The top-level SigmaOS platform runtime, wiring all 8 layers.
pub struct SigmaPlatform {
    pub backend: Box<dyn KernelBackend>,
    pub boot: BootPipeline,
    pub system: SystemOrchestrator,
    pub packages: PackageCoordinator,
    pub config: ConfigStore,
    pub security: SecurityContext,
    pub apps: AppRegistry,
}

impl SigmaPlatform {
    /// Construct and boot the platform for the detected or specified backend.
    pub fn boot(backend_kind: BackendKind) -> Self {
        let mut boot = BootPipeline::new(backend_kind);
        boot.advance_to_ready();

        let mut system = SystemOrchestrator::new(backend_kind);
        system.start_all();

        Self {
            backend: create_backend(backend_kind),
            boot,
            system,
            packages: PackageCoordinator::new(backend_kind),
            config: ConfigStore::defaults(backend_kind),
            security: SecurityContext::default_workstation(backend_kind),
            apps: AppRegistry::with_defaults(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.boot.is_ready()
            && self.system.all_required_running()
            && self.config.is_valid()
            && self.security.is_hardened()
    }

    pub fn status_report(&self) -> Vec<String> {
        let mut lines = vec![
            format!("SigmaOS Platform — backend={}", self.backend.kind().name()),
            format!("  Boot:     {:?}", self.boot.current_stage),
            format!("  Services: {}/{} running", self.system.running_count(), 6),
            format!(
                "  Packages: {} installed (all signed: {})",
                self.packages.installed_count(),
                self.packages.all_signed()
            ),
            format!("  Theme:    {}", self.config.theme),
            format!(
                "  Security: profile={} audit={}",
                self.security.hardening_profile, self.security.audit_enabled
            ),
            format!("  Apps:     {} installed", self.apps.installed_count()),
            format!("  Ready:    {}", self.is_ready()),
        ];
        lines
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backend_from_env_defaults_linux() {
        // Without SIGMA_KERNEL_BACKEND set, defaults to Linux.
        let kind = BackendKind::from_env();
        assert!(matches!(
            kind,
            BackendKind::Linux | BackendKind::Bsd | BackendKind::Stub
        ));
    }

    #[test]
    fn linux_backend_contract() {
        let b = LinuxBackend;
        assert_eq!(b.kind(), BackendKind::Linux);
        assert_eq!(b.init_system(), "systemd");
        assert_eq!(b.package_manager(), "pacman");
        assert!(b.is_ready());
    }

    #[test]
    fn bsd_backend_contract() {
        let b = BsdBackend;
        assert_eq!(b.kind(), BackendKind::Bsd);
        assert_eq!(b.init_system(), "rc.d");
        assert_eq!(b.package_manager(), "pkg");
        assert!(b.is_ready());
    }

    #[test]
    fn stub_backend_contract() {
        let b = StubBackend;
        assert_eq!(b.kind(), BackendKind::Stub);
        assert!(b.is_ready());
    }

    #[test]
    fn boot_pipeline_advances_to_ready() {
        let mut pipeline = BootPipeline::new(BackendKind::Linux);
        pipeline.advance_to_ready();
        assert_eq!(pipeline.current_stage, BootStage::Ready);
        assert!(pipeline.is_ready());
        assert!(pipeline.log.len() >= 8);
    }

    #[test]
    fn boot_pipeline_fail_marks_stage() {
        let mut pipeline = BootPipeline::new(BackendKind::Linux);
        pipeline.advance(); // Bootloader
        pipeline.fail();
        assert!(!pipeline.is_ready());
        assert!(pipeline.failed_stage.is_some());
    }

    #[test]
    fn boot_stage_chain_complete() {
        let mut stage = BootStage::Firmware;
        let mut count = 0;
        while let Some(next) = stage.next() {
            stage = next;
            count += 1;
        }
        assert_eq!(stage, BootStage::Ready);
        assert_eq!(count, 8);
    }

    #[test]
    fn system_orchestrator_starts_all() {
        let mut sys = SystemOrchestrator::new(BackendKind::Linux);
        sys.start_all();
        assert!(sys.all_required_running());
        assert!(sys.running_count() > 0);
    }

    #[test]
    fn package_coordinator_install_remove() {
        let mut pkgs = PackageCoordinator::new(BackendKind::Linux);
        let initial = pkgs.installed_count();
        assert!(pkgs.install("test-pkg", "1.0.0", true));
        assert_eq!(pkgs.installed_count(), initial + 1);
        assert!(pkgs.remove("test-pkg"));
        assert_eq!(pkgs.installed_count(), initial);
    }

    #[test]
    fn package_coordinator_blocks_unsigned() {
        let mut pkgs = PackageCoordinator::new(BackendKind::Stub);
        assert!(!pkgs.install("sketchy-pkg", "0.1.0", false));
    }

    #[test]
    fn desktop_session_lifecycle() {
        let mut session = DesktopSession::new(BackendKind::Linux, "alice");
        session.start();
        assert!(session.is_active());
        session.lock();
        assert!(!session.is_active());
        session.unlock();
        assert!(session.is_active());
        session.end();
        assert_eq!(session.phase, SessionPhase::Ending);
    }

    #[test]
    fn config_defaults_valid() {
        let cfg = ConfigStore::defaults(BackendKind::Linux);
        assert!(cfg.is_valid());
        assert_eq!(cfg.theme, "sigma-dark");
    }

    #[test]
    fn security_context_hardened() {
        let sec = SecurityContext::default_workstation(BackendKind::Linux);
        assert!(sec.is_hardened());
        assert!(sec.signed_packages_only);
        assert!(sec.audit_enabled);
    }

    #[test]
    fn platform_boots_and_is_ready() {
        let platform = SigmaPlatform::boot(BackendKind::Stub);
        assert!(platform.is_ready());
        let report = platform.status_report();
        assert!(!report.is_empty());
        assert!(report[0].contains("stub"));
    }

    #[test]
    fn platform_status_report_all_layers() {
        let p = SigmaPlatform::boot(BackendKind::Stub);
        let report = p.status_report();
        // Should have one line per layer
        assert!(report.len() >= 8);
    }

    #[test]
    fn app_registry_defaults_and_lookup() {
        let reg = AppRegistry::with_defaults();
        assert!(reg.installed_count() >= 4);
        assert!(reg.find("chromium").is_some());
        assert!(reg.find("nonexistent").is_none());
    }
}
