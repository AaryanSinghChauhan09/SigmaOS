// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

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
    pub dependencies: Vec<String>,
}

/// Setup wizard for system configuration
/// Inspired by Omarchy setup- commands and Linux firstboot
#[derive(Debug, Clone)]
pub struct SigmaSetupManager2 {
    steps: Vec<SetupStep>,
    current_step: Option<String>,
}

impl SigmaSetupManager2 {
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            current_step: None,
        }
    }

    /// Add a setup step
    pub fn add_step(&mut self, step: SetupStep) {
        self.steps.push(step);
    }

    /// Get step by ID
    pub fn get_step(&self, id: &str) -> Option<&SetupStep> {
        self.steps.iter().find(|s| s.id == id)
    }

    /// Get all steps
    pub fn get_steps(&self) -> &[SetupStep] {
        &self.steps
    }

    /// Start a setup step
    pub fn start_step(&mut self, id: &str) -> Result<String, String> {
        if let Some(step) = self.steps.iter_mut().find(|s| s.id == id) {
            step.status = SetupStepStatus::InProgress;
            self.current_step = Some(id.to_string());
            Ok(format!("Started step: {}", step.name))
        } else {
            Err(format!("Step not found: {}", id))
        }
    }

    /// Complete a setup step
    pub fn complete_step(&mut self, id: &str) -> Result<String, String> {
        if let Some(step) = self.steps.iter_mut().find(|s| s.id == id) {
            step.status = SetupStepStatus::Completed;
            Ok(format!("Completed step: {}", step.name))
        } else {
            Err(format!("Step not found: {}", id))
        }
    }

    /// Fail a setup step
    pub fn fail_step(&mut self, id: &str) -> Result<String, String> {
        if let Some(step) = self.steps.iter_mut().find(|s| s.id == id) {
            step.status = SetupStepStatus::Failed;
            Ok(format!("Step failed: {}", step.name))
        } else {
            Err(format!("Step not found: {}", id))
        }
    }

    /// Skip a setup step
    pub fn skip_step(&mut self, id: &str) -> Result<String, String> {
        if let Some(step) = self.steps.iter_mut().find(|s| s.id == id) {
            step.status = SetupStepStatus::Skipped;
            Ok(format!("Skipped step: {}", step.name))
        } else {
            Err(format!("Step not found: {}", id))
        }
    }

    /// Get current step
    pub fn get_current_step(&self) -> Option<&SetupStep> {
        self.current_step
            .as_ref()
            .and_then(|id| self.get_step(id))
    }

    /// Get pending steps
    pub fn get_pending_steps(&self) -> Vec<&SetupStep> {
        self.steps
            .iter()
            .filter(|s| s.status == SetupStepStatus::Pending)
            .collect()
    }

    /// Get completed steps
    pub fn get_completed_steps(&self) -> Vec<&SetupStep> {
        self.steps
            .iter()
            .filter(|s| s.status == SetupStepStatus::Completed)
            .collect()
    }

    /// Check if setup is complete
    pub fn is_setup_complete(&self) -> bool {
        self.steps
            .iter()
            .all(|s| s.status == SetupStepStatus::Completed || s.status == SetupStepStatus::Skipped)
    }

    /// Get next pending step
    pub fn get_next_step(&self) -> Option<&SetupStep> {
        self.steps
            .iter()
            .find(|s| s.status == SetupStepStatus::Pending)
    }
}

impl Default for SigmaSetupManager2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_step() {
        let mut manager = SigmaSetupManager2::new();
        let step = SetupStep {
            id: "locale".to_string(),
            name: "Locale Selection".to_string(),
            description: "Select system locale".to_string(),
            status: SetupStepStatus::Pending,
            dependencies: Vec::new(),
        };
        manager.add_step(step);
        assert_eq!(manager.get_steps().len(), 1);
    }

    #[test]
    fn test_start_step() {
        let mut manager = SigmaSetupManager2::new();
        let step = SetupStep {
            id: "locale".to_string(),
            name: "Locale Selection".to_string(),
            description: "Select system locale".to_string(),
            status: SetupStepStatus::Pending,
            dependencies: Vec::new(),
        };
        manager.add_step(step);
        manager.start_step("locale").unwrap();
        assert_eq!(
            manager.get_step("locale").unwrap().status,
            SetupStepStatus::InProgress
        );
    }

    #[test]
    fn test_complete_step() {
        let mut manager = SigmaSetupManager2::new();
        let step = SetupStep {
            id: "locale".to_string(),
            name: "Locale Selection".to_string(),
            description: "Select system locale".to_string(),
            status: SetupStepStatus::InProgress,
            dependencies: Vec::new(),
        };
        manager.add_step(step);
        manager.complete_step("locale").unwrap();
        assert_eq!(
            manager.get_step("locale").unwrap().status,
            SetupStepStatus::Completed
        );
    }

    #[test]
    fn test_skip_step() {
        let mut manager = SigmaSetupManager2::new();
        let step = SetupStep {
            id: "locale".to_string(),
            name: "Locale Selection".to_string(),
            description: "Select system locale".to_string(),
            status: SetupStepStatus::Pending,
            dependencies: Vec::new(),
        };
        manager.add_step(step);
        manager.skip_step("locale").unwrap();
        assert_eq!(
            manager.get_step("locale").unwrap().status,
            SetupStepStatus::Skipped
        );
    }

    #[test]
    fn test_is_setup_complete() {
        let mut manager = SigmaSetupManager2::new();
        let step1 = SetupStep {
            id: "locale".to_string(),
            name: "Locale Selection".to_string(),
            description: "Select system locale".to_string(),
            status: SetupStepStatus::Completed,
            dependencies: Vec::new(),
        };
        let step2 = SetupStep {
            id: "timezone".to_string(),
            name: "Timezone Selection".to_string(),
            description: "Select system timezone".to_string(),
            status: SetupStepStatus::Skipped,
            dependencies: Vec::new(),
        };
        manager.add_step(step1);
        manager.add_step(step2);
        assert!(manager.is_setup_complete());
    }

    #[test]
    fn test_get_next_step() {
        let mut manager = SigmaSetupManager2::new();
        let step1 = SetupStep {
            id: "locale".to_string(),
            name: "Locale Selection".to_string(),
            description: "Select system locale".to_string(),
            status: SetupStepStatus::Completed,
            dependencies: Vec::new(),
        };
        let step2 = SetupStep {
            id: "timezone".to_string(),
            name: "Timezone Selection".to_string(),
            description: "Select system timezone".to_string(),
            status: SetupStepStatus::Pending,
            dependencies: Vec::new(),
        };
        manager.add_step(step1);
        manager.add_step(step2);
        assert!(manager.get_next_step().is_some());
    }
}
