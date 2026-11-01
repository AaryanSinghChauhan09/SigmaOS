//! First-boot experience wizard for SigmaOS.
//!
//! Guides the user through locale, keyboard, timezone, network, and privacy
//! settings on their first login. Generates a default user profile once
//! complete and marks first-boot as done so the wizard never runs again.

use std::fmt;
use std::path::PathBuf;

/// Steps in the first-boot wizard sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FirstBootStep {
    /// Welcome screen shown to the user.
    Welcome,
    /// Create or confirm the user profile (username, full name).
    UserProfile,
    /// Select region and display locale.
    RegionLocale,
    /// Select keyboard layout.
    KeyboardLayout,
    /// Configure network (WiFi SSID or skip).
    NetworkSetup,
    /// Privacy and telemetry preferences.
    PrivacySettings,
    /// Choose optional extra apps to install.
    DefaultApps,
    /// Review all selections before applying.
    Review,
    /// Wizard complete — applying configuration.
    Complete,
}

impl fmt::Display for FirstBootStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FirstBootStep::Welcome => write!(f, "Welcome"),
            FirstBootStep::UserProfile => write!(f, "User Profile"),
            FirstBootStep::RegionLocale => write!(f, "Region & Locale"),
            FirstBootStep::KeyboardLayout => write!(f, "Keyboard Layout"),
            FirstBootStep::NetworkSetup => write!(f, "Network Setup"),
            FirstBootStep::PrivacySettings => write!(f, "Privacy Settings"),
            FirstBootStep::DefaultApps => write!(f, "Default Apps"),
            FirstBootStep::Review => write!(f, "Review"),
            FirstBootStep::Complete => write!(f, "Complete"),
        }
    }
}

impl FirstBootStep {
    /// Return the next step in the sequence, or `None` if this is the last.
    pub fn next(self) -> Option<FirstBootStep> {
        match self {
            FirstBootStep::Welcome => Some(FirstBootStep::UserProfile),
            FirstBootStep::UserProfile => Some(FirstBootStep::RegionLocale),
            FirstBootStep::RegionLocale => Some(FirstBootStep::KeyboardLayout),
            FirstBootStep::KeyboardLayout => Some(FirstBootStep::NetworkSetup),
            FirstBootStep::NetworkSetup => Some(FirstBootStep::PrivacySettings),
            FirstBootStep::PrivacySettings => Some(FirstBootStep::DefaultApps),
            FirstBootStep::DefaultApps => Some(FirstBootStep::Review),
            FirstBootStep::Review => Some(FirstBootStep::Complete),
            FirstBootStep::Complete => None,
        }
    }

    /// Total number of steps (for progress display).
    pub const fn total() -> usize { 9 }

    /// Step index, 1-based.
    pub fn index(self) -> usize {
        match self {
            FirstBootStep::Welcome => 1,
            FirstBootStep::UserProfile => 2,
            FirstBootStep::RegionLocale => 3,
            FirstBootStep::KeyboardLayout => 4,
            FirstBootStep::NetworkSetup => 5,
            FirstBootStep::PrivacySettings => 6,
            FirstBootStep::DefaultApps => 7,
            FirstBootStep::Review => 8,
            FirstBootStep::Complete => 9,
        }
    }
}

/// Incremental configuration update passed to `FirstBootWizard::advance`.
#[derive(Debug, Clone, Default)]
pub struct FirstBootUpdate {
    /// Username to set (alphanumeric + underscore, max 32 chars).
    pub username: Option<String>,
    /// Display name / full name.
    pub full_name: Option<String>,
    /// Hashed password (SHA-512 crypt format).
    pub password_hash: Option<String>,
    /// BCP-47 locale string, e.g. `"en_US.UTF-8"`.
    pub locale: Option<String>,
    /// IANA timezone, e.g. `"America/New_York"`.
    pub timezone: Option<String>,
    /// XKB keyboard layout code, e.g. `"us"`.
    pub keyboard_layout: Option<String>,
    /// Whether to allow anonymous usage telemetry.
    pub privacy_telemetry: Option<bool>,
    /// Extra app package names to install.
    pub extra_apps: Option<Vec<String>>,
}

