// Desktop Print Manager
// Linux Mint & Omarchy inspiration for comprehensive print management

use std::collections::HashMap;

/// Printer Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrinterStatus {
    Idle,
    Printing,
    Stopped,
    Error,
    Offline,
}

impl PrinterStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PrinterStatus::Idle => "idle",
            PrinterStatus::Printing => "printing",
            PrinterStatus::Stopped => "stopped",
            PrinterStatus::Error => "error",
            PrinterStatus::Offline => "offline",
        }
    }
}

/// Print Job Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintJobStatus {
    Pending,
    Processing,
    Completed,
    Canceled,
    Failed,
}

impl PrintJobStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PrintJobStatus::Pending => "pending",
            PrintJobStatus::Processing => "processing",
            PrintJobStatus::Completed => "completed",
            PrintJobStatus::Canceled => "canceled",
            PrintJobStatus::Failed => "failed",
        }
    }
}

/// Printer
#[derive(Debug, Clone)]
pub struct Printer {
    pub id: String,
    pub name: String,
    pub location: String,
    pub make_and_model: String,
    pub uri: String,
    pub status: PrinterStatus,
    pub is_default: bool,
    pub is_shared: bool,
}

impl Printer {
    pub fn new(id: String, name: String, uri: String) -> Self {
        Self {
            id,
            name,
            location: String::new(),
            make_and_model: String::new(),
            uri,
            status: PrinterStatus::Idle,
            is_default: false,
            is_shared: false,
        }
    }

    pub fn with_location(mut self, location: String) -> Self {
        self.location = location;
        self
    }

    pub fn with_make_and_model(mut self, make_and_model: String) -> Self {
        self.make_and_model = make_and_model;
        self
    }

    pub fn set_status(&mut self, status: PrinterStatus) {
        self.status = status;
    }

    pub fn set_default(&mut self, is_default: bool) {
        self.is_default = is_default;
    }

    pub fn set_shared(&mut self, is_shared: bool) {
        self.is_shared = is_shared;
    }
}

/// Print Job
#[derive(Debug, Clone)]
pub struct PrintJob {
    pub id: String,
    pub printer_id: String,
    pub title: String,
    pub file_path: String,
    pub status: PrintJobStatus,
    pub pages: u32,
    pub copies: u32,
    pub color_mode: bool,
    pub duplex: bool,
}

impl PrintJob {
    pub fn new(id: String, printer_id: String, title: String, file_path: String) -> Self {
        Self {
            id,
            printer_id,
            title,
            file_path,
            status: PrintJobStatus::Pending,
            pages: 1,
            copies: 1,
            color_mode: false,
            duplex: false,
        }
    }

    pub fn with_pages(mut self, pages: u32) -> Self {
        self.pages = pages;
        self
    }

    pub fn with_copies(mut self, copies: u32) -> Self {
        self.copies = copies;
        self
    }

    pub fn with_color_mode(mut self, color_mode: bool) -> Self {
        self.color_mode = color_mode;
        self
    }

    pub fn with_duplex(mut self, duplex: bool) -> Self {
        self.duplex = duplex;
        self
    }

    pub fn set_status(&mut self, status: PrintJobStatus) {
        self.status = status;
    }
}

/// Desktop Print Manager
pub struct DesktopPrintManager {
    printers: HashMap<String, Printer>,
    print_jobs: HashMap<String, PrintJob>,
    default_printer: Option<String>,
    counter: u32,
}

impl DesktopPrintManager {
    pub fn new() -> Self {
        let mut manager = Self {
            printers: HashMap::new(),
            print_jobs: HashMap::new(),
            default_printer: None,
            counter: 1000,
        };

        // Add default printer
        manager.add_default_printer();

        manager
    }

    fn add_default_printer(&mut self) {
        let mut printer = Printer::new(
            "printer_0".to_string(),
            "Default Printer".to_string(),
            "ipp://localhost:631/printers/default".to_string(),
        );
        printer.set_status(PrinterStatus::Idle);
        printer.set_default(true);

        let printer_id = printer.id.clone();
        self.printers.insert(printer_id.clone(), printer);
        self.default_printer = Some(printer_id);
    }

    pub fn add_printer(&mut self, printer: Printer) -> String {
        let id = format!("printer_{}", self.counter);
        self.counter += 1;

        let printer = Printer {
            id: id.clone(),
            ..printer
        };

        self.printers.insert(id.clone(), printer);
        id
    }

    pub fn remove_printer(&mut self, id: &str) -> bool {
        if Some(id.to_string()) == self.default_printer {
            self.default_printer = None;
        }
        self.printers.remove(id).is_some()
    }

    pub fn get_printer(&self, id: &str) -> Option<&Printer> {
        self.printers.get(id)
    }

    pub fn get_printers(&self) -> Vec<&Printer> {
        self.printers.values().collect()
    }

    pub fn set_printer_status(&mut self, id: &str, status: PrinterStatus) -> bool {
        if let Some(printer) = self.printers.get_mut(id) {
            printer.set_status(status);
            true
        } else {
            false
        }
    }

