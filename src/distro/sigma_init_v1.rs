// SPDX-License-Identifier: MIT
//! SigmaOS Init System v1 — PID-1 Service Manager & Boot Orchestrator
//!
//! Inspired by systemd (Arch/Debian) and OpenRC (Gentoo) design philosophies.
//! Provides unit lifecycle (start/stop/restart), dependency ordering via
//! topological sort, service supervision with exponential-back-off respawn,
//! and boot target sequencing (sysinit → multi-user → graphical).
//!
//! Principles: SOLID, fail-fast, AAA tests, no unsafe code.

#![forbid(unsafe_code)]

use std::collections::{HashMap, VecDeque};
use std::fmt;

// ─── Service State ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServiceState {
    Dead,
    Starting,
    Running,
    Stopping,
    Failed,
    Restarting,
}

impl fmt::Display for ServiceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dead       => write!(f, "dead"),
            Self::Starting   => write!(f, "starting"),
            Self::Running    => write!(f, "running"),
            Self::Stopping   => write!(f, "stopping"),
            Self::Failed     => write!(f, "failed"),
            Self::Restarting => write!(f, "restarting"),
        }
    }
}

// ─── Boot Target ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BootTarget {
    SysInit,
    MultiUser,
    Graphical,
    Rescue,
    Emergency,
}

impl BootTarget {
    pub fn description(&self) -> &'static str {
        match self {
            Self::SysInit   => "System Initialization (mount, udev, networking)",
            Self::MultiUser => "Multi-User Target (network services, SSH, cron)",
            Self::Graphical => "Graphical Target (Wayland/Hyprland, display manager)",
            Self::Rescue    => "Rescue Shell (minimal environment)",
            Self::Emergency => "Emergency Shell (read-only root)",
        }
    }
}

impl fmt::Display for BootTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SysInit   => write!(f, "sysinit.target"),
            Self::MultiUser => write!(f, "multi-user.target"),
            Self::Graphical => write!(f, "graphical.target"),
            Self::Rescue    => write!(f, "rescue.target"),
            Self::Emergency => write!(f, "emergency.target"),
        }
    }
}

// ─── Restart Policy ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartPolicy {
    Never,
    OnFailure,
    Always,
}

// ─── Service Unit ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ServiceUnit {
    pub name: String,
    pub description: String,
    pub exec_start: String,
    pub exec_stop: Option<String>,
    pub after: Vec<String>,
    pub requires: Vec<String>,
    pub wants: Vec<String>,
    pub target: BootTarget,
    pub restart: RestartPolicy,
    pub restart_delay_ms: u64,
    pub restart_count: u32,
    pub state: ServiceState,
}

impl ServiceUnit {
    pub fn new(name: &str, description: &str, exec_start: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            exec_start: exec_start.to_string(),
            exec_stop: None,
            after: Vec::new(),
            requires: Vec::new(),
            wants: Vec::new(),
            target: BootTarget::MultiUser,
            restart: RestartPolicy::OnFailure,
            restart_delay_ms: 100,
            restart_count: 0,
            state: ServiceState::Dead,
        }
    }

    pub fn with_target(mut self, target: BootTarget) -> Self {
        self.target = target;
        self
    }

    pub fn with_after(mut self, deps: &[&str]) -> Self {
        self.after = deps.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_requires(mut self, deps: &[&str]) -> Self {
        self.requires = deps.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn start(&mut self) -> Result<(), InitError> {
        if self.state == ServiceState::Running {
            return Err(InitError::AlreadyRunning(self.name.clone()));
        }
        self.state = ServiceState::Starting;
        // In production: exec_start subprocess would be spawned here
        self.state = ServiceState::Running;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), InitError> {
        if self.state == ServiceState::Dead {
            return Err(InitError::NotRunning(self.name.clone()));
        }
        self.state = ServiceState::Stopping;
        self.state = ServiceState::Dead;
        Ok(())
    }

    pub fn fail(&mut self) {
        self.state = ServiceState::Failed;
        self.restart_count += 1;
    }

    pub fn restart_delay_ms(&self) -> u64 {
        // exponential back-off, capped at 30s
        let backoff = self.restart_delay_ms * (1 << self.restart_count.min(8));
        backoff.min(30_000)
    }

    /// Render unit as systemd-compatible unit file content
    pub fn to_unit_file(&self) -> String {
        let mut s = String::new();
        s.push_str("[Unit]\n");
        s.push_str(&format!("Description={}\n", self.description));
        if !self.after.is_empty() {
            s.push_str(&format!("After={}\n", self.after.join(" ")));
        }
        if !self.requires.is_empty() {
            s.push_str(&format!("Requires={}\n", self.requires.join(" ")));
        }
        if !self.wants.is_empty() {
            s.push_str(&format!("Wants={}\n", self.wants.join(" ")));
        }
        s.push_str("\n[Service]\n");
        s.push_str(&format!("ExecStart={}\n", self.exec_start));
        if let Some(stop) = &self.exec_stop {
            s.push_str(&format!("ExecStop={}\n", stop));
        }
        let restart_str = match self.restart {
            RestartPolicy::Never     => "no",
            RestartPolicy::OnFailure => "on-failure",
            RestartPolicy::Always    => "always",
        };
        s.push_str(&format!("Restart={}\n", restart_str));
        s.push_str(&format!("RestartSec={}ms\n", self.restart_delay_ms));
        s.push_str("\n[Install]\n");
        s.push_str(&format!("WantedBy={}\n", self.target));
        s
    }
}

// ─── Dependency Resolver (topological sort) ──────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitError {
    AlreadyRunning(String),
    NotRunning(String),
    CyclicDependency(String),
    UnknownUnit(String),
    DependencyFailed(String),
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyRunning(s)    => write!(f, "unit already running: {}", s),
            Self::NotRunning(s)        => write!(f, "unit not running: {}", s),
            Self::CyclicDependency(s)  => write!(f, "cyclic dependency detected involving: {}", s),
            Self::UnknownUnit(s)       => write!(f, "unknown unit: {}", s),
            Self::DependencyFailed(s)  => write!(f, "dependency failed: {}", s),
        }
    }
}

