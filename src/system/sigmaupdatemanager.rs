//! SigmaUpdateManager - System Update Manager
//!
//! Inspired by Update Manager (Linux Mint), this module provides comprehensive
//! system update management including security updates, package updates, and
//! automatic update scheduling.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Update type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateType {
    Security,
    BugFix,
    Feature,
    Driver,
    Kernel,
    Unknown,
}

/// Update state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateState {
    Available,
    Downloading,
    Installing,
    Installed,
    Failed,
    Unknown,
}

/// Update data structure
#[derive(Debug, Clone)]
pub struct Update {
    pub update_id: String,
    pub name: String,
    pub version: String,
    pub update_type: UpdateType,
    pub state: UpdateState,
    pub size: u64,
    pub description: String,
    pub is_critical: bool,
}

impl Update {
    /// Create a new update
    pub fn new(
        update_id: String,
        name: String,
        version: String,
        update_type: UpdateType,
        description: String,
    ) -> Self {
        Self {
            update_id,
            name,
            version,
            update_type,
            state: UpdateState::Available,
            size: 0,
            description,
            is_critical: false,
        }
    }

    /// Set state
    pub fn set_state(&mut self, state: UpdateState) {
        self.state = state;
    }

    /// Set size
    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    /// Set critical flag
    pub fn set_critical(&mut self, critical: bool) {
        self.is_critical = critical;
    }

    /// Check if update is installed
    pub fn is_installed(&self) -> bool {
        self.state == UpdateState::Installed
    }

    /// Check if update is critical
    pub fn is_update_critical(&self) -> bool {
        self.is_critical
    }
}

/// Update schedule data structure
#[derive(Debug, Clone)]
pub struct UpdateSchedule {
    pub schedule_id: String,
    pub enabled: bool,
    pub auto_install: bool,
    pub check_interval_hours: u32,
}

impl UpdateSchedule {
    /// Create a new update schedule
    pub fn new(schedule_id: String, check_interval_hours: u32) -> Self {
        Self {
            schedule_id,
            enabled: true,
            auto_install: false,
            check_interval_hours,
        }
    }

    /// Enable schedule
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable schedule
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Set auto install
    pub fn set_auto_install(&mut self, auto_install: bool) {
        self.auto_install = auto_install;
    }

