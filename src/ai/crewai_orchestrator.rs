// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - CrewAI multi-agent team orchestration
// Principles: COLLABORATION, MODULARITY, SEPARATION OF CONCERNS, COMPOSITION OVER INHERITANCE
use alloc::{collections::BTreeMap, string::String, vec::Vec};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AgentRole {
    Developer,
    QaEngineer,
    SecurityLead,
    ProjectManager,
    TechWriter,
    DataAnalyst,
    DevOps,
    Designer,
}
impl AgentRole {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Developer => "Developer",
            Self::QaEngineer => "QA Engineer",
            Self::SecurityLead => "Security Lead",
            Self::ProjectManager => "Project Manager",
            Self::TechWriter => "Tech Writer",
            Self::DataAnalyst => "Data Analyst",
            Self::DevOps => "DevOps",
            Self::Designer => "Designer",
        }
    }
}
#[derive(Debug, Clone)]
pub struct AgentMemory {
    pub short_term: Vec<String>,
    pub long_term: BTreeMap<String, String>,
}
impl AgentMemory {
    pub fn new() -> Self {
        Self {
            short_term: Vec::new(),
            long_term: BTreeMap::new(),
        }
    }
    pub fn remember(&mut self, fact: &str) {
        self.short_term.push(fact.into());
        if self.short_term.len() > 50 {
            self.short_term.remove(0);
        }
    }
    pub fn store(&mut self, key: &str, value: &str) {
        self.long_term.insert(key.into(), value.into());
    }
    pub fn recall(&self, key: &str) -> Option<&str> {
        self.long_term.get(key).map(|s| s.as_str())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Done { result: String },
    Failed { error: String },
}
#[derive(Debug, Clone)]
pub struct AgentTask {
    pub id: u32,
    pub description: String,
    pub assigned_to: AgentRole,
    pub priority: u8,
    pub status: TaskStatus,
    pub dependencies: Vec<u32>,
}
impl AgentTask {
    pub fn new(id: u32, desc: &str, role: AgentRole, priority: u8) -> Self {
        Self {
            id,
            description: desc.into(),
            assigned_to: role,
            priority,
            status: TaskStatus::Pending,
            dependencies: Vec::new(),
        }
    }
    pub fn is_ready(&self, completed: &[u32]) -> bool {
        self.dependencies.iter().all(|d| completed.contains(d))
    }
}
#[derive(Debug, Clone)]
pub struct CrewAgent {
    pub role: AgentRole,
    pub memory: AgentMemory,
    pub skills: Vec<String>,
    pub current_task: Option<u32>,
}
impl CrewAgent {
    pub fn new(role: AgentRole, skills: Vec<&str>) -> Self {
        Self {
            role,
            memory: AgentMemory::new(),
            skills: skills.iter().map(|s| s.to_string()).collect(),
            current_task: Option::None,
        }
    }
    pub fn process_task(&mut self, task: &mut AgentTask) -> String {
        task.status = TaskStatus::InProgress;
        self.current_task = Some(task.id);
        let result = alloc::format!("{} completed: {}", self.role.name(), task.description);
        task.status = TaskStatus::Done {
            result: result.clone(),
        };
        self.current_task = Option::None;
        result
    }
}
#[derive(Debug, Clone)]
pub struct CrewMessage {
    pub from: AgentRole,
    pub to: AgentRole,
    pub content: String,
    pub timestamp: u64,
}
pub struct CrewOrchestrator {
    pub agents: BTreeMap<AgentRole, CrewAgent>,
    pub task_queue: Vec<AgentTask>,
    pub message_log: Vec<CrewMessage>,
    pub completed_task_ids: Vec<u32>,
    pub next_task_id: u32,
}
impl CrewOrchestrator {
    pub fn new() -> Self {
        let mut agents = BTreeMap::new();
        agents.insert(
            AgentRole::Developer,
            CrewAgent::new(
                AgentRole::Developer,
                alloc::vec!["rust", "python", "typescript"],
            ),
        );
        agents.insert(
            AgentRole::QaEngineer,
            CrewAgent::new(AgentRole::QaEngineer, alloc::vec!["testing", "automation"]),
        );
        agents.insert(
            AgentRole::SecurityLead,
            CrewAgent::new(
                AgentRole::SecurityLead,
                alloc::vec!["audit", "cryptography"],
            ),
        );
        agents.insert(
            AgentRole::ProjectManager,
            CrewAgent::new(
                AgentRole::ProjectManager,
                alloc::vec!["planning", "coordination"],
            ),
        );
        Self {
            agents,
            task_queue: Vec::new(),
            message_log: Vec::new(),
            completed_task_ids: Vec::new(),
            next_task_id: 1,
        }
    }
    pub fn add_task(&mut self, desc: &str, role: AgentRole, priority: u8) -> u32 {
        let id = self.next_task_id;
        self.next_task_id += 1;
        self.task_queue
            .push(AgentTask::new(id, desc, role, priority));
        id
    }
    pub fn send_message(&mut self, from: AgentRole, to: AgentRole, content: &str) {
        self.message_log.push(CrewMessage {
            from,
            to,
            content: content.into(),
            timestamp: self.message_log.len() as u64,
        });
    }
    pub fn run_cycle(&mut self) -> Vec<String> {
        let mut results = Vec::new();
        let ready_ids: Vec<u32> = self
            .task_queue
            .iter()
            .filter(|t| t.status == TaskStatus::Pending && t.is_ready(&self.completed_task_ids))
            .map(|t| t.id)
            .collect();
        for task_id in ready_ids {
            if let Some(task_idx) = self.task_queue.iter().position(|t| t.id == task_id) {
                let role = self.task_queue[task_idx].assigned_to;
                if let Some(agent) = self.agents.get_mut(&role) {
                    let task = &mut self.task_queue[task_idx];
                    let r = agent.process_task(task);
                    self.completed_task_ids.push(task_id);
                    results.push(r);
                }
            }
        }
        results
    }
    pub fn aggregate_results(&self) -> String {
        let done: Vec<&AgentTask> = self
            .task_queue
            .iter()
            .filter(|t| matches!(t.status, TaskStatus::Done { .. }))
            .collect();
        alloc::format!("{}/{} tasks completed", done.len(), self.task_queue.len())
    }
    pub fn pending_count(&self) -> usize {
        self.task_queue
            .iter()
            .filter(|t| t.status == TaskStatus::Pending)
            .count()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_and_run_task() {
        let mut crew = CrewOrchestrator::new();
        let id = crew.add_task("implement feature X", AgentRole::Developer, 5);
        let results = crew.run_cycle();
        assert!(!results.is_empty());
        assert!(results[0].contains("Developer"));
    }
    #[test]
    fn test_message_log() {
        let mut crew = CrewOrchestrator::new();
        crew.send_message(
            AgentRole::ProjectManager,
            AgentRole::Developer,
            "Please implement auth",
        );
        assert_eq!(crew.message_log.len(), 1);
    }
    #[test]
    fn test_aggregate() {
        let mut crew = CrewOrchestrator::new();
        crew.add_task("task1", AgentRole::Developer, 3);
        crew.add_task("task2", AgentRole::QaEngineer, 2);
        crew.run_cycle();
        let summary = crew.aggregate_results();
        assert!(summary.contains("2/2"));
    }
}
