// SigmaOS Desktop Environment Manager
// Inspired by Linux Mint's desktop environment settings and Omarchy's DE utilities

use std::collections::HashMap;

/// Desktop environment type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopEnvironment {
    Zenith,     // SigmaOS native Wayland compositor
    GNOME,      // GNOME
    KDE,        // KDE Plasma
    XFCE,       // XFCE
    MATE,       // MATE
    Cinnamon,   // Cinnamon (Linux Mint)
    LXQt,       // LXQt
    Pantheon,   // Pantheon (elementary OS)
    TTY,        // Text terminal
}

impl DesktopEnvironment {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopEnvironment::Zenith => "Zenith",
            DesktopEnvironment::GNOME => "GNOME",
            DesktopEnvironment::KDE => "KDE Plasma",
            DesktopEnvironment::XFCE => "XFCE",
            DesktopEnvironment::MATE => "MATE",
            DesktopEnvironment::Cinnamon => "Cinnamon",
            DesktopEnvironment::LXQt => "LXQt",
            DesktopEnvironment::Pantheon => "Pantheon",
            DesktopEnvironment::TTY => "TTY",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "zenith" => Some(DesktopEnvironment::Zenith),
            "gnome" => Some(DesktopEnvironment::GNOME),
            "kde" | "plasma" => Some(DesktopEnvironment::KDE),
            "xfce" => Some(DesktopEnvironment::XFCE),
            "mate" => Some(DesktopEnvironment::MATE),
            "cinnamon" => Some(DesktopEnvironment::Cinnamon),
            "lxqt" => Some(DesktopEnvironment::LXQt),
            "pantheon" => Some(DesktopEnvironment::Pantheon),
            "tty" => Some(DesktopEnvironment::TTY),
            _ => None,
        }
    }
}

/// Session type for DE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopDESessionType {
    Wayland,
    X11,
    TTY,
}

impl DesktopDESessionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopDESessionType::Wayland => "Wayland",
            DesktopDESessionType::X11 => "X11",
            DesktopDESessionType::TTY => "TTY",
        }
    }
}

/// Desktop environment configuration
#[derive(Debug, Clone)]
pub struct DEConfig {
    pub de_type: DesktopEnvironment,
    pub session_type: DesktopDESessionType,
    pub command: String,
    pub display_server: String,
    pub is_available: bool,
    pub is_default: bool,
}

impl DEConfig {
    pub fn new(
        de_type: DesktopEnvironment,
        session_type: DesktopDESessionType,
        command: String,
        display_server: String,
    ) -> Self {
        DEConfig {
            de_type,
            session_type,
            command,
            display_server,
            is_available: true,
            is_default: false,
        }
    }

    pub fn set_available(&mut self, available: bool) {
        self.is_available = available;
    }

    pub fn set_default(&mut self, default: bool) {
        self.is_default = default;
    }
}

/// Desktop Environment Manager
pub struct DEManager {
    configs: HashMap<String, DEConfig>,
    default_de: Option<String>,
    installed_des: Vec<String>,
}

impl DEManager {
    pub fn new() -> Self {
        let mut manager = DEManager {
            configs: HashMap::new(),
            default_de: None,
            installed_des: Vec::new(),
        };

        // Add default DE configurations
        manager.add_default_configs();

        manager
    }

