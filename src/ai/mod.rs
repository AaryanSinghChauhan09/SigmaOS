// SigmaOS AI Module
// S-AI engine, agents, orchestrator, and local inference

pub mod agent;
pub mod orchestrator;
pub mod sai;

pub use agent::{Agent, AgentRole, AgentState};
pub use orchestrator::{AgentOrchestrator, Task, TaskStatus, TaskType};
pub use sai::{
    Agent as SaiAgent, AgentOrchestrator as SaiOrchestrator, AgentTask, AgentTask as SaiTask,
    AiError, ComputeBackend, LocalModel, ModelSize, SaiEngine, Tensor, TensorCore,
};
