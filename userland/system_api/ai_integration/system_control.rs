// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS System Control - AI-powered system control

use serde::{Deserialize, Serialize};

/// System Control for AI-powered system management
pub struct SystemControl {
    capabilities: Vec<SystemCapability>,
}

impl SystemControl {
    /// Create a new System Control
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            capabilities: Self::get_default_capabilities(),
        })
    }

    /// Get available system control capabilities
    fn get_default_capabilities() -> Vec<SystemCapability> {
        vec![
            SystemCapability {
                name: "install_software".to_string(),
                description: "Install software packages".to_string(),
                requires_confirmation: true,
            },
            SystemCapability {
                name: "optimize_system".to_string(),
                description: "Optimize system performance".to_string(),
                requires_confirmation: false,
            },
            SystemCapability {
                name: "create_backup".to_string(),
                description: "Create system backup".to_string(),
                requires_confirmation: true,
            },
            SystemCapability {
                name: "manage_services".to_string(),
                description: "Manage system services".to_string(),
                requires_confirmation: true,
            },
        ]
    }

    /// Execute a system control command
    pub fn execute(&self, parameters: &[String]) -> Result<super::AIResponse, Box<dyn std::error::Error>> {
        if let Some(action) = parameters.first() {
            match action.as_str() {
                "install" => self.execute_install(&parameters[1..]),
                "optimize" => self.execute_optimize(),
                "backup" => self.execute_backup(),
                "cuda" => self.execute_cuda_install(),
                _ => self.execute_generic(action),
            }
        } else {
            Ok(super::AIResponse {
                message: "No action specified".to_string(),
                confidence: 0.0,
                action: None,
            })
        }
    }

    /// Execute software installation
    fn execute_install(&self, parameters: &[String]) -> Result<super::AIResponse, Box<dyn std::error::Error>> {
        if let Some(software) = parameters.first() {
            Ok(super::AIResponse {
                message: format!("Installing {}... This may take a few minutes.", software),
                confidence: 0.95,
                action: Some(format!("install_{}", software)),
            })
        } else {
            Ok(super::AIResponse {
                message: "Please specify what software to install".to_string(),
                confidence: 0.5,
                action: None,
            })
        }
    }

    /// Execute system optimization
    fn execute_optimize(&self) -> Result<super::AIResponse, Box<dyn std::error::Error>> {
        Ok(super::AIResponse {
            message: "I'm optimizing your system. This includes: clearing temporary files, adjusting power settings, and disabling unnecessary startup programs.".to_string(),
            confidence: 0.9,
            action: Some("optimize_system".to_string()),
        })
    }

    /// Execute backup creation
    fn execute_backup(&self) -> Result<super::AIResponse, Box<dyn std::error::Error>> {
        Ok(super::AIResponse {
            message: "Creating a system backup. This may take several minutes depending on your data size.".to_string(),
            confidence: 0.95,
            action: Some("create_backup".to_string()),
        })
    }

    /// Execute CUDA installation
    fn execute_cuda_install(&self) -> Result<super::AIResponse, Box<dyn std::error::Error>> {
        Ok(super::AIResponse {
            message: "I'll detect your GPU and install the appropriate CUDA version. This includes: GPU detection, CUDA toolkit installation, driver configuration, and environment setup.".to_string(),
            confidence: 0.92,
            action: Some("install_cuda".to_string()),
        })
    }

    /// Execute generic command
    fn execute_generic(&self, action: &str) -> Result<super::AIResponse, Box<dyn std::error::Error>> {
        Ok(super::AIResponse {
            message: format!("Executing: {}", action),
            confidence: 0.7,
            action: Some(action.to_string()),
        })
    }

    /// Get suggestions based on system state
    pub fn get_suggestions(&self, system_state: &super::SystemState) -> Vec<super::AISuggestion> {
        let mut suggestions = Vec::new();
        
        if system_state.cpu_usage > 80.0 {
            suggestions.push(super::AISuggestion {
                title: "High CPU Usage".to_string(),
                description: "Your CPU usage is high. Consider optimizing or closing resource-intensive applications.".to_string(),
                priority: super::SuggestionPriority::High,
                action: Some("optimize_cpu".to_string()),
            });
        }
        
        if system_state.memory_usage > 80.0 {
            suggestions.push(super::AISuggestion {
                title: "High Memory Usage".to_string(),
                description: "Your memory usage is high. Consider closing applications or increasing swap space.".to_string(),
                priority: super::SuggestionPriority::Medium,
                action: Some("optimize_memory".to_string()),
            });
        }
        
        suggestions
    }

    /// Get available capabilities
    pub fn get_capabilities(&self) -> Vec<SystemCapability> {
        self.capabilities.clone()
    }

    /// Check if a capability is available
    pub fn has_capability(&self, capability_name: &str) -> bool {
        self.capabilities.iter().any(|c| c.name == capability_name)
    }
}

/// System capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCapability {
    pub name: String,
    pub description: String,
    pub requires_confirmation: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_control_creation() {
        let control = SystemControl::new();
        assert!(control.is_ok());
    }

    #[test]
    fn test_execute_install() {
        let control = SystemControl::new().unwrap();
        let response = control.execute(&["install".to_string(), "firefox".to_string()]);
        assert!(response.is_ok());
        assert!(response.unwrap().confidence > 0.5);
    }
}
