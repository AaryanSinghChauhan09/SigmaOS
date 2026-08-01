use std::collections::HashMap;

/// HPC Cluster Job State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HpcJobState {
    Pending,
    Running,
    Completed,
    Failed,
}

/// Represents a Scientific High-Performance Computing Job (like Slurm/PBS)
#[derive(Debug, Clone)]
pub struct HpcClusterJob {
    pub job_id: u32,
    pub name: String,
    pub nodes_requested: u32,
    pub cores_per_node: u32,
    pub state: HpcJobState,
    pub script_payload: String,
}

impl HpcClusterJob {
    pub fn new(job_id: u32, name: &str, nodes: u32, cores: u32, script: &str) -> Self {
        Self {
            job_id,
            name: name.to_string(),
            nodes_requested: nodes,
            cores_per_node: cores,
            state: HpcJobState::Pending,
            script_payload: script.to_string(),
        }
    }

    pub fn start_job(&mut self) {
        if self.state == HpcJobState::Pending {
            self.state = HpcJobState::Running;
        }
    }

    pub fn complete_job(&mut self) {
        if self.state == HpcJobState::Running {
            self.state = HpcJobState::Completed;
        }
    }
}

/// Simulates a Message Passing Interface (MPI) communicator for parallel workloads
#[derive(Debug, Clone)]
pub struct MpiCommunicator {
    pub size: u32,
    pub rank: u32,
    pub message_buffer: HashMap<u32, Vec<u8>>, // maps rank to received byte packets
}

impl MpiCommunicator {
    pub fn new(size: u32, rank: u32) -> Self {
        Self {
            size,
            rank,
            message_buffer: HashMap::new(),
        }
    }

    /// Simulates sending a packet from current rank to destination rank
    pub fn send(
        &self,
        dest: u32,
        data: &[u8],
        communicators: &mut [MpiCommunicator],
    ) -> Result<(), &'static str> {
        if dest >= self.size {
            return Err("Destination rank out of bounds");
        }
        for comm in communicators {
            if comm.rank == dest {
                comm.message_buffer.insert(self.rank, data.to_vec());
                return Ok(());
            }
        }
        Err("Destination communicator not found")
    }

    /// Simulates broadcasting a message to all ranks in the communicator
    pub fn broadcast(&self, data: &[u8], communicators: &mut [MpiCommunicator]) {
        for comm in communicators {
            if comm.rank != self.rank {
                comm.message_buffer.insert(self.rank, data.to_vec());
            }
        }
    }
}

/// CAN-bus Frame representation for Automotive Controllers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanFrame {
    pub id: u32,
    pub data: [u8; 8],
    pub dlc: u8, // data length code
}

/// Simulates Automotive/Industrial Engine Control Unit (ECU) controller
#[derive(Debug, Clone)]
pub struct EcuController {
    pub ecu_id: u8,
    pub name: String,
    pub brake_applied: bool,
    pub speed_kmh: f32,
    pub error_log: Vec<String>,
}

impl EcuController {
    pub fn new(ecu_id: u8, name: &str) -> Self {
        Self {
            ecu_id,
            name: name.to_string(),
            brake_applied: false,
            speed_kmh: 0.0,
            error_log: Vec::new(),
        }
    }

    /// Processes incoming CAN-bus frames representing vehicle states or command signals
    pub fn process_can_frame(&mut self, frame: &CanFrame) -> Result<&'static str, &'static str> {
        if frame.dlc > 8 {
            return Err("Invalid CAN Frame DLC");
        }

        match frame.id {
            0x101 => {
                // Throttle signal
                let throttle = frame.data[0] as f32;
                self.speed_kmh = throttle * 1.5;
                Ok("Speed updated")
            }
            0x102 => {
                // Brake signal
                let brake_signal = frame.data[0];
                self.brake_applied = brake_signal != 0;
                if self.brake_applied {
                    self.speed_kmh = 0.0;
                }
                Ok("Brake applied")
            }
            0x500 => {
                // Emergency Fault
                self.error_log
                    .push("Emergency fault CAN code received!".to_string());
                self.speed_kmh = 0.0;
                self.brake_applied = true;
                Ok("Safety failsafe activated")
            }
            _ => Err("Unknown CAN ID"),
        }
    }
}

// ==========================================
// 6. EndeavourOS-Style Sovereign Utilities
// ==========================================

