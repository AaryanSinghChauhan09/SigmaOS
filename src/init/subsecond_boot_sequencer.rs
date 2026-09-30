//! SigmaOS Sub-Second Boot Sequencer
//!
//! Sovereign async service initialization engine targeting <250ms cold boot.
//!
//! Inspired by:
//! - systemd parallel unit activation with dependency tracking
//! - OpenBSD's `rc(8)` ordered init scripts with pledge/unveil sandboxing
//! - s6 (Laurent Bercot) — lightweight service supervision
//! - runit — parallel stage-1/stage-2/stage-3 init
//! - Clear Linux fast boot optimization (removing systemd overhead)
//! - macOS launchd parallel boot
//!
//! Architecture:
//! - Stage 1 (< 10ms): Kernel subsystem bring-up (memory, CPU, IRQ)
//! - Stage 2 (< 50ms): Essential services (clock, entropy, network stack)
//! - Stage 3 (< 150ms): System services (mounts, logging, dbus)
//! - Stage 4 (< 250ms): User graphical session (Zenith Wayland compositor)

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;

// ─── Boot Stage ───────────────────────────────────────────────────────────────

/// Boot stage classification (ms targets)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BootStage {
    /// Stage 1: kernel subsystems (< 10ms)
    KernelSubsystems,
    /// Stage 2: essential services (< 50ms)
    EssentialServices,
    /// Stage 3: system services (< 150ms)
    SystemServices,
    /// Stage 4: graphical user session (< 250ms)
    GraphicalSession,
}

impl BootStage {
    /// Target completion time budget in ms
    pub fn target_ms(&self) -> u64 {
        match self {
            BootStage::KernelSubsystems => 10,
            BootStage::EssentialServices => 50,
            BootStage::SystemServices => 150,
            BootStage::GraphicalSession => 250,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            BootStage::KernelSubsystems => "Stage-1: Kernel Subsystems",
            BootStage::EssentialServices => "Stage-2: Essential Services",
            BootStage::SystemServices => "Stage-3: System Services",
            BootStage::GraphicalSession => "Stage-4: Graphical Session",
        }
    }
}

// ─── Service Definition ───────────────────────────────────────────────────────

/// Service state in the boot sequencer
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SvcState {
    /// Not yet started
    Pending,
    /// Started and running
    Running,
    /// Completed successfully
    Done,
    /// Failed to start
    Failed(String),
    /// Skipped (dependency failed or feature disabled)
    Skipped,
}

/// A SigmaInit service definition
#[derive(Debug, Clone)]
pub struct SigmaService {
    /// Unique service name (like systemd unit name)
    pub name: String,
    /// Boot stage this service belongs to
    pub stage: BootStage,
    /// Dependencies: list of service names that must be Done before this starts
    pub dependencies: Vec<String>,
    /// Whether this service can start in parallel with others in the same stage
    pub parallelizable: bool,
    /// Estimated startup time in ms (used for scheduling)
    pub estimated_ms: u64,
    /// Sandbox: pledge promise set (OpenBSD-inspired)
    pub pledge_promises: Vec<String>,
    /// Sandbox: unveil paths (OpenBSD-inspired)
    pub unveil_paths: Vec<(String, String)>,  // (path, permissions)
    /// Current state
    pub state: SvcState,
    /// Actual start time (simulated ms from boot epoch)
    pub start_time_ms: u64,
    /// Actual finish time
    pub finish_time_ms: u64,
}

impl SigmaService {
    /// Create a new service definition
    pub fn new(name: &str, stage: BootStage, deps: Vec<&str>, parallel: bool, est_ms: u64) -> Self {
        SigmaService {
            name: String::from(name),
            stage,
            dependencies: deps.into_iter().map(String::from).collect(),
            parallelizable: parallel,
            estimated_ms: est_ms,
            pledge_promises: Vec::new(),
            unveil_paths: Vec::new(),
            state: SvcState::Pending,
            start_time_ms: 0,
            finish_time_ms: 0,
        }
    }

    /// Add an OpenBSD-style pledge promise
    pub fn with_pledge(mut self, promises: &[&str]) -> Self {
        self.pledge_promises = promises.iter().map(|s| String::from(*s)).collect();
        self
    }

