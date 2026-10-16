// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaPrint - Printer Daemon
//!
//! SigmaPrint provides printer management for SigmaOS, inspired by CUPS from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Printer status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrinterStatus {
    /// Idle
    Idle,
    /// Printing
    Printing,
    /// Stopped
    Stopped,
    /// Error
    Error,
}

impl fmt::Display for PrinterStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrinterStatus::Idle => write!(f, "idle"),
            PrinterStatus::Printing => write!(f, "printing"),
            PrinterStatus::Stopped => write!(f, "stopped"),
            PrinterStatus::Error => write!(f, "error"),
        }
    }
}

/// Print job status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintJobStatus {
    /// Pending
    Pending,
    /// Processing
    Processing,
    /// Completed
    Completed,
    /// Failed
    Failed,
    /// Cancelled
    Cancelled,
}

impl fmt::Display for PrintJobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrintJobStatus::Pending => write!(f, "pending"),
            PrintJobStatus::Processing => write!(f, "processing"),
            PrintJobStatus::Completed => write!(f, "completed"),
            PrintJobStatus::Failed => write!(f, "failed"),
            PrintJobStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Printer
#[derive(Debug, Clone)]
pub struct Printer {
    /// Printer ID
    pub printer_id: String,
    /// Printer name
    pub name: String,
    /// Printer location
    pub location: String,
    /// Printer status
    pub status: PrinterStatus,
    /// Default printer
    pub is_default: bool,
}

impl Printer {
    /// Create a new printer
    pub fn new(printer_id: String, name: String, location: String) -> Self {
        Self {
            printer_id,
            name,
            location,
            status: PrinterStatus::Idle,
            is_default: false,
        }
    }

    /// Set status
    pub fn with_status(mut self, status: PrinterStatus) -> Self {
        self.status = status;
        self
    }

    /// Set as default
    pub fn with_default(mut self, is_default: bool) -> Self {
        self.is_default = is_default;
        self
    }
}

/// Print job
#[derive(Debug, Clone)]
pub struct PrintJob {
    /// Job ID
    pub job_id: String,
    /// Printer ID
    pub printer_id: String,
    /// Document name
    pub document_name: String,
    /// File path
    pub file_path: String,
    /// Job status
    pub status: PrintJobStatus,
    /// Pages
    pub pages: u32,
    /// Timestamp
    pub timestamp: u64,
}

impl PrintJob {
    /// Create a new print job
    pub fn new(
        job_id: String,
        printer_id: String,
        document_name: String,
        file_path: String,
    ) -> Self {
        Self {
            job_id,
            printer_id,
            document_name,
            file_path,
            status: PrintJobStatus::Pending,
            pages: 1,
            timestamp: 0,
        }
    }

    /// Set pages
    pub fn with_pages(mut self, pages: u32) -> Self {
        self.pages = pages;
        self
    }

    /// Set status
    pub fn with_status(mut self, status: PrintJobStatus) -> Self {
        self.status = status;
        self
    }
}

/// SigmaPrint - Printer Daemon
pub struct SigmaPrint {
    /// Printers (printer_id -> printer)
    printers: BTreeMap<String, Printer>,
    /// Print jobs (job_id -> job)
    jobs: BTreeMap<String, PrintJob>,
    /// Running state
    running: bool,
    /// Job counter
    job_counter: u32,
}

impl SigmaPrint {
    /// Create a new SigmaPrint instance
    pub fn new() -> Self {
        Self {
            printers: BTreeMap::new(),
            jobs: BTreeMap::new(),
            running: false,
            job_counter: 0,
        }
    }

    /// Start the print daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the print daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a printer
    pub fn add_printer(&mut self, printer: Printer) -> Result<(), String> {
        if self.printers.contains_key(&printer.printer_id) {
            return Err(format!("Printer '{}' already exists", printer.printer_id));
        }

        // If this is the default, unset other defaults
        if printer.is_default {
            for p in self.printers.values_mut() {
                p.is_default = false;
            }
        }

        self.printers.insert(printer.printer_id.clone(), printer);
        Ok(())
    }

    /// Remove a printer
    pub fn remove_printer(&mut self, printer_id: &str) -> Result<(), String> {
        if self.printers.remove(printer_id).is_none() {
            return Err(format!("Printer '{}' not found", printer_id));
        }
        Ok(())
    }

    /// Get a printer
    pub fn get_printer(&self, printer_id: &str) -> Option<&Printer> {
        self.printers.get(printer_id)
    }

    /// Get default printer
    pub fn get_default_printer(&self) -> Option<&Printer> {
        self.printers.values().find(|p| p.is_default)
    }

