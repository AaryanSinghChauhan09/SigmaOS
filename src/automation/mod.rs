// SigmaOS Automation Module
pub mod ai_optimizer;
pub mod system_level;
pub mod orchestrator;

pub use ai_optimizer::{
    AiOptimizer, OptimizationCategory, OptimizationError, OptimizationRecommendation, SystemState,
};
pub use orchestrator::{
    ActionPriority, ActionType, AiOptimizationStrategy, AiOrchestrator, MlOptimizer,
    OptimizationError as OrchestratorError,
    OptimizationRecommendation as OrchestratorRecommendation, PredictiveModel as OrchestratorModel,
    RuleBasedOptimizer, SystemAction as OrchestratorAction, SystemState as OrchestratorState,
};
pub use system_level::{
    AutomationError, PerformanceProfile, PredictiveModel, SystemAction, SystemAutomationManager,
    SystemAutomationRule, SystemEventType, SystemPrediction,
};
