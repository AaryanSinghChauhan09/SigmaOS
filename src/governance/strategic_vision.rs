// SigmaOS Strategic Vision Roadmap & OKR Engine
// Pure Rust implementation of 3-Year Strategic Vision and Milestone Evaluators.

// #![no_std]

extern crate alloc;

#[cfg(not(feature = "standalone_test"))]
use alloc::{vec::Vec, string::{String, ToString}};

#[cfg(feature = "standalone_test")]
extern crate std;

#[cfg(feature = "standalone_test")]
use alloc::{vec::Vec, string::{String, ToString}};

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
#[derive(Debug, Clone)]
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

pub struct StrategicOkrEvaluator {
    pub milestones: Vec<StrategicMilestone>,
}

impl StrategicOkrEvaluator {
    pub fn new() -> Self {
        let mut evaluator = StrategicOkrEvaluator { milestones: Vec::new() };
        evaluator.register_milestone(1, "Phase G Kernel".to_string(), MilestoneCategory::CoreKernel, 100.0);
        evaluator.register_milestone(2, "Local AI Serving".to_string(), MilestoneCategory::AiOrchestration, 100.0);
        evaluator.register_milestone(3, "Dev Studio".to_string(), MilestoneCategory::DeveloperExperience, 100.0);
        evaluator
    }

    pub fn register_milestone(&mut self, id: u32, title: String, category: MilestoneCategory, progress: f64) {
        let clamped_progress = if progress < 0.0 {
            0.0
        } else if progress > 100.0 {
            100.0
        } else {
            progress
        };
        let milestone = StrategicMilestone {
            id,
            title,
            category,
            completion_percentage: clamped_progress,
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
}

impl Default for StrategicOkrEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl OkrTracker for StrategicOkrEvaluator {
    fn name(&self) -> &str {
        "SigmaOS 3-Year Strategic Vision Tracker"
    }

    fn evaluate_progress(&self) -> f64 {
        self.compute_roadmap_completion()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategic_okr_evaluator() {
        let mut evaluator = StrategicOkrEvaluator::new();
        assert_eq!(evaluator.milestones.len(), 3);
        assert_eq!(evaluator.compute_roadmap_completion(), 100.0);

        evaluator.register_milestone(4, "Enterprise AD/LDAP".to_string(), MilestoneCategory::SecurityEnterprise, 50.0);
        assert_eq!(evaluator.milestones.len(), 4);
        assert_eq!(evaluator.compute_roadmap_completion(), 87.5); // (100+100+100+50)/4
    }
}