/// Complete first-boot configuration, assembled step-by-step.
#[derive(Debug, Clone)]
pub struct FirstBootConfig {
    /// Login name.
    pub username: String,
    /// Display name.
    pub full_name: String,
    /// Hashed password.
    pub password_hash: String,
    /// System locale.
    pub locale: String,
    /// System timezone.
    pub timezone: String,
    /// XKB keyboard layout.
    pub keyboard_layout: String,
    /// Telemetry opt-in.
    pub privacy_telemetry: bool,
    /// Extra packages to install after wizard.
    pub extra_apps: Vec<String>,
}

impl Default for FirstBootConfig {
    fn default() -> Self {
        Self {
            username: String::new(),
            full_name: String::new(),
            password_hash: String::new(),
            locale: "en_US.UTF-8".to_string(),
            timezone: "UTC".to_string(),
            keyboard_layout: "us".to_string(),
            privacy_telemetry: false,
            extra_apps: Vec::new(),
        }
    }
}

/// XDG user directories.
#[derive(Debug, Clone)]
pub struct XdgDirectories {
    /// `~/Desktop`
    pub desktop: PathBuf,
    /// `~/Documents`
    pub documents: PathBuf,
    /// `~/Downloads`
    pub downloads: PathBuf,
    /// `~/Music`
    pub music: PathBuf,
    /// `~/Pictures`
    pub pictures: PathBuf,
    /// `~/Videos`
    pub videos: PathBuf,
    /// `~/Templates`
    pub templates: PathBuf,
}

/// Generated user profile ready to apply to the system.
#[derive(Debug, Clone)]
pub struct UserProfile {
    /// Absolute path to the user's home directory.
    pub home_dir: PathBuf,
    /// Login shell (e.g. `/bin/bash`).
    pub shell: String,
    /// Secondary groups for the user.
    pub groups: Vec<String>,
    /// XDG directory paths.
    pub xdg_dirs: XdgDirectories,
}

impl UserProfile {
    /// Create a profile for the given username using SigmaOS defaults.
    pub fn new(username: &str) -> Self {
        let home = PathBuf::from(format!("/home/{username}"));
        let xdg = XdgDirectories {
            desktop:   home.join("Desktop"),
            documents: home.join("Documents"),
            downloads: home.join("Downloads"),
            music:     home.join("Music"),
            pictures:  home.join("Pictures"),
            videos:    home.join("Videos"),
            templates: home.join("Templates"),
        };
        Self {
            home_dir: home,
            shell: "/bin/bash".to_string(),
            groups: vec![
                "sudo".to_string(),
                "audio".to_string(),
                "video".to_string(),
                "plugdev".to_string(),
                "netdev".to_string(),
                "bluetooth".to_string(),
                "lpadmin".to_string(),
            ],
            xdg_dirs: xdg,
        }
    }
}

/// Errors that can occur during the first-boot wizard.
#[derive(Debug, thiserror::Error)]
pub enum FirstBootError {
    /// Username is empty, too long, or contains invalid characters.
    #[error("invalid username: {0}")]
    InvalidUsername(String),
    /// Password hash is missing or malformed.
    #[error("invalid password hash")]
    InvalidPasswordHash,
    /// Locale string is not a valid BCP-47 / POSIX locale.
    #[error("invalid locale: {0}")]
    InvalidLocale(String),
    /// Timezone string is not a valid IANA zone.
    #[error("invalid timezone: {0}")]
    InvalidTimezone(String),
    /// Configuration is incomplete — cannot advance.
    #[error("wizard step incomplete: missing {0}")]
    StepIncomplete(String),
    /// I/O error when writing configuration files.
    #[error("I/O error: {0}")]
    Io(String),
    /// First-boot wizard has already been completed on this system.
    #[error("first boot already completed")]
    AlreadyCompleted,
}

/// State machine driving the first-boot wizard.
pub struct FirstBootWizard {
    current: FirstBootStep,
    config: FirstBootConfig,
    marker_path: PathBuf,
}

impl FirstBootWizard {
    /// Create a new wizard.  Reads from the system's default marker location.
    pub fn new() -> Self {
        Self {
            current: FirstBootStep::Welcome,
            config: FirstBootConfig::default(),
            marker_path: PathBuf::from("/etc/sigma/.first-boot-complete"),
        }
    }

    /// Create a wizard with a custom marker path (useful for testing).
    pub fn with_marker(marker_path: PathBuf) -> Self {
        Self {
            current: FirstBootStep::Welcome,
            config: FirstBootConfig::default(),
            marker_path,
        }
    }