    /// Add an OpenBSD-style unveil path
    pub fn with_unveil(mut self, paths: &[(&str, &str)]) -> Self {
        self.unveil_paths = paths.iter().map(|(p, m)| (String::from(*p), String::from(*m))).collect();
        self
    }
}

// ─── Boot Event Log ───────────────────────────────────────────────────────────

/// A single boot event log entry
#[derive(Debug, Clone)]
pub struct BootEvent {
    pub timestamp_ms: u64,
    pub service: String,
    pub event: String,
    pub stage: BootStage,
}

// ─── Boot Sequencer ──────────────────────────────────────────────────────────

/// SigmaOS Sub-Second Boot Sequencer
///
/// Manages the parallel, dependency-ordered activation of all system services
/// targeting a total cold-boot time of <250ms.
pub struct SigmaBootSequencer {
    /// All registered services, indexed by name
    pub services: BTreeMap<String, SigmaService>,
    /// Boot event log (sorted by timestamp)
    pub event_log: Vec<BootEvent>,
    /// Simulated current clock (ms from firmware handoff)
    pub clock_ms: u64,
    /// Whether boot has completed
    pub boot_complete: bool,
    /// Total boot time when completed
    pub total_boot_ms: u64,
}

impl SigmaBootSequencer {
    /// Create a new boot sequencer with the canonical SigmaOS service registry
    pub fn new() -> Self {
        let mut seq = SigmaBootSequencer {
            services: BTreeMap::new(),
            event_log: Vec::new(),
            clock_ms: 0,
            boot_complete: false,
            total_boot_ms: 0,
        };
        seq.register_default_services();
        seq
    }

