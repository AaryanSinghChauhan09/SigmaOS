// SigmaOS Productivity Module
pub mod calendar;
pub mod clipboard_manager;
pub mod document_engine;
pub mod editor;
pub mod email;
pub mod finance;
pub mod flint_chart;
pub mod gamification;

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
pub use flint_chart::{FlintChartEngine, FlintChartSpec, FlintEncoding, FlintMark, FlintValue};
pub use gamification::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
