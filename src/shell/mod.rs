#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Shell Module
pub mod alias_system;
pub mod command;
pub mod repl;
pub mod sigma_sh;
pub mod sovereign_shell_parity;
pub mod terminal_emulator;
pub mod zsh_bash_parity;

pub use sigma_sh::{
    AutoSuggestTabPopup, ContextualCompleter, HistoryExpansionEngine, JobControlManager,
    ParameterExpansionEngine, PipelineExecutor, PipelinePlan, ReplLineEditor, ShellPledgeUnveilGuard, SimpleShell,
    SovereignSigmaShRepl, ZshPromptFormatter,
};

pub use repl::ShellRepl;
pub use zsh_bash_parity::{
    BashParameterExpansion, BsdDirectoryStack, DashPosixShValidator, FishAbbreviationEngine,
    FuzzyCompletionEngine, KshParameterExpansionEngine, PowerlinePromptBuilder,
    ShellArithmeticEvaluator, ShellDialect, ShellJobControl, ShellScriptHookEngine,
    TcshHistorySubstitutionEngine, UniversalScriptTranspiler, UniversalShellCompatibilityEngine,
    WildcardGlobMatcher, ZshSyntaxHighlighter,
};
pub use alias_system::{AliasManager, AliasType, SigmaAlias};
pub mod zsh_bash_parity;
pub use command::{
    CommandError, CommandParser, CommandRegistry, ShellCommand, ShellSession,
    SimpleCommandRegistry, SimpleShellSession,
};
pub use terminal_emulator::{
    TerminalSession, UserDefinedFunction, AutoSuggestionEngine, AnsiColor,
    BsdConsoleColorPalette, BsdConsoleTheme,
    TermiosInputEvent, TermiosInputMode, TermiosLineDiscipline,
};
pub use alias_system::{AliasManager, SigmaAlias, AliasType};
pub use zsh_bash_parity::{
    PowerlinePromptBuilder, PromptTheme, FuzzyCompletionEngine, CompletionCandidate, CandidateCategory,
    ZshSyntaxHighlighter, SyntaxTokenKind, HighlightedToken, BashParameterExpansion, WildcardGlobMatcher,
    ShellPipelineParser, ShellPipeline, PipelineCommand, BsdDirectoryStack, ShellJobControl, ShellJob, JobState,
};
pub use sigma_sh::{
    ContextualCompleter, HistoryExpansionEngine, JobControlManager, ParameterExpansionEngine,
    PipelineExecutor, ShellPledgeUnveilGuard, ShellSyntaxHighlighter, ZshPromptFormatter,
};
