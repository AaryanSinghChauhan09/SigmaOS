// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Troubleshooting Engine - AI-powered troubleshooting

use serde::{Deserialize, Serialize};

/// Troubleshooting Engine for AI-powered system diagnostics
pub struct TroubleshootingEngine {
    knowledge_base: Vec<TroubleshootingEntry>,
}

impl TroubleshootingEngine {
    /// Create a new Troubleshooting Engine
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            knowledge_base: Self::load_knowledge_base(),
        })
    }

    /// Load troubleshooting knowledge base
    fn load_knowledge_base() -> Vec<TroubleshootingEntry> {
        vec![
            TroubleshootingEntry {
                id: "thermal_throttling".to_string(),
                symptoms: vec!["high temperature".to_string(), "slow performance".to_string()],
                causes: vec!["dust buildup".to_string(), "inadequate cooling".to_string(), "high ambient temperature".to_string()],
                solutions: vec![
                    "Clean dust from fans and heatsinks".to_string(),
                    "Ensure proper ventilation".to_string(),
                    "Check thermal paste application".to_string(),
                    "Adjust power settings".to_string(),
                ],
                severity: Severity::Medium,
            },
            TroubleshootingEntry {
                id: "memory_leak".to_string(),
                symptoms: vec!["high memory usage".to_string(), "slow performance".to_string(), "system freeze".to_string()],
                causes: vec!["faulty application".to_string(), "memory leak in software".to_string()],
                solutions: vec![
                    "Identify memory-intensive processes".to_string(),
                    "Restart affected applications".to_string(),
                    "Check for application updates".to_string(),
                    "Increase swap space".to_string(),
                ],
                severity: Severity::High,
            },
            TroubleshootingEntry {
                id: "network_disconnect".to_string(),
                symptoms: vec!["no internet".to_string(), "network timeout".to_string()],
                causes: vec!["driver issue".to_string(), "hardware problem".to_string(), "router issue".to_string()],
                solutions: vec![
                    "Restart network interface".to_string(),
                    "Update network drivers".to_string(),
                    "Check router connectivity".to_string(),
                    "Verify cable connections".to_string(),
                ],
                severity: Severity::Medium,
            },
        ]
    }

    /// Diagnose an issue based on symptoms
    pub fn diagnose(&self, parameters: &[String]) -> Result<super::AIResponse, Box<dyn std::error::Error>> {
        if let Some(symptom) = parameters.first() {
            let matches = self.find_matches(symptom);
            
            if let Some(entry) = matches.first() {
                let message = self.format_diagnosis(entry);
                Ok(super::AIResponse {
                    message,
                    confidence: 0.85,
                    action: Some(format!("troubleshoot_{}", entry.id)),
                })
            } else {
                Ok(super::AIResponse {
                    message: "I couldn't identify the specific issue. Could you provide more details about the symptoms?".to_string(),
                    confidence: 0.4,
                    action: None,
                })
            }
        } else {
            Ok(super::AIResponse {
                message: "Please describe the issue you're experiencing".to_string(),
                confidence: 0.0,
                action: None,
            })
        }
    }

    /// Find matching troubleshooting entries
    fn find_matches(&self, symptom: &str) -> Vec<TroubleshootingEntry> {
        let symptom_lower = symptom.to_lowercase();
        
        self.knowledge_base
            .iter()
            .filter(|entry| {
                entry.symptoms.iter().any(|s| s.contains(&symptom_lower))
            })
            .cloned()
            .collect()
    }

    /// Format diagnosis message
    fn format_diagnosis(&self, entry: &TroubleshootingEntry) -> String {
        let causes = entry.causes.join(", ");
        let solutions = entry.solutions.iter().enumerate().map(|(i, s)| format!("{}. {}", i + 1, s)).collect::<Vec<_>>().join("\n");
        
        format!(
            "Based on your symptoms, this appears to be: {}\n\nPossible causes: {}\n\nSuggested solutions:\n{}",
            entry.id, causes, solutions
        )
    }

    /// Get suggestions based on system state
    pub fn get_suggestions(&self, system_state: &super::SystemState) -> Vec<super::AISuggestion> {
        let mut suggestions = Vec::new();
        
        if system_state.temperature > 80.0 {
            suggestions.push(super::AISuggestion {
                title: "Potential Thermal Issue".to_string(),
                description: "System temperature is elevated. Check cooling system and consider cleaning dust from fans.".to_string(),
                priority: super::SuggestionPriority::High,
                action: Some("check_thermal".to_string()),
            });
        }
        
        if !system_state.network_status {
            suggestions.push(super::AISuggestion {
                title: "Network Connectivity Issue".to_string(),
                description: "Network appears to be disconnected. Check network settings and physical connections.".to_string(),
                priority: super::SuggestionPriority::Medium,
                action: Some("check_network".to_string()),
            });
        }
        
        suggestions
    }

    /// Add a new troubleshooting entry
    pub fn add_entry(&mut self, entry: TroubleshootingEntry) {
        self.knowledge_base.push(entry);
    }

    /// Get all troubleshooting entries
    pub fn get_entries(&self) -> Vec<TroubleshootingEntry> {
        self.knowledge_base.clone()
    }
}

/// Troubleshooting entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TroubleshootingEntry {
    pub id: String,
    pub symptoms: Vec<String>,
    pub causes: Vec<String>,
    pub solutions: Vec<String>,
    pub severity: Severity,
}

/// Severity level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_troubleshooting_engine_creation() {
        let engine = TroubleshootingEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_diagnose() {
        let engine = TroubleshootingEngine::new().unwrap();
        let response = engine.diagnose(&["high temperature".to_string()]);
        assert!(response.is_ok());
        assert!(response.unwrap().confidence > 0.5);
    }
}
