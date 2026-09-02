// SigmaOS AI Module
// S-AI engine, agents, orchestrator, local inference, tensor memory, scheduler, quantization, and Agentic OS runtime

extern crate alloc;

pub mod agent;
pub mod agentic_os_runtime;
pub mod apm;
pub mod autogen;
pub mod compute_scheduler;
pub mod developer_platform;
pub mod lift_engine;
pub mod llm;
pub mod local_llm;
pub mod next_gen;
pub mod open_computer;
pub mod openclaw;
pub mod orchestrator;
pub mod perplexity;
pub mod quantization;
pub mod qwenpaw;
pub mod sai;
pub mod sigma_data;
pub mod sigma_logic;
pub mod system;
pub mod tensor_memory;
pub mod voice;
pub mod wandr;
pub mod wiki;

pub use local_llm::{
    LocalLlmWrapper, QuantizationType as LocalQuantizationType, WhisperSpeechToText,
};
pub use sigma_data::{KMeansClustering, PrincipalComponentAnalysis};

pub use agentic_os_runtime::{
    AgentAuditEvent, BootContainer, ContainerEngineType, ContextMemorySegment, ContextVirtualMmu,
    EbpfNetworkFilter, EphemeralAgentSandbox, GpuBackend, HybridContainerRuntime, LocalLlmDaemon,
    LocalLlmSystemDaemon, OmniAutomatorStudioApi, TamperProofActionAuditLog, TpmHardwareVault,
};

pub use openclaw::{AlertPlatform, ClawBackgroundDaemon, ClawChatIntegrator, ClawVoiceTranscriber};

pub use agent::{AIAgent, SimpleAIAgent};
pub use autogen::{
    AgentRole as AutoGenRole, AutoGenError, AutoGenMessage, AutoGenTool, ConversableAgent,
    GroupChat, SandboxCodeExecutor,
};
pub use developer_platform::{
    AiSafetyPolicyEngine, DefaultDenyNetworkPolicy, DeviceTarget, ExperimentRun,
    LocalLlmOrchestrator, MarketplaceModel, MlExperimentTracker, OpenShellAgentSandbox,
    PrivacyRouter, SignedModelMarketplace,
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