    /// Set default printer
    pub fn set_default_printer(&mut self, printer_id: &str) -> Result<(), String> {
        if !self.printers.contains_key(printer_id) {
            return Err(format!("Printer '{}' not found", printer_id));
        }

        for p in self.printers.values_mut() {
            p.is_default = false;
        }

        if let Some(printer) = self.printers.get_mut(printer_id) {
            printer.is_default = true;
        }

        Ok(())
    }

    /// List all printers
    pub fn list_printers(&self) -> Vec<&Printer> {
        self.printers.values().collect()
    }

    /// Add a print job
    pub fn add_job(&mut self, job: PrintJob) -> Result<(), String> {
        if !self.running {
            return Err("SigmaPrint not running".to_string());
        }

        if !self.printers.contains_key(&job.printer_id) {
            return Err(format!("Printer '{}' not found", job.printer_id));
        }

        self.jobs.insert(job.job_id.clone(), job);
        self.job_counter += 1;
        Ok(())
    }

    /// Cancel a job
    pub fn cancel_job(&mut self, job_id: &str) -> Result<(), String> {
        match self.jobs.get_mut(job_id) {
            Some(job) => {
                if job.status == PrintJobStatus::Completed {
                    return Err("Cannot cancel completed job".to_string());
                }
                job.status = PrintJobStatus::Cancelled;
                Ok(())
            }
            None => Err(format!("Job '{}' not found", job_id)),
        }
    }

    /// Get a job
    pub fn get_job(&self, job_id: &str) -> Option<&PrintJob> {
        self.jobs.get(job_id)
    }

    /// List all jobs
    pub fn list_jobs(&self) -> Vec<&PrintJob> {
        self.jobs.values().collect()
    }

    /// List jobs by printer
    pub fn list_jobs_by_printer(&self, printer_id: &str) -> Vec<&PrintJob> {
        self.jobs
            .values()
            .filter(|j| j.printer_id == printer_id)
            .collect()
    }

    /// List pending jobs
    pub fn list_pending(&self) -> Vec<&PrintJob> {
        self.jobs
            .values()
            .filter(|j| j.status == PrintJobStatus::Pending)
            .collect()
    }

    /// Get printer count
    pub fn printer_count(&self) -> usize {
        self.printers.len()
    }

    /// Get job count
    pub fn job_count(&self) -> usize {
        self.jobs.len()
    }

