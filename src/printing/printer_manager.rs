//! Printer Manager
//!
//! Printer management inspired by Linux Mint's printer settings and Omarchy's
//! printer utilities, supporting printer discovery, queue management, and job control.

use std::collections::HashMap;

/// Printer type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrinterType {
    Local,
    Network,
    IPP,
    PDF,
}

impl PrinterType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "local" => Some(PrinterType::Local),
            "network" => Some(PrinterType::Network),
            "ipp" => Some(PrinterType::IPP),
            "pdf" => Some(PrinterType::PDF),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PrinterType::Local => "Local",
            PrinterType::Network => "Network",
            PrinterType::IPP => "IPP",
            PrinterType::PDF => "PDF",
        }
    }
}

/// Printer status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrinterStatus {
    Idle,
    Printing,
    Paused,
    Error,
    Offline,
}

impl PrinterStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "idle" => Some(PrinterStatus::Idle),
            "printing" => Some(PrinterStatus::Printing),
            "paused" => Some(PrinterStatus::Paused),
            "error" => Some(PrinterStatus::Error),
            "offline" => Some(PrinterStatus::Offline),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PrinterStatus::Idle => "Idle",
            PrinterStatus::Printing => "Printing",
            PrinterStatus::Paused => "Paused",
            PrinterStatus::Error => "Error",
            PrinterStatus::Offline => "Offline",
        }
    }
}

/// Print job status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintJobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

impl PrintJobStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Some(PrintJobStatus::Pending),
            "processing" => Some(PrintJobStatus::Processing),
            "completed" => Some(PrintJobStatus::Completed),
            "failed" => Some(PrintJobStatus::Failed),
            "cancelled" => Some(PrintJobStatus::Cancelled),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PrintJobStatus::Pending => "Pending",
            PrintJobStatus::Processing => "Processing",
            PrintJobStatus::Completed => "Completed",
            PrintJobStatus::Failed => "Failed",
            PrintJobStatus::Cancelled => "Cancelled",
        }
    }
}

/// Print job
#[derive(Debug, Clone)]
pub struct PrintJob {
    pub id: String,
    pub title: String,
    pub file_path: String,
    pub pages: u32,
    pub copies: u32,
    pub color: bool,
    pub duplex: bool,
    pub status: PrintJobStatus,
    pub created_at: u64,
}

