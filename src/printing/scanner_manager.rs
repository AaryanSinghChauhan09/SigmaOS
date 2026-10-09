//! Scanner Manager
//!
//! Scanner management inspired by Linux Mint's scanner settings and Omarchy's
//! scanner utilities, supporting scanner discovery, scan operations, and image processing.

use std::collections::HashMap;

/// Scanner type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScannerType {
    Flatbed,
    SheetFed,
    AllInOne,
    Network,
}

impl ScannerType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "flatbed" => Some(ScannerType::Flatbed),
            "sheetfed" => Some(ScannerType::SheetFed),
            "allinone" | "all-in-one" => Some(ScannerType::AllInOne),
            "network" => Some(ScannerType::Network),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ScannerType::Flatbed => "Flatbed",
            ScannerType::SheetFed => "Sheet-fed",
            ScannerType::AllInOne => "All-in-One",
            ScannerType::Network => "Network",
        }
    }
}

/// Scan resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanResolution {
    DPI75,
    DPI150,
    DPI300,
    DPI600,
    DPI1200,
    DPI2400,
}

impl ScanResolution {
    pub fn from_dpi(dpi: u32) -> Option<Self> {
        match dpi {
            75 => Some(ScanResolution::DPI75),
            150 => Some(ScanResolution::DPI150),
            300 => Some(ScanResolution::DPI300),
            600 => Some(ScanResolution::DPI600),
            1200 => Some(ScanResolution::DPI1200),
            2400 => Some(ScanResolution::DPI2400),
            _ => None,
        }
    }

    pub fn as_dpi(&self) -> u32 {
        match self {
            ScanResolution::DPI75 => 75,
            ScanResolution::DPI150 => 150,
            ScanResolution::DPI300 => 300,
            ScanResolution::DPI600 => 600,
            ScanResolution::DPI1200 => 1200,
            ScanResolution::DPI2400 => 2400,
        }
    }
}

/// Scan color mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanColorMode {
    BlackAndWhite,
    Grayscale,
    Color,
}

impl ScanColorMode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "bw" | "blackandwhite" | "black-and-white" => Some(ScanColorMode::BlackAndWhite),
            "gray" | "grayscale" => Some(ScanColorMode::Grayscale),
            "color" => Some(ScanColorMode::Color),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ScanColorMode::BlackAndWhite => "Black & White",
            ScanColorMode::Grayscale => "Grayscale",
            ScanColorMode::Color => "Color",
        }
    }
}

/// Scan format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanFormat {
    PNG,
    JPEG,
    TIFF,
    PDF,
    BMP,
}

impl ScanFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "png" => Some(ScanFormat::PNG),
            "jpeg" | "jpg" => Some(ScanFormat::JPEG),
            "tiff" => Some(ScanFormat::TIFF),
            "pdf" => Some(ScanFormat::PDF),
            "bmp" => Some(ScanFormat::BMP),
            _ => None,
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            ScanFormat::PNG => ".png",
            ScanFormat::JPEG => ".jpg",
            ScanFormat::TIFF => ".tiff",
            ScanFormat::PDF => ".pdf",
            ScanFormat::BMP => ".bmp",
        }
    }
}

/// Scan job status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanJobStatus {
    Pending,
    Scanning,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

impl ScanJobStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Some(ScanJobStatus::Pending),
            "scanning" => Some(ScanJobStatus::Scanning),
            "processing" => Some(ScanJobStatus::Processing),
            "completed" => Some(ScanJobStatus::Completed),
            "failed" => Some(ScanJobStatus::Failed),
            "cancelled" => Some(ScanJobStatus::Cancelled),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ScanJobStatus::Pending => "Pending",
            ScanJobStatus::Scanning => "Scanning",
            ScanJobStatus::Processing => "Processing",
            ScanJobStatus::Completed => "Completed",
            ScanJobStatus::Failed => "Failed",
            ScanJobStatus::Cancelled => "Cancelled",
        }
    }
}

/// Scan job
#[derive(Debug, Clone)]
pub struct ScanJob {
    pub id: String,
    pub scanner_id: String,
    pub output_path: String,
    pub resolution: ScanResolution,
    pub color_mode: ScanColorMode,
    pub format: ScanFormat,
    pub pages: u32,
    pub status: ScanJobStatus,
    pub created_at: u64,
}