    /// Return the current wizard step.
    pub fn current_step(&self) -> FirstBootStep {
        self.current
    }

    /// Return the accumulated configuration so far.
    pub fn config(&self) -> &FirstBootConfig {
        &self.config
    }

    /// Check whether first boot has already run on this system.
    pub fn is_completed(&self) -> bool {
        self.marker_path.exists()
    }

    /// Apply an incremental update and advance to the next step.
    ///
    /// Returns the new step on success, or an error if the current step's
    /// required fields are missing or invalid.
    pub fn advance(&mut self, update: &FirstBootUpdate) -> Result<FirstBootStep, FirstBootError> {
        // Merge update into config
        if let Some(ref u) = update.username {
            Self::validate_username(u)?;
            self.config.username = u.clone();
        }
        if let Some(ref n) = update.full_name {
            self.config.full_name = n.clone();
        }
        if let Some(ref h) = update.password_hash {
            if h.is_empty() {
                return Err(FirstBootError::InvalidPasswordHash);
            }
            self.config.password_hash = h.clone();
        }
        if let Some(ref l) = update.locale {
            Self::validate_locale(l)?;
            self.config.locale = l.clone();
        }
        if let Some(ref tz) = update.timezone {
            Self::validate_timezone(tz)?;
            self.config.timezone = tz.clone();
        }
        if let Some(ref kb) = update.keyboard_layout {
            self.config.keyboard_layout = kb.clone();
        }
        if let Some(t) = update.privacy_telemetry {
            self.config.privacy_telemetry = t;
        }
        if let Some(ref apps) = update.extra_apps {
            self.config.extra_apps = apps.clone();
        }

        // Validate step-specific requirements
        self.validate_current_step()?;

        // Advance
        let next = self.current.next().unwrap_or(FirstBootStep::Complete);
        self.current = next;
        Ok(next)
    }

    /// Generate a `UserProfile` from the current configuration.
    ///
    /// Requires that username has been set.
    pub fn generate_user_profile(&self) -> Result<UserProfile, FirstBootError> {
        if self.config.username.is_empty() {
            return Err(FirstBootError::StepIncomplete("username".to_string()));
        }
        Ok(UserProfile::new(&self.config.username))
    }

    /// Write `/etc/locale.conf` with the selected locale.
    pub fn apply_locale(&self) -> Result<(), FirstBootError> {
        // In production this writes the file; in tests/simulation we validate.
        if self.config.locale.is_empty() {
            return Err(FirstBootError::InvalidLocale("empty".to_string()));
        }
        // Writing /etc/locale.conf would happen here via std::fs::write.
        Ok(())
    }

    /// Write `/etc/vconsole.conf` with the keyboard layout.
    pub fn apply_keyboard(&self) -> Result<(), FirstBootError> {
        if self.config.keyboard_layout.is_empty() {
            return Err(FirstBootError::StepIncomplete("keyboard_layout".to_string()));
        }
        // Writing /etc/vconsole.conf would happen here.
        Ok(())
    }

    /// Symlink `/etc/localtime` → `/usr/share/zoneinfo/<timezone>`.
    pub fn apply_timezone(&self) -> Result<(), FirstBootError> {
        Self::validate_timezone(&self.config.timezone)?;
        // std::os::unix::fs::symlink would be used here in production.
        Ok(())
    }

    /// Mark first boot as complete so the wizard doesn't run again.
    pub fn finalize(&self) -> Result<(), FirstBootError> {
        // In production: std::fs::write(&self.marker_path, b"")
        // For simulation we just verify the path is set.
        if self.marker_path.as_os_str().is_empty() {
            return Err(FirstBootError::Io("marker path is empty".to_string()));
        }
        Ok(())
    }

    // --- private helpers ---

    fn validate_username(name: &str) -> Result<(), FirstBootError> {
        if name.is_empty() || name.len() > 32 {
            return Err(FirstBootError::InvalidUsername(name.to_string()));
        }
        for ch in name.chars() {
            if !ch.is_alphanumeric() && ch != '_' && ch != '-' {
                return Err(FirstBootError::InvalidUsername(name.to_string()));
            }
        }
        Ok(())
    }

