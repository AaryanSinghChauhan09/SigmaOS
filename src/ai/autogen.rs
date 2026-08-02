// (no_std only applicable at crate root - removed)
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;

/// Intent/Message structures for Multi-Agent AutoGen Simulation
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentRole {
    Assistant = 0,
    UserProxy = 1,
    Critic = 2,
    Admin = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoGenError {
    Success = 0,
    AgentNotFound = 1,
    ExecutionFailed = 2,
    GroupChatFull = 3,
    SandboxViolation = 4,
}

/// Message payload representing a chat transmission between AutoGen Agents
pub struct AutoGenMessage {
    pub sender_id: usize,
    pub role: AgentRole,
    pub content: [u8; 256],
    pub content_len: usize,
}

impl AutoGenMessage {
    pub fn new(sender_id: usize, role: AgentRole, content: &[u8]) -> Self {
        let mut msg = [0u8; 256];
        let len = content.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(content.as_ptr(), msg.as_mut_ptr(), len);
        }
        AutoGenMessage {
            sender_id,
            role,
            content: msg,
            content_len: len,
        }
    }
}

/// Tool representing registered custom callable commands (Function Calling)
pub struct AutoGenTool {
    pub name: [u8; 64],
    pub name_len: usize,
    pub description: [u8; 128],
    pub description_len: usize,
    pub capability_id: u32,
}

impl AutoGenTool {
    pub fn new(name: &[u8], description: &[u8], capability_id: u32) -> Self {
        let mut n = [0u8; 64];
        let mut d = [0u8; 128];
        let nl = name.len().min(63);
        let dl = description.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), n.as_mut_ptr(), nl);
            core::ptr::copy_nonoverlapping(description.as_ptr(), d.as_mut_ptr(), dl);
        }
        AutoGenTool {
            name: n,
            name_len: nl,
            description: d,
            description_len: dl,
            capability_id,
        }
    }
}

/// Conversable Agent containing a local message history log (AutoGen philosophy)
pub struct ConversableAgent {
    pub id: usize,
    pub name: [u8; 64],
    pub name_len: usize,
    pub role: AgentRole,
    pub history: Vec<AutoGenMessage>,
    pub tools: Vec<AutoGenTool>,
    pub human_input_mode: bool, // Human-in-the-loop enabled flag
}

impl ConversableAgent {
    pub fn new(id: usize, name: &[u8], role: AgentRole, human_input_mode: bool) -> Self {
        let mut n = [0u8; 64];
        let nl = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), n.as_mut_ptr(), nl);
        }
        ConversableAgent {
            id,
            name: n,
            name_len: nl,
            role,
            history: Vec::new(),
            tools: Vec::new(),
            human_input_mode,
        }
    }

    pub fn register_tool(&mut self, tool: AutoGenTool) {
        self.tools.push(tool);
    }

    pub fn receive_message(&mut self, message: AutoGenMessage) {
        self.history.push(message);
    }

    /// Simulates LLM response generation or tool execution trigger based on input conversation context
    pub fn generate_reply(&self) -> Result<AutoGenMessage, AutoGenError> {
        if self.history.len() == 0 {
            return Err(AutoGenError::ExecutionFailed);
        }

        let last_msg = &self.history[self.history.len() - 1];
        let last_content = &last_msg.content[..last_msg.content_len];

        // Trigger function calling simulation if message asks to calculate or fetch stats
        let is_tool_request = last_content.windows(4).any(|w| w == b"calc" || w == b"fetch");
        if is_tool_request && self.tools.len() > 0 {
            let tool = &self.tools[0];
            let mut reply_content = [0u8; 128];
            let reply_prefix = b"Executing registered capability: ";
            unsafe {
                core::ptr::copy_nonoverlapping(reply_prefix.as_ptr(), reply_content.as_mut_ptr(), reply_prefix.len());
                core::ptr::copy_nonoverlapping(tool.name.as_ptr(), reply_content.as_mut_ptr().add(reply_prefix.len()), tool.name_len);
            }
            let total_len = reply_prefix.len() + tool.name_len;
            return Ok(AutoGenMessage::new(self.id, self.role, &reply_content[..total_len]));
        }

        // Standard response based on roles
        match self.role {
            AgentRole::Assistant => {
                Ok(AutoGenMessage::new(self.id, self.role, b"I have analyzed your input and verified standard system health."))
            }
            AgentRole::Critic => {
                Ok(AutoGenMessage::new(self.id, self.role, b"Critique: The proposed architectural plan conforms perfectly to safety limits."))
            }
            _ => {
                Ok(AutoGenMessage::new(self.id, self.role, b"Awaiting next turn."))
            }
        }
    }
}

