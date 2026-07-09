// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Linux-inspired system services framework for SigmaOS
// Zero-allocation, performance-optimized service management

/// Service trait for system services
pub trait Service {
    /// Initialize service
    fn init(&mut self) -> Result<(), ServiceError>;
    
    /// Start service
    fn start(&mut self) -> Result<(), ServiceError>;
    
    /// Stop service
    fn stop(&mut self) -> Result<(), ServiceError>;
    
    /// Restart service
    fn restart(&mut self) -> Result<(), ServiceError>;
    
    /// Reload service configuration
    fn reload(&mut self) -> Result<(), ServiceError>;
    
    /// Get service status
    fn status(&self) -> ServiceStatus;
    
    /// Get service name
    fn name(&self) -> &str;
    
    /// Get service description
    fn description(&self) -> &str;
    
    /// Check if service is enabled
    fn is_enabled(&self) -> bool;
    
    /// Enable service
    fn enable(&mut self) -> Result<(), ServiceError>;
    
    /// Disable service
    fn disable(&mut self) -> Result<(), ServiceError>;
}

/// Service manager trait
pub trait ServiceManager {
    /// Register service
    fn register(&mut self, service: Box<dyn Service>) -> Result<(), ServiceError>;
    
    /// Unregister service
    fn unregister(&mut self, name: &str) -> Result<(), ServiceError>;
    
    /// Start service by name
    fn start_service(&mut self, name: &str) -> Result<(), ServiceError>;
    
    /// Stop service by name
    fn stop_service(&mut self, name: &str) -> Result<(), ServiceError>;
    
    /// Restart service by name
    fn restart_service(&mut self, name: &str) -> Result<(), ServiceError>;
    
    /// Get service by name
    fn get_service(&self, name: &str) -> Option<&dyn Service>;
    
    /// Get mutable service by name
    fn get_service_mut(&mut self, name: &str) -> Option<&mut dyn Service>;
    
    /// List all services
    fn list_services(&self) -> Vec<&str>;
    
    /// List active services
    fn list_active_services(&self) -> Vec<&str>;
    
    /// List enabled services
    fn list_enabled_services(&self) -> Vec<&str>;
}

/// Service status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    Unknown,
    Loaded,
    Active,
    Inactive,
    Failed,
    Activating,
    Deactivating,
}

/// Service error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceError {
    ServiceNotFound,
    ServiceAlreadyRunning,
    ServiceNotRunning,
    DependencyFailed,
    PermissionDenied,
    InvalidConfiguration,
    Timeout,
    ResourceUnavailable,
    Other,
}

/// Service unit types (systemd-style)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceType {
    Service,
    Socket,
    BusName,
    Target,
    Device,
    Mount,
    Automount,
    Swap,
    Timer,
    Path,
}

/// Service dependency types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyType {
    Requires,
    Wants,
    Requisite,
    Conflicts,
    Before,
    After,
    OnFailure,
}

/// Service configuration
pub struct ServiceConfig {
    pub name: String,
    pub description: String,
    pub service_type: ServiceType,
    pub dependencies: Vec<(DependencyType, String)>,
    pub exec_start: Vec<String>,
    pub exec_stop: Vec<String>,
    pub exec_reload: Vec<String>,
    pub restart_policy: RestartPolicy,
    pub environment: Vec<(String, String)>,
    pub working_directory: Option<String>,
    pub user: Option<String>,
    pub group: Option<String>,
}

impl ServiceConfig {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            service_type: ServiceType::Service,
            dependencies: Vec::new(),
            exec_start: Vec::new(),
            exec_stop: Vec::new(),
            exec_reload: Vec::new(),
            restart_policy: RestartPolicy::No,
            environment: Vec::new(),
            working_directory: None,
            user: None,
            group: None,
        }
    }
    
    pub fn add_dependency(&mut self, dep_type: DependencyType, target: &str) {
        self.dependencies.push((dep_type, target.to_string()));
    }
    
    pub fn add_exec_start(&mut self, command: &str) {
        self.exec_start.push(command.to_string());
    }
    
    pub fn add_environment(&mut self, key: &str, value: &str) {
        self.environment.push((key.to_string(), value.to_string()));
    }
}

