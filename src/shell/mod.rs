// SigmaOS Shell Module
pub mod alias_system;
pub mod command;
pub mod repl;
pub mod sigma_sh;
pub mod sovereign_shell_parity;
pub mod terminal_emulator;
pub mod zsh_bash_parity;
pub mod tech_media_shell_innovations;

pub use sigma_sh::{
    AutoSuggestTabPopup, ContextualCompleter, HistoryExpansionEngine, JobControlManager,
    ParameterExpansionEngine, PipelineExecutor, PipelinePlan, ReplLineEditor, ShellPledgeUnveilGuard,
    ShellSyntaxHighlighter, SimpleShell, SovereignSigmaShRepl, ZshPromptFormatter,
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
pub use command::{
    CommandError, CommandParser, CommandRegistry, ShellCommand, ShellSession,
    SimpleCommandRegistry, SimpleShellSession,
};
pub use terminal_emulator::{
    TerminalSession, UserDefinedFunction, AutoSuggestionEngine, AnsiColor,
    BsdConsoleColorPalette, BsdConsoleTheme,
    TermiosInputEvent, TermiosInputMode, TermiosLineDiscipline,
};
pub use tech_media_shell_innovations::{
    AdbDeviceDescriptor, BenchmarkResultNode, HwbustersPsuSensorMonitorEngine,
    KdnuggetsAiDataShellEngine, KubectlPodNode, PhoronixAutomatedBenchmarkShellHook,
    SovereignTechMediaShellInnovationsSuite, TensorDataColumn, TheNewStackCloudKubectlEngine,
    XdaAndroidAdbFastbootShellBridge,
};
