#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Automation Module
pub mod ai_optimizer;
pub mod orchestrator;
pub mod system_level;

pub use ai_optimizer::{
    AiOptimizer, OptimizationCategory, OptimizationError, OptimizationRecommendation, SystemState,
};
pub use system_level::{
    AutomationError, PerformanceProfile, PredictiveModel, SystemAction, SystemAutomationManager,
    SystemAutomationRule, SystemEventType, SystemPrediction,
};
