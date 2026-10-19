//! SigmaOS Display Text Size Manager
//!
//! Inspired by: Omarchy display text size (omancy-display-text-size)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, USER_FREEDOM_AND_CONTROL,
//! ACCESSIBILITY, USER_CUSTOMIZATION, CLEAR_ABSTRACTION, LEVEL_OF_ABSTRACTION.

/// Text size level
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextSizeLevel {
    Small,
    Normal,
    Large,
    ExtraLarge,
    Custom(u32),
}

/// Display text size manager
pub struct DisplayTextSizeManager {
    pub current_level: TextSizeLevel,
    pub scale_factor: f32,
    pub min_scale: f32,
    pub max_scale: f32,
    pub app_overrides: HashMap<String, f32>,
}

impl DisplayTextSizeManager {
    pub fn new() -> Self {
        Self {
            current_level: TextSizeLevel::Normal,
            scale_factor: 1.0,
            min_scale: 0.5,
            max_scale: 3.0,
            app_overrides: HashMap::new(),
        }
    }

    /// Set text size level
    pub fn set_level(&mut self, level: TextSizeLevel) {
        self.current_level = level.clone();
        self.scale_factor = match level {
            TextSizeLevel::Small => 0.75,
            TextSizeLevel::Normal => 1.0,
            TextSizeLevel::Large => 1.25,
            TextSizeLevel::ExtraLarge => 1.5,
            TextSizeLevel::Custom(px) => (px as f32) / 16.0, // Assume 16px base
        };
    }

    /// Set custom scale factor
    pub fn set_scale_factor(&mut self, factor: f32) -> Result<(), String> {
        if factor < self.min_scale || factor > self.max_scale {
            return Err(format!(
                "Scale factor must be between {} and {}",
                self.min_scale, self.max_scale
            ));
        }
        self.scale_factor = factor;
        self.current_level = TextSizeLevel::Custom((factor * 16.0) as u32);
        Ok(())
    }

    /// Increase scale
    pub fn increase(&mut self, step: f32) -> Result<(), String> {
        let new_factor = (self.scale_factor + step).min(self.max_scale);
        self.set_scale_factor(new_factor)
    }

    /// Decrease scale
    pub fn decrease(&mut self, step: f32) -> Result<(), String> {
        let new_factor = (self.scale_factor - step).max(self.min_scale);
        self.set_scale_factor(new_factor)
    }

    /// Reset to normal
    pub fn reset(&mut self) {
        self.set_level(TextSizeLevel::Normal);
    }

    /// Set app-specific override
    pub fn set_app_override(&mut self, app: String, factor: f32) -> Result<(), String> {
        if factor < self.min_scale || factor > self.max_scale {
            return Err(format!("Scale factor {} out of range", factor));
        }
        self.app_overrides.insert(app, factor);
        Ok(())
    }

    /// Get effective scale for an app
    pub fn get_effective_scale(&self, app: Option<&str>) -> f32 {
        if let Some(app) = app {
            if let Some(factor) = self.app_overrides.get(app) {
                return *factor;
            }
        }
        self.scale_factor
    }

    /// Remove app override
    pub fn remove_app_override(&mut self, app: &str) -> Option<f32> {
        self.app_overrides.remove(app)
    }

    /// List app overrides
    pub fn list_overrides(&self) -> Vec<(&String, &f32)> {
        self.app_overrides.iter().collect()
    }
}

impl Default for DisplayTextSizeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_scale() {
        let mgr = DisplayTextSizeManager::new();
        assert_eq!(mgr.scale_factor, 1.0);
    }

    #[test]
    fn test_set_level() {
        let mut mgr = DisplayTextSizeManager::new();
        mgr.set_level(TextSizeLevel::Large);
        assert_eq!(mgr.scale_factor, 1.25);
        assert!(matches!(mgr.current_level, TextSizeLevel::Large));
    }

    #[test]
    fn test_set_scale_factor() {
        let mut mgr = DisplayTextSizeManager::new();
        assert!(mgr.set_scale_factor(1.5).is_ok());
        assert_eq!(mgr.scale_factor, 1.5);
    }

    #[test]
    fn test_scale_out_of_range() {
        let mut mgr = DisplayTextSizeManager::new();
        assert!(mgr.set_scale_factor(5.0).is_err());
        assert!(mgr.set_scale_factor(0.1).is_err());
    }

    #[test]
    fn test_increase_decrease() {
        let mut mgr = DisplayTextSizeManager::new();
        assert!(mgr.increase(0.25).is_ok());
        assert_eq!(mgr.scale_factor, 1.25);
        assert!(mgr.decrease(0.5).is_ok());
        assert_eq!(mgr.scale_factor, 0.75);
    }

    #[test]
    fn test_app_override() {
        let mut mgr = DisplayTextSizeManager::new();
        assert!(mgr.set_app_override("terminal".to_string(), 1.5).is_ok());
        assert_eq!(mgr.get_effective_scale(Some("terminal")), 1.5);
        assert_eq!(mgr.get_effective_scale(Some("browser")), 1.0);
    }

    #[test]
    fn test_remove_override() {
        let mut mgr = DisplayTextSizeManager::new();
        mgr.set_app_override("app".to_string(), 2.0).ok();
        assert!(mgr.remove_app_override("app").is_some());
        assert_eq!(mgr.get_effective_scale(Some("app")), 1.0);
    }
}