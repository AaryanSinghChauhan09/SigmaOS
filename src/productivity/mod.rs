// SigmaOS Productivity Module
pub mod editor;
pub mod gamification;
pub mod notes;
pub mod screen_recorder;
pub mod tasks;
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
pub use notes::{
    ContentType, Folder, InMemoryNoteStorage, Notebook, Note, NoteError, NoteSearchResult,
    NoteStorage, NoteTakingApp,
};
pub use screen_recorder::{
    AudioQuality, FfmpegBackend, GStreamerBackend, RecorderError, RecordingBackend,
    RecordingConfig, RecordingFormat, RecordingProgress, RecordingRegion, RecordingState,
    ScreenRecorder, VideoQuality,
};
pub use tasks::{
    InMemoryStorage, KanbanBoard, KanbanColumn, Project, Reminder, ReminderType, Subtask, Task,
    TaskError, TaskManager, TaskPriority, TaskStatus, TaskStorage,
};
pub use terminal::{
    BashShell, ColorScheme, CommandResult, CursorStyle, IntegratedTerminal, SigmaShell, ShellImpl,
    ShellType, TerminalConfig, TerminalError, TerminalSession, ZshShell,
};
