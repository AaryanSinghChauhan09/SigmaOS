// SigmaOS Shell Module
pub mod alias_system;
pub mod command;
pub mod sigma_sh;
pub mod sovereign_shell_parity;
pub mod terminal_emulator;
pub mod zsh_bash_parity;

pub use sigma_sh::{
    AutoSuggestTabPopup, ContextualCompleter, HistoryExpansionEngine, JobControlManager,
    ParameterExpansionEngine, ReplLineEditor, ShellPledgeUnveilGuard, SimpleShell,
    SovereignSigmaShRepl, ZshPromptFormatter,
};

pub use sigma_sh::{
    AutoSuggestTabPopup, ContextualCompleter, HistoryExpansionEngine, JobControlManager,
    ParameterExpansionEngine, ReplLineEditor, ShellPledgeUnveilGuard, SimpleShell,
    SovereignSigmaShRepl, ZshPromptFormatter,
};

pub use sigma_sh::{
    AutoSuggestTabPopup, ContextualCompleter, HistoryExpansionEngine, JobControlManager,
    ParameterExpansionEngine, ReplLineEditor, ShellPledgeUnveilGuard, SimpleShell,
    SovereignSigmaShRepl, ZshPromptFormatter,
};

// pub use repl::{ShellCommand, ShellRepl};
pub use alias_system::{AliasManager, AliasType, SigmaAlias};
pub use command::{
    CommandError, CommandParser, CommandRegistry, ShellCommand, ShellSession,
    SimpleCommandRegistry, SimpleShellSession,
};
pub use sovereign_shell_parity::{
    ParsedPipelineCommand, RedirectionType, SovereignBashZshParityShell,
};
pub use terminal_emulator::{
    AnsiColor, AutoSuggestionEngine, TerminalSession, UserDefinedFunction,
};