    /// Check if schedule is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Update manager
pub struct SigmaUpdateManager {
    updates: BTreeMap<String, Update>,
    schedules: BTreeMap<String, UpdateSchedule>,
    running: AtomicBool,
}

impl SigmaUpdateManager {
    /// Create a new update manager
    pub fn new() -> Self {
        Self {
            updates: BTreeMap::new(),
            schedules: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the update manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the update manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add an update
    pub fn add_update(&mut self, update: Update) -> bool {
        let update_id = update.update_id.clone();
        self.updates.insert(update_id, update);
        true
    }

    /// Remove an update
    pub fn remove_update(&mut self, update_id: &str) -> Option<Update> {
        self.updates.remove(update_id)
    }

    /// Get an update by ID
    pub fn get_update(&self, update_id: &str) -> Option<&Update> {
        self.updates.get(update_id)
    }

    /// List all updates
    pub fn list_updates(&self) -> Vec<&Update> {
        self.updates.values().collect()
    }

    /// List updates by type
    pub fn list_updates_by_type(&self, update_type: UpdateType) -> Vec<&Update> {
        self.updates
            .values()
            .filter(|u| u.update_type == update_type)
            .collect()
    }

    /// List available updates
    pub fn list_available_updates(&self) -> Vec<&Update> {
        self.updates
            .values()
            .filter(|u| u.state == UpdateState::Available)
            .collect()
    }

    /// List critical updates
    pub fn list_critical_updates(&self) -> Vec<&Update> {
        self.updates
            .values()
            .filter(|u| u.is_update_critical())
            .collect()
    }

    /// Install an update
    pub fn install_update(&mut self, update_id: &str) -> bool {
        if let Some(update) = self.updates.get_mut(update_id) {
            update.set_state(UpdateState::Downloading);
            update.set_state(UpdateState::Installing);
            update.set_state(UpdateState::Installed);
            true
        } else {
            false
        }
    }

    /// Install all available updates
    pub fn install_all_updates(&mut self) -> usize {
        let mut count = 0;
        let update_ids: Vec<String> = self
            .updates
            .values()
            .filter(|u| u.state == UpdateState::Available)
            .map(|u| u.update_id.clone())
            .collect();

        for id in update_ids {
            if self.install_update(&id) {
                count += 1;
            }
        }

        count
    }

    /// Check for updates
    pub fn check_for_updates(&self) -> usize {
        self.list_available_updates().len()
    }

    /// Add a schedule
    pub fn add_schedule(&mut self, schedule: UpdateSchedule) -> bool {
        let schedule_id = schedule.schedule_id.clone();
        self.schedules.insert(schedule_id, schedule);
        true
    }

    /// Remove a schedule
    pub fn remove_schedule(&mut self, schedule_id: &str) -> Option<UpdateSchedule> {
        self.schedules.remove(schedule_id)
    }

    /// Get a schedule by ID
    pub fn get_schedule(&self, schedule_id: &str) -> Option<&UpdateSchedule> {
        self.schedules.get(schedule_id)
    }

    /// List all schedules
    pub fn list_schedules(&self) -> Vec<&UpdateSchedule> {
        self.schedules.values().collect()
    }

    /// List enabled schedules
    pub fn list_enabled_schedules(&self) -> Vec<&UpdateSchedule> {
        self.schedules
            .values()
            .filter(|s| s.is_enabled())
            .collect()
    }

    /// Get update count
    pub fn update_count(&self) -> usize {
        self.updates.len()
    }

    /// Get schedule count
    pub fn schedule_count(&self) -> usize {
        self.schedules.len()
    }
}

impl Default for SigmaUpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaUpdateManager instance (placeholder for actual global state)
static unsafe mutable SIGMAUPDATEMANAGER_INSTANCE: Option<SigmaUpdateManager> = None;

/// Initialize the global SigmaUpdateManager instance
pub fn init_sigmaupdatemanager() {
    unsafe {
        SIGMAUPDATEMANAGER_INSTANCE = Some(SigmaUpdateManager::new());
    }
}

/// Get the global SigmaUpdateManager instance
pub fn get_sigmaupdatemanager() -> Option<&'static SigmaUpdateManager> {
    unsafe {
        SIGMAUPDATEMANAGER_INSTANCE.as_ref()
    }
}

/// Get mutable global SigmaUpdateManager instance
pub fn get_sigmaupdatemanager_mut() -> Option<&'static mut SigmaUpdateManager> {
    unsafe {
        SIGMAUPDATEMANAGER_INSTANCE.as_mut()
    }
}

/// Convenience function to add an update
pub fn add_update(update: Update) -> bool {
    if let Some(update_mgr) = get_sigmaupdatemanager_mut() {
        update_mgr.add_update(update)
    } else {
        false
    }
}

/// Convenience function to install an update
pub fn install_update(update_id: &str) -> bool {
    if let Some(update_mgr) = get_sigmaupdatemanager_mut() {
        update_mgr.install_update(update_id)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_creation() {
        let update = Update::new(
            "update1".to_string(),
            "kernel".to_string(),
            "6.5.0".to_string(),
            UpdateType::Kernel,
            "Linux kernel update".to_string(),
        );

        assert_eq!(update.update_id, "update1");
        assert_eq!(update.update_type, UpdateType::Kernel);
        assert_eq!(update.state, UpdateState::Available);
    }

    #[test]
    fn test_update_state() {
        let mut update = Update::new(
            "update1".to_string(),
            "kernel".to_string(),
            "6.5.0".to_string(),
            UpdateType::Kernel,
            "Linux kernel update".to_string(),
        );

        update.set_state(UpdateState::Installed);
        assert!(update.is_installed());

        update.set_critical(true);
        assert!(update.is_update_critical());
    }

    #[test]
    fn test_update_schedule_creation() {
        let schedule = UpdateSchedule::new("daily".to_string(), 24);
        assert_eq!(schedule.schedule_id, "daily");
        assert!(schedule.is_enabled());
    }

    #[test]
    fn test_sigmaupdatemanager_creation() {
        let update_mgr = SigmaUpdateManager::new();
        assert!(!update_mgr.is_running());
        assert_eq!(update_mgr.update_count(), 0);
        assert_eq!(update_mgr.schedule_count(), 0);
    }

    #[test]
    fn test_sigmaupdatemanager_start_stop() {
        let mut update_mgr = SigmaUpdateManager::new();
        update_mgr.start();
        assert!(update_mgr.is_running());
        update_mgr.stop();
        assert!(!update_mgr.is_running());
    }

    #[test]
    fn test_sigmaupdatemanager_add_update() {
        let mut update_mgr = SigmaUpdateManager::new();
        let update = Update::new(
            "update1".to_string(),
            "kernel".to_string(),
            "6.5.0".to_string(),
            UpdateType::Kernel,
            "Linux kernel update".to_string(),
        );

        let added = update_mgr.add_update(update);
        assert!(added);
        assert_eq!(update_mgr.update_count(), 1);
    }

    #[test]
    fn test_sigmaupdatemanager_install_update() {
        let mut update_mgr = SigmaUpdateManager::new();

        update_mgr.add_update(Update::new(
            "update1".to_string(),
            "kernel".to_string(),
            "6.5.0".to_string(),
            UpdateType::Kernel,
            "Linux kernel update".to_string(),
        ));

        let installed = update_mgr.install_update("update1");
        assert!(installed);

        let update = update_mgr.get_update("update1").unwrap();
        assert!(update.is_installed());
    }

    #[test]
    fn test_sigmaupdatemanager_install_all() {
        let mut update_mgr = SigmaUpdateManager::new();

        update_mgr.add_update(Update::new(
            "update1".to_string(),
            "kernel".to_string(),
            "6.5.0".to_string(),
            UpdateType::Kernel,
            "Linux kernel update".to_string(),
        ));

        update_mgr.add_update(Update::new(
            "update2".to_string(),
            "firefox".to_string(),
            "120.0".to_string(),
            UpdateType::Feature,
            "Firefox update".to_string(),
        ));

        let count = update_mgr.install_all_updates();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_global_instance() {
        init_sigmaupdatemanager();

        let update_mgr = get_sigmaupdatemanager();
        assert!(update_mgr.is_some());

        let mut update_mgr_mut = get_sigmaupdatemanager_mut();
        assert!(update_mgr_mut.is_some());

        if let Some(update_mgr) = update_mgr_mut {
            update_mgr.start();
            assert!(update_mgr.is_running());
        }
    }

    #[test]
    fn test_convenience_add_update() {
        init_sigmaupdatemanager();

        let update = Update::new(
            "update1".to_string(),
            "kernel".to_string(),
            "6.5.0".to_string(),
            UpdateType::Kernel,
            "Linux kernel update".to_string(),
        );

        let added = add_update(update);
        assert!(added);

        let update_mgr = get_sigmaupdatemanager().unwrap();
        assert_eq!(update_mgr.update_count(), 1);
    }

    #[test]
    fn test_convenience_install_update() {
        init_sigmaupdatemanager();

        let update = Update::new(
            "update1".to_string(),
            "kernel".to_string(),
            "6.5.0".to_string(),
            UpdateType::Kernel,
            "Linux kernel update".to_string(),
        );

        add_update(update);
        let installed = install_update("update1");
        assert!(installed);

        let update_mgr = get_sigmaupdatemanager().unwrap();
        let upd = update_mgr.get_update("update1").unwrap();
        assert!(upd.is_installed());
    }
}