pub struct InitManager {
    units: HashMap<String, ServiceUnit>,
}

impl InitManager {
    pub fn new() -> Self {
        Self { units: HashMap::new() }
    }

    pub fn register(&mut self, unit: ServiceUnit) {
        self.units.insert(unit.name.clone(), unit);
    }

    pub fn get_state(&self, name: &str) -> Option<ServiceState> {
        self.units.get(name).map(|u| u.state)
    }

    /// Topological sort of units for a given boot target
    pub fn boot_order(&self, target: BootTarget) -> Result<Vec<String>, InitError> {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();

        for (name, unit) in &self.units {
            if unit.target <= target {
                graph.entry(name.clone()).or_default();
                in_degree.entry(name.clone()).or_insert(0);
                for dep in unit.after.iter().chain(unit.requires.iter()) {
                    if self.units.contains_key(dep) {
                        graph.entry(dep.clone()).or_default().push(name.clone());
                        *in_degree.entry(name.clone()).or_insert(0) += 1;
                    }
                }
            }
        }

        let mut queue: VecDeque<String> = in_degree.iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(name, _)| name.clone())
            .collect();

        let mut order = Vec::new();
        while let Some(name) = queue.pop_front() {
            order.push(name.clone());
            if let Some(dependents) = graph.get(&name) {
                for dep in dependents {
                    let deg = in_degree.entry(dep.clone()).or_insert(0);
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(dep.clone());
                    }
                }
            }
        }

        if order.len() < in_degree.len() {
            return Err(InitError::CyclicDependency("dependency graph has cycle".to_string()));
        }
        Ok(order)
    }

    pub fn start_target(&mut self, target: BootTarget) -> Result<Vec<String>, InitError> {
        let order = self.boot_order(target)?;
        let mut started = Vec::new();
        for name in &order {
            if let Some(unit) = self.units.get_mut(name) {
                if unit.state == ServiceState::Dead {
                    unit.start()?;
                    started.push(name.clone());
                }
            }
        }
        Ok(started)
    }
}

// ─── Default SigmaOS Service Units ───────────────────────────────────────────