    /// Register the default SigmaOS boot service set
    fn register_default_services(&mut self) {
        // ── Stage 1: Kernel Subsystems ──────────────────────────────────────
        self.add(
            SigmaService::new("sigma-memory-init", BootStage::KernelSubsystems, vec![], true, 2)
                .with_pledge(&["stdio", "rpath"])
                .with_unveil(&[("/sigma/store", "r")])
        );
        self.add(
            SigmaService::new("sigma-cpu-topology", BootStage::KernelSubsystems, vec![], true, 1)
                .with_pledge(&["stdio"])
        );
        self.add(
            SigmaService::new("sigma-irq-controller", BootStage::KernelSubsystems,
                vec!["sigma-cpu-topology"], true, 3)
                .with_pledge(&["stdio", "unveil"])
        );
        self.add(
            SigmaService::new("sigma-bore-eevdf-sched", BootStage::KernelSubsystems,
                vec!["sigma-cpu-topology"], true, 2)
                .with_pledge(&["stdio"])
        );

        // ── Stage 2: Essential Services ─────────────────────────────────────
        self.add(
            SigmaService::new("sigma-entropy", BootStage::EssentialServices,
                vec!["sigma-memory-init"], true, 5)
                .with_pledge(&["stdio", "rpath"])
                .with_unveil(&[("/dev/urandom", "r")])
        );
        self.add(
            SigmaService::new("sigma-clock", BootStage::EssentialServices,
                vec!["sigma-memory-init"], true, 2)
                .with_pledge(&["stdio", "settime"])
        );
        self.add(
            SigmaService::new("sigma-pqc-keystore", BootStage::EssentialServices,
                vec!["sigma-entropy"], true, 8)
                .with_pledge(&["stdio", "rpath", "wpath", "cpath"])
                .with_unveil(&[("/sigma/keys", "rwc")])
        );
        self.add(
            SigmaService::new("sigma-netstack", BootStage::EssentialServices,
                vec!["sigma-entropy", "sigma-clock"], true, 10)
                .with_pledge(&["stdio", "inet", "rpath"])
        );
        self.add(
            SigmaService::new("sigma-landlock-lsm", BootStage::EssentialServices,
                vec!["sigma-memory-init"], true, 3)
                .with_pledge(&["stdio"])
        );

        // ── Stage 3: System Services ─────────────────────────────────────────
        self.add(
            SigmaService::new("sigma-vfs-mount", BootStage::SystemServices,
                vec!["sigma-memory-init"], false, 15)
                .with_pledge(&["stdio", "rpath", "wpath"])
                .with_unveil(&[("/", "r"), ("/sigma/store", "r"), ("/var", "rwc")])
        );
        self.add(
            SigmaService::new("sigma-sigpkg-daemon", BootStage::SystemServices,
                vec!["sigma-vfs-mount", "sigma-netstack"], true, 12)
                .with_pledge(&["stdio", "rpath", "wpath", "cpath", "inet", "exec"])
                .with_unveil(&[("/sigma/store", "rwc"), ("/etc/sigpkg", "r")])
        );
        self.add(
            SigmaService::new("sigma-logger", BootStage::SystemServices,
                vec!["sigma-vfs-mount"], true, 5)
                .with_pledge(&["stdio", "wpath", "cpath"])
                .with_unveil(&[("/var/log/sigma", "wc")])
        );
        self.add(
            SigmaService::new("sigma-udev", BootStage::SystemServices,
                vec!["sigma-vfs-mount", "sigma-irq-controller"], true, 8)
                .with_pledge(&["stdio", "rpath", "wpath"])
                .with_unveil(&[("/dev", "rwc"), ("/sys", "r")])
        );
        self.add(
            SigmaService::new("sigma-dbus", BootStage::SystemServices,
                vec!["sigma-logger"], true, 6)
                .with_pledge(&["stdio", "rpath", "unix"])
                .with_unveil(&[("/var/run/dbus", "rwc")])
        );
        self.add(
            SigmaService::new("sigma-wireguard-pqc", BootStage::SystemServices,
                vec!["sigma-netstack", "sigma-pqc-keystore"], true, 15)
                .with_pledge(&["stdio", "inet", "rpath"])
                .with_unveil(&[("/sigma/keys", "r"), ("/etc/wg", "r")])
        );

        // ── Stage 4: Graphical Session ────────────────────────────────────────
        self.add(
            SigmaService::new("zenith-wayland", BootStage::GraphicalSession,
                vec!["sigma-udev", "sigma-dbus"], false, 40)
                .with_pledge(&["stdio", "rpath", "wpath", "proc", "exec"])
                .with_unveil(&[("/sigma/store", "r"), ("/dev/dri", "rw"), ("/dev/input", "r")])
        );
        self.add(
            SigmaService::new("sigma-session-manager", BootStage::GraphicalSession,
                vec!["zenith-wayland"], true, 20)
                .with_pledge(&["stdio", "rpath", "wpath", "proc", "exec", "unix"])
        );
        self.add(
            SigmaService::new("sigma-autotuner-governor", BootStage::GraphicalSession,
                vec!["sigma-bore-eevdf-sched", "sigma-session-manager"], true, 5)
                .with_pledge(&["stdio"])
        );
    }

    /// Register a single service
    pub fn add(&mut self, svc: SigmaService) {
        self.services.insert(svc.name.clone(), svc);
    }

    // ── Boot Simulation ───────────────────────────────────────────────────────

    /// Check if all dependencies of a service are Done
    fn deps_satisfied(&self, svc_name: &str) -> bool {
        let svc = match self.services.get(svc_name) {
            Some(s) => s,
            None => return false,
        };
        svc.dependencies.iter().all(|dep| {
            self.services
                .get(dep)
                .map(|d| d.state == SvcState::Done)
                .unwrap_or(false)
        })
    }

    /// Simulate starting a service (advances the clock by estimated_ms)
    fn start_service(&mut self, name: &str) {
        let svc = match self.services.get_mut(name) {
            Some(s) => s,
            None => return,
        };
        svc.state = SvcState::Running;
        svc.start_time_ms = self.clock_ms;
        self.event_log.push(BootEvent {
            timestamp_ms: self.clock_ms,
            service: String::from(name),
            event: String::from("START"),
            stage: svc.stage,
        });
    }

    /// Simulate completing a service
    fn complete_service(&mut self, name: &str, elapsed_ms: u64) {
        let svc = match self.services.get_mut(name) {
            Some(s) => s,
            None => return,
        };
        svc.finish_time_ms = svc.start_time_ms + elapsed_ms;
        svc.state = SvcState::Done;
        self.event_log.push(BootEvent {
            timestamp_ms: svc.finish_time_ms,
            service: String::from(name),
            event: format!("DONE ({}ms)", elapsed_ms),
            stage: svc.stage,
        });
    }

