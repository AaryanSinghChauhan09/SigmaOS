// SigmaOS Desktop Accessibility Manager
// Inspired by Linux Mint's accessibility tools and Omarchy's accessibility features

use std::collections::HashMap;

/// Screen reader mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenReaderMode {
    Off,
    On,
    OnWithMagnification,
}

impl ScreenReaderMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScreenReaderMode::Off => "Off",
            ScreenReaderMode::On => "On",
            ScreenReaderMode::OnWithMagnification => "On with Magnification",
        }
    }
}

/// High contrast mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighContrastMode {
    Off,
    On,
    HighContrastBlack,
    HighContrastWhite,
}

impl HighContrastMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            HighContrastMode::Off => "Off",
            HighContrastMode::On => "On",
            HighContrastMode::HighContrastBlack => "High Contrast Black",
            HighContrastMode::HighContrastWhite => "High Contrast White",
        }
    }
}

/// Text scaling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextScaling {
    Normal,
    Large,
    Larger,
    ExtraLarge,
}

impl TextScaling {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextScaling::Normal => "Normal",
            TextScaling::Large => "Large",
            TextScaling::Larger => "Larger",
            TextScaling::ExtraLarge => "Extra Large",
        }
    }

    pub fn scale_factor(&self) -> f32 {
        match self {
            TextScaling::Normal => 1.0,
            TextScaling::Large => 1.25,
            TextScaling::Larger => 1.5,
            TextScaling::ExtraLarge => 2.0,
        }
    }
}

/// Cursor size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum A11yCursorSize {
    Default,
    Medium,
    Large,
    ExtraLarge,
}

impl A11yCursorSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            A11yCursorSize::Default => "Default",
            A11yCursorSize::Medium => "Medium",
            A11yCursorSize::Large => "Large",
            A11yCursorSize::ExtraLarge => "Extra Large",
        }
    }

    pub fn scale_factor(&self) -> f32 {
        match self {
            A11yCursorSize::Default => 1.0,
            A11yCursorSize::Medium => 1.5,
            A11yCursorSize::Large => 2.0,
            A11yCursorSize::ExtraLarge => 2.5,
        }
    }
}

/// Keyboard repeat
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardRepeat {
    Off,
    Slow,
    Medium,
    Fast,
}

impl KeyboardRepeat {
    pub fn as_str(&self) -> &'static str {
        match self {
            KeyboardRepeat::Off => "Off",
            KeyboardRepeat::Slow => "Slow",
            KeyboardRepeat::Medium => "Medium",
            KeyboardRepeat::Fast => "Fast",
        }
    }

    pub fn delay_ms(&self) -> u32 {
        match self {
            KeyboardRepeat::Off => 0,
            KeyboardRepeat::Slow => 600,
            KeyboardRepeat::Medium => 400,
            KeyboardRepeat::Fast => 200,
        }
    }

    pub fn rate_ms(&self) -> u32 {
        match self {
            KeyboardRepeat::Off => 0,
            KeyboardRepeat::Slow => 100,
            KeyboardRepeat::Medium => 50,
            KeyboardRepeat::Fast => 30,
        }
    }
}

/// Desktop Accessibility Profile
#[derive(Debug, Clone)]
pub struct A11yProfile {
    pub id: String,
    pub name: String,
    pub screen_reader: ScreenReaderMode,
    pub high_contrast: HighContrastMode,
    pub text_scaling: TextScaling,
    pub cursor_size: A11yCursorSize,
    pub keyboard_repeat: KeyboardRepeat,
    pub sticky_keys: bool,
    pub slow_keys: bool,
    pub bounce_keys: bool,
}

impl A11yProfile {
    pub fn new(id: String, name: String) -> Self {
        A11yProfile {
            id,
            name,
            screen_reader: ScreenReaderMode::Off,
            high_contrast: HighContrastMode::Off,
            text_scaling: TextScaling::Normal,
            cursor_size: A11yCursorSize::Default,
            keyboard_repeat: KeyboardRepeat::Medium,
            sticky_keys: false,
            slow_keys: false,
            bounce_keys: false,
        }
    }

    pub fn set_screen_reader(&mut self, mode: ScreenReaderMode) {
        self.screen_reader = mode;
    }