    fn add_default_configs(&mut self) {
        // Zenith (SigmaOS native)
        let mut zenith = DEConfig::new(
            DesktopEnvironment::Zenith,
            DesktopDESessionType::Wayland,
            "sigma-wm".to_string(),
            "zenith".to_string(),
        );
        zenith.set_default(true);
        let zenith_id = "zenith-wayland".to_string();
        self.configs.insert(zenith_id.clone(), zenith);
        self.default_de = Some(zenith_id.clone());
        self.installed_des.push(zenith_id);

        // GNOME Wayland
        let gnome = DEConfig::new(
            DesktopEnvironment::GNOME,
            DesktopDESessionType::Wayland,
            "gnome-shell".to_string(),
            "gnome-shell".to_string(),
        );
        let gnome_id = "gnome-wayland".to_string();
        self.configs.insert(gnome_id.clone(), gnome);
        self.installed_des.push(gnome_id);

        // GNOME X11
        let gnome_x11 = DEConfig::new(
            DesktopEnvironment::GNOME,
            DesktopDESessionType::X11,
            "gnome-session".to_string(),
            "Xorg".to_string(),
        );
        let gnome_x11_id = "gnome-x11".to_string();
        self.configs.insert(gnome_x11_id.clone(), gnome_x11);

        // KDE Plasma Wayland
        let kde = DEConfig::new(
            DesktopEnvironment::KDE,
            DesktopDESessionType::Wayland,
            "startplasma-wayland".to_string(),
            "plasma-wayland".to_string(),
        );
        let kde_id = "kde-wayland".to_string();
        self.configs.insert(kde_id.clone(), kde);
        self.installed_des.push(kde_id);

        // XFCE X11
        let xfce = DEConfig::new(
            DesktopEnvironment::XFCE,
            DesktopDESessionType::X11,
            "startxfce4".to_string(),
            "Xorg".to_string(),
        );
        let xfce_id = "xfce-x11".to_string();
        self.configs.insert(xfce_id.clone(), xfce);
        self.installed_des.push(xfce_id);

        // Cinnamon X11
        let cinnamon = DEConfig::new(
            DesktopEnvironment::Cinnamon,
            DesktopDESessionType::X11,
            "cinnamon-session".to_string(),
            "Xorg".to_string(),
        );
        let cinnamon_id = "cinnamon-x11".to_string();
        self.configs.insert(cinnamon_id.clone(), cinnamon);
        self.installed_des.push(cinnamon_id);
    }

    pub fn add_config(&mut self, id: String, config: DEConfig) -> bool {
        if self.configs.contains_key(&id) {
            return false;
        }
        self.configs.insert(id.clone(), config);
        true
    }

    pub fn remove_config(&mut self, id: &str) -> bool {
        if let Some(config) = self.configs.get(id) {
            if config.is_default {
                return false; // Cannot remove default DE
            }
        }
        self.configs.remove(id).is_some()
    }

    pub fn get_config(&self, id: &str) -> Option<&DEConfig> {
        self.configs.get(id)
    }

    pub fn get_configs(&self) -> Vec<&DEConfig> {
        self.configs.values().collect()
    }

    pub fn get_configs_by_de(&self, de_type: DesktopEnvironment) -> Vec<&DEConfig> {
        self.configs
            .values()
            .filter(|c| c.de_type == de_type)
            .collect()
    }

    pub fn get_configs_by_session_type(&self, session_type: DesktopDESessionType) -> Vec<&DEConfig> {
        self.configs
            .values()
            .filter(|c| c.session_type == session_type)
            .collect()
    }

    pub fn get_default_de(&self) -> Option<&DEConfig> {
        if let Some(default_id) = &self.default_de {
            self.configs.get(default_id)
        } else {
            None
        }
    }