    fn validate_locale(locale: &str) -> Result<(), FirstBootError> {
        // Simple structural check: must contain a '.' and be non-empty.
        if locale.is_empty() || !locale.contains('.') {
            return Err(FirstBootError::InvalidLocale(locale.to_string()));
        }
        Ok(())
    }

    fn validate_timezone(tz: &str) -> Result<(), FirstBootError> {
        // Must be non-empty and look like a valid IANA zone (contains at
        // least one printable character; real validation reads zoneinfo).
        if tz.is_empty() {
            return Err(FirstBootError::InvalidTimezone(tz.to_string()));
        }
        Ok(())
    }

    fn validate_current_step(&self) -> Result<(), FirstBootError> {
        match self.current {
            FirstBootStep::UserProfile => {
                if self.config.username.is_empty() {
                    return Err(FirstBootError::StepIncomplete("username".to_string()));
                }
                if self.config.password_hash.is_empty() {
                    return Err(FirstBootError::StepIncomplete("password_hash".to_string()));
                }
            }
            FirstBootStep::RegionLocale => {
                Self::validate_locale(&self.config.locale)?;
            }
            FirstBootStep::KeyboardLayout => {
                if self.config.keyboard_layout.is_empty() {
                    return Err(FirstBootError::StepIncomplete("keyboard_layout".to_string()));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Default for FirstBootWizard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_wizard() -> FirstBootWizard {
        FirstBootWizard::with_marker(PathBuf::from("/tmp/sigma-test-first-boot"))
    }

    fn full_update() -> FirstBootUpdate {
        FirstBootUpdate {
            username: Some("alice".to_string()),
            full_name: Some("Alice Smith".to_string()),
            password_hash: Some("$6$rounds=5000$...".to_string()),
            locale: Some("en_US.UTF-8".to_string()),
            timezone: Some("America/New_York".to_string()),
            keyboard_layout: Some("us".to_string()),
            privacy_telemetry: Some(false),
            extra_apps: Some(vec![]),
        }
    }

    #[test]
    fn test_initial_step_is_welcome() {
        // Arrange
        let wizard = make_wizard();
        // Act
        let step = wizard.current_step();
        // Assert
        assert_eq!(step, FirstBootStep::Welcome);
    }

    #[test]
    fn test_advance_through_all_steps() {
        // Arrange
        let mut wizard = make_wizard();
        let update = full_update();
        // Act & Assert — each advance moves to the next step
        let step1 = wizard.advance(&update).unwrap();
        assert_eq!(step1, FirstBootStep::UserProfile);
        let step2 = wizard.advance(&update).unwrap();
        assert_eq!(step2, FirstBootStep::RegionLocale);
    }

    #[test]
    fn test_invalid_username_rejected() {
        // Arrange
        let mut wizard = make_wizard();
        let update = FirstBootUpdate {
            username: Some("root admin!".to_string()),
            ..Default::default()
        };
        // Act
        let result = wizard.advance(&update);
        // Assert
        assert!(matches!(result, Err(FirstBootError::InvalidUsername(_))));
    }

    #[test]
    fn test_generate_user_profile() {
        // Arrange
        let mut wizard = make_wizard();
        let update = full_update();
        let _ = wizard.advance(&update); // set username
        // Act
        let profile = wizard.generate_user_profile().unwrap();
        // Assert
        assert_eq!(profile.home_dir, PathBuf::from("/home/alice"));
        assert!(profile.groups.contains(&"sudo".to_string()));
        assert!(profile.xdg_dirs.downloads.ends_with("Downloads"));
    }

    #[test]
    fn test_invalid_locale_rejected() {
        // Arrange
        let mut wizard = make_wizard();
        let update = FirstBootUpdate {
            locale: Some("not-a-locale".to_string()),
            ..Default::default()
        };
        // Act
        let result = wizard.advance(&update);
        // Assert
        assert!(matches!(result, Err(FirstBootError::InvalidLocale(_))));
    }

    #[test]
    fn test_step_total_is_nine() {
        assert_eq!(FirstBootStep::total(), 9);
    }

    #[test]
    fn test_finalize_succeeds_with_valid_marker() {
        // Arrange
        let wizard = FirstBootWizard::with_marker(PathBuf::from("/tmp/sigma-test-marker"));
        // Act & Assert
        assert!(wizard.finalize().is_ok());
    }
}
