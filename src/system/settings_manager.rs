//! Settings Manager
//!
//! System settings management inspired by Linux Mint's settings and Omarchy's
//! configuration utilities, supporting system-wide and user settings.

use std::collections::HashMap;

/// Setting value type
#[derive(Debug, Clone, PartialEq)]
pub enum SettingValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    Float(f64),
    StringList(Vec<String>),
}

impl SettingValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            SettingValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            SettingValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            SettingValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            SettingValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_string_list(&self) -> Option<&Vec<String>> {
        match self {
            SettingValue::StringList(list) => Some(list),
            _ => None,
        }
    }
}

/// Setting category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettingCategory {
    Appearance,
    Display,
    Sound,
    Network,
    Power,
    Privacy,
    Accessibility,
    Input,
    System,
}

impl SettingCategory {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "appearance" => Some(SettingCategory::Appearance),
            "display" => Some(SettingCategory::Display),
            "sound" => Some(SettingCategory::Sound),
            "network" => Some(SettingCategory::Network),
            "power" => Some(SettingCategory::Power),
            "privacy" => Some(SettingCategory::Privacy),
            "accessibility" => Some(SettingCategory::Accessibility),
            "input" => Some(SettingCategory::Input),
            "system" => Some(SettingCategory::System),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SettingCategory::Appearance => "Appearance",
            SettingCategory::Display => "Display",
            SettingCategory::Sound => "Sound",
            SettingCategory::Network => "Network",
            SettingCategory::Power => "Power",
            SettingCategory::Privacy => "Privacy",
            SettingCategory::Accessibility => "Accessibility",
            SettingCategory::Input => "Input",
            SettingCategory::System => "System",
        }
    }
}

/// Setting entry
#[derive(Debug, Clone)]
pub struct SettingEntry {
    pub key: String,
    pub value: SettingValue,
    pub category: SettingCategory,
    pub description: String,
    pub is_readonly: bool,
}

impl SettingEntry {
    pub fn new(key: String, value: SettingValue, category: SettingCategory, description: String) -> Self {
        Self {
            key,
            value,
            category,
            description,
            is_readonly: false,
        }
    }

    pub fn set_readonly(&mut self, readonly: bool) {
        self.is_readonly = readonly;
    }

    pub fn update_value(&mut self, value: SettingValue) -> Result<(), String> {
        if self.is_readonly {
            return Err(format!("Setting {} is read-only", self.key));
        }
        self.value = value;
        Ok(())
    }
}

/// Settings manager
#[derive(Debug)]
pub struct SettingsManager {
    settings: HashMap<String, SettingEntry>,
}

impl SettingsManager {
    pub fn new() -> Self {
        let mut manager = Self {
            settings: HashMap::new(),
        };

        manager.add_default_settings();
        manager
    }

    /// Add default system settings
    fn add_default_settings(&mut self) {
        let defaults = vec![
            // Appearance
            ("theme", SettingValue::String("Sigma Dark".to_string()), SettingCategory::Appearance, "Current theme"),
            ("accent_color", SettingValue::String("Blue".to_string()), SettingCategory::Appearance, "Accent color"),
            ("font_size", SettingValue::Integer(14), SettingCategory::Appearance, "Default font size"),
            
            // Display
            ("brightness", SettingValue::Integer(100), SettingCategory::Display, "Screen brightness"),
            ("night_light", SettingValue::Boolean(false), SettingCategory::Display, "Night light enabled"),
            
            // Sound
            ("volume", SettingValue::Integer(75), SettingCategory::Sound, "Master volume"),
            ("mute", SettingValue::Boolean(false), SettingCategory::Sound, "Mute all sounds"),
            
            // Network
            ("auto_connect", SettingValue::Boolean(true), SettingCategory::Network, "Auto-connect to known networks"),
            ("metered_warning", SettingValue::Boolean(true), SettingCategory::Network, "Warn on metered connections"),
            
            // Power
            ("power_profile", SettingValue::String("Balanced".to_string()), SettingCategory::Power, "Power profile"),
            ("auto_sleep", SettingValue::Boolean(true), SettingCategory::Power, "Auto-sleep enabled"),
            ("sleep_timeout", SettingValue::Integer(30), SettingCategory::Power, "Sleep timeout (minutes)"),
            
            // Privacy
            ("collect_analytics", SettingValue::Boolean(false), SettingCategory::Privacy, "Collect analytics"),
            ("location_services", SettingValue::Boolean(false), SettingCategory::Privacy, "Location services"),
            
            // Input
            ("keyboard_layout", SettingValue::String("us".to_string()), SettingCategory::Input, "Keyboard layout"),
            ("mouse_acceleration", SettingValue::Boolean(true), SettingCategory::Input, "Mouse acceleration"),
            
            // System
            ("timezone", SettingValue::String("UTC".to_string()), SettingCategory::System, "System timezone"),
            ("language", SettingValue::String("en_US".to_string()), SettingCategory::System, "System language"),
        ];

        for (key, value, category, description) in defaults {
            let mut entry = SettingEntry::new(
                key.to_string(),
                value,
                category,
                description.to_string(),
            );
            
            // Mark some settings as readonly
            if key == "timezone" || key == "language" {
                entry.set_readonly(true);
            }
            
            self.settings.insert(key.to_string(), entry);
        }
    }

    /// Get a setting
    pub fn get(&self, key: &str) -> Option<&SettingEntry> {
        self.settings.get(key)
    }

    /// Get a setting mutably
    pub fn get_mut(&mut self, key: &str) -> Option<&mut SettingEntry> {
        self.settings.get_mut(key)
    }

