// SigmaOS Productivity Module
pub mod advanced_app_absorber;
pub mod calendar;
pub mod clipboard_manager;
pub mod document_engine;
pub mod editor;
pub mod email;
pub mod enterprise_productivity_suite;
pub mod finance;
pub mod flint_chart;
pub mod gamification;
pub mod linux_bsd_tools;
pub mod media;
pub mod mind_map;
pub mod mint_competitor;
pub mod reminders_advanced;
pub mod sigma_office;
pub mod sovereign_apps;
pub mod subtitle_editor;
pub mod tmux;
pub mod itsfoss_apps_synthesis;

pub use enterprise_productivity_suite::*;

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

pub use sigma_office::{
    CallLogEntry, CellValue, CpqProductBundleItem, DbColumnType, DbRow, DbTable, DbTableColumn,
    DealEscalationLevel, DiagramConnector, DiagramNode, DiagramNodeType, DocumentBranch,
    DocumentNode, DocumentType, DripStep, DtpPageLayout, EnterpriseDeal, EnterpriseInvoice,
    FieldServiceJob, FormQuestion, FormResponse, HelpdeskTicket, InlineDocComment, InventoryItem,
    JournalEntryLine, KdsOrderTicket, KdsTicketStatus, KnowledgeArticle, LeadAssignmentRule,
    LiveCoAuthoringManager, LookerChartWidget, LookerFilterControl, LookerGaugeWidget,
    LookerMetricCard, LoopComponent, MacroExecutor, MultiEntityJournalTransaction, OdfDocumentKind,
    ParagraphStyle, PosOrderItem, PresentationProcessor, QuestionType, QuickNoteItem,
    ScriptExecutionRecord, ScriptTriggerEvent, SigmaFormulaParserEngine, SigmaOdfPackageEngine,
    SigmaOffice, SigmaSlideDetails, SigmaSpellCheckerEngine, SigmaStyleThemeEngine,
    SigmaTrackChangesEngine, SmartChipNode, SmartChipType, SovereignAppsScriptTriggerEngine,
    SovereignCollaborativeWhiteboardEngine, SovereignCpqEngine, SovereignCrmPipeline,
    SovereignEmployeeOrgChartEngine, SovereignFleetFieldServiceEngine, SovereignFormsSurveyEngine,
    SovereignHelpdeskSlaEngine, SovereignIntegrationWorkflowEngine,
    SovereignInventoryWarehouseEngine, SovereignLoopPortableComponentEngine,
    SovereignLowCodeDatabaseEngine, SovereignMacroAutomationSandbox,
    SovereignManufacturingMrpEngine, SovereignMarketingCampaignEngine, SovereignPbxCallCenterEngine,
    SovereignPosKitchenDisplayEngine, SovereignPublisherDtpEngine, SovereignQuickNotesEngine,
    SovereignServiceCloudKnowledgeEngine, SovereignSmartCanvasEngine,
    SovereignTaxGstAccountingEngine, SovereignVidsPresentationEngine, SovereignVisioDiagrammingEngine,
    SovereignWebPublisherEngine, SovereignWorkgroupGanttEngine, SpreadsheetProcessor, SuggestionEdit,
    TextProcessor, TicketPriority, TicketStatus, TypographyRenderer, VersionHistoryManager, VidScene,
    WebLayoutBlock, WhiteboardElement, WhiteboardElementType, WorkgroupTask, WorkflowAction, WorkflowRule,
    WorkflowTrigger,
};
