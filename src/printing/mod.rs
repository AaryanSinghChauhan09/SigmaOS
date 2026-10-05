//! Printing Module
//!
//! Printer management and print job queue system for SigmaOS.

pub mod printer_manager;
pub mod scanner_manager;

pub use printer_manager::{
    Printer, PrinterManager, PrinterStatistics, PrinterStatus, PrinterType,
    PrintJob, PrintJobStatus,
};
pub use scanner_manager::{
    ScanColorMode, ScanFormat, ScanJob, ScanJobStatus, ScanResolution,
    Scanner, ScannerManager, ScannerStatistics, ScannerType,
};
