// SigmaOS Shell Module
pub mod alias_system;
pub mod command;
pub mod repl;
pub mod terminal_emulator;

// pub use repl::{ShellCommand, ShellRepl};
pub use alias_system::{AliasManager, AliasType, SigmaAlias};
pub use command::{
    CommandError, CommandParser, CommandRegistry, ShellCommand, ShellSession,
    SimpleCommandRegistry, SimpleShellSession,
};
pub use terminal_emulator::{
    AnsiColor, AutoSuggestionEngine, TerminalSession, UserDefinedFunction,
};
