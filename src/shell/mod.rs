// SigmaOS Shell Module
pub mod alias_system;
pub mod command;
pub mod intelligent_terminal;
pub mod kimi_code_agent;
pub mod multicall;
pub mod repl;
pub mod terminal_emulator;
pub mod zsh_bash_parity;
pub mod sigma_sh;

pub use alias_system::{AliasManager, AliasType, SigmaAlias};
pub use command::{
    CommandError, CommandParser, CommandRegistry, ShellCommand, ShellSession,
    SimpleCommandRegistry, SimpleShellSession,
};
pub use terminal_emulator::{
    AnsiColor, AutoSuggestionEngine, BracketedPasteBuffer, BsdConsoleColorPalette,
    BsdConsoleTheme, TerminalSession, TermiosInputEvent, TermiosInputMode,
    TermiosLineDiscipline, UserDefinedFunction,
};
pub use zsh_bash_parity::{
    BashParameterExpansion, BsdDirectoryStack, CandidateCategory, CompletionCandidate,
    FuzzyCompletionEngine, HighlightedToken, PipelineCommand, PowerlinePromptBuilder,
    PromptTheme, ShellJob, ShellJobControl, ShellPipeline, ShellPipelineParser,
    SyntaxTokenKind, WildcardGlobMatcher, ZshSyntaxHighlighter, JobState,
};
pub use sigma_sh::{
    ContextualCompleter, HistoryExpansionEngine, JobControlManager, ParameterExpansionEngine,
    PipelineExecutor, ShellPledgeUnveilGuard, ShellSyntaxHighlighter as SigmaShSyntaxHighlighter, ZshPromptFormatter,
};
