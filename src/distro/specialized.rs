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
// UBUNTU LINUX DISTROS TOOLS & CONCEPTS
// ==========================================

/// Ubuntu Desktop: Simulated Advanced Package Tool (APT) & Personal Package Archive (PPA) Engine
#[derive(Debug, Clone)]
pub struct UbuntuAptEngine {
    pub registered_packages: HashMap<String, String>, // maps PackageName -> Version
    pub installed_packages: HashMap<String, String>,  // maps PackageName -> Version
    pub added_ppas: Vec<String>,                      // trusted third-party repositories
    pub apt_cache_synchronized: bool,
}

impl UbuntuAptEngine {
    pub fn new() -> Self {
        let mut registered = HashMap::new();
        registered.insert("gnome-shell".to_string(), "45.0".to_string());
        registered.insert("build-essential".to_string(), "12.9".to_string());
        registered.insert("curl".to_string(), "8.2.1".to_string());
        registered.insert("vlc".to_string(), "3.0.18".to_string());

        Self {
            registered_packages: registered,
            installed_packages: HashMap::new(),
            added_ppas: Vec::new(),
            apt_cache_synchronized: false,
        }
    }

    /// Simulates add-apt-repository
    pub fn add_ppa(&mut self, ppa_uri: &str) -> Result<(), &'static str> {
        if ppa_uri.starts_with("ppa:") {
            self.added_ppas.push(ppa_uri.to_string());
            self.apt_cache_synchronized = false; // cache needs sync
            Ok(())
        } else {
            Err("Invalid PPA format: expected ppa:<launchpad-author>/<archive>")
        }
    }

    /// Simulates apt-get update
    pub fn apt_get_update(&mut self) {
        self.apt_cache_synchronized = true;
    }

    /// Simulates apt-get install <package>
    pub fn apt_get_install(&mut self, package_name: &str) -> Result<String, &'static str> {
        if !self.apt_cache_synchronized {
            return Err("APT Cache is out of date. Run apt_get_update first.");
        }

        if let Some(version) = self.registered_packages.get(package_name) {
            self.installed_packages.insert(package_name.to_string(), version.clone());
            Ok(format!("Successfully installed {} (v{}) via APT.", package_name, version))
        } else {
            Err("E: Unable to locate package in configured repositories")
        }
    }
}

impl Default for UbuntuAptEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Ubuntu Server: Netplan network auto-configurer processing YAML-like definitions
#[derive(Debug, Clone)]
pub struct NetplanConfig {
    pub interface_name: String,
    pub dhcp4: bool,
    pub static_ip: Option<String>,
    pub gateway: Option<String>,
}

pub struct NetplanConfigEngine {
    pub configs: HashMap<String, NetplanConfig>,
}

impl NetplanConfigEngine {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    /// Parses declarative Netplan configurations (analogous to /etc/netplan/01-netcfg.yaml)
    pub fn apply_netplan_yaml(&mut self, interface: &str, yaml_data: &str) -> Result<(), &'static str> {
        if !yaml_data.contains("ethernets:") {
            return Err("Invalid Netplan YAML: missing 'ethernets' definition");
        }

        let dhcp4 = yaml_data.contains("dhcp4: true");
        let mut static_ip = None;
        let mut gateway = None;

        if !dhcp4 {
            if let Some(ip_idx) = yaml_data.find("addresses:") {
                let segment = &yaml_data[ip_idx..];
                if let Some(start) = segment.find('[') {
                    if let Some(end) = segment.find(']') {
                        static_ip = Some(segment[start + 1..end].to_string());
                    }
                }
            }
            if let Some(gw_idx) = yaml_data.find("gateway4:") {
                let segment = &yaml_data[gw_idx..];
                let lines: Vec<&str> = segment.lines().collect();
                if let Some(first_line) = lines.first() {
                    let parts: Vec<&str> = first_line.split(':').collect();
                    if let Some(val) = parts.get(1) {
                        gateway = Some(val.trim().to_string());
                    }
                }
            }
        }

        let net_config = NetplanConfig {
            interface_name: interface.to_string(),
            dhcp4,
            static_ip,
            gateway,
        };

