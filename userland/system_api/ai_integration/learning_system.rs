// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Learning System - Learning from user behavior

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Learning System for adapting to user behavior
pub struct LearningSystem {
    interaction_history: Vec<InteractionRecord>,
    user_preferences: UserPreferences,
    command_patterns: HashMap<String, CommandPattern>,
}

impl LearningSystem {
    /// Create a new Learning System
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            interaction_history: Vec::new(),
            user_preferences: UserPreferences::default(),
            command_patterns: HashMap::new(),
        })
    }

    /// Record an interaction for learning
    pub fn record_interaction(&mut self, command: &str, response: &super::AIResponse, confidence: f32) {
        let record = InteractionRecord {
            command: command.to_string(),
            response: response.message.clone(),
            confidence,
            timestamp: chrono::Utc::now().to_rfc3339(),
            user_satisfaction: None, // Would be filled in by user feedback
        };
        
        self.interaction_history.push(record);
        
        // Learn from this interaction
        self.learn_from_interaction(command, response);
    }

    /// Learn from an interaction
    fn learn_from_interaction(&mut self, command: &str, response: &super::AIResponse) {
        // Extract command pattern
        let pattern = self.extract_pattern(command);
        
        // Update pattern statistics
        let pattern_entry = self.command_patterns
            .entry(pattern.clone())
            .or_insert_with(|| CommandPattern {
                pattern,
                count: 0,
                success_count: 0,
                last_used: None,
            });
        
        pattern_entry.count += 1;
        pattern_entry.last_used = Some(chrono::Utc::now().to_rfc3339());
        
        if response.confidence > 0.7 {
            pattern_entry.success_count += 1;
        }
    }

    /// Extract a pattern from a command
    fn extract_pattern(&self, command: &str) -> String {
        // Simple pattern extraction - replace specific values with placeholders
        let mut pattern = command.to_lowercase();
        
        // Replace common software names with placeholder
        let software_names = vec!["cuda", "docker", "kubernetes", "python", "rust"];
        for name in software_names {
            pattern = pattern.replace(name, "<software>");
        }
        
        // Replace numbers with placeholder
        pattern = pattern.replace(r"\d+", "<number>");
        
        pattern
    }

    /// Get suggestions based on learned patterns
    pub fn get_suggestions(&self, partial_command: &str) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();
        
        // Find matching patterns
        for (pattern, pattern_info) in &self.command_patterns {
            if pattern.contains(partial_command) {
                let success_rate = if pattern_info.count > 0 {
                    pattern_info.success_count as f32 / pattern_info.count as f32
                } else {
                    0.0
                };
                
                if success_rate > 0.7 {
                    suggestions.push(Suggestion {
                        pattern: pattern.clone(),
                        confidence: success_rate,
                        usage_count: pattern_info.count,
                    });
                }
            }
        }
        
        // Sort by confidence
        suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        suggestions
    }

    /// Update user preferences
    pub fn update_preferences(&mut self, preferences: UserPreferences) {
        self.user_preferences = preferences;
    }

    /// Get user preferences
    pub fn get_preferences(&self) -> &UserPreferences {
        &self.user_preferences
    }

    /// Get interaction statistics
    pub fn get_statistics(&self) -> LearningStatistics {
        let total_interactions = self.interaction_history.len();
        let successful_interactions = self.interaction_history
            .iter()
            .filter(|r| r.confidence > 0.7)
            .count();
        
        let success_rate = if total_interactions > 0 {
            successful_interactions as f32 / total_interactions as f32
        } else {
            0.0
        };
        
        LearningStatistics {
            total_interactions,
            successful_interactions,
            success_rate,
            unique_patterns: self.command_patterns.len(),
        }
    }

    /// Export learning data
    pub fn export_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        serde_json::to_string_pretty(&self.interaction_history).map_err(Into::into)
    }

    /// Import learning data
    pub fn import_data(&mut self, data: &str) -> Result<(), Box<dyn std::error::Error>> {
        let imported: Vec<InteractionRecord> = serde_json::from_str(data)?;
        self.interaction_history.extend(imported);
        Ok(())
    }
}

/// Interaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionRecord {
    pub command: String,
    pub response: String,
    pub confidence: f32,
    pub timestamp: String,
    pub user_satisfaction: Option<f32>,
}

/// User preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub response_brevity: BrevityLevel,
    pub technical_detail: TechnicalDetailLevel,
    pub proactive_suggestions: bool,
    pub learning_enabled: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            response_brevity: BrevityLevel::Medium,
            technical_detail: TechnicalDetailLevel::Medium,
            proactive_suggestions: true,
            learning_enabled: true,
        }
    }
}

/// Brevity level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrevityLevel {
    Concise,
    Medium,
    Detailed,
}

/// Technical detail level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TechnicalDetailLevel {
    Simple,
    Medium,
    Technical,
}

/// Command pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPattern {
    pub pattern: String,
    pub count: usize,
    pub success_count: usize,
    pub last_used: Option<String>,
}

/// Suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub pattern: String,
    pub confidence: f32,
    pub usage_count: usize,
}

/// Learning statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStatistics {
    pub total_interactions: usize,
    pub successful_interactions: usize,
    pub success_rate: f32,
    pub unique_patterns: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learning_system_creation() {
        let system = LearningSystem::new();
        assert!(system.is_ok());
    }

    #[test]
    fn test_record_interaction() {
        let mut system = LearningSystem::new().unwrap();
        let response = super::super::AIResponse {
            message: "Test response".to_string(),
            confidence: 0.9,
            action: None,
        };
        system.record_interaction("test command", &response, 0.9);
        assert_eq!(system.get_statistics().total_interactions, 1);
    }
}
