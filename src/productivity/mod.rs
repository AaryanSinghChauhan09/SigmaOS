// SigmaOS Productivity Module
pub mod calendar;
pub mod clipboard_manager;
pub mod document_engine;
pub mod editor;
pub mod email;
pub mod finance;
pub mod gamification;
pub mod notes;
pub mod screen_recorder;
pub mod screenshot;
pub mod sigma_office;
pub mod tasks;
pub mod terminal;

pub use calendar::{
    CalendarApp, CalendarError, CalendarEvent, CalendarStorage, CalendarView, DayInfo, EventStatus,
    EventType, InMemoryCalendarStorage, RecurrencePattern,
};
pub use clipboard_manager::{
    ClipboardBackend, ClipboardError, ClipboardFilter, ClipboardHistoryConfig, ClipboardItem,
    ClipboardItemType, ClipboardManager, SystemClipboardBackend,
};
pub use document_engine::{Document, DocumentEngine, DocumentFormat, DocumentMetadata};
pub use editor::{
    CodeEditor, CompletionItem, CompletionKind, CursorPosition, Diagnostic, DiagnosticSeverity,
    Document as EditorDocument, EditorConfig, EditorError, Language, LspClient, LspFeature,
    MockLspClient, RegexHighlighter, SyntaxHighlighter, SyntaxToken, TextRange, TextSelection,
    TokenType,
};
pub use email::{
    EmailAccountConfig, EmailAddress, EmailAttachment, EmailBackend, EmailClient, EmailError,
    EmailFilter, EmailFolder, EmailMessage, EmailPriority, ImapSmtpBackend,
};
pub use finance::{
    DeducteeType, GstBreakdown, GstCalculator, GstTransactionType, HsnEntry, IncomeTaxCalculator,
    IncomeTaxResult, IndiaIncomeTaxCalculator, IndiaTdsCalculator, IndianNumberFormatter,
    IndicLanguage, NpciUpiGateway, StandardGstCalculator, TaxRegime, TdsCalculator, TdsResult,
    TdsSection, UpiGateway, UpiPaymentRequest, UpiVpa,
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
    CellValue, ChartType, DocumentMetadata, DocumentNode, DocumentType, PresentationProcessor,
    ShapeType, SigmaDocument, SigmaOffice, SlideElementType, SpreadsheetProcessor, TextProcessor,
    TypographyRenderer,
};
pub use tasks::{
    InMemoryStorage, KanbanBoard, KanbanColumn, Project, Reminder, ReminderType, Subtask, Task,
    TaskError, TaskManager, TaskPriority, TaskStatus, TaskStorage,
};
pub use terminal::{
    BashShell, ColorScheme, CommandResult, CursorStyle, IntegratedTerminal, ShellImpl, ShellType,
    SigmaShell, TerminalConfig, TerminalError, TerminalSession, ZshShell,
};