    /// Set a setting value
    pub fn set(&mut self, key: &str, value: SettingValue) -> Result<(), String> {
        let entry = self.settings.get_mut(key)
            .ok_or_else(|| format!("Setting {} not found", key))?;

        entry.update_value(value)
    }

    /// Get setting value
    pub fn get_value(&self, key: &str) -> Option<&SettingValue> {
        self.settings.get(key).map(|e| &e.value)
    }

    /// List all settings
    pub fn list_all(&self) -> Vec<&SettingEntry> {
        self.settings.values().collect()
    }

    /// List settings by category
    pub fn list_by_category(&self, category: SettingCategory) -> Vec<&SettingEntry> {
        self.settings.values()
            .filter(|s| s.category == category)
            .collect()
    }

    /// Search settings by key
    pub fn search(&self, query: &str) -> Vec<&SettingEntry> {
        let query_lower = query.to_lowercase();
        self.settings.values()
            .filter(|s| s.key.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Reset a setting to default
    pub fn reset(&mut self, key: &str) -> Result<(), String> {
        let entry = self.settings.get_mut(key)
            .ok_or_else(|| format!("Setting {} not found", key))?;

        if entry.is_readonly {
            return Err(format!("Cannot reset read-only setting {}", key));
        }

        // Reset to default (simplified - in real implementation would store defaults)
        match entry.key.as_str() {
            "theme" => entry.value = SettingValue::String("Sigma Dark".to_string()),
            "volume" => entry.value = SettingValue::Integer(75),
            "mute" => entry.value = SettingValue::Boolean(false),
            _ => {}
        }

        Ok(())
    }

    /// Reset all settings in a category
    pub fn reset_category(&mut self, category: SettingCategory) -> usize {
        let keys: Vec<String> = self.settings.values()
            .filter(|s| s.category == category && !s.is_readonly)
            .map(|s| s.key.clone())
            .collect();

        let mut count = 0;
        for key in keys {
            if self.reset(&key).is_ok() {
                count += 1;
            }
        }

        count
    }

    /// Export settings
    pub fn export(&self) -> HashMap<String, SettingValue> {
        self.settings.values()
            .map(|s| (s.key.clone(), s.value.clone()))
            .collect()
    }

    /// Import settings
    pub fn import(&mut self, settings: HashMap<String, SettingValue>) -> Result<(), String> {
        for (key, value) in settings {
            if let Some(entry) = self.settings.get_mut(&key) {
                if !entry.is_readonly {
                    entry.value = value;
                }
            }
        }
        Ok(())
    }

    /// Get statistics
    pub fn get_statistics(&self) -> SettingsStatistics {
        let total_settings = self.settings.len();
        let readonly_count = self.settings.values().filter(|s| s.is_readonly).count();
        let mut by_category = HashMap::new();

        for entry in self.settings.values() {
            *by_category.entry(entry.category).or_insert(0) += 1;
        }

        SettingsStatistics {
            total_settings,
            readonly_count,
            by_category,
        }
    }
}

impl Default for SettingsManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Settings statistics
#[derive(Debug, Clone)]
pub struct SettingsStatistics {
    pub total_settings: usize,
    pub readonly_count: usize,
    pub by_category: HashMap<SettingCategory, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setting_value() {
        let val = SettingValue::String("test".to_string());
        assert_eq!(val.as_string(), Some("test"));
    }

    #[test]
    fn test_setting_category_from_str() {
        assert_eq!(SettingCategory::from_str("appearance"), Some(SettingCategory::Appearance));
        assert_eq!(SettingCategory::from_str("network"), Some(SettingCategory::Network));
    }

    #[test]
    fn test_setting_entry_creation() {
        let entry = SettingEntry::new(
            "test".to_string(),
            SettingValue::Boolean(true),
            SettingCategory::System,
            "Test setting".to_string(),
        );
        assert_eq!(entry.key, "test");
    }

    #[test]
    fn test_settings_manager_creation() {
        let manager = SettingsManager::new();
        assert!(manager.list_all().len() >= 15);
    }

    #[test]
    fn test_get_setting() {
        let manager = SettingsManager::new();
        assert!(manager.get("theme").is_some());
    }

    #[test]
    fn test_set_setting() {
        let mut manager = SettingsManager::new();
        assert!(manager.set("theme", SettingValue::String("Sigma Light".to_string())).is_ok());
    }

    #[test]
    fn test_set_readonly_fails() {
        let mut manager = SettingsManager::new();
        assert!(manager.set("timezone", SettingValue::String("EST".to_string())).is_err());
    }

    #[test]
    fn test_list_by_category() {
        let manager = SettingsManager::new();
        let appearance = manager.list_by_category(SettingCategory::Appearance);
        assert!(appearance.len() >= 2);
    }

    #[test]
    fn test_search() {
        let manager = SettingsManager::new();
        let results = manager.search("theme");
        assert!(results.len() >= 1);
    }

    #[test]
    fn test_reset() {
        let mut manager = SettingsManager::new();
        manager.set("theme", SettingValue::String("Custom".to_string())).ok();
        assert!(manager.reset("theme").is_ok());
    }

    #[test]
    fn test_export_import() {
        let mut manager = SettingsManager::new();
        let exported = manager.export();
        let mut manager2 = SettingsManager::new();
        assert!(manager2.import(exported).is_ok());
    }

    #[test]
    fn test_statistics() {
        let manager = SettingsManager::new();
        let stats = manager.get_statistics();
        assert!(stats.total_settings >= 15);
    }
}
