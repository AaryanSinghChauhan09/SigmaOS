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
pub mod notes;
pub mod pdf;
pub mod reminders_advanced;
pub mod screen_recorder;
pub mod screenshot;
pub mod sigma_office;
pub mod sigmadev;
pub mod sovereign_application_replacements_pr_suite;
pub mod sovereign_application_replacements_v2_pr_suite;
pub mod sovereign_application_replacements_v3_pr_suite;
pub mod sovereign_application_replacements_v4_pr_suite;
pub mod sovereign_apps;
pub mod subtitle_editor;
pub mod system_settings;
pub mod tasks;
pub mod terminal;
pub mod tmux;
pub mod utility_suite;

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
pub use sovereign_application_replacements_pr_suite::{
    BrowserSandboxPolicy, ChatProtocolDriver, CodecDecodersAvailable, CompressionFormatKind,
    ContainerIsolationLevel, DocumentFileType, EmailSecurityProtocol, EncryptionVaultAlgorithm,
    FileManagerLayout, LayerBlendMode, OfficeSuiteComponent, P2pEncryptionMode, PartitionTableKind,
    SovereignAppPrProposalManifest, SovereignApplicationReplacementsPrGateway,
    SovereignArchiveCompressionPrEngine, SovereignBittorrentP2pClientPrEngine,
    SovereignBrowserEnginePrEngine, SovereignDiskPartitionStoragePrEngine,
    SovereignDocumentViewerPrEngine, SovereignEditorEmulationMode,
    SovereignFileManagerMatrixPrEngine, SovereignLibreOfficeSuitePrEngine,
    SovereignMediaPlayerAudioStudioPrEngine, SovereignPasswordVaultPassPrEngine,
    SovereignRasterGraphicsPrEngine, SovereignSystemMonitorProcessPrEngine,
    SovereignTerminalMultiplexerPrEngine, SovereignTextEditorsPrEngine,
    SovereignThunderbirdEmailPrEngine, TerminalGraphicsProtocol, VfsMountProtocol,
};
pub use sovereign_application_replacements_v2_pr_suite::{
    BrowserV2SecurityFeature, SovereignAppV2PrProposalManifest,
    SovereignApplicationReplacementsV2PrGateway, SovereignArchiveCompressorV2PrEngine,
    SovereignBrowserEngineV2PrEngine, SovereignDiskPartitionManagerV2PrEngine,
    SovereignDocumentViewerV2PrEngine, SovereignEmailClientV2PrEngine,
    SovereignFileManagerV2PrEngine, SovereignMatrixIrcMessengerV2PrEngine,
    SovereignMediaPlayerStudioV2PrEngine, SovereignOfficeSuiteV2PrEngine,
    SovereignP2pTorrentClientV2PrEngine, SovereignPasswordVaultV2PrEngine,
    SovereignRasterGraphicsV2PrEngine, SovereignSystemMonitorV2PrEngine,
    SovereignTerminalMultiplexerV2PrEngine, SovereignTextEditorV2PrEngine,
};
pub use sovereign_application_replacements_v3_pr_suite::{
    BrowserV3SecurityFeature, SovereignAppV3PrProposalManifest,
    SovereignApplicationReplacementsV3PrGateway, SovereignArchiveCompressorV3PrEngine,
    SovereignBrowserEngineV3PrEngine, SovereignDiskPartitionManagerV3PrEngine,
    SovereignDocumentViewerV3PrEngine, SovereignEmailClientV3PrEngine,
    SovereignFileManagerV3PrEngine, SovereignMatrixIrcMessengerV3PrEngine,
    SovereignMediaPlayerStudioV3PrEngine, SovereignOfficeSuiteV3PrEngine,
    SovereignP2pTorrentClientV3PrEngine, SovereignPasswordVaultV3PrEngine,
    SovereignRasterGraphicsV3PrEngine, SovereignSystemMonitorV3PrEngine,
    SovereignTerminalMultiplexerV3PrEngine, SovereignTextEditorV3PrEngine,
};
pub use sovereign_application_replacements_v4_pr_suite::{
    BrowserV4SecurityFeature, SovereignAppV4PrProposalManifest,
    SovereignApplicationReplacementsV4PrGateway, SovereignArchiveCompressorV4PrEngine,
    SovereignBrowserEngineV4PrEngine, SovereignDiskPartitionManagerV4PrEngine,
    SovereignDocumentViewerV4PrEngine, SovereignEmailClientV4PrEngine,
    SovereignFileManagerV4PrEngine, SovereignMatrixIrcMessengerV4PrEngine,
    SovereignMediaPlayerStudioV4PrEngine, SovereignOfficeSuiteV4PrEngine,
    SovereignP2pTorrentClientV4PrEngine, SovereignPasswordVaultV4PrEngine,
    SovereignRasterGraphicsV4PrEngine, SovereignSystemMonitorV4PrEngine,
    SovereignTerminalMultiplexerV4PrEngine, SovereignTextEditorV4PrEngine,
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
    BatchSerialItem, CalculationDagResult, CallLogEntry, CellValue, ChurnRiskLevel, CitationItem,
    CitationStyle, CoauthorOpKind, CoauthorOperation, CommunicationChannel, ConditionalFormatType,
    CpqProductBundleItem, CustomConditionalFormatRule, CustomerMetrics, DataStoryStep,
    DbColumnType, DbRow, DbTable, DbTableColumn, DealEscalationLevel, DependencyNode,
    DiagramConnector, DiagramNode, DiagramNodeType, DigitalSignatureStamp, DocumentBranch,
    DocumentNode, DocumentType, DripStep, DtpPageLayout, DynamicArraySpillResult,
    EinsteinLeadScoreResult, EnterpriseDeal, EnterpriseInvoice, EquipmentMaintenanceSchedule,
    EvaluatedCellFormatting, ExecutiveKpiItem, ExpenseApprovalStatus, ExpenseClaimItem,
    FieldServiceJob, FormQuestion, FormResponse, FxAccountBalance, FxRevaluationResult,
    HeaderFooterSection, HelpdeskTicket, InlineDocComment, IntercompanyConsolidationResult,
    IntercompanyTransfer, InventoryItem, InventoryMovementResult, IvrCallFlowResult,
    JournalEntryLine, KanbanAutomationResult, KanbanTaskItem, KdsOrderTicket, KdsTicketStatus,
    KnowledgeArticle, KnowledgeGraphEdge, KnowledgeGraphNode, KnowledgeGraphNodeType,
    KpiMetricStatus, LeadAssignmentRule, LeadBehaviorEvent, LeadScoreRecord, LeadScoringInput,
    LiveCoAuthoringManager, LookerChartWidget, LookerFilterControl, LookerGaugeWidget,
    LookerMetricCard, LoopComponent, MacroExecutor, MaintenanceReliabilityResult,
    ModelScenarioSpec, MultiEntityJournalTransaction, OdfDocumentKind, OeeCalculationResult,
    OmnichannelMessage, OmnichannelSlaRule, ParagraphStyle, PivotAggregateFunc, PivotSummaryResult,
    PosOrderItem, PowerBiCalculatedTableResult, PresentationProcessor, QuestionType, QuickNoteItem,
    RenderedPageHeaderFooter, RenewalQuote, ResourceAllocation, ScenarioComparisonResult,
    ScriptExecutionRecord, ScriptTriggerEvent, SectionHeaderFooterConfig, SharedDriveFileLock,
    SharedDriveMember, SharedDriveRole, SheetDataValidationRule, SigmaFormulaParserEngine,
    SigmaOdfPackageEngine, SigmaOffice, SigmaSlideDetails, SigmaSpellCheckerEngine,
    SigmaStyleThemeEngine, SigmaTrackChangesEngine, SlaStatus, SmartCanvasChip,
    SmartCanvasChipKind, SourceTableColumnSpec, SovereignAcademicCitationEngine,
    SovereignAiDocumentSummarizerEngine, SovereignAppsScriptTriggerEngine,
    SovereignBitrix24CrmVoiceIvrCallFlowEngine,
    SovereignBitrix24OmnichannelSlaEscalationRouterEngine,
    SovereignBitrix24OmnichannelTaskKanbanAutomationEngine,
    SovereignBitrix24OmnichannelTelephonyCrmEngine, SovereignChurnPredictionRetentionEngine,
    SovereignCollaborativeWhiteboardEngine, SovereignCpqEngine, SovereignCrmLeadScoringEngine,
    SovereignCrmPipeline, SovereignDataStorytellingEngine, SovereignDigitalContractSignatureEngine,
    SovereignDynamicArrayFormulaSpillEngine, SovereignEmployeeOrgChartEngine,
    SovereignEnterpriseChatSpaceEngine, SovereignEnterpriseIntranetPortalEngine,
    SovereignEnterpriseKnowledgeGraphEngine, SovereignEnterpriseResourceAllocationEngine,
    SovereignEquipmentMaintenanceEngine, SovereignExecutiveKpiScorecardEngine,
    SovereignExpenseClaimApprovalEngine, SovereignFinancialValuationEngine,
    SovereignFleetFieldServiceEngine, SovereignFormsSurveyEngine, SovereignGoalSeekSolverEngine,
    SovereignGoogleDocsPaginatedHeaderFooterEngine,
    SovereignGoogleDocsWatermarkAndHeaderFooterLayoutEngine,
    SovereignGoogleSheetsCustomConditionalFormattingRuleEngine,
    SovereignGoogleSheetsDataValidationListFormulaEngine,
    SovereignGoogleSheetsFormulaDependencyTreeEngine, SovereignGoogleSmartCanvasChipEngine,
    SovereignGoogleWorkspaceGeminiSmartSummaryEngine, SovereignHelpdeskSlaEngine,
    SovereignIntegrationWorkflowEngine, SovereignInventoryWarehouseEngine,
    SovereignLandingPageCmsEngine, SovereignLookerAdvancedVisualizationEngine,
    SovereignLoopPortableComponentEngine, SovereignLowCodeBusinessProcessEngine,
    SovereignLowCodeDatabaseEngine, SovereignMacroAutomationSandbox,
    SovereignManufacturingMrpEngine, SovereignMarketingCampaignEngine,
    SovereignMicrosoft365CoauthoringOperationalTransformEngine,
    SovereignMicrosoftExcelSolverScenarioManagerEngine,
    SovereignMicrosoftPowerBiCalculatedTableEngine,
    SovereignOdooInventoryBatchSerialTrackingEngine, SovereignOdooInventorySupplyChainMrpEngine,
    SovereignOdooMrpWorkCenterOeeEfficiencyEngine,
    SovereignOdooPreventiveMaintenanceScheduleEngine, SovereignOmnichannelCommunicationGateway,
    SovereignOmnichannelLiveChatEngine, SovereignPbxCallCenterEngine,
    SovereignPivotTableSummaryEngine, SovereignPosKitchenDisplayEngine,
    SovereignPublisherDtpEngine, SovereignQuickNotesEngine, SovereignQuoteToCashEngine,
    SovereignSalesforceCpqSubscriptionRenewalDiscountEngine,
    SovereignSalesforceCpqUsageBasedBillingEngine,
    SovereignSalesforceEinsteinAnalyticsPipelineEngine,
    SovereignSalesforceEinsteinLeadScoringEngine, SovereignServiceCloudKnowledgeEngine,
    SovereignSharedDrivePermissionEngine, SovereignSmartCanvasEngine,
    SovereignSmartDocumentTemplateEngine, SovereignTaxGstAccountingEngine,
    SovereignVidsPresentationEngine, SovereignVisioDiagrammingEngine,
    SovereignWebConferencingEngine, SovereignWebPublisherEngine,
    SovereignWorkgroupActivityStreamEngine, SovereignWorkgroupGanttEngine,
    SovereignWorkspaceAddonExtensionEngine, SovereignZohoBooksMultiCurrencyFxRevaluationEngine,
    SovereignZohoBooksMultiEntityIntercompanyEngine, SpreadsheetProcessor, SubscriptionContract,
    SuggestionEdit, TextProcessor, TicketPriority, TicketStatus, TypographyRenderer,
    UsageBillingResult, UsageConsumptionTier, ValidationRuleKind, VersionHistoryManager, VidScene,
    WatermarkSpec, WebLayoutBlock, WhiteboardElement, WhiteboardElementType, WorkCenterOeeMetrics,
    WorkflowAction, WorkflowRule, WorkflowTrigger, WorkgroupChannelMessage, WorkgroupTask,
    WorkspaceSmartSummary,
};
