// OOP-based AI Orchestrator for SigmaOS
// Implements sigma-ai core with multi-agent coordination, workflow automation,
// and self-diagnosis capabilities for system optimization.

use core::mem;
/// Local LLM Orchestrator for SigmaOS
/// Dynamically schedules models, checks device bounds, and prunes context windows.
use core::sync::atomic::{AtomicUsize, Ordering};

pub type AgentID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    Idle = 0,
    Active = 1,
    Busy = 2,
    Error = 3,
    Learning = 4,
}

#[repr(C)]
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

<<<<<<< HEAD
impl ModelResource {
    pub fn new(name: &[u8], memory_required_mb: usize, target: DeviceTarget) -> Self {
        let mut name_array = [0u8; 32];
        let len = name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), len);
        }
        ModelResource {
            name: name_array,
            memory_required_mb,
            target,
||||||| 23ef22a4a
pub struct SimpleAIAgent {
    pub id: AgentID,
    pub name: String,
    pub state: AtomicUsize,
}

impl SimpleAIAgent {
    pub fn new(id: AgentID, name: &str) -> Self {
        SimpleAIAgent {
            id,
            name: name.to_string(),
            state: AtomicUsize::new(AgentState::Idle as usize),
=======
pub struct SimpleAIAgent {
    pub id: AgentID,
    pub name: String,
    pub state: AtomicUsize,
}

impl SimpleAIAgent {
    pub fn new(id: AgentID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        name_array[..name_len].copy_from_slice(&name[..name_len]);

        SimpleAIAgent {
            id,
            name: name.to_string(),
            state: AtomicUsize::new(AgentState::Idle as usize),
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
        }
    }
}

<<<<<<< HEAD
/// Local LLM and deep learning model resource orchestrator
pub struct LocalLlmOrchestrator {
    pub active_models: Vec<Option<ModelResource>>,
    pub total_gpu_memory_mb: usize,
    pub total_tpu_memory_mb: usize,
    pub allocated_gpu_memory_mb: AtomicUsize,
    pub allocated_tpu_memory_mb: AtomicUsize,
||||||| 23ef22a4a
impl AIAgent for SimpleAIAgent {
    fn id(&self) -> AgentID {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn state(&self) -> AgentState {
        let raw = self.state.load(Ordering::SeqCst);
        match raw {
            1 => AgentState::Active,
            2 => AgentState::Busy,
            3 => AgentState::Error,
            4 => AgentState::Learning,
            _ => AgentState::Idle,
        }
    }

    fn execute(&mut self, task: &[u8]) -> Result<Vec<u8>, AgentError> {
        self.state.store(AgentState::Busy as usize, Ordering::SeqCst);
        let mut result = Vec::new();
        for &byte in self.name.as_bytes() {
            result.push(byte);
        }
        result.push(b':');
        result.push(b' ');
        for &byte in task {
            result.push(byte);
        }
        self.state.store(AgentState::Idle as usize, Ordering::SeqCst);
        Ok(result)
    }
=======
impl AIAgent for SimpleAIAgent {
    fn id(&self) -> AgentID {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }

    fn state(&self) -> AgentState {
        let raw = self.state.load(Ordering::SeqCst);
        match raw {
            1 => AgentState::Active,
            2 => AgentState::Busy,
            3 => AgentState::Error,
            4 => AgentState::Learning,
            _ => AgentState::Idle,
        }
    }

    fn execute(&mut self, task: &[u8]) -> Result<Vec<u8>, AgentError> {
        self.state.store(AgentState::Busy as usize, Ordering::SeqCst);
        let mut result = Vec::new();
        for &byte in self.name.as_bytes() {
            result.push(byte);
        }
        result.push(b':');
        result.push(b' ');
        for &byte in task {
            result.push(byte);
        }
        self.state.store(AgentState::Idle as usize, Ordering::SeqCst);
        Ok(result)
    }
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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

<<<<<<< HEAD
        match prefer_device {
            DeviceTarget::Gpu => {
                let current_gpu = self.allocated_gpu_memory_mb.load(Ordering::SeqCst);
                if current_gpu + size_mb <= self.total_gpu_memory_mb {
                    self.allocated_gpu_memory_mb
                        .store(current_gpu + size_mb, Ordering::SeqCst);
                } else {
                    // Fallback to CPU
                    final_device = DeviceTarget::Cpu;
                }
||||||| 23ef22a4a
pub struct SimpleAgentOrchestrator {
    pub agents: Vec<Box<dyn AIAgent>>,
    pub next_id: AtomicUsize,
    pub model_temperature: f32,
    pub response_timeout_secs: u32,
}

impl SimpleAgentOrchestrator {
    pub fn new() -> Self {
        SimpleAgentOrchestrator {
            agents: Vec::new(),
            next_id: AtomicUsize::new(1),
            model_temperature: 0.7,
            response_timeout_secs: 30,
        }
    }

    pub fn set_model_temperature(&mut self, temp: f32) {
        self.model_temperature = temp;
    }

    pub fn set_response_timeout(&mut self, secs: u32) {
        self.response_timeout_secs = secs;
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

    fn dispatch_task(
        &mut self,
        task: &[u8],
        agent_id: Option<AgentID>,
    ) -> Result<Vec<u8>, AgentError> {
        if let Some(target_id) = agent_id {
            if let Some(agent) = self.agents.iter_mut().find(|a| a.id() == target_id) {
                agent.execute(task)
            } else {
                Err(AgentError::NotFound)
=======
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

    fn dispatch_task(
        &mut self,
        task: &[u8],
        agent_id: Option<AgentID>,
    ) -> Result<Vec<u8>, AgentError> {
        if let Some(target_id) = agent_id {
            if let Some(agent) = self.agents.iter_mut().find(|a| a.id() == target_id) {
                agent.execute(task)
            } else {
                Err(AgentError::NotFound)
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
            }
            DeviceTarget::Tpu => {
                let current_tpu = self.allocated_tpu_memory_mb.load(Ordering::SeqCst);
                if current_tpu + size_mb <= self.total_tpu_memory_mb {
                    self.allocated_tpu_memory_mb
                        .store(current_tpu + size_mb, Ordering::SeqCst);
                } else {
                    // Fallback to GPU if available, else CPU
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
            DeviceTarget::Cpu => {
                // CPU is always standard fallback with VM paging bounds
            }
        }

        let resource = ModelResource::new(name, size_mb, final_device);
        self.active_models.push(Some(resource));

        Ok(final_device)
    }

    /// Evict model resources on shutdown/unload
    pub fn evict_model(&mut self, name: &[u8]) -> Result<(), OrchestratorError> {
        for i in 0..self.active_models.len {
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
        None
    }

    fn list_agents(&self) -> Vec<AgentID> {
        self.agents.iter().map(|a| a.id()).collect()
    }
}

pub trait TaskQueue {
    fn enqueue(&mut self, task: &[u8], priority: u8);
    fn dequeue(&mut self) -> Option<[u8; 256]>;
    fn peek(&self) -> Option<&[u8]>;
    fn size(&self) -> usize;
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
        unsafe {
            core::ptr::copy_nonoverlapping(text.as_ptr(), entry.as_mut_ptr(), len);
        }

        self.history.push(entry);

        // Slide window by removing the oldest context if exceeding max lines limit
        while self.history.len > self.max_lines {
            self.history.remove(0);
        }
    }
}

struct Vec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_scheduling() {
        let mut orchestrator = LocalLlmOrchestrator::new(4096, 8192);

        // Schedule model preferring TPU
        let target_res = orchestrator.schedule_model(b"phi-3", 2048, DeviceTarget::Tpu);
        assert_eq!(target_res.unwrap(), DeviceTarget::Tpu);

        // Schedule model preferring GPU
        let target_res_gpu = orchestrator.schedule_model(b"mistral-7b", 3072, DeviceTarget::Gpu);
        assert_eq!(target_res_gpu.unwrap(), DeviceTarget::Gpu);

        // Schedule model exceeding GPU limit - should fallback to CPU
        let target_res_cpu = orchestrator.schedule_model(b"llama-13b", 2048, DeviceTarget::Gpu);
        assert_eq!(target_res_cpu.unwrap(), DeviceTarget::Cpu);

        // Evict Mistral GPU model
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
        assert_eq!(pruner.history.len, 2);

        // Turn 3 should displace Turn 1 (FIFO)
        pruner.append_context(b"Context turn 3");
        assert_eq!(pruner.history.len, 2);

        let mut turn_first = [0u8; 14];
        for i in 0..14 {
            turn_first[i] = pruner.history[0][i];
        }
        assert_eq!(&turn_first, b"Context turn 2");
    }
}
