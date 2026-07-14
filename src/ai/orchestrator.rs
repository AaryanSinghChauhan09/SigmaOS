#![no_std]
#![no_main]

/// OOP-based AI Orchestrator for SigmaOS
/// Based on Ideas-999-Structured: AI & Automation Item 335
/// Implements sigma-ai core with multi-agent coordination

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type AgentID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AgentState { Idle = 0, Active = 1, Busy = 2, Error = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AgentError { Success = 0, NotFound = 1, ExecutionFailed = 2, Timeout = 3 }

pub trait AIAgent {
    fn id(&self) -> AgentID;
    fn name(&self) -> &[u8];
    fn state(&self) -> AgentState;
    fn execute(&mut self, task: &[u8]) -> Result<Vec<u8>, AgentError>;
}

#[repr(C)]
pub struct SimpleAIAgent {
    pub id: AgentID,
    pub name: [u8; 64],
    pub state: AtomicUsize,
}

impl SimpleAIAgent {
    pub fn new(id: AgentID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleAIAgent {
            id,
            name: name_array,
            state: AtomicUsize::new(AgentState::Idle as usize),
        }
    }
}

impl AIAgent for SimpleAIAgent {
    fn id(&self) -> AgentID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn state(&self) -> AgentState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }

    fn execute(&mut self, task: &[u8]) -> Result<Vec<u8>, AgentError> {
        self.state.store(AgentState::Busy as usize, Ordering::SeqCst);
        let mut result = Vec::new();
        let name = self.name();
        for &byte in name { result.push(byte); }
        result.push(b':');
        result.push(b' ');
        for &byte in task { result.push(byte); }
        self.state.store(AgentState::Idle as usize, Ordering::SeqCst);
        Ok(result)
    }
}

pub trait AgentOrchestrator {
    fn register_agent(&mut self, agent: Box<dyn AIAgent>) -> Result<AgentID, AgentError>;
    fn dispatch_task(&mut self, task: &[u8], agent_id: Option<AgentID>) -> Result<Vec<u8>, AgentError>;
    fn get_agent(&self, id: AgentID) -> Option<&dyn AIAgent>;
    fn list_agents(&self) -> Vec<AgentID>;
}

#[repr(C)]
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

    fn dispatch_task(&mut self, task: &[u8], agent_id: Option<AgentID>) -> Result<Vec<u8>, AgentError> {
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
                if agent.id() == id { return Some(agent.as_ref()); }
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

#[repr(C)]
pub struct SimpleTaskQueue {
    pub tasks: Vec<([u8; 256], u8)>,
}

impl SimpleTaskQueue {
    pub fn new() -> Self {
        SimpleTaskQueue {
            tasks: Vec::new(),
        }
    }
}

impl TaskQueue for SimpleTaskQueue {
    fn enqueue(&mut self, task: &[u8], priority: u8) {
        let mut task_array = [0u8; 256];
        let task_len = task.len().min(255);
        for i in 0..task_len {
            task_array[i] = task[i];
        }
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
            return None
        }
        Some(&self.tasks[0].0)
    }

    fn size(&self) -> usize { self.tasks.len() }
}

pub trait AgentCommunication {
    fn send_message(&mut self, from: AgentID, to: AgentID, message: &[u8]) -> Result<(), AgentError>;
    fn receive_message(&mut self, agent_id: AgentID) -> Option<[u8; 256]>;
    fn broadcast(&mut self, from: AgentID, message: &[u8]);
}

#[repr(C)]
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

impl AgentCommunication for SimpleAgentCommunication {
    fn send_message(&mut self, from: AgentID, to: AgentID, message: &[u8]) -> Result<(), AgentError> {
        let mut msg_array = [0u8; 256];
        let msg_len = message.len().min(255);
        for i in 0..msg_len {
            msg_array[i] = message[i];
        }
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
        for i in 0..msg_len {
            msg_array[i] = message[i];
        }
        self.messages.push((from, 0, msg_array));
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
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
    fn is_empty(&self) -> bool { self.len == 0 }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
