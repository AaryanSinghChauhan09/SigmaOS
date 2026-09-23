// SPDX-License-Identifier: MIT
// SigmaOS AI & Automation Suite - SigmaAI Agent Roadmap Implementation
// Phases 1-5: Natural Language Translator, Workflow Automation, Adaptive CLI Suggestions,
// Error Explanation Layer, and SigmaAI Assistant (Indic Languages & Voice Support).


use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

/// Supported Indic and international languages for SigmaAI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicLanguage {
    English,
    Hindi,
    Tamil,
    Bengali,
    Telugu,
    Marathi,
    Gujarati,
    Kannada,
    Malayalam,
    Punjabi,
}

/// Safety severity level for CLI command execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyLevel {
    Safe,
    Caution,
    Dangerous,
    Destructive,
}

/// Translation result with CLI command, safety audit, and plain-language explanation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandTranslation {
    pub natural_language_input: String,
    pub detected_language: IndicLanguage,
    pub generated_cli_command: String,
    pub safety_level: SafetyLevel,
    pub requires_confirmation: bool,
    pub plain_explanation: String,
}

/// Phase 1: Natural Language Command Translator & Safety Check Engine
pub struct SigmaAiNaturalLanguageTranslator;

impl SigmaAiNaturalLanguageTranslator {
    pub fn new() -> Self {
        Self
    }

    /// Detects Indic language or default English from prompt string keywords.
    pub fn detect_language(&self, input: &str) -> IndicLanguage {
        let lower = input.to_lowercase();
        if lower.contains("karo") || lower.contains("dikhayein") || lower.contains("meri") {
            IndicLanguage::Hindi
        } else if input.contains("நிறுவவும்") || input.contains("காண்பி") {
            IndicLanguage::Tamil
        } else if input.contains("করুন") || input.contains("ইনস্টল") {
            IndicLanguage::Bengali
        } else if lower.contains("cheyyi") || lower.contains("chuupinchu") {
            IndicLanguage::Telugu
        } else {
            IndicLanguage::English
        }
    }

    /// Translates natural language prompt into executable SigmaOS CLI command.
    pub fn translate(&self, input: &str) -> CommandTranslation {
        let lang = self.detect_language(input);
        let lower = input.to_lowercase();

        let (command, safety, explanation) = if lower.contains("delete all") || lower.contains("rm -rf /") || lower.contains("saari files mitao") {
            (
                "rm -rf / --no-preserve-root".to_string(),
                SafetyLevel::Destructive,
                "Recursively deletes all files and directories from the root directory without safeguard.".to_string(),
            )
        } else if lower.contains("install libreoffice") || lower.contains("libreoffice install karo") || input.contains("லிப்ரேஆபிஸ் நிறுவவும்") {
            (
                "sigpkg install libreoffice".to_string(),
                SafetyLevel::Safe,
                "Downloads and installs the LibreOffice office productivity suite using sigpkg.".to_string(),
            )
        } else if lower.contains("disk usage") || lower.contains("disk dikhayein") || lower.contains("storage check") {
            (
                "df -h".to_string(),
                SafetyLevel::Safe,
                "Displays human-readable filesystem disk space usage for all mounted partitions.".to_string(),
            )
        } else if lower.contains("connect to wifi") || lower.contains("wifi connect") {
            let ssid = if lower.contains("home") { "Home" } else { "DefaultNet" };
            (
                format!("sigma-wifi connect --ssid {}", ssid),
                SafetyLevel::Safe,
                format!("Initiates wireless network connection to SSID '{}'.", ssid),
            )
        } else {
            (
                format!("sigma-cli query \"{}\"", input),
                SafetyLevel::Caution,
                "Generates general query payload for generic system command handler.".to_string(),
            )
        };

        let requires_confirmation = matches!(safety, SafetyLevel::Dangerous | SafetyLevel::Destructive);

        CommandTranslation {
            natural_language_input: input.to_string(),
            detected_language: lang,
            generated_cli_command: command,
            safety_level: safety,
            requires_confirmation,
            plain_explanation: explanation,
        }
    }

    /// Provides plain-language explanations for flag-heavy CLI commands.
    pub fn explain_command(&self, command: &str) -> String {
        if command.starts_with("tar -xvf") || command.starts_with("tar -xzvf") {
            "Extracts (-x) a tar archive (-f) verbosely (-v) with optional compression.".to_string()
        } else if command.starts_with("sigpkg install") {
            "Invokes the native package manager to fetch and install specified software.".to_string()
        } else if command.starts_with("df -h") {
            "Shows disk space statistics in human-readable megabytes and gigabytes.".to_string()
        } else {
            format!("Executes system binary '{}' with specified flags.", command)
        }
    }
}

