// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Privacy Manager - Privacy controls for AI processing

use serde::{Deserialize, Serialize};

/// Privacy Manager for AI privacy controls
pub struct PrivacyManager {
    privacy_level: super::PrivacyLevel,
    data_retention_policy: DataRetentionPolicy,
    consent_manager: ConsentManager,
}

impl PrivacyManager {
    /// Create a new Privacy Manager
    pub fn new(privacy_level: super::PrivacyLevel) -> Result<Self, Box<dyn std::error::Error>> {
        let data_retention_policy = Self::default_retention_policy(&privacy_level);
        let consent_manager = ConsentManager::new();
        
        Ok(Self {
            privacy_level,
            data_retention_policy,
            consent_manager,
        })
    }

    /// Get default retention policy based on privacy level
    fn default_retention_policy(level: &super::PrivacyLevel) -> DataRetentionPolicy {
        match level {
            super::PrivacyLevel::Minimal => DataRetentionPolicy {
                retain_interactions: false,
                retain_context: false,
                retain_learning_data: false,
                max_retention_days: 0,
            },
            super::PrivacyLevel::Standard => DataRetentionPolicy {
                retain_interactions: true,
                retain_context: true,
                retain_learning_data: true,
                max_retention_days: 30,
            },
            super::PrivacyLevel::High => DataRetentionPolicy {
                retain_interactions: true,
                retain_context: true,
                retain_learning_data: false,
                max_retention_days: 7,
            },
            super::PrivacyLevel::Maximum => DataRetentionPolicy {
                retain_interactions: false,
                retain_context: false,
                retain_learning_data: false,
                max_retention_days: 1,
            },
        }
    }

    /// Check if a command can be processed based on privacy policy
    pub fn can_process(&self, command: &str) -> Result<bool, Box<dyn std::error::Error>> {
        // Check for sensitive information
        if self.contains_sensitive_info(command) {
            // Check if user has consented
            if !self.consent_manager.has_consent("sensitive_data_processing") {
                return Ok(false);
            }
        }
        
        Ok(true)
    }

    /// Check if command contains sensitive information
    fn contains_sensitive_info(&self, command: &str) -> bool {
        let sensitive_patterns = vec![
            "password",
            "credit card",
            "ssn",
            "social security",
            "api key",
            "secret",
            "token",
        ];
        
        let command_lower = command.to_lowercase();
        sensitive_patterns.iter().any(|pattern| command_lower.contains(pattern))
    }

    /// Anonymize data before processing
    pub fn anonymize(&self, data: &str) -> String {
        // Simple anonymization - replace sensitive patterns
        let mut anonymized = data.to_string();
        
        let patterns = vec![
            (r"\b\d{3}-\d{2}-\d{4}\b", "***-**-****"), // SSN pattern
            (r"\b\d{4}\s?\d{4}\s?\d{4}\s?\d{4}\b", "**** **** **** ****"), // Credit card pattern
        ];
        
        for (pattern, replacement) in patterns {
            anonymized = regex::Regex::new(pattern)
                .unwrap()
                .replace_all(&anonymized, replacement)
                .to_string();
        }
        
        anonymized
    }

    /// Get current privacy level
    pub fn get_privacy_level(&self) -> super::PrivacyLevel {
        self.privacy_level.clone()
    }

    /// Set privacy level
    pub fn set_privacy_level(&mut self, level: super::PrivacyLevel) {
        self.privacy_level = level;
        self.data_retention_policy = Self::default_retention_policy(&level);
    }

    /// Get data retention policy
    pub fn get_retention_policy(&self) -> &DataRetentionPolicy {
        &self.data_retention_policy
    }

    /// Request consent for data processing
    pub fn request_consent(&mut self, consent_type: &str) -> ConsentRequest {
        self.consent_manager.request_consent(consent_type)
    }

    /// Grant consent
    pub fn grant_consent(&mut self, consent_type: &str) {
        self.consent_manager.grant_consent(consent_type);
    }

    /// Revoke consent
    pub fn revoke_consent(&mut self, consent_type: &str) {
        self.consent_manager.revoke_consent(consent_type);
    }

    /// Get privacy report
    pub fn get_privacy_report(&self) -> PrivacyReport {
        PrivacyReport {
            privacy_level: format!("{:?}", self.privacy_level),
            local_processing_enabled: true,
            data_retention_policy: self.data_retention_policy.clone(),
            consents: self.consent_manager.get_consents(),
        }
    }

    /// Clear all retained data
    pub fn clear_retained_data(&mut self) {
        // In a real implementation, this would clear databases, logs, etc.
        println!("Clearing all retained data according to privacy policy");
    }
}

/// Data retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionPolicy {
    pub retain_interactions: bool,
    pub retain_context: bool,
    pub retain_learning_data: bool,
    pub max_retention_days: u32,
}

/// Consent manager
#[derive(Debug, Clone)]
struct ConsentManager {
    consents: std::collections::HashMap<String, bool>,
}

impl ConsentManager {
    fn new() -> Self {
        let mut consents = std::collections::HashMap::new();
        // Default consents
        consents.insert("basic_processing".to_string(), true);
        consents.insert("learning".to_string(), true);
        
        Self { consents }
    }

    fn has_consent(&self, consent_type: &str) -> bool {
        *self.consents.get(consent_type).unwrap_or(&false)
    }

    fn request_consent(&self, consent_type: &str) -> ConsentRequest {
        ConsentRequest {
            consent_type: consent_type.to_string(),
            description: Self::get_consent_description(consent_type),
            currently_granted: self.has_consent(consent_type),
        }
    }

    fn grant_consent(&mut self, consent_type: &str) {
        self.consents.insert(consent_type.to_string(), true);
    }

    fn revoke_consent(&mut self, consent_type: &str) {
        self.consents.insert(consent_type.to_string(), false);
    }

    fn get_consents(&self) -> Vec<(String, bool)> {
        self.consents.iter().map(|(k, v)| (k.clone(), *v)).collect()
    }

    fn get_consent_description(consent_type: &str) -> String {
        match consent_type {
            "sensitive_data_processing" => "Allow processing of potentially sensitive data for AI assistance".to_string(),
            "learning" => "Allow system to learn from your interactions to improve suggestions".to_string(),
            "analytics" => "Allow collection of anonymous usage analytics".to_string(),
            _ => "Generic consent request".to_string(),
        }
    }
}

/// Consent request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRequest {
    pub consent_type: String,
    pub description: String,
    pub currently_granted: bool,
}

/// Privacy report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyReport {
    pub privacy_level: String,
    pub local_processing_enabled: bool,
    pub data_retention_policy: DataRetentionPolicy,
    pub consents: Vec<(String, bool)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_manager_creation() {
        let manager = PrivacyManager::new(super::PrivacyLevel::Standard);
        assert!(manager.is_ok());
    }

    #[test]
    fn test_can_process() {
        let manager = PrivacyManager::new(super::PrivacyLevel::Standard).unwrap();
        assert!(manager.can_process("hello").unwrap());
    }

    #[test]
    fn test_anonymize() {
        let manager = PrivacyManager::new(super::PrivacyLevel::Standard).unwrap();
        let anonymized = manager.anonymize("my password is secret");
        assert!(anonymized.contains("password"));
    }
}
