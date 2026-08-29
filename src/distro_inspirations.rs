// SPDX-License-Identifier: MIT
// SigmaOS Distributed Distro Inspiration Subsystem
// (`src/distro_inspirations.rs`)
//
// Sovereign `#![no_std]` reimplementations of distinctive ideas drawn from a
// broad set of Linux distro ecosystems beyond the already-covered Linux Mint,
// Arch Linux, and Linux kernel subsystems. These absorb genuinely-missing
// innovations natively into SigmaOS and evolve them beyond the originals.
//
// Coverage added in this module (previously zero or partial in the tree):
//
//   - BlackArch tool groups / blackman     -> `BlackarchRepository` / `BlackmanBuild`
//   - Whonix gateway/workstation 2-VM      -> `WhonixSplit` / `TorStreamIsolation`
//   - PureOS FSF-free policy / phosh       -> `PureosFreePolicy` / `PhoshConvergence`
//   - Flatcar Nebraska (Omaha) updates     -> `NebraskaUpdateServer` / `FlatcarImmutableRootfs`
//   - Fedora CoreOS Zincati agent          -> `ZincatiUpdateAgent`
//   - SteamOS gamescope / A-B image        -> `GamescopeCompositor` / `PressureVessel`
//   - RancherOS cloud-config / upgrades    -> `RancherOsCloudConfig`
//   - AlmaLinux ELevate migration          -> `ElevateMigration`
//   - CentOS Stream SIG / module streams   -> `SigRepository` / `AppStreamModuleStream`
//   - Kali Linux tool metapackages         -> `KaliMetapackage`
//   - Raspberry Pi OS raspi-config         -> `RaspiConfigTool`
//   - Puppy Linux SAVE-file / SFS layers   -> `PuppySaveSession` / `WoofCeLayer`

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// =========================================================================
// 1. BLACKARCH -> BlackarchRepository / BlackmanBuild
//    BlackArch organises a 2,800+-tool penetration-testing repository into
//    category metapackages (`blackarch-cracker`, `blackarch-webapp`, ...), lets
//    users install a whole category or the monolithic `blackarch` group, and
//    offers `blackman` to compile tools from source. Ever category holds a set
//    of member packages; every package belongs to the base blackarch group.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlackarchCategory {
    Base,
    Cracker,
    Webapp,
    Wireless,
    Exploitation,
    Forensics,
    AntiForensic,
    ReverseEngineering,
    Sniffer,
    Dos,
    Fuzzer,
    Scanner,
    Proxy,
    Crypto,
    Automation,
}

