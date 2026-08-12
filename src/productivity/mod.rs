// SigmaOS Productivity Module
pub mod advanced_app_absorber;
pub mod gamification;
pub mod media;
pub mod mind_map;
pub mod notes;
pub mod screen_recorder;
pub mod screenshot;
pub mod sigma_office;
pub mod tasks;
pub mod terminal;
pub mod tmux;
pub mod editor;

pub use advanced_app_absorber::{
    AudacityEditor, BraveBrowserEngine, EarTrumpetAudioRouter, EverythingSearchEngine,
    NotepadPlusWorkspace, ObsStudioMixer, OneCommanderDualPane, PotPlayerVlcEngine,
    SevenZipCompressor, ShareXFlameshotEngine,
};
pub use gamification::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use mind_map::{
    MindMapCreator, MindMapLayout, MindMapNode, NodeShape, NodeStyle, RelationshipConnection,
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
pub use sigma_office::{
    CellValue, ChartType, DocumentMetadata as SigmaOfficeDocumentMetadata, DocumentNode,
    DocumentType, PresentationProcessor, ShapeType, SigmaDocument, SigmaOffice, SlideElementType,
    SpreadsheetProcessor, TextProcessor, TypographyRenderer,
};
pub use tasks::{
    InMemoryStorage, KanbanBoard, KanbanColumn, Project, Reminder, ReminderType, Subtask, Task,
    TaskError, TaskManager, TaskPriority, TaskStatus, TaskStorage,
};
pub use terminal::{
    BashShell, ColorScheme, CommandResult, CursorStyle, IntegratedTerminal, ShellImpl, ShellType,
    SigmaShell, TerminalConfig, TerminalError, TerminalSession, ZshShell,
};
pub use tmux::{
    LayoutPreset, SplitDirection, TmuxPane, TmuxSession, TmuxSessionManager, TmuxWindow,
};
pub use editor::{
    CodeEditor, Document as EditorDocument, EditorConfig, EditorError, Language as EditorLanguage,
    LspClient, SyntaxHighlighter,
};
