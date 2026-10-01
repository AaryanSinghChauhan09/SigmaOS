// Onboarding Wizard for SigmaOS
// Onboarding wizard per Wiki 02-Getting-Started.md
// Provides first-boot configuration wizard

use std::string::{String, ToString};
use std::vec::Vec;

/// Language
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language {
    pub code: String,
    pub name: String,
    pub native_name: String,
}

impl Language {
    pub fn new(code: String, name: String, native_name: String) -> Self {
        Language {
            code,
            name,
            native_name,
        }
    }

    pub fn english() -> Self {
        Language::new(
            String::from("en"),
            String::from("English"),
            String::from("English"),
        )
    }

    pub fn spanish() -> Self {
        Language::new(
            String::from("es"),
            String::from("Spanish"),
            String::from("Español"),
        )
    }

    pub fn french() -> Self {
        Language::new(
            String::from("fr"),
            String::from("French"),
            String::from("Français"),
        )
    }

    pub fn german() -> Self {
        Language::new(
            String::from("de"),
            String::from("German"),
            String::from("Deutsch"),
        )
    }

    pub fn japanese() -> Self {
        Language::new(
            String::from("ja"),
            String::from("Japanese"),
            String::from("日本語"),
        )
    }

    pub fn chinese() -> Self {
        Language::new(
            String::from("zh"),
            String::from("Chinese"),
            String::from("中文"),
        )
    }
}

/// Region
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub code: String,
    pub name: String,
    pub timezone: String,
}

impl Region {
    pub fn new(code: String, name: String, timezone: String) -> Self {
        Region {
            code,
            name,
            timezone,
        }
    }

    pub fn us() -> Self {
        Region::new(
            String::from("US"),
            String::from("United States"),
            String::from("America/New_York"),
        )
    }

    pub fn eu() -> Self {
        Region::new(
            String::from("EU"),
            String::from("Europe"),
            String::from("Europe/Brussels"),
        )
    }

    pub fn uk() -> Self {
        Region::new(
            String::from("UK"),
            String::from("United Kingdom"),
            String::from("Europe/London"),
        )
    }

    pub fn jp() -> Self {
        Region::new(
            String::from("JP"),
            String::from("Japan"),
            String::from("Asia/Tokyo"),
        )
    }

    pub fn cn() -> Self {
        Region::new(
            String::from("CN"),
            String::from("China"),
            String::from("Asia/Shanghai"),
        )
    }
}

/// Onboarding step
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnboardingStep {
    LanguageAndRegion,
    NetworkConfiguration,
    UserAccountSetup,
    DesktopThemeSelection,
    PrivacySettings,
    Complete,
}

impl OnboardingStep {
    pub fn as_str(&self) -> &str {
        match self {
            OnboardingStep::LanguageAndRegion => "Language and Region Selection",
            OnboardingStep::NetworkConfiguration => "Network Configuration",
            OnboardingStep::UserAccountSetup => "User Account Setup",
            OnboardingStep::DesktopThemeSelection => "Desktop Theme Selection",
            OnboardingStep::PrivacySettings => "Privacy Settings",
            OnboardingStep::Complete => "Complete",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            OnboardingStep::LanguageAndRegion => OnboardingStep::NetworkConfiguration,
            OnboardingStep::NetworkConfiguration => OnboardingStep::UserAccountSetup,
            OnboardingStep::UserAccountSetup => OnboardingStep::DesktopThemeSelection,
            OnboardingStep::DesktopThemeSelection => OnboardingStep::PrivacySettings,
            OnboardingStep::PrivacySettings => OnboardingStep::Complete,
            OnboardingStep::Complete => OnboardingStep::Complete,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            OnboardingStep::LanguageAndRegion => OnboardingStep::LanguageAndRegion,
            OnboardingStep::NetworkConfiguration => OnboardingStep::LanguageAndRegion,
            OnboardingStep::UserAccountSetup => OnboardingStep::NetworkConfiguration,
            OnboardingStep::DesktopThemeSelection => OnboardingStep::UserAccountSetup,
            OnboardingStep::PrivacySettings => OnboardingStep::DesktopThemeSelection,
            OnboardingStep::Complete => OnboardingStep::PrivacySettings,
        }
    }
}

/// Desktop theme
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopTheme {
    Light,
    Dark,
    Auto,
}

impl DesktopTheme {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "light" => DesktopTheme::Light,
            "dark" => DesktopTheme::Dark,
            "auto" => DesktopTheme::Auto,
            _ => DesktopTheme::Dark,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            DesktopTheme::Light => "light",
            DesktopTheme::Dark => "dark",
            DesktopTheme::Auto => "auto",
        }
    }
}

/// Privacy settings
#[derive(Debug, Clone)]
pub struct PrivacySettings {
    pub send_anonymous_usage_data: bool,
    pub send_crash_reports: bool,
    pub enable_location_services: bool,
    pub enable_automatic_updates: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        PrivacySettings {
            send_anonymous_usage_data: false,
            send_crash_reports: true,
            enable_location_services: false,
            enable_automatic_updates: true,
        }
    }
}

