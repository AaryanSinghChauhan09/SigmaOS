// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - CodexCLI natural language → shell command translator
// Principles: SAFETY, DEFENSIVE PROGRAMMING, USER FREEDOM AND CONTROL
use alloc::{string::String, vec::Vec};
#[derive(Debug, Clone)]
pub struct ShellCommand {
    pub program: String,
    pub args: Vec<String>,
    pub explanation: String,
}
impl ShellCommand {
    pub fn new(prog: &str, args: &[&str], explanation: &str) -> Self {
        Self {
            program: prog.into(),
            args: args.iter().map(|s| s.to_string()).collect(),
            explanation: explanation.into(),
        }
    }
    pub fn to_string(&self) -> String {
        if self.args.is_empty() {
            self.program.clone()
        } else {
            alloc::format!("{} {}", self.program, self.args.join(" "))
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum SafetyLevel {
    Safe,
    ReviewRequired,
    Dangerous { reason: String },
}
#[derive(Debug, Clone)]
pub enum SandboxMode {
    None,
    ReadOnly,
    NetworkIsolated,
    FullIsolation,
}
#[derive(Debug, Clone)]
pub struct CodexRequest {
    pub natural_language: String,
    pub context: String,
    pub sandbox: SandboxMode,
}
#[derive(Debug, Clone)]
pub struct CodexTranslation {
    pub command: ShellCommand,
    pub safety: SafetyLevel,
    pub alternatives: Vec<ShellCommand>,
}
pub struct CodexCli {
    pub history: Vec<CodexRequest>,
}
impl CodexCli {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
    pub fn classify_safety(&self, cmd: &str) -> SafetyLevel {
        let danger_patterns = [
            "rm -rf",
            "dd if=",
            "mkfs",
            "> /dev/",
            "chmod 777 /",
            ":(){ :|:& };:",
        ];
        for p in &danger_patterns {
            if cmd.contains(p) {
                return SafetyLevel::Dangerous {
                    reason: alloc::format!("pattern: {}", p),
                };
            }
        }
        let review_patterns = [
            "sudo",
            "chmod",
            "chown",
            "iptables",
            "systemctl",
            "kill",
            "pkill",
        ];
        for p in &review_patterns {
            if cmd.contains(p) {
                return SafetyLevel::ReviewRequired;
            }
        }
        SafetyLevel::Safe
    }
    pub fn translate(&mut self, req: &CodexRequest) -> CodexTranslation {
        self.history.push(req.clone());
        let nl = req.natural_language.to_ascii_lowercase();
        let (cmd, alts) = if nl.contains("list") && nl.contains("file") {
            (
                ShellCommand::new("ls", &["-la"], "List files with details"),
                alloc::vec![ShellCommand::new(
                    "find",
                    &["."],
                    "Find all files recursively"
                )],
            )
        } else if nl.contains("disk") && (nl.contains("space") || nl.contains("usage")) {
            (
                ShellCommand::new("df", &["-h"], "Show disk usage"),
                alloc::vec![ShellCommand::new(
                    "du",
                    &["-sh", "*"],
                    "Show directory sizes"
                )],
            )
        } else if nl.contains("process") || nl.contains("running") {
            (
                ShellCommand::new("ps", &["aux"], "Show all processes"),
                alloc::vec![ShellCommand::new("top", &[], "Interactive process viewer")],
            )
        } else if nl.contains("network") || nl.contains("port") {
            (
                ShellCommand::new("ss", &["-tulpn"], "Show listening ports"),
                alloc::vec![ShellCommand::new(
                    "netstat",
                    &["-tulpn"],
                    "Network statistics"
                )],
            )
        } else {
            (
                ShellCommand::new("echo", &["done"], "Default command"),
                Vec::new(),
            )
        };
        let safety = self.classify_safety(&cmd.to_string());
        CodexTranslation {
            command: cmd,
            safety,
            alternatives: alts,
        }
    }
    pub fn execute_sandboxed(
        &self,
        cmd: &ShellCommand,
        _mode: &SandboxMode,
    ) -> Result<Vec<u8>, &'static str> {
        Ok(alloc::format!(
            "$ {}
[sandboxed output]
",
            cmd.to_string()
        )
        .into_bytes())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list_files() {
        let mut cli = CodexCli::new();
        let req = CodexRequest {
            natural_language: String::from("list all files"),
            context: String::new(),
            sandbox: SandboxMode::ReadOnly,
        };
        let t = cli.translate(&req);
        assert_eq!(t.command.program, "ls");
        assert_eq!(t.safety, SafetyLevel::Safe);
    }
    #[test]
    fn test_dangerous_classification() {
        let cli = CodexCli::new();
        assert!(matches!(
            cli.classify_safety("rm -rf /home"),
            SafetyLevel::Dangerous { .. }
        ));
    }
    #[test]
    fn test_review_required() {
        let cli = CodexCli::new();
        assert_eq!(
            cli.classify_safety("sudo apt update"),
            SafetyLevel::ReviewRequired
        );
    }
    #[test]
    fn test_safe() {
        let cli = CodexCli::new();
        assert_eq!(cli.classify_safety("ls -la"), SafetyLevel::Safe);
    }
}
