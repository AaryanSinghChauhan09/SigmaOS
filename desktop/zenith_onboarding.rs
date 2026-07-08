// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// desktop/zenith_onboarding.rs — Zenith Onboarding Wizard
//
// Implements an onboarding wizard for new users to guide them through
// initial system setup and configuration
//
// Language: Rust (std for userland services)

use std::collections::HashMap;

// ─── Onboarding Step ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OnboardingStep {
    Welcome,
    Language,
    Region,
    Keyboard,
    Network,
    Privacy,
    Account,
    Theme,
    Accessibility,
    Complete,
}

#[derive(Debug, Clone)]
pub struct OnboardingPage {
    pub step: OnboardingStep,
    pub title: String,
    pub description: String,
    pub can_skip: bool,
    pub can_go_back: bool,
}

// ─── User Configuration ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct UserConfig {
    pub username: String,
    pub full_name: String,
    pub email: Option<String>,
    pub password: String,
    pub language: String,
    pub region: String,
    pub timezone: String,
    pub keyboard_layout: String,
}

impl Default for UserConfig {
    fn default() -> Self {
        UserConfig {
            username: String::new(),
            full_name: String::new(),
            email: None,
            password: String::new(),
            language: "en_US".to_string(),
            region: "US".to_string(),
            timezone: "America/New_York".to_string(),
            keyboard_layout: "us".to_string(),
        }
    }
}

// ─── System Configuration ────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SystemConfig {
    pub theme_mode: String,
    pub auto_updates: bool,
    pub telemetry_enabled: bool,
    pub crash_reports: bool,
    pub accessibility_enabled: bool,
    pub screen_reader_enabled: bool,
    pub high_contrast_enabled: bool,
}

impl Default for SystemConfig {
    fn default() -> Self {
        SystemConfig {
            theme_mode: "auto".to_string(),
            auto_updates: true,
            telemetry_enabled: false,
            crash_reports: true,
            accessibility_enabled: false,
            screen_reader_enabled: false,
            high_contrast_enabled: false,
        }
    }
}

// ─── Onboarding Wizard State ─────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct OnboardingWizard {
    pub current_step: OnboardingStep,
    pub completed_steps: Vec<OnboardingStep>,
    pub user_config: UserConfig,
    pub system_config: SystemConfig,
    pub pages: HashMap<OnboardingStep, OnboardingPage>,
    pub initialized: bool,
    pub skipped: bool,
}

impl OnboardingWizard {
    pub fn new() -> Self {
        let mut wizard = OnboardingWizard {
            current_step: OnboardingStep::Welcome,
            completed_steps: vec![],
            user_config: UserConfig::default(),
            system_config: SystemConfig::default(),
            pages: HashMap::new(),
            initialized: false,
            skipped: false,
        };

        wizard.init_pages();
        wizard
    }

    fn init_pages(&mut self) {
        self.pages.insert(
            OnboardingStep::Welcome,
            OnboardingPage {
                step: OnboardingStep::Welcome,
                title: "Welcome to SigmaOS".to_string(),
                description: "Let's get you set up with a few quick steps.".to_string(),
                can_skip: false,
                can_go_back: false,
            },
        );

        self.pages.insert(
            OnboardingStep::Language,
            OnboardingPage {
                step: OnboardingStep::Language,
                title: "Language".to_string(),
                description: "Select your preferred language.".to_string(),
                can_skip: false,
                can_go_back: true,
            },
        );

        self.pages.insert(
            OnboardingStep::Region,
            OnboardingPage {
                step: OnboardingStep::Region,
                title: "Region".to_string(),
                description: "Select your region for localization.".to_string(),
                can_skip: false,
                can_go_back: true,
            },
        );

        self.pages.insert(
            OnboardingStep::Keyboard,
            OnboardingPage {
                step: OnboardingStep::Keyboard,
                title: "Keyboard".to_string(),
                description: "Select your keyboard layout.".to_string(),
                can_skip: true,
                can_go_back: true,
            },
        );

        self.pages.insert(
            OnboardingStep::Network,
            OnboardingPage {
                step: OnboardingStep::Network,
                title: "Network".to_string(),
                description: "Connect to a network to get started.".to_string(),
                can_skip: true,
                can_go_back: true,
            },
        );

        self.pages.insert(
            OnboardingStep::Privacy,
            OnboardingPage {
                step: OnboardingStep::Privacy,
                title: "Privacy".to_string(),
                description: "Configure your privacy settings.".to_string(),
                can_skip: false,
                can_go_back: true,
            },
        );

        self.pages.insert(
            OnboardingStep::Account,
            OnboardingPage {
                step: OnboardingStep::Account,
                title: "Create Account".to_string(),
                description: "Create your user account.".to_string(),
                can_skip: false,
                can_go_back: true,
            },
        );

        self.pages.insert(
            OnboardingStep::Theme,
            OnboardingPage {
                step: OnboardingStep::Theme,
                title: "Theme".to_string(),
                description: "Choose your preferred theme.".to_string(),
                can_skip: true,
                can_go_back: true,
            },
        );

        self.pages.insert(
            OnboardingStep::Accessibility,
            OnboardingPage {
                step: OnboardingStep::Accessibility,
                title: "Accessibility".to_string(),
                description: "Configure accessibility options.".to_string(),
                can_skip: true,
                can_go_back: true,
            },
        );

        self.pages.insert(
            OnboardingStep::Complete,
            OnboardingPage {
                step: OnboardingStep::Complete,
                title: "All Done!".to_string(),
                description: "Your system is ready to use.".to_string(),
                can_skip: false,
                can_go_back: true,
            },
        );
    }