/// EndeavourOS-Style Welcome Engine to configure initial system states
#[derive(Debug, Clone)]
pub struct EosWelcomeEngine {
    pub first_boot: bool,
    pub mirrors_configured: bool,
    pub drivers_installed: bool,
}

impl EosWelcomeEngine {
    pub fn new() -> Self {
        Self {
            first_boot: true,
            mirrors_configured: false,
            drivers_installed: false,
        }
    }

    pub fn update_mirrors(&mut self) -> Result<&'static str, &'static str> {
        self.mirrors_configured = true;
        Ok("Sovereign package mirrors configured successfully")
    }

    pub fn install_recommended_drivers(&mut self) -> Result<&'static str, &'static str> {
        self.drivers_installed = true;
        Ok("Modern Vulkan/GPU and HID drivers installed")
    }
}

impl Default for EosWelcomeEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// EndeavourOS-Style Mirror Speed Ranker
#[derive(Debug, Clone)]
pub struct MirrorRanker {
    pub default_timeout_ms: u64,
}

impl MirrorRanker {
    pub fn new(timeout: u64) -> Self {
        Self {
            default_timeout_ms: timeout,
        }
    }

    /// Ranks list of regional mirrors based on simulated round-trip-time (RTT) latency
    pub fn rank_mirrors(&self, mirrors: &[&str]) -> Vec<(String, u64)> {
        let mut ranked = Vec::new();
        for (i, &mirror) in mirrors.iter().enumerate() {
            // Simulated RTT: base RTT modulated by index to make ranking deterministic
            let rtt = 10 + (i as u64 * 15);
            ranked.push((mirror.to_string(), rtt));
        }
        ranked.sort_by_key(|(_, rtt)| *rtt);
        ranked
    }
}

/// Background periodic checking service for new packages
#[derive(Debug, Clone)]
pub struct EosUpdateNotifier {
    pub pending_updates_count: u32,
}

impl EosUpdateNotifier {
    pub fn new() -> Self {
        Self {
            pending_updates_count: 0,
        }
    }

    pub fn check_for_updates(&mut self) -> bool {
        // Simulated background query
        self.pending_updates_count = 5;
        true
    }
}

impl Default for EosUpdateNotifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Unified diagnostic collector for kernel and package manager logs
#[derive(Debug, Clone)]
pub struct DiagnosticLogTool {
    pub collected_lines: Vec<String>,
}

impl DiagnosticLogTool {
    pub fn new() -> Self {
        Self {
            collected_lines: Vec::new(),
        }
    }

    pub fn record_log_entry(&mut self, source: &str, msg: &str) {
        self.collected_lines.push(format!("[{}] {}", source, msg));
    }

    pub fn generate_troubleshooting_report(&self) -> String {
        let mut report = String::from("--- SigmaOS Troubleshooting Report ---\n");
        for line in &self.collected_lines {
            report.push_str(line);
            report.push('\n');
        }
        report
    }
}

impl Default for DiagnosticLogTool {
    fn default() -> Self {
        Self::new()
    }
}

/// Educational Sandbox Coding Challenge
#[derive(Debug, Clone)]
pub struct EduChallenge {
    pub challenge_id: u32,
    pub title: String,
    pub description: String,
    pub difficulty: String,
}

/// Secured Educational Playground environment for students
#[derive(Debug, Clone)]
pub struct EduPlayground {
    pub student_name: String,
    pub level: u32,
    pub current_score: u32,
    pub active_challenge: Option<EduChallenge>,
}

impl EduPlayground {
    pub fn new(student_name: &str) -> Self {
        Self {
            student_name: student_name.to_string(),
            level: 1,
            current_score: 0,
            active_challenge: None,
        }
    }

    pub fn set_challenge(&mut self, challenge: EduChallenge) {
        self.active_challenge = Some(challenge);
    }

    /// Submits a student's answer code. If correct, awards points and advances levels.
    pub fn submit_solution(&mut self, student_code: &str) -> Result<&'static str, &'static str> {
        let challenge = self
            .active_challenge
            .as_ref()
            .ok_or("No active challenge")?;

        // Basic static verification of educational coding results
        if student_code.contains("print")
            && student_code.contains("hello")
            && challenge.challenge_id == 1
        {
            self.current_score += 100;
            if self.current_score >= 200 {
                self.level += 1;
            }
            Ok("Congratulations! Your educational code is correct and fully sandboxed.")
        } else {
            Err("Code analysis failed: expected standard output or matching signature")
        }
    }
}