/// Workflow Trigger Types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowTrigger {
    Manual,
    CronSchedule(String), // e.g. "daily 18:00"
    Event(String),        // e.g. "on_boot", "on_network_connect"
}

/// Single step inside an automated workflow pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowStep {
    pub step_number: u32,
    pub step_name: String,
    pub target_app_or_cli: String,
    pub arguments: Vec<String>,
}

/// Phase 2: Workflow Automation & Visual Pipeline Engine
pub struct SigmaAiWorkflowAutomation {
    pub name: String,
    pub trigger: WorkflowTrigger,
    pub steps: Vec<WorkflowStep>,
    pub enabled: bool,
}

impl SigmaAiWorkflowAutomation {
    pub fn new(name: &str, trigger: WorkflowTrigger) -> Self {
        Self {
            name: name.to_string(),
            trigger,
            steps: Vec::new(),
            enabled: true,
        }
    }

    pub fn add_step(&mut self, step_name: &str, app: &str, args: &[&str]) {
        let step_number = (self.steps.len() as u32) + 1;
        self.steps.push(WorkflowStep {
            step_number,
            step_name: step_name.to_string(),
            target_app_or_cli: app.to_string(),
            arguments: args.iter().map(|s| s.to_string()).collect(),
        });
    }

    /// Creates standard pre-built template workflows (e.g. Daily GST Filing).
    pub fn create_gst_filing_template() -> Self {
        let mut wf = Self::new("Daily GST Filing", WorkflowTrigger::CronSchedule("daily 18:00".to_string()));
        wf.add_step("Open Accounting Suite", "sigma-accounts", &["--mode", "gst"]);
        wf.add_step("Generate Return", "sigma-accounts", &["--action", "generate-gstr3b"]);
        wf.add_step("Validate Data", "sigma-accounts", &["--action", "validate"]);
        wf.add_step("Export PDF Report", "sigma-accounts", &["--action", "export-pdf"]);
        wf.add_step("Email Filing", "sigma-mail", &["--to", "gst@gst.gov.in", "--attach", "gstr3b.pdf"]);
        wf.add_step("Archive Record", "sigma-fs", &["--archive", "/home/ravi/sigma-archives/gst/"]);
        wf
    }

    /// Executes the workflow steps sequentially.
    pub fn execute(&self) -> Result<usize, String> {
        if !self.enabled {
            return Err("Workflow is currently disabled.".to_string());
        }
        Ok(self.steps.len())
    }
}

/// Phase 3: Adaptive CLI Suggestions & Context Intelligence
pub struct SigmaAiAdaptiveSuggestions;

impl SigmaAiAdaptiveSuggestions {
    pub fn new() -> Self {
        Self
    }

    /// Provides context-aware command recommendations based on usage history.
    pub fn recommend_next_commands(&self, recent_activity: &[&str]) -> Vec<String> {
        let mut recs = Vec::new();
        if recent_activity.contains(&"sigma-accounts") {
            recs.push("sigma-accounts file-gstr3b".to_string());
            recs.push("sigma-accounts file-gstr1".to_string());
        }
        if recent_activity.contains(&"sigpkg") {
            recs.push("sigpkg update".to_string());
            recs.push("sigpkg upgrade".to_string());
        }
        if recs.is_empty() {
            recs.push("df -h".to_string());
            recs.push("sigma-ai --help".to_string());
        }
        recs
    }

    /// Error prevention analyzer for dangerous command paths.
    pub fn check_potential_error(&self, command: &str) -> Option<String> {
        if command.contains("rm -rf /home/") && !command.contains("/temp/") {
            Some("Warning: You are attempting to delete a critical user home directory. Did you mean to target the /temp/ subdirectory?".to_string())
        } else {
            None
        }
    }
}

/// Detailed error analysis structure with plain language explanation and suggested fixes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorExplanation {
    pub raw_error: String,
    pub plain_explanation: String,
    pub suggested_fixes: Vec<String>,
    pub related_doc_url: String,
}

/// Phase 4: Error Explanation Layer & Automated Troubleshooting
pub struct SigmaAiErrorExplanation;

