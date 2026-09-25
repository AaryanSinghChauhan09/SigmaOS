#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WizardStep {
    Welcome,
    SystemSnapshot,
    DriverCheck,
    MultimediaCodecs,
    Firewall,
    UpdateManager,
    AppTheme,
    DesktopLayout,
    Accounts,
    Privacy,
    Finish,
}

#[derive(Debug, Clone)]
pub struct DriverRecommendation {
    pub device_name: String,
    pub driver_package: String,
    pub proprietary: bool,
}

pub struct CodecInstaller;

impl CodecInstaller {
    pub fn is_installed() -> bool {
        false
    }
    
    pub fn install_nonfree_codecs() -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeVariant {
    Dark,
    Light,
    Auto,
}

pub struct ThemeSelector;

impl ThemeSelector {
    pub fn apply_theme(variant: ThemeVariant, accent_color: &str) {
        // apply theme settings
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesktopLayoutVariant {
    Traditional,
    Tiling,
    Hybrid,
}

pub struct LayoutSelector;

impl LayoutSelector {
    pub fn apply_layout(variant: DesktopLayoutVariant) {
        // apply layout settings
    }
}

#[derive(Debug, Clone)]
pub struct PrivacySettings {
    pub telemetry_opt_in: bool,
    pub crash_reporting: bool,
    pub location_services: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        PrivacySettings {
            telemetry_opt_in: false,
            crash_reporting: true,
            location_services: false,
        }
    }
}

pub enum UserRole {
    Developer,
    Creative,
    Office,
    Gaming,
}

pub struct AppSuggestions;

impl AppSuggestions {
    pub fn get_recommendations(role: UserRole) -> Vec<String> {
        match role {
            UserRole::Developer => vec!["vscode".to_string(), "git".to_string(), "docker".to_string()],
            UserRole::Creative => vec!["gimp".to_string(), "blender".to_string(), "kdenlive".to_string()],
            UserRole::Office => vec!["libreoffice".to_string(), "thunderbird".to_string()],
            UserRole::Gaming => vec!["steam".to_string(), "lutris".to_string(), "mangohud".to_string()],
        }
    }
}

pub struct OmarchyDevSetup;

impl OmarchyDevSetup {
    pub fn install_tools() -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SummaryReport {
    pub theme: ThemeVariant,
    pub layout: DesktopLayoutVariant,
    pub privacy: PrivacySettings,
    pub codecs_installed: bool,
}

pub struct OnboardingWizard {
    current_step: WizardStep,
    theme: ThemeVariant,
    layout: DesktopLayoutVariant,
    privacy: PrivacySettings,
    codecs_requested: bool,
}

impl OnboardingWizard {
    pub fn new() -> Self {
        OnboardingWizard {
            current_step: WizardStep::Welcome,
            theme: ThemeVariant::Auto,
            layout: DesktopLayoutVariant::Traditional,
            privacy: PrivacySettings::default(),
            codecs_requested: false,
        }
    }

    pub fn next_step(&mut self) {
        self.current_step = match self.current_step {
            WizardStep::Welcome => WizardStep::SystemSnapshot,
            WizardStep::SystemSnapshot => WizardStep::DriverCheck,
            WizardStep::DriverCheck => WizardStep::MultimediaCodecs,
            WizardStep::MultimediaCodecs => WizardStep::Firewall,
            WizardStep::Firewall => WizardStep::UpdateManager,
            WizardStep::UpdateManager => WizardStep::AppTheme,
            WizardStep::AppTheme => WizardStep::DesktopLayout,
            WizardStep::DesktopLayout => WizardStep::Accounts,
            WizardStep::Accounts => WizardStep::Privacy,
            WizardStep::Privacy => WizardStep::Finish,
            WizardStep::Finish => WizardStep::Finish,
        };
    }

    pub fn set_theme(&mut self, theme: ThemeVariant) {
        self.theme = theme;
    }

    pub fn set_layout(&mut self, layout: DesktopLayoutVariant) {
        self.layout = layout;
    }

    pub fn set_privacy(&mut self, privacy: PrivacySettings) {
        self.privacy = privacy;
    }

    pub fn generate_summary(&self) -> SummaryReport {
        SummaryReport {
            theme: self.theme.clone(),
            layout: self.layout.clone(),
            privacy: self.privacy.clone(),
            codecs_installed: self.codecs_requested,
        }
    }
    
    pub fn get_current_step(&self) -> &WizardStep {
        &self.current_step
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wizard_navigation() {
        let mut wizard = OnboardingWizard::new();
        assert_eq!(*wizard.get_current_step(), WizardStep::Welcome);
        wizard.next_step();
        assert_eq!(*wizard.get_current_step(), WizardStep::SystemSnapshot);
    }

    #[test]
    fn test_app_suggestions_developer() {
        let recs = AppSuggestions::get_recommendations(UserRole::Developer);
        assert!(recs.contains(&"git".to_string()));
    }

    #[test]
    fn test_set_preferences() {
        let mut wizard = OnboardingWizard::new();
        wizard.set_theme(ThemeVariant::Dark);
        wizard.set_layout(DesktopLayoutVariant::Tiling);
        
        let summary = wizard.generate_summary();
        assert_eq!(summary.theme, ThemeVariant::Dark);
        assert_eq!(summary.layout, DesktopLayoutVariant::Tiling);
    }

    #[test]
    fn test_privacy_defaults() {
        let p = PrivacySettings::default();
        assert!(!p.telemetry_opt_in);
        assert!(p.crash_reporting);
    }

    #[test]
    fn test_codec_installer() {
        assert!(!CodecInstaller::is_installed());
        assert!(CodecInstaller::install_nonfree_codecs().is_ok());
    }

    #[test]
    fn test_dev_setup() {
        assert!(OmarchyDevSetup::install_tools().is_ok());
    }
}
