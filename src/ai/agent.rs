// SigmaOS AI Agent & Intent Parser
// Custom, OOP-driven AI subsystem for intent parsing, agent execution, and command synthesis.

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentType {
    SystemOptimization = 0,
    SecurityAudit = 1,
    ResourceAllocation = 2,
    CustomCommand = 3,
    Unknown = 4,
}

#[derive(Debug, Clone)]
pub struct Intent {
    pub intent_type: IntentType,
    pub command: String,
    pub parameters: Vec<String>,
    pub confidence: f32,
}

impl Intent {
    pub fn new(intent_type: IntentType, command: String) -> Self {
        Intent {
            intent_type,
            command,
            parameters: Vec::new(),
            confidence: 1.0,
        }
    }

    pub fn with_parameter(mut self, param: String) -> Self {
        self.parameters.push(param);
        self
    }
    
    pub fn with_parameters(mut self, params: &str) -> Self {
        self.parameters = params.to_string();
        self
    }
}

pub trait IntentParser {
    fn parse(&self, input: &str) -> Intent;
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

pub struct SimpleIntentParser;

impl IntentParser for SimpleIntentParser {
    fn parse(&self, input: &str) -> Intent {
        let trimmed = input.trim();
        if trimmed.starts_with("optimize") {
            Intent::new(IntentType::SystemOptimization, trimmed.to_string())
        } else if trimmed.starts_with("audit") || trimmed.starts_with("scan") {
            Intent::new(IntentType::SecurityAudit, trimmed.to_string())
        } else if trimmed.starts_with("allocate") || trimmed.starts_with("memory") {
            Intent::new(IntentType::ResourceAllocation, trimmed.to_string())
        } else {
            Intent::new(IntentType::CustomCommand, trimmed.to_string())
        }
    }
}

pub trait AIAgent {
    fn name(&self) -> &str;
    fn execute_intent(&mut self, intent: &Intent) -> Result<String, &'static str>;
}

pub struct SystemOptimizerAgent {
    pub name: String,
    pub execution_count: AtomicUsize,
}

impl SystemOptimizerAgent {
    pub fn new() -> Self {
        SystemOptimizerAgent {
            name: "SystemOptimizer".to_string(),
            execution_count: AtomicUsize::new(0),
        }
    }
}

impl Default for SystemOptimizerAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl AIAgent for SystemOptimizerAgent {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_intent(&mut self, intent: &Intent) -> Result<String, &'static str> {
        self.execution_count.fetch_add(1, Ordering::SeqCst);
        match intent.intent_type {
            IntentType::SystemOptimization => Ok("System optimization executed successfully.".to_string()),
            IntentType::SecurityAudit => Ok("Security audit executed successfully.".to_string()),
            IntentType::ResourceAllocation => Ok("Resource allocation updated.".to_string()),
            _ => Ok("Custom agent task completed.".to_string()),
        }
    }
}

pub struct AgentOrchestrator {
    pub agents: Vec<Box<dyn AIAgent>>,
    pub parser: SimpleIntentParser,
}

impl AgentOrchestrator {
    pub fn new() -> Self {
        AgentOrchestrator {
            agents: Vec::new(),
            parser: SimpleIntentParser,
        }
    }

    pub fn register_agent(&mut self, agent: Box<dyn AIAgent>) {
        self.agents.push(agent);
    }

    pub fn process_input(&mut self, input: &str) -> Result<String, &'static str> {
        let intent = self.parser.parse(input);
        if let Some(agent) = self.agents.first_mut() {
            agent.execute_intent(&intent)
        } else {
            Err("No active AI agent registered to process intent.")
        }
    }
}

impl Default for AgentOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_parser() {
        let parser = SimpleIntentParser;
        let intent = parser.parse("optimize memory");
        assert_eq!(intent.intent_type, IntentType::SystemOptimization);

        let intent_sec = parser.parse("audit security");
        assert_eq!(intent_sec.intent_type, IntentType::SecurityAudit);
    }

    #[test]
    fn test_orchestrator() {
        let mut orchestrator = AgentOrchestrator::new();
        orchestrator.register_agent(Box::new(SystemOptimizerAgent::new()));

        let res = orchestrator.process_input("optimize kernel").unwrap();
        assert_eq!(res, "System optimization executed successfully.");
    }
}