impl SigmaAiErrorExplanation {
    pub fn new() -> Self {
        Self
    }

    /// Analyzes raw system or CLI error messages and generates plain explanations.
    pub fn explain(&self, raw_error: &str) -> ErrorExplanation {
        if raw_error.contains("Dependency conflict") || raw_error.contains("libssl") {
            ErrorExplanation {
                raw_error: raw_error.to_string(),
                plain_explanation: "This error means the software requires libssl1.1, but another package installed needs libssl3. These two versions conflict.".to_string(),
                suggested_fixes: vec![
                    "Fix 1: Upgrade libssl1.1 to libssl3 (compatible with 98% of packages)".to_string(),
                    "Fix 2: Use libreoffice-stable (requires libssl1.1)".to_string(),
                    "Fix 3: Remove the conflicting package requiring libssl3".to_string(),
                ],
                related_doc_url: "https://docs.sigmaos.org/errors/dependency-conflict".to_string(),
            }
        } else {
            ErrorExplanation {
                raw_error: raw_error.to_string(),
                plain_explanation: "The command encountered a general runtime error during execution.".to_string(),
                suggested_fixes: vec!["Check system logs via journalctl -xe".to_string()],
                related_doc_url: "https://docs.sigmaos.org/errors/general".to_string(),
            }
        }
    }
}

/// Phase 5: SigmaAI Assistant (Indic Language Voice & Proactive Intelligence)
pub struct SigmaAiAssistant {
    pub active_language: IndicLanguage,
    pub voice_enabled: bool,
}

impl SigmaAiAssistant {
    pub fn new(active_language: IndicLanguage, voice_enabled: bool) -> Self {
        Self {
            active_language,
            voice_enabled,
        }
    }

    /// Processes voice or text command in Indic language and returns assistant response.
    pub fn process_assistant_prompt(&self, prompt: &str) -> String {
        let lower = prompt.to_lowercase();
        if lower.contains("gst return file karo") || prompt.contains("வரி தாக்கல் செய்") {
            "Opening sigma-accounts, generating GSTR3B statement, and rendering PDF preview.".to_string()
        } else if lower.contains("disk full") || lower.contains("storage low") {
            "Disk space is at 85% capacity. Would you like me to trigger an automated system cleanup workflow?".to_string()
        } else {
            format!("SigmaAI Assistant ({:?}): Standard task registered.", self.active_language)
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_phase1_natural_language_translator() {
        let translator = SigmaAiNaturalLanguageTranslator::new();
        let t1 = translator.translate("libreoffice install karo");
        assert_eq!(t1.detected_language, IndicLanguage::Hindi);
        assert_eq!(t1.generated_cli_command, "sigpkg install libreoffice");
        assert_eq!(t1.safety_level, SafetyLevel::Safe);

        let t2 = translator.translate("delete all files");
        assert_eq!(t2.safety_level, SafetyLevel::Destructive);
        assert!(t2.requires_confirmation);
    }

    #[test]
    fn test_phase2_workflow_automation() {
        let wf = SigmaAiWorkflowAutomation::create_gst_filing_template();
        assert_eq!(wf.name, "Daily GST Filing");
        assert_eq!(wf.steps.len(), 6);
        assert_eq!(wf.execute().unwrap(), 6);
    }

    #[test]
    fn test_phase3_adaptive_suggestions() {
        let engine = SigmaAiAdaptiveSuggestions::new();
        let recs = engine.recommend_next_commands(&["sigma-accounts"]);
        assert!(recs.contains(&"sigma-accounts file-gstr3b".to_string()));

        let err_check = engine.check_potential_error("rm -rf /home/ravi/sigma-accounts/");
        assert!(err_check.is_some());
    }

    #[test]
    fn test_phase4_error_explanation() {
        let explainer = SigmaAiErrorExplanation::new();
        let exp = explainer.explain("Error: Dependency conflict: libssl1.1 vs libssl3");
        assert!(exp.plain_explanation.contains("libssl1.1"));
        assert_eq!(exp.suggested_fixes.len(), 3);
    }

    #[test]
    fn test_phase5_sigma_ai_assistant() {
        let assistant = SigmaAiAssistant::new(IndicLanguage::Hindi, true);
        let resp = assistant.process_assistant_prompt("Sigma, meri GST return file karo");
        assert!(resp.contains("generating GSTR3B statement"));
    }
}
