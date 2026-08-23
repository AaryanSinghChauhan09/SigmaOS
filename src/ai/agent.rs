// OOP-based AI Agent Framework for SigmaOS
// Implements AI agent using OOP principles with traits and structs.

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Intent type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentType {
    SystemCommand = 0,
    FileOperation = 1,
    NetworkRequest = 2,
    ApplicationLaunch = 3,
    InformationQuery = 4,
    Custom = 5,
}

/// Intent
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentCapability {
    pub can_parse: bool,
    pub can_execute: bool,
    pub can_learn: bool,
}

impl AgentCapability {
    pub fn new() -> Self {
        AgentCapability {
            can_parse: false,
            can_execute: false,
            can_learn: false,
        }
    }

    pub fn full() -> Self {
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

/// AI agent trait
pub trait AIAgent {
    fn parse(&mut self, input: &str) -> Result<Intent, AIError>;
    fn execute(&mut self, intent: &Intent) -> Result<Vec<u8>, AIError>;
    fn register_mcp_tool(&mut self, name: String, desc: String);
    fn optimize_prompt_weights(&mut self) -> f32;
}

/// Simple AI agent
pub struct SimpleAIAgent {
    pub name: String,
    pub version: (u32, u32, u32),
    pub execution_count: AtomicUsize,
    pub mcp_tools: Vec<(String, String)>,
    pub prompt_optim_weight: f32,
}

impl SimpleAIAgent {
    pub fn new(name: &str, version: (u32, u32, u32)) -> Self {
        SimpleAIAgent {
            name: name.to_string(),
            version,
            execution_count: AtomicUsize::new(0),
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

/// AI agent manager trait
pub trait AIAgentManager {
    fn register_agent(&mut self, agent: Box<dyn AIAgent>) -> Result<usize, AIError>;
    fn get_agent(&self, id: usize) -> Option<&dyn AIAgent>;
    fn process_request(&mut self, id: usize, input: &str) -> Result<Vec<u8>, AIError>;
}

/// Simple AI agent manager
pub struct SimpleAIAgentManager {
    pub agents: Vec<Box<dyn AIAgent>>,
}

impl SimpleAIAgentManager {
    pub fn new() -> Self {
        SimpleAIAgentManager {
            agents: Vec::new(),
        }
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
        Ok(id)
    }

    fn get_agent(&self, id: usize) -> Option<&dyn AIAgent> {
        self.agents.get(id).map(|a| a.as_ref())
    }

    fn process_request(&mut self, id: usize, input: &str) -> Result<Vec<u8>, AIError> {
        if let Some(agent) = self.agents.get_mut(id) {
            let intent = agent.parse(input)?;
            agent.execute(&intent)
        } else {
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
        let response_str = std::str::from_utf8(&response).unwrap();
        assert!(response_str.contains("file_io"));
        assert!(response_str.contains("read file /etc/hosts"));
    }
}
