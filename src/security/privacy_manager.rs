//! Privacy Manager
//!
//! Privacy management inspired by Linux Mint's privacy settings and Omarchy's
//! privacy utilities, supporting privacy controls, data collection, and user privacy.

use std::collections::HashMap;

/// Privacy level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyLevel {
    Minimal,
    Standard,
    High,
    Maximum,
}

impl PrivacyLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "minimal" => Some(PrivacyLevel::Minimal),
            "standard" => Some(PrivacyLevel::Standard),
            "high" => Some(PrivacyLevel::High),
            "maximum" => Some(PrivacyLevel::Maximum),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PrivacyLevel::Minimal => "Minimal",
            PrivacyLevel::Standard => "Standard",
            PrivacyLevel::High => "High",
            PrivacyLevel::Maximum => "Maximum",
        }
    }
}

/// Privacy setting
#[derive(Debug, Clone)]
pub struct PrivacySetting {
    pub id: String,
    pub name: String,
    pub description: String,
    pub is_enabled: bool,
    pub category: String,
}

impl PrivacySetting {
    pub fn new(id: String, name: String, description: String, category: String) -> Self {
        Self {
            id,
            name,
            description,
            is_enabled: true,
            category,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }
}

/// Privacy manager
#[derive(Debug)]
pub struct PrivacyManager {
    settings: HashMap<String, PrivacySetting>,
    privacy_level: PrivacyLevel,
}

impl PrivacyManager {
    pub fn new() -> Self {
        let mut manager = Self {
            settings: HashMap::new(),
            privacy_level: PrivacyLevel::Standard,
        };

        // Add default privacy settings
        manager.add_default_settings();

        manager
    }

    /// Add default settings
    fn add_default_settings(&mut self) {
        let settings = vec![
            ("telemetry", "Telemetry", "Send anonymous usage data", "Data Collection"),
            ("crash-reports", "Crash Reports", "Send crash reports to developers", "Data Collection"),
            ("location-services", "Location Services", "Allow applications to access location", "Permissions"),
            ("camera-access", "Camera Access", "Allow applications to access camera", "Permissions"),
            ("microphone-access", "Microphone Access", "Allow applications to access microphone", "Permissions"),
            ("desktop-notifications", "Desktop Notifications", "Allow desktop notifications", "Notifications"),
            ("background-activity", "Background Activity", "Allow applications to run in background", "Permissions"),
            ("search-history", "Search History", "Save search history", "Data Collection"),
            ("analytics", "Analytics", "Send analytics data", "Data Collection"),
        ];

        for (id, name, description, category) in settings {
            let setting = PrivacySetting::new(
                id.to_string(),
                name.to_string(),
                description.to_string(),
                category.to_string(),
            );
            self.settings.insert(id.to_string(), setting);
        }
    }

    /// Add a setting
    pub fn add_setting(&mut self, setting: PrivacySetting) {
        self.settings.insert(setting.id.clone(), setting);
    }

    /// Get a setting
    pub fn get_setting(&self, id: &str) -> Option<&PrivacySetting> {
        self.settings.get(id)
    }

    /// List all settings
    pub fn list_settings(&self) -> Vec<&PrivacySetting> {
        self.settings.values().collect()
    }

    /// List by category
    pub fn list_by_category(&self, category: &str) -> Vec<&PrivacySetting> {
        self.settings.values()
            .filter(|s| s.category == category)
            .collect()
    }

    /// Enable a setting
    pub fn enable(&mut self, id: &str) -> Result<(), String> {
        let setting = self.settings.get_mut(id)
            .ok_or_else(|| format!("Setting {} not found", id))?;

        setting.set_enabled(true);
        Ok(())
    }

    /// Disable a setting
    pub fn disable(&mut self, id: &str) -> Result<(), String> {
        let setting = self.settings.get_mut(id)
            .ok_or_else(|| format!("Setting {} not found", id))?;

        setting.set_enabled(false);
        Ok(())
    }

    /// Get privacy level
    pub fn get_privacy_level(&self) -> PrivacyLevel {
        self.privacy_level
    }

    /// Set privacy level
    pub fn set_privacy_level(&mut self, level: PrivacyLevel) {
        self.privacy_level = level;

        // Automatically adjust settings based on level
        match level {
            PrivacyLevel::Minimal => {
                // Enable most features
                for setting in self.settings.values_mut() {
                    setting.set_enabled(true);
                }
            }
            PrivacyLevel::Maximum => {
                // Disable most data collection
                for setting in self.settings.values_mut() {
                    if setting.category == "Data Collection" {
                        setting.set_enabled(false);
                    }
                }
            }
            _ => {}
        }
    }

    /// Get statistics
    pub fn get_statistics(&self) -> PrivacyStatistics {
        let total_settings = self.settings.len();
        let enabled_count = self.settings.values()
            .filter(|s| s.is_enabled)
            .count();
        let data_collection_count = self.settings.values()
            .filter(|s| s.category == "Data Collection" && s.is_enabled)
            .count();
        let permissions_count = self.settings.values()
            .filter(|s| s.category == "Permissions")
            .count();

        PrivacyStatistics {
            total_settings,
            enabled_count,
            data_collection_count,
            permissions_count,
            privacy_level: self.privacy_level,
        }
    }
}

impl Default for PrivacyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Privacy statistics
#[derive(Debug, Clone)]
pub struct PrivacyStatistics {
    pub total_settings: usize,
    pub enabled_count: usize,
    pub data_collection_count: usize,
    pub permissions_count: usize,
    pub privacy_level: PrivacyLevel,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_level_from_str() {
        assert_eq!(PrivacyLevel::from_str("standard"), Some(PrivacyLevel::Standard));
        assert_eq!(PrivacyLevel::from_str("maximum"), Some(PrivacyLevel::Maximum));
    }

    #[test]
    fn test_privacy_setting_creation() {
        let setting = PrivacySetting::new(
            "test".to_string(),
            "Test".to_string(),
            "Test setting".to_string(),
            "Test".to_string(),
        );
        assert_eq!(setting.name, "Test");
    }

    #[test]
    fn test_privacy_manager_creation() {
        let manager = PrivacyManager::new();
        assert!(manager.get_setting("telemetry").is_some());
    }

    #[test]
    fn test_add_setting() {
        let mut manager = PrivacyManager::new();
        let setting = PrivacySetting::new(
            "test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
        );
        manager.add_setting(setting);
        assert!(manager.get_setting("test").is_some());
    }

    #[test]
    fn test_enable_disable() {
        let mut manager = PrivacyManager::new();
        manager.disable("telemetry").ok();
        assert!(!manager.get_setting("telemetry").unwrap().is_enabled);
        manager.enable("telemetry").ok();
        assert!(manager.get_setting("telemetry").unwrap().is_enabled);
    }

    #[test]
    fn test_set_privacy_level() {
        let mut manager = PrivacyManager::new();
        manager.set_privacy_level(PrivacyLevel::Maximum);
        assert_eq!(manager.get_privacy_level(), PrivacyLevel::Maximum);
    }

    #[test]
    fn test_list_by_category() {
        let manager = PrivacyManager::new();
        let data = manager.list_by_category("Data Collection");
        assert!(data.len() > 0);
    }

    #[test]
    fn test_statistics() {
        let manager = PrivacyManager::new();
        let stats = manager.get_statistics();
        assert!(stats.total_settings >= 10);
    }
}
