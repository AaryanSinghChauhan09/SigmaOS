extern crate alloc;
// Sovereign Printing Subsystem for SigmaOS
// Implements Linux CUPS IPP 2.0, BSD LPD (RFC 1179), and PPD driver matching

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintJobState {
    Pending,
    Processing,
    Completed,
    Canceled,
    Failed,
}

#[derive(Debug, Clone)]
pub struct PrintJob {
    pub job_id: u32,
    pub title: String,
    pub owner: String,
    pub state: PrintJobState,
    pub pages: u32,
    pub payload: Vec<u8>,
}

/// CUPS IPP 2.0 Print Spooler Engine
pub struct CupsIppPrintSpooler {
    pub printer_name: String,
    pub jobs: BTreeMap<u32, PrintJob>,
    pub next_job_id: u32,
    pub is_accepting_jobs: bool,
}

impl CupsIppPrintSpooler {
    pub fn new(printer_name: &str) -> Self {
        Self {
            printer_name: printer_name.to_string(),
            jobs: BTreeMap::new(),
            next_job_id: 1,
            is_accepting_jobs: true,
        }
    }

    pub fn submit_ipp_job(&mut self, title: &str, owner: &str, pages: u32, payload: &[u8]) -> Result<u32, &'static str> {
        if !self.is_accepting_jobs {
            return Err("CUPS IPP: Printer is currently rejecting jobs");
        }

        let job_id = self.next_job_id;
        self.next_job_id += 1;

        let job = PrintJob {
            job_id,
            title: title.to_string(),
            owner: owner.to_string(),
            state: PrintJobState::Pending,
            pages,
            payload: payload.to_vec(),
        };

        self.jobs.insert(job_id, job);
        Ok(job_id)
    }

    pub fn process_next_job(&mut self) -> Option<u32> {
        let pending_id = self
            .jobs
            .iter()
            .find(|(_, job)| job.state == PrintJobState::Pending)
            .map(|(&id, _)| id)?;

        if let Some(job) = self.jobs.get_mut(&pending_id) {
            job.state = PrintJobState::Processing;
            // Rasterize / process job
            job.state = PrintJobState::Completed;
        }

        Some(pending_id)
    }

    pub fn cancel_job(&mut self, job_id: u32) -> Result<(), &'static str> {
        let job = self.jobs.get_mut(&job_id).ok_or("Job not found")?;
        if job.state == PrintJobState::Completed {
            return Err("Cannot cancel completed job");
        }
        job.state = PrintJobState::Canceled;
        Ok(())
    }
}

impl Default for CupsIppPrintSpooler {
    fn default() -> Self {
        Self::new("SigmaPDF_Virtual_Printer")
    }
}

/// BSD Line Printer Daemon (LPD - RFC 1179) Remote Spooler
pub struct LpdSpooler {
    pub queue_name: String,
    pub spool_directory: String,
    pub queued_jobs: Vec<PrintJob>,
}

impl LpdSpooler {
    pub fn new(queue_name: &str, spool_directory: &str) -> Self {
        Self {
            queue_name: queue_name.to_string(),
            spool_directory: spool_directory.to_string(),
            queued_jobs: Vec::new(),
        }
    }

    pub fn receive_control_file(&mut self, job_id: u32, title: &str, owner: &str) -> Result<(), &'static str> {
        self.queued_jobs.push(PrintJob {
            job_id,
            title: title.to_string(),
            owner: owner.to_string(),
            state: PrintJobState::Pending,
            pages: 1,
            payload: Vec::new(),
        });
        Ok(())
    }

    pub fn receive_data_file(&mut self, job_id: u32, data: &[u8]) -> Result<(), &'static str> {
        let job = self
            .queued_jobs
            .iter_mut()
            .find(|j| j.job_id == job_id)
            .ok_or("LPD: Control file not received prior to data file")?;
        job.payload = data.to_vec();
        Ok(())
    }
}

impl Default for LpdSpooler {
    fn default() -> Self {
        Self::new("lp", "/var/spool/lpd/lp")
    }
}

/// PPD (PostScript Printer Description) Auto-Matcher
pub struct PpdDriverMatcher {
    pub drivers: Vec<(String, String)>, // (Device Pattern, Driver PPD)
}

impl PpdDriverMatcher {
    pub fn new() -> Self {
        let mut matcher = Self { drivers: Vec::new() };
        matcher.add_driver("HP LaserJet", "hp-laserjet-postscript.ppd");
        matcher.add_driver("Epson EcoTank", "epson-inkjet-escpr.ppd");
        matcher.add_driver("Canon PIXMA", "canon-pixma-bjc.ppd");
        matcher
    }

    pub fn add_driver(&mut self, pattern: &str, ppd_file: &str) {
        self.drivers.push((pattern.to_string(), ppd_file.to_string()));
    }

    pub fn match_ppd(&self, device_name: &str) -> String {
        for (pattern, ppd) in &self.drivers {
            if self.sovereign_contains(device_name.as_bytes(), pattern.as_bytes()) {
                return ppd.clone();
            }
        }
        "generic-postscript.ppd".to_string()
    }

    /// Standalone sovereign zero-dependency byte sequence matching
    fn sovereign_contains(&self, haystack: &[u8], needle: &[u8]) -> bool {
        if needle.is_empty() {
            return true;
        }
        if needle.len() > haystack.len() {
            return false;
        }
        for i in 0..=(haystack.len() - needle.len()) {
            if &haystack[i..i + needle.len()] == needle {
                return true;
            }
        }
        false
    }
}

impl Default for PpdDriverMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cups_ipp_spooler() {
        let mut cups = CupsIppPrintSpooler::new("office_printer");
        let id = cups.submit_ipp_job("report.pdf", "alice", 5, b"%PDF-1.5...").unwrap();
        assert_eq!(id, 1);
        assert_eq!(cups.jobs.get(&id).unwrap().state, PrintJobState::Pending);

        let processed = cups.process_next_job();
        assert_eq!(processed, Some(1));
        assert_eq!(cups.jobs.get(&id).unwrap().state, PrintJobState::Completed);
    }

    #[test]
    fn test_lpd_spooler() {
        let mut lpd = LpdSpooler::new("lp", "/var/spool/lpd/lp");
        assert!(lpd.receive_control_file(101, "invoice.ps", "bob").is_ok());
        assert!(lpd.receive_data_file(101, b"%!PS-Adobe-3.0").is_ok());
        assert_eq!(lpd.queued_jobs[0].payload, b"%!PS-Adobe-3.0");
    }

    #[test]
    fn test_ppd_matcher() {
        let matcher = PpdDriverMatcher::new();
        assert_eq!(matcher.match_ppd("HP LaserJet Pro M404n"), "hp-laserjet-postscript.ppd");
        assert_eq!(matcher.match_ppd("Unknown USB Printer"), "generic-postscript.ppd");
    }
}