impl PrintJob {
    pub fn new(id: String, title: String, file_path: String, pages: u32) -> Self {
        Self {
            id,
            title,
            file_path,
            pages,
            copies: 1,
            color: false,
            duplex: false,
            status: PrintJobStatus::Pending,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn set_copies(&mut self, copies: u32) {
        self.copies = copies;
    }

    pub fn set_color(&mut self, color: bool) {
        self.color = color;
    }

    pub fn set_duplex(&mut self, duplex: bool) {
        self.duplex = duplex;
    }

    pub fn set_status(&mut self, status: PrintJobStatus) {
        self.status = status;
    }
}

/// Printer
#[derive(Debug, Clone)]
pub struct Printer {
    pub id: String,
    pub name: String,
    pub location: String,
    pub printer_type: PrinterType,
    pub status: PrinterStatus,
    pub uri: String,
    pub driver: String,
    pub is_default: bool,
    pub is_enabled: bool,
}

impl Printer {
    pub fn new(id: String, name: String, printer_type: PrinterType, uri: String) -> Self {
        Self {
            id,
            name,
            location: String::new(),
            printer_type,
            status: PrinterStatus::Idle,
            uri,
            driver: String::new(),
            is_default: false,
            is_enabled: true,
        }
    }

    pub fn set_location(&mut self, location: String) {
        self.location = location;
    }

    pub fn set_driver(&mut self, driver: String) {
        self.driver = driver;
    }

    pub fn set_default(&mut self, is_default: bool) {
        self.is_default = is_default;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    pub fn set_status(&mut self, status: PrinterStatus) {
        self.status = status;
    }
}

/// Printer manager
#[derive(Debug)]
pub struct PrinterManager {
    printers: HashMap<String, Printer>,
    jobs: HashMap<String, PrintJob>,
    next_job_id: u64,
}

impl PrinterManager {
    pub fn new() -> Self {
        let mut manager = Self {
            printers: HashMap::new(),
            jobs: HashMap::new(),
            next_job_id: 1,
        };

        // Add default PDF printer
        let mut pdf_printer = Printer::new(
            "pdf-printer".to_string(),
            "Print to PDF".to_string(),
            PrinterType::PDF,
            "pdf://".to_string(),
        );
        pdf_printer.set_default(true);
        manager
            .printers
            .insert("pdf-printer".to_string(), pdf_printer);

        manager
    }

    /// Add a printer
    pub fn add_printer(&mut self, printer: Printer) {
        self.printers.insert(printer.id.clone(), printer);
    }

    /// Get a printer
    pub fn get_printer(&self, id: &str) -> Option<&Printer> {
        self.printers.get(id)
    }

    /// Get a printer mutably
    pub fn get_printer_mut(&mut self, id: &str) -> Option<&mut Printer> {
        self.printers.get_mut(id)
    }

    /// List all printers
    pub fn list_printers(&self) -> Vec<&Printer> {
        self.printers.values().collect()
    }

    /// List enabled printers
    pub fn list_enabled(&self) -> Vec<&Printer> {
        self.printers.values().filter(|p| p.is_enabled).collect()
    }

    /// Get default printer
    pub fn get_default(&self) -> Option<&Printer> {
        self.printers.values().find(|p| p.is_default)
    }

    /// Set default printer
    pub fn set_default(&mut self, id: &str) -> Result<(), String> {
        // Clear existing default
        for printer in self.printers.values_mut() {
            printer.set_default(false);
        }

        let printer = self
            .get_printer_mut(id)
            .ok_or_else(|| format!("Printer {} not found", id))?;

        printer.set_default(true);
        Ok(())
    }

    /// Enable a printer
    pub fn enable(&mut self, id: &str) -> Result<(), String> {
        let printer = self
            .get_printer_mut(id)
            .ok_or_else(|| format!("Printer {} not found", id))?;

        printer.set_enabled(true);
        Ok(())
    }

    /// Disable a printer
    pub fn disable(&mut self, id: &str) -> Result<(), String> {
        let printer = self
            .get_printer_mut(id)
            .ok_or_else(|| format!("Printer {} not found", id))?;

        printer.set_enabled(false);
        Ok(())
    }

    /// Remove a printer
    pub fn remove(&mut self, id: &str) -> Result<(), String> {
        let printer = self
            .get_printer(id)
            .ok_or_else(|| format!("Printer {} not found", id))?;

        if printer.is_default {
            return Err("Cannot remove default printer".to_string());
        }

        self.printers
            .remove(id)
            .ok_or_else(|| format!("Printer {} not found", id))?;
        Ok(())
    }

    /// Submit a print job
    pub fn submit_job(
        &mut self,
        printer_id: &str,
        title: String,
        file_path: String,
        pages: u32,
    ) -> Result<String, String> {
        let printer = self
            .get_printer(printer_id)
            .ok_or_else(|| format!("Printer {} not found", printer_id))?;

        if !printer.is_enabled {
            return Err(format!("Printer {} is disabled", printer_id));
        }

        let job_id = format!("job-{}", self.next_job_id);
        self.next_job_id += 1;

        let job = PrintJob::new(job_id.clone(), title, file_path, pages);
        self.jobs.insert(job_id.clone(), job);

        Ok(job_id)
    }

    /// Get a job
    pub fn get_job(&self, id: &str) -> Option<&PrintJob> {
        self.jobs.get(id)
    }

    /// List all jobs
    pub fn list_jobs(&self) -> Vec<&PrintJob> {
        self.jobs.values().collect()
    }

    /// List jobs by status
    pub fn list_jobs_by_status(&self, status: PrintJobStatus) -> Vec<&PrintJob> {
        self.jobs.values().filter(|j| j.status == status).collect()
    }

    /// Cancel a job
    pub fn cancel_job(&mut self, id: &str) -> Result<(), String> {
        let job = self
            .jobs
            .get_mut(id)
            .ok_or_else(|| format!("Job {} not found", id))?;

        if job.status == PrintJobStatus::Completed {
            return Err("Cannot cancel completed job".to_string());
        }

        job.set_status(PrintJobStatus::Cancelled);
        Ok(())
    }

    /// Get statistics
    pub fn get_statistics(&self) -> PrinterStatistics {
        let total_printers = self.printers.len();
        let enabled_count = self.printers.values().filter(|p| p.is_enabled).count();
        let offline_count = self
            .printers
            .values()
            .filter(|p| p.status == PrinterStatus::Offline)
            .count();
        let total_jobs = self.jobs.len();
        let pending_jobs = self
            .jobs
            .values()
            .filter(|j| j.status == PrintJobStatus::Pending)
            .count();
        let processing_jobs = self
            .jobs
            .values()
            .filter(|j| j.status == PrintJobStatus::Processing)
            .count();

        PrinterStatistics {
            total_printers,
            enabled_count,
            offline_count,
            total_jobs,
            pending_jobs,
            processing_jobs,
        }
    }
}

impl Default for PrinterManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Printer statistics
#[derive(Debug, Clone)]
pub struct PrinterStatistics {
    pub total_printers: usize,
    pub enabled_count: usize,
    pub offline_count: usize,
    pub total_jobs: usize,
    pub pending_jobs: usize,
    pub processing_jobs: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printer_type_from_str() {
        assert_eq!(PrinterType::from_str("local"), Some(PrinterType::Local));
        assert_eq!(PrinterType::from_str("pdf"), Some(PrinterType::PDF));
    }

    #[test]
    fn test_printer_status_from_str() {
        assert_eq!(PrinterStatus::from_str("idle"), Some(PrinterStatus::Idle));
        assert_eq!(
            PrinterStatus::from_str("printing"),
            Some(PrinterStatus::Printing)
        );
    }

    #[test]
    fn test_printer_creation() {
        let printer = Printer::new(
            "test".to_string(),
            "Test Printer".to_string(),
            PrinterType::Local,
            "usb://".to_string(),
        );
        assert_eq!(printer.name, "Test Printer");
    }

    #[test]
    fn test_printer_manager_creation() {
        let manager = PrinterManager::new();
        assert!(manager.get_default().is_some());
    }

    #[test]
    fn test_add_printer() {
        let mut manager = PrinterManager::new();
        let printer = Printer::new(
            "test".to_string(),
            "Test".to_string(),
            PrinterType::Local,
            "usb://".to_string(),
        );
        manager.add_printer(printer);
        assert!(manager.get_printer("test").is_some());
    }

    #[test]
    fn test_set_default() {
        let mut manager = PrinterManager::new();
        let printer = Printer::new(
            "test".to_string(),
            "Test".to_string(),
            PrinterType::Local,
            "usb://".to_string(),
        );
        manager.add_printer(printer);
        assert!(manager.set_default("test").is_ok());
    }

    #[test]
    fn test_submit_job() {
        let mut manager = PrinterManager::new();
        let job_id = manager.submit_job(
            "pdf-printer",
            "Test Document".to_string(),
            "/tmp/doc.pdf".to_string(),
            5,
        );
        assert!(job_id.is_ok());
    }

    #[test]
    fn test_cancel_job() {
        let mut manager = PrinterManager::new();
        let job_id = manager
            .submit_job(
                "pdf-printer",
                "Test".to_string(),
                "/tmp/doc.pdf".to_string(),
                1,
            )
            .unwrap();
        assert!(manager.cancel_job(&job_id).is_ok());
    }

    #[test]
    fn test_statistics() {
        let manager = PrinterManager::new();
        let stats = manager.get_statistics();
        assert!(stats.total_printers >= 1);
    }
}
