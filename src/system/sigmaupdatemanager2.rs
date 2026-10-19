//! SigmaUpdateManager - System Update Management System
//!
//! Provides comprehensive system update management inspired by Linux Mint and Omarchy.
//! Supports package updates, security patches, kernel updates, and update scheduling.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Update priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum UpdatePriority {
    Critical,
    High,
    Normal,
    Low,
;

/// Update type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateType {
    Security,
    Bugfix,
    Feature,
    Kernel,
    Driver,
    Application,
;

/// Update status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateStatus {
    Available,
    Downloading,
    Downloaded,
    Installing,
    Installed,
    Failed,
;

/// System update
#[derive(Debug, Clone)]
pub struct SystemUpdate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub update_type: UpdateType,
    pub priority: UpdatePriority,
    pub status: UpdateStatus,
    pub version: String,
    pub current_version: String,
    pub size: u64, // in bytes
    pub download_size: u64,
    pub changelog: String,
    pub security_advisory: Option<String>,
;

impl SystemUpdate {
    pub fn new(id: String, name: String, description: String, update_type: UpdateType, priority: UpdatePriority, version: String, current_version: String) -> Self {
        Self {
            id,
            name,
            description,
            update_type,
            priority,
            status: UpdateStatus::Available,
            version,
            current_version,
            size: 0,
            download_size: 0,
            changelog: String::new(),
            security_advisory: None,
        }
    }

