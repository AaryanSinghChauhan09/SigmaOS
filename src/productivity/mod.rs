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
pub mod itsfoss_apps_synthesis;
pub mod linux_bsd_tools;
pub mod media;
pub mod mind_map;
pub mod mint_competitor;
pub mod reminders_advanced;
pub mod sigma_office;
pub mod sovereign_apps;
pub mod subtitle_editor;
pub mod tmux;

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
pub use reminders_advanced::{
    EnhancedRemindersEngine, RecurrencePattern, ReminderItem, ReminderPriority,
};
pub use sovereign_apps::{
    ProductivityTask, SigmaOfficeDocument, SigmaTasksBoard, SigmaVaultContainer, TaskPriority,
    TextNode,
};
pub use tmux::{
    LayoutPreset, SplitDirection, TmuxPane, TmuxSession, TmuxSessionManager, TmuxWindow,
};

pub use mind_map::{
    IndentedTextMindMapParserEngine, MindMapCreator, MindMapLayout, MindMapNode, NodeShape,
    NodeStyle, RelationshipConnection,
};

pub use sigma_office::{
    ApprovalThresholdTier, BigQueryConnectedSheetConfig, BlendDataSourceSpec, BlendJoinType,
    BomTreeNode, CallLogEntry, CanvasAppScreen, CellOrSectionLock, CellValue, ChurnRiskLevel,
    CitationItem, CitationStyle, CohortMetrics, CommunicationChannel, ConnectedSheetQueryResult,
    CpqProductBundleItem, CustomerMetrics, DataStoryStep, DbColumnType, DbRow, DbTable,
    DbTableColumn, DealEscalationLevel, DiagramConnector, DiagramNode, DiagramNodeType,
    DigitalSignatureStamp, DocumentBranch, DocumentNode, DocumentType, DripStep, DtpPageLayout,
    DynamicArraySpillResult, EnterpriseDeal, EnterpriseInvoice, ExecutiveKpiItem,
    ExpenseApprovalStatus, ExpenseClaimItem, FieldServiceJob, FormQuestion, FormResponse,
    HelpdeskTicket, InlineDocComment, InventoryItem, JournalEntryLine, KdsOrderTicket,
    KdsTicketStatus, KnowledgeArticle, KnowledgeGraphEdge, KnowledgeGraphNode,
    KnowledgeGraphNodeType, KpiMetricStatus, LeadAssignmentRule, LeadBehaviorEvent,
    LeadScoreRecord, LiveCoAuthoringManager, LookerChartWidget, LookerFilterControl,
    LookerGaugeWidget, LookerMetricCard, LoopComponent, MacroExecutor,
    MultiEntityJournalTransaction, OdfDocumentKind, OmnichannelMessage, ParagraphStyle,
    PivotAggregateFunc, PivotSummaryResult, PosOrderItem, PresentationProcessor, QualityCheckPoint,
    QuestionType, QuickNoteItem, ResourceAllocation, RpaStage, ScriptExecutionRecord,
    ScriptTriggerEvent, SharedDriveFileLock, SharedDriveMember, SharedDriveRole,
    SigmaFormulaParserEngine, SigmaOdfPackageEngine, SigmaOffice, SigmaSlideDetails,
    SigmaSpellCheckerEngine, SigmaStyleThemeEngine, SigmaTrackChangesEngine, SlaEscalationRule,
    SmartCanvasChip, SmartCanvasChipKind, SmartWriterTone, SovereignAcademicCitationEngine,
    SovereignAiDocumentSummarizerEngine, SovereignAppsScriptTriggerEngine,
    SovereignBitrix24OmnichannelTelephonyCrmEngine, SovereignBitrix24RpaWorkflowEngine,
    SovereignBitrix24SpeechAiSentimentAnalyzerEngine, SovereignChurnPredictionRetentionEngine,
    SovereignCollaborativeWhiteboardEngine, SovereignCpqEngine, SovereignCrmLeadScoringEngine,
    SovereignCrmPipeline, SovereignDataStorytellingEngine, SovereignDigitalContractSignatureEngine,
    SovereignDynamicArrayFormulaSpillEngine, SovereignEmployeeOrgChartEngine,
    SovereignEnterpriseChatSpaceEngine, SovereignEnterpriseIntranetPortalEngine,
    SovereignEnterpriseKnowledgeGraphEngine, SovereignEnterpriseResourceAllocationEngine,
    SovereignEquipmentMaintenanceEngine, SovereignExecutiveKpiScorecardEngine,
    SovereignExpenseClaimApprovalEngine, SovereignFinancialValuationEngine,
    SovereignFleetFieldServiceEngine, SovereignFormsSurveyEngine, SovereignGoalSeekSolverEngine,
    SovereignGoogleDocsGeminiSmartWriterEngine, SovereignGoogleLookerStudioDataBlendEngine,
    SovereignGoogleSheetsBigQueryDataConnectorEngine, SovereignGoogleSmartCanvasChipEngine,
    SovereignHelpdeskSlaEngine, SovereignIntegrationWorkflowEngine,
    SovereignInventoryWarehouseEngine, SovereignLandingPageCmsEngine,
    SovereignLookerAdvancedVisualizationEngine, SovereignLoopPortableComponentEngine,
    SovereignLowCodeBusinessProcessEngine, SovereignLowCodeDatabaseEngine,
    SovereignMacroAutomationSandbox, SovereignManufacturingMrpEngine,
    SovereignMarketingCampaignEngine, SovereignMicrosoft365CoauthoringLockEngine,
    SovereignMicrosoftPowerAppsCanvasEngine, SovereignMicrosoftTeamsChannelBotWorkflowEngine,
    SovereignOdooInventorySupplyChainMrpEngine, SovereignOdooMrpBillOfMaterialsTreeEngine,
    SovereignOdooQualityControlInspectionEngine, SovereignOmnichannelCommunicationGateway,
    SovereignOmnichannelLiveChatEngine, SovereignPbxCallCenterEngine,
    SovereignPivotTableSummaryEngine, SovereignPosKitchenDisplayEngine,
    SovereignPublisherDtpEngine, SovereignQuickNotesEngine, SovereignQuoteToCashEngine,
    SovereignSalesforceCpqQuoteDiscountEngine, SovereignSalesforceEinsteinAnalyticsPipelineEngine,
    SovereignSalesforceEinsteinLeadScoringAiEngine, SovereignServiceCloudKnowledgeEngine,
    SovereignSharedDrivePermissionEngine, SovereignSmartCanvasEngine,
    SovereignSmartDocumentTemplateEngine, SovereignTaxGstAccountingEngine,
    SovereignVidsPresentationEngine, SovereignVisioDiagrammingEngine,
    SovereignWebConferencingEngine, SovereignWebPublisherEngine,
    SovereignWorkgroupActivityStreamEngine, SovereignWorkgroupGanttEngine,
    SovereignWorkspaceAddonExtensionEngine, SovereignZohoAnalyticsCohortAnalysisEngine,
    SovereignZohoDeskTicketSlaEscalationEngine, SpreadsheetProcessor, SuggestionEdit,
    TeamsBotTriggerEvent, TextProcessor, TicketPriority, TicketStatus, TranscriptAnalysisResult,
    TypographyRenderer, VectorTimestamp, VersionHistoryManager, VidScene, WebLayoutBlock,
    WhiteboardElement, WhiteboardElementType, WorkflowAction, WorkflowRule, WorkflowTrigger,
    WorkgroupChannelMessage, WorkgroupTask,
};
