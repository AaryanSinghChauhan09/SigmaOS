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
    pub fn is_installed() -> Result<bool, &'static str> {
        Err("Codec package query backend is unavailable")
    }

    pub fn install_nonfree_codecs() -> Result<(), &'static str> {
        Err("Codec package installation backend is unavailable")
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
    pub fn apply_theme(_variant: ThemeVariant, _accent_color: &str) -> Result<(), &'static str> {
        Err("Desktop theme settings backend is unavailable")
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
    pub fn apply_layout(_variant: DesktopLayoutVariant) -> Result<(), &'static str> {
        Err("Desktop layout settings backend is unavailable")
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
            crash_reporting: false,
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
            UserRole::Developer => vec![
                "vscode".to_string(),
                "git".to_string(),
                "docker".to_string(),
            ],
            UserRole::Creative => vec![
                "gimp".to_string(),
                "blender".to_string(),
                "kdenlive".to_string(),
            ],
            UserRole::Office => vec!["libreoffice".to_string(), "thunderbird".to_string()],
            UserRole::Gaming => vec![
                "steam".to_string(),
                "lutris".to_string(),
                "mangohud".to_string(),
            ],
        }
    }
}

pub struct OmarchyDevSetup;

impl OmarchyDevSetup {
    pub fn install_tools() -> Result<(), &'static str> {
        Err("Developer tool package backend is unavailable")
    }
}

#[derive(Debug, Clone)]
pub struct SummaryReport {
    pub theme: ThemeVariant,
    pub layout: DesktopLayoutVariant,
    pub privacy: PrivacySettings,
    pub codecs_requested: bool,
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

    pub fn previous_step(&mut self) {
        self.current_step = match self.current_step {
            WizardStep::Welcome => WizardStep::Welcome,
            WizardStep::SystemSnapshot => WizardStep::Welcome,
            WizardStep::DriverCheck => WizardStep::SystemSnapshot,
            WizardStep::MultimediaCodecs => WizardStep::DriverCheck,
            WizardStep::Firewall => WizardStep::MultimediaCodecs,
            WizardStep::UpdateManager => WizardStep::Firewall,
            WizardStep::AppTheme => WizardStep::UpdateManager,
            WizardStep::DesktopLayout => WizardStep::AppTheme,
            WizardStep::Accounts => WizardStep::DesktopLayout,
            WizardStep::Privacy => WizardStep::Accounts,
            WizardStep::Finish => WizardStep::Privacy,
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

    pub fn request_codecs(&mut self, requested: bool) {
        self.codecs_requested = requested;
    }

    pub fn generate_summary(&self) -> SummaryReport {
        SummaryReport {
            theme: self.theme.clone(),
            layout: self.layout.clone(),
            privacy: self.privacy.clone(),
            codecs_requested: self.codecs_requested,
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
        wizard.previous_step();
        assert_eq!(*wizard.get_current_step(), WizardStep::Welcome);
        wizard.next_step();
        assert_eq!(*wizard.get_current_step(), WizardStep::SystemSnapshot);
        wizard.previous_step();
        assert_eq!(*wizard.get_current_step(), WizardStep::Welcome);
        wizard.next_step();
        wizard.next_step();
        assert_eq!(*wizard.get_current_step(), WizardStep::DriverCheck);
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
        assert!(!p.crash_reporting);
        assert!(!p.location_services);
    }

    #[test]
    fn test_unavailable_onboarding_actions_fail_closed() {
        assert!(CodecInstaller::is_installed().is_err());
        assert!(CodecInstaller::install_nonfree_codecs().is_err());
        assert!(ThemeSelector::apply_theme(ThemeVariant::Dark, "blue").is_err());
        assert!(LayoutSelector::apply_layout(DesktopLayoutVariant::Tiling).is_err());
        assert!(OmarchyDevSetup::install_tools().is_err());
    }

    #[test]
    fn test_summary_reports_request_without_claiming_installation() {
        let mut wizard = OnboardingWizard::new();
        wizard.request_codecs(true);
        assert!(wizard.generate_summary().codecs_requested);
        assert!(CodecInstaller::install_nonfree_codecs().is_err());
    }
}