impl PrivacySettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_usage_data(mut self, enabled: bool) -> Self {
        self.send_anonymous_usage_data = enabled;
        self
    }

    pub fn with_crash_reports(mut self, enabled: bool) -> Self {
        self.send_crash_reports = enabled;
        self
    }

    pub fn with_location_services(mut self, enabled: bool) -> Self {
        self.enable_location_services = enabled;
        self
    }

    pub fn with_automatic_updates(mut self, enabled: bool) -> Self {
        self.enable_automatic_updates = enabled;
        self
    }
}

/// Onboarding configuration
#[derive(Debug, Clone)]
pub struct OnboardingConfig {
    pub language: Language,
    pub region: Region,
    pub username: String,
    pub display_name: String,
    pub hostname: String,
    pub theme: DesktopTheme,
    pub privacy_settings: PrivacySettings,
}

impl Default for OnboardingConfig {
    fn default() -> Self {
        OnboardingConfig {
            language: Language::english(),
            region: Region::us(),
            username: String::from("sigma"),
            display_name: String::from("Sigma User"),
            hostname: String::from("sigmaos"),
            theme: DesktopTheme::Dark,
            privacy_settings: PrivacySettings::default(),
        }
    }
}

impl OnboardingConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }

    pub fn with_region(mut self, region: Region) -> Self {
        self.region = region;
        self
    }

    pub fn with_username(mut self, username: String) -> Self {
        self.username = username;
        self
    }

    pub fn with_display_name(mut self, display_name: String) -> Self {
        self.display_name = display_name;
        self
    }

    pub fn with_hostname(mut self, hostname: String) -> Self {
        self.hostname = hostname;
        self
    }

    pub fn with_theme(mut self, theme: DesktopTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn with_privacy_settings(mut self, privacy_settings: PrivacySettings) -> Self {
        self.privacy_settings = privacy_settings;
        self
    }

    pub fn validate(&self) -> Result<String, String> {
        let mut issues = Vec::new();

        if self.username.is_empty() {
            issues.push("Username cannot be empty");
        }

        if self.username.len() < 3 {
            issues.push("Username must be at least 3 characters");
        }

        if self.hostname.is_empty() {
            issues.push("Hostname cannot be empty");
        }

        if self.hostname.len() < 2 {
            issues.push("Hostname must be at least 2 characters");
        }

        if issues.is_empty() {
            Ok(String::from("Configuration is valid"))
        } else {
            Err(issues.join("; "))
        }
    }
}

/// Onboarding wizard
#[derive(Debug, Clone)]
pub struct OnboardingWizard {
    pub config: OnboardingConfig,
    pub current_step: OnboardingStep,
    pub steps_completed: Vec<OnboardingStep>,
}

impl Default for OnboardingWizard {
    fn default() -> Self {
        OnboardingWizard {
            config: OnboardingConfig::default(),
            current_step: OnboardingStep::LanguageAndRegion,
            steps_completed: Vec::new(),
        }
    }
}