    pub fn set_high_contrast(&mut self, mode: HighContrastMode) {
        self.high_contrast = mode;
    }

    pub fn set_text_scaling(&mut self, scaling: TextScaling) {
        self.text_scaling = scaling;
    }

    pub fn set_cursor_size(&mut self, size: A11yCursorSize) {
        self.cursor_size = size;
    }

    pub fn set_keyboard_repeat(&mut self, repeat: KeyboardRepeat) {
        self.keyboard_repeat = repeat;
    }

    pub fn set_sticky_keys(&mut self, enabled: bool) {
        self.sticky_keys = enabled;
    }

    pub fn set_slow_keys(&mut self, enabled: bool) {
        self.slow_keys = enabled;
    }

    pub fn set_bounce_keys(&mut self, enabled: bool) {
        self.bounce_keys = enabled;
    }
}

/// Desktop Accessibility Manager
pub struct DesktopA11yManager {
    profiles: HashMap<String, A11yProfile>,
    current_profile_id: Option<String>,
    next_profile_id: u32,
}

impl DesktopA11yManager {
    pub fn new() -> Self {
        let mut manager = DesktopA11yManager {
            profiles: HashMap::new(),
            current_profile_id: None,
            next_profile_id: 1,
        };

        manager.add_default_profiles();
        manager
    }

    fn add_default_profiles(&mut self) {
        // Default Profile
        let default_id = format!("profile_{}", self.next_profile_id);
        self.next_profile_id += 1;
        let default = A11yProfile::new(default_id.clone(), "Default".to_string());
        self.profiles.insert(default_id.clone(), default);
        self.current_profile_id = Some(default_id);

        // Visual Impairment Profile
        let visual_id = format!("profile_{}", self.next_profile_id);
        self.next_profile_id += 1;
        let mut visual = A11yProfile::new(visual_id.clone(), "Visual Impairment".to_string());
        visual.set_screen_reader(ScreenReaderMode::On);
        visual.set_high_contrast(HighContrastMode::HighContrastBlack);
        visual.set_text_scaling(TextScaling::Larger);
        visual.set_cursor_size(A11yCursorSize::Large);
        self.profiles.insert(visual_id, visual);

        // Motor Impairment Profile
        let motor_id = format!("profile_{}", self.next_profile_id);
        self.next_profile_id += 1;
        let mut motor = A11yProfile::new(motor_id.clone(), "Motor Impairment".to_string());
        motor.set_sticky_keys(true);
        motor.set_slow_keys(true);
        motor.set_bounce_keys(true);
        motor.set_keyboard_repeat(KeyboardRepeat::Slow);
        self.profiles.insert(motor_id, motor);

        // High Contrast Profile
        let contrast_id = format!("profile_{}", self.next_profile_id);
        self.next_profile_id += 1;
        let mut contrast = A11yProfile::new(contrast_id.clone(), "High Contrast".to_string());
        contrast.set_high_contrast(HighContrastMode::HighContrastBlack);
        contrast.set_text_scaling(TextScaling::Large);
        self.profiles.insert(contrast_id, contrast);
    }

    pub fn add_profile(&mut self, name: String) -> String {
        let id = format!("profile_{}", self.next_profile_id);
        self.next_profile_id += 1;

        let profile = A11yProfile::new(id.clone(), name);
        self.profiles.insert(id.clone(), profile);
        id
    }

    pub fn remove_profile(&mut self, id: &str) -> bool {
        if self.current_profile_id.as_ref() == Some(&id.to_string()) {
            return false;
        }
        self.profiles.remove(id).is_some()
    }

