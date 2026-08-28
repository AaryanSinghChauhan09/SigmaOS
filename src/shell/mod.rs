// SigmaOS Shell Module
pub mod alias_system;
pub mod command;
pub mod intelligent_terminal;
pub mod kimi_code_agent;
pub mod multicall;
pub mod repl;
pub mod terminal_emulator;
pub mod alias_system;
pub mod sovereign_shell_parity;

// pub use repl::{ShellCommand, ShellRepl};
pub use command::{CommandError, CommandParser, CommandRegistry, ShellCommand, ShellSession, SimpleCommandRegistry, SimpleShellSession};
pub use terminal_emulator::{TerminalSession, UserDefinedFunction, AutoSuggestionEngine, AnsiColor};
pub use alias_system::{AliasManager, SigmaAlias, AliasType};
pub use sovereign_shell_parity::{
    RedirectionType, ParsedPipelineCommand, SovereignBashZshParityShell,
};
