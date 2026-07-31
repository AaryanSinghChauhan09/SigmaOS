// SigmaOS AI Module
// S-AI engine, agents, orchestrator, and local inference

pub mod agent;
pub mod llm;
pub mod orchestrator;
pub mod sai;
pub mod system;
pub mod voice;
pub mod qwenpaw;
pub mod perplexity;
pub mod awesome_ai;

pub use agent::{AIAgent, SimpleAIAgent};
pub use qwenpaw::{PawThreeLayerMemory, PawToolGuard, PawFileGuard, PawAgentCommunicationProtocol};
pub use perplexity::{PerplexitySearchCli, PerplexitySearchResult, PerplexitySnippetResult};
pub use awesome_ai::{AwesomeCodeAiRegistry, AwesomeToolInfo};
pub use llm::{
    LlmConfig, LocalLlmEngine, InferenceRequest, InferenceResponse,
    QuantizationType, InferenceBackend, BatchingStrategy,
    StreamingLlmEngine, StreamingInference,
    JaxTensorSharding, SwiGluActivation, GrokMoeRouter, RotaryPositionEmbedding,
    GrokGqaMapper, GrokWeightStreamer,
};
pub use orchestrator::{AgentOrchestrator, SimpleAgentOrchestrator, AgentState};
pub use sai::{
    Agent, AgentRole, AgentTask as Task, TaskStatus,
};
pub use sai::{
    Agent as SaiAgent, AgentOrchestrator as SaiOrchestrator, AgentTask, AgentTask as SaiTask,
    AiError, ComputeBackend, LocalModel, ModelSize, SaiEngine, Tensor, TensorCore,
};
pub use system::{
    AiSystemService, AiServiceManager, AiServiceConfig, AiServiceState,
    ResourceManagementService, PredictiveMaintenanceService, AdaptiveSchedulingService,
    AiServiceType, ServicePriority, AiServiceMetrics,
};
pub use voice::{
    VoiceAssistant, VoiceModel, VoiceRecognizer, VoiceSynthesizer,
    RecognitionResult, SynthesisResult, AudioFormat, SynthesisModel,
};