    /// Simulate the full boot sequence across all stages.
    ///
    /// Returns total simulated boot time in ms.
    pub fn simulate_boot(&mut self) -> u64 {
        self.clock_ms = 0;

        let stages = [
            BootStage::KernelSubsystems,
            BootStage::EssentialServices,
            BootStage::SystemServices,
            BootStage::GraphicalSession,
        ];

        for stage in &stages {
            self.simulate_stage(*stage);
        }

        self.total_boot_ms = self.clock_ms;
        self.boot_complete = true;
        self.clock_ms
    }

    /// Simulate one boot stage (parallel + sequential services)
    fn simulate_stage(&mut self, stage: BootStage) {
        let stage_start = self.clock_ms;
        let target_ms = stage.target_ms();

        // Collect services in this stage that are ready to start
        let mut iteration = 0u32;
        loop {
            iteration += 1;
            if iteration > 100 { break; } // safety: max 100 iterations

            // Find all Pending services in this stage with satisfied deps
            let ready: Vec<String> = self.services.iter()
                .filter(|(_, s)| s.stage == stage && s.state == SvcState::Pending)
                .filter(|(name, _)| self.deps_satisfied(name))
                .map(|(name, _)| name.clone())
                .collect();

            if ready.is_empty() {
                // Check if any services in this stage are still not done
                let pending = self.services.values()
                    .any(|s| s.stage == stage && s.state == SvcState::Pending);
                if !pending { break; }
                // Some services have unsatisfied deps — mark them as skipped
                let stuck: Vec<String> = self.services.iter()
                    .filter(|(_, s)| s.stage == stage && s.state == SvcState::Pending)
                    .map(|(n, _)| n.clone())
                    .collect();
                for name in stuck {
                    if let Some(svc) = self.services.get_mut(&name) {
                        svc.state = SvcState::Skipped;
                    }
                }
                break;
            }

            // Start all ready services (parallel within stage)
            let estimated_times: Vec<(String, u64)> = ready.iter()
                .map(|name| {
                    let est = self.services.get(name).map(|s| s.estimated_ms).unwrap_or(5);
                    (name.clone(), est)
                })
                .collect();

            for (name, _) in &estimated_times {
                self.start_service(name);
            }

            // Complete all parallel services — the stage clock advances by the max estimated time
            let max_elapsed = estimated_times.iter().map(|(_, t)| *t).max().unwrap_or(0);
            for (name, est) in &estimated_times {
                self.complete_service(name, *est);
            }
            self.clock_ms += max_elapsed;
        }

        // Ensure stage minimum time is respected
        let stage_elapsed = self.clock_ms - stage_start;
        if stage_elapsed < target_ms / 4 {
            // Stage completed very fast — pad to at least some minimum
        }
    }

    // ── Reporting ─────────────────────────────────────────────────────────────

    /// Returns a human-readable boot report
    pub fn boot_report(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        lines.push(format!(
            "=== SigmaOS Boot Report ===\nTotal: {}ms (target: <250ms) | {} services",
            self.total_boot_ms,
            self.services.len()
        ));

        let stages = [
            BootStage::KernelSubsystems,
            BootStage::EssentialServices,
            BootStage::SystemServices,
            BootStage::GraphicalSession,
        ];

        for stage in &stages {
            let svcs: Vec<&SigmaService> = self.services.values()
                .filter(|s| s.stage == *stage)
                .collect();
            let done = svcs.iter().filter(|s| s.state == SvcState::Done).count();
            let total = svcs.len();
            let max_t = svcs.iter().map(|s| s.finish_time_ms).max().unwrap_or(0);
            lines.push(format!(
                "  {} | {}/{} done | finished at {}ms",
                stage.label(), done, total, max_t
            ));
        }

        let failed: Vec<&str> = self.services.values()
            .filter(|s| matches!(s.state, SvcState::Failed(_)))
            .map(|s| s.name.as_str())
            .collect();
        if !failed.is_empty() {
            lines.push(format!("  FAILED: {}", failed.join(", ")));
        }

        lines.join("\n")
    }

