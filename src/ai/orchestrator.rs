// OOP-based AI Orchestrator for SigmaOS
// Implements sigma-ai core with multi-agent coordination, workflow automation,
// and self-diagnosis capabilities for system optimization.

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
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
    fn name(&self) -> &[u8];
    fn state(&self) -> AgentState;
    fn execute(&mut self, task: &[u8]) -> Result<Vec<u8>, AgentError>;
}

pub struct SimpleAIAgent {
    pub id: AgentID,
    pub name: [u8; 64],
    pub state: AtomicUsize,
}

impl SimpleAIAgent {
    pub fn new(id: AgentID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        name_array[..name_len].copy_from_slice(&name[..name_len]);

        SimpleAIAgent {
            id,
            name: name_array,
            state: AtomicUsize::new(AgentState::Idle as usize),
        }
    }
}

impl AIAgent for SimpleAIAgent {
    fn id(&self) -> AgentID {
        self.id
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn state(&self) -> AgentState {
        match self.state.load(Ordering::SeqCst) {
            0 => AgentState::Idle,
            1 => AgentState::Active,
            2 => AgentState::Busy,
            3 => AgentState::Error,
            _ => AgentState::Learning,
        }
    }

    fn execute(&mut self, task: &[u8]) -> Result<Vec<u8>, AgentError> {
        self.state
            .store(AgentState::Busy as usize, Ordering::SeqCst);
        let mut result = Vec::new();
        result.extend_from_slice(self.name());
        result.push(b':');
        result.push(b' ');
        result.extend_from_slice(task);
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
    pub agents: Vec<Option<Box<dyn AIAgent>>>,
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

impl AgentOrchestrator for SimpleAgentOrchestrator {
    fn register_agent(&mut self, agent: Box<dyn AIAgent>) -> Result<AgentID, AgentError> {
        let id = agent.id();
        self.agents.push(Some(agent));
        Ok(id)
    }

    fn dispatch_task(
        &mut self,
        task: &[u8],
        agent_id: Option<AgentID>,
    ) -> Result<Vec<u8>, AgentError> {
        if let Some(target_id) = agent_id {
            for agent_option in &mut self.agents {
                if let Some(ref mut agent) = *agent_option {
                    if agent.id() == target_id {
                        return agent.execute(task);
                    }
                }
            }
            Err(AgentError::NotFound)
        } else {
            for agent_option in &mut self.agents {
                if let Some(ref mut agent) = *agent_option {
                    if agent.state() == AgentState::Idle {
                        return agent.execute(task);
                    }
                }
            }
            Err(AgentError::NotFound)
        }
    }

    fn get_agent(&self, id: AgentID) -> Option<&dyn AIAgent> {
        for agent_option in &self.agents {
            if let Some(ref agent) = *agent_option {
                if agent.id() == id {
                    return Some(agent.as_ref());
                }
            }
        }
        None
    }

    fn list_agents(&self) -> Vec<AgentID> {
        let mut ids = Vec::new();
        for agent_option in &self.agents {
            if let Some(ref agent) = *agent_option {
                ids.push(agent.id());
            }
        }
        ids
    }
}

pub trait TaskQueue {
    fn enqueue(&mut self, task: &[u8], priority: u8);
    fn dequeue(&mut self) -> Option<[u8; 256]>;
    fn peek(&self) -> Option<&[u8]>;
    fn size(&self) -> usize;
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
        for i in 0..self.messages.len() {
            if self.messages[i].1 == agent_id {
                return Some(self.messages.remove(i).2);
            }
        }
        None
    }

    fn broadcast(&mut self, from: AgentID, message: &[u8]) {
        let mut msg_array = [0u8; 256];
        let msg_len = message.len().min(255);
        msg_array[..msg_len].copy_from_slice(&message[..msg_len]);
        self.messages.push((from, 0, msg_array));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_orchestrator_flows() {
        let mut orchestrator = SimpleAgentOrchestrator::new();
        let agent = SimpleAIAgent::new(99, b"SovereignSchedulerOptimizer");
        orchestrator.register_agent(Box::new(agent)).unwrap();

        assert_eq!(orchestrator.list_agents().len(), 1);

        let response = orchestrator
            .dispatch_task(b"optimize core affinity", Some(99))
            .unwrap();
        assert_eq!(
            response,
            b"SovereignSchedulerOptimizer: optimize core affinity"
        );
    }
}