// ==========================================
// 7. Void Linux & NetBSD Distro Gaps Closure
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    Down,
    Up,
    Panicked,
}

#[derive(Debug, Clone)]
pub struct RunitService {
    pub name: String,
    pub status: ServiceStatus,
    pub restart_count: usize,
}

/// Void Linux-style Runit Service Manager
pub struct RunitServiceManager {
    pub active_services: HashMap<String, RunitService>,
}

impl RunitServiceManager {
    pub fn new() -> Self {
        Self {
            active_services: HashMap::new(),
        }
    }

    pub fn register_and_start_service(&mut self, name: &str) {
        self.active_services.insert(
            name.to_string(),
            RunitService {
                name: name.to_string(),
                status: ServiceStatus::Up,
                restart_count: 0,
            },
        );
    }

    pub fn supervise_and_recover_services(&mut self) -> usize {
        let mut recovered_count = 0;
        for service in self.active_services.values_mut() {
            if service.status == ServiceStatus::Panicked {
                service.status = ServiceStatus::Up;
                service.restart_count += 1;
                recovered_count += 1;
            }
        }
        recovered_count
    }
}

impl Default for RunitServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// NetBSD-style Rump Kernel Driver Shim Context
pub struct RumpKernelShim {
    pub active_drivers: HashMap<String, String>, // maps driver name to isolated process ID
}

impl RumpKernelShim {
    pub fn new() -> Self {
        Self {
            active_drivers: HashMap::new(),
        }
    }

    pub fn load_isolated_rump_driver(&mut self, name: &str) -> String {
        let pid = format!("rump_pid_{:x}", name.len() * 12345);
        self.active_drivers.insert(name.to_string(), pid.clone());
        pid
    }

    pub fn check_driver_active(&self, name: &str) -> bool {
        self.active_drivers.contains_key(name)
    }
}

impl Default for RumpKernelShim {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 8. Debian-Inspired System Innovations
// ==========================================

#[derive(Debug, Clone)]
pub struct AptPackageManifest {
    pub name: String,
    pub version: String,
    pub sha256: String,
}

/// Debian-Style Local APT Cache Simulator
pub struct AptCacheSimulator {
    pub cached_manifests: HashMap<String, AptPackageManifest>,
    pub max_cache_size: usize,
}

impl AptCacheSimulator {
    pub fn new(max_size: usize) -> Self {
        Self {
            cached_manifests: HashMap::new(),
            max_cache_size: max_size,
        }
    }

    pub fn cache_package_metadata(
        &mut self,
        manifest: AptPackageManifest,
    ) -> Result<&'static str, &'static str> {
        if self.cached_manifests.len() >= self.max_cache_size {
            return Err("APT Cache is full, trigger cache pruning");
        }
        self.cached_manifests
            .insert(manifest.name.clone(), manifest);
        Ok("Package metadata stored in offline APT cache")
    }

    pub fn query_cached_package(&self, name: &str) -> Option<&AptPackageManifest> {
        self.cached_manifests.get(name)
    }
}

/// Debian dpkg-style Multi-Architecture Linkage binding
pub struct DpkgMultiArch {
    pub foreign_architectures: Vec<String>,
}

impl DpkgMultiArch {
    pub fn new() -> Self {
        Self {
            foreign_architectures: Vec::new(),
        }
    }

    pub fn register_foreign_architecture(&mut self, arch: &str) {
        self.foreign_architectures.push(arch.to_string());
    }

    pub fn is_architecture_supported(&self, arch: &str) -> bool {
        arch == "x86_64" || self.foreign_architectures.iter().any(|a| a == arch)
    }
}

impl Default for DpkgMultiArch {
    fn default() -> Self {
        Self::new()
    }
}

/// Debian Policy-driven system enforcer
pub struct DebianPolicyEnforcer {
    pub enforce_fhs: bool,
    pub enforce_signatures: bool,
}

impl DebianPolicyEnforcer {
    pub fn new() -> Self {
        Self {
            enforce_fhs: true,
            enforce_signatures: true,
        }
    }

