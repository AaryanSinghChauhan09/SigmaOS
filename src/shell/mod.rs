// SigmaOS Shell Module
pub mod alias_system;
pub mod command;
pub mod intelligent_terminal;
pub mod kimi_code_agent;
pub mod multicall;
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
pub mod alias_system;
pub mod zsh_bash_parity;
pub use command::{CommandError, CommandParser, CommandRegistry, ShellCommand, ShellSession, SimpleCommandRegistry, SimpleShellSession};
pub use terminal_emulator::{TerminalSession, UserDefinedFunction, AutoSuggestionEngine, AnsiColor};
pub use alias_system::{AliasManager, SigmaAlias, AliasType};
pub use zsh_bash_parity::{
    PowerlinePromptBuilder, PromptTheme, FuzzyCompletionEngine, CompletionCandidate, CandidateCategory,
    ZshSyntaxHighlighter, SyntaxTokenKind, HighlightedToken, BashParameterExpansion, WildcardGlobMatcher,
    ShellPipelineParser, ShellPipeline, PipelineCommand, BsdDirectoryStack, ShellJobControl, ShellJob, JobState,
};
