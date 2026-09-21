// BSD Jails (FreeBSD-inspired)
// Provides process isolation with restricted filesystem view

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};

/// BSD Jail configuration
#[derive(Debug, Clone)]
pub struct BsdJailConfig {
    pub name: String,
    pub path: String,
    pub hostname: String,
    pub ip4: Option<IpAddr>,
    pub ip6: Option<IpAddr>,
    pub securelevel: u32,
}

impl BsdJailConfig {
    pub fn new(name: String, path: String) -> Self {
        Self {
            name,
            path,
            hostname: "localhost".to_string(),
            ip4: None,
            ip6: None,
            securelevel: 0,
        }
    }

    pub fn with_hostname(mut self, hostname: String) -> Self {
        self.hostname = hostname;
        self
    }

    pub fn with_ip4(mut self, ip: IpAddr) -> Self {
        self.ip4 = Some(ip);
        self
    }

    pub fn with_ip6(mut self, ip: IpAddr) -> Self {
        self.ip6 = Some(ip);
        self
    }

    pub fn with_securelevel(mut self, level: u32) -> Self {
        self.securelevel = level;
        self
    }
}

/// BSD Jail for process isolation
#[derive(Debug, Clone)]
pub struct BsdJail {
    pub id: u64,
    pub config: BsdJailConfig,
    pub process_ids: Vec<u64>,
    pub active: bool,
}

impl BsdJail {
    pub fn new(id: u64, config: BsdJailConfig) -> Self {
        Self {
            id,
            config,
            process_ids: Vec::new(),
            active: false,
        }
    }

    /// Start the jail
    pub fn start(&mut self) -> Result<(), String> {
        if self.active {
            return Err("Jail already active".to_string());
        }
        self.active = true;
        Ok(())
    }

    /// Stop the jail
    pub fn stop(&mut self) -> Result<(), String> {
        if !self.active {
            return Err("Jail not active".to_string());
        }
        if !self.process_ids.is_empty() {
            return Err("Cannot stop jail with active processes".to_string());
        }
        self.active = false;
        Ok(())
    }

    /// Add a process to the jail
    pub fn add_process(&mut self, pid: u64) -> Result<(), String> {
        if !self.active {
            return Err("Jail not active".to_string());
        }
        self.process_ids.push(pid);
        Ok(())
    }

    /// Remove a process from the jail
    pub fn remove_process(&mut self, pid: u64) -> Result<(), String> {
        if let Some(pos) = self.process_ids.iter().position(|&p| p == pid) {
            self.process_ids.remove(pos);
            Ok(())
        } else {
            Err(format!("Process {} not found in jail", pid))
        }
    }

    /// Check if a process is in the jail
    pub fn contains_process(&self, pid: u64) -> bool {
        self.process_ids.contains(&pid)
    }

    /// Get process count
    pub fn process_count(&self) -> usize {
        self.process_ids.len()
    }

    /// Check if jail is active
    pub fn is_active(&self) -> bool {
        self.active
    }
}

/// BSD Jail manager for system-wide jail management
pub struct BsdJailManager {
    jails: Arc<Mutex<HashMap<u64, BsdJail>>>,
    next_jail_id: Arc<Mutex<u64>>,
}

impl BsdJailManager {
    pub fn new() -> Self {
        Self {
            jails: Arc::new(Mutex::new(HashMap::new())),
            next_jail_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new jail
    pub fn create_jail(&self, config: BsdJailConfig) -> u64 {
        let mut next_id = self.next_jail_id.lock().unwrap();
        let jail_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let jail = BsdJail::new(jail_id, config);
        let mut jails = self.jails.lock().unwrap();
        jails.insert(jail_id, jail);

        jail_id
    }

    /// Get a jail by ID
    pub fn get_jail(&self, jail_id: u64) -> Option<BsdJail> {
        let jails = self.jails.lock().unwrap();
        jails.get(&jail_id).cloned()
    }

    /// Remove a jail
    pub fn remove_jail(&self, jail_id: u64) -> Result<(), String> {
        let mut jails = self.jails.lock().unwrap();
        match jails.remove(&jail_id) {
            Some(jail) => {
                if jail.active {
                    return Err("Cannot remove active jail".to_string());
                }
                Ok(())
            }
            None => Err(format!("Jail {} not found", jail_id)),
        }
    }

    /// Start a jail
    pub fn start_jail(&self, jail_id: u64) -> Result<(), String> {
        let mut jails = self.jails.lock().unwrap();
        match jails.get_mut(&jail_id) {
            Some(jail) => jail.start(),
            None => Err(format!("Jail {} not found", jail_id)),
        }
    }

    /// Stop a jail
    pub fn stop_jail(&self, jail_id: u64) -> Result<(), String> {
        let mut jails = self.jails.lock().unwrap();
        match jails.get_mut(&jail_id) {
            Some(jail) => jail.stop(),
            None => Err(format!("Jail {} not found", jail_id)),
        }
    }

    /// Add process to a jail
    pub fn add_process(&self, jail_id: u64, pid: u64) -> Result<(), String> {
        let mut jails = self.jails.lock().unwrap();
        match jails.get_mut(&jail_id) {
            Some(jail) => jail.add_process(pid),
            None => Err(format!("Jail {} not found", jail_id)),
        }
    }

    /// Remove process from a jail
    pub fn remove_process(&self, jail_id: u64, pid: u64) -> Result<(), String> {
        let mut jails = self.jails.lock().unwrap();
        match jails.get_mut(&jail_id) {
            Some(jail) => jail.remove_process(pid),
            None => Err(format!("Jail {} not found", jail_id)),
        }
    }

    /// Find which jail a process belongs to
    pub fn find_jail_for_process(&self, pid: u64) -> Option<u64> {
        let jails = self.jails.lock().unwrap();
        for (jail_id, jail) in jails.iter() {
            if jail.contains_process(pid) {
                return Some(*jail_id);
            }
        }
        None
    }

    /// Get number of active jails
    pub fn active_jail_count(&self) -> usize {
        let jails = self.jails.lock().unwrap();
        jails.values().filter(|j| j.is_active()).count()
    }

    /// Get total jail count
    pub fn jail_count(&self) -> usize {
        let jails = self.jails.lock().unwrap();
        jails.len()
    }
}

impl Default for BsdJailManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_bsd_jail_config_creation() {
        let config = BsdJailConfig::new("test_jail".to_string(), "/srv/jail/test".to_string());
        assert_eq!(config.name, "test_jail");
        assert_eq!(config.path, "/srv/jail/test");
        assert_eq!(config.hostname, "localhost");
    }

    #[test]
    fn test_bsd_jail_config_builder() {
        let config = BsdJailConfig::new("test_jail".to_string(), "/srv/jail/test".to_string())
            .with_hostname("jail.example.com".to_string())
            .with_ip4(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)))
            .with_securelevel(2);