    /// Initialize onboarding wizard
    pub fn init(&mut self) {
        self.initialized = true;
    }

    /// Get current page
    pub fn get_current_page(&self) -> Option<&OnboardingPage> {
        self.pages.get(&self.current_step)
    }

    /// Next step
    pub fn next_step(&mut self) -> bool {
        if !self.completed_steps.contains(&self.current_step) {
            self.completed_steps.push(self.current_step);
        }

        match self.current_step {
            OnboardingStep::Welcome => self.current_step = OnboardingStep::Language,
            OnboardingStep::Language => self.current_step = OnboardingStep::Region,
            OnboardingStep::Region => self.current_step = OnboardingStep::Keyboard,
            OnboardingStep::Keyboard => self.current_step = OnboardingStep::Network,
            OnboardingStep::Network => self.current_step = OnboardingStep::Privacy,
            OnboardingStep::Privacy => self.current_step = OnboardingStep::Account,
            OnboardingStep::Account => self.current_step = OnboardingStep::Theme,
            OnboardingStep::Theme => self.current_step = OnboardingStep::Accessibility,
            OnboardingStep::Accessibility => self.current_step = OnboardingStep::Complete,
            OnboardingStep::Complete => return false,
        }

        true
    }

    /// Previous step
    pub fn previous_step(&mut self) -> bool {
        match self.current_step {
            OnboardingStep::Welcome => return false,
            OnboardingStep::Language => self.current_step = OnboardingStep::Welcome,
            OnboardingStep::Region => self.current_step = OnboardingStep::Language,
            OnboardingStep::Keyboard => self.current_step = OnboardingStep::Region,
            OnboardingStep::Network => self.current_step = OnboardingStep::Keyboard,
            OnboardingStep::Privacy => self.current_step = OnboardingStep::Network,
            OnboardingStep::Account => self.current_step = OnboardingStep::Privacy,
            OnboardingStep::Theme => self.current_step = OnboardingStep::Account,
            OnboardingStep::Accessibility => self.current_step = OnboardingStep::Theme,
            OnboardingStep::Complete => self.current_step = OnboardingStep::Accessibility,
        }

        true
    }

    /// Skip current step
    pub fn skip_step(&mut self) -> bool {
        if let Some(page) = self.pages.get(&self.current_step) {
            if page.can_skip {
                self.next_step();
                return true;
            }
        }
        false
    }

    /// Jump to specific step
    pub fn jump_to_step(&mut self, step: OnboardingStep) -> bool {
        if self.pages.contains_key(&step) {
            self.current_step = step;
            true
        } else {
            false
        }
    }

    /// Set language
    pub fn set_language(&mut self, language: &str) {
        self.user_config.language = language.to_string();
    }

    /// Set region
    pub fn set_region(&mut self, region: &str) {
        self.user_config.region = region.to_string();
    }

    /// Set timezone
    pub fn set_timezone(&mut self, timezone: &str) {
        self.user_config.timezone = timezone.to_string();
    }

    /// Set keyboard layout
    pub fn set_keyboard_layout(&mut self, layout: &str) {
        self.user_config.keyboard_layout = layout.to_string();
    }

    /// Set username
    pub fn set_username(&mut self, username: &str) {
        self.user_config.username = username.to_string();
    }

    /// Set full name
    pub fn set_full_name(&mut self, full_name: &str) {
        self.user_config.full_name = full_name.to_string();
    }

