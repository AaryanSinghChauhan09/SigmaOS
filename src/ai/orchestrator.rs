// Local LLM Orchestrator for SigmaOS
// Dynamically schedules models, checks device bounds, and prunes context windows.

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type AgentID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceTarget {
    Cpu = 0,
    Gpu = 1,
    Tpu = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    Idle = 0,
    Active = 1,
    Busy = 2,
    Error = 3,
    Learning = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentError {
    Success = 0,
    NotFound = 1,
    ExecutionFailed = 2,
    Timeout = 3,
    InvalidInput = 4,
}

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

impl AIAgent for SimpleAIAgent {
    fn id(&self) -> AgentID {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn state(&self) -> AgentState {
        self.state
    }

    fn execute(&mut self, task: &[u8]) -> Result<Vec<u8>, AgentError> {
        self.state = AgentState::Busy;
        let mut result = Vec::new();
        for &byte in self.name.as_bytes() {
            result.push(byte);
        }
        result.push(b':');
        result.push(b' ');
        for &byte in task {
            result.push(byte);
        }
        self.state = AgentState::Idle;
        Ok(result)
    }
}

pub trait AgentOrchestrator {
    fn register_agent(&mut self, agent: Box<dyn AIAgent>) -> Result<AgentID, AgentError>;
    fn dispatch_task(&mut self, task: &[u8], agent_id: Option<AgentID>) -> Result<Vec<u8>, AgentError>;
    fn get_agent(&self, id: AgentID) -> Option<&dyn AIAgent>;
    fn list_agents(&self) -> Vec<AgentID>;
}

pub struct SimpleAgentOrchestrator {
    pub agents: Vec<Box<dyn AIAgent>>,
    pub next_id: AtomicUsize,
}

impl SimpleAgentOrchestrator {
    pub fn new() -> Self {
        SimpleAgentOrchestrator {
            agents: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Default for SimpleAgentOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentOrchestrator for SimpleAgentOrchestrator {
    fn register_agent(&mut self, agent: Box<dyn AIAgent>) -> Result<AgentID, AgentError> {
        let id = agent.id();
        self.agents.push(agent);
        Ok(id)
    }

    fn dispatch_task(&mut self, task: &[u8], agent_id: Option<AgentID>) -> Result<Vec<u8>, AgentError> {
        if let Some(target_id) = agent_id {
            if let Some(agent) = self.agents.iter_mut().find(|a| a.id() == target_id) {
                agent.execute(task)
            } else {
                Err(AgentError::NotFound)
            }
        } else {
            if let Some(agent) = self.agents.iter_mut().find(|a| a.state() == AgentState::Idle) {
                agent.execute(task)
            } else {
                Err(AgentError::NotFound)
            }
        }
    }

    fn get_agent(&self, id: AgentID) -> Option<&dyn AIAgent> {
        self.agents.iter().find(|a| a.id() == id).map(|a| a.as_ref())
    }

    fn list_agents(&self) -> Vec<AgentID> {
        self.agents.iter().map(|a| a.id()).collect()
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

pub trait TaskQueue {
    fn enqueue(&mut self, task: &[u8], priority: u8);
    fn dequeue(&mut self) -> Option<[u8; 256]>;
    fn size(&self) -> usize;
}

pub struct SimpleTaskQueue {
    pub tasks: Vec<([u8; 256], u8)>,
}

impl SimpleTaskQueue {
    pub fn new() -> Self {
        SimpleTaskQueue { tasks: Vec::new() }
    }
}

impl Default for SimpleTaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskQueue for SimpleTaskQueue {
    fn enqueue(&mut self, task: &[u8], priority: u8) {
        let mut task_array = [0u8; 256];
        let task_len = task.len().min(255);
        task_array[..task_len].copy_from_slice(&task[..task_len]);
        self.tasks.push((task_array, priority));
    }

    fn dequeue(&mut self) -> Option<[u8; 256]> {
        if self.tasks.is_empty() {
            return None;
        }
        let mut highest_idx = 0;
        let mut highest_priority = 0;

        for (i, (_, priority)) in self.tasks.iter().enumerate() {
            if *priority > highest_priority {
                highest_priority = *priority;
                highest_idx = i;
            }
        }

        Some(self.tasks.remove(highest_idx).0)
    }

    fn size(&self) -> usize {
        self.tasks.len()
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

        assert_eq!(&pruner.history[0][..14], b"Context turn 2");
    }

    #[test]
    fn test_orchestrator_and_queue() {
        let mut orchestrator = SimpleAgentOrchestrator::new();
        let agent = SimpleAIAgent::new(1, "TaskAgent");
        orchestrator.register_agent(Box::new(agent)).unwrap();

        let response = orchestrator.dispatch_task(b"RELOAD_CORES", Some(1)).unwrap();
        assert_eq!(core::str::from_utf8(&response).unwrap(), "TaskAgent: RELOAD_CORES");

        let mut queue = SimpleTaskQueue::new();
        queue.enqueue(b"TASK_PRIO_HIGH", 10);
        queue.enqueue(b"TASK_PRIO_LOW", 1);
        assert_eq!(queue.size(), 2);

        let task = queue.dequeue().unwrap();
        assert_eq!(core::str::from_utf8(&task[..14]).unwrap(), "TASK_PRIO_HIGH");
    }
}
