//! SigmaPrint - Print Management System
//!
//! Provides comprehensive print management inspired by Print Craft.
//! Supports print jobs, printer discovery, queue management, and printing options.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Paper size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaperSize {
    A4,
    Letter,
    Legal,
    A3,
    A5,
    Tabloid,
;

/// Print quality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintQuality {
    Draft,
    Normal,
    High,
    Photo,
;

/// Print orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintOrientation {
    Portrait,
    Landscape,
;

/// Color mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    Grayscale,
    Color,
;

/// Print options
#[derive(Debug, Clone)]
pub struct PrintOptions {
    pub paper_size: PaperSize,
    pub print_quality: PrintQuality,
    pub orientation: PrintOrientation,
    pub color_mode: ColorMode,
    pub copies: u32,
    pub duplex: bool,
;

impl PrintOptions {
    pub fn new() -> Self {
        Self {
            paper_size: PaperSize::A4,
            print_quality: PrintQuality::Normal,
            orientation: PrintOrientation::Portrait,
            color_mode: ColorMode::Color,
            copies: 1,
            duplex: false,
        }
    }

    pub fn set_paper_size(&mut self, size: PaperSize) {
        self.paper_size = size;
    }

    pub fn set_print_quality(&mut self, quality: PrintQuality) {
        self.print_quality = quality;
    }

    pub fn set_orientation(&mut self, orientation: PrintOrientation) {
        self.orientation = orientation;
    }

    pub fn set_color_mode(&mut self, mode: ColorMode) {
        self.color_mode = mode;
    }

    pub fn set_copies(&mut self, copies: u32) {
        self.copies = copies;
    }

