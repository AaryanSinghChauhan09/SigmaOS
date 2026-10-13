//! SigmaQuota - Filesystem Quota Management
//!
//! Inspired by Linux quota, this module provides comprehensive filesystem quota
//! management including user quotas, group quotas, and space usage tracking.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Quota type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuotaType {
    User,
    Group,
    Unknown,
}

/// Quota state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuotaState {
    Active,
    Inactive,
    Exceeded,
    Warning,
    Unknown,
}

/// Quota data structure
#[derive(Debug, Clone)]
pub struct Quota {
    pub quota_id: String,
    pub quota_type: QuotaType,
    pub identifier: String,
    pub filesystem: String,
    pub soft_limit: u64,
    pub hard_limit: u64,
    pub used_space: u64,
    pub state: QuotaState,
}

impl Quota {
    /// Create a new quota
    pub fn new(
        quota_id: String,
        quota_type: QuotaType,
        identifier: String,
        filesystem: String,
        soft_limit: u64,
        hard_limit: u64,
    ) -> Self {
        Self {
            quota_id,
            quota_type,
            identifier,
            filesystem,
            soft_limit,
            hard_limit,
            used_space: 0,
            state: QuotaState::Inactive,
        }
    }

    /// Set quota state
    pub fn set_state(&mut self, state: QuotaState) {
        self.state = state;
    }

    /// Set used space
    pub fn set_used_space(&mut self, used_space: u64) {
        self.used_space = used_space;
        self.update_state();
    }

    /// Update quota state based on usage
    pub fn update_state(&mut self) {
        if self.used_space >= self.hard_limit {
            self.state = QuotaState::Exceeded;
        } else if self.used_space >= self.soft_limit {
            self.state = QuotaState::Warning;
        } else {
            self.state = QuotaState::Active;
        }
    }

    /// Get remaining space
    pub fn get_remaining(&self) -> u64 {
        if self.hard_limit > self.used_space {
            self.hard_limit - self.used_space
        } else {
            0
        }
    }

    /// Check if quota is exceeded
    pub fn is_exceeded(&self) -> bool {
        self.state == QuotaState::Exceeded
    }

    /// Check if quota is in warning state
    pub fn is_warning(&self) -> bool {
        self.state == QuotaState::Warning
    }

    /// Check if quota is active
    pub fn is_active(&self) -> bool {
        self.state == QuotaState::Active
    }
}

/// Quota manager
pub struct SigmaQuota {
    quotas: BTreeMap<String, Quota>,
    running: AtomicBool,
}