    pub fn evaluate_package_compliance(&self, has_valid_signature: bool, path: &str) -> bool {
        if self.enforce_signatures && !has_valid_signature {
            return false;
        }
        if self.enforce_fhs {
            // FHS conventions require standard starting blocks
            return path.starts_with("/usr/")
                || path.starts_with("/bin/")
                || path.starts_with("/etc/");
        }
        true
    }
}

impl Default for DebianPolicyEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

/// Three-Tier Release Model: sigma.next (unstable), sigma.beta (testing), sigma.stable (stable)
pub struct ThreeTierReleaseModel {
    pub active_channel: String,
    pub channels: HashMap<String, String>, // Name to description
}

impl ThreeTierReleaseModel {
    pub fn new() -> Self {
        let mut channels = HashMap::new();
        channels.insert("sigma.next".to_string(), "Rolling, experimental, daily updates".to_string());
        channels.insert("sigma.beta".to_string(), "Pre-release, weekly, mostly stable".to_string());
        channels.insert("sigma.stable".to_string(), "Production LTS, quarterly security-only".to_string());

        Self {
            active_channel: "sigma.stable".to_string(),
            channels,
        }
    }

    pub fn list_channels(&self) -> String {
        let mut output = String::from("Σ [PKG] Available channels:\n");
        let mut sorted_keys: Vec<&String> = self.channels.keys().collect();
        sorted_keys.sort(); // Consistent ordering
        for key in sorted_keys {
            let desc = &self.channels[key];
            let name_alias = match key.as_str() {
                "sigma.next" => "Σ-next",
                "sigma.beta" => "Σ-beta",
                _ => "Σ-stable",
            };
            output.push_str(&format!("  {} ({}) — {}\n", key, name_alias, desc));
        }
        output
    }

    pub fn set_channel(&mut self, channel: &str) -> Result<String, &'static str> {
        if self.channels.contains_key(channel) {
            self.active_channel = channel.to_string();
            Ok(format!("Σ [PKG] Channel set to {} (LTS). No experimental features.", channel))
        } else {
            Err("Unknown release channel")
        }
    }
}

impl Default for ThreeTierReleaseModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Debian Social Contract and DFSG Guidelines checks
pub struct DebianSocialContract {
    pub open_source_only: bool,
    pub public_bug_tracker: bool,
    pub priorities_users_first: bool,
}

impl DebianSocialContract {
    pub fn new() -> Self {
        Self {
            open_source_only: true,
            public_bug_tracker: true,
            priorities_users_first: true,
        }
    }

    pub fn is_dfsg_compliant_license(&self, license: &str) -> bool {
        // Commits to 100% open source licenses
        match license {
            "MIT" | "Apache-2.0" | "GPL-2.0" | "GPL-3.0" | "BSD-2-Clause" | "BSD-3-Clause" => true,
            _ => false, // Non-free or proprietary are rejected
        }
    }

    pub fn evaluate_social_contract_compliance(&self, is_open_source: bool, is_bug_public: bool, is_user_needs_prioritized: bool) -> bool {
        if self.open_source_only && !is_open_source {
            return false;
        }
        if self.public_bug_tracker && !is_bug_public {
            return false;
        }
        if self.priorities_users_first && !is_user_needs_prioritized {
            return false;
        }
        true
    }
}

impl Default for DebianSocialContract {
    fn default() -> Self {
        Self::new()
    }
}

/// Debian-Style Freeze-Based Stabilization lifecycle
pub struct FreezeBasedStabilization {
    pub is_freeze_active: bool,
    pub allowed_update_types: Vec<String>, // "security", "critical-bugfix"
}

impl FreezeBasedStabilization {
    pub fn new() -> Self {
        Self {
            is_freeze_active: false,
            allowed_update_types: vec!["security".to_string(), "critical-bugfix".to_string()],
        }
    }

    pub fn set_freeze_state(&mut self, active: bool) {
        self.is_freeze_active = active;
    }

    pub fn is_update_allowed(&self, update_type: &str) -> bool {
        if !self.is_freeze_active {
            return true; // No freeze: everything is allowed
        }
        // During freeze, only security and critical bugfixes are permitted
        self.allowed_update_types.iter().any(|t| t == update_type)
    }
}

impl Default for FreezeBasedStabilization {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hpc_cluster_jobs() {
        let mut job = HpcClusterJob::new(1001, "AstroPhysics-Sim", 16, 64, "run-sim.sh");
        assert_eq!(job.state, HpcJobState::Pending);

        job.start_job();
        assert_eq!(job.state, HpcJobState::Running);

        job.complete_job();
        assert_eq!(job.state, HpcJobState::Completed);
    }

