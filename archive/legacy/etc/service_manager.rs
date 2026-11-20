// SPDX-License-Identifier: MIT
// SigmaOS Service Manager - Arch Linux Inspired
// Implements systemd-style service management with Arch's simplicity

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Service state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
    Reloading,
}

impl ServiceState {
    pub fn as_str(&self) -> &str {
        match self {
            ServiceState::Stopped => "stopped",
            ServiceState::Starting => "starting",
            ServiceState::Running => "running",
            ServiceState::Stopping => "stopping",
            ServiceState::Failed => "failed",
            ServiceState::Reloading => "reloading",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "stopped" => Some(ServiceState::Stopped),
            "starting" => Some(ServiceState::Starting),
            "running" => Some(ServiceState::Running),
            "stopping" => Some(ServiceState::Stopping),
            "failed" => Some(ServiceState::Failed),
            "reloading" => Some(ServiceState::Reloading),
            _ => None,
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, ServiceState::Running | ServiceState::Starting | ServiceState::Reloading)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, ServiceState::Failed)
    }
}

/// Service type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceType {
    Simple,
    Forking,
    Notify,
    Idle,
    Dbus,
    Socket,
    Timer,
    Path,
}

impl ServiceType {
    pub fn as_str(&self) -> &str {
        match self {
            ServiceType::Simple => "simple",
            ServiceType::Forking => "forking",
            ServiceType::Notify => "notify",
            ServiceType::Idle => "idle",
            ServiceType::Dbus => "dbus",
            ServiceType::Socket => "socket",
            ServiceType::Timer => "timer",
            ServiceType::Path => "path",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "simple" => Some(ServiceType::Simple),
            "forking" => Some(ServiceType::Forking),
            "notify" => Some(ServiceType::Notify),
            "idle" => Some(ServiceType::Idle),
            "dbus" => Some(ServiceType::Dbus),
            "socket" => Some(ServiceType::Socket),
            "timer" => Some(ServiceType::Timer),
            "path" => Some(ServiceType::Path),
            _ => None,
        }
    }
}

/// Restart policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    No,
    OnSuccess,
    OnFailure,
    OnAbnormal,
    OnWatchdog,
    OnAbort,
    Always,
}

impl RestartPolicy {
    pub fn as_str(&self) -> &str {
        match self {
            RestartPolicy::No => "no",
            RestartPolicy::OnSuccess => "on-success",
            RestartPolicy::OnFailure => "on-failure",
            RestartPolicy::OnAbnormal => "on-abnormal",
            RestartPolicy::OnWatchdog => "on-watchdog",
            RestartPolicy::OnAbort => "on-abort",
            RestartPolicy::Always => "always",
        }
    }

    pub fn should_restart(&self, state: ServiceState) -> bool {
        match self {
            RestartPolicy::No => false,
            RestartPolicy::OnSuccess => state == ServiceState::Stopped,
            RestartPolicy::OnFailure => state == ServiceState::Failed,
            RestartPolicy::OnAbnormal => state == ServiceState::Failed,
            RestartPolicy::OnWatchdog => state == ServiceState::Failed,
            RestartPolicy::OnAbort => state == ServiceState::Failed,
            RestartPolicy::Always => true,
        }
    }
}

/// Service configuration
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub description: String,
    pub service_type: ServiceType,
    pub exec_start: String,
    pub exec_start_pre: Vec<String>,
    pub exec_start_post: Vec<String>,
    pub exec_reload: Option<String>,
    pub exec_stop: Option<String>,
    pub exec_stop_post: Vec<String>,
    pub working_directory: Option<String>,
    pub user: Option<String>,
    pub group: Option<String>,
    pub restart_policy: RestartPolicy,
    pub restart_sec: Option<u64>,
    pub timeout_start_sec: Option<u64>,
    pub timeout_stop_sec: Option<u64>,
    pub depends_on: Vec<String>,
    pub wanted_by: Vec<String>,
    pub required_by: Vec<String>,
    pub after: Vec<String>,
    pub before: Vec<String>,
    pub conflicts: Vec<String>,
    pub environment: BTreeMap<String, String>,
    pub environment_files: Vec<String>,
    pub standard_input: Option<String>,
    pub standard_output: Option<String>,
    pub standard_error: Option<String>,
    pub nice: Option<i32>,
    pub oom_score_adjust: Option<i32>,
    pub cpu_affinity: Option<Vec<u32>>,
    pub limit_nofile: Option<u64>,
    pub limit_nproc: Option<u64>,
    pub limit_core: Option<String>,
    pub readonly_paths: Vec<String>,
    pub readwrite_paths: Vec<String>,
    pub private_tmp: bool,
    pub private_devices: bool,
    pub protect_system: bool,
    pub protect_home: bool,
    pub no_new_privileges: bool,
    pub capability_bounding_set: Option<Vec<String>>,
    pub ambient_capabilities: Option<Vec<String>>,
}

