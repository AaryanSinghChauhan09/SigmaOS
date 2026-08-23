// SigmaOS AI Module
// S-AI engine, agents, orchestrator, local inference, tensor memory, scheduler, and quantization

pub mod agent;
pub mod apm;
pub mod autogen;
pub mod autonomous_agents;
pub mod awesome_ai;
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
pub mod runtime;
pub mod sai;
pub mod sigma_ai_suite;
pub mod system;
pub mod system_ai;
pub mod tensor_memory;
pub mod voice;
pub mod wandr;
pub mod wiki;

pub use sigma_ai_suite::{
    CommandTranslation, ErrorExplanation, IndicLanguage, SafetyLevel, SigmaAiAdaptiveSuggestions,
    SigmaAiAssistant, SigmaAiErrorExplanation, SigmaAiNaturalLanguageTranslator,
    SigmaAiWorkflowAutomation, WorkflowStep, WorkflowTrigger,
};

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
pub use open_computer::{
    OpenComputerVirtualMachine, MachineState, Qcow2Overlay, A11yWidget,
    AgentA11yInterface, HumanInTheLoopController, AgentMemoryInspector,
};
pub use tensor_memory::{
    AiTensorMemoryManager, MemoryPinMode, TensorBuffer, TensorDtype, TensorMemoryStats,
};
pub use compute_scheduler::{
    AiComputeQuota, AiComputeScheduler, AiComputeTask, AiTaskPriority, AiTaskState, ComputeDeviceTarget,
};
pub use quantization::{
    AiExecutionDispatcher, DeviceFallbackRoute, QuantizedMatrix,
};
