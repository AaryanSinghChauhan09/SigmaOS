// OOP-based AI Orchestrator for SigmaOS
// Implements sigma-ai core with multi-agent coordination, workflow automation,
// and self-diagnosis capabilities for system optimization

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type AgentID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    Idle = 0,
    Active = 1,
    Busy = 2,
    Error = 3,
    Learning = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
        self.state
            .store(AgentState::Busy as usize, Ordering::SeqCst);
        let mut result = Vec::new();
        for &byte in self.name.as_bytes() {
            result.push(byte);
        }
        result.push(b':');
        result.push(b' ');
        for &byte in task {
            result.push(byte);
        }
        self.state
            .store(AgentState::Idle as usize, Ordering::SeqCst);
        Ok(result)
    }
}

pub trait AgentOrchestrator {
    fn register_agent(&mut self, agent: Box<dyn AIAgent>) -> Result<AgentID, AgentError>;
    fn dispatch_task(
        &mut self,
        task: &[u8],
        agent_id: Option<AgentID>,
    ) -> Result<Vec<u8>, AgentError>;
    fn get_agent(&self, id: AgentID) -> Option<&dyn AIAgent>;
    fn list_agents(&self) -> Vec<AgentID>;
}

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
            }
        } else {
            if let Some(agent) = self
                .agents
                .iter_mut()
                .find(|a| a.state() == AgentState::Idle)
            {
                agent.execute(task)
            } else {
                Err(AgentError::NotFound)
            }
        }
    }

    fn get_agent(&self, id: AgentID) -> Option<&dyn AIAgent> {
        for agent in &self.agents {
            if agent.id() == id {
                return Some(agent.as_ref());
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

    fn peek(&self) -> Option<&[u8]> {
        if self.tasks.is_empty() {
            return None;
        }
        Some(&self.tasks[0].0)
    }

    fn size(&self) -> usize {
        self.tasks.len()
    }
}

pub trait AgentCommunication {
    fn send_message(
        &mut self,
        from: AgentID,
        to: AgentID,
        message: &[u8],
    ) -> Result<(), AgentError>;
    fn receive_message(&mut self, agent_id: AgentID) -> Option<[u8; 256]>;
    fn broadcast(&mut self, from: AgentID, message: &[u8]);
}

pub struct SimpleAgentCommunication {
    pub messages: Vec<(AgentID, AgentID, [u8; 256])>,
}

impl SimpleAgentCommunication {
    pub fn new() -> Self {
        SimpleAgentCommunication {
            messages: Vec::new(),
        }
    }
}

impl Default for SimpleAgentCommunication {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentCommunication for SimpleAgentCommunication {
    fn send_message(
        &mut self,
        from: AgentID,
        to: AgentID,
        message: &[u8],
    ) -> Result<(), AgentError> {
        let mut msg_array = [0u8; 256];
        let msg_len = message.len().min(255);
        msg_array[..msg_len].copy_from_slice(&message[..msg_len]);
        self.messages.push((from, to, msg_array));
        Ok(())
    }

    fn receive_message(&mut self, agent_id: AgentID) -> Option<[u8; 256]> {
        if let Some(pos) = self.messages.iter().position(|m| m.1 == agent_id) {
            Some(self.messages.remove(pos).2)
        } else {
            None
        }
    }

    fn broadcast(&mut self, from: AgentID, message: &[u8]) {
        let mut msg_array = [0u8; 256];
        let msg_len = message.len().min(255);
        msg_array[..msg_len].copy_from_slice(&message[..msg_len]);
        self.messages.push((from, 0, msg_array));
    }
}