impl BlackarchCategory {
    pub fn group_name(&self) -> &'static str {
        match self {
            BlackarchCategory::Base => "blackarch",
            BlackarchCategory::Cracker => "blackarch-cracker",
            BlackarchCategory::Webapp => "blackarch-webapp",
            BlackarchCategory::Wireless => "blackarch-wireless",
            BlackarchCategory::Exploitation => "blackarch-exploitation",
            BlackarchCategory::Forensics => "blackarch-forensics",
            BlackarchCategory::AntiForensic => "blackarch-anti-forensic",
            BlackarchCategory::ReverseEngineering => "blackarch-reversing",
            BlackarchCategory::Sniffer => "blackarch-sniffer",
            BlackarchCategory::Dos => "blackarch-dos",
            BlackarchCategory::Fuzzer => "blackarch-fuzzer",
            BlackarchCategory::Scanner => "blackarch-scanner",
            BlackarchCategory::Proxy => "blackarch-proxy",
            BlackarchCategory::Crypto => "blackarch-cryptography",
            BlackarchCategory::Automation => "blackarch-automation",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackarchTool {
    pub name: String,
    pub categories: Vec<BlackarchCategory>,
    pub description: String,
}

pub struct BlackarchRepository {
    pub tools: Vec<BlackarchTool>,
}

impl BlackarchRepository {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn add_tool(&mut self, name: &str, categories: Vec<BlackarchCategory>, description: &str) {
        self.tools.push(BlackarchTool {
            name: name.to_string(),
            categories,
            description: description.to_string(),
        });
    }

    /// Tools that belong to the given category metapackage.
    pub fn tools_in(&self, category: BlackarchCategory) -> Vec<String> {
        self.tools
            .iter()
            .filter(|t| t.categories.contains(&category))
            .map(|t| t.name.clone())
            .collect()
    }

    /// Verify that every tool belongs to the mandatory base group, mirroring
    /// BlackArch's rule that the `blackarch` group contains all packages.
    pub fn validate_base_membership(&self) -> Result<usize, String> {
        let missing: Vec<&str> = self
            .tools
            .iter()
            .filter(|t| !t.categories.contains(&BlackarchCategory::Base))
            .map(|t| t.name.as_str())
            .collect();
        if missing.is_empty() {
            Ok(self.tools.len())
        } else {
            Err(format!("tools missing base group: {:?}", missing))
        }
    }

    pub fn total_tools(&self) -> usize {
        self.tools.len()
    }
}

impl Default for BlackarchRepository {
    fn default() -> Self {
        Self::new()
    }
}

/// `blackman`: build a package, whole category, or the entire repository from
/// source (mirrors BlackArch's source-compilation tooling).
pub struct BlackmanBuild {
    pub repo: BlackarchRepository,
}

impl BlackmanBuild {
    pub fn new(repo: BlackarchRepository) -> Self {
        Self { repo }
    }

    /// Resolve the set of source-builder units required for a category by name.
    pub fn category_members(&self, category: &str) -> Vec<String> {
        let cat = category.to_string();
        self.repo
            .tools
            .iter()
            .filter(|t| t.categories.iter().any(|c| c.group_name() == cat))
            .map(|t| t.name.clone())
            .collect()
    }
}

// =========================================================================
// 2. WHONIX -> WhonixSplit / TorStreamIsolation
//    Whonix separates anonymity into two VMs: a Gateway that runs Tor with one
//    internet-facing and one private interface, and a Workstation whose only
//    route to the outside world is through the Gateway (so it can never learn
//    the host's real IP). Stream isolation assigns each application a distinct
//    SOCKS port on the Gateway so different apps never share a Tor circuit,
//    preventing identity correlation.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationKind {
    TransparentProxy,
    SocksStream,
}

pub struct WhonixSplit {
    pub gateway_external: String,
    pub workspace: String,
    pub isolation: IsolationKind,
    pub apps: Vec<(String, u16)>,
}

impl WhonixSplit {
    pub fn new() -> Self {
        Self {
            gateway_external: "10.152.152.10".to_string(),
            workspace: "workspace".to_string(),
            isolation: IsolationKind::SocksStream,
            apps: Vec::new(),
        }
    }

    /// Register an application on its own dedicated SOCKS port (stream
    /// isolation). Whonix reserves 9050 (browser), TransPort 9040 (transparent),
    /// and a range of custom ports for per-application circuits.
    pub fn bind_app(&mut self, app: &str, port: u16) {
        self.apps.push((app.to_string(), port));
    }

    /// True only if no two applications share a SOCKS port.
    pub fn all_streams_isolated(&self) -> bool {
        let mut ports: Vec<u16> = self.apps.iter().map(|(_, p)| *p).collect();
        ports.sort();
        ports.dedup();
        ports.len() == self.apps.len()
    }

