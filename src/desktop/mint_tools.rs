// SigmaOS Linux Mint Parity Tools
// Implements Linux Mint-style desktop utilities for SigmaOS
// Inspired by Linux Mint's user-friendly system tools

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// Update level (1-5 tier system from Linux Mint)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UpdateLevel {
    Level1, // Safe updates
    Level2, // Recommended updates
    Level3, // Unstable updates
    Level4, // Dangerous updates
    Level5, // Experimental updates
}

/// Update package information
#[derive(Debug, Clone)]
pub struct UpdatePackage {
    pub name: String,
    pub version: String,
    pub level: UpdateLevel,
    pub size: u64,
    pub description: String,
    pub security: bool,
    pub kernel: bool,
}

/// Update manager
pub struct MintUpdateManager {
    pub updates: Vec<UpdatePackage>,
    pub auto_update_levels: Vec<UpdateLevel>,
}

impl MintUpdateManager {
    pub fn new() -> Self {
        Self {
            updates: Vec::new(),
            auto_update_levels: vec![UpdateLevel::Level1, UpdateLevel::Level2],
        }
    }

    /// Add update
    pub fn add_update(&mut self, update: UpdatePackage) {
        self.updates.push(update);
    }

    /// Get updates by level
    pub fn get_updates_by_level(&self, level: UpdateLevel) -> Vec<&UpdatePackage> {
        self.updates.iter().filter(|u| u.level == level).collect()
    }

    /// Get security updates
    pub fn get_security_updates(&self) -> Vec<&UpdatePackage> {
        self.updates.iter().filter(|u| u.security).collect()
    }

    /// Get kernel updates
    pub fn get_kernel_updates(&self) -> Vec<&UpdatePackage> {
        self.updates.iter().filter(|u| u.kernel).collect()
    }

    /// Install updates up to specified level
    pub fn install_updates(&mut self, max_level: UpdateLevel) -> Vec<String> {
        let mut installed = Vec::new();
        
        for update in &self.updates {
            if self.is_update_allowed(update.level, max_level) {
                println!("Installing {} ({})", update.name, update.version);
                installed.push(update.name.clone());
            }
        }

        installed
    }

    /// Check if update is allowed based on level
    fn is_update_allowed(&self, update_level: UpdateLevel, max_level: UpdateLevel) -> bool {
        if self.auto_update_levels.contains(&update_level) {
            return true;
        }
        
        match update_level {
            UpdateLevel::Level1 => true,
            UpdateLevel::Level2 => max_level >= UpdateLevel::Level2,
            UpdateLevel::Level3 => max_level >= UpdateLevel::Level3,
            UpdateLevel::Level4 => max_level >= UpdateLevel::Level4,
            UpdateLevel::Level5 => max_level >= UpdateLevel::Level5,
        }
    }

    /// Set auto-update levels
    pub fn set_auto_update_levels(&mut self, levels: Vec<UpdateLevel>) {
        self.auto_update_levels = levels;
    }
}

impl Default for MintUpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotType {
    Btrfs,
    Rsync,
}

/// Timeshift snapshot
#[derive(Debug, Clone)]
pub struct TimeshiftSnapshot {
    pub id: String,
    pub timestamp: u64,
    pub snapshot_type: SnapshotType,
    pub description: String,
    pub size: u64,
    pub on_boot: bool,
    pub on_hourly: bool,
    pub on_daily: bool,
    pub on_weekly: bool,
    pub on_monthly: bool,
}

/// Timeshift engine
pub struct MintTimeshiftEngine {
    pub snapshots: Vec<TimeshiftSnapshot>,
    pub snapshot_interval: String,
    pub max_snapshots: usize,
}

