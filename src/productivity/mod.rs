// SigmaOS Productivity Module
<<<<<<< HEAD
pub mod advanced_app_absorber;
pub mod gamification;
pub mod media;
pub mod notes;
pub mod screen_recorder;
pub mod screenshot;
pub mod sigma_office;
=======
pub mod calendar;
pub mod clipboard_manager;
pub mod editor;
pub mod email;
pub mod gamification;
pub mod notes;
pub mod screen_recorder;
pub mod screenshot;
>>>>>>> origin/jules-18101178622594638830-97dc43c6
pub mod tasks;
pub mod terminal;

pub use advanced_app_absorber::{
    AudacityEditor, BraveBrowserEngine, EarTrumpetAudioRouter, EverythingSearchEngine,
    NotepadPlusWorkspace, ObsStudioMixer, OneCommanderDualPane, PotPlayerVlcEngine,
    SevenZipCompressor, ShareXFlameshotEngine,
};
<<<<<<< HEAD
=======
pub use clipboard_manager::{
    ClipboardBackend, ClipboardError, ClipboardFilter, ClipboardHistoryConfig, ClipboardItem,
    ClipboardItemType, ClipboardManager, SystemClipboardBackend,
};
pub use editor::{
    CodeEditor, CompletionItem, CompletionKind, CursorPosition, Diagnostic, DiagnosticSeverity,
    Document, EditorConfig, EditorError, Language, LspClient, LspFeature, MockLspClient,
    RegexHighlighter, SyntaxHighlighter, SyntaxToken, TextRange, TextSelection, TokenType,
};
pub use email::{
    EmailAccountConfig, EmailAddress, EmailAttachment, EmailBackend, EmailClient, EmailError,
    EmailFilter, EmailFolder, EmailMessage, EmailPriority, ImapSmtpBackend,
};
>>>>>>> origin/jules-18101178622594638830-97dc43c6
pub use gamification::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use notes::{
    ContentType, Folder, InMemoryNoteStorage, Note, NoteError, NoteSearchResult, NoteStorage,
    NoteTakingApp, Notebook,
};
pub use screen_recorder::{
    AudioQuality, FfmpegBackend, GStreamerBackend, RecorderError, RecordingBackend,
    RecordingConfig, RecordingFormat, RecordingProgress, RecordingRegion, RecordingState,
    ScreenRecorder, VideoQuality,
};
pub use screenshot::{
    CaptureRegion, ImageFormat, MacOsBackend, ScreenshotBackend, ScreenshotConfig, ScreenshotError,
    ScreenshotMode, ScreenshotResult, ScreenshotTool, WaylandBackend, WindowsBackend, X11Backend,
};
pub use tasks::{
    InMemoryStorage, KanbanBoard, KanbanColumn, Project, Reminder, ReminderType, Subtask, Task,
    TaskError, TaskManager, TaskPriority, TaskStatus, TaskStorage,
};
pub use terminal::{
    BashShell, ColorScheme, CommandResult, CursorStyle, IntegratedTerminal, ShellImpl, ShellType,
    SigmaShell, TerminalConfig, TerminalError, TerminalSession, ZshShell,
};
