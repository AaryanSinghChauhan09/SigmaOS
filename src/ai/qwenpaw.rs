#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// QwenPaw Personal Agent Workstation Integration Module
//
// Formally implements compilable, production-ready Rust structures for the absorbed QwenPaw Personal Assistant:
// 1. PawThreeLayerMemory (Live context, verbatim history, distilled knowledge)
// 2. PawToolGuard & PawFileGuard (Security layers)
// 3. PawAgentCommunicationProtocol (Multi-agent orchestration/ACP)

use crate::klib::VecDeque;

pub struct PawThreeLayerMemory {
    pub live_context: Vec<String>,
    pub verbatim_history: Vec<String>,
    pub distilled_knowledge: Vec<String>,
    pub max_live_turns: usize,
}

impl PawThreeLayerMemory {
    pub fn new(max_live_turns: usize) -> Self {
        Self {
            live_context: Vec::new(),
            verbatim_history: Vec::new(),
            distilled_knowledge: Vec::new(),
            max_live_turns,
        }
    }

    pub fn add_turn(&mut self, prompt: String, response: String) {
        let turn = format!("User: {}\nAssistant: {}", prompt, response);
        self.verbatim_history.push(turn.clone());
        self.live_context.push(turn);

        // Evict older turns if limit exceeded, but keep them verbatim
        if self.live_context.len() > self.max_live_turns {
            let evicted = self.live_context.remove(0);
            self.distill_knowledge(&evicted);
        }
    }

    pub fn distill_knowledge(&mut self, turn_data: &str) {
        let summary = format!("Distilled summary of historical event: {}", turn_data);
        self.distilled_knowledge.push(summary);
    }

    pub fn recall_all_history(&self) -> Vec<String> {
        self.verbatim_history.clone()
    }
}

pub struct PawToolGuard {
    pub blocked_keywords: Vec<String>,
}

impl PawToolGuard {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            blocked_keywords: vec![
                "rm -rf /".to_string(),
                ":(){ :|:& };:".to_string(),
                "sh -i".to_string(),
                "nc -e".to_string(),
            ],
        }
    }

    pub fn inspect_command(&self, cmd: &str) -> Result<(), &'static str> {
        for keyword in &self.blocked_keywords {
            if cmd.contains(keyword) {
                return Err("ToolGuard Block: Dangerous execution pattern detected");
            }
        }
        Ok(())
    }
}

impl Default for PawToolGuard {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PawFileGuard {
    pub protected_paths: Vec<String>,
}

impl PawFileGuard {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            protected_paths: vec![
                "/etc/shadow".to_string(),
                "~/.ssh".to_string(),
                "~/.qwenpaw.secret".to_string(),
            ],
        }
    }

    pub fn inspect_access(&self, path: &str) -> Result<(), &'static str> {
        for protected in &self.protected_paths {
            if path.contains(protected) {
                return Err("FileGuard Block: Access to sensitive path is prohibited");
            }
        }
        Ok(())
    }
}

impl Default for PawFileGuard {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PawAgentMessage {
    pub sender: String,
    pub recipient: String,
    pub payload: String,
}

pub struct PawAgentCommunicationProtocol {
    pub active_agents: Vec<String>,
    pub message_queue: VecDeque<PawAgentMessage>,
}

impl PawAgentCommunicationProtocol {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            active_agents: Vec::new(),
            message_queue: VecDeque::new(),
        }
    }

    pub fn register_agent(&mut self, name: &str) {
        self.active_agents.push(name.to_string());
    }

    pub fn send_message(&mut self, sender: &str, recipient: &str, payload: &str) -> Result<(), &'static str> {
        if !self.active_agents.contains(&sender.to_string()) || !self.active_agents.contains(&recipient.to_string()) {
            return Err("Sender or recipient agent not registered");
        }
        self.message_queue.push_back(PawAgentMessage {
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            payload: payload.to_string(),
        });
        Ok(())
    }

    pub fn dispatch_next(&mut self) -> Option<PawAgentMessage> {
        self.message_queue.pop_front()
    }
}

impl Default for PawAgentCommunicationProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_layer_memory() {
        let mut memory = PawThreeLayerMemory::new(2);
        memory.add_turn("ping".to_string(), "pong".to_string());
        memory.add_turn("hello".to_string(), "world".to_string());
        assert_eq!(memory.live_context.len(), 2);
        assert_eq!(memory.verbatim_history.len(), 2);

        memory.add_turn("test".to_string(), "pass".to_string());
        // Live context is capped, but verbatim history retains all
        assert_eq!(memory.live_context.len(), 2);
        assert_eq!(memory.verbatim_history.len(), 3);
        assert_eq!(memory.distilled_knowledge.len(), 1);
    }

    #[test]
    fn test_tool_and_file_guards() {
        let tool_guard = PawToolGuard::new();
        assert!(tool_guard.inspect_command("echo 'hello'").is_ok());
        assert!(tool_guard.inspect_command("rm -rf /").is_err());

        let file_guard = PawFileGuard::new();
        assert!(file_guard.inspect_access("/home/user/document.txt").is_ok());
        assert!(file_guard.inspect_access("/etc/shadow").is_err());
    }

    #[test]
    fn test_acp_coordination() {
        let mut acp = PawAgentCommunicationProtocol::new();
        acp.register_agent("AgentA");
        acp.register_agent("AgentB");

        assert!(acp.send_message("AgentA", "AgentB", "Initiate scan").is_ok());
        assert!(acp.send_message("AgentA", "AgentC", "Initiate scan").is_err());

        let msg = acp.dispatch_next().unwrap();
        assert_eq!(msg.sender, "AgentA");
        assert_eq!(msg.recipient, "AgentB");
        assert_eq!(msg.payload, "Initiate scan");
    }
}
