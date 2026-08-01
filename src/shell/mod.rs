// SigmaOS Shell Module
pub mod command;
pub mod repl;
pub mod intelligent_terminal;
#[cfg(any())]
pub mod command;

pub use command::{CommandError, ShellSession, SimpleShellSession};
pub use repl::{ShellCommand, ShellRepl};
pub use intelligent_terminal::{AcpMessage, AcpMessageType, ShellContext, TerminalErrorHook, IntelligentTerminal};
