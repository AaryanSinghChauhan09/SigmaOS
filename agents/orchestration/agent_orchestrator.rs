// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Agent Orchestrator (Rust, no_std)
//! Replaces: agents/orchestration/AgentOrchestrator.cpp
//! =========================================================================

const MAX_AGENTS: usize = 64;

#[derive(Copy, Clone, PartialEq)]
pub enum AgentStatus {
    Idle,
    Running,
    Stopped,
    Failed,
}

#[derive(Copy, Clone)]
pub struct Agent {
    pub id: usize,
    pub status: AgentStatus,
    pub priority: u8,
}

impl Agent {
    pub const fn new(id: usize, priority: u8) -> Self {
        Self { id, status: AgentStatus::Idle, priority }
    }

    pub fn start(&mut self) {
        self.status = AgentStatus::Running;
    }

    pub fn stop(&mut self) {
        self.status = AgentStatus::Stopped;
    }

    pub fn class_name(&self) -> &'static str {
        "Agent"
    }
}

pub struct AgentOrchestrator {
    agents: [Option<Agent>; MAX_AGENTS],
    count: usize,
}

impl AgentOrchestrator {
    pub const fn new() -> Self {
        Self {
            agents: [None; MAX_AGENTS],
            count: 0,
        }
    }

    pub fn register(&mut self, id: usize, priority: u8) -> bool {
        if self.count >= MAX_AGENTS {
            return false;
        }
        self.agents[self.count] = Some(Agent::new(id, priority));
        self.count += 1;
        true
    }

    pub fn start_agent(&mut self, id: usize) -> bool {
        for i in 0..self.count {
            if let Some(ref mut agent) = self.agents[i] {
                if agent.id == id {
                    agent.start();
                    return true;
                }
            }
        }
        false
    }

    pub fn stop_agent(&mut self, id: usize) -> bool {
        for i in 0..self.count {
            if let Some(ref mut agent) = self.agents[i] {
                if agent.id == id {
                    agent.stop();
                    return true;
                }
            }
        }
        false
    }

    pub fn running_count(&self) -> usize {
        let mut n = 0;
        for i in 0..self.count {
            if let Some(ref a) = self.agents[i] {
                if a.status == AgentStatus::Running {
                    n += 1;
                }
            }
        }
        n
    }

    pub fn class_name(&self) -> &'static str {
        "AgentOrchestrator"
    }
}
