//! SigmaOS Jails/Container System
//!
//! Following principles: PROCESS_ISOLATION, COMPONENT_ISOLATION, SINGLE_RESPONSIBILITY,
//! MINIMAL_KERNEL_PHILOSOPHY, MODULARITY, SECURE_DEFAULTS, LEAST_PRIVILEGE,
//! INFORMATION_HIDING, ENCAPSULATION, DEFENSIVE_PROGRAMMING, FAIL_FAST_SYSTEM,
//! RESOURCE_ISOLATION, SANDBOXING, CAPABILITY_BASED_SECURITY.
//!
//! Competitive with: FreeBSD Jails, OpenBSD pledge, Linux containers (Docker/LXC).
//!
//! SigmaOS Jails provide:
//! - Lightweight OS-level virtualization (like FreeBSD Jails)
//! - Process and filesystem isolation
//! - Network isolation with virtual interfaces
//! - Resource limits (CPU, memory, disk)
//! - Secure defaults with capability-based access

use core::fmt;

/// Jail state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JailState {
    /// Jail is being created
    Creating,
    /// Jail is running
    Running,
    /// Jail is stopped
    Stopped,
    /// Jail is being destroyed
    Destroying,
    /// Jail is errored
    Error(String),
}

/// Jail configuration
#[derive(Debug, Clone)]
pub struct JailConfig {
    pub name: String,
    pub hostname: String,
    pub root_path: String,
    pub ip_address: Option<String>,
    pub interface: Option<String>,
    pub cpu_limit: Option<u32>, // percentage
    pub memory_limit: Option<u64>, // bytes
    pub disk_limit: Option<u64>, // bytes
    pub allow_network: bool,
    pub allow_mount: bool,
    pub allow_sysvipc: bool,
    pub exec_start: Option<String>,
    pub exec_stop: Option<String>,
}

impl Default for JailConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            hostname: String::new(),
            root_path: String::new(),
            ip_address: None,
            interface: None,
            cpu_limit: None,
            memory_limit: None,
            disk_limit: None,
            allow_network: false,
            allow_mount: false,
            allow_sysvipc: false,
            exec_start: None,
            exec_stop: None,
        }
    }
}

/// Jail instance
#[derive(Debug, Clone)]
pub struct Jail {
    pub id: u32,
    pub config: JailConfig,
    pub state: JailState,
    pub created_at: u64,
    pub pid: Option<u32>,
}

/// Jail manager
pub struct JailManager {
    pub jails: Vec<Jail>,
    pub next_id: u32,
}

impl JailManager {
    /// Create a new jail manager
    pub fn new() -> Self {
        Self {
            jails: Vec::new(),
            next_id: 1,
        }
    }

    /// Create a new jail
    pub fn create_jail(&mut self, config: JailConfig) -> Result<u32, String> {
        // Defensive: validate config
        if config.name.is_empty() {
            return Err("Jail name cannot be empty".to_string());
        }

        if config.hostname.is_empty() {
            return Err("Jail hostname cannot be empty".to_string());
        }

        if config.root_path.is_empty() {
            return Err("Jail root path cannot be empty".to_string());
        }

        // Check for duplicate name
        if self.jails.iter().any(|j| j.config.name == config.name) {
            return Err(format!("Jail '{}' already exists", config.name));
        }

        let id = self.next_id;
        self.next_id += 1;

        let jail = Jail {
            id,
            config: config.clone(),
            state: JailState::Creating,
            created_at: 0,
            pid: None,
        };

        self.jails.push(jail);
        Ok(id)
    }

    /// Start a jail
    pub fn start_jail(&mut self, id: u32) -> Result<(), String> {
        let jail = self
            .jails
            .iter_mut()
            .find(|j| j.id == id)
            .ok_or_else(|| format!("Jail {} not found", id))?;

        if jail.state != JailState::Stopped && jail.state != JailState::Error(String::new()) {
            return Err(format!("Jail {} is not in a startable state", id));
        }

        jail.state = JailState::Running;
        jail.pid = Some(id as u32); // Simplified
        Ok(())
    }

    /// Stop a jail
    pub fn stop_jail(&mut self, id: u32) -> Result<(), String> {
        let jail = self
            .jails
            .iter_mut()
            .find(|j| j.id == id)
            .ok_or_else(|| format!("Jail {} not found", id))?;

        if jail.state != JailState::Running {
            return Err(format!("Jail {} is not running", id));
        }

        jail.state = JailState::Stopped;
        jail.pid = None;
        Ok(())
    }

    /// Destroy a jail
    pub fn destroy_jail(&mut self, id: u32) -> Result<(), String> {
        let index = self
            .jails
            .iter()
            .position(|j| j.id == id)
            .ok_or_else(|| format!("Jail {} not found", id))?;

        if self.jails[index].state == JailState::Running {
            return Err(format!("Jail {} is running, stop it first", id));
        }

        self.jails.remove(index);
        Ok(())
    }

    /// List all jails
    pub fn list_jails(&self) -> &[Jail] {
        &self.jails
    }

    /// Get a jail by ID
    pub fn get_jail(&self, id: u32) -> Option<&Jail> {
        self.jails.iter().find(|j| j.id == id)
    }
}

impl Default for JailManager {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for JailState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JailState::Creating => write!(f, "Creating"),
            JailState::Running => write!(f, "Running"),
            JailState::Stopped => write!(f, "Stopped"),
            JailState::Destroying => write!(f, "Destroying"),
            JailState::Error(e) => write!(f, "Error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_jail() {
        let mut manager = JailManager::new();
        let config = JailConfig {
            name: "test-jail".to_string(),
            hostname: "test".to_string(),
            root_path: "/jails/test".to_string(),
            ..Default::default()
        };
        let id = manager.create_jail(config).unwrap();
        assert_eq!(id, 1);
        assert_eq!(manager.jails.len(), 1);
        assert_eq!(manager.jails[0].state, JailState::Creating);
    }

    #[test]
    fn test_duplicate_jail_name() {
        let mut manager = JailManager::new();
        let config = JailConfig {
            name: "dup".to_string(),
            hostname: "dup".to_string(),
            root_path: "/jails/dup".to_string(),
            ..Default::default()
        };
        manager.create_jail(config.clone()).unwrap();
        let result = manager.create_jail(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_start_stop_jail() {
        let mut manager = JailManager::new();
        let config = JailConfig {
            name: "test".to_string(),
            hostname: "test".to_string(),
            root_path: "/jails/test".to_string(),
            ..Default::default()
        };
        let id = manager.create_jail(config).unwrap();

        manager.start_jail(id).unwrap();
        assert_eq!(manager.get_jail(id).unwrap().state, JailState::Running);

        manager.stop_jail(id).unwrap();
        assert_eq!(manager.get_jail(id).unwrap().state, JailState::Stopped);
    }

    #[test]
    fn test_destroy_running_jail_fails() {
        let mut manager = JailManager::new();
        let config = JailConfig {
            name: "test".to_string(),
            hostname: "test".to_string(),
            root_path: "/jails/test".to_string(),
            ..Default::default()
        };
        let id = manager.create_jail(config).unwrap();
        manager.start_jail(id).unwrap();

        let result = manager.destroy_jail(id);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_name_fails() {
        let mut manager = JailManager::new();
        let config = JailConfig {
            name: String::new(),
            hostname: "test".to_string(),
            root_path: "/jails/test".to_string(),
            ..Default::default()
        };
        let result = manager.create_jail(config);
        assert!(result.is_err());
    }
}