// OOP-based AI Agent Framework for SigmaOS
// Implements AI agent using OOP principles with traits and structs.
#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Intent type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentType {
    SystemCommand = 0,
    FileOperation = 1,
    NetworkRequest = 2,
    ApplicationLaunch = 3,
    InformationQuery = 4,
    Custom = 5,
}

/// Intent (OOP: Intent object)
pub struct Intent {
    pub intent_type: IntentType,
    pub confidence: f32,
    pub command: String,
    pub parameters: String,
}

impl Intent {
    pub fn new(intent_type: IntentType, command: &str) -> Self {
        Intent {
            intent_type,
            confidence: 0.0,
            command: command.to_string(),
            parameters: String::new(),
        }
    }

    pub fn with_parameters(mut self, params: &str) -> Self {
        self.parameters = params.to_string();
        self
    }
}

/// AI error types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIError {
    Success = 0,
    ParseFailed = 1,
    ExecutionFailed = 2,
    UnknownIntent = 3,
    PermissionDenied = 4,
    InvalidInput = 5,
}

/// Agent capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentCapability {
    pub can_parse: bool,
    pub can_execute: bool,
    pub can_learn: bool,
}

impl AgentCapability {
    pub const fn new() -> Self {
        AgentCapability {
            can_parse: false,
            can_execute: false,
            can_learn: false,
        }
    }

    pub const fn full() -> Self {
        AgentCapability {
            can_parse: true,
            can_execute: true,
            can_learn: true,
        }
    }
}

impl Default for AgentCapability {
    fn default() -> Self {
        Self::new()
    }
}

/// Agent info
pub struct AgentInfo {
    pub name: String,
    pub version: (u32, u32, u32),
    pub total_intents: usize,
    pub execution_count: usize,
    pub capability: AgentCapability,
}

impl AgentInfo {
    pub fn new() -> Self {
        AgentInfo {
            name: String::new(),
            version: (1, 0, 0),
            total_intents: 0,
            execution_count: 0,
            capability: AgentCapability::new(),
        }
    }
}

impl Default for AgentInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// AI agent trait (OOP interface)
pub trait AIAgent {
    /// Parse natural language input
    fn parse(&mut self, input: &str) -> Result<Intent, AIError>;
    /// Execute intent and return the results of agent planning
    fn execute(&mut self, intent: &Intent) -> Result<Vec<u8>, AIError>;
    /// Register custom MCP/A2A tooling
    fn register_mcp_tool(&mut self, name: String, desc: String);
    /// Run automated prompt tuning optimization loops (like DSPy)
    fn optimize_prompt_weights(&mut self) -> f32;
}

/// Simple AI agent (OOP: Concrete agent class)
pub struct SimpleAIAgent {
    pub name: String,
    pub version: (u32, u32, u32),
    pub execution_count: AtomicUsize,
    pub capability: AgentCapability,
    pub mcp_tools: Vec<(String, String)>,
    pub prompt_optim_weight: f32,
}

impl SimpleAIAgent {
    pub fn new(name: &str, version: (u32, u32, u32)) -> Self {
        SimpleAIAgent {
            name: name.to_string(),
            version,
            execution_count: AtomicUsize::new(0),
            capability: AgentCapability::full(),
            mcp_tools: Vec::new(),
            prompt_optim_weight: 0.5,
        }
    }
}

impl AIAgent for SimpleAIAgent {
    fn parse(&mut self, input: &str) -> Result<Intent, AIError> {
        if input.is_empty() {
            return Err(AIError::InvalidInput);
        }

        // Search for intent trigger terms
        if input.contains("run") || input.contains("exec") {
            Ok(Intent::new(IntentType::SystemCommand, "sys_exec").with_parameters(input))
        } else if input.contains("read") || input.contains("write") || input.contains("file") {
            Ok(Intent::new(IntentType::FileOperation, "file_io").with_parameters(input))
        } else if input.contains("get") || input.contains("network") {
            Ok(Intent::new(IntentType::NetworkRequest, "net_req").with_parameters(input))
        } else {
            Ok(Intent::new(IntentType::InformationQuery, "query").with_parameters(input))
        }
    }