    /// Set email
    pub fn set_email(&mut self, email: &str) {
        self.user_config.email = Some(email.to_string());
    }

    /// Set password
    pub fn set_password(&mut self, password: &str) {
        self.user_config.password = password.to_string();
    }

    /// Set theme mode
    pub fn set_theme_mode(&mut self, theme_mode: &str) {
        self.system_config.theme_mode = theme_mode.to_string();
    }

    /// Set auto updates
    pub fn set_auto_updates(&mut self, enabled: bool) {
        self.system_config.auto_updates = enabled;
    }

    /// Set telemetry
    pub fn set_telemetry(&mut self, enabled: bool) {
        self.system_config.telemetry_enabled = enabled;
    }

    /// Set crash reports
    pub fn set_crash_reports(&mut self, enabled: bool) {
        self.system_config.crash_reports = enabled;
    }

    /// Set accessibility enabled
    pub fn set_accessibility_enabled(&mut self, enabled: bool) {
        self.system_config.accessibility_enabled = enabled;
    }

    /// Set screen reader enabled
    pub fn set_screen_reader_enabled(&mut self, enabled: bool) {
        self.system_config.screen_reader_enabled = enabled;
    }

    /// Set high contrast enabled
    pub fn set_high_contrast_enabled(&mut self, enabled: bool) {
        self.system_config.high_contrast_enabled = enabled;
    }

    /// Validate current step
    pub fn validate_current_step(&self) -> bool {
        match self.current_step {
            OnboardingStep::Account => {
                !self.user_config.username.is_empty()
                    && !self.user_config.full_name.is_empty()
                    && !self.user_config.password.is_empty()
            }
            _ => true,
        }
    }

    /// Get progress percentage
    pub fn get_progress(&self) -> u8 {
        let total_steps = 10; // Welcome through Complete
        let current_index = match self.current_step {
            OnboardingStep::Welcome => 0,
            OnboardingStep::Language => 1,
            OnboardingStep::Region => 2,
            OnboardingStep::Keyboard => 3,
            OnboardingStep::Network => 4,
            OnboardingStep::Privacy => 5,
            OnboardingStep::Account => 6,
            OnboardingStep::Theme => 7,
            OnboardingStep::Accessibility => 8,
            OnboardingStep::Complete => 9,
        };

        ((current_index + 1) * 100 / total_steps) as u8
    }

    /// Check if onboarding is complete
    pub fn is_complete(&self) -> bool {
        self.current_step == OnboardingStep::Complete
    }

    /// Skip entire onboarding
    pub fn skip_all(&mut self) {
        self.skipped = true;
        self.current_step = OnboardingStep::Complete;
    }

    /// Check if onboarding was skipped
    pub fn was_skipped(&self) -> bool {
        self.skipped
    }

    /// Get user config
    pub fn get_user_config(&self) -> &UserConfig {
        &self.user_config
    }

    /// Get system config
    pub fn get_system_config(&self) -> &SystemConfig {
        &self.system_config
    }

    /// Apply configurations
    pub fn apply_configurations(&self) -> bool {
        // In real implementation, apply user and system configurations
        // Create user account, set timezone, configure theme, etc.
        true
    }

    /// Export configuration to JSON
    pub fn export_json(&self) -> String {
        // In real implementation, generate JSON representation
        r#"{"user":{}, "system":{}}"#.to_string()
    }

    /// Import configuration from JSON
    pub fn import_json(&mut self, json: &str) -> bool {
        // In real implementation, parse JSON and apply configurations
        true
    }

    /// Reset wizard
    pub fn reset(&mut self) {
        self.current_step = OnboardingStep::Welcome;
        self.completed_steps.clear();
        self.user_config = UserConfig::default();
        self.system_config = SystemConfig::default();
        self.skipped = false;
    }
}

// ─── Onboarding Manager ───────────────────────────────────────────────────────

pub struct OnboardingManager {
    pub wizard: OnboardingWizard,
    pub first_boot: bool,
}

impl OnboardingManager {
    pub fn new() -> Self {
        OnboardingManager {
            wizard: OnboardingWizard::new(),
            first_boot: true,
        }
    }

    pub fn init(&mut self) {
        self.wizard.init();
    }

    pub fn start_onboarding(&mut self) {
        self.wizard.current_step = OnboardingStep::Welcome;
    }

    pub fn complete_onboarding(&mut self) -> bool {
        if self.wizard.is_complete() {
            self.wizard.apply_configurations();
            self.first_boot = false;
            true
        } else {
            false
        }
    }

    pub fn is_first_boot(&self) -> bool {
        self.first_boot
    }
}
