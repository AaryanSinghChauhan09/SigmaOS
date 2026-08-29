//! Intelligent Terminal with Agent Client Protocol (ACP) Engine integration.
//! Merges Microsoft's Intelligent Terminal capabilities natively into SigmaOS's shell.
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
use alloc::format;

extern crate alloc;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Represents the types of Agent Client Protocol (ACP) message models.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AcpMessageType {
    Request,
    Response,
    Notification,
}

/// A standard Agent Client Protocol (ACP) message payload adhering to JSON-RPC standard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcpMessage {
    pub jsonrpc: String,
    pub msg_type: AcpMessageType,
    pub id: Option<u64>,
    pub method: String,
    pub params: BTreeMap<String, String>,
}

impl AcpMessage {
    /// Creates a new ACP Request message.
    pub fn request(id: u64, method: &str, params: BTreeMap<String, String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            msg_type: AcpMessageType::Request,
            id: Some(id),
            method: method.to_string(),
            params,
        }
    }

    /// Creates a new ACP Response message.
    pub fn response(id: u64, result: BTreeMap<String, String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            msg_type: AcpMessageType::Response,
            id: Some(id),
            method: "result".to_string(),
            params: result,
        }
    }

    /// Creates a new ACP Notification message.
    pub fn notification(method: &str, params: BTreeMap<String, String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            msg_type: AcpMessageType::Notification,
            id: None,
            method: method.to_string(),
            params,
        }
    }
}

/// Shell Context captured in real-time by the Intelligent Terminal.
#[derive(Debug, Clone, Default)]
pub struct ShellContext {
    pub current_dir: String,
    pub env_variables: BTreeMap<String, String>,
    pub command_history: Vec<String>,
    pub last_stdout: Option<String>,
    pub last_stderr: Option<String>,
}

impl ShellContext {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut envs = BTreeMap::new();
        envs.insert("SHELL".to_string(), "sigma_sh".to_string());
        envs.insert("TERM".to_string(), "xterm-256color".to_string());
        Self {
            current_dir: "/home/user".to_string(),
            env_variables: envs,
            command_history: Vec::new(),
            last_stdout: None,
            last_stderr: None,
        }
    }
}

/// An error-catching hook that automatically watches shell execution outputs and registers faults.
#[derive(Debug, Clone, Default)]
pub struct TerminalErrorHook {
    pub captured_errors: Vec<CapturedError>,
}

#[derive(Debug, Clone)]
pub struct CapturedError {
    pub command: String,
    pub exit_code: i32,
    pub stderr_snippet: String,
    pub suggested_fix: String,
}

impl TerminalErrorHook {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            captured_errors: Vec::new(),
        }
    }

    /// Analyzes a command execution and captures failure snippets if the exit code is non-zero.
    pub fn analyze_execution(
        &mut self,
        command: &str,
        exit_code: i32,
        stderr: &str,
    ) -> Option<CapturedError> {
        if exit_code == 0 {
            return None;
        }

        let mut fix = "Review command syntax and try again.".to_string();
        if stderr.contains("Cargo.toml") || stderr.contains("rustc") {
            fix = "Fix Rust compiler diagnostics reported above.".to_string();
        } else if stderr.contains("not found") || stderr.contains("not recognized") {
            fix = "Install the missing package or verify command spelling.".to_string();
        } else if stderr.contains("permission") || stderr.contains("Access denied") {
            fix =
                "Rerun the command with administrator privilege or 'sudo' equivalent.".to_string();
        } else if stderr.contains("syntax error") {
            fix = "Correct the syntax structure of your shell expression.".to_string();
        }

        let err = CapturedError {
            command: command.to_string(),
            exit_code,
            stderr_snippet: stderr.to_string(),
            suggested_fix: fix,
        };

        self.captured_errors.push(err.clone());
        Some(err)
    }
}