impl ServiceConfig {
    /// Create a new service configuration
    pub fn new(name: String, description: String, exec_start: String) -> Self {
        Self {
            name,
            description,
            service_type: ServiceType::Simple,
            exec_start,
            exec_start_pre: Vec::new(),
            exec_start_post: Vec::new(),
            exec_reload: None,
            exec_stop: None,
            exec_stop_post: Vec::new(),
            working_directory: None,
            user: None,
            group: None,
            restart_policy: RestartPolicy::OnFailure,
            restart_sec: None,
            timeout_start_sec: Some(90),
            timeout_stop_sec: Some(90),
            depends_on: Vec::new(),
            wanted_by: Vec::new(),
            required_by: Vec::new(),
            after: Vec::new(),
            before: Vec::new(),
            conflicts: Vec::new(),
            environment: BTreeMap::new(),
            environment_files: Vec::new(),
            standard_input: None,
            standard_output: None,
            standard_error: None,
            nice: None,
            oom_score_adjust: None,
            cpu_affinity: None,
            limit_nofile: None,
            limit_nproc: None,
            limit_core: None,
            readonly_paths: Vec::new(),
            readwrite_paths: Vec::new(),
            private_tmp: false,
            private_devices: false,
            protect_system: false,
            protect_home: false,
            no_new_privileges: false,
            capability_bounding_set: None,
            ambient_capabilities: None,
        }
    }

    /// Set service type
    pub fn set_type(mut self, service_type: ServiceType) -> Self {
        self.service_type = service_type;
        self
    }

    /// Add pre-start command
    pub fn pre_start(mut self, command: impl Into<String>) -> Self {
        self.exec_start_pre.push(command.into());
        self
    }

    /// Add post-start command
    pub fn post_start(mut self, command: impl Into<String>) -> Self {
        self.exec_start_post.push(command.into());
        self
    }

    /// Set reload command
    pub fn reload(mut self, command: impl Into<String>) -> Self {
        self.exec_reload = Some(command.into());
        self
    }

    /// Set stop command
    pub fn stop(mut self, command: impl Into<String>) -> Self {
        self.exec_stop = Some(command.into());
        self
    }

    /// Add post-stop command
    pub fn post_stop(mut self, command: impl Into<String>) -> Self {
        self.exec_stop_post.push(command.into());
        self
    }

    /// Set working directory
    pub fn working_dir(mut self, dir: impl Into<String>) -> Self {
        self.working_directory = Some(dir.into());
        self
    }

    /// Set user
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    /// Set group
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Set restart policy
    pub fn restart_policy(mut self, policy: RestartPolicy) -> Self {
        self.restart_policy = policy;
        self
    }

    /// Set restart delay
    pub fn restart_delay(mut self, delay: u64) -> Self {
        self.restart_sec = Some(delay);
        self
    }

    /// Set timeout for start
    pub fn start_timeout(mut self, timeout: u64) -> Self {
        self.timeout_start_sec = Some(timeout);
        self
    }

    /// Set timeout for stop
    pub fn stop_timeout(mut self, timeout: u64) -> Self {
        self.timeout_stop_sec = Some(timeout);
        self
    }

    /// Add dependency
    pub fn depends_on(mut self, dep: impl Into<String>) -> Self {
        self.depends_on.push(dep.into());
        self
    }

    /// Add wanted_by
    pub fn wanted_by(mut self, target: impl Into<String>) -> Self {
        self.wanted_by.push(target.into());
        self
    }

    /// Add required_by
    pub fn required_by(mut self, target: impl Into<String>) -> Self {
        self.required_by.push(target.into());
        self
    }

    /// Add after dependency
    pub fn after(mut self, dep: impl Into<String>) -> Self {
        self.after.push(dep.into());
        self
    }

    /// Add before dependency
    pub fn before(mut self, dep: impl Into<String>) -> Self {
        self.before.push(dep.into());
        self
    }

    /// Add conflict
    pub fn conflicts(mut self, conflict: impl Into<String>) -> Self {
        self.conflicts.push(conflict.into());
        self
    }

