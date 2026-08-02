// SigmaOS Shell Module
pub mod command;
pub mod kimi_code_agent;
pub mod repl;

pub use command::{CommandError, ShellSession, SimpleShellSession};
pub use kimi_code_agent::{
    AcpMessage, AgentState, KimiCodeAgent, LifecycleHook, McpTool, SubagentTask, SubagentType,
};
pub use repl::{ShellCommand, ShellRepl};
