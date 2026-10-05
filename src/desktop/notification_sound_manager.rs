// SigmaOS Desktop Notification Sound Manager
// Inspired by Linux Mint's notification sounds and Omarchy's sound feedback

use std::collections::HashMap;

/// Notification sound type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationSoundType {
    Default,
    Minimal,
    None,
}

impl NotificationSoundType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationSoundType::Default => "Default",
            NotificationSoundType::Minimal => "Minimal",
            NotificationSoundType::None => "None",
        }
    }
}

/// Sound event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundEvent {
    Notification,
    Message,
    Email,
    Error,
    Warning,
    Success,
    Information,
    Custom,
}

impl SoundEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            SoundEvent::Notification => "Notification",
            SoundEvent::Message => "Message",
            SoundEvent::Email => "Email",
            SoundEvent::Error => "Error",
            SoundEvent::Warning => "Warning",
            SoundEvent::Success => "Success",
            SoundEvent::Information => "Information",
            SoundEvent::Custom => "Custom",
        }
    }
}

/// Sound configuration
#[derive(Debug, Clone)]
pub struct SoundConfig {
    pub id: String,
    pub event: SoundEvent,
    pub sound_file: String,
    pub volume: u8,
    pub enabled: bool,
}

impl SoundConfig {
    pub fn new(
        id: String,
        event: SoundEvent,
        sound_file: String,
    ) -> Self {
        SoundConfig {
            id,
            event,
            sound_file,
            volume: 80,
            enabled: true,
        }
    }

    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume.min(100);
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Notification Sound Manager
pub struct NotificationSoundManager {
    sounds: HashMap<String, SoundConfig>,
    sound_type: NotificationSoundType,
    master_volume: u8,
    next_sound_id: u32,
}

impl NotificationSoundManager {
    pub fn new() -> Self {
        let mut manager = NotificationSoundManager {
            sounds: HashMap::new(),
            sound_type: NotificationSoundType::Default,
            master_volume: 80,
            next_sound_id: 1,
        };

        manager.add_default_sounds();
        manager
    }

    fn add_default_sounds(&mut self) {
        // Notification
        let notif_id = format!("sound_{}", self.next_sound_id);
        self.next_sound_id += 1;
        let notification = SoundConfig::new(
            notif_id.clone(),
            SoundEvent::Notification,
            "/usr/share/sounds/default/notification.wav".to_string(),
        );
        self.sounds.insert(notif_id, notification);

        // Message
        let msg_id = format!("sound_{}", self.next_sound_id);
        self.next_sound_id += 1;
        let message = SoundConfig::new(
            msg_id.clone(),
            SoundEvent::Message,
            "/usr/share/sounds/default/message.wav".to_string(),
        );
        self.sounds.insert(msg_id, message);

        // Email
        let email_id = format!("sound_{}", self.next_sound_id);
        self.next_sound_id += 1;
        let email = SoundConfig::new(
            email_id.clone(),
            SoundEvent::Email,
            "/usr/share/sounds/default/email.wav".to_string(),
        );
        self.sounds.insert(email_id, email);

        // Error
        let error_id = format!("sound_{}", self.next_sound_id);
        self.next_sound_id += 1;
        let error = SoundConfig::new(
            error_id.clone(),
            SoundEvent::Error,
            "/usr/share/sounds/default/error.wav".to_string(),
        );
        self.sounds.insert(error_id, error);

        // Success
        let success_id = format!("sound_{}", self.next_sound_id);
        self.next_sound_id += 1;
        let success = SoundConfig::new(
            success_id.clone(),
            SoundEvent::Success,
            "/usr/share/sounds/default/success.wav".to_string(),
        );
        self.sounds.insert(success_id, success);
    }

    pub fn set_sound_type(&mut self, sound_type: NotificationSoundType) {
        self.sound_type = sound_type;
    }

    pub fn set_master_volume(&mut self, volume: u8) {
        self.master_volume = volume.min(100);
    }

    pub fn add_sound(
        &mut self,
        event: SoundEvent,
        sound_file: String,
    ) -> String {
        let id = format!("sound_{}", self.next_sound_id);
        self.next_sound_id += 1;

        let sound = SoundConfig::new(id.clone(), event, sound_file);
        self.sounds.insert(id.clone(), sound);
        id
    }

    pub fn remove_sound(&mut self, id: &str) -> bool {
        self.sounds.remove(id).is_some()
    }

    pub fn get_sound(&self, id: &str) -> Option<&SoundConfig> {
        self.sounds.get(id)
    }

    pub fn get_sounds(&self) -> Vec<&SoundConfig> {
        self.sounds.values().collect()
    }

