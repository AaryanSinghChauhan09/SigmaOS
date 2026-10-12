//! Printing & Scanning Subsystem
//!
//! Enterprise printer & scanner management inspired by Linux & BSD ecosystems,
//! CUPS, IPP Everywhere driverless discovery, BSD LPD protocol (RFC 1179),
//! PPD compiler, Ghostscript rasterization, Poppler PDF rendering,
//! SANE backends, eSCL/AirScan & WSD, ADF batch scanning, and Pull Request Gateway.

pub mod printer_manager;
pub mod scanner_manager;

pub use printer_manager::{
    BsdLpdProtocolQueueManager, GhostscriptColorModel, GhostscriptDevice,
    GhostscriptRasterizationEngine, IppEverywhereDiscoveryEngine, IppPrinterAttributes,
    LpdQueueControl, PopplerAnnotation, PopplerBackend, PopplerPdfRenderingEngine,
    PpdOption, PpdPostScriptCompilerEngine, PrintJob, PrintJobStatus, Printer, PrinterManager,
    PrinterStatistics, PrinterStatus, PrinterType,
};
pub use scanner_manager::{
    AdfBatchScanEngine, DriverlessAirScanEngine, DriverlessScanProtocol,
    PrintScanPullRequestGatewayEngine, PrintScanPrCategory, SaneBackendEngine,
    SaneFrame, SaneParameters, SaneStatus, ScanColorMode, ScanFormat, ScanJob,
    ScanJobStatus, ScanResolution, Scanner, ScannerManager, ScannerStatistics,
    ScannerType,
};
