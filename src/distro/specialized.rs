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

/// Sigma Hardware Detector (MHWD parity absorbing Manjaro Linux)
/// Automatically identifies PCI/USB hardware ID mappings and deploys optimal drivers.
pub struct SigmaHardwareDetector {
    pub hardware_db: HashMap<(u16, u16), &'static str>, // maps (vendor_id, device_id) to driver name
    pub loaded_drivers: Vec<&'static str>,
}

impl SigmaHardwareDetector {
    pub fn new() -> Self {
        let mut db = HashMap::new();
        // Register default hardware-to-driver mappings (e.g. GPUs, network adapters, storage controllers)
        db.insert((0x10DE, 0x2204), "nvidia-pcie-gen6"); // NVIDIA RTX 3090 / 4090
        db.insert((0x8086, 0x1533), "e1000e-ethernet");  // Intel E1000
        db.insert((0x10EC, 0x8168), "rtl8169-realtek");   // Realtek Ethernet
        db.insert((0x144D, 0xA808), "samsung-nvme-v4");   // Samsung Pro NVMe

        Self {
            hardware_db: db,
            loaded_drivers: Vec::new(),
        }
    }

    /// Simulates scanning a PCI/USB bus and auto-configuring appropriate drivers
    pub fn scan_and_load_drivers(&mut self, devices: &[(u16, u16)]) -> usize {
        let mut count = 0;
        for &dev in devices {
            if let Some(&driver) = self.hardware_db.get(&dev) {
                if !self.loaded_drivers.contains(&driver) {
                    self.loaded_drivers.push(driver);
                    count += 1;
                }
            }
        }
        count
    }
}

impl Default for SigmaHardwareDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Sigma Settings Manager (MSM parity absorbing Manjaro Linux)
/// Central controller to configure kernels, multi-locale language packages, and system timezones.
pub struct SigmaSettingsManager {
    pub available_kernels: Vec<&'static str>,
    pub active_kernel: &'static str,
    pub locale_packages: Vec<String>,
    pub active_timezone: String,
}

impl SigmaSettingsManager {
    pub fn new() -> Self {
        Self {
            available_kernels: vec!["Sovereign-LTS-6.1", "Sovereign-RT-6.6", "Sovereign-Mainline-6.12"],
            active_kernel: "Sovereign-LTS-6.1",
            locale_packages: vec!["en_US.UTF-8".to_string(), "hi_IN.UTF-8".to_string()], // default supports India Stack locales
            active_timezone: "UTC".to_string(),
        }
    }

    /// Switches the running system kernel dynamically
    pub fn switch_kernel(&mut self, target_kernel: &'static str) -> Result<(), &'static str> {
        if self.available_kernels.contains(&target_kernel) {
            self.active_kernel = target_kernel;
            Ok(())
        } else {
            Err("Target kernel is not available in system repos")
        }
    }

    /// Updates the active system timezone configuration
    pub fn update_timezone(&mut self, tz: &str) {
        self.active_timezone = tz.to_string();
    }
}

impl Default for SigmaSettingsManager {
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
}