        self.configs.insert(interface.to_string(), net_config);
        Ok(())
    }
}

impl Default for NetplanConfigEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Ubuntu Server: Cloud-Init automated user and metadata provisioner
#[derive(Debug, Clone)]
pub struct CloudInitEngine {
    pub hostname: String,
    pub authorized_ssh_keys: Vec<String>,
    pub default_user_created: bool,
}

impl CloudInitEngine {
    pub fn new() -> Self {
        Self {
            hostname: "ubuntu".to_string(),
            authorized_ssh_keys: Vec::new(),
            default_user_created: false,
        }
    }

    /// Provisions operating parameters during early VM boot cycles
    pub fn execute_cloud_config(&mut self, config_yaml: &str) -> Result<(), &'static str> {
        if !config_yaml.contains("#cloud-config") {
            return Err("Invalid cloud-config header: missing '#cloud-config'");
        }

        if let Some(host_idx) = config_yaml.find("hostname:") {
            let line = config_yaml[host_idx..].lines().next().unwrap_or("");
            let parts: Vec<&str> = line.split(':').collect();
            if let Some(h) = parts.get(1) {
                self.hostname = h.trim().to_string();
            }
        }

        if config_yaml.contains("ssh_authorized_keys:") {
            self.authorized_ssh_keys.push("ssh-rsa AAAAB3NzaC1yc2E...".to_string());
        }

        if config_yaml.contains("users:") && config_yaml.contains("name:") {
            self.default_user_created = true;
        }

        Ok(())
    }
}

impl Default for CloudInitEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Lubuntu: Ultra-Lightweight LXQt Resource Watchdog and out-of-memory preventer
#[derive(Debug, Clone)]
pub struct LxqtProcess {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub memory_mb: f32,
}

pub struct LxqtResourceMonitor {
    pub max_ram_mb: f32,
    pub running_processes: Vec<LxqtProcess>,
}

impl LxqtResourceMonitor {
    pub fn new(max_ram: f32) -> Self {
        Self {
            max_ram_mb: max_ram,
            running_processes: Vec::new(),
        }
    }

    /// Monitors resource footprint and auto-kills high-consumption tasks to prevent LXQt jank
    pub fn check_and_prevent_oom(&mut self) -> Vec<String> {
        let mut killed_process_names = Vec::new();
        let total_ram_used: f32 = self.running_processes.iter().map(|p| p.memory_mb).sum();

        if total_ram_used > self.max_ram_mb {
            // Sort processes by memory usage descending to target major OOM triggers first
            self.running_processes.sort_by(|a, b| b.memory_mb.partial_cmp(&a.memory_mb).unwrap());

            while let Some(hog) = self.running_processes.first() {
                let current_total: f32 = self.running_processes.iter().map(|p| p.memory_mb).sum();
                if current_total <= self.max_ram_mb {
                    break;
                }
                killed_process_names.push(hog.name.clone());
                self.running_processes.remove(0); // remove first/largest process
            }
        }
        killed_process_names
    }
}

/// Ubuntu Studio: Low-Latency creative audio routing manager (PipeWire / JACK inspired)
#[derive(Debug, Clone)]
pub struct PipewireAudioRouter {
    pub active_routes: HashMap<String, String>, // maps InputPort -> OutputPort
    pub low_latency_mode: bool,
}

impl PipewireAudioRouter {
    pub fn new() -> Self {
        Self {
            active_routes: HashMap::new(),
            low_latency_mode: true,
        }
    }

    /// Connects two creative ports (e.g., VirtualSynthesizer -> LowLatencySpeaker)
    pub fn connect_ports(&mut self, source: &str, destination: &str) {
        self.active_routes.insert(source.to_string(), destination.to_string());
    }

    /// Simulates route dispatching to measure real-time audio latency offsets
    pub fn get_dispatch_latency_ms(&self) -> f32 {
        if self.low_latency_mode {
            0.8 // sub-millisecond JACK audio routing latency
        } else {
            12.5 // standard audio routing latency
        }
    }
}

