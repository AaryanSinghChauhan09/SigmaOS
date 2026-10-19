//! SigmaOS Miscellaneous Tools
//!
//! Inspired by: Omarchy miscellaneous tools (omancy-done, omancy-drive-*, omancy-file-select,
//! omancy-font-*, omancy-games-*, omancy-git-*, omancy-hook, omancy-hw-*, omancy-monitor-*,
//! omancy-refresh-*, omancy-reminder, omancy-state, omancy-transcode-*, omancy-tui-*,
//! omancy-voxtype-*, omancy-windows-vm)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, DIRECT_DEVICE_CONTROL, LOW_LEVEL_CONTROL, USER_CONTROL,
/// CODE_REUSABILITY, COMPOSITION_OVER_INHERITANCE, PLUG_IN_DRIVER_MODEL.

/// Drive information
#[derive(Debug, Clone)]
pub struct DriveInfo {
    pub id: String,
    pub name: String,
    pub device: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub mount_point: String,
    pub filesystem: String,
}

/// Drive manager
pub struct DriveManager {
    pub drives: HashMap<String, DriveInfo>,
}

impl DriveManager {
    pub fn new() -> Self {
        Self {
            drives: HashMap::new(),
        }
    }

    /// Add a drive
    pub fn add_drive(&mut self, drive: DriveInfo) {
        self.drives.insert(drive.id.clone(), drive);
    }

    /// Get drive by ID
    pub fn get_drive(&self, id: &str) -> Option<&DriveInfo> {
        self.drives.get(id)
    }

    /// List drives
    pub fn list_drives(&self) -> Vec<&DriveInfo> {
        self.drives.values().collect()
    }

    /// Get usage percent for a drive
    pub fn get_usage_percent(&self, id: &str) -> Option<f32> {
        let drive = self.drives.get(id)?;
        if drive.total_bytes == 0 {
            return Some(0.0);
        }
        Some((drive.used_bytes as f32 / drive.total_bytes as f32) * 100.0)
    }

    /// Drive count
    pub fn drive_count(&self) -> usize {
        self.drives.len()
    }
}

impl Default for DriveManager {
    fn default() -> Self {
        Self::new()
    }
}

/// TUI manager
pub struct TuiManager {
    pub installed_apps: HashMap<String, String>,
}

impl TuiManager {
    pub fn new() -> Self {
        Self {
            installed_apps: HashMap::new(),
        }
    }

    /// Install a TUI app
    pub fn install(&mut self, name: String, binary: String) {
        self.installed_apps.insert(name, binary);
    }

    /// Remove a TUI app
    pub fn remove(&mut self, name: &str) -> Result<(), String> {
        self.installed_apps
            .remove(name)
            .ok_or_else(|| format!("TUI app {} not found", name))?;
        Ok(())
    }

    /// List installed apps
    pub fn list_apps(&self) -> Vec<(&String, &String)> {
        self.installed_apps.iter().collect()
    }

    /// App count
    pub fn app_count(&self) -> usize {
        self.installed_apps.len()
    }
}

impl Default for TuiManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Miscellaneous tools manager
pub struct MiscToolsManager {
    pub drive_manager: DriveManager,
    pub tui_manager: TuiManager,
}

impl MiscToolsManager {
    pub fn new() -> Self {
        Self {
            drive_manager: DriveManager::new(),
            tui_manager: TuiManager::new(),
        }
    }
}

impl Default for MiscToolsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drive_manager() {
        let mut mgr = DriveManager::new();
        mgr.add_drive(DriveInfo {
            id: "root".to_string(),
            name: "Root".to_string(),
            device: "/dev/sda1".to_string(),
            total_bytes: 500_000_000_000,
            used_bytes: 250_000_000_000,
            available_bytes: 250_000_000_000,
            mount_point: "/".to_string(),
            filesystem: "ext4".to_string(),
        });
        assert_eq!(mgr.drive_count(), 1);
        assert!((mgr.get_usage_percent("root").unwrap() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_tui_manager() {
        let mut mgr = TuiManager::new();
        mgr.install("btop".to_string(), "/usr/bin/btop".to_string());
        mgr.install("lazygit".to_string(), "/usr/bin/lazygit".to_string());
        assert_eq!(mgr.app_count(), 2);
        assert!(mgr.remove("btop").is_ok());
        assert_eq!(mgr.app_count(), 1);
    }

    #[test]
    fn test_misc_tools_manager() {
        let mgr = MiscToolsManager::new();
        assert_eq!(mgr.drive_manager.drive_count(), 0);
        assert_eq!(mgr.tui_manager.app_count(), 0);
    }
}