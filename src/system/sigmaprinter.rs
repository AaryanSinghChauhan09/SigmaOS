//! SigmaPrinter - Printer Management System
//!
//! Provides comprehensive printer management inspired by Linux CUPS and BSD printing.
//! Supports printer discovery, job management, and printer configuration.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Printer state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrinterState {
    Idle,
    Printing,
    Stopped,
    Error,
    Unknown,
;

/// Print job state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Pending,
    Processing,
    Completed,
    Cancelled,
    Failed,
    Unknown,
;

/// Printer
#[derive(Debug, Clone)]
pub struct Printer {
    pub id: String,
    pub name: String,
    pub state: PrinterState,
    pub location: String,
    pub description: String,
;

impl Printer {
    pub fn new(id: String, name: String, location: String) -> Self {
        Self {
            id,
            name,
            state: PrinterState::Idle,
            location,
            description: String::new(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn print(&mut self) {
        self.state = PrinterState::Printing;
    }

    pub fn stop(&mut self) {
        self.state = PrinterState::Stopped;
    }

    pub fn set_idle(&mut self) {
        self.state = PrinterState::Idle;
    }
;

/// Print job
#[derive(Debug, Clone)]
pub struct PrintJob {
    pub id: String,
    pub printer_id: String,
    pub document: String,
    pub state: JobState,
    pub pages: u32,
;

impl PrintJob {
    pub fn new(id: String, printer_id: String, document: String, pages: u32) -> Self {
        Self {
            id,
            printer_id,
            document,
            state: JobState::Pending,
            pages,
        }
    }

    pub fn process(&mut self) {
        self.state = JobState::Processing;
    }

    pub fn complete(&mut self) {
        self.state = JobState::Completed;
    }

    pub fn cancel(&mut self) {
        self.state = JobState::Cancelled;
    }

    pub fn fail(&mut self) {
        self.state = JobState::Failed;
    }
;

/// Printer manager
pub struct SigmaPrinter {
    printers: BTreeMap<String, Printer>,
    jobs: BTreeMap<String, PrintJob>,
    next_printer_id: u64,
    next_job_id: u64,
;

impl SigmaPrinter {
    pub fn new() -> Self {
        Self {
            printers: BTreeMap::new(),
            jobs: BTreeMap::new(),
            next_printer_id: 1,
            next_job_id: 1,
        }
    }

    pub fn add_printer(&mut self, name: String, location: String) -> String {
        let id = format!("printer-{}", self.next_printer_id);
        self.next_printer_id += 1;
        let printer = Printer::new(id.clone(), name, location);
        self.printers.insert(id.clone(), printer);
        id
    }

    pub fn get_printer(&self, id: &str) -> Option<&Printer> {
        self.printers.get(id)
    }

    pub fn get_printer_mut(&mut self, id: &str) -> Option<&mut Printer> {
        self.printers.get_mut(id)
    }

    pub fn list_printers(&self) -> Vec<&Printer> {
        self.printers.values().collect()
    }

    pub fn remove_printer(&mut self, id: &str) -> bool {
        self.printers.remove(id).is_some()
    }

    pub fn add_job(&mut self, printer_id: String, document: String, pages: u32) -> String {
        let id = format!("job-{}", self.next_job_id);
        self.next_job_id += 1;
        let job = PrintJob::new(id.clone(), printer_id, document, pages);
        self.jobs.insert(id.clone(), job);
        id
    }

    pub fn get_job(&self, id: &str) -> Option<&PrintJob> {
        self.jobs.get(id)
    }

    pub fn get_job_mut(&mut self, id: &str) -> Option<&mut PrintJob> {
        self.jobs.get_mut(id)
    }

    pub fn list_jobs(&self) -> Vec<&PrintJob> {
        self.jobs.values().collect()
    }

    pub fn list_jobs_by_printer(&self, printer_id: &str) -> Vec<&PrintJob> {
        self.jobs
            .values()
            .filter(|j| j.printer_id == printer_id)
            .collect()
    }

    pub fn remove_job(&mut self, id: &str) -> bool {
        self.jobs.remove(id).is_some()
    }

    pub fn printer_count(&self) -> usize {
        self.printers.len()
    }

    pub fn job_count(&self) -> usize {
        self.jobs.len()
    }
;

impl Default for SigmaPrinter {
    fn default() -> Self {
        Self::new()
    }
;

static mut SIGMAPRINTER_INSTANCE: Option<SigmaPrinter> = None;

pub fn init_sigmaprinter() {
    unsafe {
        SIGMAPRINTER_INSTANCE = Some(SigmaPrinter::new());
    }
;

pub fn get_sigmaprinter() -> &'static SigmaPrinter {
    unsafe {
        SIGMAPRINTER_INSTANCE
            .as_ref()
            .expect("SigmaPrinter not initialized")
    }
;

pub fn get_sigmaprinter_mut() -> &'static mut SigmaPrinter {
    unsafe {
        SIGMAPRINTER_INSTANCE
            .as_mut()
            .expect("SigmaPrinter not initialized")
    }
;

pub fn add_printer(name: String, location: String) -> String {
    let manager = get_sigmaprinter_mut();
    manager.add_printer(name, location)
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printer_creation() {
        let printer = Printer::new(
            "printer-1".to_string(),
            "HP LaserJet".to_string(),
            "Office".to_string(),
        );
        assert_eq!(printer.id, "printer-1");
        assert_eq!(printer.name, "HP LaserJet");
        assert_eq!(printer.state, PrinterState::Idle);
    }

    #[test]
    fn test_printer_state() {
        let mut printer = Printer::new(
            "printer-1".to_string(),
            "HP LaserJet".to_string(),
            "Office".to_string(),
        );
        assert_eq!(printer.state, PrinterState::Idle);

        printer.print();
        assert_eq!(printer.state, PrinterState::Printing);

        printer.stop();
        assert_eq!(printer.state, PrinterState::Stopped);
    }

    #[test]
    fn test_print_job_creation() {
        let job = PrintJob::new(
            "job-1".to_string(),
            "printer-1".to_string(),
            "document.pdf".to_string(),
            5,
        );
        assert_eq!(job.id, "job-1");
        assert_eq!(job.state, JobState::Pending);
        assert_eq!(job.pages, 5);
    }

    #[test]
    fn test_print_job_state() {
        let mut job = PrintJob::new(
            "job-1".to_string(),
            "printer-1".to_string(),
            "document.pdf".to_string(),
            5,
        );
        job.process();
        assert_eq!(job.state, JobState::Processing);

        job.complete();
        assert_eq!(job.state, JobState::Completed);
    }

    #[test]
    fn test_printer_manager() {
        let manager = SigmaPrinter::new();
        assert_eq!(manager.printer_count(), 0);
        assert_eq!(manager.job_count(), 0);
    }

    #[test]
    fn test_add_printer() {
        let mut manager = SigmaPrinter::new();
        let id = manager.add_printer("HP LaserJet".to_string(), "Office".to_string());
        assert_eq!(id, "printer-1");
        assert_eq!(manager.printer_count(), 1);
    }

    #[test]
    fn test_add_job() {
        let mut manager = SigmaPrinter::new();
        let printer_id = manager.add_printer("HP LaserJet".to_string(), "Office".to_string());
        let job_id = manager.add_job(printer_id, "document.pdf".to_string(), 5);
        assert_eq!(job_id, "job-1");
        assert_eq!(manager.job_count(), 1);
    }

    #[test]
    fn test_list_jobs_by_printer() {
        let mut manager = SigmaPrinter::new();
        let printer_id = manager.add_printer("HP LaserJet".to_string(), "Office".to_string());
        manager.add_job(printer_id.clone(), "doc1.pdf".to_string(), 5);
        manager.add_job(printer_id, "doc2.pdf".to_string(), 3);

        let jobs = manager.list_jobs_by_printer(&printer_id);
        assert_eq!(jobs.len(), 2);
    }

    #[test]
    fn test_global_instance() {
        init_sigmaprinter();
        let manager = get_sigmaprinter();
        assert_eq!(manager.printer_count(), 0);

        let manager_mut = get_sigmaprinter_mut();
        manager_mut.add_printer("HP LaserJet".to_string(), "Office".to_string());
        assert_eq!(manager.printer_count(), 1);
    }

    #[test]
    fn test_add_printer_convenience() {
        init_sigmaprinter();
        let id = add_printer("HP LaserJet".to_string(), "Office".to_string());
        assert_eq!(id, "printer-1");

        let manager = get_sigmaprinter();
        assert!(manager.get_printer(&id).is_some());
    }
;
