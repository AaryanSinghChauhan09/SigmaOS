// SigmaOS Productivity Module
pub mod editor;
pub mod gamification;
pub mod terminal;

pub use editor::{
    CodeEditor, CompletionItem, CompletionKind, CursorPosition, Diagnostic, DiagnosticSeverity,
    Document, EditorConfig, EditorError, Language, LspClient, LspFeature, MockLspClient, RegexHighlighter,
    SyntaxHighlighter, SyntaxToken, TextRange, TextSelection, TokenType,
};
pub use gamification::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use terminal::{
    BashShell, ColorScheme, CommandResult, CursorStyle, IntegratedTerminal, SigmaShell, ShellImpl,
    ShellType, TerminalConfig, TerminalError, TerminalSession, ZshShell,
};