    pub fn set_default_de(&mut self, id: &str) -> bool {
        if !self.configs.contains_key(id) {
            return false;
        }

        // Clear default from all configs
        for config in self.configs.values_mut() {
            config.set_default(false);
        }

        // Set new default
        if let Some(config) = self.configs.get_mut(id) {
            config.set_default(true);
            self.default_de = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn set_de_available(&mut self, id: &str, available: bool) -> bool {
        if let Some(config) = self.configs.get_mut(id) {
            config.set_available(available);
            true
        } else {
            false
        }
    }

    pub fn mark_installed(&mut self, id: &str) -> bool {
        if !self.configs.contains_key(id) {
            return false;
        }

        if !self.installed_des.contains(&id.to_string()) {
            self.installed_des.push(id.to_string());
        }

        if let Some(config) = self.configs.get_mut(id) {
            config.set_available(true);
        }

        true
    }

    pub fn mark_uninstalled(&mut self, id: &str) -> bool {
        self.installed_des.retain(|i| i != id);

        if let Some(config) = self.configs.get_mut(id) {
            config.set_available(false);
            true
        } else {
            false
        }
    }

    pub fn get_installed_des(&self) -> Vec<&DEConfig> {
        self.installed_des
            .iter()
            .filter_map(|id| self.configs.get(id))
            .collect()
    }

    pub fn get_available_des(&self) -> Vec<&DEConfig> {
        self.configs
            .values()
            .filter(|c| c.is_available)
            .collect()
    }

    pub fn get_statistics(&self) -> DEStatistics {
        DEStatistics {
            total_configs: self.configs.len(),
            installed_count: self.installed_des.len(),
            available_count: self.get_available_des().len(),
            wayland_count: self.get_configs_by_session_type(DesktopDESessionType::Wayland).len(),
            x11_count: self.get_configs_by_session_type(DesktopDESessionType::X11).len(),
        }
    }
}

impl Default for DEManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Desktop Environment statistics
#[derive(Debug, Clone, Copy)]
pub struct DEStatistics {
    pub total_configs: usize,
    pub installed_count: usize,
    pub available_count: usize,
    pub wayland_count: usize,
    pub x11_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_from_str() {
        assert_eq!(DesktopEnvironment::from_str("zenith"), Some(DesktopEnvironment::Zenith));
        assert_eq!(DesktopEnvironment::from_str("gnome"), Some(DesktopEnvironment::GNOME));
        assert_eq!(DesktopEnvironment::from_str("kde"), Some(DesktopEnvironment::KDE));
        assert_eq!(DesktopEnvironment::from_str("invalid"), None);
    }

    #[test]
    fn test_de_manager_initialization() {
        let manager = DEManager::new();
        assert_eq!(manager.get_configs().len(), 6);
        assert!(manager.get_default_de().is_some());
        assert_eq!(manager.get_installed_des().len(), 4);
    }

    #[test]
    fn test_add_config() {
        let mut manager = DEManager::new();
        let config = DEConfig::new(
            DesktopEnvironment::MATE,
            DesktopDESessionType::X11,
            "mate-session".to_string(),
            "Xorg".to_string(),
        );
        assert!(manager.add_config("mate-x11".to_string(), config));
        assert_eq!(manager.get_configs().len(), 7);
    }

    #[test]
    fn test_remove_config() {
        let mut manager = DEManager::new();
        let config = DEConfig::new(
            DesktopEnvironment::MATE,
            DesktopDESessionType::X11,
            "mate-session".to_string(),
            "Xorg".to_string(),
        );
        manager.add_config("mate-x11".to_string(), config);
        assert!(manager.remove_config("mate-x11"));
        assert_eq!(manager.get_configs().len(), 6);
    }

    #[test]
    fn test_set_default_de() {
        let mut manager = DEManager::new();
        assert!(manager.set_default_de("gnome-wayland"));
        assert_eq!(manager.get_default_de().unwrap().de_type, DesktopEnvironment::GNOME);
    }

    #[test]
    fn test_mark_installed() {
        let mut manager = DEManager::new();
        assert!(manager.mark_installed("gnome-x11"));
        assert_eq!(manager.get_installed_des().len(), 5);
    }

    #[test]
    fn test_mark_uninstalled() {
        let mut manager = DEManager::new();
        assert!(manager.mark_uninstalled("gnome-wayland"));
        assert_eq!(manager.get_installed_des().len(), 3);
    }

    #[test]
    fn test_get_configs_by_de() {
        let manager = DEManager::new();
        let gnome_configs = manager.get_configs_by_de(DesktopEnvironment::GNOME);
        assert_eq!(gnome_configs.len(), 2);
    }

    #[test]
    fn test_get_configs_by_session_type() {
        let manager = DEManager::new();
        let wayland_configs = manager.get_configs_by_session_type(DesktopDESessionType::Wayland);
        assert_eq!(wayland_configs.len(), 2);
    }

    #[test]
    fn test_statistics() {
        let manager = DEManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.total_configs, 6);
        assert_eq!(stats.installed_count, 4);
    }
}
