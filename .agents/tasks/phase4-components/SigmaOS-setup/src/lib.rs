//! SigmaOS Setup — first-run configuration wizard.
//!
//! Guides the user through initial system profile selection, hardware
//! configuration, desktop defaults, and first-boot validation.

#![allow(dead_code)]

// ---------------------------------------------------------------------------
// Setup step
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupStep {
    Welcome,
    Language,
    Timezone,
    User,
    DesktopProfile,
    HardwareProfile,
    NetworkSetup,
    ApplyDefaults,
    Validation,
    Done,
}

impl SetupStep {
    pub fn next(self) -> Option<Self> {
        match self {
            Self::Welcome => Some(Self::Language),
            Self::Language => Some(Self::Timezone),
            Self::Timezone => Some(Self::User),
            Self::User => Some(Self::DesktopProfile),
            Self::DesktopProfile => Some(Self::HardwareProfile),
            Self::HardwareProfile => Some(Self::NetworkSetup),
            Self::NetworkSetup => Some(Self::ApplyDefaults),
            Self::ApplyDefaults => Some(Self::Validation),
            Self::Validation => Some(Self::Done),
            Self::Done => None,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Welcome => "Welcome to SigmaOS",
            Self::Language => "Select Language",
            Self::Timezone => "Set Timezone",
            Self::User => "Create User Account",
            Self::DesktopProfile => "Choose Desktop Profile",
            Self::HardwareProfile => "Hardware Configuration",
            Self::NetworkSetup => "Network Setup",
            Self::ApplyDefaults => "Applying Defaults",
            Self::Validation => "Validation",
            Self::Done => "Setup Complete",
        }
    }
}

// ---------------------------------------------------------------------------
// Setup state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct SetupConfig {
    pub language: String,
    pub timezone: String,
    pub username: String,
    pub desktop_profile: String,
    pub hardware_profile: String,
    pub network_configured: bool,
}

impl Default for SetupConfig {
    fn default() -> Self {
        Self {
            language: "en_US.UTF-8".into(),
            timezone: "UTC".into(),
            username: String::new(),
            desktop_profile: "balanced".into(),
            hardware_profile: "auto".into(),
            network_configured: false,
        }
    }
}

pub struct SetupWizard {
    pub current_step: SetupStep,
    pub config: SetupConfig,
    completed_steps: Vec<SetupStep>,
}

impl SetupWizard {
    pub fn new() -> Self {
        Self { current_step: SetupStep::Welcome, config: SetupConfig::default(), completed_steps: Vec::new() }
    }

    pub fn advance(&mut self) -> Option<SetupStep> {
        self.completed_steps.push(self.current_step);
        if let Some(next) = self.current_step.next() {
            self.current_step = next;
            Some(next)
        } else {
            None
        }
    }

    pub fn is_complete(&self) -> bool { self.current_step == SetupStep::Done }

    pub fn steps_completed(&self) -> usize { self.completed_steps.len() }

    pub fn progress_pct(&self) -> u32 {
        (self.completed_steps.len() as u32 * 100) / 9 // 9 non-done steps
    }
}

impl Default for SetupWizard {
    fn default() -> Self { Self::new() }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wizard_advances_through_all_steps() {
        let mut wiz = SetupWizard::new();
        let mut count = 0;
        while !wiz.is_complete() {
            wiz.advance();
            count += 1;
            assert!(count < 20, "infinite loop guard");
        }
        assert!(count >= 9);
    }

    #[test]
    fn step_labels_non_empty() {
        let mut step = SetupStep::Welcome;
        loop {
            assert!(!step.label().is_empty());
            match step.next() { Some(s) => step = s, None => break }
        }
    }

    #[test]
    fn progress_increases() {
        let mut wiz = SetupWizard::new();
        let mut last = 0u32;
        while !wiz.is_complete() {
            let p = wiz.progress_pct();
            assert!(p >= last);
            last = p;
            wiz.advance();
        }
    }
}
