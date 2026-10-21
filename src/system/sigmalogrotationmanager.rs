#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Log rotation manager for automated log file rotation
/// Inspired by Linux logrotate and Omarchy log management
#[derive(Debug, Clone)]
pub struct LogRotationManager {
    policies: Vec<RotationPolicy>,
}

/// A log rotation policy
#[derive(Debug, Clone)]
pub struct RotationPolicy {
    id: String,
    log_path: String,
    rotation_interval: RotationInterval,
    max_size_bytes: u64,
    max_age_days: u32,
    max_count: u32,
    compress: bool,
    state: PolicyState,
}

/// Rotation interval
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationInterval {
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

/// Policy state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyState {
    Active,
    Disabled,
    Paused,
}

impl LogRotationManager {
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
        }
    }

    /// Create a rotation policy
    pub fn create_policy(&mut self, id: String, log_path: String, rotation_interval: RotationInterval, max_size_bytes: u64, max_age_days: u32, max_count: u32, compress: bool) -> Result<String, String> {
        if self.policies.iter().any(|p| p.id == id) {
            return Err(format!("Policy with id {} already exists", id));
        }
        
        let policy = RotationPolicy {
            id: id.clone(),
            log_path,
            rotation_interval,
            max_size_bytes,
            max_age_days,
            max_count,
            compress,
            state: PolicyState::Active,
        };
        self.policies.push(policy);
        Ok(id)
    }

    /// Remove a policy by ID
    pub fn remove_policy(&mut self, id: &str) -> Result<(), String> {
        let pos = self.policies.iter().position(|p| p.id == id)
            .ok_or_else(|| format!("Policy with id {} not found", id))?;
        self.policies.remove(pos);
        Ok(())
    }

    /// Enable a policy
    pub fn enable_policy(&mut self, id: &str) -> Result<(), String> {
        let policy = self.policies.iter_mut().find(|p| p.id == id)
            .ok_or_else(|| format!("Policy with id {} not found", id))?;
        policy.state = PolicyState::Active;
        Ok(())
    }

    /// Disable a policy
    pub fn disable_policy(&mut self, id: &str) -> Result<(), String> {
        let policy = self.policies.iter_mut().find(|p| p.id == id)
            .ok_or_else(|| format!("Policy with id {} not found", id))?;
        policy.state = PolicyState::Disabled;
        Ok(())
    }

    /// Pause a policy
    pub fn pause_policy(&mut self, id: &str) -> Result<(), String> {
        let policy = self.policies.iter_mut().find(|p| p.id == id)
            .ok_or_else(|| format!("Policy with id {} not found", id))?;
        policy.state = PolicyState::Paused;
        Ok(())
    }

    /// Update policy interval
    pub fn update_interval(&mut self, id: &str, interval: RotationInterval) -> Result<(), String> {
        let policy = self.policies.iter_mut().find(|p| p.id == id)
            .ok_or_else(|| format!("Policy with id {} not found", id))?;
        policy.rotation_interval = interval;
        Ok(())
    }

    /// Update max size
    pub fn update_max_size(&mut self, id: &str, max_size_bytes: u64) -> Result<(), String> {
        let policy = self.policies.iter_mut().find(|p| p.id == id)
            .ok_or_else(|| format!("Policy with id {} not found", id))?;
        policy.max_size_bytes = max_size_bytes;
        Ok(())
    }

    /// Get all policies
    pub fn get_policies(&self) -> &[RotationPolicy] {
        &self.policies
    }

    /// Get active policies
    pub fn get_active_policies(&self) -> Vec<&RotationPolicy> {
        self.policies.iter().filter(|p| p.state == PolicyState::Active).collect()
    }

    /// Get policy by ID
    pub fn get_policy(&self, id: &str) -> Option<&RotationPolicy> {
        self.policies.iter().find(|p| p.id == id)
    }

    /// Get policies by log path
    pub fn get_policies_by_path(&self, log_path: &str) -> Vec<&RotationPolicy> {
        self.policies.iter().filter(|p| p.log_path == log_path).collect()
    }
}

impl Default for LogRotationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_policy() {
        let mut manager = LogRotationManager::new();
        let result = manager.create_policy("policy1".to_string(), "/var/log/syslog".to_string(), RotationInterval::Daily, 100 * 1024 * 1024, 30, 7, true);
        assert!(result.is_ok());
        assert_eq!(manager.get_policies().len(), 1);
    }

    #[test]
    fn test_duplicate_policy() {
        let mut manager = LogRotationManager::new();
        manager.create_policy("policy1".to_string(), "/var/log/syslog".to_string(), RotationInterval::Daily, 100 * 1024 * 1024, 30, 7, true).unwrap();
        let result = manager.create_policy("policy1".to_string(), "/var/log/auth.log".to_string(), RotationInterval::Weekly, 50 * 1024 * 1024, 14, 4, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_enable_disable_policy() {
        let mut manager = LogRotationManager::new();
        manager.create_policy("policy1".to_string(), "/var/log/syslog".to_string(), RotationInterval::Daily, 100 * 1024 * 1024, 30, 7, true).unwrap();
        
        manager.disable_policy("policy1").unwrap();
        assert_eq!(manager.get_policy("policy1").unwrap().state, PolicyState::Disabled);
        
        manager.enable_policy("policy1").unwrap();
        assert_eq!(manager.get_policy("policy1").unwrap().state, PolicyState::Active);
    }

    #[test]
    fn test_pause_policy() {
        let mut manager = LogRotationManager::new();
        manager.create_policy("policy1".to_string(), "/var/log/syslog".to_string(), RotationInterval::Daily, 100 * 1024 * 1024, 30, 7, true).unwrap();
        
        manager.pause_policy("policy1").unwrap();
        assert_eq!(manager.get_policy("policy1").unwrap().state, PolicyState::Paused);
    }

    #[test]
    fn test_update_interval() {
        let mut manager = LogRotationManager::new();
        manager.create_policy("policy1".to_string(), "/var/log/syslog".to_string(), RotationInterval::Daily, 100 * 1024 * 1024, 30, 7, true).unwrap();
        
        manager.update_interval("policy1", RotationInterval::Weekly).unwrap();
        assert_eq!(manager.get_policy("policy1").unwrap().rotation_interval, RotationInterval::Weekly);
    }
}