/// Group Chat manager orchestrating multi-agent collaboration environments (AutoGen group chat)
pub struct GroupChat {
    pub agents: Vec<ConversableAgent>,
    pub max_rounds: usize,
    pub speaker_selection_strategy: [u8; 32],
}

impl GroupChat {
    pub fn new(max_rounds: usize) -> Self {
        GroupChat {
            agents: Vec::new(),
            max_rounds,
            speaker_selection_strategy: [0; 32],
        }
    }

    pub fn add_agent(&mut self, agent: ConversableAgent) -> Result<(), AutoGenError> {
        if self.agents.len() >= 10 {
            return Err(AutoGenError::GroupChatFull);
        }
        self.agents.push(agent);
        Ok(())
    }

    /// Broadcasters a message from a sender agent to all other conversing agents in the group chat
    pub fn broadcast_message(&mut self, sender_id: usize, content: &[u8]) -> Result<(), AutoGenError> {
        let mut sender_role = AgentRole::UserProxy;
        let mut found = false;

        for agent in &self.agents {
            if agent.id == sender_id {
                sender_role = agent.role;
                found = true;
                break;
            }
        }

        if !found {
            return Err(AutoGenError::AgentNotFound);
        }

        for agent in &mut self.agents {
            agent.receive_message(AutoGenMessage::new(sender_id, sender_role, content));
        }

        Ok(())
    }
}

/// Secure sandboxed executor simulator (AutoGen local code executor with permission boundaries)
pub struct SandboxCodeExecutor {
    pub max_cpu_cycles: u64,
    pub network_gated: bool,
}

impl SandboxCodeExecutor {
    pub fn new(max_cpu_cycles: u64, network_gated: bool) -> Self {
        SandboxCodeExecutor {
            max_cpu_cycles,
            network_gated,
        }
    }

    /// Verifies if a generated command/code matches safety criteria before executing in Ring 3
    pub fn execute_code_sandboxed(&self, code: &[u8]) -> Result<Vec<u8>, AutoGenError> {
        // Block obvious exploits or privilege escalations
        let has_escalation = code.windows(4).any(|w| w == b"root" || w == b"sudo") || code.windows(5).any(|w| w == b"chmod");
        if has_escalation {
            return Err(AutoGenError::SandboxViolation);
        }

        let mut output = Vec::new();
        let success_msg = b"Execution output: Command executed inside clean sandbox.";
        for &byte in success_msg {
            output.push(byte);
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autogen_conversable_agent() {
        let mut agent = ConversableAgent::new(1, b"AssistantAgent", AgentRole::Assistant, false);
        let tool = AutoGenTool::new(b"calculator", b"Performs math", 101);
        agent.register_tool(tool);

        // Broadcasters user request
        let msg = AutoGenMessage::new(2, AgentRole::UserProxy, b"Please calculate server uptime stats");
        agent.receive_message(msg);

        // Generate automated reply utilizing function calling template
        let reply = agent.generate_reply().unwrap();
        assert!(reply.content_len > 0);
        let content = &reply.content[..reply.content_len];
        assert!(content.windows(10).any(|w| w == b"calculator"));
    }

    #[test]
    fn test_autogen_group_chat() {
        let mut group = GroupChat::new(10);
        let agent1 = ConversableAgent::new(1, b"UserProxy", AgentRole::UserProxy, true);
        let agent2 = ConversableAgent::new(2, b"CriticAgent", AgentRole::Critic, false);

        assert!(group.add_agent(agent1).is_ok());
        assert!(group.add_agent(agent2).is_ok());

        assert!(group.broadcast_message(1, b"Let's draft system rollback policies").is_ok());
        assert_eq!(group.agents[0].history.len(), 1);
        assert_eq!(group.agents[1].history.len(), 1);
    }

    #[test]
    fn test_autogen_sandbox_executor() {
        let executor = SandboxCodeExecutor::new(1000000, true);

        // Safe command
        let safe_res = executor.execute_code_sandboxed(b"echo 1");
        assert!(safe_res.is_ok());

        // Dangerous command violating security boundaries
        let dangerous_res = executor.execute_code_sandboxed(b"sudo rm -rf /");
        assert_eq!(dangerous_res.err(), Some(AutoGenError::SandboxViolation));
    }
}