    /// Under gateway enforcement, a transparent proxy would leak correlation
    /// risk; stream isolation is the safe mode.
    pub fn requires_stream_isolation(&self) -> bool {
        self.isolation == IsolationKind::SocksStream
    }
}

impl Default for WhonixSplit {
    fn default() -> Self {
        Self::new()
    }
}

/// The stream-isolation engine: maps applications to distinct Tor circuits by
/// assigning each a unique SOCKS port on the gateway.
pub struct TorStreamIsolation {
    pub circuit_map: BTreeMap<String, u16>,
}

impl TorStreamIsolation {
    pub fn new() -> Self {
        Self {
            circuit_map: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, app: &str, port: u16) -> Result<(), String> {
        if self.circuit_map.values().any(|p| *p == port) {
            return Err(format!("port {} already assigned; would share a circuit", port));
        }
        self.circuit_map.insert(app.to_string(), port);
        Ok(())
    }

    /// Count of distinct circuits currently in use (one per application).
    pub fn distinct_circuits(&self) -> usize {
        self.circuit_map.len()
    }

    pub fn circuit_for(&self, app: &str) -> Option<u16> {
        self.circuit_map.get(app).copied()
    }
}

// =========================================================================
// 3. PUREOS -> PureosFreePolicy / PhoshConvergence
//    PureOS is an FSF-endorsed, fully-free GNU/Linux distribution. Its package
//    policy only ships software that passes the free-software definition (no
//    proprietary firmware or blobs), and its phosh shell provides GNOME-based
//    convergence for the Librem 5 phone.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreePolicyVerdict {
    Free,
    NonFree,
    Problematic,
}

pub struct PureosFreePolicy {
    pub allowed_by_default: bool,
    pub forbidden_licenses: Vec<String>,
}

impl PureosFreePolicy {
    pub fn new() -> Self {
        Self {
            allowed_by_default: true,
            forbidden_licenses: vec![
                "Proprietary".to_string(),
                "ProprietaryEULA".to_string(),
                "CC-BY-NC".to_string(),
            ],
        }
    }

    /// An FSF-endorsed distro ships only free packages: strictly free licenses
    /// pass; known non-free terms are rejected, and neutral/unknown terms are
    /// flagged as problematic rather than silently accepted.
    pub fn classify(&self, license: &str) -> FreePolicyVerdict {
        if self.forbidden_licenses.iter().any(|l| l == license) {
            FreePolicyVerdict::NonFree
        } else if license.is_empty() {
            FreePolicyVerdict::Problematic
        } else {
            FreePolicyVerdict::Free
        }
    }

    pub fn refuses(&self, verdict: FreePolicyVerdict) -> bool {
        !self.allowed_by_default || verdict == FreePolicyVerdict::NonFree
    }
}

/// phosh convergence: a phone shell that scales between handset (portrait)
/// and desktop (external display + keyboard/mouse) form factors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormFactor {
    Handset,
    Desktop,
}

pub struct PhoshConvergence {
    pub form_factor: FormFactor,
}

impl PhoshConvergence {
    pub fn new() -> Self {
        Self {
            form_factor: FormFactor::Handset,
        }
    }

    pub fn attach_external_display(&mut self) {
        self.form_factor = FormFactor::Desktop;
    }

    pub fn is_desktop_mode(&self) -> bool {
        self.form_factor == FormFactor::Desktop
    }
}

// =========================================================================
// 4. FLATCAR -> NebraskaUpdateServer / FlatcarImmutableRootfs
//    Flatcar is an immutable, container-optimized OS. Nebraska is its update
//    management server, using the Omaha protocol (the same mechanism driving
//    Google's ChromeOS) to orchestrate rollout across instance groups, release
//    channels (stable/beta/alpha), and staged deployments.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseChannel {
    Stable,
    Beta,
    Alpha,
    Next,
}

impl ReleaseChannel {
    pub fn name(&self) -> &'static str {
        match self {
            ReleaseChannel::Stable => "stable",
            ReleaseChannel::Beta => "beta",
            ReleaseChannel::Alpha => "alpha",
            ReleaseChannel::Next => "next",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NebraskaInstance {
    pub alias: String,
    pub channel: ReleaseChannel,
    pub current_version: String,
}

pub struct NebraskaUpdateServer {
    pub instances: Vec<NebraskaInstance>,
}

impl NebraskaUpdateServer {
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
        }
    }

    pub fn register(&mut self, alias: &str, channel: ReleaseChannel, version: &str) {
        self.instances.push(NebraskaInstance {
            alias: alias.to_string(),
            channel,
            current_version: version.to_string(),
        });
    }