    /// Set environment variable
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.environment.insert(key.into(), value.into());
        self
    }

    /// Add environment file
    pub fn environment_file(mut self, file: impl Into<String>) -> Self {
        self.environment_files.push(file.into());
        self
    }

    /// Set standard input
    pub fn stdin(mut self, input: impl Into<String>) -> Self {
        self.standard_input = Some(input.into());
        self
    }

    /// Set standard output
    pub fn stdout(mut self, output: impl Into<String>) -> Self {
        self.standard_output = Some(output.into());
        self
    }

    /// Set standard error
    pub fn stderr(mut self, error: impl Into<String>) -> Self {
        self.standard_error = Some(error.into());
        self
    }

    /// Set nice value
    pub fn nice(mut self, nice: i32) -> Self {
        self.nice = Some(nice);
        self
    }

    /// Set OOM score adjustment
    pub fn oom_score_adjust(mut self, score: i32) -> Self {
        self.oom_score_adjust = Some(score);
        self
    }

    /// Set CPU affinity
    pub fn cpu_affinity(mut self, cpus: Vec<u32>) -> Self {
        self.cpu_affinity = Some(cpus);
        self
    }

    /// Set file descriptor limit
    pub fn limit_nofile(mut self, limit: u64) -> Self {
        self.limit_nofile = Some(limit);
        self
    }

    /// Set process limit
    pub fn limit_nproc(mut self, limit: u64) -> Self {
        self.limit_nproc = Some(limit);
        self
    }

    /// Set core limit
    pub fn limit_core(mut self, limit: impl Into<String>) -> Self {
        self.limit_core = Some(limit.into());
        self
    }

    /// Add read-only path
    pub fn readonly_path(mut self, path: impl Into<String>) -> Self {
        self.readonly_paths.push(path.into());
        self
    }

    /// Add read-write path
    pub fn readwrite_path(mut self, path: impl Into<String>) -> Self {
        self.readwrite_paths.push(path.into());
        self
    }

    /// Enable private tmp
    pub fn private_tmp(mut self, enable: bool) -> Self {
        self.private_tmp = enable;
        self
    }

    /// Enable private devices
    pub fn private_devices(mut self, enable: bool) -> Self {
        self.private_devices = enable;
        self
    }

    /// Enable system protection
    pub fn protect_system(mut self, enable: bool) -> Self {
        self.protect_system = enable;
        self
    }

    /// Enable home protection
    pub fn protect_home(mut self, enable: bool) -> Self {
        self.protect_home = enable;
        self
    }

    /// Enable no new privileges
    pub fn no_new_privileges(mut self, enable: bool) -> Self {
        self.no_new_privileges = enable;
        self
    }

    /// Set capability bounding set
    pub fn capability_bounding_set(mut self, caps: Vec<String>) -> Self {
        self.capability_bounding_set = Some(caps);
        self
    }

    /// Set ambient capabilities
    pub fn ambient_capabilities(mut self, caps: Vec<String>) -> Self {
        self.ambient_capabilities = Some(caps);
        self
    }

    /// Convert to unit file format
    pub fn to_unit_file(&self) -> String {
        let mut content = String::new();

        content.push_str(&format!("[Unit]\n"));
        content.push_str(&format!("Description={}\n", self.description));
        content.push_str(&format!("Type={}\n", self.service_type.as_str()));

        if !self.depends_on.is_empty() {
            content.push_str(&format!("Requires={}\n", self.depends_on.join(" ")));
        }
        if !self.wanted_by.is_empty() {
            content.push_str(&format!("WantedBy={}\n", self.wanted_by.join(" ")));
        }
        if !self.required_by.is_empty() {
            content.push_str(&format!("RequiredBy={}\n", self.required_by.join(" ")));
        }
        if !self.after.is_empty() {
            content.push_str(&format!("After={}\n", self.after.join(" ")));
        }
        if !self.before.is_empty() {
            content.push_str(&format!("Before={}\n", self.before.join(" ")));
        }
        if !self.conflicts.is_empty() {
            content.push_str(&format!("Conflicts={}\n", self.conflicts.join(" ")));
        }

        content.push_str("\n[Service]\n");

        for cmd in &self.exec_start_pre {
            content.push_str(&format!("ExecStartPre={}\n", cmd));
        }

        content.push_str(&format!("ExecStart={}\n", self.exec_start));

        for cmd in &self.exec_start_post {
            content.push_str(&format!("ExecStartPost={}\n", cmd));
        }

        if let Some(reload) = &self.exec_reload {
            content.push_str(&format!("ExecReload={}\n", reload));
        }

        if let Some(stop) = &self.exec_stop {
            content.push_str(&format!("ExecStop={}\n", stop));
        }

        for cmd in &self.exec_stop_post {
            content.push_str(&format!("ExecStopPost={}\n", cmd));
        }

        if let Some(dir) = &self.working_directory {
            content.push_str(&format!("WorkingDirectory={}\n", dir));
        }

        if let Some(user) = &self.user {
            content.push_str(&format!("User={}\n", user));
        }

        if let Some(group) = &self.group {
            content.push_str(&format!("Group={}\n", group));
        }

        content.push_str(&format!("Restart={}\n", self.restart_policy.as_str()));

        if let Some(delay) = self.restart_sec {
            content.push_str(&format!("RestartSec={}\n", delay));
        }

        if let Some(timeout) = self.timeout_start_sec {
            content.push_str(&format!("TimeoutStartSec={}\n", timeout));
        }

        if let Some(timeout) = self.timeout_stop_sec {
            content.push_str(&format!("TimeoutStopSec={}\n", timeout));
        }

        for (key, value) in &self.environment {
            content.push_str(&format!("Environment=\"{}={}\"\n", key, value));
        }

        for file in &self.environment_files {
            content.push_str(&format!("EnvironmentFile={}\n", file));
        }

        if let Some(input) = &self.standard_input {
            content.push_str(&format!("StandardInput={}\n", input));
        }

        if let Some(output) = &self.standard_output {
            content.push_str(&format!("StandardOutput={}\n", output));
        }

        if let Some(error) = &self.standard_error {
            content.push_str(&format!("StandardError={}\n", error));
        }

        if let Some(nice) = self.nice {
            content.push_str(&format!("Nice={}\n", nice));
        }

        if let Some(score) = self.oom_score_adjust {
            content.push_str(&format!("OOMScoreAdjust={}\n", score));
        }

        if let Some(cpus) = &self.cpu_affinity {
            content.push_str(&format!("CPUAffinity={}\n", cpus.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" ")));
        }

        if let Some(limit) = self.limit_nofile {
            content.push_str(&format!("LimitNOFILE={}\n", limit));
        }

        if let Some(limit) = self.limit_nproc {
            content.push_str(&format!("LimitNPROC={}\n", limit));
        }

        if let Some(limit) = &self.limit_core {
            content.push_str(&format!("LimitCORE={}\n", limit));
        }

        for path in &self.readonly_paths {
            content.push_str(&format!("ReadOnlyPaths={}\n", path));
        }

        for path in &self.readwrite_paths {
            content.push_str(&format!("ReadWritePaths={}\n", path));
        }

        if self.private_tmp {
            content.push_str("PrivateTmp=true\n");
        }

        if self.private_devices {
            content.push_str("PrivateDevices=true\n");
        }

        if self.protect_system {
            content.push_str("ProtectSystem=true\n");
        }

        if self.protect_home {
            content.push_str("ProtectHome=true\n");
        }

        if self.no_new_privileges {
            content.push_str("NoNewPrivileges=true\n");
        }

        if let Some(caps) = &self.capability_bounding_set {
            content.push_str(&format!("CapabilityBoundingSet={}\n", caps.join(" ")));
        }

        if let Some(caps) = &self.ambient_capabilities {
            content.push_str(&format!("AmbientCapabilities={}\n", caps.join(" ")));
        }

        content
    }
}

