// SigmaOS AI-Native Subsystem Module

pub mod agent;
pub mod orchestrator;
pub mod wiki;

pub use agent::{
    AIAgent, AIAgentManager, AIError, AIStats, AgentCapability, AgentInfo, Intent, IntentType,
    ManagerCapability, Pattern, SimpleAIAgent, SimpleAIAgentManager,
};
pub use orchestrator::{
    LocalLlmOrchestrator, ModelResource, DeviceTarget, OrchestratorError, ContextWindowPruner,
};
pub use wiki::{SovereignWikiEngine, WikiArticle};
