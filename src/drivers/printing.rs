// SigmaOS Sovereign Printing Subsystem
// Linux & BSD inspired print spooling and driver infrastructure
// - CUPS IPP 2.0 (Internet Printing Protocol) daemon and job queue
// - BSD LPD (Line Printer Daemon RFC 1179) remote print spooler
// - PPD (PostScript Printer Description) file auto-matching engine

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintJobState {
    Pending,
    Processing,
    Completed,
    Canceled,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintJob {
    pub job_id: u32,
    pub title: String,
    pub user: String,
    pub printer_name: String,
    pub state: PrintJobState,
    pub page_count: u32,
    pub payload_bytes: Vec<u8>,
}

/// CUPS IPP 2.0 (Internet Printing Protocol) Spooler
#[derive(Debug, Clone)]
pub struct CupsIppPrintSpooler {
    pub next_job_id: u32,
    pub jobs: BTreeMap<u32, PrintJob>,
    pub active_printers: Vec<String>,
}

impl CupsIppPrintSpooler {
    pub fn new() -> Self {
        Self {
            next_job_id: 1,
            jobs: BTreeMap::new(),
            active_printers: vec![
                "CUPS-PDF-Virtual".to_string(),
                "HP-LaserJet-IPP".to_string(),
            ],
        }
    }

    pub fn submit_ipp_job(
        &mut self,
        printer_name: &str,
        title: &str,
        user: &str,
        payload_bytes: &[u8],
    ) -> Result<u32, &'static str> {
        if !self.active_printers.iter().any(|p| p == printer_name) {
            return Err("CUPS: Requested IPP printer target not found");
        }

        let job_id = self.next_job_id;
        self.next_job_id += 1;

        let job = PrintJob {
            job_id,
            title: title.to_string(),
            user: user.to_string(),
            printer_name: printer_name.to_string(),
            state: PrintJobState::Pending,
            page_count: ((payload_bytes.len() / 4096) + 1) as u32,
            payload_bytes: payload_bytes.to_vec(),
        };

        self.jobs.insert(job_id, job);
        Ok(job_id)
    }

    pub fn process_next_job(&mut self, printer_name: &str) -> Option<u32> {
        let mut target_id = None;
        for (job_id, job) in self.jobs.iter_mut() {
            if job.printer_name == printer_name && job.state == PrintJobState::Pending {
                job.state = PrintJobState::Processing;
                target_id = Some(*job_id);
                break;
            }
        }

        if let Some(job_id) = target_id {
            if let Some(job) = self.jobs.get_mut(&job_id) {
                job.state = PrintJobState::Completed;
            }
        }
        target_id
    }

    pub fn cancel_job(&mut self, job_id: u32) -> bool {
        if let Some(job) = self.jobs.get_mut(&job_id) {
            if job.state == PrintJobState::Pending || job.state == PrintJobState::Processing {
                job.state = PrintJobState::Canceled;
                return true;
            }
        }
        false
    }
}

impl Default for CupsIppPrintSpooler {
    fn default() -> Self {
        Self::new()
    }
}

/// BSD LPD (Line Printer Daemon - RFC 1179) Spooler
#[derive(Debug, Clone)]
pub struct LpdSpooler {
    pub spool_directory: String,
    pub queued_jobs: Vec<PrintJob>,
}

impl LpdSpooler {
    pub fn new(spool_directory: &str) -> Self {
        Self {
            spool_directory: spool_directory.to_string(),
            queued_jobs: Vec::new(),
        }
    }

    pub fn receive_control_and_data_file(
        &mut self,
        queue_name: &str,
        user: &str,
        control_file: &str,
        data_bytes: &[u8],
    ) -> u32 {
        let job_id = (self.queued_jobs.len() as u32) + 100;
        let job = PrintJob {
            job_id,
            title: format!("LPD-Job-{}", control_file),
            user: user.to_string(),
            printer_name: queue_name.to_string(),
            state: PrintJobState::Pending,
            page_count: 1,
            payload_bytes: data_bytes.to_vec(),
        };

        self.queued_jobs.push(job);
        job_id
    }
}

/// PPD (PostScript Printer Description) Auto-Matcher
#[derive(Debug, Clone)]
pub struct PpdDriverMatcher {
    pub database: BTreeMap<String, String>, // IEEE 1284 Device ID -> PPD File
}

impl PpdDriverMatcher {
    pub fn new() -> Self {
        let mut db = BTreeMap::new();
        db.insert(
            "MFG:HP;MDL:LaserJet 1020;".to_string(),
            "/usr/share/ppd/hp/HP-LaserJet-1020.ppd".to_string(),
        );
        db.insert(
            "MFG:EPSON;MDL:Ecotank L3150;".to_string(),
            "/usr/share/ppd/epson/Epson-Ecotank-L3150.ppd".to_string(),
        );

        Self { database: db }
    }

    pub fn match_ppd_for_device_id(&self, ieee1284_id: &str) -> Option<String> {
        for (device_pattern, ppd_path) in &self.database {
            if ieee1284_id.contains(device_pattern) || device_pattern.contains(ieee1284_id) {
                return Some(ppd_path.clone());
            }
        }
        Some("/usr/share/ppd/generic/Generic-PostScript.ppd".to_string())
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
    fn test_cups_ipp_job_queueing() {
        let mut cups = CupsIppPrintSpooler::new();
        let job_id = cups
            .submit_ipp_job("CUPS-PDF-Virtual", "Report.pdf", "alice", b"%PDF-1.4...")
            .unwrap();

        assert_eq!(job_id, 1);
        assert_eq!(cups.jobs.get(&job_id).unwrap().state, PrintJobState::Pending);

        let processed = cups.process_next_job("CUPS-PDF-Virtual");
        assert_eq!(processed, Some(1));
        assert_eq!(cups.jobs.get(&job_id).unwrap().state, PrintJobState::Completed);
    }

    #[test]
    fn test_lpd_spooling() {
        let mut lpd = LpdSpooler::new("/var/spool/lpd");
        let job_id = lpd.receive_control_and_data_file("lp0", "bob", "cfA001sigmaos", b"Hello LPD");

        assert_eq!(job_id, 100);
        assert_eq!(lpd.queued_jobs.len(), 1);
        assert_eq!(lpd.queued_jobs[0].printer_name, "lp0");
    }

    #[test]
    fn test_ppd_driver_matching() {
        let matcher = PpdDriverMatcher::new();
        let hp_ppd = matcher.match_ppd_for_device_id("MFG:HP;MDL:LaserJet 1020;").unwrap();
        assert!(hp_ppd.contains("HP-LaserJet-1020.ppd"));

        let generic_ppd = matcher.match_ppd_for_device_id("MFG:Unknown;MDL:X1;").unwrap();
        assert!(generic_ppd.contains("Generic-PostScript.ppd"));
    }
}