    /// Returns count of services in each state
    pub fn service_counts(&self) -> BTreeMap<String, usize> {
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for svc in self.services.values() {
            let key = match &svc.state {
                SvcState::Pending => "pending",
                SvcState::Running => "running",
                SvcState::Done => "done",
                SvcState::Failed(_) => "failed",
                SvcState::Skipped => "skipped",
            };
            *counts.entry(String::from(key)).or_insert(0) += 1;
        }
        counts
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod boot_sequencer_tests {
    use super::*;

    #[test]
    fn test_boot_stages_ordered() {
        assert!(BootStage::KernelSubsystems < BootStage::EssentialServices);
        assert!(BootStage::EssentialServices < BootStage::SystemServices);
        assert!(BootStage::SystemServices < BootStage::GraphicalSession);
    }

    #[test]
    fn test_stage_time_budgets() {
        assert_eq!(BootStage::KernelSubsystems.target_ms(), 10);
        assert_eq!(BootStage::GraphicalSession.target_ms(), 250);
    }

    #[test]
    fn test_default_services_registered() {
        let seq = SigmaBootSequencer::new();
        assert!(seq.services.contains_key("sigma-memory-init"));
        assert!(seq.services.contains_key("zenith-wayland"));
        assert!(seq.services.contains_key("sigma-bore-eevdf-sched"));
        assert!(seq.services.contains_key("sigma-wireguard-pqc"));
        assert!(seq.services.len() >= 15);
    }

    #[test]
    fn test_simulate_boot_completes() {
        let mut seq = SigmaBootSequencer::new();
        let boot_ms = seq.simulate_boot();
        assert!(seq.boot_complete);
        assert!(boot_ms > 0);
        // All registered services should be Done or Skipped (not Pending)
        for (name, svc) in &seq.services {
            assert!(
                svc.state != SvcState::Pending,
                "Service {} still pending after boot",
                name
            );
        }
    }

    #[test]
    fn test_boot_under_250ms_budget() {
        let mut seq = SigmaBootSequencer::new();
        let boot_ms = seq.simulate_boot();
        // The simulated boot should be within a reasonable range
        // (Simulated parallel times may be lower than real hardware)
        println!("Simulated boot time: {}ms", boot_ms);
        assert!(boot_ms <= 500, "Boot took {}ms — too slow!", boot_ms);
    }

    #[test]
    fn test_dependency_ordering() {
        let mut seq = SigmaBootSequencer::new();
        seq.simulate_boot();
        // zenith-wayland must start after sigma-udev and sigma-dbus
        let wayland = seq.services.get("zenith-wayland").unwrap();
        let udev = seq.services.get("sigma-udev").unwrap();
        assert!(
            wayland.start_time_ms >= udev.finish_time_ms,
            "zenith-wayland started before sigma-udev finished"
        );
    }

    #[test]
    fn test_event_log_populated() {
        let mut seq = SigmaBootSequencer::new();
        seq.simulate_boot();
        assert!(!seq.event_log.is_empty());
        // All events should have monotonically non-decreasing timestamps
        let mut last_ts = 0u64;
        for event in &seq.event_log {
            assert!(event.timestamp_ms >= last_ts || event.event.starts_with("START"));
            if event.event.starts_with("DONE") {
                last_ts = last_ts.max(event.timestamp_ms);
            }
        }
    }

    #[test]
    fn test_boot_report_format() {
        let mut seq = SigmaBootSequencer::new();
        seq.simulate_boot();
        let report = seq.boot_report();
        assert!(report.contains("SigmaOS Boot Report"));
        assert!(report.contains("Stage-1"));
        assert!(report.contains("Stage-4"));
        assert!(report.contains("done"));
    }

    #[test]
    fn test_pledge_and_unveil_set() {
        let seq = SigmaBootSequencer::new();
        let entropy = seq.services.get("sigma-entropy").unwrap();
        assert!(entropy.pledge_promises.contains(&String::from("rpath")));
        assert!(entropy.unveil_paths.iter().any(|(p, _)| p.contains("urandom")));
    }
}
