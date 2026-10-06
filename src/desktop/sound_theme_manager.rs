// Desktop Sound Theme Manager
// Linux Mint & Omarchy inspiration for comprehensive sound theme management

use std::collections::HashMap;

/// Sound Event Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundEventType {
    Boot,
    Shutdown,
    Login,
    Logout,
    DesktopLogin,
    DesktopLogout,
    Notification,
    Message,
    Error,
    Warning,
    Success,
    BatteryLow,
    BatteryFull,
    VolumeChange,
    FileCopyComplete,
    FileMoveComplete,
    DeviceConnect,
    DeviceDisconnect,
}

impl SoundEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SoundEventType::Boot => "boot",
            SoundEventType::Shutdown => "shutdown",
            SoundEventType::Login => "login",
            SoundEventType::Logout => "logout",
            SoundEventType::DesktopLogin => "desktop-login",
            SoundEventType::DesktopLogout => "desktop-logout",
            SoundEventType::Notification => "notification",
            SoundEventType::Message => "message",
            SoundEventType::Error => "error",
            SoundEventType::Warning => "warning",
            SoundEventType::Success => "success",
            SoundEventType::BatteryLow => "battery-low",
            SoundEventType::BatteryFull => "battery-full",
            SoundEventType::VolumeChange => "volume-change",
            SoundEventType::FileCopyComplete => "file-copy-complete",
            SoundEventType::FileMoveComplete => "file-move-complete",
            SoundEventType::DeviceConnect => "device-connect",
            SoundEventType::DeviceDisconnect => "device-disconnect",
        }
    }
}

/// Desktop Sound Theme
#[derive(Debug, Clone)]
pub struct DesktopSoundTheme {
    pub id: String,
    pub name: String,
    pub author: String,
    pub sounds: HashMap<SoundEventType, String>,
}

impl DesktopSoundTheme {
    pub fn new(id: String, name: String, author: String) -> Self {
        Self {
            id,
            name,
            author,
            sounds: HashMap::new(),
        }
    }

    pub fn add_sound(&mut self, event: SoundEventType, file_path: String) {
        self.sounds.insert(event, file_path);
    }

    pub fn get_sound(&self, event: SoundEventType) -> Option<&String> {
        self.sounds.get(&event)
    }
}

/// Desktop Sound Theme Manager
pub struct DesktopSoundThemeManager {
    themes: HashMap<String, DesktopSoundTheme>,
    current_theme: Option<String>,
    volume: u32,  // 0-100
    counter: u32,
}

