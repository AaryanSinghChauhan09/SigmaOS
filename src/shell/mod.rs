// SigmaOS Shell Module
pub mod command;
pub mod repl;
pub mod multicall;
pub mod sigma_sh;
pub mod terminal_emulator;

pub use command::{CommandError, ShellSession, SimpleShellSession};
pub use repl::{ShellCommand, ShellRepl};
pub use terminal_emulator::{TerminalSession, UserDefinedFunction, AutoSuggestionEngine, AnsiColor};