    /// Get print statistics
    pub fn print_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert(
            "total_printers".to_string(),
            self.printer_count().to_string(),
        );
        stats.insert("total_jobs".to_string(), self.job_count().to_string());
        stats.insert(
            "pending_jobs".to_string(),
            self.list_pending().len().to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaPrint {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaPrint instance (placeholder for actual global state)
static mut SIGMAPRINT_INSTANCE: Option<SigmaPrint> = None;

/// Initialize the global SigmaPrint instance
pub fn init_sigmaprint() {
    unsafe {
        SIGMAPRINT_INSTANCE = Some(SigmaPrint::new());
    }
}

/// Get the global SigmaPrint instance
pub fn get_sigmaprint() -> Option<&'static mut SigmaPrint> {
    unsafe { SIGMAPRINT_INSTANCE.as_mut() }
}

/// Convenience function to add a print job
pub fn add_print_job(job: PrintJob) -> Result<(), String> {
    if let Some(print) = get_sigmaprint() {
        print.add_job(job)
    } else {
        Err("SigmaPrint not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printer_status_display() {
        assert_eq!(PrinterStatus::Idle.to_string(), "idle");
        assert_eq!(PrinterStatus::Printing.to_string(), "printing");
    }

    #[test]
    fn test_print_job_status_display() {
        assert_eq!(PrintJobStatus::Pending.to_string(), "pending");
        assert_eq!(PrintJobStatus::Completed.to_string(), "completed");
    }

    #[test]
    fn test_printer() {
        let printer = Printer::new(
            "printer1".to_string(),
            "HP LaserJet".to_string(),
            "Office".to_string(),
        )
        .with_status(PrinterStatus::Printing)
        .with_default(true);

        assert_eq!(printer.printer_id, "printer1");
        assert_eq!(printer.status, PrinterStatus::Printing);
        assert!(printer.is_default);
    }

    #[test]
    fn test_print_job() {
        let job = PrintJob::new(
            "job1".to_string(),
            "printer1".to_string(),
            "Document.pdf".to_string(),
            "/tmp/document.pdf".to_string(),
        )
        .with_pages(5)
        .with_status(PrintJobStatus::Completed);

        assert_eq!(job.job_id, "job1");
        assert_eq!(job.pages, 5);
        assert_eq!(job.status, PrintJobStatus::Completed);
    }

    #[test]
    fn test_sigmaprint() {
        let mut print = SigmaPrint::new();
        assert!(!print.is_running());
        print.start();
        assert!(print.is_running());
        print.stop();
        assert!(!print.is_running());
    }

    #[test]
    fn test_sigmaprint_add_printer() {
        let mut print = SigmaPrint::new();

        let printer = Printer::new(
            "printer1".to_string(),
            "HP LaserJet".to_string(),
            "Office".to_string(),
        );

        assert!(print.add_printer(printer).is_ok());
        assert_eq!(print.printer_count(), 1);
    }

    #[test]
    fn test_sigmaprint_set_default() {
        let mut print = SigmaPrint::new();

        let printer1 = Printer::new(
            "printer1".to_string(),
            "HP LaserJet".to_string(),
            "Office".to_string(),
        );

        let printer2 = Printer::new(
            "printer2".to_string(),
            "Canon Pixma".to_string(),
            "Home".to_string(),
        );

        print.add_printer(printer1).unwrap();
        print.add_printer(printer2).unwrap();

        assert!(print.set_default_printer("printer2").is_ok());

        let default = print.get_default_printer().unwrap();
        assert_eq!(default.printer_id, "printer2");
    }

    #[test]
    fn test_sigmaprint_add_job() {
        let mut print = SigmaPrint::new();
        print.start();

        let printer = Printer::new(
            "printer1".to_string(),
            "HP LaserJet".to_string(),
            "Office".to_string(),
        );

        print.add_printer(printer).unwrap();

        let job = PrintJob::new(
            "job1".to_string(),
            "printer1".to_string(),
            "Document.pdf".to_string(),
            "/tmp/document.pdf".to_string(),
        );

        assert!(print.add_job(job).is_ok());
        assert_eq!(print.job_count(), 1);
    }

    #[test]
    fn test_sigmaprint_cancel_job() {
        let mut print = SigmaPrint::new();
        print.start();

        let printer = Printer::new(
            "printer1".to_string(),
            "HP LaserJet".to_string(),
            "Office".to_string(),
        );

        print.add_printer(printer).unwrap();

        let job = PrintJob::new(
            "job1".to_string(),
            "printer1".to_string(),
            "Document.pdf".to_string(),
            "/tmp/document.pdf".to_string(),
        );

        print.add_job(job).unwrap();
        assert!(print.cancel_job("job1").is_ok());

        let job = print.get_job("job1").unwrap();
        assert_eq!(job.status, PrintJobStatus::Cancelled);
    }

    #[test]
    fn test_sigmaprint_list_by_printer() {
        let mut print = SigmaPrint::new();
        print.start();

        let printer = Printer::new(
            "printer1".to_string(),
            "HP LaserJet".to_string(),
            "Office".to_string(),
        );

        print.add_printer(printer).unwrap();

        let job = PrintJob::new(
            "job1".to_string(),
            "printer1".to_string(),
            "Document.pdf".to_string(),
            "/tmp/document.pdf".to_string(),
        );

        print.add_job(job).unwrap();

        let jobs = print.list_jobs_by_printer("printer1");
        assert_eq!(jobs.len(), 1);
    }

    #[test]
    fn test_sigmaprint_stats() {
        let mut print = SigmaPrint::new();
        print.start();

        let printer = Printer::new(
            "printer1".to_string(),
            "HP LaserJet".to_string(),
            "Office".to_string(),
        );

        print.add_printer(printer).unwrap();

        let job = PrintJob::new(
            "job1".to_string(),
            "printer1".to_string(),
            "Document.pdf".to_string(),
            "/tmp/document.pdf".to_string(),
        );

        print.add_job(job).unwrap();

        let stats = print.print_stats();
        assert_eq!(stats.get("total_printers"), Some(&"1".to_string()));
        assert_eq!(stats.get("total_jobs"), Some(&"1".to_string()));
    }

    #[test]
    fn test_add_print_job_convenience() {
        init_sigmaprint();

        let mut print = get_sigmaprint().unwrap();
        print.start();

        let printer = Printer::new(
            "printer1".to_string(),
            "HP LaserJet".to_string(),
            "Office".to_string(),
        );

        print.add_printer(printer).unwrap();

        let job = PrintJob::new(
            "job1".to_string(),
            "printer1".to_string(),
            "Document.pdf".to_string(),
            "/tmp/document.pdf".to_string(),
        );

        let result = add_print_job(job);
        assert!(result.is_ok());
    }
}