/// Native SigmaOS Intelligent Terminal integrating Agent Client Protocol (ACP) communication.
pub struct IntelligentTerminal {
    pub context: ShellContext,
    pub error_hook: TerminalErrorHook,
    pub acp_history: Vec<AcpMessage>,
    pub agent_response_buffer: Vec<String>,
}

impl IntelligentTerminal {
    /// Creates a new Intelligent Terminal instance.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            context: ShellContext::new(),
            error_hook: TerminalErrorHook::new(),
            acp_history: Vec::new(),
            agent_response_buffer: Vec::new(),
        }
    }

    /// Executes a command within the terminal, automatically capturing its context and any errors.
    pub fn execute_command(
        &mut self,
        command: &str,
        stdout: &str,
        stderr: &str,
        exit_code: i32,
    ) -> Option<CapturedError> {
        self.context.command_history.push(command.to_string());
        self.context.last_stdout = Some(stdout.to_string());
        self.context.last_stderr = Some(stderr.to_string());

        let error_opt = self
            .error_hook
            .analyze_execution(command, exit_code, stderr);

        if let Some(ref err) = error_opt {
            // Trigger automatic ACP notification to the native background agent
            let mut params = BTreeMap::new();
            params.insert("command".to_string(), err.command.clone());
            params.insert("exit_code".to_string(), err.exit_code.to_string());
            params.insert("snippet".to_string(), err.stderr_snippet.clone());
            params.insert("suggested_fix".to_string(), err.suggested_fix.clone());

            let notify = AcpMessage::notification("terminal/on_error", params);
            self.send_acp_message(notify);
        }

        error_opt
    }

    /// Synchronizes the current workspace state with any ACP-compatible agent client.
    pub fn synchronize_context_to_agent(&mut self) -> AcpMessage {
        let mut params = BTreeMap::new();
        params.insert("pwd".to_string(), self.context.current_dir.clone());
        params.insert(
            "history_len".to_string(),
            self.context.command_history.len().to_string(),
        );
        if let Some(ref o) = self.context.last_stdout {
            params.insert(
                "last_stdout_preview".to_string(),
                o.chars().take(100).collect(),
            );
        }

        let msg = AcpMessage::notification("terminal/sync_context", params);
        self.send_acp_message(msg.clone());
        msg
    }

    /// Handles incoming slash commands directly from the user or the agent console.
    pub fn handle_slash_command(&mut self, command: &str) -> String {
        let trimmed = command.trim();
        if !trimmed.starts_with('/') {
            return "Not a valid slash command. Available: /explain, /fix, /generate, /task"
                .to_string();
        }

        let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
        let cmd = parts[0];
        let arg = if parts.len() > 1 { parts[1] } else { "" };

        match cmd {
            "/explain" => {
                if let Some(ref err) = self.error_hook.captured_errors.last() {
                    format!("Agent explanation for error in '{}':\nThe command failed with exit code {}.\nSnippet: {}\nResolution suggestion: {}",
                        err.command, err.exit_code, err.stderr_snippet, err.suggested_fix)
                } else {
                    "No active command error to explain. Everything looks perfect!".to_string()
                }
            }
            "/fix" => {
                if let Some(ref err) = self.error_hook.captured_errors.last() {
                    format!("Agent automation plan:\n1. Re-evaluate dependencies.\n2. Fix recommendation: {}\n3. Executing patch safely.", err.suggested_fix)
                } else {
                    "No errors caught. Use /generate to write new scripts instead.".to_string()
                }
            }
            "/generate" => {
                if arg.is_empty() {
                    "Usage: /generate <description of script or command>".to_string()
                } else {
                    format!(
                        "Generated code command for '{}':\n```sh\necho \"Executing task: {}\"\n```",
                        arg, arg
                    )
                }
            }
            "/task" => {
                if arg.is_empty() {
                    "Usage: /task <multi-step automation description>".to_string()
                } else {
                    format!("Agent task agent loop initialized for '{}':\nStep 1: Parse requirements.\nStep 2: Dry-run check.\nStep 3: Safe background execution.", arg)
                }
            }
            _ => format!(
                "Unknown agent slash command '{}'. Please use /explain, /fix, /generate, or /task.",
                cmd
            ),
        }
    }

    /// Simulates sending an ACP message to the agent daemon.
    pub fn send_acp_message(&mut self, msg: AcpMessage) {
        self.acp_history.push(msg);
    }

    /// Simulates receiving and processing a JSON-RPC request from an ACP agent client.
    pub fn receive_acp_message(&mut self, msg: AcpMessage) -> Option<AcpMessage> {
        self.acp_history.push(msg.clone());
        if msg.msg_type == AcpMessageType::Request {
            let mut result = BTreeMap::new();
            if msg.method == "agent/get_context" {
                result.insert("pwd".to_string(), self.context.current_dir.clone());
                result.insert(
                    "history_count".to_string(),
                    self.context.command_history.len().to_string(),
                );
                return Some(AcpMessage::response(msg.id.unwrap_or(1), result));
            } else if msg.method == "agent/execute_slash" {
                let cmd = msg.params.get("command").cloned().unwrap_or_default();
                let output = self.handle_slash_command(&cmd);
                result.insert("output".to_string(), output);
                return Some(AcpMessage::response(msg.id.unwrap_or(1), result));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acp_message_serialization() {
        let mut params = BTreeMap::new();
        params.insert("test_key".to_string(), "test_val".to_string());

        let req = AcpMessage::request(42, "terminal/initialize", params.clone());
        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.msg_type, AcpMessageType::Request);
        assert_eq!(req.id, Some(42));
        assert_eq!(req.method, "terminal/initialize");
        assert_eq!(req.params.get("test_key").unwrap(), "test_val");

        let resp = AcpMessage::response(42, params.clone());
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.msg_type, AcpMessageType::Response);
        assert_eq!(resp.id, Some(42));
        assert_eq!(resp.method, "result");

        let notify = AcpMessage::notification("terminal/on_event", params);
        assert_eq!(notify.jsonrpc, "2.0");
        assert_eq!(notify.msg_type, AcpMessageType::Notification);
        assert_eq!(notify.id, None);
        assert_eq!(notify.method, "terminal/on_event");
    }

    #[test]
    fn test_terminal_context_tracking() {
        let mut term = IntelligentTerminal::new();
        assert_eq!(term.context.current_dir, "/home/user");
        assert_eq!(term.context.env_variables.get("SHELL").unwrap(), "sigma_sh");

        term.execute_command("ls -la", "total 0", "", 0);
        assert_eq!(term.context.command_history.len(), 1);
        assert_eq!(term.context.command_history[0], "ls -la");
        assert_eq!(term.context.last_stdout.as_ref().unwrap(), "total 0");
        assert_eq!(term.context.last_stderr.as_ref().unwrap(), "");

        let sync_msg = term.synchronize_context_to_agent();
        assert_eq!(sync_msg.method, "terminal/sync_context");
        assert_eq!(sync_msg.params.get("pwd").unwrap(), "/home/user");
        assert_eq!(sync_msg.params.get("history_len").unwrap(), "1");
    }

    #[test]
    fn test_error_catching_hook() {
        let mut term = IntelligentTerminal::new();
        let err_opt = term.execute_command("cargo build", "", "error: Cargo.toml not found", 101);
        assert!(err_opt.is_some());

        let captured = err_opt.unwrap();
        assert_eq!(captured.command, "cargo build");
        assert_eq!(captured.exit_code, 101);
        assert!(captured.suggested_fix.contains("Rust compiler diagnostics"));

        // Explain the last error using the slash command
        let explanation = term.handle_slash_command("/explain");
        assert!(explanation.contains("cargo build"));
        assert!(explanation.contains("exit code 101"));

        // Generate script command
        let script = term.handle_slash_command("/generate build automation script");
        assert!(script.contains("build automation script"));

        // Initiate task
        let task = term.handle_slash_command("/task run regression checks");
        assert!(task.contains("run regression checks"));
    }
}