/// Service instance
#[derive(Debug, Clone)]
pub struct Service {
    pub config: ServiceConfig,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub main_pid: Option<u32>,
    pub start_time: Option<u64>,
    pub restart_count: u32,
    pub last_exit_status: Option<i32>,
    pub last_exit_time: Option<u64>,
}

impl Service {
    pub fn new(config: ServiceConfig) -> Self {
        Self {
            config,
            state: ServiceState::Stopped,
            pid: None,
            main_pid: None,
            start_time: None,
            restart_count: 0,
            last_exit_status: None,
            last_exit_time: None,
        }
    }

    pub fn set_state(&mut self, state: ServiceState) {
        self.state = state;
    }

    pub fn set_pid(&mut self, pid: u32) {
        self.pid = Some(pid);
        self.main_pid = Some(pid);
    }

    pub fn set_start_time(&mut self, time: u64) {
        self.start_time = Some(time);
    }

    pub fn set_exit_status(&mut self, status: i32, time: u64) {
        self.last_exit_status = Some(status);
        self.last_exit_time = Some(time);
    }

    pub fn increment_restart_count(&mut self) {
        self.restart_count += 1;
    }

    pub fn get_uptime(&self) -> Option<u64> {
        if let (Some(start), Some(now)) = (self.start_time, SystemTime::now().duration_since(UNIX_EPOCH).ok().map(|d| d.as_secs())) {
            Some(now - start)
        } else {
            None
        }
    }

    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    pub fn is_failed(&self) -> bool {
        self.state.is_failed()
    }
}

/// Service manager
#[derive(Debug, Clone)]
pub struct ServiceManager {
    pub services: BTreeMap<String, Service>,
    pub unit_files_dir: PathBuf,
    pub runtime_dir: PathBuf,
    pub systemctl: bool,
    pub systemd_available: bool,
}

