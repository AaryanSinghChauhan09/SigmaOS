//! SigmaOS Screensaver System
//!
//! Inspired by: Omarchy screensaver (omancy-screensaver, branding, animation)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, IDLE_DETECTION, ENERGY_EFFICIENCY, USER_CONTROL.

/// Screensaver type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScreensaverType {
    Blank,
    Logo,
    Animation,
    Custom(String),
}

/// Screensaver status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScreensaverStatus {
    Disabled,
    Enabled,
    Active,
    Idle,
}

/// Screensaver configuration
#[derive(Debug, Clone)]
pub struct ScreensaverConfig {
    pub enabled: bool,
    pub idle_seconds: u64,
    pub screensaver_type: ScreensaverType,
    pub animation_speed: f32,
    pub logo_path: Option<String>,
    pub branding_text: Option<String>,
    pub fade_duration: u64,
    pub wake_on_input: bool,
}

/// Screensaver state
#[derive(Debug, Clone)]
pub struct ScreensaverState {
    pub status: ScreensaverStatus,
    pub idle_time_seconds: u64,
    pub activated_at: Option<u64>,
    pub deactivated_at: Option<u64>,
}

/// Screensaver manager
pub struct ScreensaverManager {
    pub config: ScreensaverConfig,
    pub state: ScreensaverState,
    pub themes: Vec<ScreensaverTheme>,
}

/// Screensaver theme
#[derive(Debug, Clone)]
pub struct ScreensaverTheme {
    pub name: String,
    pub screensaver_type: ScreensaverType,
    pub preview_path: Option<String>,
    pub colors: Vec<String>,
}

impl ScreensaverManager {
    pub fn new() -> Self {
        Self {
            config: ScreensaverConfig::default(),
            state: ScreensaverState::default(),
            themes: Vec::new(),
        }
    }

    /// Enable screensaver
    pub fn enable(&mut self) {
        self.config.enabled = true;
        self.state.status = ScreensaverStatus::Enabled;
    }

    /// Disable screensaver
    pub fn disable(&mut self) {
        self.config.enabled = false;
        self.state.status = ScreensaverStatus::Disabled;
        self.state.idle_time_seconds = 0;
        self.state.activated_at = None;
        self.state.deactivated_at = None;
    }

    /// Set idle timeout
    pub fn set_idle_seconds(&mut self, seconds: u64) -> Result<(), String> {
        if seconds < 10 {
            return Err("Idle timeout must be at least 10 seconds".to_string());
        }
        self.config.idle_seconds = seconds;
        Ok(())
    }

    /// Set screensaver type
    pub fn set_type(&mut self, screensaver_type: ScreensaverType) {
        self.config.screensaver_type = screensaver_type;
    }

    /// Set branding text
    pub fn set_branding_text(&mut self, text: Option<String>) {
        self.config.branding_text = text;
    }

    /// Set logo path
    pub fn set_logo_path(&mut self, path: Option<String>) {
        self.config.logo_path = path;
    }

    /// Update idle time
    pub fn update_idle_time(&mut self, seconds: u64) {
        self.state.idle_time_seconds = seconds;
        if self.config.enabled && seconds >= self.config.idle_seconds {
            self.state.status = ScreensaverStatus::Idle;
        } else {
            self.state.status = ScreensaverStatus::Enabled;
        }
    }

    /// Activate screensaver
    pub fn activate(&mut self) {
        if self.config.enabled {
            self.state.status = ScreensaverStatus::Active;
            self.state.activated_at = Some(now_secs());
            self.state.deactivated_at = None;
        }
    }

    /// Deactivate screensaver (wake)
    pub fn deactivate(&mut self) {
        if self.state.status == ScreensaverStatus::Active {
            self.state.status = ScreensaverStatus::Enabled;
            self.state.deactivated_at = Some(now_secs());
            self.state.idle_time_seconds = 0;
        }
    }

    /// Register a theme
    pub fn register_theme(&mut self, theme: ScreensaverTheme) {
        self.themes.push(theme);
    }

    /// List themes
    pub fn list_themes(&self) -> Vec<&ScreensaverTheme> {
        self.themes.iter().collect()
    }

    /// Is screensaver active
    pub fn is_active(&self) -> bool {
        self.state.status == ScreensaverStatus::Active
    }
}

impl Default for ScreensaverConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            idle_seconds: 300, // 5 minutes
            screensaver_type: ScreensaverType::Logo,
            animation_speed: 1.0,
            logo_path: None,
            branding_text: Some("SigmaOS".to_string()),
            fade_duration: 500,
            wake_on_input: true,
        }
    }
}

impl Default for ScreensaverState {
    fn default() -> Self {
        Self {
            status: ScreensaverStatus::Enabled,
            idle_time_seconds: 0,
            activated_at: None,
            deactivated_at: None,
        }
    }
}

impl Default for ScreensaverManager {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enable_disable() {
        let mut mgr = ScreensaverManager::new();
        assert!(mgr.config.enabled);
        mgr.disable();
        assert!(!mgr.config.enabled);
        assert_eq!(mgr.state.status, ScreensaverStatus::Disabled);
        mgr.enable();
        assert!(mgr.config.enabled);
    }

    #[test]
    fn test_set_idle_seconds() {
        let mut mgr = ScreensaverManager::new();
        assert!(mgr.set_idle_seconds(60).is_ok());
        assert_eq!(mgr.config.idle_seconds, 60);
        assert!(mgr.set_idle_seconds(5).is_err());
    }

    #[test]
    fn test_activate_deactivate() {
        let mut mgr = ScreensaverManager::new();
        mgr.enable();
        mgr.activate();
        assert!(mgr.is_active());
        mgr.deactivate();
        assert!(!mgr.is_active());
    }

    #[test]
    fn test_activate_when_disabled() {
        let mut mgr = ScreensaverManager::new();
        mgr.disable();
        mgr.activate();
        assert!(!mgr.is_active());
    }

    #[test]
    fn test_update_idle_time() {
        let mut mgr = ScreensaverManager::new();
        mgr.enable();
        mgr.set_idle_seconds(300).unwrap();
        mgr.update_idle_time(100);
        assert_eq!(mgr.state.status, ScreensaverStatus::Enabled);
        mgr.update_idle_time(300);
        assert_eq!(mgr.state.status, ScreensaverStatus::Idle);
    }

    #[test]
    fn test_register_theme() {
        let mut mgr = ScreensaverManager::new();
        mgr.register_theme(ScreensaverTheme {
            name: "sigma-logo".to_string(),
            screensaver_type: ScreensaverType::Logo,
            preview_path: Some("/usr/share/sigmaos/themes/logo-preview.png".to_string()),
            colors: vec!["#1e1e2e".to_string(), "#89b4fa".to_string()],
        });
        assert_eq!(mgr.list_themes().len(), 1);
    }
}