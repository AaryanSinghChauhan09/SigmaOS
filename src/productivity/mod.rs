// SigmaOS Productivity Module
pub mod gamification;
pub mod media;
pub mod notes;
pub mod screen_recorder;
pub mod screenshot;
pub mod sigma_office;
pub mod tasks;
pub mod terminal;
pub mod tmux;
pub mod mind_map;
pub mod utility_suite;

pub use utility_suite::{
    EverythingSearchEngine, NotepadPlusPlusBuffer, SovereignBrowserEngine, SevenZipEngine,
    CompressionMethod, FlameshotAnnotator, AnnotationShape, ObsStudioMixer,
    AudacityWaveEditor, VlcCodecPipeline, DaVinciTimeline, OneCommanderFileGrid,
    ItemAgeColor, EarTrumpetVolumeMatrix, IrfanViewEngine,
};
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
    SplitDirection, LayoutPreset, TmuxPane, TmuxWindow, TmuxSession, TmuxSessionManager,
};
pub use mind_map::{
    MindMapCreator, MindMapNode, MindMapLayout, NodeShape, NodeStyle, RelationshipConnection,
};
