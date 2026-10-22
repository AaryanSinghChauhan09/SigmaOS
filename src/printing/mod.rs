//! Printing & Scanning Subsystem
//!
//! Enterprise printer & scanner management inspired by Linux & BSD ecosystems,
//! CUPS, IPP Everywhere driverless discovery, BSD LPD protocol (RFC 1179),
//! PPD compiler, Ghostscript rasterization, Poppler PDF rendering,
//! SANE backends, eSCL/AirScan & WSD, ADF batch scanning, and Pull Request Gateway.

pub mod printer_manager;
pub mod scanner_manager;

pub use printer_manager::{
    BsdLpdAccountingQuotaEngine, BsdLpdProtocolQueueManager, Cups3IppSubscriptionsWebsocketEngine,
    Cups3IppSystemServiceEngine, Cups3ServiceType, Cups3SharedMemorySpoolerEngine,
    CupsAccountingJobQuotaEngine, CupsDriverlessMDNSDiscoveryEngine, CupsFilterPipelineEngine,
    CupsFilterStage, CupsIppAirPrintServerEngine, CupsJobAccountingPreflightEngine,
    CupsJobRecoveryPolicy, CupsLocalIppPrinterApplicationProxyEngine,
    CupsMimeFilterConverterEngine, CupsPreflightReport, GhostscriptCjkFontMapperEngine,
    GhostscriptColorManagementEngine, GhostscriptColorModel, GhostscriptColorProfileIccTransformEngine,
    GhostscriptColorSeparationEngine, GhostscriptDevice, GhostscriptPdfDistillerEngine,
    GhostscriptPostScriptDistillerLevel3Engine, GhostscriptPostscriptFontEmbedderEngine,
    GhostscriptRasterizationEngine, GhostscriptSecurityFontEngine, IppEverywhereDiscoveredPrinter,
    IppEverywhereDiscoveryEngine, IppPrinterAttributes, LpdPrintAccount, LpdQueueControl,
    LpdUserQuota, PdfBox, PdfComplianceStandard, PdfFormFieldInfo, PopplerAnnotation,
    PopplerBackend, PopplerCmsColorProfileEngine, PopplerDigitalSignatureFormEngine,
    PopplerOptionalContentGroupEngine, PopplerPageLayoutMetricsEngine, PopplerPdfAValidationEngine,
    PopplerPdfAValidationReport, PopplerPdfFormField, PopplerPdfFormInteractiveEngine,
    PopplerPdfRenderingEngine, PopplerSearchHit, PopplerTextSearchHighlightEngine,
    PopplerVectorFormExtractionEngine, PpdOption, PpdPostScriptCompilerEngine, PrintJob,
    PrintJobStatus, Printer, PrinterManager, PrinterStatistics, PrinterStatus, PrinterType,
};
pub use scanner_manager::{
    AdfBatchOcrDocumentEngine, AdfBatchScanEngine, AirScanEsclScannerCapabilitiesEngine,
    AirScanEsclServerEngine, AirScanEsclStreamingFeederEngine, AirScanMultiFormatOutputEngine,
    AirScanWsdDiscoveryEngine, DiagnosticCheckResult, DiagnosticCheckStatus,
    DiscoveredNetworkScanner, DriverlessAirScanEngine, DriverlessScanProtocol,
    PrintScanDiagnosticsTelemetryEngine, PrintScanPrCategory, PrintScanPullRequestGatewayEngine,
    SaneBackendConfigurationParserEngine, SaneBackendDeviceConfig, SaneBackendEngine,
    SaneColorCalibrationProfileEngine, SaneDaemonAuthenticationEngine, SaneFrame,
    SaneHardwareLockDeviceBridgeEngine, SaneImageProcessingPipelineEngine,
    SaneNetSessionGovernorEngine, SaneOptionDescriptor, SaneOptionDescriptorEngine,
    SaneOptionValueType, SaneParameters, SaneStatus, SaneUnit, SaneUsbIpRemoteScannerBridgeEngine,
    ScanColorMode, ScanFormat, ScanJob, ScanJobStatus, ScanResolution, ScannedPageMetadata,
    Scanner, ScannerManager, ScannerStatistics, ScannerType, SovereignPrintScanEcosystemOrchestrator,
};