        assert_eq!(config.hostname, "jail.example.com");
        assert_eq!(config.ip4, Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100))));
        assert_eq!(config.securelevel, 2);
    }

    #[test]
    fn test_bsd_jail_lifecycle() {
        let config = BsdJailConfig::new("test_jail".to_string(), "/srv/jail/test".to_string());
        let mut jail = BsdJail::new(1, config);

        assert!(!jail.is_active());
        jail.start().unwrap();
        assert!(jail.is_active());

        jail.add_process(100).unwrap();
        assert_eq!(jail.process_count(), 1);
        assert!(jail.contains_process(100));

        jail.remove_process(100).unwrap();
        assert_eq!(jail.process_count(), 0);

        jail.stop().unwrap();
        assert!(!jail.is_active());
    }

    #[test]
    fn test_bsd_jail_double_start() {
        let config = BsdJailConfig::new("test_jail".to_string(), "/srv/jail/test".to_string());
        let mut jail = BsdJail::new(1, config);

        jail.start().unwrap();
        assert!(jail.start().is_err());
    }

    #[test]
    fn test_bsd_jail_stop_with_processes() {
        let config = BsdJailConfig::new("test_jail".to_string(), "/srv/jail/test".to_string());
        let mut jail = BsdJail::new(1, config);

        jail.start().unwrap();
        jail.add_process(100).unwrap();
        assert!(jail.stop().is_err());
    }

    #[test]
    fn test_bsd_jail_manager() {
        let manager = BsdJailManager::new();

        let config = BsdJailConfig::new("test_jail".to_string(), "/srv/jail/test".to_string());
        let jail_id = manager.create_jail(config);
        assert_eq!(jail_id, 1);

        manager.start_jail(jail_id).unwrap();
        manager.add_process(jail_id, 100).unwrap();

        assert_eq!(manager.active_jail_count(), 1);
        assert_eq!(manager.find_jail_for_process(100), Some(jail_id));

        manager.remove_process(jail_id, 100).unwrap();
        manager.stop_jail(jail_id).unwrap();
        manager.remove_jail(jail_id).unwrap();

        assert_eq!(manager.jail_count(), 0);
    }

    #[test]
    fn test_bsd_jail_manager_multiple_jails() {
        let manager = BsdJailManager::new();

        let config1 = BsdJailConfig::new("jail1".to_string(), "/srv/jail1".to_string());
        let config2 = BsdJailConfig::new("jail2".to_string(), "/srv/jail2".to_string());

        let jail_id1 = manager.create_jail(config1);
        let jail_id2 = manager.create_jail(config2);

        manager.start_jail(jail_id1).unwrap();
        manager.start_jail(jail_id2).unwrap();

        manager.add_process(jail_id1, 100).unwrap();
        manager.add_process(jail_id2, 200).unwrap();

        assert_eq!(manager.find_jail_for_process(100), Some(jail_id1));
        assert_eq!(manager.find_jail_for_process(200), Some(jail_id2));
        assert_eq!(manager.find_jail_for_process(300), None);

        assert_eq!(manager.active_jail_count(), 2);
    }

    #[test]
    fn test_bsd_jail_remove_active() {
        let manager = BsdJailManager::new();

        let config = BsdJailConfig::new("test_jail".to_string(), "/srv/jail/test".to_string());
        let jail_id = manager.create_jail(config);

        manager.start_jail(jail_id).unwrap();
        assert!(manager.remove_jail(jail_id).is_err());

        manager.stop_jail(jail_id).unwrap();
        assert!(manager.remove_jail(jail_id).is_ok());
    }
}