    pub fn get_sounds_by_event(&self, event: SoundEvent) -> Vec<&SoundConfig> {
        self.sounds
            .values()
            .filter(|s| s.event == event)
            .collect()
    }

    pub fn get_enabled_sounds(&self) -> Vec<&SoundConfig> {
        self.sounds
            .values()
            .filter(|s| s.enabled)
            .collect()
    }

    pub fn set_sound_volume(&mut self, id: &str, volume: u8) -> bool {
        if let Some(sound) = self.sounds.get_mut(id) {
            sound.set_volume(volume);
            true
        } else {
            false
        }
    }

    pub fn set_sound_enabled(&mut self, id: &str, enabled: bool) -> bool {
        if let Some(sound) = self.sounds.get_mut(id) {
            sound.set_enabled(enabled);
            true
        } else {
            false
        }
    }

    pub fn play_sound(&self, event: SoundEvent) -> Option<&str> {
        if self.sound_type == NotificationSoundType::None {
            return None;
        }

        self.sounds
            .values()
            .find(|s| s.event == event && s.enabled)
            .map(|s| s.sound_file.as_str())
    }

    pub fn get_sound_type(&self) -> NotificationSoundType {
        self.sound_type
    }

    pub fn get_master_volume(&self) -> u8 {
        self.master_volume
    }

    pub fn get_statistics(&self) -> NotificationSoundStatistics {
        NotificationSoundStatistics {
            total_sounds: self.sounds.len(),
            enabled_sounds: self.get_enabled_sounds().len(),
            sound_type: self.sound_type,
        }
    }
}

impl Default for NotificationSoundManager {
    fn default() -> Self {
        Self::new()
    }
}

/// NotificationSoundStatistics
#[derive(Debug, Clone, Copy)]
pub struct NotificationSoundStatistics {
    pub total_sounds: usize,
    pub enabled_sounds: usize,
    pub sound_type: NotificationSoundType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_sound_manager_initialization() {
        let manager = NotificationSoundManager::new();
        assert_eq!(manager.get_sounds().len(), 5);
        assert_eq!(manager.get_sound_type(), NotificationSoundType::Default);
    }

    #[test]
    fn test_add_sound() {
        let mut manager = NotificationSoundManager::new();
        let id = manager.add_sound(SoundEvent::Custom, "/path/to/sound.wav".to_string());
        assert!(manager.get_sound(&id).is_some());
        assert_eq!(manager.get_sounds().len(), 6);
    }

    #[test]
    fn test_remove_sound() {
        let mut manager = NotificationSoundManager::new();
        let id = manager.add_sound(SoundEvent::Custom, "/path/to/sound.wav".to_string());
        assert!(manager.remove_sound(&id));
        assert_eq!(manager.get_sounds().len(), 5);
    }

    #[test]
    fn test_set_sound_type() {
        let mut manager = NotificationSoundManager::new();
        manager.set_sound_type(NotificationSoundType::Minimal);
        assert_eq!(manager.get_sound_type(), NotificationSoundType::Minimal);
    }

    #[test]
    fn test_set_master_volume() {
        let mut manager = NotificationSoundManager::new();
        manager.set_master_volume(90);
        assert_eq!(manager.get_master_volume(), 90);
    }

    #[test]
    fn test_set_sound_volume() {
        let mut manager = NotificationSoundManager::new();
        let sounds = manager.get_sounds();
        if let Some(sound) = sounds.first() {
            assert!(manager.set_sound_volume(&sound.id, 75));
            assert_eq!(manager.get_sound(&sound.id).unwrap().volume, 75);
        }
    }

    #[test]
    fn test_set_sound_enabled() {
        let mut manager = NotificationSoundManager::new();
        let sounds = manager.get_sounds();
        if let Some(sound) = sounds.first() {
            assert!(manager.set_sound_enabled(&sound.id, false));
            assert!(!manager.get_sound(&sound.id).unwrap().enabled);
        }
    }

    #[test]
    fn test_play_sound() {
        let manager = NotificationSoundManager::new();
        let sound = manager.play_sound(SoundEvent::Notification);
        assert!(sound.is_some());
    }

    #[test]
    fn test_play_sound_none_type() {
        let mut manager = NotificationSoundManager::new();
        manager.set_sound_type(NotificationSoundType::None);
        let sound = manager.play_sound(SoundEvent::Notification);
        assert!(sound.is_none());
    }

    #[test]
    fn test_get_sounds_by_event() {
        let manager = NotificationSoundManager::new();
        let notifications = manager.get_sounds_by_event(SoundEvent::Notification);
        assert_eq!(notifications.len(), 1);
    }

    #[test]
    fn test_statistics() {
        let manager = NotificationSoundManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.total_sounds, 5);
    }
}
