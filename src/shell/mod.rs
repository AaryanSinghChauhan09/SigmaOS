// SigmaOS Shell Module
pub mod command;
pub mod repl;
#[cfg(any())]
pub mod command;

pub use command::{CommandError, ShellSession, SimpleShellSession};
pub use repl::{ShellCommand, ShellRepl};
pub use terminal_emulator::{TerminalSession, UserDefinedFunction, AutoSuggestionEngine, AnsiColor};
