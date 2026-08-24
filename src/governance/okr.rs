// SPDX-License-Identifier: MIT
//! Strategic OKR Engine & Milestone Evaluation Subsystem for SigmaOS

// #![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Strategic evaluation error states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OkrError {
    Success = 0,
    MilestoneNotFound = 1,
    DuplicateMilestone = 2,
    MetricOutOfRange = 3,
}

/// Strategic milestone categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MilestoneCategory {
    CoreKernel,
    AiOrchestration,
    DeveloperExperience,
    SecurityEnterprise,
}

/// Roadmap milestone
#[derive(Debug, Clone, PartialEq)]
pub struct StrategicMilestone {
    pub id: u32,
    pub title: String,
    pub category: MilestoneCategory,
    pub completion_percentage: f64, // 0.0 to 100.0
}

/// Base OOP interface representing any strategic tracker
pub trait OkrTracker {
    fn name(&self) -> &str;
    fn evaluate_progress(&self) -> f64;
}

// ==========================================
// 1. Concrete OKR Evaluator Implementation
// ==========================================

#[derive(Debug, Clone)]
pub struct StrategicOkrEvaluator {
    pub milestones: Vec<StrategicMilestone>,
}

impl StrategicOkrEvaluator {
    pub fn new() -> Self {
        let mut evaluator = StrategicOkrEvaluator {
            milestones: Vec::new(),
        };
        evaluator.register_milestone(
            1,
            "Phase G Kernel".to_string(),
            MilestoneCategory::CoreKernel,
            100.0,
        );
        evaluator.register_milestone(
            2,
            "Local AI Serving".to_string(),
            MilestoneCategory::AiOrchestration,
            100.0,
        );
        evaluator.register_milestone(
            3,
            "Dev Studio".to_string(),
            MilestoneCategory::DeveloperExperience,
            100.0,
        );
        evaluator
    }

    pub fn register_milestone(
        &mut self,
        id: u32,
        title: String,
        category: MilestoneCategory,
        progress: f64,
    ) {
        let milestone = StrategicMilestone {
            id,
            title,
            category,
            completion_percentage: progress.clamp(0.0, 100.0),
        };
        self.milestones.push(milestone);
    }

    pub fn compute_roadmap_completion(&self) -> f64 {
        if self.milestones.is_empty() {
            return 100.0;
        }
        let sum: f64 = self.milestones.iter().map(|m| m.completion_percentage).sum();
        sum / self.milestones.len() as f64
    }

    pub fn get_milestones_by_category(&self, category: MilestoneCategory) -> Vec<&StrategicMilestone> {
        self.milestones.iter().filter(|m| m.category == category).collect()
    }

    pub fn get_milestone(&self, id: u32) -> Option<&StrategicMilestone> {
        self.milestones.iter().find(|m| m.id == id)
    }

    pub fn remove_milestone(&mut self, id: u32) -> Result<StrategicMilestone, OkrError> {
        if let Some(pos) = self.milestones.iter().position(|m| m.id == id) {
            Ok(self.milestones.remove(pos))
        } else {
            Err(OkrError::MilestoneNotFound)
        }
    }
}

impl Default for StrategicOkrEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl OkrTracker for StrategicOkrEvaluator {
    fn name(&self) -> &str {
        "StrategicOkrEvaluator"
    }

    fn evaluate_progress(&self) -> f64 {
        self.compute_roadmap_completion()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_okr_evaluator_default_init() {
        let evaluator = StrategicOkrEvaluator::new();
        assert_eq!(evaluator.milestones.len(), 3);
        assert_eq!(evaluator.compute_roadmap_completion(), 100.0);
        assert_eq!(evaluator.name(), "StrategicOkrEvaluator");
        assert_eq!(evaluator.evaluate_progress(), 100.0);
    }

    #[test]
    fn test_okr_register_milestone_clamp() {
        let mut evaluator = StrategicOkrEvaluator::new();
        evaluator.register_milestone(
            4,
            "Security Audit".to_string(),
            MilestoneCategory::SecurityEnterprise,
            150.0, // Should be clamped to 100.0
        );
        let m = evaluator.get_milestone(4).unwrap();
        assert_eq!(m.completion_percentage, 100.0);
        assert_eq!(m.category, MilestoneCategory::SecurityEnterprise);

        evaluator.register_milestone(
            5,
            "Negative Progress".to_string(),
            MilestoneCategory::CoreKernel,
            -50.0, // Should be clamped to 0.0
        );
        let m5 = evaluator.get_milestone(5).unwrap();
        assert_eq!(m5.completion_percentage, 0.0);
    }

    #[test]
    fn test_okr_get_by_category() {
        let evaluator = StrategicOkrEvaluator::new();
        let kernel_milestones = evaluator.get_milestones_by_category(MilestoneCategory::CoreKernel);
        assert_eq!(kernel_milestones.len(), 1);
        assert_eq!(kernel_milestones[0].id, 1);
    }

    #[test]
    fn test_okr_remove_milestone() {
        let mut evaluator = StrategicOkrEvaluator::new();
        assert!(evaluator.remove_milestone(1).is_ok());
        assert_eq!(evaluator.milestones.len(), 2);
        assert_eq!(evaluator.remove_milestone(99), Err(OkrError::MilestoneNotFound));
    }

    #[test]
    fn test_empty_evaluator_completion() {
        let evaluator = StrategicOkrEvaluator {
            milestones: Vec::new(),
        };
        assert_eq!(evaluator.compute_roadmap_completion(), 100.0);
    }

    #[test]
    fn test_partial_progress_completion() {
        let mut evaluator = StrategicOkrEvaluator {
            milestones: Vec::new(),
        };
        evaluator.register_milestone(1, "M1".to_string(), MilestoneCategory::CoreKernel, 50.0);
        evaluator.register_milestone(2, "M2".to_string(), MilestoneCategory::AiOrchestration, 75.0);
        assert_eq!(evaluator.compute_roadmap_completion(), 62.5);
    }

    struct CustomOkrTracker;
    impl OkrTracker for CustomOkrTracker {
        fn name(&self) -> &str {
            "CustomOkrTracker"
        }
        fn evaluate_progress(&self) -> f64 {
            88.5
        }
    }

    #[test]
    fn test_custom_okr_tracker_trait() {
        let tracker = CustomOkrTracker;
        assert_eq!(tracker.name(), "CustomOkrTracker");
        assert_eq!(tracker.evaluate_progress(), 88.5);
    }
}
