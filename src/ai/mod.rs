<<<<<<< HEAD
<<<<<<< HEAD
||||||| 23ef22a4a
=======
// SigmaOS AI-Native Subsystem Module

>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
pub mod agent;
pub mod orchestrator;
pub mod sai;
pub mod system;
pub mod voice;
pub mod open_computer;

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
pub use open_computer::{
    OpenComputerVirtualMachine, MachineState, Qcow2Overlay, A11yWidget,
    AgentA11yInterface, HumanInTheLoopController, AgentMemoryInspector,
};
||||||| 43be3a7e8
=======
pub mod agent;
pub mod orchestrator;
>>>>>>> origin/fix/mem-leak-custom-vec-drop-7188808108065826003
