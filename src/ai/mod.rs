// SigmaOS AI-Native Subsystem Module

pub mod agent;
pub mod orchestrator;
pub mod wiki;
pub mod apm;

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
pub use apm::{
    ApmDependency, ApmLockfile, ApmManifest, ApmPolicy, ApmStatus, DependencySource, McpServer,
    SovereignApmEngine,
};
