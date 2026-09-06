// SigmaOS AI Module
// S-AI engine, agents, orchestrator, local inference, tensor memory, scheduler, and quantization

extern crate alloc;

pub mod agent;
pub mod apm;
pub mod autogen;
pub mod compute_scheduler;
pub mod developer_platform;
pub mod lift_engine;
pub mod llm;
pub mod next_gen;
pub mod open_computer;
pub mod openclaw;
pub mod orchestrator;
pub mod perplexity;
pub mod quantization;
pub mod qwenpaw;
pub mod sai;
pub mod system;
pub mod tensor_memory;
pub mod voice;
pub mod wandr;
pub mod wiki;

pub use openclaw::{AlertPlatform, ClawBackgroundDaemon, ClawChatIntegrator, ClawVoiceTranscriber};

pub use agent::{AIAgent, SimpleAIAgent};
pub use autogen::{
    AgentRole as AutoGenRole, AutoGenError, AutoGenMessage, AutoGenTool, ConversableAgent,
    GroupChat, SandboxCodeExecutor,
};
pub use llm::{
    BatchingStrategy, InferenceBackend, InferenceRequest, InferenceResponse, LlmConfig,
    LocalLlmEngine, QuantizationType, StreamingInference, StreamingLlmEngine,
};
pub use sai::{
    Agent as SaiAgent, AgentOrchestrator as SaiOrchestrator, AgentTask, AgentTask as SaiTask,
    AiError, ComputeBackend, LocalModel, ModelSize, SaiEngine, SovereignGpuAiAccelerator, Tensor,
    TensorCore,
};
pub use system::{
    AdaptiveSchedulingService, AiServiceConfig, AiServiceManager, AiServiceMetrics, AiServiceState,
    AiServiceType, AiSystemService, PredictiveMaintenanceService, ResourceManagementService,
    ServicePriority,
};
pub use voice::{
    AudioFormat, RecognitionResult, SynthesisModel, SynthesisResult, VoiceAssistant, VoiceModel,
    VoiceRecognizer, VoiceSynthesizer,
};
pub use developer_platform::{
    AiSafetyPolicyEngine, DefaultDenyNetworkPolicy, DeviceTarget, ExperimentRun,
    LocalLlmOrchestrator, MarketplaceModel, MlExperimentTracker, OpenShellAgentSandbox,
    PrivacyRouter, SignedModelMarketplace,
};