    pub fn set_duplex(&mut self, duplex: bool) {
        self.duplex = duplex;
    }
;

/// Print job
#[derive(Debug, Clone)]
pub struct PrintJob {
    pub id: String,
    pub name: String,
    pub file_path: String,
    pub printer_id: String,
    pub options: PrintOptions,
    pub status: String,
    pub pages: u32,
;

impl PrintJob {
    pub fn new(id: String, name: String, file_path: String, printer_id: String, options: PrintOptions) -> Self {
        Self {
            id,
            name,
            file_path,
            printer_id,
            options,
            status: "Pending".to_string(),
            pages: 0,
        }
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn set_pages(&mut self, pages: u32) {
        self.pages = pages;
    }
;

/// SigmaPrint - Print management system
#[derive(Debug)]
pub struct SigmaPrint {
    print_jobs: BTreeMap<String, PrintJob>,
    print_queue: Vec<String>, // job IDs in order
;

impl SigmaPrint {
    pub fn new() -> Self {
        Self {
            print_jobs: BTreeMap::new(),
            print_queue: Vec::new(),
        }
    }

    pub fn add_print_job(&mut self, job: PrintJob) {
        self.print_queue.push(job.id.clone());
        self.print_jobs.insert(job.id.clone(), job);
    }

    pub fn remove_print_job(&mut self, id: &str) -> bool {
        if self.print_jobs.remove(id).is_some() {
            self.print_queue.retain(|x| x != id);
            true
        } else {
            false
        }
    }

    pub fn get_print_job(&self, id: &str) -> Option<&PrintJob> {
        self.print_jobs.get(id)
    }

    pub fn get_print_job_mut(&mut self, id: &str) -> Option<&mut PrintJob> {
        self.print_jobs.get_mut(id)
    }

    pub fn get_next_job(&self) -> Option<&PrintJob> {
        self.print_queue.first()
            .and_then(|id| self.print_jobs.get(id))
    }

    pub fn complete_job(&mut self, id: &str) -> bool {
        if let Some(job) = self.print_jobs.get_mut(id) {
            job.status = "Completed".to_string();
            self.print_queue.retain(|x| x != id);
            true
        } else {
            false
        }
    }

    pub fn fail_job(&mut self, id: &str) -> bool {
        if let Some(job) = self.print_jobs.get_mut(id) {
            job.status = "Failed".to_string();
            self.print_queue.retain(|x| x != id);
            true
        } else {
            false
        }
    }

    pub fn list_print_jobs(&self) -> Vec<&PrintJob> {
        self.print_jobs.values().collect()
    }

    pub fn list_queue(&self) -> Vec<&PrintJob> {
        self.print_queue.iter()
            .filter_map(|id| self.print_jobs.get(id))
            .collect()
    }

    pub fn job_count(&self) -> usize {
        self.print_jobs.len()
    }

    pub fn queue_size(&self) -> usize {
        self.print_queue.len()
    }
;

impl Default for SigmaPrint {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaPrint instance
static mut SIGMAPRINT_INSTANCE: Option<SigmaPrint> = None;

/// Initialize the global SigmaPrint instance
pub fn init_sigmaprint() {
    unsafe {
        SIGMAPRINT_INSTANCE = Some(SigmaPrint::new());
    }
;

/// Get the global SigmaPrint instance
pub fn get_sigmaprint() -> Option<&'static SigmaPrint> {
    unsafe {
        SIGMAPRINT_INSTANCE.as_ref()
    }
;

/// Get the global SigmaPrint instance (mutable)
pub fn get_sigmaprint_mut() -> Option<&'static mut SigmaPrint> {
    unsafe {
        SIGMAPRINT_INSTANCE.as_mut()
    }
;

/// Convenience function to add a print job
pub fn add_print_job(job: PrintJob) -> bool {
    if let Some(print_sys) = get_sigmaprint_mut() {
        print_sys.add_print_job(job);
        true
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_options_creation() {
        let options = PrintOptions::new();
        assert_eq!(options.paper_size, PaperSize::A4);
        assert_eq!(options.copies, 1);
        assert!(!options.duplex);
    }

    #[test]
    fn test_print_options_modification() {
        let mut options = PrintOptions::new();
        options.set_paper_size(PaperSize::Letter);
        options.set_copies(2);
        options.set_duplex(true);
        assert_eq!(options.paper_size, PaperSize::Letter);
        assert_eq!(options.copies, 2);
        assert!(options.duplex);
    }

    #[test]
    fn test_print_job_creation() {
        let options = PrintOptions::new();
        let job = PrintJob::new("job1".to_string(), "Document".to_string(), "/path/doc.pdf".to_string(), "printer1".to_string(), options);
        assert_eq!(job.id, "job1");
        assert_eq!(job.status, "Pending");
    }

    #[test]
    fn test_print_job_modification() {
        let options = PrintOptions::new();
        let mut job = PrintJob::new("job1".to_string(), "Test".to_string(), "/path".to_string(), "printer1".to_string(), options);
        job.set_status("Printing".to_string());
        job.set_pages(5);
        assert_eq!(job.status, "Printing");
        assert_eq!(job.pages, 5);
    }

    #[test]
    fn test_sigmaprint_creation() {
        let print_sys = SigmaPrint::new();
        assert_eq!(print_sys.job_count(), 0);
        assert_eq!(print_sys.queue_size(), 0);
    }

    #[test]
    fn test_sigmaprint_management() {
        let mut print_sys = SigmaPrint::new();
        let options = PrintOptions::new();
        let job = PrintJob::new("job1".to_string(), "Test".to_string(), "/path".to_string(), "printer1".to_string(), options);
        print_sys.add_print_job(job);
        assert_eq!(print_sys.job_count(), 1);
        assert_eq!(print_sys.queue_size(), 1);
    }

    #[test]
    fn test_job_completion() {
        let mut print_sys = SigmaPrint::new();
        let options = PrintOptions::new();
        let job = PrintJob::new("job1".to_string(), "Test".to_string(), "/path".to_string(), "printer1".to_string(), options);
        print_sys.add_print_job(job);
        assert!(print_sys.complete_job("job1"));
        assert_eq!(print_sys.queue_size(), 0);
    }
;