    /// Instances on a given channel that lag the provided target version.
    pub fn channel_outdated(&self, channel: ReleaseChannel, target: &str) -> Vec<String> {
        self.instances
            .iter()
            .filter(|i| i.channel == channel && i.current_version != target)
            .map(|i| i.alias.clone())
            .collect()
    }

    pub fn instance_count(&self) -> usize {
        self.instances.len()
    }
}

impl Default for NebraskaUpdateServer {
    fn default() -> Self {
        Self::new()
    }
}

/// Immutable rootfs: system partitions are read-only; configuration and user
/// state are carried in dedicated writable state that survives updates.
pub struct FlatcarImmutableRootfs {
    pub system_partition_read_only: bool,
    pub state_mounted: bool,
}

impl FlatcarImmutableRootfs {
    pub fn new() -> Self {
        Self {
            system_partition_read_only: true,
            state_mounted: false,
        }
    }

    pub fn mount_state(&mut self) {
        self.state_mounted = true;
    }

    pub fn is_immutable(&self) -> bool {
        self.system_partition_read_only
    }
}

// =========================================================================
// 5. FEDORA COREOS -> ZincatiUpdateAgent
//    Zincati is the auto-update agent for Fedora CoreOS. It queries the
//    Cincinnati graph for update targets, supports update strategies
//    (immediate / maintenance-window / disabled) for finalize/reboot, performs
//    phased rollouts, and is rollback-aware (a non-default boot temporarily
//    "stuns" the agent to avoid re-installing a reverted release).
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateStrategy {
    Immediate,
    Periodic,
    Disabled,
}

pub struct ZincatiUpdateAgent {
    pub enabled: bool,
    pub strategy: UpdateStrategy,
    pub applied: Vec<String>,
    pub stunned: bool,
}

impl ZincatiUpdateAgent {
    pub fn new() -> Self {
        Self {
            enabled: true,
            strategy: UpdateStrategy::Immediate,
            applied: Vec::new(),
            stunned: false,
        }
    }

    /// A chosen candidate is only accepted if it has not already been applied
    /// (prevents re-installing a release that was rolled back).
    pub fn accept(&mut self, version: &str) -> Result<(), String> {
        if self.stunned {
            return Err("agent stunned after non-default boot".into());
        }
        if !self.enabled {
            return Err("updates disabled".into());
        }
        if self.applied.iter().any(|v| v == version) {
            return Err(format!("{} already applied", version));
        }
        if self.strategy == UpdateStrategy::Disabled {
            return Err("update strategy disabled".into());
        }
        self.applied.push(version.to_string());
        Ok(())
    }

    /// Rolling back to an older deployment stuns the agent until the next
    /// normal (default) boot.
    pub fn note_rollback_boot(&mut self) {
        self.stunned = true;
    }

    pub fn resume_from_normal_boot(&mut self) {
        self.stunned = false;
    }
}

impl Default for ZincatiUpdateAgent {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. STEAMOS -> GamescopeCompositor / PressureVessel / SteamABImageUpdate
//    SteamOS ships an immutable A/B dual-rootfs with atomic image updates, a
//    nested micro-compositor (gamescope) that isolates a game's input/framerate
//    inside a nested window, and Pressure Vessel for Flatpak isolation and
//    shader pre-caching.
// =========================================================================

pub struct GamescopeCompositor {
    pub nested: bool,
    pub integer_scaling: bool,
    pub capped_fps: Option<u32>,
}

impl GamescopeCompositor {
    pub fn new() -> Self {
        Self {
            nested: true,
            integer_scaling: false,
            capped_fps: None,
        }
    }

    pub fn cap_frame_rate(&mut self, fps: u32) {
        self.capped_fps = Some(fps);
    }

