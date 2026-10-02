// Service Manager for SigmaOS
// Service management per Wiki 06-Networking.md and general system services
// Provides systemd-style service management for SSH and other services

use std::string::{String, ToString};
use std::vec::Vec;

/// Service state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
}

impl SystemServiceState {
    pub fn as_str(&self) -> &str {
        match self {
            SystemServiceState::Stopped => "stopped",
            SystemServiceState::Starting => "starting",
            SystemServiceState::Running => "running",
            SystemServiceState::Stopping => "stopping",
            SystemServiceState::Failed => "failed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "stopped" => Some(SystemServiceState::Stopped),
            "starting" => Some(SystemServiceState::Starting),
            "running" => Some(SystemServiceState::Running),
            "stopping" => Some(SystemServiceState::Stopping),
            "failed" => Some(SystemServiceState::Failed),
            _ => None,
        }
    }
}

/// Service type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemServiceType {
    System,
    User,
    Socket,
}

impl SystemServiceType {
    pub fn as_str(&self) -> &str {
        match self {
            SystemServiceType::System => "system",
            SystemServiceType::User => "user",
            SystemServiceType::Socket => "socket",
        }
    }
}

/// Service restart policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemRestartPolicy {
    Never,
    OnFailure,
    Always,
}

impl SystemRestartPolicy {
    pub fn as_str(&self) -> &str {
        match self {
            SystemRestartPolicy::Never => "never",
            SystemRestartPolicy::OnFailure => "on-failure",
            SystemRestartPolicy::Always => "always",
        }
    }
}

/// Service configuration
#[derive(Debug, Clone)]
pub struct SystemServiceConfig {
    pub name: String,
    pub description: String,
    pub service_type: SystemServiceType,
    pub exec_start: String,
    pub exec_stop: Option<String>,
    pub restart_policy: SystemRestartPolicy,
    pub auto_start: bool,
    pub depends_on: Vec<String>,
}

impl SystemServiceConfig {
    pub fn new(name: String, description: String, exec_start: String) -> Self {
        SystemServiceConfig {
            name,
            description,
            service_type: SystemServiceType::System,
            exec_start,
            exec_stop: None,
            restart_policy: SystemRestartPolicy::OnFailure,
            auto_start: false,
            depends_on: Vec::new(),
        }
    }

    pub fn set_exec_stop(&mut self, exec_stop: String) {
        self.exec_stop = Some(exec_stop);
    }

    pub fn set_restart_policy(&mut self, policy: SystemRestartPolicy) {
        self.restart_policy = policy;
    }

    pub fn set_auto_start(&mut self, auto_start: bool) {
        self.auto_start = auto_start;
    }

    pub fn add_dependency(&mut self, dependency: String) {
        self.depends_on.push(dependency);
    }
}

/// Service instance
#[derive(Debug, Clone)]
pub struct SystemService {
    pub config: SystemServiceConfig,
    pub state: SystemServiceState,
    pub pid: Option<u32>,
    pub start_time: Option<u64>,
    pub restart_count: u32,
}

impl SystemService {
    pub fn new(config: SystemServiceConfig) -> Self {
        SystemService {
            config,
            state: SystemServiceState::Stopped,
            pid: None,
            start_time: None,
            restart_count: 0,
        }
    }

    pub fn set_state(&mut self, state: SystemServiceState) {
        self.state = state;
    }

    pub fn set_pid(&mut self, pid: u32) {
        self.pid = Some(pid);
    }

    pub fn set_start_time(&mut self, time: u64) {
        self.start_time = Some(time);
    }

    pub fn increment_restart_count(&mut self) {
        self.restart_count += 1;
    }

    pub fn get_uptime(&self) -> Option<u64> {
        if let Some(_start_time) = self.start_time {
            Some(0) // Would calculate actual uptime
        } else {
            None
        }
    }
}

/// Service manager
#[derive(Debug, Clone)]
pub struct SystemServiceManager {
    pub services: Vec<SystemService>,
}

impl Default for SystemServiceManager {
    fn default() -> Self {
        SystemServiceManager {
            services: Vec::new(),
        }
    }
}

