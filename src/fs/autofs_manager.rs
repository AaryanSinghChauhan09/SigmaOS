// AutoFS for SigmaOS
// AutoFS per Wiki 04-Kernel.md
// Provides on-demand filesystem mounting

use std::string::{String, ToString};
use std::vec::Vec;

/// AutoFS mount trigger
#[derive(Debug, Clone)]
pub struct AutoFsTrigger {
    pub mount_point: String,
    pub device: String,
    pub fs_type: String,
    pub options: Vec<String>,
}

impl AutoFsTrigger {
    pub fn new(mount_point: String, device: String, fs_type: String) -> Self {
        AutoFsTrigger {
            mount_point,
            device,
            fs_type,
            options: Vec::new(),
        }
    }

    pub fn with_option(mut self, option: String) -> Self {
        self.options.push(option);
        self
    }

    pub fn with_options(mut self, options: Vec<String>) -> Self {
        self.options = options;
        self
    }

    pub fn generate_mount_command(&self) -> String {
        let mut cmd = format!("mount -t {} {}", self.fs_type, self.device);
        
        if !self.options.is_empty() {
            cmd.push_str(" -o ");
            cmd.push_str(&self.options.join(","));
        }
        
        cmd.push_str(&format!(" {}", self.mount_point));
        
        cmd
    }

    pub fn generate_umount_command(&self) -> String {
        format!("umount {}", self.mount_point)
    }
}

/// AutoFS mount state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoFsState {
    Idle,
    Triggered,
    Mounted,
    Failed,
}

impl AutoFsState {
    pub fn as_str(&self) -> &str {
        match self {
            AutoFsState::Idle => "idle",
            AutoFsState::Triggered => "triggered",
            AutoFsState::Mounted => "mounted",
            AutoFsState::Failed => "failed",
        }
    }
}

/// AutoFS entry
#[derive(Debug, Clone)]
pub struct AutoFsEntry {
    pub trigger: AutoFsTrigger,
    pub state: AutoFsState,
    pub last_access: Option<u64>,
    pub idle_timeout_seconds: u32,
}

impl AutoFsEntry {
    pub fn new(trigger: AutoFsTrigger, idle_timeout_seconds: u32) -> Self {
        AutoFsEntry {
            trigger,
            state: AutoFsState::Idle,
            last_access: None,
            idle_timeout_seconds,
        }
    }

    pub fn with_idle_timeout(mut self, timeout: u32) -> Self {
        self.idle_timeout_seconds = timeout;
        self
    }

    pub fn trigger(&mut self) {
        self.state = AutoFsState::Triggered;
    }

    pub fn mount(&mut self) {
        self.state = AutoFsState::Mounted;
        self.last_access = Some(0); // Would be actual timestamp
    }

    pub fn unmount(&mut self) {
        self.state = AutoFsState::Idle;
        self.last_access = None;
    }

    pub fn fail(&mut self) {
        self.state = AutoFsState::Failed;
    }

    pub fn update_access(&mut self, timestamp: u64) {
        self.last_access = Some(timestamp);
    }

    pub fn is_idle(&self, current_timestamp: u64) -> bool {
        if let Some(last_access) = self.last_access {
            (current_timestamp - last_access) as u32 > self.idle_timeout_seconds
        } else {
            true
        }
    }

    pub fn should_unmount(&self, current_timestamp: u64) -> bool {
        self.state == AutoFsState::Mounted && self.is_idle(current_timestamp)
    }
}

/// AutoFS manager
#[derive(Debug, Clone)]
pub struct AutoFsManager {
    pub entries: Vec<AutoFsEntry>,
    pub enabled: bool,
}

impl Default for AutoFsManager {
    fn default() -> Self {
        AutoFsManager {
            entries: Vec::new(),
            enabled: true,
        }
    }
}