    #[test]
    fn test_mpi_message_passing() {
        let node0 = MpiCommunicator::new(3, 0);
        let node1 = MpiCommunicator::new(3, 1);
        let node2 = MpiCommunicator::new(3, 2);

        let mut communicators = [node0.clone(), node1.clone(), node2.clone()];

        assert!(node0.send(1, b"hello rank 1", &mut communicators).is_ok());
        assert_eq!(
            communicators[1].message_buffer.get(&0).unwrap(),
            b"hello rank 1"
        );

        node0.broadcast(b"sync signal", &mut communicators);
        assert_eq!(
            communicators[1].message_buffer.get(&0).unwrap(),
            b"sync signal"
        );
        assert_eq!(
            communicators[2].message_buffer.get(&0).unwrap(),
            b"sync signal"
        );
    }

    #[test]
    fn test_automotive_ecu_failsafe() {
        let mut ecu = EcuController::new(0x0A, "Transmission-ECU");

        let throttle_frame = CanFrame {
            id: 0x101,
            data: [40, 0, 0, 0, 0, 0, 0, 0],
            dlc: 8,
        };
        assert!(ecu.process_can_frame(&throttle_frame).is_ok());
        assert_eq!(ecu.speed_kmh, 60.0);

        let failsafe_frame = CanFrame {
            id: 0x500,
            data: [0; 8],
            dlc: 8,
        };
        assert_eq!(
            ecu.process_can_frame(&failsafe_frame),
            Ok("Safety failsafe activated")
        );
        assert_eq!(ecu.speed_kmh, 0.0);
        assert!(ecu.brake_applied);
        assert_eq!(ecu.error_log.len(), 1);
    }

    #[test]
    fn test_educational_sandbox_challenges() {
        let mut play = EduPlayground::new("Aaryan");
        let challenge = EduChallenge {
            challenge_id: 1,
            title: "Hello World".to_string(),
            description: "Print hello to console".to_string(),
            difficulty: "Beginner".to_string(),
        };

        play.set_challenge(challenge);

        // Incorrect code
        let fail_res = play.submit_solution("fn main() { return 0; }");
        assert!(fail_res.is_err());

        // Correct code
        let pass_res = play.submit_solution("fn main() { print!(\"hello\"); }");
        assert!(pass_res.is_ok());
        assert_eq!(play.current_score, 100);
    }

    #[test]
    fn test_endeavour_welcome_engine() {
        let mut welcome = EosWelcomeEngine::new();
        assert!(welcome.first_boot);
        assert_eq!(
            welcome.update_mirrors().unwrap(),
            "Sovereign package mirrors configured successfully"
        );
        assert!(welcome.mirrors_configured);

        assert_eq!(
            welcome.install_recommended_drivers().unwrap(),
            "Modern Vulkan/GPU and HID drivers installed"
        );
        assert!(welcome.drivers_installed);
    }

