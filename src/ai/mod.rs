// SigmaOS AI Module
// S-AI engine, agents, orchestrator, local inference, tensor memory, scheduler, quantization, and Agentic OS runtime

pub mod agent;
pub mod agentic_os_runtime;
pub mod apm;
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
pub mod open_computer;
pub mod tensor_memory;
pub mod compute_scheduler;
pub mod lift_engine;
pub mod next_gen;
pub mod perplexity;
pub mod quantization;
pub mod wandr;

pub use openclaw::{
    ClawBackgroundDaemon, ClawVoiceTranscriber, ClawChatIntegrator, AlertPlatform,
};

pub use agent::{AIAgent, SimpleAIAgent};
pub use agentic_os_runtime::{
    AgentAuditRecord, ContainerConfig, ContainerFirstRuntimeHost, ContainerState, ContextTokenType,
    DeterministicAgentSandbox, EbpfTraceEvent, EbpfTraceEventType, EbpfTracingMonitor, LocalLlmModel,
    LocalLlmSystemDaemon, OmniAutomatorStudioApi, PosixNativeBridgeLayer, TamperProofAgentAuditLogger,
    TpmTokenKeyVault, VectorContextBlock, VectorContextMmu,
};
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
    AiError, ComputeBackend, LocalModel, ModelSize, SaiEngine, Tensor, TensorCore,
    SovereignGpuAiAccelerator,
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
    LocalLlmOrchestrator, ExperimentRun, MlExperimentTracker, AiSafetyPolicyEngine, MarketplaceModel, SignedModelMarketplace,
};