impl DesktopSoundThemeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            themes: HashMap::new(),
            current_theme: None,
            volume: 100,
            counter: 0,
        };

        // Add default theme
        manager.add_default_theme();

        manager
    }

    fn add_default_theme(&mut self) {
        let mut theme = DesktopSoundTheme::new(
            "theme_0".to_string(),
            "Default Sound Theme".to_string(),
            "SigmaOS".to_string(),
        );

        // Add default sounds
        theme.add_sound(SoundEventType::Boot, "/usr/share/sounds/sigmaos/boot.ogg".to_string());
        theme.add_sound(SoundEventType::Shutdown, "/usr/share/sounds/sigmaos/shutdown.ogg".to_string());
        theme.add_sound(SoundEventType::Notification, "/usr/share/sounds/sigmaos/notification.ogg".to_string());
        theme.add_sound(SoundEventType::Message, "/usr/share/sounds/sigmaos/message.ogg".to_string());
        theme.add_sound(SoundEventType::Error, "/usr/share/sounds/sigmaos/error.ogg".to_string());
        theme.add_sound(SoundEventType::Warning, "/usr/share/sounds/sigmaos/warning.ogg".to_string());
        theme.add_sound(SoundEventType::Success, "/usr/share/sounds/sigmaos/success.ogg".to_string());
        theme.add_sound(SoundEventType::BatteryLow, "/usr/share/sounds/sigmaos/battery-low.ogg".to_string());
        theme.add_sound(SoundEventType::DeviceConnect, "/usr/share/sounds/sigmaos/device-connect.ogg".to_string());
        theme.add_sound(SoundEventType::DeviceDisconnect, "/usr/share/sounds/sigmaos/device-disconnect.ogg".to_string());

        let theme_id = theme.id.clone();
        self.themes.insert(theme_id.clone(), theme);
        self.current_theme = Some(theme_id);
    }

    pub fn add_theme(&mut self, theme: DesktopSoundTheme) -> String {
        let id = format!("theme_{}", self.counter);
        self.counter += 1;

        let theme = DesktopSoundTheme {
            id: id.clone(),
            ..theme
        };

        self.themes.insert(id.clone(), theme);
        id
    }

    pub fn remove_theme(&mut self, id: &str) -> bool {
        if Some(id.to_string()) == self.current_theme {
            return false;
        }
        self.themes.remove(id).is_some()
    }

    pub fn get_theme(&self, id: &str) -> Option<&DesktopSoundTheme> {
        self.themes.get(id)
    }

    pub fn get_themes(&self) -> Vec<&DesktopSoundTheme> {
        self.themes.values().collect()
    }

    pub fn set_current_theme(&mut self, id: &str) -> bool {
        if self.themes.contains_key(id) {
            self.current_theme = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_theme(&self) -> Option<&DesktopSoundTheme> {
        self.current_theme
            .as_ref()
            .and_then(|id| self.themes.get(id))
    }

    pub fn play_sound(&self, event: SoundEventType) -> Option<&String> {
        if let Some(theme) = self.get_current_theme() {
            theme.get_sound(event)
        } else {
            None
        }
    }

    pub fn set_volume(&mut self, volume: u32) {
        self.volume = volume.min(100);
    }

    pub fn get_volume(&self) -> u32 {
        self.volume
    }

    pub fn add_sound_to_theme(&mut self, theme_id: &str, event: SoundEventType, file_path: String) -> bool {
        if let Some(theme) = self.themes.get_mut(theme_id) {
            theme.add_sound(event, file_path);
            true
        } else {
            false
        }
    }

    pub fn remove_sound_from_theme(&mut self, theme_id: &str, event: SoundEventType) -> bool {
        if let Some(theme) = self.themes.get_mut(theme_id) {
            theme.sounds.remove(&event).is_some()
        } else {
            false
        }
    }

    pub fn get_statistics(&self) -> DesktopSoundThemeManagerStatistics {
        DesktopSoundThemeManagerStatistics {
            total_themes: self.themes.len(),
            current_theme_set: self.current_theme.is_some(),
            volume: self.volume,
        }
    }
}

impl Default for DesktopSoundThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// DesktopSoundThemeManagerStatistics
#[derive(Debug, Clone, Copy)]
pub struct DesktopSoundThemeManagerStatistics {
    pub total_themes: usize,
    pub current_theme_set: bool,
    pub volume: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_manager_state() {
        let manager = DesktopSoundThemeManager::new();
        let stats = manager.get_statistics();

        assert_eq!(stats.total_themes, 1);
        assert!(stats.current_theme_set);
        assert_eq!(stats.volume, 100);
    }

    #[test]
    fn test_add_theme() {
        let mut manager = DesktopSoundThemeManager::new();
        let initial_count = manager.get_themes().len();

        let theme = DesktopSoundTheme::new(
            "custom".to_string(),
            "Custom Theme".to_string(),
            "Author".to_string(),
        );

        let id = manager.add_theme(theme);
        assert!(manager.get_theme(&id).is_some());
        assert_eq!(manager.get_themes().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_theme() {
        let mut manager = DesktopSoundThemeManager::new();

        let theme = DesktopSoundTheme::new(
            "custom".to_string(),
            "Custom Theme".to_string(),
            "Author".to_string(),
        );

        let id = manager.add_theme(theme);
        assert!(manager.remove_theme(&id));
        assert!(manager.get_theme(&id).is_none());
    }

    #[test]
    fn test_remove_current_theme() {
        let mut manager = DesktopSoundThemeManager::new();

        // Try to remove current theme (should fail)
        let current_id = manager.get_current_theme().unwrap().id.clone();
        let result = manager.remove_theme(&current_id);
        assert!(!result);
    }

    #[test]
    fn test_set_current_theme() {
        let mut manager = DesktopSoundThemeManager::new();

        let theme = DesktopSoundTheme::new(
            "custom".to_string(),
            "Custom Theme".to_string(),
            "Author".to_string(),
        );

        let id = manager.add_theme(theme);
        assert!(manager.set_current_theme(&id));

        let current = manager.get_current_theme().unwrap();
        assert_eq!(current.id, id);
    }

    #[test]
    fn test_play_sound() {
        let manager = DesktopSoundThemeManager::new();

        let sound = manager.play_sound(SoundEventType::Notification);
        assert!(sound.is_some());
        assert!(sound.unwrap().contains("notification"));

        let no_sound = manager.play_sound(SoundEventType::FileCopyComplete);
        assert!(no_sound.is_none());
    }

    #[test]
    fn test_volume() {
        let mut manager = DesktopSoundThemeManager::new();

        manager.set_volume(75);
        assert_eq!(manager.get_volume(), 75);

        manager.set_volume(150);
        assert_eq!(manager.get_volume(), 100);
    }

    #[test]
    fn test_add_sound_to_theme() {
        let mut manager = DesktopSoundThemeManager::new();
        let theme_id = "theme_0";

        assert!(manager.add_sound_to_theme(
            theme_id,
            SoundEventType::Message,
            "/custom/message.ogg".to_string(),
        ));

        let theme = manager.get_theme(theme_id).unwrap();
        assert_eq!(theme.get_sound(SoundEventType::Message), Some(&"/custom/message.ogg".to_string()));
    }

    #[test]
    fn test_remove_sound_from_theme() {
        let mut manager = DesktopSoundThemeManager::new();
        let theme_id = "theme_0";

        assert!(manager.remove_sound_from_theme(theme_id, SoundEventType::Notification));

        let theme = manager.get_theme(theme_id).unwrap();
        assert!(theme.get_sound(SoundEventType::Notification).is_none());
    }
}
