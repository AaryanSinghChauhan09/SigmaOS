//! Printing & Scanning Subsystem
//!
//! Enterprise printer & scanner management inspired by Linux & BSD ecosystems,
//! CUPS, IPP Everywhere driverless discovery, BSD LPD protocol (RFC 1179),
//! PPD compiler, Ghostscript rasterization, Poppler PDF rendering,
//! SANE backends, eSCL/AirScan & WSD, ADF batch scanning, and Pull Request Gateway.

pub mod printer_manager;
pub mod scanner_manager;

pub use printer_manager::{
    BsdLpdAccountingQuotaEngine, BsdLpdProtocolQueueManager, CupsAccountingJobQuotaEngine,
    CupsFilterPipelineEngine, CupsFilterStage, CupsIppAirPrintServerEngine, CupsJobRecoveryPolicy,
    CupsMimeFilterConverterEngine, GhostscriptColorManagementEngine, GhostscriptColorModel,
    GhostscriptDevice, GhostscriptPdfDistillerEngine, GhostscriptRasterizationEngine,
    GhostscriptSecurityFontEngine, IppEverywhereDiscoveryEngine, IppPrinterAttributes,
    LpdPrintAccount, LpdQueueControl, LpdUserQuota, PdfComplianceStandard, PdfFormFieldInfo,
    PopplerAnnotation, PopplerBackend, PopplerDigitalSignatureFormEngine,
    PopplerPdfRenderingEngine, PopplerSearchHit, PopplerTextSearchHighlightEngine,
    PopplerVectorFormExtractionEngine, PpdOption, PpdPostScriptCompilerEngine, PrintJob,
    PrintJobStatus, Printer, PrinterManager, PrinterStatistics, PrinterStatus, PrinterType,
};
pub use scanner_manager::{
    AdfBatchOcrDocumentEngine, AdfBatchScanEngine, AirScanEsclServerEngine,
    AirScanWsdDiscoveryEngine, DiagnosticCheckResult, DiagnosticCheckStatus,
    DiscoveredNetworkScanner, DriverlessAirScanEngine, DriverlessScanProtocol,
    PrintScanDiagnosticsTelemetryEngine, PrintScanPrCategory, PrintScanPullRequestGatewayEngine,
    SaneBackendEngine, SaneDaemonAuthenticationEngine, SaneFrame,
    SaneImageProcessingPipelineEngine, SaneOptionDescriptor, SaneOptionDescriptorEngine,
    SaneOptionValueType, SaneParameters, SaneStatus, SaneUnit, ScanColorMode, ScanFormat, ScanJob,
    ScanJobStatus, ScanResolution, ScannedPageMetadata, Scanner, ScannerManager, ScannerStatistics,
    ScannerType,
};