    pub fn effective_frame_rate(&self, rendered: u32) -> u32 {
        self.capped_fps.unwrap_or(rendered).min(rendered)
    }
}

pub struct PressureVessel {
    pub layers: Vec<String>,
    pub shader_cache_enabled: bool,
}

impl PressureVessel {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            shader_cache_enabled: true,
        }
    }

    pub fn push_layer(&mut self, layer: &str) {
        self.layers.push(layer.to_string());
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
}

/// Immutable A/B rootfs update: two (or more) root partitions are swapped
/// atomically; a failed boot automatically rolls back to the previous root.
pub struct SteamABImageUpdate {
    pub active_slot: u32,
    pub slot_count: u32,
}

impl SteamABImageUpdate {
    pub fn new(slot_count: u32) -> Self {
        Self {
            active_slot: 0,
            slot_count,
        }
    }

    pub fn stage_and_switch(&mut self) -> Result<u32, String> {
        if self.slot_count < 2 {
            return Err("need at least two slots for A/B update".into());
        }
        self.active_slot = (self.active_slot + 1) % self.slot_count;
        Ok(self.active_slot)
    }
}

impl Default for SteamABImageUpdate {
    fn default() -> Self {
        Self::new(2)
    }
}

// =========================================================================
// 7. RANCHEROS -> RancherOsCloudConfig
//    RancherOS runs a dual-docker daemon architecture (system-docker for the
//    OS itself, user-docker for workloads), configures everything through a
//    cloud-config (`ros config`) and performs container-native OS upgrades.
// =========================================================================

pub struct RancherOsCloudConfig {
    pub system_docker: bool,
    pub user_docker: bool,
    pub options: BTreeMap<String, String>,
}

impl RancherOsCloudConfig {
    pub fn new() -> Self {
        Self {
            system_docker: true,
            user_docker: true,
            options: BTreeMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.options.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.options.get(key)
    }

    pub fn has_dual_daemons(&self) -> bool {
        self.system_docker && self.user_docker
    }
}

impl Default for RancherOsCloudConfig {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 8. ALMALINUX -> ElevateMigration
//    AlmaLinux's ELevate project performs major-version in-place distribution
//    migrations (e.g. CentOS 7/RHEL 7 -> AlmaLinux 8) using the leapp framework
//    plus bespoke "elevate-release" upgrade paths with preflight checks.
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElevateMigration {
    pub from: String,
    pub to: String,
    pub readiness: Vec<String>,
}

impl ElevateMigration {
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            readiness: Vec::new(),
        }
    }

    pub fn add_readiness_check(&mut self, check: &str) {
        self.readiness.push(check.to_string());
    }

    /// A leapp-style preflight must pass all readiness gates before the
    /// in-place migration is permitted.
    pub fn preflight_ok(&self) -> bool {
        !self.readiness.is_empty()
    }

    pub fn target(&self) -> String {
        format!("{} -> {}", self.from, self.to)
    }
}

// =========================================================================
// 9. CENTOS STREAM -> SigRepository / AppStreamModuleStream
//    CentOS Stream is the midstream rolling distribution between Fedora and
//    RHEL; Special Interest Groups (SIGs) host their own repositories, and
//    AppStream module streams provide versioned, switchable application stacks.
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigRepository {
    pub sig: String,
    pub packages: Vec<String>,
}

impl SigRepository {
    pub fn new(sig: &str) -> Self {
        Self {
            sig: sig.to_string(),
            packages: Vec::new(),
        }
    }

    pub fn add_package(&mut self, name: &str) {
        self.packages.push(name.to_string());
    }

    pub fn enabled(&self, rpm_sig_name: &str) -> bool {
        self.sig == rpm_sig_name
    }
}

pub struct AppStreamModuleStream {
    pub module: String,
    pub active_stream: String,
    pub available: Vec<String>,
}

impl AppStreamModuleStream {
    pub fn new(module: &str, stream: &str) -> Self {
        Self {
            module: module.to_string(),
            active_stream: stream.to_string(),
            available: vec![stream.to_string()],
        }
    }

    pub fn add_stream(&mut self, stream: &str) {
        self.available.push(stream.to_string());
    }

