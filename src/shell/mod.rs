// SigmaOS Shell Module
pub mod alias_system;
pub mod busybox_applet;
pub mod command;
pub mod intelligent_terminal;
pub mod kimi_code_agent;
pub mod multicall;
pub mod repl;
pub mod sigma_sh;
pub mod terminal_emulator;
pub mod zsh_bash_parity;
pub mod sovereign_shell_parity;

// pub use repl::{ShellCommand, ShellRepl};
pub use command::{CommandError, CommandParser, CommandRegistry, ShellCommand, ShellSession, SimpleCommandRegistry, SimpleShellSession};
pub use terminal_emulator::{TerminalSession, UserDefinedFunction, AutoSuggestionEngine, AnsiColor};
pub use alias_system::{AliasManager, SigmaAlias, AliasType};
pub use sovereign_shell_parity::{
    RedirectionType, ParsedPipelineCommand, SovereignBashZshParityShell,
};
pub use sigma_sh::{
    ContextualCompleter, HistoryExpansionEngine, JobControlManager, ParameterExpansionEngine,
    PipelineExecutor, ZshPromptFormatter,
};
pub use zsh_bash_parity::{
    BsdDirectoryStack, FuzzyCompletionEngine, PowerlinePromptBuilder, ShellJobControl,
    ZshSyntaxHighlighter,
};