impl MintTimeshiftEngine {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            snapshot_interval: "hourly".to_string(),
            max_snapshots: 24,
        }
    }

    /// Create snapshot
    pub fn create_snapshot(&mut self, description: String) -> String {
        let id = format!("snapshot_{}", self.snapshots.len() + 1);
        let snapshot = TimeshiftSnapshot {
            id: id.clone(),
            timestamp: 1234567890,
            snapshot_type: SnapshotType::Btrfs,
            description,
            size: 1024 * 1024 * 100,
            on_boot: false,
            on_hourly: true,
            on_daily: false,
            on_weekly: false,
            on_monthly: false,
        };
        
        self.snapshots.push(snapshot);
        println!("Created snapshot: {}", id);
        id
    }

    /// Restore snapshot
    pub fn restore_snapshot(&self, id: &str) -> Result<(), String> {
        if let Some(snapshot) = self.snapshots.iter().find(|s| s.id == id) {
            println!("Restoring snapshot: {} ({})", id, snapshot.description);
            Ok(())
        } else {
            Err(format!("Snapshot {} not found", id))
        }
    }

    /// Delete snapshot
    pub fn delete_snapshot(&mut self, id: &str) -> Result<(), String> {
        if let Some(pos) = self.snapshots.iter().position(|s| s.id == id) {
            self.snapshots.remove(pos);
            println!("Deleted snapshot: {}", id);
            Ok(())
        } else {
            Err(format!("Snapshot {} not found", id))
        }
    }

    /// Get latest snapshot
    pub fn get_latest_snapshot(&self) -> Option<&TimeshiftSnapshot> {
        self.snapshots.last()
    }

    /// Clean old snapshots
    pub fn clean_old_snapshots(&mut self) {
        while self.snapshots.len() > self.max_snapshots {
            if let Some(snapshot) = self.snapshots.first() {
                println!("Cleaning old snapshot: {}", snapshot.id);
                self.snapshots.remove(0);
            }
        }
    }
}

impl Default for MintTimeshiftEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Software manager app metadata
#[derive(Debug, Clone)]
pub struct AppMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub category: String,
    pub icon: String,
    pub screenshots: Vec<String>,
    pub rating: f32,
    pub reviews: u32,
    pub size: u64,
    pub installed: bool,
}

/// Software manager
pub struct MintSoftwareManager {
    pub apps: BTreeMap<String, AppMetadata>,
    pub installed_apps: Vec<String>,
}

impl MintSoftwareManager {
    pub fn new() -> Self {
        Self {
            apps: BTreeMap::new(),
            installed_apps: Vec::new(),
        }
    }

    /// Add app
    pub fn add_app(&mut self, app: AppMetadata) {
        self.apps.insert(app.name.clone(), app);
    }

    /// Search apps
    pub fn search_apps(&self, query: &str) -> Vec<&AppMetadata> {
        let query_lower = query.to_lowercase();
        self.apps.values()
            .filter(|app| {
                app.name.to_lowercase().contains(&query_lower) ||
                app.description.to_lowercase().contains(&query_lower) ||
                app.category.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get apps by category
    pub fn get_apps_by_category(&self, category: &str) -> Vec<&AppMetadata> {
        self.apps.values()
            .filter(|app| app.category == category)
            .collect()
    }

    /// Install app
    pub fn install_app(&mut self, name: &str) -> Result<(), String> {
        if let Some(app) = self.apps.get(name) {
            println!("Installing {} ({})", name, app.version);
            self.installed_apps.push(name.to_string());
            Ok(())
        } else {
            Err(format!("App {} not found", name))
        }
    }

    /// Remove app
    pub fn remove_app(&mut self, name: &str) -> Result<(), String> {
        if let Some(pos) = self.installed_apps.iter().position(|n| n == name) {
            self.installed_apps.remove(pos);
            println!("Removed {}", name);
            Ok(())
        } else {
            Err(format!("App {} not installed", name))
        }
    }

    /// Get installed apps
    pub fn get_installed_apps(&self) -> Vec<&AppMetadata> {
        self.installed_apps.iter()
            .filter_map(|name| self.apps.get(name))
            .collect()
    }
}

impl Default for MintSoftwareManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_manager() {
        let mut manager = MintUpdateManager::new();
        
        let update = UpdatePackage {
            name: "linux-kernel".to_string(),
            version: "6.0.0".to_string(),
            level: UpdateLevel::Level1,
            size: 1024 * 1024 * 50,
            description: "Linux kernel update".to_string(),
            security: true,
            kernel: true,
        };
        
        manager.add_update(update);
        let installed = manager.install_updates(UpdateLevel::Level2);
        assert_eq!(installed.len(), 1);
    }

    #[test]
    fn test_timeshift() {
        let mut timeshift = MintTimeshiftEngine::new();
        let id = timeshift.create_snapshot("Test snapshot".to_string());
        assert!(timeshift.restore_snapshot(&id).is_ok());
    }

    #[test]
    fn test_software_manager() {
        let mut manager = MintSoftwareManager::new();
        
        let app = AppMetadata {
            name: "firefox".to_string(),
            version: "120.0".to_string(),
            description: "Web browser".to_string(),
            category: "Internet".to_string(),
            icon: "firefox".to_string(),
            screenshots: vec![],
            rating: 4.5,
            reviews: 1000,
            size: 1024 * 1024 * 100,
            installed: false,
        };
        
        manager.add_app(app);
        let result = manager.install_app("firefox");
        assert!(result.is_ok());
    }
}