    pub fn set_current_profile(&mut self, id: &str) -> bool {
        if self.profiles.contains_key(id) {
            self.current_profile_id = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_profile(&self, id: &str) -> Option<&A11yProfile> {
        self.profiles.get(id)
    }

    pub fn get_profiles(&self) -> Vec<&A11yProfile> {
        self.profiles.values().collect()
    }

    pub fn get_current_profile(&self) -> Option<&A11yProfile> {
        if let Some(current_id) = &self.current_profile_id {
            self.profiles.get(current_id)
        } else {
            None
        }
    }

    pub fn update_profile_screen_reader(&mut self, id: &str, mode: ScreenReaderMode) -> bool {
        if let Some(profile) = self.profiles.get_mut(id) {
            profile.set_screen_reader(mode);
            true
        } else {
            false
        }
    }

    pub fn update_profile_high_contrast(&mut self, id: &str, mode: HighContrastMode) -> bool {
        if let Some(profile) = self.profiles.get_mut(id) {
            profile.set_high_contrast(mode);
            true
        } else {
            false
        }
    }

    pub fn update_profile_text_scaling(&mut self, id: &str, scaling: TextScaling) -> bool {
        if let Some(profile) = self.profiles.get_mut(id) {
            profile.set_text_scaling(scaling);
            true
        } else {
            false
        }
    }

    pub fn update_profile_sticky_keys(&mut self, id: &str, enabled: bool) -> bool {
        if let Some(profile) = self.profiles.get_mut(id) {
            profile.set_sticky_keys(enabled);
            true
        } else {
            false
        }
    }

    pub fn get_statistics(&self) -> A11yManagerStatistics {
        A11yManagerStatistics {
            total_profiles: self.profiles.len(),
            current_profile: self.current_profile_id.is_some(),
        }
    }
}

impl Default for DesktopA11yManager {
    fn default() -> Self {
        Self::new()
    }
}

/// A11yManagerStatistics
#[derive(Debug, Clone, Copy)]
pub struct A11yManagerStatistics {
    pub total_profiles: usize,
    pub current_profile: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a11y_manager_initialization() {
        let manager = DesktopA11yManager::new();
        assert_eq!(manager.get_profiles().len(), 4);
        assert!(manager.get_current_profile().is_some());
    }

    #[test]
    fn test_add_profile() {
        let mut manager = DesktopA11yManager::new();
        let id = manager.add_profile("Custom".to_string());
        assert!(manager.get_profile(&id).is_some());
        assert_eq!(manager.get_profiles().len(), 5);
    }

    #[test]
    fn test_remove_profile() {
        let mut manager = DesktopA11yManager::new();
        let id = manager.add_profile("Custom".to_string());
        assert!(manager.remove_profile(&id));
        assert_eq!(manager.get_profiles().len(), 4);
    }

    #[test]
    fn test_set_current_profile() {
        let mut manager = DesktopA11yManager::new();
        let id = manager.add_profile("Custom".to_string());
        assert!(manager.set_current_profile(&id));
        assert_eq!(manager.get_current_profile().unwrap().id, id);
    }

    #[test]
    fn test_update_profile_screen_reader() {
        let mut manager = DesktopA11yManager::new();
        let profile_id = manager
            .get_profiles()
            .first()
            .map(|p| p.id.clone())
            .unwrap();
        assert!(manager.update_profile_screen_reader(&profile_id, ScreenReaderMode::On));
    }

    #[test]
    fn test_update_profile_high_contrast() {
        let mut manager = DesktopA11yManager::new();
        let profile_id = manager
            .get_profiles()
            .first()
            .map(|p| p.id.clone())
            .unwrap();
        assert!(manager.update_profile_high_contrast(&profile_id, HighContrastMode::On));
    }

    #[test]
    fn test_update_profile_sticky_keys() {
        let mut manager = DesktopA11yManager::new();
        let profile_id = manager
            .get_profiles()
            .first()
            .map(|p| p.id.clone())
            .unwrap();
        assert!(manager.update_profile_sticky_keys(&profile_id, true));
    }

    #[test]
    fn test_text_scaling_scale_factor() {
        assert_eq!(TextScaling::Normal.scale_factor(), 1.0);
        assert_eq!(TextScaling::Large.scale_factor(), 1.25);
        assert_eq!(TextScaling::Larger.scale_factor(), 1.5);
        assert_eq!(TextScaling::ExtraLarge.scale_factor(), 2.0);
    }

    #[test]
    fn test_keyboard_repeat_timing() {
        assert_eq!(KeyboardRepeat::Slow.delay_ms(), 600);
        assert_eq!(KeyboardRepeat::Medium.delay_ms(), 400);
        assert_eq!(KeyboardRepeat::Fast.delay_ms(), 200);
    }

    #[test]
    fn test_statistics() {
        let manager = DesktopA11yManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.total_profiles, 4);
    }
}