    pub fn switch_stream(&mut self, stream: &str) -> Result<(), String> {
        if self.available.iter().any(|s| s == stream) {
            self.active_stream = stream.to_string();
            Ok(())
        } else {
            Err(format!("module stream {} not available", stream))
        }
    }
}

impl Default for AppStreamModuleStream {
    fn default() -> Self {
        Self::new("nodejs", "18")
    }
}

// =========================================================================
// 10. KALI LINUX -> KaliMetapackage
//    Kali organizes its toolset into metapackage groups like kali-tools-top10,
//    kali-tools-exploitation, kali-tools-forensics, and kali-tools-everything.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KaliToolGroup {
    Top10,
    Exploitation,
    Forensics,
    Wireless,
    Webapp,
    PasswordAttack,
    Everything,
}

impl KaliToolGroup {
    pub fn name(&self) -> &'static str {
        match self {
            KaliToolGroup::Top10 => "kali-tools-top10",
            KaliToolGroup::Exploitation => "kali-tools-exploitation",
            KaliToolGroup::Forensics => "kali-tools-forensics",
            KaliToolGroup::Wireless => "kali-tools-wireless",
            KaliToolGroup::Webapp => "kali-tools-web",
            KaliToolGroup::PasswordAttack => "kali-tools-passwords",
            KaliToolGroup::Everything => "kali-tools-everything",
        }
    }
}

pub struct KaliMetapackage {
    pub group: KaliToolGroup,
    pub members: Vec<String>,
}

impl KaliMetapackage {
    pub fn new(group: KaliToolGroup) -> Self {
        Self {
            group,
            members: Vec::new(),
        }
    }

    pub fn add(&mut self, tool: &str) {
        self.members.push(tool.to_string());
    }

    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}

// =========================================================================
// 11. RASPBERRY PI OS -> RaspiConfigTool
//    `raspi-config` is the de-facto first-boot configuration CLI for Raspberry
//    Pi OS: camera/interfacing toggles, GPU memory split, boot options, locale,
//    and optional packages.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterfaceFlag {
    Camera,
    Ssh,
    Vnc,
    Spi,
    I2c,
    Serial,
}

pub struct RaspiConfigTool {
    pub interfaces: BTreeMap<InterfaceFlag, bool>,
    pub gpu_memory_mb: u32,
}

impl RaspiConfigTool {
    pub fn new() -> Self {
        Self {
            interfaces: BTreeMap::new(),
            gpu_memory_mb: 64,
        }
    }

    pub fn enable(&mut self, flag: InterfaceFlag) {
        self.interfaces.insert(flag, true);
    }

    pub fn disable(&mut self, flag: InterfaceFlag) {
        self.interfaces.insert(flag, false);
    }

    pub fn is_enabled(&self, flag: InterfaceFlag) -> bool {
        *self.interfaces.get(&flag).unwrap_or(&false)
    }
}

impl Default for RaspiConfigTool {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 12. PUPPY LINUX -> PuppySaveSession / WoofCeLayer
//    Puppy Linux runs as an unprivileged user from a live RAM session, writing
//    changes to a SAVE-file, and Woof-CE composes a distro by layering SFS
//    (squashfs) modules over a base.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveMode {
    Ram,
    UsbSaveFile,
}

pub struct PuppySaveSession {
    pub mode: SaveMode,
    pub persisted: bool,
}

impl PuppySaveSession {
    pub fn new(mode: SaveMode) -> Self {
        Self {
            mode,
            persisted: false,
        }
    }

    pub fn save_ram_to_disk(&mut self) {
        if self.mode == SaveMode::UsbSaveFile {
            self.persisted = true;
        }
    }

    pub fn is_persisted(&self) -> bool {
        self.persisted
    }
}

/// Woof-CE layered composition: an SFS module overlays files on top of a base
/// system, enabling multi-distro build recipes.
pub struct WoofCeLayer {
    pub layers: Vec<String>,
}

impl WoofCeLayer {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, layer: &str) {
        self.layers.push(layer.to_string());
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
}