impl AutoFsManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, mount_point: String, device: String, fs_type: String) -> bool {
        let trigger = AutoFsTrigger::new(mount_point, device, fs_type);
        let entry = AutoFsEntry::new(trigger, 300); // Default 5 minute timeout
        self.entries.push(entry);
        true
    }

    pub fn register_with_timeout(&mut self, mount_point: String, device: String, fs_type: String, timeout: u32) -> bool {
        let trigger = AutoFsTrigger::new(mount_point, device, fs_type);
        let entry = AutoFsEntry::new(trigger, timeout);
        self.entries.push(entry);
        true
    }

    pub fn trigger(&mut self, mount_point: &str) -> Result<String, String> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.trigger.mount_point == mount_point) {
            entry.trigger();
            Ok(String::from("Triggered successfully"))
        } else {
            Err(String::from("Mount point not found"))
        }
    }

    pub fn mount(&mut self, mount_point: &str) -> Result<String, String> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.trigger.mount_point == mount_point) {
            entry.mount();
            Ok(format!("Mounted: {}", entry.trigger.generate_mount_command()))
        } else {
            Err(String::from("Mount point not found"))
        }
    }

    pub fn unmount(&mut self, mount_point: &str) -> Result<String, String> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.trigger.mount_point == mount_point) {
            entry.unmount();
            Ok(format!("Unmounted: {}", entry.trigger.generate_umount_command()))
        } else {
            Err(String::from("Mount point not found"))
        }
    }

    pub fn set_timeout(&mut self, mount_point: &str, timeout: u32) -> Result<String, String> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.trigger.mount_point == mount_point) {
            entry.idle_timeout_seconds = timeout;
            Ok(String::from("Timeout updated"))
        } else {
            Err(String::from("Mount point not found"))
        }
    }

    pub fn unregister(&mut self, mount_point: &str) -> bool {
        let original_len = self.entries.len();
        self.entries.retain(|e| e.trigger.mount_point != mount_point);
        original_len > self.entries.len()
    }

    pub fn get_entry(&self, mount_point: &str) -> Option<&AutoFsEntry> {
        self.entries.iter().find(|e| e.trigger.mount_point == mount_point)
    }

    pub fn get_entry_mut(&mut self, mount_point: &str) -> Option<&mut AutoFsEntry> {
        self.entries.iter_mut().find(|e| e.trigger.mount_point == mount_point)
    }

    pub fn list_entries(&self) -> Vec<String> {
        self.entries.iter()
            .map(|e| format!("{} - {} ({})", e.trigger.mount_point, e.trigger.device, e.state.as_str()))
            .collect()
    }

    pub fn list_mounted(&self) -> Vec<String> {
        self.entries.iter()
            .filter(|e| e.state == AutoFsState::Mounted)
            .map(|e| e.trigger.mount_point.clone())
            .collect()
    }

    pub fn list_idle(&self, current_timestamp: u64) -> Vec<String> {
        self.entries.iter()
            .filter(|e| e.should_unmount(current_timestamp))
            .map(|e| e.trigger.mount_point.clone())
            .collect()
    }

    pub fn cleanup_idle(&mut self, current_timestamp: u64) -> Vec<String> {
        let mut unmounted = Vec::new();
        
        for entry in self.entries.iter_mut() {
            if entry.should_unmount(current_timestamp) {
                entry.unmount();
                unmounted.push(entry.trigger.mount_point.clone());
            }
        }
        
        unmounted
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_stats(&self) -> AutoFsStats {
        let mounted = self.entries.iter().filter(|e| e.state == AutoFsState::Mounted).count();
        let idle = self.entries.iter().filter(|e| e.state == AutoFsState::Idle).count();
        let triggered = self.entries.iter().filter(|e| e.state == AutoFsState::Triggered).count();
        let failed = self.entries.iter().filter(|e| e.state == AutoFsState::Failed).count();
        
        AutoFsStats {
            total_entries: self.entries.len(),
            mounted,
            idle,
            triggered,
            failed,
            enabled: self.enabled,
        }
    }
}