impl OnboardingWizard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(mut self, config: OnboardingConfig) -> Self {
        self.config = config;
        self
    }

    pub fn set_language(&mut self, language: Language) {
        self.config.language = language;
    }

    pub fn set_region(&mut self, region: Region) {
        self.config.region = region;
    }

    pub fn set_username(&mut self, username: String) {
        self.config.username = username;
    }

    pub fn set_display_name(&mut self, display_name: String) {
        self.config.display_name = display_name;
    }

    pub fn set_hostname(&mut self, hostname: String) {
        self.config.hostname = hostname;
    }

    pub fn set_theme(&mut self, theme: DesktopTheme) {
        self.config.theme = theme;
    }

    pub fn set_privacy_settings(&mut self, privacy_settings: PrivacySettings) {
        self.config.privacy_settings = privacy_settings;
    }

    pub fn next_step(&mut self) {
        self.steps_completed.push(self.current_step);
        self.current_step = self.current_step.next();
    }

    pub fn previous_step(&mut self) {
        if let Some(last_completed) = self.steps_completed.pop() {
            self.current_step = last_completed;
        }
    }

    pub fn is_complete(&self) -> bool {
        self.current_step == OnboardingStep::Complete
    }

    pub fn can_proceed(&self) -> bool {
        match self.current_step {
            OnboardingStep::LanguageAndRegion => true,
            OnboardingStep::NetworkConfiguration => true,
            OnboardingStep::UserAccountSetup => !self.config.username.is_empty(),
            OnboardingStep::DesktopThemeSelection => true,
            OnboardingStep::PrivacySettings => true,
            OnboardingStep::Complete => true,
        }
    }

    pub fn validate(&self) -> Result<String, String> {
        self.config.validate()
    }

    pub fn get_current_step_title(&self) -> String {
        String::from(self.current_step.as_str())
    }

    pub fn get_step_progress(&self) -> (usize, usize) {
        let total_steps = 5;
        let completed = self.steps_completed.len();
        (completed, total_steps)
    }

    pub fn finish(&self) -> Result<String, String> {
        self.validate()?;
        Ok(String::from("Onboarding completed successfully"))
    }

    pub fn get_available_languages(&self) -> Vec<Language> {
        vec![
            Language::english(),
            Language::spanish(),
            Language::french(),
            Language::german(),
            Language::japanese(),
            Language::chinese(),
        ]
    }

    pub fn get_available_regions(&self) -> Vec<Region> {
        vec![
            Region::us(),
            Region::eu(),
            Region::uk(),
            Region::jp(),
            Region::cn(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_creation() {
        let lang = Language::english();
        assert_eq!(lang.code, "en");
        assert_eq!(lang.name, "English");
    }

    #[test]
    fn test_region_creation() {
        let region = Region::us();
        assert_eq!(region.code, "US");
        assert_eq!(region.timezone, "America/New_York");
    }

    #[test]
    fn test_onboarding_step_navigation() {
        let step = OnboardingStep::LanguageAndRegion;
        assert_eq!(step.next(), OnboardingStep::NetworkConfiguration);
        assert_eq!(step.next().next(), OnboardingStep::UserAccountSetup);
    }

    #[test]
    fn test_onboarding_step_previous() {
        let step = OnboardingStep::UserAccountSetup;
        assert_eq!(step.previous(), OnboardingStep::NetworkConfiguration);
    }

    #[test]
    fn test_desktop_theme_from_str() {
        assert_eq!(DesktopTheme::from_str("dark"), DesktopTheme::Dark);
        assert_eq!(DesktopTheme::from_str("light"), DesktopTheme::Light);
        assert_eq!(DesktopTheme::from_str("auto"), DesktopTheme::Auto);
    }

    #[test]
    fn test_privacy_settings_default() {
        let settings = PrivacySettings::default();
        assert!(!settings.send_anonymous_usage_data);
        assert!(settings.send_crash_reports);
        assert!(settings.enable_automatic_updates);
    }

    #[test]
    fn test_privacy_settings_builder() {
        let settings = PrivacySettings::new()
            .with_usage_data(true)
            .with_crash_reports(false);

        assert!(settings.send_anonymous_usage_data);
        assert!(!settings.send_crash_reports);
    }

    #[test]
    fn test_onboarding_config_default() {
        let config = OnboardingConfig::default();
        assert_eq!(config.username, "sigma");
        assert_eq!(config.hostname, "sigmaos");
        assert_eq!(config.theme, DesktopTheme::Dark);
    }

    #[test]
    fn test_onboarding_config_builder() {
        let config = OnboardingConfig::new()
            .with_username(String::from("testuser"))
            .with_hostname(String::from("testhost"));

        assert_eq!(config.username, "testuser");
        assert_eq!(config.hostname, "testhost");
    }

    #[test]
    fn test_onboarding_config_validate() {
        let config = OnboardingConfig::default();
        assert!(config.validate().is_ok());

        let invalid_config = OnboardingConfig::new().with_username(String::from("ab"));
        let invalid_config = OnboardingConfig::new()
            .with_username(String::from("ab"));
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_onboarding_wizard_creation() {
        let wizard = OnboardingWizard::new();
        assert_eq!(wizard.current_step, OnboardingStep::LanguageAndRegion);
        assert!(!wizard.is_complete());
    }

    #[test]
    fn test_onboarding_wizard_navigation() {
        let mut wizard = OnboardingWizard::new();
        wizard.next_step();
        assert_eq!(wizard.current_step, OnboardingStep::NetworkConfiguration);

        wizard.previous_step();
        assert_eq!(wizard.current_step, OnboardingStep::LanguageAndRegion);
    }

    #[test]
    fn test_onboarding_wizard_completion() {
        let mut wizard = OnboardingWizard::new();
        assert!(!wizard.is_complete());

        wizard.next_step();
        wizard.next_step();
        wizard.next_step();
        wizard.next_step();
        wizard.next_step();

        assert!(wizard.is_complete());
    }

    #[test]
    fn test_onboarding_wizard_can_proceed() {
        let mut wizard = OnboardingWizard::new();
        assert!(wizard.can_proceed());

        wizard.current_step = OnboardingStep::UserAccountSetup;
        assert!(!wizard.can_proceed());

        wizard.set_username(String::from("testuser"));
        assert!(wizard.can_proceed());
    }

    #[test]
    fn test_onboarding_wizard_finish() {
        let wizard = OnboardingWizard::new();
        assert!(wizard.finish().is_ok());
    }

    #[test]
    fn test_onboarding_wizard_get_step_progress() {
        let wizard = OnboardingWizard::new();
        let (completed, total) = wizard.get_step_progress();
        assert_eq!(completed, 0);
        assert_eq!(total, 5);
    }

    #[test]
    fn test_onboarding_wizard_available_languages() {
        let wizard = OnboardingWizard::new();
        let languages = wizard.get_available_languages();
        assert!(!languages.is_empty());
        assert!(languages.iter().any(|l| l.code == "en"));
    }

    #[test]
    fn test_onboarding_wizard_available_regions() {
        let wizard = OnboardingWizard::new();
        let regions = wizard.get_available_regions();
        assert!(!regions.is_empty());
        assert!(regions.iter().any(|r| r.code == "US"));
    }
}
