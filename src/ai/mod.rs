// SigmaOS AI-Native Subsystem Module

pub mod agent;
pub mod orchestrator;
pub mod sai;
pub mod system;
pub mod voice;
pub mod lift_engine;

pub use lift_engine::{FieldType, ExtractionSchema, Citation, ExtractionResult, LiftError, DocumentExtractor};

pub use agent::{
    AIAgent, AIAgentManager, AIError, AIStats, AgentCapability, AgentInfo, Intent, IntentType,
    ManagerCapability, Pattern, SimpleAIAgent, SimpleAIAgentManager,
};
pub use orchestrator::{
    AIAgent as OrchestratorAIAgent, AgentCommunication, AgentError, AgentID, AgentOrchestrator,
    AgentState as OrchestratorAgentState, SimpleAIAgent as SimpleOrchestratorAgent,
    SimpleAgentCommunication, SimpleAgentOrchestrator, SimpleTaskQueue, TaskQueue,
};
pub use wiki::{SovereignWikiEngine, WikiArticle};