pub fn sigma_default_units() -> Vec<ServiceUnit> {
    vec![
        ServiceUnit::new("sigma-udev", "SigmaOS udev Device Manager", "/usr/lib/systemd/systemd-udevd")
            .with_target(BootTarget::SysInit),
        ServiceUnit::new("sigma-network", "SigmaOS Network Manager (IWD/NetworkManager)", "/usr/bin/NetworkManager")
            .with_target(BootTarget::MultiUser)
            .with_after(&["sigma-udev"]),
        ServiceUnit::new("sigma-pipewire", "SigmaOS PipeWire Audio Server", "/usr/bin/pipewire")
            .with_target(BootTarget::Graphical)
            .with_after(&["sigma-network"]),
        ServiceUnit::new("sigma-hyprland", "SigmaOS Hyprland Wayland Compositor", "/usr/bin/Hyprland")
            .with_target(BootTarget::Graphical)
            .with_after(&["sigma-pipewire", "sigma-network"])
            .with_requires(&["sigma-network"]),
        ServiceUnit::new("sigma-waybar", "SigmaOS Waybar Status Panel", "/usr/bin/waybar")
            .with_target(BootTarget::Graphical)
            .with_after(&["sigma-hyprland"]),
    ]
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn default_manager() -> InitManager {
        let mut mgr = InitManager::new();
        for unit in sigma_default_units() {
            mgr.register(unit);
        }
        mgr
    }

    #[test]
    fn test_service_start_stop_lifecycle_aaa() {
        // Arrange
        let mut unit = ServiceUnit::new("test-svc", "Test Service", "/bin/test-daemon");
        // Act: start
        let result = unit.start();
        // Assert
        assert!(result.is_ok());
        assert_eq!(unit.state, ServiceState::Running);

        // Act: stop
        let result = unit.stop();
        // Assert
        assert!(result.is_ok());
        assert_eq!(unit.state, ServiceState::Dead);
    }

    #[test]
    fn test_start_already_running_fails_aaa() {
        // Arrange
        let mut unit = ServiceUnit::new("test-svc", "Test", "/bin/test");
        unit.start().unwrap();
        // Act
        let result = unit.start();
        // Assert
        assert!(matches!(result, Err(InitError::AlreadyRunning(_))));
    }

    #[test]
    fn test_stop_not_running_fails_aaa() {
        // Arrange
        let unit = ServiceUnit::new("test-svc", "Test", "/bin/test");
        let mut unit = unit;
        // Act
        let result = unit.stop();
        // Assert
        assert!(matches!(result, Err(InitError::NotRunning(_))));
    }

    #[test]
    fn test_exponential_backoff_aaa() {
        // Arrange
        let mut unit = ServiceUnit::new("flaky", "Flaky Service", "/bin/flaky");
        unit.restart_delay_ms = 100;
        // Act: simulate 3 failures
        unit.fail(); unit.fail(); unit.fail();
        // Assert: delay should be 100 * 2^3 = 800ms
        assert_eq!(unit.restart_delay_ms(), 800);
        assert_eq!(unit.restart_count, 3);
    }

    #[test]
    fn test_boot_order_respects_dependencies_aaa() {
        // Arrange
        let mgr = default_manager();
        // Act
        let order = mgr.boot_order(BootTarget::Graphical).unwrap();
        // Assert: udev before network, network before hyprland
        let udev_pos = order.iter().position(|s| s == "sigma-udev").unwrap();
        let net_pos  = order.iter().position(|s| s == "sigma-network").unwrap();
        let hype_pos = order.iter().position(|s| s == "sigma-hyprland").unwrap();
        assert!(udev_pos < net_pos, "udev must start before network");
        assert!(net_pos < hype_pos, "network must start before hyprland");
    }

    #[test]
    fn test_start_target_graphical_aaa() {
        // Arrange
        let mut mgr = default_manager();
        // Act
        let started = mgr.start_target(BootTarget::Graphical).unwrap();
        // Assert
        assert!(started.contains(&"sigma-hyprland".to_string()));
        assert!(started.contains(&"sigma-network".to_string()));
        assert!(started.contains(&"sigma-waybar".to_string()));
        assert_eq!(mgr.get_state("sigma-hyprland"), Some(ServiceState::Running));
    }

    #[test]
    fn test_unit_file_generation_aaa() {
        // Arrange
        let unit = ServiceUnit::new("sigma-hyprland", "Hyprland Compositor", "/usr/bin/Hyprland")
            .with_target(BootTarget::Graphical)
            .with_after(&["sigma-pipewire"]);
        // Act
        let file = unit.to_unit_file();
        // Assert
        assert!(file.contains("[Unit]"));
        assert!(file.contains("[Service]"));
        assert!(file.contains("[Install]"));
        assert!(file.contains("After=sigma-pipewire"));
        assert!(file.contains("WantedBy=graphical.target"));
        assert!(file.contains("ExecStart=/usr/bin/Hyprland"));
    }

    #[test]
    fn test_boot_target_ordering_aaa() {
        // Assert: SysInit < MultiUser < Graphical
        assert!(BootTarget::SysInit < BootTarget::MultiUser);
        assert!(BootTarget::MultiUser < BootTarget::Graphical);
    }
}
