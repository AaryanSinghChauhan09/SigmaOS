#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
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
    AutomatedRacctPolicy, AutomatedSandboxPolicy, AutomationError, DeclarativeSpecState,
    DistroInspiredAutomationEngine, PerformanceProfile, PredictiveModel, SupervisedService,
    SupervisedServiceState, SystemAction, SystemAutomationManager, SystemAutomationRule,
    SystemEventType, SystemPrediction, TieredStorageExtent, TransactionalAutomationHook,
};
