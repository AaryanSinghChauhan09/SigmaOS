// Local LLM Orchestrator for SigmaOS
// Dynamically schedules models, checks device bounds, and prunes context windows.

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::Write;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type AgentID = u64;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceTarget {
    Cpu = 0,
    Gpu = 1,
    Tpu = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState { Idle = 0, Active = 1, Busy = 2, Error = 3, Learning = 4 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentError { Success = 0, NotFound = 1, ExecutionFailed = 2, Timeout = 3, InvalidInput = 4 }

pub trait AIAgent {
    fn id(&self) -> AgentID;
    fn name(&self) -> &str;
    fn state(&self) -> AgentState;
    fn execute(&mut self, task: &[u8]) -> Result<Vec<u8>, AgentError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrchestratorError {
    Success = 0,
    OutOfMemory = 1,
    ModelNotFound = 2,
    LimitExceeded = 3,
}

pub struct ModelResource {
    pub name: [u8; 32],
    pub memory_required_mb: usize,
    pub target: DeviceTarget,
}

impl ModelResource {
    pub fn new(name: &[u8], memory_required_mb: usize, target: DeviceTarget) -> Self {
        let mut name_array = [0u8; 32];
        let len = name.len().min(31);
        name_array[..len].copy_from_slice(&name[..len]);
        ModelResource {
            name: name_array,
            memory_required_mb,
            target,
        }
    }
}

/// Type representing different local model sizes managed by the S-AI Engine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalModelSize {
    Tiny1B,   // DeepSeek-R1-Distill-1.5B equivalent (Fast, low-latency, headless tools)
    Medium8B, // LLaMA-3-8B / Qwen-2.5-7B equivalent (Analytical reasoning, complex logic)
    Large70B, // DeepSeek-V3 MoE / LLaMA-70B equivalent (Highly complex mathematical or coding tasks)
}

/// A target agent profile managed by the multi-agent task planner
#[derive(Debug, Clone)]
pub struct AIOSAgent {
    pub name: String,
    pub role: String,
    pub system_instructions: String,
    pub primary_model: LocalModelSize,
}

/// Represents an active multi-agent plan routed dynamically across model constraints
pub struct SovereignMultiAgentPlanner {
    agents: Vec<AIOSAgent>,
    active_tasks: AtomicUsize,
    memory_vector_db: Arc<BTreeMap<String, Vec<f32>>>,
}

impl SovereignMultiAgentPlanner {
    /// Creates a new self-contained multi-agent orchestrator
    pub fn new() -> Self {
        let mut default_agents = Vec::new();

        // 1. CrewAI / Auto-GPT style analytical reasoning agent
        default_agents.push(AIOSAgent {
            name: "Sovereign_Researcher".to_string(),
            role: "Information extraction and reasoning solver".to_string(),
            system_instructions: "Solve complex tasks step-by-step by generating rationales.".to_string(),
            primary_model: LocalModelSize::Medium8B,
        });

        // 2. High-speed automation agent
        default_agents.push(AIOSAgent {
            name: "Sovereign_Automator".to_string(),
            role: "Task pipeline execution engine".to_string(),
            system_instructions: "Extract actionable API mappings from user input.".to_string(),
            primary_model: LocalModelSize::Tiny1B,
        });

        Self {
            agents: default_agents,
            active_tasks: AtomicUsize::new(0),
            memory_vector_db: Arc::new(BTreeMap::new()),
        }
    }

    /// Dynamically routes a user query to the optimal model size, avoiding resource starvation
    pub fn route_task(&self, task_description: &str) -> (LocalModelSize, &str) {
        self.active_tasks.fetch_add(1, Ordering::SeqCst);

        // Simple heuristic search on target terms to replace Python-based classification runtimes
        if task_description.contains("orbit")
            || task_description.contains("quantum")
            || task_description.contains("backprop")
        {
            (
                LocalModelSize::Large70B,
                "Routing to Large MoE Engine for high-precision scientific analysis.",
            )
        } else if task_description.contains("reason")
            || task_description.contains("compile")
            || task_description.contains("audit")
        {
            (
                LocalModelSize::Medium8B,
                "Routing to Medium Reasoning Engine for analytical task decomposition.",
            )
        } else {
            (
                LocalModelSize::Tiny1B,
                "Routing to Tiny local model for immediate response.",
            )
        }
    }

    /// Simulates multi-agent negotiation (AutoGPT / CrewAI parity) for task completion
    pub fn run_negotiated_task(&self, query: &str) -> Result<String, &'static str> {
        let (model, rationale) = self.route_task(query);
        let mut final_result = format!("Rationalization: {}\n", rationale);

        for agent in &self.agents {
            if agent.primary_model == model || model == LocalModelSize::Large70B {
                let _ = write!(
                    final_result,
                    "[{}] executed task using instruction: '{}'\n",
                    agent.name, agent.system_instructions
                );
            }
        }

        self.active_tasks.fetch_sub(1, Ordering::SeqCst);
        Ok(final_result)
    }

    /// Embedded Cosine Similarity vector database lookup for agent memory search
    pub fn search_memory(&self, query_vector: &[f32], threshold: f32) -> Vec<String> {
        let mut matches = Vec::new();

        for (text, vector) in self.memory_vector_db.iter() {
            if vector.len() != query_vector.len() {
                continue;
            }

            // Perform manual dot product to avoid third-party BLAS bindings
            let dot_product: f32 = query_vector.iter().zip(vector.iter()).map(|(a, b)| a * b).sum();
            let query_norm: f32 = fast_sqrt(query_vector.iter().map(|x| x * x).sum::<f32>());
            let vector_norm: f32 = fast_sqrt(vector.iter().map(|x| x * x).sum::<f32>());

            if query_norm > 0.0 && vector_norm > 0.0 {
                let similarity = dot_product / (query_norm * vector_norm);
                if similarity >= threshold {
                    matches.push(text.clone());
                }
            }
        }

        matches
    }
}

fn fast_sqrt(val: f32) -> f32 {
    if val <= 0.0 {
        return 0.0;
    }
    let mut x = val;
    for _ in 0..10 {
        x = 0.5 * (x + val / x);
    }
    x
}

impl Default for SovereignMultiAgentPlanner {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SimpleAIAgent {
    pub id: AgentID,
    pub name: String,
    pub state: AgentState,
}

impl SimpleAIAgent {
    pub fn new(id: AgentID, name: &str) -> Self {
        SimpleAIAgent {
            id,
            name: name.to_string(),
            state: AgentState::Idle,
        }
    }
}

/// Local LLM and deep learning model resource orchestrator
pub struct LocalLlmOrchestrator {
    pub active_models: Vec<Option<ModelResource>>,
    pub total_gpu_memory_mb: usize,
    pub total_tpu_memory_mb: usize,
    pub allocated_gpu_memory_mb: AtomicUsize,
    pub allocated_tpu_memory_mb: AtomicUsize,
}

impl LocalLlmOrchestrator {
    pub fn new(gpu_mem: usize, tpu_mem: usize) -> Self {
        LocalLlmOrchestrator {
            active_models: Vec::new(),
            total_gpu_memory_mb: gpu_mem,
            total_tpu_memory_mb: tpu_mem,
            allocated_gpu_memory_mb: AtomicUsize::new(0),
            allocated_tpu_memory_mb: AtomicUsize::new(0),
        }
    }

    /// Schedule and allocate resources for a local LLM model
    pub fn schedule_model(
        &mut self,
        name: &[u8],
        size_mb: usize,
        prefer_device: DeviceTarget,
    ) -> Result<DeviceTarget, OrchestratorError> {
        let mut final_device = prefer_device;

        match prefer_device {
            DeviceTarget::Gpu => {
                let current_gpu = self.allocated_gpu_memory_mb.load(Ordering::SeqCst);
                if current_gpu + size_mb <= self.total_gpu_memory_mb {
                    self.allocated_gpu_memory_mb
                        .store(current_gpu + size_mb, Ordering::SeqCst);
                } else {
                    final_device = DeviceTarget::Cpu;
                }
            }
            DeviceTarget::Tpu => {
                let current_tpu = self.allocated_tpu_memory_mb.load(Ordering::SeqCst);
                if current_tpu + size_mb <= self.total_tpu_memory_mb {
                    self.allocated_tpu_memory_mb
                        .store(current_tpu + size_mb, Ordering::SeqCst);
                } else {
                    let current_gpu = self.allocated_gpu_memory_mb.load(Ordering::SeqCst);
                    if current_gpu + size_mb <= self.total_gpu_memory_mb {
                        self.allocated_gpu_memory_mb
                            .store(current_gpu + size_mb, Ordering::SeqCst);
                        final_device = DeviceTarget::Gpu;
                    } else {
                        final_device = DeviceTarget::Cpu;
                    }
                }
            }
            DeviceTarget::Cpu => {}
        }

        let resource = ModelResource::new(name, size_mb, final_device);
        self.active_models.push(Some(resource));

        Ok(final_device)
    }

    /// Evict model resources on shutdown/unload
    pub fn evict_model(&mut self, name: &[u8]) -> Result<(), OrchestratorError> {
        for i in 0..self.active_models.len() {
            if let Some(ref res) = self.active_models[i] {
                let len = res.name.iter().position(|&b| b == 0).unwrap_or(32);
                if &res.name[..len] == name {
                    match res.target {
                        DeviceTarget::Gpu => {
                            self.allocated_gpu_memory_mb
                                .fetch_sub(res.memory_required_mb, Ordering::SeqCst);
                        }
                        DeviceTarget::Tpu => {
                            self.allocated_tpu_memory_mb
                                .fetch_sub(res.memory_required_mb, Ordering::SeqCst);
                        }
                        DeviceTarget::Cpu => {}
                    }
                    self.active_models[i] = None;
                    return Ok(());
                }
            }
        }
        Err(OrchestratorError::ModelNotFound)
    }
}

pub trait TaskQueue {
    fn enqueue(&mut self, task: &[u8], priority: u8);
    fn dequeue(&mut self) -> Option<[u8; 256]>;
    fn size(&self) -> usize;
}

pub struct ContextWindowPruner {
    pub history: Vec<[u8; 128]>,
    pub max_lines: usize,
}

impl ContextWindowPruner {
    pub fn new(max_lines: usize) -> Self {
        ContextWindowPruner {
            history: Vec::new(),
            max_lines,
        }
    }

    /// Add a dialogue turn context string and prune old turns once exceeding limit (FIFO)
    pub fn append_context(&mut self, text: &[u8]) {
        let mut entry = [0u8; 128];
        let len = text.len().min(127);
        entry[..len].copy_from_slice(&text[..len]);

        self.history.push(entry);

        while self.history.len() > self.max_lines {
            self.history.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_scheduling() {
        let mut orchestrator = LocalLlmOrchestrator::new(4096, 8192);

        let target_res = orchestrator.schedule_model(b"phi-3", 2048, DeviceTarget::Tpu);
        assert_eq!(target_res.unwrap(), DeviceTarget::Tpu);

        let target_res_gpu = orchestrator.schedule_model(b"mistral-7b", 3072, DeviceTarget::Gpu);
        assert_eq!(target_res_gpu.unwrap(), DeviceTarget::Gpu);

        let target_res_cpu = orchestrator.schedule_model(b"llama-13b", 2048, DeviceTarget::Gpu);
        assert_eq!(target_res_cpu.unwrap(), DeviceTarget::Cpu);

        assert!(orchestrator.evict_model(b"mistral-7b").is_ok());
        assert_eq!(
            orchestrator.allocated_gpu_memory_mb.load(Ordering::SeqCst),
            0
        );
    }

    #[test]
    fn test_context_window_pruner() {
        let mut pruner = ContextWindowPruner::new(2);
        pruner.append_context(b"Context turn 1");
        pruner.append_context(b"Context turn 2");
        assert_eq!(pruner.history.len(), 2);

        pruner.append_context(b"Context turn 3");
        assert_eq!(pruner.history.len(), 2);

        let mut turn_first = [0u8; 14];
        turn_first.copy_from_slice(&pruner.history[0][..14]);
        assert_eq!(&turn_first, b"Context turn 2");
    }

    #[test]
    fn test_orchestrator_routing() {
        let orchestrator = SovereignMultiAgentPlanner::new();
        let (model, _) =
            orchestrator.route_task("Compute the quantum backpropagation step of a DeepSeek node");
        assert_eq!(model, LocalModelSize::Large70B);

        let (model2, _) =
            orchestrator.route_task("Help compile this rust file and reason about the error");
        assert_eq!(model2, LocalModelSize::Medium8B);
    }

    #[test]
    fn test_negotiation_pipeline() {
        let orchestrator = SovereignMultiAgentPlanner::new();
        let output = orchestrator
            .run_negotiated_task("Determine the optimal task execution pipeline")
            .unwrap();
        assert!(output.contains("Tiny1B") || output.contains("Sovereign_Automator"));
    }
}
