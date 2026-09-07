// SigmaOS Shell Module
pub mod alias_system;
pub mod command;
pub mod repl;
pub mod sigma_sh;
pub mod sovereign_shell_parity;
pub mod terminal_emulator;
pub mod zsh_bash_parity;

pub use alias_system::{AliasManager, AliasType, SigmaAlias};
pub use command::{
    CommandError, CommandParser, CommandRegistry, ShellCommand, ShellSession,
    SimpleCommandRegistry, SimpleShellSession,
};
pub use repl::ShellRepl;
pub use sigma_sh::{
    AutoSuggestTabPopup, ContextualCompleter, HistoryExpansionEngine, JobControlManager,
    ParameterExpansionEngine, PipelineExecutor, PipelinePlan, ReplLineEditor, ShellPledgeUnveilGuard, ShellSyntaxHighlighter, SimpleShell,
    SovereignSigmaShRepl, ZshPromptFormatter,
};
pub use terminal_emulator::{
    AnsiColor, AutoSuggestionEngine, BsdConsoleColorPalette, BsdConsoleTheme,
    TerminalSession, TermiosInputEvent, TermiosInputMode, TermiosLineDiscipline,
    UserDefinedFunction,
};
pub use zsh_bash_parity::{
    BashParameterExpansion, BsdDirectoryStack, CandidateCategory, CompletionCandidate,
    DashPosixShValidator, FishAbbreviationEngine, FuzzyCompletionEngine, HighlightedToken,
    JobState, KshParameterExpansionEngine, PipelineCommand, PowerlinePromptBuilder, PromptTheme,
    ShellArithmeticEvaluator, ShellDialect, ShellJob, ShellJobControl, ShellPipeline,
    ShellPipelineParser, ShellScriptHookEngine, SyntaxTokenKind, TcshHistorySubstitutionEngine,
    UniversalScriptTranspiler, UniversalShellCompatibilityEngine, WildcardGlobMatcher,
    ZshSyntaxHighlighter,
};