impl SystemServiceManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add service
    pub fn add_service(&mut self, config: SystemServiceConfig) -> Result<(), String> {
        if self.services.iter().any(|s| s.config.name == config.name) {
            return Err(format!("Service {} already exists", config.name));
        }

        let service = SystemService::new(config);
        self.services.push(service);
        Ok(())
    }

    /// Remove service
    pub fn remove_service(&mut self, name: &str) -> Result<(), String> {
        if let Some(pos) = self.services.iter().position(|s| s.config.name == name) {
            let service = &self.services[pos];
            if service.state == SystemServiceState::Running {
                return Err(format!("Service {} is running, stop it first", name));
            }
            self.services.remove(pos);
            Ok(())
        } else {
            Err(format!("Service {} not found", name))
        }
    }

    /// Get service
    pub fn get_service(&self, name: &str) -> Option<&SystemService> {
        self.services.iter().find(|s| s.config.name == name)
    }

    /// Get service mutably
    pub fn get_service_mut(&mut self, name: &str) -> Option<&mut SystemService> {
        self.services.iter_mut().find(|s| s.config.name == name)
    }

    /// Start service
    pub fn start_service(&mut self, name: &str) -> Result<(), String> {
        // Check dependencies first (immutable borrow)
        let dependencies: Vec<String> = if let Some(service) = self.get_service(name) {
            service.config.depends_on.clone()
        } else {
            return Err(format!("Service {} not found", name));
        };

        for dependency in &dependencies {
            if let Some(dep_service) = self.get_service(dependency) {
                if dep_service.state != SystemServiceState::Running {
                    return Err(format!("Dependency {} is not running", dependency));
                }
            }
        }

        // Now get mutable borrow
        let service = self
            .get_service_mut(name)
            .ok_or_else(|| format!("Service {} not found", name))?;

        if service.state == SystemServiceState::Running {
            return Err(format!("Service {} is already running", name));
        }

        service.set_state(SystemServiceState::Starting);
        // Would actually start the process here
        service.set_state(SystemServiceState::Running);
        service.set_pid(1234); // Would be actual PID
        service.set_start_time(0); // Would be actual timestamp

        Ok(())
    }

    /// Stop service
    pub fn stop_service(&mut self, name: &str) -> Result<(), String> {
        let service = self
            .get_service_mut(name)
            .ok_or_else(|| format!("Service {} not found", name))?;

        if service.state != SystemServiceState::Running {
            return Err(format!("Service {} is not running", name));
        }

        service.set_state(SystemServiceState::Stopping);
        // Would actually stop the process here
        service.set_state(SystemServiceState::Stopped);
        service.pid = None;
        service.start_time = None;

        Ok(())
    }

    /// Restart service
    pub fn restart_service(&mut self, name: &str) -> Result<(), String> {
        self.stop_service(name)?;
        self.start_service(name)?;

        if let Some(service) = self.get_service_mut(name) {
            service.increment_restart_count();
        }

        Ok(())
    }

    /// Enable service (auto-start)
    pub fn enable_service(&mut self, name: &str) -> Result<(), String> {
        let service = self
            .get_service_mut(name)
            .ok_or_else(|| format!("Service {} not found", name))?;

        service.config.set_auto_start(true);
        Ok(())
    }

    /// Disable service (no auto-start)
    pub fn disable_service(&mut self, name: &str) -> Result<(), String> {
        let service = self
            .get_service_mut(name)
            .ok_or_else(|| format!("Service {} not found", name))?;

        service.config.set_auto_start(false);
        Ok(())
    }

    /// Get service status
    pub fn get_service_status(&self, name: &str) -> Result<String, String> {
        let service = self
            .get_service(name)
            .ok_or_else(|| format!("Service {} not found", name))?;

        let mut status = format!("Service: {}\n", service.config.name);
        status.push_str(&format!("Description: {}\n", service.config.description));
        status.push_str(&format!("State: {}\n", service.state.as_str()));
        status.push_str(&format!("Type: {}\n", service.config.service_type.as_str()));
        status.push_str(&format!(
            "PID: {}\n",
            service
                .pid
                .map(|p| p.to_string())
                .unwrap_or(String::from("N/A"))
        ));
        status.push_str(&format!(
            "Auto-start: {}\n",
            if service.config.auto_start {
                "enabled"
            } else {
                "disabled"
            }
        ));
        status.push_str(&format!(
            "Restart policy: {}\n",
            service.config.restart_policy.as_str()
        ));
        status.push_str(&format!("Restart count: {}\n", service.restart_count));

        Ok(status)
    }

    /// List all services
    pub fn list_services(&self) -> Vec<String> {
        self.services
            .iter()
            .map(|s| {
                format!(
                    "{} ({}, {})",
                    s.config.name,
                    s.state.as_str(),
                    if s.config.auto_start {
                        "enabled"
                    } else {
                        "disabled"
                    }
                )
            })
            .collect()
    }

    /// List running services
    pub fn list_running_services(&self) -> Vec<String> {
        self.services
            .iter()
            .filter(|s| s.state == SystemServiceState::Running)
            .map(|s| {
                format!(
                    "{} (PID: {})",
                    s.config.name,
                    s.pid.map(|p| p.to_string()).unwrap_or(String::from("N/A"))
                )
            })
            .collect()
    }

    /// List enabled services
    pub fn list_enabled_services(&self) -> Vec<String> {
        self.services
            .iter()
            .filter(|s| s.config.auto_start)
            .map(|s| format!("{} ({})", s.config.name, s.state.as_str()))
            .collect()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> String {
        let mut stats = String::from("Service Manager Statistics:\n");

        let total_services = self.services.len();
        let running_services = self
            .services
            .iter()
            .filter(|s| s.state == SystemServiceState::Running)
            .count();
        let stopped_services = self
            .services
            .iter()
            .filter(|s| s.state == SystemServiceState::Stopped)
            .count();
        let failed_services = self
            .services
            .iter()
            .filter(|s| s.state == SystemServiceState::Failed)
            .count();
        let enabled_services = self.services.iter().filter(|s| s.config.auto_start).count();

        stats.push_str(&format!("Total services: {}\n", total_services));
        stats.push_str(&format!("Running: {}\n", running_services));
        stats.push_str(&format!("Stopped: {}\n", stopped_services));
        stats.push_str(&format!("Failed: {}\n", failed_services));
        stats.push_str(&format!("Enabled (auto-start): {}\n", enabled_services));

        stats
    }

    /// Add SSH service
    pub fn add_ssh_service(&mut self) -> Result<(), String> {
        let mut config = SystemServiceConfig::new(
            String::from("sshd"),
            String::from("OpenSSH server daemon"),
            String::from("/usr/sbin/sshd -D"),
        );
        config.set_exec_stop(String::from("/usr/bin/pkill sshd"));
        config.set_auto_start(false);
        self.add_service(config)
    }

    /// Add firewall service
    pub fn add_firewall_service(&mut self) -> Result<(), String> {
        let mut config = SystemServiceConfig::new(
            String::from("firewall"),
            String::from("Network firewall service"),
            String::from("/usr/sbin/sigma-firewall start"),
        );
        config.set_exec_stop(String::from("/usr/sbin/sigma-firewall stop"));
        config.set_auto_start(true);
        self.add_service(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_state_as_str() {
        assert_eq!(SystemServiceState::Running.as_str(), "running");
        assert_eq!(SystemServiceState::Stopped.as_str(), "stopped");
    }

    #[test]
    fn test_service_state_from_str() {
        assert_eq!(
            SystemServiceState::from_str("running"),
            Some(SystemServiceState::Running)
        );
        assert_eq!(SystemServiceState::from_str("invalid"), None);
    }

    #[test]
    fn test_service_config_creation() {
        let config = SystemServiceConfig::new(
            String::from("test"),
            String::from("Test service"),
            String::from("/usr/bin/test"),
        );
        assert_eq!(config.name, "test");
        assert_eq!(config.service_type, SystemServiceType::System);
    }

    #[test]
    fn test_service_config_set_auto_start() {
        let mut config = SystemServiceConfig::new(
            String::from("test"),
            String::from("Test service"),
            String::from("/usr/bin/test"),
        );
        config.set_auto_start(true);
        assert!(config.auto_start);
    }

    #[test]
    fn test_service_creation() {
        let config = SystemServiceConfig::new(
            String::from("test"),
            String::from("Test service"),
            String::from("/usr/bin/test"),
        );
        let service = SystemService::new(config);
        assert_eq!(service.state, SystemServiceState::Stopped);
    }

    #[test]
    fn test_service_set_state() {
        let config = SystemServiceConfig::new(
            String::from("test"),
            String::from("Test service"),
            String::from("/usr/bin/test"),
        );
        let mut service = SystemService::new(config);
        service.set_state(SystemServiceState::Running);
        assert_eq!(service.state, SystemServiceState::Running);
    }

    #[test]
    fn test_service_manager_creation() {
        let manager = SystemServiceManager::new();
        assert_eq!(manager.services.len(), 0);
    }

    #[test]
    fn test_service_manager_add_service() {
        let mut manager = SystemServiceManager::new();
        let config = SystemServiceConfig::new(
            String::from("test"),
            String::from("Test service"),
            String::from("/usr/bin/test"),
        );
        assert!(manager.add_service(config).is_ok());
        assert_eq!(manager.services.len(), 1);
    }

    #[test]
    fn test_service_manager_add_duplicate_service() {
        let mut manager = SystemServiceManager::new();
        let config = SystemServiceConfig::new(
            String::from("test"),
            String::from("Test service"),
            String::from("/usr/bin/test"),
        );
        manager.add_service(config.clone()).unwrap();
        assert!(manager.add_service(config).is_err());
    }

    #[test]
    fn test_service_manager_start_service() {
        let mut manager = SystemServiceManager::new();
        let config = SystemServiceConfig::new(
            String::from("test"),
            String::from("Test service"),
            String::from("/usr/bin/test"),
        );
        manager.add_service(config).unwrap();
        assert!(manager.start_service("test").is_ok());
    }

    #[test]
    fn test_service_manager_stop_service() {
        let mut manager = SystemServiceManager::new();
        let config = SystemServiceConfig::new(
            String::from("test"),
            String::from("Test service"),
            String::from("/usr/bin/test"),
        );
        manager.add_service(config).unwrap();
        manager.start_service("test").unwrap();
        assert!(manager.stop_service("test").is_ok());
    }

    #[test]
    fn test_service_manager_restart_service() {
        let mut manager = SystemServiceManager::new();
        let config = SystemServiceConfig::new(
            String::from("test"),
            String::from("Test service"),
            String::from("/usr/bin/test"),
        );
        manager.add_service(config).unwrap();
        manager.start_service("test").unwrap();
        assert!(manager.restart_service("test").is_ok());
    }

    #[test]
    fn test_service_manager_enable_service() {
        let mut manager = SystemServiceManager::new();
        let config = SystemServiceConfig::new(
            String::from("test"),
            String::from("Test service"),
            String::from("/usr/bin/test"),
        );
        manager.add_service(config).unwrap();
        assert!(manager.enable_service("test").is_ok());
    }

    #[test]
    fn test_service_manager_get_service_status() {
        let mut manager = SystemServiceManager::new();
        let config = SystemServiceConfig::new(
            String::from("test"),
            String::from("Test service"),
            String::from("/usr/bin/test"),
        );
        manager.add_service(config).unwrap();
        let status = manager.get_service_status("test");
        assert!(status.is_ok());
        assert!(status.unwrap().contains("test"));
    }

    #[test]
    fn test_service_manager_list_services() {
        let mut manager = SystemServiceManager::new();
        let config = SystemServiceConfig::new(
            String::from("test"),
            String::from("Test service"),
            String::from("/usr/bin/test"),
        );
        manager.add_service(config).unwrap();
        let services = manager.list_services();
        assert_eq!(services.len(), 1);
    }

    #[test]
    fn test_service_manager_list_running_services() {
        let mut manager = SystemServiceManager::new();
        let config = SystemServiceConfig::new(
            String::from("test"),
            String::from("Test service"),
            String::from("/usr/bin/test"),
        );
        manager.add_service(config).unwrap();
        manager.start_service("test").unwrap();
        let running = manager.list_running_services();
        assert_eq!(running.len(), 1);
    }

    #[test]
    fn test_service_manager_get_statistics() {
        let mut manager = SystemServiceManager::new();
        let config = SystemServiceConfig::new(
            String::from("test"),
            String::from("Test service"),
            String::from("/usr/bin/test"),
        );
        manager.add_service(config).unwrap();
        let stats = manager.get_statistics();
        assert!(stats.contains("Total services: 1"));
    }

    #[test]
    fn test_service_manager_add_ssh_service() {
        let mut manager = SystemServiceManager::new();
        assert!(manager.add_ssh_service().is_ok());
        assert_eq!(manager.services.len(), 1);
    }

    #[test]
    fn test_service_manager_add_firewall_service() {
        let mut manager = SystemServiceManager::new();
        assert!(manager.add_firewall_service().is_ok());
        assert_eq!(manager.services.len(), 1);
    }
}