    pub fn set_status(&mut self, status: UpdateStatus) {
        self.status = status;
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn set_download_size(&mut self, size: u64) {
        self.download_size = size;
    }

    pub fn set_changelog(&mut self, changelog: String) {
        self.changelog = changelog;
    }

    pub fn set_security_advisory(&mut self, advisory: Option<String>) {
        self.security_advisory = advisory;
    }

    pub fn is_security(&self) -> bool {
        self.update_type == UpdateType::Security
    }

    pub fn is_critical(&self) -> bool {
        self.priority == UpdatePriority::Critical
    }
;

/// Update schedule
#[derive(Debug, Clone)]
pub struct UpdateSchedule {
    pub id: String,
    pub name: String,
    pub auto_update: bool,
    pub auto_download: bool,
    pub schedule_type: String, // daily, weekly, monthly
    pub day_of_week: Option<String>, // Monday, Tuesday, etc.
    pub time: String, // HH:MM
    pub enabled: bool,
;

impl UpdateSchedule {
    pub fn new(id: String, name: String, auto_update: bool, auto_download: bool, schedule_type: String, time: String) -> Self {
        Self {
            id,
            name,
            auto_update,
            auto_download,
            schedule_type,
            day_of_week: None,
            time,
            enabled: true,
        }
    }

    pub fn set_day_of_week(&mut self, day: Option<String>) {
        self.day_of_week = day;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
;

/// SigmaUpdateManager - System update management
#[derive(Debug)]
pub struct SigmaUpdateManager {
    updates: BTreeMap<String, SystemUpdate>,
    schedules: BTreeMap<String, UpdateSchedule>,
    last_check: Option<String>,
    last_update: Option<String>,
;

impl SigmaUpdateManager {
    pub fn new() -> Self {
        Self {
            updates: BTreeMap::new(),
            schedules: BTreeMap::new(),
            last_check: None,
            last_update: None,
        }
    }

    pub fn add_update(&mut self, update: SystemUpdate) {
        self.updates.insert(update.id.clone(), update);
    }

    pub fn remove_update(&mut self, id: &str) -> bool {
        self.updates.remove(id).is_some()
    }

    pub fn get_update(&self, id: &str) -> Option<&SystemUpdate> {
        self.updates.get(id)
    }

    pub fn get_update_mut(&mut self, id: &str) -> Option<&mut SystemUpdate> {
        self.updates.get_mut(id)
    }

    pub fn add_schedule(&mut self, schedule: UpdateSchedule) {
        self.schedules.insert(schedule.id.clone(), schedule);
    }

    pub fn remove_schedule(&mut self, id: &str) -> bool {
        self.schedules.remove(id).is_some()
    }

    pub fn get_schedule(&self, id: &str) -> Option<&UpdateSchedule> {
        self.schedules.get(id)
    }

    pub fn list_security_updates(&self) -> Vec<&SystemUpdate> {
        self.updates.values()
            .filter(|update| update.is_security())
            .collect()
    }

    pub fn list_critical_updates(&self) -> Vec<&SystemUpdate> {
        self.updates.values()
            .filter(|update| update.is_critical())
            .collect()
    }

    pub fn list_by_priority(&self, priority: UpdatePriority) -> Vec<&SystemUpdate> {
        self.updates.values()
            .filter(|update| update.priority == priority)
            .collect()
    }

    pub fn list_by_type(&self, update_type: UpdateType) -> Vec<&SystemUpdate> {
        self.updates.values()
            .filter(|update| update.update_type == update_type)
            .collect()
    }

    pub fn list_available(&self) -> Vec<&SystemUpdate> {
        self.updates.values()
            .filter(|update| update.status == UpdateStatus::Available)
            .collect()
    }

    pub fn total_download_size(&self) -> u64 {
        self.updates.values()
            .map(|update| update.download_size)
            .sum()
    }

    pub fn set_last_check(&mut self, time: String) {
        self.last_check = Some(time);
    }

    pub fn set_last_update(&mut self, time: String) {
        self.last_update = Some(time);
    }

    pub fn update_count(&self) -> usize {
        self.updates.len()
    }

    pub fn schedule_count(&self) -> usize {
        self.schedules.len()
    }
;

impl Default for SigmaUpdateManager {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaUpdateManager instance
static mut SIGMAUPDATEMANAGER_INSTANCE: Option<SigmaUpdateManager> = None;

/// Initialize the global SigmaUpdateManager instance
pub fn init_sigmaupdatemanager() {
    unsafe {
        SIGMAUPDATEMANAGER_INSTANCE = Some(SigmaUpdateManager::new());
    }
;

/// Get the global SigmaUpdateManager instance
pub fn get_sigmaupdatemanager() -> Option<&'static SigmaUpdateManager> {
    unsafe {
        SIGMAUPDATEMANAGER_INSTANCE.as_ref()
    }
;

/// Get the global SigmaUpdateManager instance (mutable)
pub fn get_sigmaupdatemanager_mut() -> Option<&'static mut SigmaUpdateManager> {
    unsafe {
        SIGMAUPDATEMANAGER_INSTANCE.as_mut()
    }
;

/// Convenience function to add an update
pub fn add_update(update: SystemUpdate) -> bool {
    if let Some(manager) = get_sigmaupdatemanager_mut() {
        manager.add_update(update);
        true
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_update_creation() {
        let update = SystemUpdate::new(
            "update1".to_string(),
            "Kernel Update".to_string(),
            "Security fix".to_string(),
            UpdateType::Security,
            UpdatePriority::Critical,
            "5.15.0".to_string(),
            "5.14.0".to_string(),
        );
        assert_eq!(update.id, "update1");
        assert!(update.is_security());
        assert!(update.is_critical());
    }

    #[test]
    fn test_system_update_modification() {
        let mut update = SystemUpdate::new(
            "update1".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            UpdateType::Bugfix,
            UpdatePriority::Normal,
            "2.0".to_string(),
            "1.0".to_string(),
        );
        update.set_status(UpdateStatus::Downloaded);
        update.set_size(1000000);
        update.set_download_size(500000);
        assert_eq!(update.status, UpdateStatus::Downloaded);
    }

    #[test]
    fn test_update_schedule_creation() {
        let schedule = UpdateSchedule::new(
            "sched1".to_string(),
            "Daily Updates".to_string(),
            true,
            true,
            "daily".to_string(),
            "03:00".to_string(),
        );
        assert_eq!(schedule.id, "sched1");
        assert!(schedule.auto_update);
    }

    #[test]
    fn test_sigmaupdatemanager_creation() {
        let manager = SigmaUpdateManager::new();
        assert_eq!(manager.update_count(), 0);
        assert_eq!(manager.schedule_count(), 0);
    }

    #[test]
    fn test_sigmaupdatemanager_management() {
        let mut manager = SigmaUpdateManager::new();
        let update = SystemUpdate::new(
            "update1".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            UpdateType::Feature,
            UpdatePriority::Normal,
            "2.0".to_string(),
            "1.0".to_string(),
        );
        manager.add_update(update);
        assert_eq!(manager.update_count(), 1);
    }

    #[test]
    fn test_security_update_filtering() {
        let mut manager = SigmaUpdateManager::new();
        let update = SystemUpdate::new(
            "update1".to_string(),
            "Security Fix".to_string(),
            "Fix".to_string(),
            UpdateType::Security,
            UpdatePriority::Critical,
            "1.1".to_string(),
            "1.0".to_string(),
        );
        manager.add_update(update);
        let security = manager.list_security_updates();
        assert_eq!(security.len(), 1);
    }
;
