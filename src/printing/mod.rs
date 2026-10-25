//! Printing & Scanning Subsystem
//!
//! Enterprise printer & scanner management inspired by Linux & BSD ecosystems,
//! CUPS, IPP Everywhere driverless discovery, BSD LPD protocol (RFC 1179),
//! PPD compiler, Ghostscript rasterization, Poppler PDF rendering,
//! SANE backends, eSCL/AirScan & WSD, ADF batch scanning, and Pull Request Gateway.

pub mod printer_manager;
pub mod scanner_manager;
pub mod sovereign_printing_scanning_v72_pr;
pub mod sovereign_printing_scanning_v74_pr;

pub use printer_manager::{
    BsdLpdAccountingQuotaEngine, BsdLpdProtocolQueueManager, Cups3IppSystemServiceEngine,
    Cups3IppUsbBridgeEngine, Cups3ServiceType, Cups3SharedMemorySpoolerEngine,
    CupsAccountingJobQuotaEngine, CupsFilterPipelineEngine, CupsFilterStage,
    CupsFilterV2PipelineEngine, CupsIppAirPrintServerEngine, CupsJobRecoveryPolicy,
    CupsKerberosGssapiAuthEngine, CupsLocalIppPrinterApplicationProxyEngine,
    CupsMimeFilterConverterEngine, GhostscriptCjkFontMapperEngine,
    GhostscriptColorManagementEngine, GhostscriptColorModel, GhostscriptColorSeparationEngine,
    GhostscriptDevice, GhostscriptPdfA3ValidationEngine, GhostscriptPdfDistillerEngine,
    GhostscriptPostscriptFontEmbedderEngine, GhostscriptRasterizationEngine,
    GhostscriptSecurityFontEngine, IppEverywhereDiscoveryEngine, IppPrinterAttributes,
    LpdPrintAccount, LpdQueueControl, LpdUserQuota, PdfBox, PdfComplianceStandard,
    PdfFormFieldInfo, PopplerAnnotation, PopplerBackend, PopplerCmsColorProfileEngine,
    PopplerDigitalSignatureFormEngine, PopplerOptionalContentGroupEngine,
    PopplerPageLayoutMetricsEngine, PopplerPdfRenderingEngine, PopplerSearchHit,
    PopplerTextSearchHighlightEngine, PopplerVectorFormExtractionEngine, PopplerXfaFormsEngine,
    PpdOption, PpdPostScriptCompilerEngine, PrintJob, PrintJobStatus, Printer, PrinterManager,
    PrinterStatistics, PrinterStatus, PrinterType,
};
pub use scanner_manager::{
    AdfBatchOcrDocumentEngine, AdfBatchScanEngine, AirScanEsclServerEngine,
    AirScanEsclStreamingFeederEngine, AirScanMultiFormatOutputEngine,
    AirScanWsdCapabilityParserEngine, AirScanWsdDiscoveryEngine, DiagnosticCheckResult,
    DiagnosticCheckStatus, DiscoveredNetworkScanner, DriverlessAirScanEngine,
    DriverlessScanOcrPdfGeneratorEngine, DriverlessScanProtocol,
    PrintScanDiagnosticsTelemetryEngine, PrintScanPrCategory, PrintScanPullRequestGatewayEngine,
    SaneBackendEngine, SaneDaemonAuthenticationEngine, SaneDeskewAutoCropPipelineEngine, SaneFrame,
    SaneGammaTableCalibrationEngine, SaneHardwareLockDeviceBridgeEngine,
    SaneImageProcessingPipelineEngine, SaneNetSessionGovernorEngine, SaneOptionDescriptor,
    SaneOptionDescriptorEngine, SaneOptionValueType, SaneParameters, SaneStatus, SaneUnit,
    SaneUsbIpRemoteScannerBridgeEngine, ScanColorMode, ScanFormat, ScanJob, ScanJobStatus,
    ScanResolution, ScannedPageMetadata, Scanner, ScannerManager, ScannerStatistics, ScannerType,
    SovereignPrintScanEcosystemOrchestrator,
};
pub use sovereign_printing_scanning_v72_pr::{
    SovereignAdfBatchOcrEngine, SovereignAirScanWsdEngine, SovereignBsdLpdAccountingEngine,
    SovereignCups3DriverlessAppGatewayEngine, SovereignCups3ServiceMode, SovereignCupsFilterNode,
    SovereignCupsFilterV2GraphEngine, SovereignGhostscriptDistillerEngine,
    SovereignGhostscriptIccTicEngine, SovereignPopplerAcroFormEngine,
    SovereignPopplerPdfUaSearchEngine, SovereignPopplerSearchHit,
    SovereignPrintScanV72PrGatewayEngine, SovereignSaneImageProcessingEngine,
    SovereignSaneNetMultiplexerEngine,
};
pub use sovereign_printing_scanning_v74_pr::{
    SovereignAdfBatchOcrEngineV74, SovereignAirScanWsdEngineV74,
    SovereignBsdLpdAccountingEngineV74, SovereignCups3DriverlessAppGatewayEngineV74,
    SovereignCups3ServiceModeV74, SovereignCupsFilterNodeV74,
    SovereignCupsFilterV2GraphEngineV74, SovereignGhostscriptDistillerEngineV74,
    SovereignGhostscriptIccTicEngineV74, SovereignPopplerAcroFormEngineV74,
    SovereignPopplerPdfUaSearchEngineV74, SovereignPopplerSearchHitV74,
    SovereignPrintScanV74PrGatewayEngine, SovereignSaneImageProcessingEngineV74,
    SovereignSaneNetMultiplexerEngineV74,
};
