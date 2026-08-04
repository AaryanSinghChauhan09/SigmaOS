// SigmaOS Productivity Module
pub mod advanced_app_absorber;
pub mod calendar;
pub mod clipboard_manager;
pub mod document_engine;
pub mod editor;
pub mod email;
pub mod finance;
pub mod flint_chart;
pub mod gamification;
pub mod media;
pub mod mind_map;
pub mod notes;
pub mod pdf;
pub mod screen_recorder;
pub mod screenshot;
pub mod sigma_office;
pub mod subtitle_editor;
pub mod tasks;
pub mod terminal;
pub mod tmux;
pub mod utility_suite;

pub use calendar::{
    CalendarApp, CalendarEvent, CalendarView, EventStatus, EventType, RecurrencePattern,
};
pub use clipboard_manager::{
    ClipboardHistory, ClipboardItem, ClipboardItemType, ClipboardManager,
};
pub use document_engine::{
    DocumentEngine, DocumentFormat, DocumentMetadata, DocumentResult,
};
pub use editor::{
    CodeEditor, CursorPosition, Document, Language, Selection, TextSelection,
};
pub use email::{
    EmailAccountConfig, EmailAddress, EmailAttachment, EmailClient, EmailFolder, EmailMessage,
    ImapSmtpBackend,
};
pub use finance::{
    Asset, Budget, Expense, FinancialManager, Income, Transaction as FinancialTransaction,
    EngineeringCalculators, FinancialProfessionCalculators, MedicalDoctorCalculators,
    DepreciationMethod, TaxRegime,
};
pub use flint_chart::{
    ChartData, ChartEngine, ChartSeries, DataPoint, LegendPosition,
};
pub use gamification::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use notes::{
    ContentType, Folder, InMemoryNoteStorage, Note, NoteError, NoteSearchResult, NoteStorage,
    NoteTakingApp, Notebook,
};
pub use pdf::{
    PdfDocument, PdfEngine, PdfMetadata, PdfPage,
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
pub use subtitle_editor::{
    Subtitle, SubtitleEditor, SubtitleFormat,
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
pub use utility_suite::{
    Calculator, SystemUtilitySuite, UnitConverter, WeatherService,
};
