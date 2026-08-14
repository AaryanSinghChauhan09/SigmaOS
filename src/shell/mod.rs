// SigmaOS Shell Module
pub mod command;
pub mod command;
pub mod repl;

pub use command::{CommandError, ShellSession, SimpleShellSession};
pub use repl::{ShellCommand, ShellRepl};
pub use terminal_emulator::{
    AnsiColor, AutoSuggestionEngine, TerminalSession, UserDefinedFunction,
};
