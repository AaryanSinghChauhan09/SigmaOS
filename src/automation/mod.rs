// SigmaOS Automation Module
pub mod ai_optimizer;
pub mod system_level;

pub use ai_optimizer::{AiOptimizer, OptimizationRecommendation, SystemState, OptimizationCategory, OptimizationError};
pub use system_level::{SystemAutomationManager, SystemAutomationRule, SystemAction, SystemEventType, PerformanceProfile, SystemPrediction, PredictiveModel, AutomationError};