    pub fn set_default_printer(&mut self, id: &str) -> bool {
        if self.printers.contains_key(id) {
            // Unset previous default
            if let Some(prev_id) = &self.default_printer {
                if let Some(printer) = self.printers.get_mut(prev_id) {
                    printer.set_default(false);
                }
            }

            // Set new default
            if let Some(printer) = self.printers.get_mut(id) {
                printer.set_default(true);
            }

            self.default_printer = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_default_printer(&self) -> Option<&Printer> {
        self.default_printer
            .as_ref()
            .and_then(|id| self.printers.get(id))
    }

    pub fn set_printer_shared(&mut self, id: &str, shared: bool) -> bool {
        if let Some(printer) = self.printers.get_mut(id) {
            printer.set_shared(shared);
            true
        } else {
            false
        }
    }

    pub fn add_print_job(&mut self, job: PrintJob) -> String {
        let id = format!("job_{}", self.counter);
        self.counter += 1;

        let job = PrintJob {
            id: id.clone(),
            ..job
        };

        self.print_jobs.insert(id.clone(), job);
        id
    }

    pub fn remove_print_job(&mut self, id: &str) -> bool {
        self.print_jobs.remove(id).is_some()
    }

    pub fn get_print_job(&self, id: &str) -> Option<&PrintJob> {
        self.print_jobs.get(id)
    }

    pub fn get_print_jobs(&self) -> Vec<&PrintJob> {
        self.print_jobs.values().collect()
    }

    pub fn get_print_jobs_by_printer(&self, printer_id: &str) -> Vec<&PrintJob> {
        self.print_jobs
            .values()
            .filter(|j| j.printer_id == printer_id)
            .collect()
    }

    pub fn get_print_jobs_by_status(&self, status: PrintJobStatus) -> Vec<&PrintJob> {
        self.print_jobs
            .values()
            .filter(|j| j.status == status)
            .collect()
    }

    pub fn set_print_job_status(&mut self, id: &str, status: PrintJobStatus) -> bool {
        if let Some(job) = self.print_jobs.get_mut(id) {
            job.set_status(status);
            true
        } else {
            false
        }
    }

    pub fn cancel_print_job(&mut self, id: &str) -> bool {
        self.set_print_job_status(id, PrintJobStatus::Canceled)
    }

    pub fn get_statistics(&self) -> PrintManagerStatistics {
        PrintManagerStatistics {
            total_printers: self.printers.len(),
            idle_printers: self.printers.values().filter(|p| p.status == PrinterStatus::Idle).count(),
            printing_printers: self.printers.values().filter(|p| p.status == PrinterStatus::Printing).count(),
            error_printers: self.printers.values().filter(|p| p.status == PrinterStatus::Error).count(),
            total_jobs: self.print_jobs.len(),
            pending_jobs: self.print_jobs.values().filter(|j| j.status == PrintJobStatus::Pending).count(),
            processing_jobs: self.print_jobs.values().filter(|j| j.status == PrintJobStatus::Processing).count(),
            default_printer_set: self.default_printer.is_some(),
        }
    }
}

impl Default for DesktopPrintManager {
    fn default() -> Self {
        Self::new()
    }
}

/// PrintManagerStatistics
#[derive(Debug, Clone, Copy)]
pub struct PrintManagerStatistics {
    pub total_printers: usize,
    pub idle_printers: usize,
    pub printing_printers: usize,
    pub error_printers: usize,
    pub total_jobs: usize,
    pub pending_jobs: usize,
    pub processing_jobs: usize,
    pub default_printer_set: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_manager_state() {
        let manager = DesktopPrintManager::new();
        let stats = manager.get_statistics();

        assert_eq!(stats.total_printers, 1);
        assert!(stats.default_printer_set);
        assert_eq!(stats.total_jobs, 0);
    }

    #[test]
    fn test_add_printer() {
        let mut manager = DesktopPrintManager::new();
        let initial_count = manager.get_printers().len();

        let printer = Printer::new(
            "custom".to_string(),
            "Custom Printer".to_string(),
            "ipp://localhost:631/printers/custom".to_string(),
        );

        let id = manager.add_printer(printer);
        assert!(manager.get_printer(&id).is_some());
        assert_eq!(manager.get_printers().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_printer() {
        let mut manager = DesktopPrintManager::new();

        let printer = Printer::new(
            "custom".to_string(),
            "Custom Printer".to_string(),
            "ipp://localhost:631/printers/custom".to_string(),
        );

        let id = manager.add_printer(printer);
        assert!(manager.remove_printer(&id));
        assert!(manager.get_printer(&id).is_none());
    }

    #[test]
    fn test_set_printer_status() {
        let mut manager = DesktopPrintManager::new();
        let printer_id = "printer_0";

        assert!(manager.set_printer_status(printer_id, PrinterStatus::Printing));
        let printer = manager.get_printer(printer_id).unwrap();
        assert_eq!(printer.status, PrinterStatus::Printing);
    }

    #[test]
    fn test_set_default_printer() {
        let mut manager = DesktopPrintManager::new();

        let printer = Printer::new(
            "custom".to_string(),
            "Custom Printer".to_string(),
            "ipp://localhost:631/printers/custom".to_string(),
        );

        let id = manager.add_printer(printer);
        assert!(manager.set_default_printer(&id));

        let default = manager.get_default_printer().unwrap();
        assert_eq!(default.id, id);
        assert!(default.is_default);
    }

    #[test]
    fn test_set_printer_shared() {
        let mut manager = DesktopPrintManager::new();
        let printer_id = "printer_0";

        assert!(manager.set_printer_shared(printer_id, true));
        let printer = manager.get_printer(printer_id).unwrap();
        assert!(printer.is_shared);
    }

    #[test]
    fn test_add_print_job() {
        let mut manager = DesktopPrintManager::new();
        let initial_count = manager.get_print_jobs().len();

        let job = PrintJob::new(
            "job".to_string(),
            "printer_0".to_string(),
            "Test Document".to_string(),
            "/tmp/document.pdf".to_string(),
        );

        let id = manager.add_print_job(job);
        assert!(manager.get_print_job(&id).is_some());
        assert_eq!(manager.get_print_jobs().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_print_job() {
        let mut manager = DesktopPrintManager::new();

        let job = PrintJob::new(
            "job".to_string(),
            "printer_0".to_string(),
            "Test Document".to_string(),
            "/tmp/document.pdf".to_string(),
        );

        let id = manager.add_print_job(job);
        assert!(manager.remove_print_job(&id));
        assert!(manager.get_print_job(&id).is_none());
    }

    #[test]
    fn test_set_print_job_status() {
        let mut manager = DesktopPrintManager::new();

        let job = PrintJob::new(
            "job".to_string(),
            "printer_0".to_string(),
            "Test Document".to_string(),
            "/tmp/document.pdf".to_string(),
        );

        let id = manager.add_print_job(job);
        assert!(manager.set_print_job_status(&id, PrintJobStatus::Processing));

        let print_job = manager.get_print_job(&id).unwrap();
        assert_eq!(print_job.status, PrintJobStatus::Processing);
    }

    #[test]
    fn test_cancel_print_job() {
        let mut manager = DesktopPrintManager::new();

        let job = PrintJob::new(
            "job".to_string(),
            "printer_0".to_string(),
            "Test Document".to_string(),
            "/tmp/document.pdf".to_string(),
        );

        let id = manager.add_print_job(job);
        assert!(manager.cancel_print_job(&id));

        let print_job = manager.get_print_job(&id).unwrap();
        assert_eq!(print_job.status, PrintJobStatus::Canceled);
    }

    #[test]
    fn test_get_print_jobs_by_printer() {
        let mut manager = DesktopPrintManager::new();

        manager.add_print_job(PrintJob::new(
            "job1".to_string(),
            "printer_0".to_string(),
            "Doc 1".to_string(),
            "/tmp/doc1.pdf".to_string(),
        ));

        manager.add_print_job(PrintJob::new(
            "job2".to_string(),
            "printer_0".to_string(),
            "Doc 2".to_string(),
            "/tmp/doc2.pdf".to_string(),
        ));

        let jobs = manager.get_print_jobs_by_printer("printer_0");
        assert_eq!(jobs.len(), 2);
    }

    #[test]
    fn test_get_print_jobs_by_status() {
        let mut manager = DesktopPrintManager::new();

        let job1 = PrintJob::new(
            "job1".to_string(),
            "printer_0".to_string(),
            "Doc 1".to_string(),
            "/tmp/doc1.pdf".to_string(),
        );

        let id1 = manager.add_print_job(job1);
        manager.set_print_job_status(&id1, PrintJobStatus::Completed);

        let job2 = PrintJob::new(
            "job2".to_string(),
            "printer_0".to_string(),
            "Doc 2".to_string(),
            "/tmp/doc2.pdf".to_string(),
        );

        manager.add_print_job(job2);

        let pending = manager.get_print_jobs_by_status(PrintJobStatus::Pending);
        assert_eq!(pending.len(), 1);

        let completed = manager.get_print_jobs_by_status(PrintJobStatus::Completed);
        assert_eq!(completed.len(), 1);
    }

    #[test]
    fn test_print_job_options() {
        let mut manager = DesktopPrintManager::new();

        let job = PrintJob::new(
            "job".to_string(),
            "printer_0".to_string(),
            "Test Document".to_string(),
            "/tmp/document.pdf".to_string(),
        )
        .with_pages(5)
        .with_copies(2)
        .with_color_mode(true)
        .with_duplex(true);

        let id = manager.add_print_job(job);
        let print_job = manager.get_print_job(&id).unwrap();

        assert_eq!(print_job.pages, 5);
        assert_eq!(print_job.copies, 2);
        assert!(print_job.color_mode);
        assert!(print_job.duplex);
    }
}