/// AutoFS statistics
#[derive(Debug, Clone)]
pub struct AutoFsStats {
    pub total_entries: usize,
    pub mounted: usize,
    pub idle: usize,
    pub triggered: usize,
    pub failed: usize,
    pub enabled: bool,
}

impl AutoFsStats {
    pub fn as_summary(&self) -> String {
        format!(
            "AutoFS Stats: {} total, {} mounted, {} idle, {} triggered, {} failed (enabled: {})",
            self.total_entries, self.mounted, self.idle, self.triggered, self.failed, self.enabled
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autofs_trigger_creation() {
        let trigger = AutoFsTrigger::new(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        
        assert_eq!(trigger.mount_point, "/mnt/data");
        assert_eq!(trigger.device, "/dev/sda1");
        assert_eq!(trigger.fs_type, "ext4");
    }

    #[test]
    fn test_autofs_trigger_with_option() {
        let trigger = AutoFsTrigger::new(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        ).with_option(String::from("noatime"));
        
        assert!(trigger.options.contains(&String::from("noatime")));
    }

    #[test]
    fn test_autofs_trigger_generate_mount_command() {
        let trigger = AutoFsTrigger::new(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        
        let cmd = trigger.generate_mount_command();
        assert!(cmd.contains("mount -t ext4 /dev/sda1"));
        assert!(cmd.contains("/mnt/data"));
    }

    #[test]
    fn test_autofs_trigger_generate_umount_command() {
        let trigger = AutoFsTrigger::new(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        
        let cmd = trigger.generate_umount_command();
        assert_eq!(cmd, "umount /mnt/data");
    }

    #[test]
    fn test_autofs_state_as_str() {
        assert_eq!(AutoFsState::Idle.as_str(), "idle");
        assert_eq!(AutoFsState::Triggered.as_str(), "triggered");
        assert_eq!(AutoFsState::Mounted.as_str(), "mounted");
        assert_eq!(AutoFsState::Failed.as_str(), "failed");
    }

    #[test]
    fn test_autofs_entry_creation() {
        let trigger = AutoFsTrigger::new(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        let entry = AutoFsEntry::new(trigger, 300);
        
        assert_eq!(entry.state, AutoFsState::Idle);
        assert_eq!(entry.idle_timeout_seconds, 300);
    }

    #[test]
    fn test_autofs_entry_state_transitions() {
        let trigger = AutoFsTrigger::new(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        let mut entry = AutoFsEntry::new(trigger, 300);
        
        entry.trigger();
        assert_eq!(entry.state, AutoFsState::Triggered);
        
        entry.mount();
        assert_eq!(entry.state, AutoFsState::Mounted);
        
        entry.unmount();
        assert_eq!(entry.state, AutoFsState::Idle);
    }

    #[test]
    fn test_autofs_entry_is_idle() {
        let trigger = AutoFsTrigger::new(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        let mut entry = AutoFsEntry::new(trigger, 300);
        
        assert!(entry.is_idle(1000));
        
        entry.mount();
        entry.update_access(1000);
        assert!(!entry.is_idle(1200)); // 200 seconds ago
        
        assert!(entry.is_idle(2000)); // 1000 seconds ago (over 300 timeout)
    }

    #[test]
    fn test_autofs_entry_should_unmount() {
        let trigger = AutoFsTrigger::new(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        let mut entry = AutoFsEntry::new(trigger, 300);
        
        assert!(!entry.should_unmount(1000));
        
        entry.mount();
        entry.update_access(1000);
        assert!(!entry.should_unmount(1200));
        
        assert!(entry.should_unmount(2000));
    }

    #[test]
    fn test_autofs_manager_creation() {
        let manager = AutoFsManager::new();
        assert!(manager.enabled);
        assert_eq!(manager.entries.len(), 0);
    }

    #[test]
    fn test_autofs_manager_register() {
        let mut manager = AutoFsManager::new();
        assert!(manager.register(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        ));
        
        assert_eq!(manager.entries.len(), 1);
    }

    #[test]
    fn test_autofs_manager_register_with_timeout() {
        let mut manager = AutoFsManager::new();
        assert!(manager.register_with_timeout(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
            600,
        ));
        
        assert_eq!(manager.entries.first().unwrap().idle_timeout_seconds, 600);
    }

    #[test]
    fn test_autofs_manager_trigger() {
        let mut manager = AutoFsManager::new();
        manager.register(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        
        assert!(manager.trigger("/mnt/data").is_ok());
        assert_eq!(manager.entries.first().unwrap().state, AutoFsState::Triggered);
    }

    #[test]
    fn test_autofs_manager_mount() {
        let mut manager = AutoFsManager::new();
        manager.register(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        
        assert!(manager.mount("/mnt/data").is_ok());
        assert_eq!(manager.entries.first().unwrap().state, AutoFsState::Mounted);
    }

    #[test]
    fn test_autofs_manager_unmount() {
        let mut manager = AutoFsManager::new();
        manager.register(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        manager.mount("/mnt/data");
        
        assert!(manager.unmount("/mnt/data").is_ok());
        assert_eq!(manager.entries.first().unwrap().state, AutoFsState::Idle);
    }

    #[test]
    fn test_autofs_manager_set_timeout() {
        let mut manager = AutoFsManager::new();
        manager.register(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        
        assert!(manager.set_timeout("/mnt/data", 600).is_ok());
        assert_eq!(manager.entries.first().unwrap().idle_timeout_seconds, 600);
    }

    #[test]
    fn test_autofs_manager_unregister() {
        let mut manager = AutoFsManager::new();
        manager.register(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        
        assert!(manager.unregister("/mnt/data"));
        assert_eq!(manager.entries.len(), 0);
    }

    #[test]
    fn test_autofs_manager_list_entries() {
        let mut manager = AutoFsManager::new();
        manager.register(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        
        let entries = manager.list_entries();
        assert_eq!(entries.len(), 1);
        assert!(entries[0].contains("/mnt/data"));
    }

    #[test]
    fn test_autofs_manager_list_mounted() {
        let mut manager = AutoFsManager::new();
        manager.register(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        manager.mount("/mnt/data");
        
        let mounted = manager.list_mounted();
        assert_eq!(mounted.len(), 1);
        assert_eq!(mounted[0], "/mnt/data");
    }

    #[test]
    fn test_autofs_manager_cleanup_idle() {
        let mut manager = AutoFsManager::new();
        manager.register(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        manager.mount("/mnt/data");
        
        // Simulate idle timeout
        let unmounted = manager.cleanup_idle(10000);
        assert_eq!(unmounted.len(), 1);
        assert_eq!(manager.entries.first().unwrap().state, AutoFsState::Idle);
    }

    #[test]
    fn test_autofs_manager_enable_disable() {
        let mut manager = AutoFsManager::new();
        assert!(manager.is_enabled());
        
        manager.disable();
        assert!(!manager.is_enabled());
        
        manager.enable();
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_autofs_manager_get_stats() {
        let mut manager = AutoFsManager::new();
        manager.register(
            String::from("/mnt/data"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        manager.mount("/mnt/data");
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_entries, 1);
        assert_eq!(stats.mounted, 1);
        assert_eq!(stats.idle, 0);
    }

    #[test]
    fn test_autofs_stats_as_summary() {
        let stats = AutoFsStats {
            total_entries: 5,
            mounted: 2,
            idle: 2,
            triggered: 1,
            failed: 0,
            enabled: true,
        };
        
        let summary = stats.as_summary();
        assert!(summary.contains("5 total"));
        assert!(summary.contains("2 mounted"));
    }
}