impl ScanJob {
    pub fn new(
        id: String,
        scanner_id: String,
        output_path: String,
        resolution: ScanResolution,
        color_mode: ScanColorMode,
        format: ScanFormat,
    ) -> Self {
        Self {
            id,
            scanner_id,
            output_path,
            resolution,
            color_mode,
            format,
            pages: 1,
            status: ScanJobStatus::Pending,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn set_pages(&mut self, pages: u32) {
        self.pages = pages;
    }

    pub fn set_status(&mut self, status: ScanJobStatus) {
        self.status = status;
    }
}

/// Scanner
#[derive(Debug, Clone)]
pub struct Scanner {
    pub id: String,
    pub name: String,
    pub scanner_type: ScannerType,
    pub uri: String,
    pub driver: String,
    pub is_available: bool,
}

impl Scanner {
    pub fn new(id: String, name: String, scanner_type: ScannerType, uri: String) -> Self {
        Self {
            id,
            name,
            scanner_type,
            uri,
            driver: String::new(),
            is_available: true,
        }
    }

    pub fn set_driver(&mut self, driver: String) {
        self.driver = driver;
    }

    pub fn set_available(&mut self, available: bool) {
        self.is_available = available;
    }
}

/// Scanner manager
#[derive(Debug)]
pub struct ScannerManager {
    scanners: HashMap<String, Scanner>,
    jobs: HashMap<String, ScanJob>,
    next_job_id: u64,
}

impl ScannerManager {
    pub fn new() -> Self {
        let mut manager = Self {
            scanners: HashMap::new(),
            jobs: HashMap::new(),
            next_job_id: 1,
        };

        // Add default scanner
        let scanner = Scanner::new(
            "default-scanner".to_string(),
            "Default Scanner".to_string(),
            ScannerType::Flatbed,
            "sane://default".to_string(),
        );
        manager
            .scanners
            .insert("default-scanner".to_string(), scanner);

        manager
    }

    /// Add a scanner
    pub fn add_scanner(&mut self, scanner: Scanner) {
        self.scanners.insert(scanner.id.clone(), scanner);
    }

    /// Get a scanner
    pub fn get_scanner(&self, id: &str) -> Option<&Scanner> {
        self.scanners.get(id)
    }

    /// List all scanners
    pub fn list_scanners(&self) -> Vec<&Scanner> {
        self.scanners.values().collect()
    }

    /// List available scanners
    pub fn list_available(&self) -> Vec<&Scanner> {
        self.scanners.values().filter(|s| s.is_available).collect()
    }

    /// Scan for devices (simulated)
    pub fn scan_devices(&mut self) -> Vec<&Scanner> {
        self.scanners.clear();

        // Simulate device discovery
        let discovered_scanners = vec![
            (
                "flatbed-001",
                "Epson Flatbed",
                ScannerType::Flatbed,
                "sane://epson:fw:00:00:00:00:00:00",
            ),
            (
                "network-001",
                "HP Network Scanner",
                ScannerType::Network,
                "sane://hp:net:192.168.1.100",
            ),
            (
                "aio-001",
                "Canon All-in-One",
                ScannerType::AllInOne,
                "sane://canon:usb:001/002",
            ),
        ];

        for (id, name, scanner_type, uri) in discovered_scanners {
            let scanner = Scanner::new(
                id.to_string(),
                name.to_string(),
                scanner_type,
                uri.to_string(),
            );
            self.scanners.insert(id.to_string(), scanner);
        }

        self.scanners.values().collect()
    }

    /// Submit a scan job
    pub fn submit_job(
        &mut self,
        scanner_id: &str,
        output_path: String,
        resolution: ScanResolution,
        color_mode: ScanColorMode,
        format: ScanFormat,
    ) -> Result<String, String> {
        let scanner = self
            .get_scanner(scanner_id)
            .ok_or_else(|| format!("Scanner {} not found", scanner_id))?;

        if !scanner.is_available {
            return Err(format!("Scanner {} is not available", scanner_id));
        }

        let job_id = format!("scan-{}", self.next_job_id);
        self.next_job_id += 1;

        let job = ScanJob::new(
            job_id.clone(),
            scanner_id.to_string(),
            output_path,
            resolution,
            color_mode,
            format,
        );
        self.jobs.insert(job_id.clone(), job);

        Ok(job_id)
    }

    /// Get a job
    pub fn get_job(&self, id: &str) -> Option<&ScanJob> {
        self.jobs.get(id)
    }

    /// List all jobs
    pub fn list_jobs(&self) -> Vec<&ScanJob> {
        self.jobs.values().collect()
    }

    /// List jobs by status
    pub fn list_jobs_by_status(&self, status: ScanJobStatus) -> Vec<&ScanJob> {
        self.jobs.values().filter(|j| j.status == status).collect()
    }

    /// Cancel a job
    pub fn cancel_job(&mut self, id: &str) -> Result<(), String> {
        let job = self
            .jobs
            .get_mut(id)
            .ok_or_else(|| format!("Job {} not found", id))?;

        if job.status == ScanJobStatus::Completed {
            return Err("Cannot cancel completed job".to_string());
        }

        job.set_status(ScanJobStatus::Cancelled);
        Ok(())
    }

    /// Get statistics
    pub fn get_statistics(&self) -> ScannerStatistics {
        let total_scanners = self.scanners.len();
        let available_count = self.scanners.values().filter(|s| s.is_available).count();
        let total_jobs = self.jobs.len();
        let pending_jobs = self
            .jobs
            .values()
            .filter(|j| j.status == ScanJobStatus::Pending)
            .count();
        let scanning_jobs = self
            .jobs
            .values()
            .filter(|j| j.status == ScanJobStatus::Scanning)
            .count();

        ScannerStatistics {
            total_scanners,
            available_count,
            total_jobs,
            pending_jobs,
            scanning_jobs,
        }
    }
}

impl Default for ScannerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Scanner statistics
#[derive(Debug, Clone)]
pub struct ScannerStatistics {
    pub total_scanners: usize,
    pub available_count: usize,
    pub total_jobs: usize,
    pub pending_jobs: usize,
    pub scanning_jobs: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_type_from_str() {
        assert_eq!(ScannerType::from_str("flatbed"), Some(ScannerType::Flatbed));
        assert_eq!(ScannerType::from_str("network"), Some(ScannerType::Network));
    }

    #[test]
    fn test_scan_resolution_from_dpi() {
        assert_eq!(ScanResolution::from_dpi(300), Some(ScanResolution::DPI300));
        assert_eq!(ScanResolution::from_dpi(600), Some(ScanResolution::DPI600));
    }

    #[test]
    fn test_scan_color_mode_from_str() {
        assert_eq!(ScanColorMode::from_str("color"), Some(ScanColorMode::Color));
        assert_eq!(
            ScanColorMode::from_str("grayscale"),
            Some(ScanColorMode::Grayscale)
        );
    }

    #[test]
    fn test_scan_format_from_str() {
        assert_eq!(ScanFormat::from_str("png"), Some(ScanFormat::PNG));
        assert_eq!(ScanFormat::from_str("pdf"), Some(ScanFormat::PDF));
    }

    #[test]
    fn test_scanner_creation() {
        let scanner = Scanner::new(
            "test".to_string(),
            "Test Scanner".to_string(),
            ScannerType::Flatbed,
            "sane://test".to_string(),
        );
        assert_eq!(scanner.name, "Test Scanner");
    }

    #[test]
    fn test_scanner_manager_creation() {
        let manager = ScannerManager::new();
        assert!(manager.get_scanner("default-scanner").is_some());
    }

    #[test]
    fn test_scan_devices() {
        let mut manager = ScannerManager::new();
        let scanners = manager.scan_devices();
        assert!(scanners.len() >= 3);
    }

    #[test]
    fn test_submit_job() {
        let mut manager = ScannerManager::new();
        let job_id = manager.submit_job(
            "default-scanner",
            "/tmp/scan.png".to_string(),
            ScanResolution::DPI300,
            ScanColorMode::Color,
            ScanFormat::PNG,
        );
        assert!(job_id.is_ok());
    }

    #[test]
    fn test_cancel_job() {
        let mut manager = ScannerManager::new();
        let job_id = manager
            .submit_job(
                "default-scanner",
                "/tmp/scan.png".to_string(),
                ScanResolution::DPI300,
                ScanColorMode::Color,
                ScanFormat::PNG,
            )
            .unwrap();
        assert!(manager.cancel_job(&job_id).is_ok());
    }

    #[test]
    fn test_statistics() {
        let manager = ScannerManager::new();
        let stats = manager.get_statistics();
        assert!(stats.total_scanners >= 1);
    }
}