impl SigmaQuota {
    /// Create a new quota manager
    pub fn new() -> Self {
        Self {
            quotas: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the quota manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the quota manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a quota
    pub fn add_quota(&mut self, quota: Quota) {
        self.quotas.insert(quota.quota_id.clone(), quota);
    }

    /// Remove a quota
    pub fn remove_quota(&mut self, quota_id: &str) -> Option<Quota> {
        self.quotas.remove(quota_id)
    }

    /// Get a quota by ID
    pub fn get_quota(&self, quota_id: &str) -> Option<&Quota> {
        self.quotas.get(quota_id)
    }

    /// List all quotas
    pub fn list_quotas(&self) -> Vec<&Quota> {
        self.quotas.values().collect()
    }

    /// List quotas by type
    pub fn list_quotas_by_type(&self, quota_type: QuotaType) -> Vec<&Quota> {
        self.quotas
            .values()
            .filter(|q| q.quota_type == quota_type)
            .collect()
    }

    /// List quotas by filesystem
    pub fn list_quotas_by_filesystem(&self, filesystem: &str) -> Vec<&Quota> {
        self.quotas
            .values()
            .filter(|q| q.filesystem == filesystem)
            .collect()
    }

    /// List exceeded quotas
    pub fn list_exceeded(&self) -> Vec<&Quota> {
        self.quotas.values().filter(|q| q.is_exceeded()).collect()
    }

    /// List warning quotas
    pub fn list_warnings(&self) -> Vec<&Quota> {
        self.quotas.values().filter(|q| q.is_warning()).collect()
    }

    /// Update quota usage
    pub fn update_usage(&mut self, quota_id: &str, used_space: u64) -> bool {
        if let Some(quota) = self.quotas.get_mut(quota_id) {
            quota.set_used_space(used_space);
            true
        } else {
            false
        }
    }

    /// Set quota limits
    pub fn set_limits(&mut self, quota_id: &str, soft_limit: u64, hard_limit: u64) -> bool {
        if let Some(quota) = self.quotas.get_mut(quota_id) {
            quota.soft_limit = soft_limit;
            quota.hard_limit = hard_limit;
            quota.update_state();
            true
        } else {
            false
        }
    }

    /// Get quota count
    pub fn quota_count(&self) -> usize {
        self.quotas.len()
    }
}

impl Default for SigmaQuota {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaQuota instance (placeholder for actual global state)
static unsafe mutable SIGMAQUOTA_INSTANCE: Option<SigmaQuota> = None;

/// Initialize the global SigmaQuota instance
pub fn init_sigmaquota() {
    unsafe {
        SIGMAQUOTA_INSTANCE = Some(SigmaQuota::new());
    }
}

/// Get the global SigmaQuota instance
pub fn get_sigmaquota() -> Option<&'static SigmaQuota> {
    unsafe {
        SIGMAQUOTA_INSTANCE.as_ref()
    }
}

/// Get mutable global SigmaQuota instance
pub fn get_sigmaquota_mut() -> Option<&'static mut SigmaQuota> {
    unsafe {
        SIGMAQUOTA_INSTANCE.as_mut()
    }
}

/// Convenience function to add a quota
pub fn add_quota(quota: Quota) {
    if let Some(quota_mgr) = get_sigmaquota_mut() {
        quota_mgr.add_quota(quota);
    }
}

/// Convenience function to update quota usage
pub fn update_quota_usage(quota_id: &str, used_space: u64) -> bool {
    if let Some(quota_mgr) = get_sigmaquota_mut() {
        quota_mgr.update_usage(quota_id, used_space)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quota_creation() {
        let quota = Quota::new(
            "quota1".to_string(),
            QuotaType::User,
            "user1".to_string(),
            "/home".to_string(),
            1024 * 1024 * 1024,
            2 * 1024 * 1024 * 1024,
        );

        assert_eq!(quota.quota_id, "quota1");
        assert_eq!(quota.quota_type, QuotaType::User);
        assert_eq!(quota.identifier, "user1");
        assert_eq!(quota.soft_limit, 1024 * 1024 * 1024);
        assert_eq!(quota.hard_limit, 2 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_quota_state_update() {
        let mut quota = Quota::new(
            "quota1".to_string(),
            QuotaType::User,
            "user1".to_string(),
            "/home".to_string(),
            1024 * 1024 * 1024,
            2 * 1024 * 1024 * 1024,
        );

        quota.set_used_space(512 * 1024 * 1024);
        assert!(quota.is_active());

        quota.set_used_space(1024 * 1024 * 1024);
        assert!(quota.is_warning());

        quota.set_used_space(2 * 1024 * 1024 * 1024);
        assert!(quota.is_exceeded());
    }

    #[test]
    fn test_quota_remaining() {
        let mut quota = Quota::new(
            "quota1".to_string(),
            QuotaType::User,
            "user1".to_string(),
            "/home".to_string(),
            1024 * 1024 * 1024,
            2 * 1024 * 1024 * 1024,
        );

        quota.set_used_space(512 * 1024 * 1024);
        let remaining = quota.get_remaining();
        assert_eq!(remaining, 1536 * 1024 * 1024);
    }

    #[test]
    fn test_sigmaquota_creation() {
        let quota_mgr = SigmaQuota::new();
        assert!(!quota_mgr.is_running());
        assert_eq!(quota_mgr.quota_count(), 0);
    }

    #[test]
    fn test_sigmaquota_start_stop() {
        let mut quota_mgr = SigmaQuota::new();
        quota_mgr.start();
        assert!(quota_mgr.is_running());
        quota_mgr.stop();
        assert!(!quota_mgr.is_running());
    }

    #[test]
    fn test_sigmaquota_add_quota() {
        let mut quota_mgr = SigmaQuota::new();
        let quota = Quota::new(
            "quota1".to_string(),
            QuotaType::User,
            "user1".to_string(),
            "/home".to_string(),
            1024 * 1024 * 1024,
            2 * 1024 * 1024 * 1024,
        );

        quota_mgr.add_quota(quota);
        assert_eq!(quota_mgr.quota_count(), 1);
        assert!(quota_mgr.get_quota("quota1").is_some());
    }

    #[test]
    fn test_sigmaquota_remove_quota() {
        let mut quota_mgr = SigmaQuota::new();
        let quota = Quota::new(
            "quota1".to_string(),
            QuotaType::User,
            "user1".to_string(),
            "/home".to_string(),
            1024 * 1024 * 1024,
            2 * 1024 * 1024 * 1024,
        );

        quota_mgr.add_quota(quota);
        assert_eq!(quota_mgr.quota_count(), 1);

        let removed = quota_mgr.remove_quota("quota1");
        assert!(removed.is_some());
        assert_eq!(quota_mgr.quota_count(), 0);
    }

    #[test]
    fn test_sigmaquota_update_usage() {
        let mut quota_mgr = SigmaQuota::new();
        let quota = Quota::new(
            "quota1".to_string(),
            QuotaType::User,
            "user1".to_string(),
            "/home".to_string(),
            1024 * 1024 * 1024,
            2 * 1024 * 1024 * 1024,
        );

        quota_mgr.add_quota(quota);
        let updated = quota_mgr.update_usage("quota1", 512 * 1024 * 1024);
        assert!(updated);

        let quota = quota_mgr.get_quota("quota1").unwrap();
        assert_eq!(quota.used_space, 512 * 1024 * 1024);
    }

    #[test]
    fn test_sigmaquota_set_limits() {
        let mut quota_mgr = SigmaQuota::new();
        let quota = Quota::new(
            "quota1".to_string(),
            QuotaType::User,
            "user1".to_string(),
            "/home".to_string(),
            1024 * 1024 * 1024,
            2 * 1024 * 1024 * 1024,
        );

        quota_mgr.add_quota(quota);
        let set = quota_mgr.set_limits("quota1", 2048 * 1024 * 1024, 4 * 1024 * 1024 * 1024);
        assert!(set);

        let quota = quota_mgr.get_quota("quota1").unwrap();
        assert_eq!(quota.soft_limit, 2048 * 1024 * 1024);
        assert_eq!(quota.hard_limit, 4 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_sigmaquota_list_by_type() {
        let mut quota_mgr = SigmaQuota::new();

        quota_mgr.add_quota(Quota::new(
            "quota1".to_string(),
            QuotaType::User,
            "user1".to_string(),
            "/home".to_string(),
            1024 * 1024 * 1024,
            2 * 1024 * 1024 * 1024,
        ));

        quota_mgr.add_quota(Quota::new(
            "quota2".to_string(),
            QuotaType::User,
            "user2".to_string(),
            "/home".to_string(),
            1024 * 1024 * 1024,
            2 * 1024 * 1024 * 1024,
        ));

        quota_mgr.add_quota(Quota::new(
            "quota3".to_string(),
            QuotaType::Group,
            "group1".to_string(),
            "/home".to_string(),
            1024 * 1024 * 1024,
            2 * 1024 * 1024 * 1024,
        ));

        let user_quotas = quota_mgr.list_quotas_by_type(QuotaType::User);
        assert_eq!(user_quotas.len(), 2);

        let group_quotas = quota_mgr.list_quotas_by_type(QuotaType::Group);
        assert_eq!(group_quotas.len(), 1);
    }

    #[test]
    fn test_sigmaquota_list_exceeded() {
        let mut quota_mgr = SigmaQuota::new();

        let mut quota1 = Quota::new(
            "quota1".to_string(),
            QuotaType::User,
            "user1".to_string(),
            "/home".to_string(),
            1024 * 1024 * 1024,
            2 * 1024 * 1024 * 1024,
        );
        quota1.set_used_space(2 * 1024 * 1024 * 1024);
        quota_mgr.add_quota(quota1);

        let quota2 = Quota::new(
            "quota2".to_string(),
            QuotaType::User,
            "user2".to_string(),
            "/home".to_string(),
            1024 * 1024 * 1024,
            2 * 1024 * 1024 * 1024,
        );
        quota_mgr.add_quota(quota2);

        let exceeded = quota_mgr.list_exceeded();
        assert_eq!(exceeded.len(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmaquota();

        let quota_mgr = get_sigmaquota();
        assert!(quota_mgr.is_some());

        let mut quota_mgr_mut = get_sigmaquota_mut();
        assert!(quota_mgr_mut.is_some());

        if let Some(quota_mgr) = quota_mgr_mut {
            quota_mgr.start();
            assert!(quota_mgr.is_running());
        }
    }

    #[test]
    fn test_convenience_add_quota() {
        init_sigmaquota();

        let quota = Quota::new(
            "quota1".to_string(),
            QuotaType::User,
            "user1".to_string(),
            "/home".to_string(),
            1024 * 1024 * 1024,
            2 * 1024 * 1024 * 1024,
        );

        add_quota(quota);

        let quota_mgr = get_sigmaquota().unwrap();
        assert_eq!(quota_mgr.quota_count(), 1);
    }

    #[test]
    fn test_convenience_update_usage() {
        init_sigmaquota();

        let quota = Quota::new(
            "quota1".to_string(),
            QuotaType::User,
            "user1".to_string(),
            "/home".to_string(),
            1024 * 1024 * 1024,
            2 * 1024 * 1024 * 1024,
        );

        add_quota(quota);

        let updated = update_quota_usage("quota1", 512 * 1024 * 1024);
        assert!(updated);

        let quota_mgr = get_sigmaquota().unwrap();
        let quota = quota_mgr.get_quota("quota1").unwrap();
        assert_eq!(quota.used_space, 512 * 1024 * 1024);
    }
}
