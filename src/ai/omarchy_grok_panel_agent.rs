// SPDX-License-Identifier: MIT
// SigmaOS — Omarchy Grok AI Panel Agent
// Inspired by Omarchy branch: agent-panel-grok
// Zero external dependencies, Safe Rust

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

#[cfg(any(feature = "standalone_test", test))]
use std::{format, string::String, vec::Vec};
#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::{format, string::String, vec::Vec};

/// AI provider for panel agent
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelAiProvider {
    Grok,
    Gpt4o,
    Claude,
    Gemini,
    Ollama,
    SigmaLocal,
}

impl PanelAiProvider {
    pub fn name(&self) -> &'static str {
        match self {
            PanelAiProvider::Grok => "xAI Grok",
            PanelAiProvider::Gpt4o => "OpenAI GPT-4o",
            PanelAiProvider::Claude => "Anthropic Claude",
            PanelAiProvider::Gemini => "Google Gemini",
            PanelAiProvider::Ollama => "Ollama (local)",
            PanelAiProvider::SigmaLocal => "SigmaOS Local LLM",
        }
    }

    pub fn api_base(&self) -> &'static str {
        match self {
            PanelAiProvider::Grok => "https://api.x.ai/v1",
            PanelAiProvider::Gpt4o => "https://api.openai.com/v1",
            PanelAiProvider::Claude => "https://api.anthropic.com/v1",
            PanelAiProvider::Gemini => "https://generativelanguage.googleapis.com/v1",
            PanelAiProvider::Ollama => "http://localhost:11434/v1",
            PanelAiProvider::SigmaLocal => "http://localhost:52000/v1",
        }
    }
}

/// A panel agent conversation message
#[derive(Debug, Clone)]
pub struct PanelMessage {
    pub role: String,
    pub content: String,
    pub timestamp_ms: u64,
}

/// Panel agent state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelState {
    Idle,
    Thinking,
    Responding,
    Error,
}

/// SigmaOS Grok AI Panel Agent
/// Surpasses Omarchy agent-panel-grok with multi-provider, tool-calling support
pub struct OmarchyGrokPanelAgent {
    provider: PanelAiProvider,
    model: String,
    system_prompt: String,
    history: Vec<PanelMessage>,
    state: PanelState,
    max_history: usize,
    tool_calls_enabled: bool,
}

impl OmarchyGrokPanelAgent {
    pub fn new(provider: PanelAiProvider) -> Self {
        let model = match provider {
            PanelAiProvider::Grok => String::from("grok-3"),
            PanelAiProvider::Gpt4o => String::from("gpt-4o"),
            PanelAiProvider::Claude => String::from("claude-3-5-sonnet-20241022"),
            PanelAiProvider::Gemini => String::from("gemini-2.0-flash"),
            PanelAiProvider::Ollama => String::from("llama3.2"),
            PanelAiProvider::SigmaLocal => String::from("sigma-7b"),
        };
        Self {
            provider,
            model,
            system_prompt: String::from(
                "You are the SigmaOS AI panel agent. Help the user with system tasks, \
                 code, queries, and productivity. You have access to system tools.",
            ),
            history: Vec::new(),
            state: PanelState::Idle,
            max_history: 50,
            tool_calls_enabled: true,
        }
    }

    /// Set a custom system prompt
    pub fn set_system_prompt(&mut self, prompt: &str) {
        self.system_prompt = String::from(prompt);
    }

    /// Set the model name override
    pub fn set_model(&mut self, model: &str) {
        self.model = String::from(model);
    }

    /// Send a user message and simulate a response (in prod: calls API)
    pub fn send_user_message(&mut self, content: &str) -> String {
        self.history.push(PanelMessage {
            role: String::from("user"),
            content: String::from(content),
            timestamp_ms: 0,
        });

        // Truncate history to max_history
        if self.history.len() > self.max_history {
            let overflow = self.history.len() - self.max_history;
            self.history.drain(0..overflow);
        }

        self.state = PanelState::Thinking;

        // In production this would call the API; here we return a structured placeholder
        let response = format!(
            "[{}·{}] Processing: \"{}\" — {} tools enabled",
            self.provider.name(),
            self.model,
            content,
            if self.tool_calls_enabled {
                "with"
            } else {
                "without"
            }
        );

        self.history.push(PanelMessage {
            role: String::from("assistant"),
            content: response.clone(),
            timestamp_ms: 1,
        });
        self.state = PanelState::Idle;
        response
    }

    /// Generate the JSON request body for the current conversation
    pub fn build_api_request_json(&self) -> String {
        let mut msgs = String::from("[");
        msgs.push_str(&format!(
            r#"{{"role":"system","content":"{}"}}"#,
            self.system_prompt
        ));
        for m in &self.history {
            msgs.push_str(&format!(
                r#",{{"role":"{}","content":"{}"}}"#,
                m.role, m.content
            ));
        }
        msgs.push(']');
        format!(
            r#"{{"model":"{}","messages":{},"stream":false}}"#,
            self.model, msgs
        )
    }

    /// Switch AI provider on the fly
    pub fn switch_provider(&mut self, provider: PanelAiProvider) {
        self.provider = provider;
        self.model = match provider {
            PanelAiProvider::Grok => String::from("grok-3"),
            PanelAiProvider::Gpt4o => String::from("gpt-4o"),
            PanelAiProvider::Claude => String::from("claude-3-5-sonnet-20241022"),
            PanelAiProvider::Gemini => String::from("gemini-2.0-flash"),
            PanelAiProvider::Ollama => String::from("llama3.2"),
            PanelAiProvider::SigmaLocal => String::from("sigma-7b"),
        };
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
        self.state = PanelState::Idle;
    }

    pub fn message_count(&self) -> usize {
        self.history.len()
    }

    pub fn state(&self) -> PanelState {
        self.state
    }

    pub fn provider(&self) -> PanelAiProvider {
        self.provider
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grok_panel_agent() {
        // Default Grok provider
        let mut agent = OmarchyGrokPanelAgent::new(PanelAiProvider::Grok);
        assert_eq!(agent.provider(), PanelAiProvider::Grok);
        assert_eq!(agent.message_count(), 0);

        // Send messages
        let resp = agent.send_user_message("Show system temperature");
        assert!(resp.contains("grok-3") || resp.contains("Grok"));
        assert_eq!(agent.message_count(), 2); // user + assistant

        // Switch to local
        agent.switch_provider(PanelAiProvider::SigmaLocal);
        assert_eq!(agent.provider(), PanelAiProvider::SigmaLocal);
        assert!(agent.provider().api_base().contains("52000"));

        // API request body
        let json = agent.build_api_request_json();
        assert!(json.contains("sigma-7b"));
        assert!(json.contains("messages"));

        // Clear
        agent.clear_history();
        assert_eq!(agent.message_count(), 0);
        assert_eq!(agent.state(), PanelState::Idle);

        // Gemini
        let g = OmarchyGrokPanelAgent::new(PanelAiProvider::Gemini);
        assert!(g.provider().api_base().contains("googleapis"));
    }
}