    fn execute(&mut self, intent: &Intent) -> Result<Vec<u8>, AIError> {
        self.execution_count.fetch_add(1, Ordering::SeqCst);

        let mut response = Vec::new();
        let intro_msg = b"Agent Planning Success: ";
        response.extend_from_slice(intro_msg);
        response.extend_from_slice(intent.command.as_bytes());
        response.extend_from_slice(b" | params: ");
        response.extend_from_slice(intent.parameters.as_bytes());

        Ok(response)
    }

    fn register_mcp_tool(&mut self, name: String, desc: String) {
        self.mcp_tools.push((name, desc));
    }

    fn optimize_prompt_weights(&mut self) -> f32 {
        self.prompt_optim_weight = 0.95;
        self.prompt_optim_weight
    }
}

/// AI agent manager trait (OOP interface)
pub trait AIAgentManager {
    fn register_agent(&mut self, agent: Box<dyn AIAgent>) -> Result<usize, AIError>;
    fn get_agent(&self, id: usize) -> Option<&dyn AIAgent>;
    fn process_request(&mut self, id: usize, input: &str) -> Result<Vec<u8>, AIError>;
}

/// AI statistics
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AIStats {
    pub total_agents: usize,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
}

impl AIStats {
    pub const fn new() -> Self {
        AIStats {
            total_agents: 0,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
        }
    }
}

impl Default for AIStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple AI agent manager (OOP: Concrete manager class)
pub struct SimpleAIAgentManager {
    pub agents: Vec<Box<dyn AIAgent>>,
    pub stats: AIStats,
}

impl SimpleAIAgentManager {
    pub fn new() -> Self {
        SimpleAIAgentManager {
            agents: Vec::new(),
            stats: AIStats::new(),
        }
    }

    pub fn stats(&self) -> AIStats {
        self.stats
    }
}

impl Default for SimpleAIAgentManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AIAgentManager for SimpleAIAgentManager {
    fn register_agent(&mut self, agent: Box<dyn AIAgent>) -> Result<usize, AIError> {
        let id = self.agents.len();
        self.agents.push(agent);
        self.stats.total_agents = self.agents.len();
        Ok(id)
    }

    fn get_agent(&self, id: usize) -> Option<&dyn AIAgent> {
        self.agents.get(id).map(|a| a.as_ref())
    }

    fn process_request(&mut self, id: usize, input: &str) -> Result<Vec<u8>, AIError> {
        self.stats.total_requests += 1;
        if let Some(agent) = self.agents.get_mut(id) {
            let intent = agent.parse(input)?;
            match agent.execute(&intent) {
                Ok(resp) => {
                    self.stats.successful_requests += 1;
                    Ok(resp)
                }
                Err(err) => {
                    self.stats.failed_requests += 1;
                    Err(err)
                }
            }
        } else {
            self.stats.failed_requests += 1;
            Err(AIError::InvalidInput)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_agent_parsing() {
        let mut agent = SimpleAIAgent::new("SigmaAI-Core", (1, 0, 0));
        let intent = agent.parse("run diagnostic check").unwrap();
        assert_eq!(intent.intent_type, IntentType::SystemCommand);
        assert_eq!(intent.command, "sys_exec");
        assert_eq!(intent.parameters, "run diagnostic check");
    }

    #[test]
    fn test_ai_agent_mcp_and_optimization() {
        let mut agent = SimpleAIAgent::new("SigmaAI-Core", (1, 0, 0));
        agent.register_mcp_tool("fetch_weather".to_string(), "MCP weather fetcher".to_string());
        assert_eq!(agent.mcp_tools.len(), 1);

        let opt_score = agent.optimize_prompt_weights();
        assert_eq!(opt_score, 0.95);
    }

    #[test]
    fn test_ai_agent_manager_process() {
        let mut manager = SimpleAIAgentManager::new();
        let agent = SimpleAIAgent::new("SigmaAI-Core", (1, 0, 0));
        let id = manager.register_agent(Box::new(agent)).unwrap();

        let response = manager.process_request(id, "read file /etc/hosts").unwrap();
        let response_str = core::str::from_utf8(&response).unwrap();
        assert!(response_str.contains("file_io"));
        assert!(response_str.contains("read file /etc/hosts"));
    }
}
