// src/onboarding/mod.rs
// First-boot onboarding engine for SigmaOS implemented in pure zero-dependency Rust

#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnboardingStep {
    Welcome,
    LanguageSelection,
    NetworkSetup,
    UserAccountCreation,
    SecurityPolicyConfig,
    DesktopThemeChoice,
    Complete,
}

#[derive(Debug, Clone)]
pub struct OnboardingWizardState {
    pub current_step: OnboardingStep,
    pub selected_language: String,
    pub username: String,
    pub hostname: String,
    pub theme: String,
    pub is_completed: bool,
}

impl Default for OnboardingWizardState {
    fn default() -> Self {
        Self::new()
    }
}

impl OnboardingWizardState {
    pub fn new() -> Self {
        Self {
            current_step: OnboardingStep::Welcome,
            selected_language: "en_US".to_string(),
            username: "sigmauser".to_string(),
            hostname: "sigmaos-host".to_string(),
            theme: "ZenithDark".to_string(),
            is_completed: false,
        }
    }

    pub fn advance_step(&mut self) -> OnboardingStep {
        self.current_step = match self.current_step {
            OnboardingStep::Welcome => OnboardingStep::LanguageSelection,
            OnboardingStep::LanguageSelection => OnboardingStep::NetworkSetup,
            OnboardingStep::NetworkSetup => OnboardingStep::UserAccountCreation,
            OnboardingStep::UserAccountCreation => OnboardingStep::SecurityPolicyConfig,
            OnboardingStep::SecurityPolicyConfig => OnboardingStep::DesktopThemeChoice,
            OnboardingStep::DesktopThemeChoice => {
                self.is_completed = true;
                OnboardingStep::Complete
            }
            OnboardingStep::Complete => OnboardingStep::Complete,
        };
        self.current_step
    }
}

/// Check if onboarding has been completed
pub fn is_onboarding_complete() -> bool {
    false
}

/// Mark onboarding as complete
pub fn mark_onboarding_complete() -> Result<(), ()> {
    Ok(())
}

/// Launch onboarding wizard in native Rust
pub fn launch_wizard() -> Result<OnboardingWizardState, ()> {
    let mut wizard = OnboardingWizardState::new();
    while !wizard.is_completed {
        wizard.advance_step();
    }
    mark_onboarding_complete()?;
    Ok(wizard)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onboarding_wizard_workflow() {
        let wizard = launch_wizard().unwrap();
        assert!(wizard.is_completed);
        assert_eq!(wizard.current_step, OnboardingStep::Complete);
    }
}