impl ServiceManager {
    /// Create a new service manager
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            unit_files_dir: PathBuf::from("/etc/systemd/system"),
            runtime_dir: PathBuf::from("/run/systemd/system"),
            systemctl: true,
            systemd_available: true,
        }
    }

    /// Create service manager with custom paths
    pub fn with_paths(unit_files_dir: PathBuf, runtime_dir: PathBuf) -> Self {
        Self {
            services: BTreeMap::new(),
            unit_files_dir,
            runtime_dir,
            systemctl: true,
            systemd_available: true,
        }
    }

    /// Load services from unit files
    pub fn load_services(&mut self) -> io::Result<()> {
        if !self.unit_files_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.unit_files_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "service") {
                let name = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(config) = self.parse_unit_file(&content) {
                        let service = Service::new(config);
                        self.services.insert(name.clone(), service);
                    }
                }
            }
        }

        Ok(())
    }

    /// Parse unit file content
    pub fn parse_unit_file(&self, content: &str) -> Result<ServiceConfig, String> {
        let mut config = ServiceConfig::new(
            String::new(),
            String::new(),
            String::new(),
        );

        let mut in_unit = false;
        let mut in_service = false;

        for line in content.lines() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                let section = &line[1..line.len()-1];
                in_unit = section == "Unit";
                in_service = section == "Service";
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                if in_unit {
                    match key {
                        "Description" => config.description = value.to_string(),
                        "After" => config.after = value.split_whitespace().map(String::from).collect(),
                        "Before" => config.before = value.split_whitespace().map(String::from).collect(),
                        "Requires" => config.depends_on = value.split_whitespace().map(String::from).collect(),
                        "WantedBy" => config.wanted_by = value.split_whitespace().map(String::from).collect(),
                        "RequiredBy" => config.required_by = value.split_whitespace().map(String::from).collect(),
                        "Conflicts" => config.conflicts = value.split_whitespace().map(String::from).collect(),
                        _ => {}
                    }
                } else if in_service {
                    match key {
                        "Type" => config.service_type = ServiceType::from_str(value).unwrap_or(ServiceType::Simple),
                        "ExecStart" => config.exec_start = value.to_string(),
                        "ExecStartPre" => config.exec_start_pre.push(value.to_string()),
                        "ExecStartPost" => config.exec_start_post.push(value.to_string()),
                        "ExecReload" => config.exec_reload = Some(value.to_string()),
                        "ExecStop" => config.exec_stop = Some(value.to_string()),
                        "ExecStopPost" => config.exec_stop_post.push(value.to_string()),
                        "WorkingDirectory" => config.working_directory = Some(value.to_string()),
                        "User" => config.user = Some(value.to_string()),
                        "Group" => config.group = Some(value.to_string()),
                        "Restart" => config.restart_policy = match value {
                            "no" => RestartPolicy::No,
                            "on-success" => RestartPolicy::OnSuccess,
                            "on-failure" => RestartPolicy::OnFailure,
                            "on-abnormal" => RestartPolicy::OnAbnormal,
                            "on-watchdog" => RestartPolicy::OnWatchdog,
                            "on-abort" => RestartPolicy::OnAbort,
                            "always" => RestartPolicy::Always,
                            _ => RestartPolicy::OnFailure,
                        },
                        "RestartSec" => config.restart_sec = value.parse().ok(),
                        "TimeoutStartSec" => config.timeout_start_sec = value.parse().ok(),
                        "TimeoutStopSec" => config.timeout_stop_sec = value.parse().ok(),
                        "Environment" => {
                            if let Some((k, v)) = value.split_once('=') {
                                config.environment.insert(k.trim().to_string(), v.trim().trim_matches('"').to_string());
                            }
                        }
                        "EnvironmentFile" => config.environment_files.push(value.to_string()),
                        "StandardInput" => config.standard_input = Some(value.to_string()),
                        "StandardOutput" => config.standard_output = Some(value.to_string()),
                        "StandardError" => config.standard_error = Some(value.to_string()),
                        "Nice" => config.nice = value.parse().ok(),
                        "OOMScoreAdjust" => config.oom_score_adjust = value.parse().ok(),
                        "PrivateTmp" => config.private_tmp = value == "true",
                        "PrivateDevices" => config.private_devices = value == "true",
                        "ProtectSystem" => config.protect_system = value == "true",
                        "ProtectHome" => config.protect_home = value == "true",
                        "NoNewPrivileges" => config.no_new_privileges = value == "true",
                        _ => {}
                    }
                }
            }
        }

        // Extract service name from description or use first word of exec_start
        if config.name.is_empty() {
            config.name = config.description.split_whitespace().next().unwrap_or("service").to_string();
        }

        Ok(config)
    }

    /// Save service to unit file
    pub fn save_service(&self, service: &Service) -> io::Result<()> {
        let path = self.unit_files_dir.join(format!("{}.service", service.config.name));
        let content = service.config.to_unit_file();
        fs::write(path, content)
    }

    /// Add service
    pub fn add_service(&mut self, config: ServiceConfig) -> Result<(), String> {
        if self.services.contains_key(&config.name) {
            return Err(format!("Service {} already exists", config.name));
        }

        let service = Service::new(config);
        self.services.insert(service.config.name.clone(), service);
        Ok(())
    }

    /// Remove service
    pub fn remove_service(&mut self, name: &str) -> Result<(), String> {
        if let Some(service) = self.services.get(name) {
            if service.is_active() {
                return Err(format!("Service {} is active, stop it first", name));
            }
        }

        if self.services.remove(name).is_some() {
            // Remove unit file
            let path = self.unit_files_dir.join(format!("{}.service", name));
            let _ = fs::remove_file(path);
            Ok(())
        } else {
            Err(format!("Service {} not found", name))
        }
    }

    /// Get service
    pub fn get_service(&self, name: &str) -> Option<&Service> {
        self.services.get(name)
    }

    /// Get service mutably
    pub fn get_service_mut(&mut self, name: &str) -> Option<&mut Service> {
        self.services.get_mut(name)
    }

    /// Start service
    pub fn start_service(&mut self, name: &str) -> Result<(), String> {
        // Check if service exists
        if !self.services.contains_key(name) {
            return Err(format!("Service {} not found", name));
        }

        // Check dependencies
        let dependencies: Vec<String> = self.services.get(name)
            .map(|s| s.config.depends_on.clone())
            .unwrap_or_default();

        for dep in &dependencies {
            if let Some(dep_service) = self.services.get(dep) {
                if !dep_service.is_active() {
                    return Err(format!("Dependency {} is not active", dep));
                }
            }
        }

        let service = self.services.get_mut(name).unwrap();

        if service.is_active() {
            return Err(format!("Service {} is already active", name));
        }

        // Check conflicts
        for conflict in &service.config.conflicts {
            if let Some(conflict_service) = self.services.get(conflict) {
                if conflict_service.is_active() {
                    return Err(format!("Service {} conflicts with {}", name, conflict));
                }
            }
        }

        // Set state to starting
        service.set_state(ServiceState::Starting);

        // Execute pre-start commands
        for cmd in &service.config.exec_start_pre {
            // In real implementation, would execute command
            println!("Executing pre-start: {}", cmd);
        }

        // Execute main command
        // In real implementation, would spawn process
        service.set_state(ServiceState::Running);
        service.set_pid(1234); // Simulated PID
        service.set_start_time(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs());

        // Execute post-start commands
        for cmd in &service.config.exec_start_post {
            // In real implementation, would execute command
            println!("Executing post-start: {}", cmd);
        }

        Ok(())
    }

    /// Stop service
    pub fn stop_service(&mut self, name: &str) -> Result<(), String> {
        let service = self.services.get_mut(name)
            .ok_or_else(|| format!("Service {} not found", name))?;

        if !service.is_active() {
            return Err(format!("Service {} is not active", name));
        }

        service.set_state(ServiceState::Stopping);

        // Execute stop command
        if let Some(stop_cmd) = &service.config.exec_stop {
            // In real implementation, would execute command
            println!("Executing stop: {}", stop_cmd);
        }

        // Execute post-stop commands
        for cmd in &service.config.exec_stop_post {
            // In real implementation, would execute command
            println!("Executing post-stop: {}", cmd);
        }

        // Kill process
        service.set_state(ServiceState::Stopped);
        service.pid = None;
        service.main_pid = None;
        service.start_time = None;
        service.set_exit_status(0, SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs());

        Ok(())
    }

    /// Restart service
    pub fn restart_service(&mut self, name: &str) -> Result<(), String> {
        self.stop_service(name)?;
        self.start_service(name)?;

        if let Some(service) = self.services.get_mut(name) {
            service.increment_restart_count();
        }

        Ok(())
    }

    /// Reload service
    pub fn reload_service(&mut self, name: &str) -> Result<(), String> {
        let service = self.services.get_mut(name)
            .ok_or_else(|| format!("Service {} not found", name))?;

        if !service.is_active() {
            return Err(format!("Service {} is not active", name));
        }

        if let Some(reload_cmd) = &service.config.exec_reload {
            service.set_state(ServiceState::Reloading);
            // In real implementation, would execute command
            println!("Executing reload: {}", reload_cmd);
            service.set_state(ServiceState::Running);
            Ok(())
        } else {
            Err(format!("Service {} does not support reload", name))
        }
    }

    /// Enable service (create symlink for auto-start)
    pub fn enable_service(&mut self, name: &str) -> Result<(), String> {
        let service = self.services.get_mut(name)
            .ok_or_else(|| format!("Service {} not found", name))?;

        // In real implementation, would create symlinks in wanted_by directories
        println!("Enabling service: {}", name);
        for target in &service.config.wanted_by {
            println!("  Creating symlink in: {}", target);
        }

        Ok(())
    }

    /// Disable service (remove symlinks)
    pub fn disable_service(&mut self, name: &str) -> Result<(), String> {
        let service = self.services.get(name)
            .ok_or_else(|| format!("Service {} not found", name))?;

        // In real implementation, would remove symlinks
        println!("Disabling service: {}", name);
        for target in &service.config.wanted_by {
            println!("  Removing symlink from: {}", target);
        }

        Ok(())
    }

    /// Get service status
    pub fn get_service_status(&self, name: &str) -> Result<String, String> {
        let service = self.services.get(name)
            .ok_or_else(|| format!("Service {} not found", name))?;

        let mut status = String::new();
        status.push_str(&format!("● {}\n", service.config.name));
        status.push_str(&format!("  Loaded: loaded (/etc/systemd/system/{}.service)\n", service.config.name));
        status.push_str(&format!("  Active: {} ({})\n", service.state.as_str(), service.state.as_str()));

        if let Some(pid) = service.main_pid {
            status.push_str(&format!("  Main PID: {} ({})\n", pid, service.config.name));
        }

        if let Some(uptime) = service.get_uptime() {
            status.push_str(&format!("  Status: \"Running for {}s\"\n", uptime));
        }

        status.push_str(&format!("  CGroup: /system.slice/{}.service\n", service.config.name));

        Ok(status)
    }

    /// List all services
    pub fn list_services(&self) -> Vec<String> {
        self.services.values()
            .map(|s| format!("{} ({}, {})", s.config.name, s.state.as_str(), s.config.description))
            .collect()
    }

    /// List running services
    pub fn list_running_services(&self) -> Vec<String> {
        self.services.values()
            .filter(|s| s.is_active())
            .map(|s| format!("{} (PID: {})", s.config.name, s.main_pid.map(|p| p.to_string()).unwrap_or("N/A".to_string())))
            .collect()
    }

    /// List failed services
    pub fn list_failed_services(&self) -> Vec<String> {
        self.services.values()
            .filter(|s| s.is_failed())
            .map(|s| format!("{} ({})", s.config.name, s.config.description))
            .collect()
    }

    /// Get service statistics
    pub fn get_statistics(&self) -> ServiceStatistics {
        let total = self.services.len();
        let running = self.services.values().filter(|s| s.is_active()).count();
        let stopped = self.services.values().filter(|s| s.state == ServiceState::Stopped).count();
        let failed = self.services.values().filter(|s| s.is_failed()).count();

        ServiceStatistics {
            total,
            running,
            stopped,
            failed,
            uptime: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
        }
    }

    /// Add SSH service
    pub fn add_ssh_service(&mut self) -> Result<(), String> {
        let config = ServiceConfig::new(
            "sshd".to_string(),
            "OpenSSH server daemon".to_string(),
            "/usr/sbin/sshd -D".to_string(),
        )
        .set_type(ServiceType::Notify)
        .stop("/usr/bin/pkill sshd".to_string())
        .depends_on("network.target".to_string())
        .wanted_by("multi-user.target".to_string())
        .after("network.target".to_string());

        self.add_service(config)
    }

    /// Add firewall service
    pub fn add_firewall_service(&mut self) -> Result<(), String> {
        let config = ServiceConfig::new(
            "firewall".to_string(),
            "Network firewall service".to_string(),
            "/usr/sbin/sigma-firewall start".to_string(),
        )
        .stop("/usr/sbin/sigma-firewall stop".to_string())
        .depends_on("network.target".to_string())
        .wanted_by("multi-user.target".to_string())
        .after("network.target".to_string());

        self.add_service(config)
    }

    /// Add cron service
    pub fn add_cron_service(&mut self) -> Result<(), String> {
        let config = ServiceConfig::new(
            "cron".to_string(),
            "Regular background program processing daemon".to_string(),
            "/usr/sbin/cron -f".to_string(),
        )
        .wanted_by("multi-user.target".to_string())
        .after("network.target".to_string());

        self.add_service(config)
    }

    /// Add syslog service
    pub fn add_syslog_service(&mut self) -> Result<(), String> {
        let config = ServiceConfig::new(
            "syslog".to_string(),
            "System Logging Service".to_string(),
            "/usr/sbin/syslogd -O /var/log/syslog".to_string(),
        )
        .wanted_by("multi-user.target".to_string())
        .after("network.target".to_string());

        self.add_service(config)
    }

    /// Add NTP service
    pub fn add_ntp_service(&mut self) -> Result<(), String> {
        let config = ServiceConfig::new(
            "ntp".to_string(),
            "Network Time Protocol daemon".to_string(),
            "/usr/sbin/ntpd -g".to_string(),
        )
        .wanted_by("multi-user.target".to_string())
        .after("network.target".to_string());

        self.add_service(config)
    }
}