impl Default for PipewireAudioRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Ubuntu Core: Snapd Transactional sandboxing and digital signature enforcement
#[derive(Debug, Clone)]
pub struct SnapPackage {
    pub name: String,
    pub revision: u32,
    pub signature_verified: bool,
    pub read_only_loop_mounted: bool,
}

pub struct SnapdEngine {
    pub installed_snaps: HashMap<String, SnapPackage>,
}

impl SnapdEngine {
    pub fn new() -> Self {
        Self {
            installed_snaps: HashMap::new(),
        }
    }

    /// Transactionally updates or installs a secure snap under loop-mounted confinement
    pub fn install_secure_snap(&mut self, snap_name: &str, rev: u32, signature: &[u8]) -> Result<(), &'static str> {
        // Simple signature check representing cryptographic assertions
        if signature.is_empty() {
            return Err("Unsigned Snap installation rejected!");
        }

        let snap = SnapPackage {
            name: snap_name.to_string(),
            revision: rev,
            signature_verified: true,
            read_only_loop_mounted: true,
        };

        self.installed_snaps.insert(snap_name.to_string(), snap);
        Ok(())
    }
}

impl Default for SnapdEngine {
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
    fn test_ubuntu_apt_and_ppas() {
        let mut apt = UbuntuAptEngine::new();
        assert!(apt.add_ppa("invalid_ppa").is_err());
        assert!(apt.add_ppa("ppa:libreoffice/ppa").is_ok());

        assert!(apt.apt_get_install("curl").is_err()); // cache not sync
        apt.apt_get_update();
        let res = apt.apt_get_install("curl").unwrap();
        assert!(res.contains("curl"));
        assert_eq!(apt.installed_packages.get("curl").unwrap(), "8.2.1");
    }

    #[test]
    fn test_ubuntu_server_netplan_provisioning() {
        let mut netplan = NetplanConfigEngine::new();
        let yaml_data = r#"
            network:
              version: 2
              ethernets:
                eth0:
                  dhcp4: false
                  addresses: [192.168.1.100/24]
                  gateway4: 192.168.1.1
        "#;
        assert!(netplan.apply_netplan_yaml("eth0", yaml_data).is_ok());
        let eth0 = netplan.configs.get("eth0").unwrap();
        assert!(!eth0.dhcp4);
        assert_eq!(eth0.static_ip.as_deref(), Some("192.168.1.100/24"));
        assert_eq!(eth0.gateway.as_deref(), Some("192.168.1.1"));
    }

    #[test]
    fn test_lubuntu_lxqt_watcher() {
        let mut monitor = LxqtResourceMonitor::new(512.0);
        monitor.running_processes.push(LxqtProcess {
            pid: 101,
            name: "lxqt-panel".to_string(),
            cpu_percent: 1.2,
            memory_mb: 45.0,
        });
        monitor.running_processes.push(LxqtProcess {
            pid: 102,
            name: "firefox-leak".to_string(),
            cpu_percent: 75.0,
            memory_mb: 600.0,
        });

        let killed = monitor.check_and_prevent_oom();
        assert_eq!(killed.len(), 1);
        assert_eq!(killed[0], "firefox-leak");
        assert_eq!(monitor.running_processes.len(), 1);
        assert_eq!(monitor.running_processes[0].name, "lxqt-panel");
    }

    #[test]
    fn test_ubuntu_studio_low_latency_audio() {
        let mut router = PipewireAudioRouter::new();
        router.connect_ports("midi_keyboard", "jack_synth");
        assert_eq!(router.active_routes.get("midi_keyboard").unwrap(), "jack_synth");
        assert!(router.get_dispatch_latency_ms() < 1.0);
    }

    #[test]
    fn test_ubuntu_core_snapd_sandbox() {
        let mut snapd = SnapdEngine::new();
        assert!(snapd.install_secure_snap("core22", 15, &[]).is_err());
        assert!(snapd.install_secure_snap("core22", 15, &[1, 2, 3]).is_ok());
        let snap = snapd.installed_snaps.get("core22").unwrap();
        assert!(snap.read_only_loop_mounted);
        assert!(snap.signature_verified);
    }
}
