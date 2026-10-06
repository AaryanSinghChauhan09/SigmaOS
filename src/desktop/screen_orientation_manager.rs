// SigmaOS Desktop Screen Orientation Manager
// Inspired by Linux Mint's display rotation and Omarchy's orientation utilities

use std::collections::HashMap;

/// Screen orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenOrientation {
    Normal,
    PortraitLeft,
    PortraitRight,
    Inverted,
}

impl ScreenOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScreenOrientation::Normal => "Normal",
            ScreenOrientation::PortraitLeft => "Portrait Left",
            ScreenOrientation::PortraitRight => "Portrait Right",
            ScreenOrientation::Inverted => "Inverted",
        }
    }

    pub fn rotation_degrees(&self) -> u32 {
        match self {
            ScreenOrientation::Normal => 0,
            ScreenOrientation::PortraitLeft => 90,
            ScreenOrientation::PortraitRight => 270,
            ScreenOrientation::Inverted => 180,
        }
    }
}

/// Screen orientation policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrientationPolicy {
    Manual,
    Auto,
    Lock,
}

impl OrientationPolicy {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrientationPolicy::Manual => "Manual",
            OrientationPolicy::Auto => "Auto",
            OrientationPolicy::Lock => "Lock",
        }
    }
}

/// Screen orientation configuration
#[derive(Debug, Clone)]
pub struct ScreenOrientationConfig {
    pub id: String,
    pub display_id: String,
    pub orientation: ScreenOrientation,
    pub policy: OrientationPolicy,
}

impl ScreenOrientationConfig {
    pub fn new(
        id: String,
        display_id: String,
        orientation: ScreenOrientation,
        policy: OrientationPolicy,
    ) -> Self {
        ScreenOrientationConfig {
            id,
            display_id,
            orientation,
            policy,
        }
    }

    pub fn set_orientation(&mut self, orientation: ScreenOrientation) {
        self.orientation = orientation;
    }

    pub fn set_policy(&mut self, policy: OrientationPolicy) {
        self.policy = policy;
    }
}

/// Desktop Screen Orientation Manager
pub struct DesktopScreenOrientationManager {
    configs: HashMap<String, ScreenOrientationConfig>,
    next_config_id: u32,
}

impl DesktopScreenOrientationManager {
    pub fn new() -> Self {
        let mut manager = DesktopScreenOrientationManager {
            configs: HashMap::new(),
            next_config_id: 1,
        };

        manager.add_default_config();
        manager
    }

    fn add_default_config(&mut self) {
        let id = format!("config_{}", self.next_config_id);
        self.next_config_id += 1;

        let config = ScreenOrientationConfig::new(
            id.clone(),
            "eDP-1".to_string(),
            ScreenOrientation::Normal,
            OrientationPolicy::Manual,
        );
        self.configs.insert(id, config);
    }

    pub fn add_config(
        &mut self,
        display_id: String,
        orientation: ScreenOrientation,
        policy: OrientationPolicy,
    ) -> String {
        let id = format!("config_{}", self.next_config_id);
        self.next_config_id += 1;

        let config = ScreenOrientationConfig::new(id.clone(), display_id, orientation, policy);
        self.configs.insert(id.clone(), config);
        id
    }

    pub fn remove_config(&mut self, id: &str) -> bool {
        self.configs.remove(id).is_some()
    }

    pub fn get_config(&self, id: &str) -> Option<&ScreenOrientationConfig> {
        self.configs.get(id)
    }

    pub fn get_configs(&self) -> Vec<&ScreenOrientationConfig> {
        self.configs.values().collect()
    }

    pub fn get_config_by_display(&self, display_id: &str) -> Option<&ScreenOrientationConfig> {
        self.configs.values().find(|c| c.display_id == display_id)
    }

    pub fn set_config_orientation(&mut self, id: &str, orientation: ScreenOrientation) -> bool {
        if let Some(config) = self.configs.get_mut(id) {
            config.set_orientation(orientation);
            true
        } else {
            false
        }
    }

    pub fn set_config_policy(&mut self, id: &str, policy: OrientationPolicy) -> bool {
        if let Some(config) = self.configs.get_mut(id) {
            config.set_policy(policy);
            true
        } else {
            false
        }
    }

    pub fn get_statistics(&self) -> ScreenOrientationStatistics {
        ScreenOrientationStatistics {
            total_configs: self.configs.len(),
            auto_orientation: self.configs.values().filter(|c| c.policy == OrientationPolicy::Auto).count(),
        }
    }
}

impl Default for DesktopScreenOrientationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// ScreenOrientationStatistics
#[derive(Debug, Clone, Copy)]
pub struct ScreenOrientationStatistics {
    pub total_configs: usize,
    pub auto_orientation: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_orientation_manager_initialization() {
        let manager = DesktopScreenOrientationManager::new();
        assert_eq!(manager.get_configs().len(), 1);
    }

    #[test]
    fn test_add_config() {
        let mut manager = DesktopScreenOrientationManager::new();
        let id = manager.add_config(
            "HDMI-1".to_string(),
            ScreenOrientation::Normal,
            OrientationPolicy::Auto,
        );
        assert!(manager.get_config(&id).is_some());
        assert_eq!(manager.get_configs().len(), 2);
    }

    #[test]
    fn test_remove_config() {
        let mut manager = DesktopScreenOrientationManager::new();
        let id = manager.add_config(
            "HDMI-1".to_string(),
            ScreenOrientation::Normal,
            OrientationPolicy::Manual,
        );
        assert!(manager.remove_config(&id));
        assert_eq!(manager.get_configs().len(), 1);
    }

    #[test]
    fn test_set_config_orientation() {
        let mut manager = DesktopScreenOrientationManager::new();
        let configs = manager.get_configs();
        if let Some(config) = configs.first() {
            let config_id = config.id.clone();
            assert!(manager.set_config_orientation(&config_id, ScreenOrientation::PortraitLeft));
            assert_eq!(manager.get_config(&config_id).unwrap().orientation, ScreenOrientation::PortraitLeft);
        }
    }

    #[test]
    fn test_set_config_policy() {
        let mut manager = DesktopScreenOrientationManager::new();
        let configs = manager.get_configs();
        if let Some(config) = configs.first() {
            let config_id = config.id.clone();
            assert!(manager.set_config_policy(&config_id, OrientationPolicy::Auto));
            assert_eq!(manager.get_config(&config_id).unwrap().policy, OrientationPolicy::Auto);
        }
    }

    #[test]
    fn test_get_config_by_display() {
        let manager = DesktopScreenOrientationManager::new();
        let config = manager.get_config_by_display("eDP-1");
        assert!(config.is_some());
    }

    #[test]
    fn test_rotation_degrees() {
        assert_eq!(ScreenOrientation::Normal.rotation_degrees(), 0);
        assert_eq!(ScreenOrientation::PortraitLeft.rotation_degrees(), 90);
        assert_eq!(ScreenOrientation::Inverted.rotation_degrees(), 180);
    }

    #[test]
    fn test_statistics() {
        let manager = DesktopScreenOrientationManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.total_configs, 1);
    }
}
