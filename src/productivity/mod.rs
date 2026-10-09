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
    AdaptiveCardAction, AgentStatus, AiWriterTone, AppPortalModule, BlendJoinType,
    CalculatedFieldOp, CallLogEntry, CallQueueStrategy, CallSentimentReport, CellValue,
    ChurnRiskLevel, CitationItem, CitationStyle, CoauthoringLock, CommunicationChannel,
    ContactPoint, CpqProductBundleItem, CustomerMetrics, DataBlendConfig, DataStoryStep,
    DbColumnType, DbRow, DbTable, DbTableColumn, DealEscalationLevel, DeskTicket, DiagramConnector,
    DiagramNode, DiagramNodeType, DigitalSignatureStamp, DiscountTier, DocSnapshot, DocumentBranch,
    DocumentNode, DocumentType, DoubleEntryLine, DripStep, DtpPageLayout, DynamicArraySpillResult,
    EnterpriseDeal, EnterpriseInvoice, ExpenseApprovalStatus, ExpenseClaimItem, FieldServiceJob,
    FlowNode, FlowNodeKind, ForecastResult, FormQuestion, FormResponse, GeminiSuggestion,
    HelpdeskTicket, InlineDocComment, InventoryItem, JournalEntryLine, KanbanStage, KdsOrderTicket,
    KdsTicketStatus, KeepNote, KnowledgeArticle, KnowledgeGraphEdge, KnowledgeGraphNode,
    KnowledgeGraphNodeType, LeadAssignmentRule, LeadBehaviorEvent, LeadScoreRecord, LeadSignal,
    LiveCoAuthoringManager, LookerChartWidget, LookerFilterControl, LookerGaugeWidget,
    LookerMetricCard, LoopBlockType, LoopComponent, LoopComponentBlock, MacroExecutor,
    MultiEntityJournalTransaction, OdfDocumentKind, OdooBomTreeNode, OmnichannelMessage,
    ParagraphStyle, PivotAggregateFunc, PivotCalculatedField, PivotSummaryResult, PosOrderItem,
    PowerFxVariable, PresentationProcessor, QualityCheckPoint, QualityCheckType,
    QualityInspectionResult, QueryJoinType, QuestionType, QuickNoteItem, RentalContract, RpaStage,
    RpaTask, ScriptExecutionRecord, ScriptTriggerEvent, SentimentScore, SharedDriveFileLock,
    SharedDriveMember, SharedDriveRole, SigmaFormulaParserEngine, SigmaOdfPackageEngine,
    SigmaOffice, SigmaSlideDetails, SigmaSpellCheckerEngine, SigmaStyleThemeEngine,
    SigmaTrackChangesEngine, SlaPriority, SlideAnimation, SlideMasterPlaceholder,
    SlidePlaceholderKind, SlideTransitionType, SmartCanvasChip, SmartCanvasChipKind,
    SovereignAcademicCitationEngine, SovereignAiDocumentSummarizerEngine,
    SovereignAppsScriptTriggerEngine, SovereignBitrix24OmnichannelCallQueueRouterEngine,
    SovereignBitrix24OmnichannelTelephonyCrmEngine, SovereignBitrix24RpaWorkflowEngine,
    SovereignBitrix24SpeechAiSentimentAnalyzerEngine, SovereignBitrix24TaskKanbanAutomationEngine,
    SovereignChurnPredictionRetentionEngine, SovereignCollaborativeWhiteboardEngine,
    SovereignCpqEngine, SovereignCrmLeadScoringEngine, SovereignCrmPipeline,
    SovereignDataStorytellingEngine, SovereignDigitalContractSignatureEngine,
    SovereignDynamicArrayFormulaSpillEngine, SovereignEmployeeOrgChartEngine,
    SovereignEnterpriseChatSpaceEngine, SovereignEnterpriseIntranetPortalEngine,
    SovereignEnterpriseKnowledgeGraphEngine, SovereignEquipmentMaintenanceEngine,
    SovereignExpenseClaimApprovalEngine, SovereignFinancialValuationEngine,
    SovereignFleetFieldServiceEngine, SovereignFormsSurveyEngine, SovereignGoalSeekSolverEngine,
    SovereignGoogleDocsGeminiSmartWriterEngine, SovereignGoogleDocsPaginatedLayoutEngine,
    SovereignGoogleDocsVersionHistoryEngine, SovereignGoogleKeepLabelTagEngine,
    SovereignGoogleSheetsBigQueryDataConnectorEngine,
    SovereignGoogleSheetsFormulaDependencyTreeEngine,
    SovereignGoogleSheetsPivotTableCalculatedFieldEngine,
    SovereignGoogleSlidesMasterTemplateEngine, SovereignGoogleSlidesTransitionAnimationEngine,
    SovereignGoogleSmartCanvasChipEngine, SovereignHelpdeskSlaEngine,
    SovereignIntegrationWorkflowEngine, SovereignInventoryWarehouseEngine,
    SovereignLandingPageCmsEngine, SovereignLookerAdvancedVisualizationEngine,
    SovereignLookerStudioDataBlendEngine, SovereignLoopPortableComponentEngine,
    SovereignLowCodeBusinessProcessEngine, SovereignLowCodeDatabaseEngine,
    SovereignMacroAutomationSandbox, SovereignManufacturingMrpEngine,
    SovereignMarketingCampaignEngine, SovereignMicrosoft365CoauthoringLockEngine,
    SovereignMicrosoftAccessLowCodeQueryEngine, SovereignMicrosoftExcelWhatIfDataEngine,
    SovereignMicrosoftLoopWorkspaceBlockEngine, SovereignMicrosoftPowerAppsCanvasEngine,
    SovereignMicrosoftTeamsChannelBotWorkflowEngine,
    SovereignMultiCurrencyLedgerConsolidationEngine, SovereignOdooDoubleEntryLedgerEngine,
    SovereignOdooInventorySupplyChainMrpEngine, SovereignOdooMrpBillOfMaterialsTreeEngine,
    SovereignOdooQualityControlInspectionEngine, SovereignOdooRentalSubscriptionLifecycleEngine,
    SovereignOmnichannelCommunicationGateway, SovereignOmnichannelLiveChatEngine,
    SovereignPbxCallCenterEngine, SovereignPivotTableSummaryEngine,
    SovereignPosKitchenDisplayEngine, SovereignPublisherDtpEngine, SovereignQuickNotesEngine,
    SovereignQuoteToCashEngine, SovereignSalesforceCpqQuoteDiscountEngine,
    SovereignSalesforceDataCloudCdpIdentityResolutionEngine,
    SovereignSalesforceEinsteinAnalyticsPipelineEngine,
    SovereignSalesforceEinsteinLeadScoringAiEngine,
    SovereignSalesforceLightningFlowOrchestratorEngine,
    SovereignSalesforceTerritoryQuotaForecastEngine, SovereignServiceCloudKnowledgeEngine,
    SovereignSharedDrivePermissionEngine, SovereignSmartCanvasEngine,
    SovereignSmartDocumentTemplateEngine, SovereignTaxGstAccountingEngine,
    SovereignVidsPresentationEngine, SovereignVisioDiagrammingEngine,
    SovereignWebConferencingEngine, SovereignWebPublisherEngine,
    SovereignWorkgroupActivityStreamEngine, SovereignWorkgroupGanttEngine,
    SovereignWorkspaceAddonExtensionEngine, SovereignZohoAnalyticsCohortAnalysisEngine,
    SovereignZohoAnalyticsPredictiveForecastingEngine, SovereignZohoBooksMultiTaxGroupEngine,
    SovereignZohoDeskTicketSlaEscalationEngine, SovereignZohoUnifiedAppSwitcherEngine,
    SpreadsheetProcessor, SuggestionEdit, TableJoin, TaxRule, TextProcessor, TicketPriority,
    TicketStatus, TypographyRenderer, UnifiedCustomerProfile, UserSessionContext,
    VersionHistoryManager, VidScene, WebLayoutBlock, WhiteboardElement, WhiteboardElementType,
    WorkflowAction, WorkflowRule, WorkflowTrigger, WorkgroupChannelMessage, WorkgroupTask,
};
