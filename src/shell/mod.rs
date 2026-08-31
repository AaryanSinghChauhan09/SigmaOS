// SigmaOS Shell Module
pub mod alias_system;
pub mod command;
pub mod sovereign_shell_parity;
pub mod terminal_emulator;
pub mod zsh_bash_parity;

pub use zsh_bash_parity::{
    DashPosixShValidator, FishAbbreviationEngine, KshParameterExpansionEngine, ShellDialect,
    ShellPipeline, ShellPipelineParser, TcshHistorySubstitutionEngine,
    UniversalShellCompatibilityEngine,
};

// pub use repl::{ShellCommand, ShellRepl};
pub use command::{CommandError, CommandParser, CommandRegistry, ShellCommand, ShellSession, SimpleCommandRegistry, SimpleShellSession};
pub use terminal_emulator::{TerminalSession, UserDefinedFunction, AutoSuggestionEngine, AnsiColor};
pub use alias_system::{AliasManager, SigmaAlias, AliasType};
pub use sovereign_shell_parity::{
    RedirectionType, ParsedPipelineCommand, SovereignBashZshParityShell,
};
