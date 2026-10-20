//! SigmaOS AI Agent Integration
//!
//! Inspired by: Omarchy's configurable default coding agent with instant launch shortcuts
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! SEPARATION_OF_CONCERNS, INVERSION_OF_CONTROL, INFORMATION_HIDING, INTERFACE_SEGREGATION,
//! PLUG_IN_DRIVER_MODEL, POLYMORPHISM, COMPOSITION_OVER_INHERITANCE, USER_FREEDOM_AND_CONTROL,
//! CONFIGURABILITY, COMMUNITY_ORIENTED_DEVELOPMENT.

use core::fmt;

/// AI agent provider
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AIProvider {
    ClaudeCode,
    Codex,
    Fireworks,
    LocalLLM,
    Custom(String),
}

/// AI agent configuration
#[derive(Debug, Clone)]
pub struct AIConfig {
    pub provider: AIProvider,
    pub model: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub instant_launch_shortcut: String,
    pub crash_diagnosis: bool,
    pub max_tokens: u32,
    pub temperature: f32,
    pub enabled: bool,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            provider: AIProvider::ClaudeCode,
            model: "claude-3-sonnet".to_string(),
            api_key: None,
            base_url: None,
            instant_launch_shortcut: "SUPER+ALT+A".to_string(),
            crash_diagnosis: true,
            max_tokens: 4096,
            temperature: 0.7,
            enabled: true,
        }
    }
}

/// AI agent manager
pub struct AIManager {
    pub config: AIConfig,
    pub is_initialized: bool,
    pub last_diagnosis: Option<String>,
}

/// Crash diagnosis report
#[derive(Debug, Clone)]
pub struct CrashReport {
    pub timestamp: u64,
    pub error_message: String,
    pub stack_trace: Option<String>,
    pub system_info: String,
    pub suggested_fix: Option<String>,
}

impl AIManager {
    pub fn new() -> Self {
        Self {
            config: AIConfig::default(),
            is_initialized: false,
            last_diagnosis: None,
        }
    }

    /// Configure AI agent
    pub fn configure(&mut self, config: AIConfig) {
        self.config = config;
        self.is_initialized = self.config.enabled && self.config.api_key.is_some();
    }

    /// Set instant launch shortcut
    pub fn set_instant_launch(&mut self, shortcut: String) {
        self.config.instant_launch_shortcut = shortcut;
    }

    /// Enable/disable crash diagnosis
    pub fn set_crash_diagnosis(&mut self, enabled: bool) {
        self.config.crash_diagnosis = enabled;
    }

    /// Run crash diagnosis
    pub fn diagnose_crash(&mut self, error_message: String, stack_trace: Option<String>) -> CrashReport {
        let timestamp = now_secs();
        let system_info = get_system_info();

        let suggested_fix = if self.config.crash_diagnosis {
            Some(format!(
                "AI diagnosis for '{}': Check logs at /var/log/sigmaos/, verify dependencies, consider rolling back recent changes.",
                error_message
            ))
        } else {
            None
        };

        let report = CrashReport {
            timestamp,
            error_message,
            stack_trace,
            system_info,
            suggested_fix,
        };

        self.last_diagnosis = Some(format!("Crash diagnosed at {}s", timestamp));
        report
    }

    /// Get configuration status
    pub fn status(&self) -> AIStatus {
        AIStatus {
            provider: self.config.provider.clone(),
            model: self.config.model.clone(),
            enabled: self.config.enabled,
            initialized: self.is_initialized,
            instant_launch: self.config.instant_launch_shortcut.clone(),
            crash_diagnosis: self.config.crash_diagnosis,
            last_diagnosis: self.last_diagnosis.clone(),
        }
    }

    /// Check if AI is available
    pub fn is_available(&self) -> bool {
        self.is_initialized && self.config.enabled
    }
}

/// AI status
#[derive(Debug, Clone)]
pub struct AIStatus {
    pub provider: AIProvider,
    pub model: String,
    pub enabled: bool,
    pub initialized: bool,
    pub instant_launch: String,
    pub crash_diagnosis: bool,
    pub last_diagnosis: Option<String>,
}

impl fmt::Display for AIProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AIProvider::ClaudeCode => write!(f, "Claude Code"),
            AIProvider::Codex => write!(f, "OpenAI Codex"),
            AIProvider::Fireworks => write!(f, "Fireworks AI"),
            AIProvider::LocalLLM => write!(f, "Local LLM"),
            AIProvider::Custom(name) => write!(f, "Custom ({})", name),
        }
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn get_system_info() -> String {
    format!("SigmaOS v{}, Arch: x86_64", env!("CARGO_PKG_VERSION"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_default_config() {
        let config = AIConfig::default();
        assert!(config.enabled);
        assert_eq!(config.instant_launch_shortcut, "SUPER+ALT+A");
        assert!(config.crash_diagnosis);
    }

    #[test]
    fn test_ai_configure() {
        let mut mgr = AIManager::new();
        let config = AIConfig {
            provider: AIProvider::Codex,
            model: "gpt-4".to_string(),
            api_key: Some("sk-...".to_string()),
            ..Default::default()
        };
        mgr.configure(config);
        assert!(mgr.is_initialized);
    }

    #[test]
    fn test_ai_crash_diagnosis() {
        let mut mgr = AIManager::new();
        mgr.configure(AIConfig::default());
        let report = mgr.diagnose_crash("panic: null pointer".to_string(), Some("at main.rs:42".to_string()));
        assert_eq!(report.error_message, "panic: null pointer");
        assert!(report.suggested_fix.is_some());
        assert!(mgr.last_diagnosis.is_some());
    }

    #[test]
    fn test_ai_not_initialized_without_key() {
        let mut mgr = AIManager::new();
        mgr.configure(AIConfig {
            enabled: true,
            api_key: None,
            ..Default::default()
        });
        assert!(!mgr.is_initialized);
        assert!(!mgr.is_available());
    }

    #[test]
    fn test_ai_status() {
        let mut mgr = AIManager::new();
        mgr.configure(AIConfig::default());
        let status = mgr.status();
        assert_eq!(status.provider, AIProvider::ClaudeCode);
        assert!(status.enabled);
    }
}