/// Restart policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    No,
    OnSuccess,
    OnFailure,
    OnAbnormal,
    Always,
}

/// Service state
pub struct ServiceState {
    pub status: ServiceStatus,
    pub pid: Option<u32>,
    pub main_pid: Option<u32>,
    pub exit_code: Option<i32>,
    pub start_time: Option<u64>,
    pub restart_count: u32,
}

impl ServiceState {
    pub const fn new() -> Self {
        Self {
            status: ServiceStatus::Unknown,
            pid: None,
            main_pid: None,
            exit_code: None,
            start_time: None,
            restart_count: 0,
        }
    }
}

/// Target unit (systemd-style)
pub trait TargetUnit: Service {
    /// Get target name
    fn target_name(&self) -> &str;
    
    /// Get required services
    fn required_services(&self) -> Vec<&str>;
    
    /// Get wanted services
    fn wanted_services(&self) -> Vec<&str>;
}

/// Socket unit (systemd-style)
pub trait SocketUnit: Service {
    /// Get socket path
    fn socket_path(&self) -> &str;
    
    /// Get socket type
    fn socket_type(&self) -> SocketType;
    
    /// Get listening address
    fn listen_address(&self) -> &str;
}

/// Socket types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Stream,
    Datagram,
    SequentialPacket,
}

/// Timer unit (systemd-style)
pub trait TimerUnit: Service {
    /// Get timer specification
    fn timer_spec(&self) -> &str;
    
    /// Get accuracy
    fn accuracy(&self) -> u64;
    
    /// Check if timer is persistent
    fn persistent(&self) -> bool;
}

/// Service runtime information
pub struct ServiceRuntime {
    pub cpu_time: u64,
    pub memory_usage: u64,
    pub file_descriptors: u32,
    pub threads: u32,
    pub uptime: u64,
}

impl ServiceRuntime {
    pub const fn new() -> Self {
        Self {
            cpu_time: 0,
            memory_usage: 0,
            file_descriptors: 0,
            threads: 0,
            uptime: 0,
        }
    }
}

/// Service log entry
pub struct ServiceLogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub message: String,
    pub source: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Emergency,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Info,
    Debug,
}

/// Service journal (logging)
pub struct ServiceJournal {
    pub entries: Vec<ServiceLogEntry>,
    pub max_entries: usize,
}

impl ServiceJournal {
    pub const fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }
    
    pub fn add_entry(&mut self, entry: ServiceLogEntry) {
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(entry);
    }
    
    pub fn get_entries(&self) -> &[ServiceLogEntry] {
        &self.entries
    }
    
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

/// Standard system targets
pub mod targets {
    pub const DEFAULT_TARGET: &str = "default.target";
    pub const MULTI_USER_TARGET: &str = "multi-user.target";
    pub const GRAPHICAL_TARGET: &str = "graphical.target";
    pub const REBOOT_TARGET: &str = "reboot.target";
    pub const POWER_OFF_TARGET: &str = "poweroff.target";
    pub const RESCUE_TARGET: &str = "rescue.target";
    pub const EMERGENCY_TARGET: &str = "emergency.target";
}

/// Standard system services
pub mod services {
    pub const SYSTEMD: &str = "systemd";
    pub const NETWORKD: &str = "systemd-networkd";
    pub const RESOLVED: &str = "systemd-resolved";
    pub const TIMEDATED: &str = "systemd-timesyncd";
    pub const LOGIND: &str = "systemd-logind";
    pub const JOURNALD: &str = "systemd-journald";
    pub const UDEVD: &str = "systemd-udevd";
    pub const SYSLOGD: &str = "syslogd";
    pub const SSHD: &str = "sshd";
    pub const CROND: &str = "crond";
}
