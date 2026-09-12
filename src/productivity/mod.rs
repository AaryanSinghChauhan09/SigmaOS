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
pub mod linux_bsd_tools;
pub mod media;
pub mod mind_map;
pub mod mint_competitor;
pub mod reminders_advanced;
pub mod sovereign_apps;
pub mod subtitle_editor;
pub mod tmux;
pub mod itsfoss_apps_synthesis;
pub mod enterprise_productivity_suite;

pub use enterprise_productivity_suite::{
    AccountRecord, CommentThread, DataConnector, DashboardWidget, DataSourceType, DealStage, EditStatus,
    EmployeeRecord, EnterpriseDocumentSession, InvoiceItem, InvoiceRecord, IntranetPost, LeadRecord,
    LookerDashboard, PresentationDeck, PresentationSlide, SigmaDocsEnterpriseCollaborationEngine,
    SigmaLookerAnalyticsEngine, SigmaSlidesPresenterEngine, SlideLayout, SovereignEnterpriseCrmErpEngine,
    SovereignOdooBitrixSuite, SuggestedEdit, TransitionEffect, UserRole, WidgetType,
};

pub use itsfoss_apps_synthesis::{
    IptvChannelNode, ItsFossBulkyBatchRenamerEngine, ItsFossHypnotixIptvEngine,
    ItsFossStickyNotesEngine, ItsFossWarpinatorLanSharingEngine, RenameRuleResult,
    SovereignItsFossAppsSuite, StickyNoteEntry, WarpinatorLanPeer,
};

pub use gamification::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use media::{AudioChannel, SigmaMediaEngine, GLOBAL_MEDIA_ENGINE};
pub use sovereign_apps::{
    ProductivityTask, SigmaOfficeDocument, SigmaTasksBoard, SigmaVaultContainer, TaskPriority,
    TextNode,
};
pub use reminders_advanced::{
    EnhancedRemindersEngine, RecurrencePattern, ReminderItem, ReminderPriority,
};
pub use tmux::{
    LayoutPreset, SplitDirection, TmuxPane, TmuxSession, TmuxSessionManager, TmuxWindow,
};

pub use mind_map::{IndentedTextMindMapParserEngine, MindMapCreator, MindMapNode, MindMapLayout, NodeShape, NodeStyle, RelationshipConnection};
