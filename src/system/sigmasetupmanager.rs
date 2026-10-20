// SPDX-License-Identifier: MIT
// SigmaOS - Setup Manager
// Setup wizard management inspired by Omarchy setup-* commands and Linux setup systems

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Setup step status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupStepStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

/// Setup step
#[derive(Debug, Clone)]
pub struct SetupStep {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: SetupStepStatus,
    pub required: bool,
}

/// Setup wizard
#[derive(Debug, Clone)]
pub struct SetupWizard {
    pub id: String,
    pub name: String,
    pub steps: Vec<SetupStep>,
}

/// Setup manager
pub struct SigmaSetupManager {
    wizards: Vec<SetupWizard>,
}

impl SigmaSetupManager {
    pub fn new() -> Self {
        Self {
            wizards: Vec::new(),
        }
    }

    pub fn add_wizard(&mut self, wizard: SetupWizard) {
        self.wizards.push(wizard);
    }

    pub fn get_wizards(&self) -> &[SetupWizard] {
        &self.wizards
    }

    pub fn get_wizard(&self, id: &str) -> Option<&SetupWizard> {
        self.wizards.iter().find(|w| w.id == id)
    }

    pub fn start_step(&mut self, wizard_id: String, step_id: String) -> Result<(), String> {
        if let Some(wizard) = self.wizards.iter_mut().find(|w| w.id == wizard_id) {
            if let Some(step) = wizard.steps.iter_mut().find(|s| s.id == step_id) {
                step.status = SetupStepStatus::InProgress;
                Ok(())
            } else {
                Err(format!("Step {} not found", step_id))
            }
        } else {
            Err(format!("Wizard {} not found", wizard_id))
        }
    }

    pub fn complete_step(&mut self, wizard_id: String, step_id: String) -> Result<(), String> {
        if let Some(wizard) = self.wizards.iter_mut().find(|w| w.id == wizard_id) {
            if let Some(step) = wizard.steps.iter_mut().find(|s| s.id == step_id) {
                step.status = SetupStepStatus::Completed;
                Ok(())
            } else {
                Err(format!("Step {} not found", step_id))
            }
        } else {
            Err(format!("Wizard {} not found", wizard_id))
        }
    }

    pub fn get_pending_steps(&self, wizard_id: &str) -> Vec<&SetupStep> {
        self.get_wizard(wizard_id)
            .map(|w| {
                w.steps
                    .iter()
                    .filter(|s| s.status == SetupStepStatus::Pending)
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl Default for SigmaSetupManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_manager_creation() {
        let manager = SigmaSetupManager::new();
        assert_eq!(manager.get_wizards().len(), 0);
    }

    #[test]
    fn test_add_wizard() {
        let mut manager = SigmaSetupManager::new();
        let wizard = SetupWizard {
            id: "initial-setup".to_string(),
            name: "Initial Setup".to_string(),
            steps: Vec::new(),
        };
        manager.add_wizard(wizard);
        assert_eq!(manager.get_wizards().len(), 1);
    }

    #[test]
    fn test_start_step() {
        let mut manager = SigmaSetupManager::new();
        let mut steps = Vec::new();
        steps.push(SetupStep {
            id: "step1".to_string(),
            name: "Step 1".to_string(),
            description: "First step".to_string(),
            status: SetupStepStatus::Pending,
            required: true,
        });
        let wizard = SetupWizard {
            id: "initial-setup".to_string(),
            name: "Initial Setup".to_string(),
            steps,
        };
        manager.add_wizard(wizard);
        manager
            .start_step("initial-setup".to_string(), "step1".to_string())
            .unwrap();
        assert_eq!(
            manager.get_wizards()[0].steps[0].status,
            SetupStepStatus::InProgress
        );
    }

    #[test]
    fn test_complete_step() {
        let mut manager = SigmaSetupManager::new();
        let mut steps = Vec::new();
        steps.push(SetupStep {
            id: "step1".to_string(),
            name: "Step 1".to_string(),
            description: "First step".to_string(),
            status: SetupStepStatus::Pending,
            required: true,
        });
        let wizard = SetupWizard {
            id: "initial-setup".to_string(),
            name: "Initial Setup".to_string(),
            steps,
        };
        manager.add_wizard(wizard);
        manager
            .start_step("initial-setup".to_string(), "step1".to_string())
            .unwrap();
        manager
            .complete_step("initial-setup".to_string(), "step1".to_string())
            .unwrap();
        assert_eq!(
            manager.get_wizards()[0].steps[0].status,
            SetupStepStatus::Completed
        );
    }

    #[test]
    fn test_pending_steps() {
        let mut manager = SigmaSetupManager::new();
        let mut steps = Vec::new();
        steps.push(SetupStep {
            id: "step1".to_string(),
            name: "Step 1".to_string(),
            description: "First step".to_string(),
            status: SetupStepStatus::Completed,
            required: true,
        });
        steps.push(SetupStep {
            id: "step2".to_string(),
            name: "Step 2".to_string(),
            description: "Second step".to_string(),
            status: SetupStepStatus::Pending,
            required: true,
        });
        let wizard = SetupWizard {
            id: "initial-setup".to_string(),
            name: "Initial Setup".to_string(),
            steps,
        };
        manager.add_wizard(wizard);
        assert_eq!(manager.get_pending_steps("initial-setup").len(), 1);
    }
}
