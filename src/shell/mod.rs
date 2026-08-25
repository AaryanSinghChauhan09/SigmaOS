// SigmaOS Shell Module
pub mod command;
pub mod repl;
pub mod terminal_emulator;
pub mod alias_system;
pub mod zsh_bash_parity;

// pub use repl::{ShellCommand, ShellRepl};
pub use command::{CommandError, CommandParser, CommandRegistry, ShellCommand, ShellSession, SimpleCommandRegistry, SimpleShellSession};
pub use terminal_emulator::{TerminalSession, UserDefinedFunction, AutoSuggestionEngine, AnsiColor};
pub use alias_system::{AliasManager, SigmaAlias, AliasType};
pub use zsh_bash_parity::{
    PowerlinePromptBuilder, PromptTheme, FuzzyCompletionEngine, CompletionCandidate, CandidateCategory,
    ZshSyntaxHighlighter, SyntaxTokenKind, HighlightedToken, BashParameterExpansion, WildcardGlobMatcher,
    ShellPipelineParser, ShellPipeline, PipelineCommand, BsdDirectoryStack, ShellJobControl, ShellJob, JobState,
};
