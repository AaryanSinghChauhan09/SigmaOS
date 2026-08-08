// SigmaOS Shell Module
pub mod command;
pub mod intelligent_terminal;
pub mod repl;

pub use command::{CommandError, ShellSession, SimpleShellSession};
pub use intelligent_terminal::{
    AcpMessage, AcpMessageType, IntelligentTerminal, ShellContext, TerminalErrorHook,
};
pub use repl::{ShellCommand, ShellRepl};
