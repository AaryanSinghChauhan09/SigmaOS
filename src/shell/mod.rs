// SigmaOS Shell Module
pub mod command;
pub mod terminal_emulator;
pub mod alias_system;

// Temporarily disable repl due to dependency issues
// pub mod repl;

// pub use repl::{ShellCommand, ShellRepl};
pub use terminal_emulator::{TerminalSession, UserDefinedFunction, AutoSuggestionEngine, AnsiColor};
pub use alias_system::{AliasManager, SigmaAlias, AliasType};
