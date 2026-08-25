// SigmaOS Automation Module
pub mod ai_optimizer;
pub mod orchestrator;
pub mod script;
pub mod system_level;

pub use script::ScriptArgumentRouter;

pub use ai_optimizer::{
    AiOptimizer, OptimizationCategory, OptimizationError, OptimizationRecommendation, SystemState,
};
pub use system_level::{
    AutomationError, PerformanceProfile, PredictiveModel, SystemAction, SystemAutomationManager,
    SystemAutomationRule, SystemEventType, SystemPrediction,
};