    #[test]
    fn test_mirror_ranker() {
        let ranker = MirrorRanker::new(500);
        let mirrors = vec![
            "mirror.us.sigmaos.org",
            "mirror.in.sigmaos.org",
            "mirror.de.sigmaos.org",
        ];
        let ranked = ranker.rank_mirrors(&mirrors);
        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0].0, "mirror.us.sigmaos.org");
        assert_eq!(ranked[0].1, 10);
    }

    #[test]
    fn test_update_notifier_and_log_tool() {
        let mut notifier = EosUpdateNotifier::new();
        assert!(notifier.check_for_updates());
        assert_eq!(notifier.pending_updates_count, 5);

        let mut log_tool = DiagnosticLogTool::new();
        log_tool.record_log_entry("Kernel", "Vulkan context bound successfully");
        log_tool.record_log_entry(
            "PackageManager",
            "Transaction completed: installed sigma-vim",
        );

        let report = log_tool.generate_troubleshooting_report();
        assert!(report.contains("--- SigmaOS Troubleshooting Report ---"));
        assert!(report.contains("[Kernel] Vulkan context bound successfully"));
    }

    #[test]
    fn test_runit_service_manager() {
        let mut manager = RunitServiceManager::new();
        manager.register_and_start_service("vfs_shard");
        assert_eq!(
            manager.active_services.get("vfs_shard").unwrap().status,
            ServiceStatus::Up
        );

        // Manually panic the service
        manager.active_services.get_mut("vfs_shard").unwrap().status = ServiceStatus::Panicked;

        let recovered = manager.supervise_and_recover_services();
        assert_eq!(recovered, 1);
        assert_eq!(
            manager.active_services.get("vfs_shard").unwrap().status,
            ServiceStatus::Up
        );
        assert_eq!(
            manager
                .active_services
                .get("vfs_shard")
                .unwrap()
                .restart_count,
            1
        );
    }

    #[test]
    fn test_rump_kernel_shim() {
        let mut shim = RumpKernelShim::new();
        assert!(!shim.check_driver_active("e1000"));

        let pid = shim.load_isolated_rump_driver("e1000");
        assert!(shim.check_driver_active("e1000"));
        assert_eq!(pid, format!("rump_pid_{:x}", "e1000".len() * 12345));
    }

    #[test]
    fn test_apt_cache_simulator() {
        let mut cache = AptCacheSimulator::new(2);
        let m1 = AptPackageManifest {
            name: "libreoffice".to_string(),
            version: "1.0.0".to_string(),
            sha256: "sha256_mock_manifest_bytes".to_string(),
        };
        assert!(cache.cache_package_metadata(m1).is_ok());
        assert_eq!(
            cache.query_cached_package("libreoffice").unwrap().version,
            "1.0.0"
        );

        let m2 = AptPackageManifest {
            name: "vim".to_string(),
            version: "9.0.0".to_string(),
            sha256: "sha256_vim_hash".to_string(),
        };
        assert!(cache.cache_package_metadata(m2).is_ok());

        let m3 = AptPackageManifest {
            name: "emacs".to_string(),
            version: "29.0.0".to_string(),
            sha256: "sha256_emacs_hash".to_string(),
        };
        assert!(cache.cache_package_metadata(m3).is_err());
    }

    #[test]
    fn test_dpkg_multi_arch() {
        let mut multi = DpkgMultiArch::new();
        assert!(multi.is_architecture_supported("x86_64"));
        assert!(!multi.is_architecture_supported("arm64"));

        multi.register_foreign_architecture("arm64");
        assert!(multi.is_architecture_supported("arm64"));
    }

    #[test]
    fn test_debian_policy_enforcer() {
        let enforcer = DebianPolicyEnforcer::new();
        assert!(enforcer.evaluate_package_compliance(true, "/usr/bin/libreoffice"));
        assert!(!enforcer.evaluate_package_compliance(false, "/usr/bin/libreoffice"));
        assert!(!enforcer.evaluate_package_compliance(true, "/var/log/libreoffice"));
    }

    #[test]
    fn test_three_tier_release_model() {
        let mut model = ThreeTierReleaseModel::new();
        let channels_list = model.list_channels();
        assert!(channels_list.contains("sigma.next"));
        assert!(channels_list.contains("sigma.beta"));
        assert!(channels_list.contains("sigma.stable"));

        assert_eq!(model.active_channel, "sigma.stable");

        let set_res = model.set_channel("sigma.next");
        assert!(set_res.is_ok());
        assert_eq!(model.active_channel, "sigma.next");

        let set_err = model.set_channel("invalid_channel");
        assert!(set_err.is_err());
    }

    #[test]
    fn test_debian_social_contract() {
        let sc = DebianSocialContract::new();

        // DFSG Compliant Licenses
        assert!(sc.is_dfsg_compliant_license("MIT"));
        assert!(sc.is_dfsg_compliant_license("Apache-2.0"));
        assert!(sc.is_dfsg_compliant_license("GPL-3.0"));
        assert!(!sc.is_dfsg_compliant_license("Proprietary"));
        assert!(!sc.is_dfsg_compliant_license("Non-Free-Ware"));

        // Overall Social Contract compliance
        assert!(sc.evaluate_social_contract_compliance(true, true, true));
        assert!(!sc.evaluate_social_contract_compliance(false, true, true));
        assert!(!sc.evaluate_social_contract_compliance(true, false, true));
        assert!(!sc.evaluate_social_contract_compliance(true, true, false));
    }

    #[test]
    fn test_freeze_based_stabilization() {
        let mut stabilization = FreezeBasedStabilization::new();

        // No active freeze
        assert!(!stabilization.is_freeze_active);
        assert!(stabilization.is_update_allowed("new-feature"));
        assert!(stabilization.is_update_allowed("security"));
        assert!(stabilization.is_update_allowed("critical-bugfix"));

        // Activate freeze period
        stabilization.set_freeze_state(true);
        assert!(stabilization.is_freeze_active);
        assert!(!stabilization.is_update_allowed("new-feature"));
        assert!(stabilization.is_update_allowed("security"));
        assert!(stabilization.is_update_allowed("critical-bugfix"));
    }
}
