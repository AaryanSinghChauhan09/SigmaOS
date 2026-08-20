// SigmaOS AI Module
// S-AI engine, agents, orchestrator, and local inference

pub mod agent;
pub mod autogen;
pub mod llm;
pub mod orchestrator;
pub mod sai;
pub mod openclaw;
pub mod system;
pub mod voice;
pub mod wiki;
pub mod qwenpaw;
pub mod developer_platform;

pub use openclaw::{
    ClawBackgroundDaemon, ClawVoiceTranscriber, ClawChatIntegrator, AlertPlatform,
};

pub use agent::{AIAgent, SimpleAIAgent};
pub use autogen::{
    AgentRole as AutoGenRole, AutoGenError, AutoGenMessage, AutoGenTool, ConversableAgent,
    GroupChat, SandboxCodeExecutor,
};
pub use llm::{
    BatchingStrategy, InferenceBackend, InferenceRequest, InferenceResponse, LlmConfig,
    LocalLlmEngine, QuantizationType, StreamingInference, StreamingLlmEngine,
};
pub use orchestrator::{AgentOrchestrator, AgentState, SimpleAgentOrchestrator};
pub use sai::{
    Agent as SaiAgent, AgentOrchestrator as SaiOrchestrator, AgentTask, AgentTask as SaiTask,
    AiError, ComputeBackend, LocalModel, ModelSize, SaiEngine, Tensor, TensorCore,
    SovereignGpuAiAccelerator,
};
pub use sai::{Agent, AgentRole, AgentTask as Task, TaskStatus};
pub use system::{
    AdaptiveSchedulingService, AiServiceConfig, AiServiceManager, AiServiceMetrics, AiServiceState,
    AiServiceType, AiSystemService, PredictiveMaintenanceService, ResourceManagementService,
    ServicePriority,
};
pub use voice::{
    AudioFormat, RecognitionResult, SynthesisModel, SynthesisResult, VoiceAssistant, VoiceModel,
    VoiceRecognizer, VoiceSynthesizer,
};
pub use wiki::{SovereignWikiEngine, WikiArticle};
pub use qwenpaw::{
    PawThreeLayerMemory, PawToolGuard, PawFileGuard, PawAgentMessage,
    PawAgentCommunicationProtocol, SemanticSkillFunction, NativeSkillFunction,
    SovereignSkillKernel,
};
pub use developer_platform::{
    AiSafetyGuardrails, CuratedAiModel, DevWorkspace, DeveloperPlatformSuite,
    MlExperimentRun, MlExperimentTracker, ModelMarketplace, SafetyViolationType,
};
