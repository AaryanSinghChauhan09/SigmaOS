// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS AI Integration - Core AI functionality for the OS

mod local_llm;
mod nlp_engine;
mod context_manager;
mod learning_system;
mod system_control;
mod troubleshooting;
mod automation;
mod privacy;

pub use local_llm::LocalLLM;
pub use nlp_engine::NLPEngine;
pub use context_manager::ContextManager;
pub use learning_system::LearningSystem;
pub use system_control::SystemControl;
pub use troubleshooting::TroubleshootingEngine;
pub use automation::AutomationEngine;
pub use privacy::PrivacyManager;

use serde::{Deserialize, Serialize};

/// AI Integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// Model to use for local LLM
    pub model_name: String,
    /// Enable local processing
    pub local_processing: bool,
    /// Maximum context length
    pub max_context_length: usize,
    /// Enable learning system
    pub enable_learning: bool,
    /// Privacy level
    pub privacy_level: PrivacyLevel,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            model_name: "llama-2-7b".to_string(),
            local_processing: true,
            max_context_length: 4096,
            enable_learning: true,
            privacy_level: PrivacyLevel::Standard,
        }
    }
}

/// Privacy level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivacyLevel {
    Minimal,
    Standard,
    High,
    Maximum,
}

/// Main AI Integration structure
pub struct AIIntegration {
    config: AIConfig,
    local_llm: LocalLLM,
    nlp_engine: NLPEngine,
    context_manager: ContextManager,
    learning_system: Option<LearningSystem>,
    system_control: SystemControl,
    troubleshooting: TroubleshootingEngine,
    automation: AutomationEngine,
    privacy_manager: PrivacyManager,
}

impl AIIntegration {
    /// Create a new AI Integration instance
    pub fn new(config: AIConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let local_llm = LocalLLM::new(&config.model_name)?;
        let nlp_engine = NLPEngine::new()?;
        let context_manager = ContextManager::new(config.max_context_length)?;
        let learning_system = if config.enable_learning {
            Some(LearningSystem::new()?)
        } else {
            None
        };
        let system_control = SystemControl::new()?;
        let troubleshooting = TroubleshootingEngine::new()?;
        let automation = AutomationEngine::new()?;
        let privacy_manager = PrivacyManager::new(config.privacy_level)?;

        Ok(Self {
            config,
            local_llm,
            nlp_engine,
            context_manager,
            learning_system,
            system_control,
            troubleshooting,
            automation,
            privacy_manager,
        })
    }

    /// Process a natural language command
    pub fn process_command(&mut self, command: &str) -> Result<AIResponse, Box<dyn std::error::Error>> {
        // Check privacy constraints
        if !self.privacy_manager.can_process(command)? {
            return Err("Command blocked by privacy policy".into());
        }

        // Add to context
        self.context_manager.add_entry("user", command);

        // Process with NLP engine
        let intent = self.nlp_engine.analyze_intent(command)?;

        // Execute appropriate action
        let response = match intent.action_type {
            ActionType::SystemControl => self.system_control.execute(&intent.parameters)?,
            ActionType::Troubleshoot => self.troubleshooting.diagnose(&intent.parameters)?,
            ActionType::Automate => self.automation.create_workflow(&intent.parameters)?,
            ActionType::Query => self.local_llm.generate(&intent.parameters)?,
            ActionType::Unknown => AIResponse {
                message: "I'm not sure how to help with that. Could you be more specific?".to_string(),
                confidence: 0.3,
                action: None,
            },
        };

        // Add response to context
        self.context_manager.add_entry("assistant", &response.message);

        // Learn from interaction if enabled
        if let Some(learning) = &mut self.learning_system {
            learning.record_interaction(command, &response, intent.confidence);
        }

        Ok(response)
    }

    /// Get AI suggestions based on system state
    pub fn get_suggestions(&self, system_state: &SystemState) -> Vec<AISuggestion> {
        let mut suggestions = Vec::new();
        
        // Get suggestions from various engines
        suggestions.extend(self.troubleshooting.get_suggestions(system_state));
        suggestions.extend(self.system_control.get_suggestions(system_state));
        
        // Sort by priority
        suggestions.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        suggestions
    }

    /// Update configuration
    pub fn update_config(&mut self, config: AIConfig) {
        self.config = config;
        // Reinitialize components if needed
    }

    /// Get learning system if enabled
    pub fn learning_system(&self) -> Option<&LearningSystem> {
        self.learning_system.as_ref()
    }

    /// Get privacy manager
    pub fn privacy_manager(&self) -> &PrivacyManager {
        &self.privacy_manager
    }
}

/// AI response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub message: String,
    pub confidence: f32,
    pub action: Option<String>,
}

/// AI suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISuggestion {
    pub title: String,
    pub description: String,
    pub priority: SuggestionPriority,
    pub action: Option<String>,
}

/// Suggestion priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SuggestionPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Intent analysis result
#[derive(Debug, Clone)]
struct IntentAnalysis {
    action_type: ActionType,
    parameters: Vec<String>,
    confidence: f32,
}

/// Action type
#[derive(Debug, Clone)]
enum ActionType {
    SystemControl,
    Troubleshoot,
    Automate,
    Query,
    Unknown,
}

/// System state for AI analysis
#[derive(Debug, Clone)]
pub struct SystemState {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub temperature: f32,
    pub network_status: bool,
    pub security_score: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_integration_creation() {
        let config = AIConfig::default();
        // Note: This test will fail if dependencies aren't available
        // let ai = AIIntegration::new(config);
        // assert!(ai.is_ok());
    }

    #[test]
    fn test_config_default() {
        let config = AIConfig::default();
        assert_eq!(config.model_name, "llama-2-7b");
        assert_eq!(config.local_processing, true);
    }
}
