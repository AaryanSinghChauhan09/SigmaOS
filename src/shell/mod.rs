// SigmaOS Shell Module
pub mod command;
pub mod repl;
pub mod terminal_emulator;
pub mod alias_system;

// pub use repl::{ShellCommand, ShellRepl};
pub use command::{CommandError, CommandParser, CommandRegistry, ShellCommand, ShellSession, SimpleCommandRegistry, SimpleShellSession};
pub use terminal_emulator::{TerminalSession, UserDefinedFunction, AutoSuggestionEngine, AnsiColor};
pub use alias_system::{AliasManager, SigmaAlias, AliasType};