impl Default for ServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Service statistics
#[derive(Debug, Clone)]
pub struct ServiceStatistics {
    pub total: usize,
    pub running: usize,
    pub stopped: usize,
    pub failed: usize,
    pub uptime: u64,
}

/// Unit file parser for systemd-style unit files
pub struct UnitFileParser;

impl UnitFileParser {
    /// Parse unit file from string
    pub fn parse(content: &str) -> Result<ServiceConfig, String> {
        let manager = ServiceManager::new();
        manager.parse_unit_file(content)
    }

    /// Parse unit file from file
    pub fn parse_file(path: &Path) -> Result<ServiceConfig, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        Self::parse(&content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_state() {
        assert_eq!(ServiceState::Running.as_str(), "running");
        assert_eq!(ServiceState::from_str("stopped"), Some(ServiceState::Stopped));
        assert!(ServiceState::Running.is_active());
        assert!(!ServiceState::Stopped.is_active());
    }

    #[test]
    fn test_service_type() {
        assert_eq!(ServiceType::Simple.as_str(), "simple");
        assert_eq!(ServiceType::from_str("notify"), Some(ServiceType::Notify));
    }

    #[test]
    fn test_restart_policy() {
        assert!(RestartPolicy::Always.should_restart(ServiceState::Failed));
        assert!(!RestartPolicy::No.should_restart(ServiceState::Failed));
        assert!(RestartPolicy::OnFailure.should_restart(ServiceState::Failed));
        assert!(!RestartPolicy::OnFailure.should_restart(ServiceState::Stopped));
    }

    #[test]
    fn test_service_config_builder() {
        let config = ServiceConfig::new("test".to_string(), "Test service".to_string(), "/usr/bin/test".to_string())
            .set_type(ServiceType::Notify)
            .depends_on("network.target".to_string())
            .wanted_by("multi-user.target".to_string())
            .restart_policy(RestartPolicy::Always)
            .env("TEST_VAR", "test_value");

        assert_eq!(config.name, "test");
        assert_eq!(config.service_type, ServiceType::Notify);
        assert!(config.depends_on.contains(&"network.target".to_string()));
    }

    #[test]
    fn test_service_config_to_unit_file() {
        let config = ServiceConfig::new("test".to_string(), "Test service".to_string(), "/usr/bin/test".to_string())
            .depends_on("network.target".to_string())
            .wanted_by("multi-user.target".to_string());

        let unit_file = config.to_unit_file();
        assert!(unit_file.contains("[Unit]"));
        assert!(unit_file.contains("[Service]"));
        assert!(unit_file.contains("Description=Test service"));
        assert!(unit_file.contains("ExecStart=/usr/bin/test"));
    }

    #[test]
    fn test_service_manager() {
        let mut manager = ServiceManager::new();

        let config = ServiceConfig::new("test".to_string(), "Test service".to_string(), "/usr/bin/test".to_string());
        manager.add_service(config).unwrap();

        assert!(manager.get_service("test").is_some());
        assert!(manager.start_service("test").is_ok());
        assert!(manager.stop_service("test").is_ok());
    }

    #[test]
    fn test_service_dependencies() {
        let mut manager = ServiceManager::new();

        let dep_config = ServiceConfig::new("dep".to_string(), "Dependency".to_string(), "/usr/bin/dep".to_string());
        let service_config = ServiceConfig::new("service".to_string(), "Service".to_string(), "/usr/bin/service".to_string())
            .depends_on("dep".to_string());

        manager.add_service(dep_config).unwrap();
        manager.add_service(service_config).unwrap();

        // Should fail to start service without starting dependency
        assert!(manager.start_service("service").is_err());

        // Start dependency first
        manager.start_service("dep").unwrap();
        assert!(manager.start_service("service").is_ok());
    }

    #[test]
    fn test_service_conflicts() {
        let mut manager = ServiceManager::new();

        let config1 = ServiceConfig::new("service1".to_string(), "Service 1".to_string(), "/usr/bin/s1".to_string());
        let config2 = ServiceConfig::new("service2".to_string(), "Service 2".to_string(), "/usr/bin/s2".to_string())
            .conflicts("service1".to_string());

        manager.add_service(config1).unwrap();
        manager.add_service(config2).unwrap();

        manager.start_service("service1").unwrap();
        assert!(manager.start_service("service2").is_err());
    }

    #[test]
    fn test_unit_file_parsing() {
        let content = r#"
[Unit]
Description=Test Service
After=network.target

[Service]
Type=notify
ExecStart=/usr/bin/test
Restart=always
Environment=TEST=value
PrivateTmp=true
"#;

        let config = UnitFileParser::parse(content).unwrap();
        assert_eq!(config.description, "Test Service");
        assert_eq!(config.service_type, ServiceType::Notify);
        assert_eq!(config.exec_start, "/usr/bin/test");
        assert!(config.private_tmp);
    }

    #[test]
    fn test_service_statistics() {
        let mut manager = ServiceManager::new();

        let config = ServiceConfig::new("test".to_string(), "Test".to_string(), "/usr/bin/test".to_string());
        manager.add_service(config).unwrap();
        manager.start_service("test").unwrap();

        let stats = manager.get_statistics();
        assert_eq!(stats.total, 1);
        assert_eq!(stats.running, 1);
    }
}
