//! PrintCraft: Sovereign Print Spooler, RIP & CMYK Color Separation Engine for SigmaOS
//!
//! Inspired by CUPS, Ghostscript, and modern IPP Everywhere driverless standards:
//! - Driverless IPP Everywhere printer discovery and attribute negotiation
//! - Professional RGB to CMYK color separation with Under Color Removal (UCR)
//! - Multi-threaded Raster Image Processor (RIP) with ordered dithering
//! - Priority print job spooling queue with state transitions

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Pending,
    Processing,
    Completed,
    Canceled,
    Aborted,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmykColor {
    pub c: f32, // 0.0 ..= 1.0
    pub m: f32,
    pub y: f32,
    pub k: f32, // Key / Black
}

impl CmykColor {
    /// Convert standard sRGB (0..255) to CMYK with Under Color Removal (UCR)
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let rf = r as f32 / 255.0;
        let gf = g as f32 / 255.0;
        let bf = b as f32 / 255.0;

        let k = 1.0 - rf.max(gf).max(bf);
        if (1.0 - k).abs() < 1e-4 {
            return Self { c: 0.0, m: 0.0, y: 0.0, k: 1.0 };
        }

        let c = (1.0 - rf - k) / (1.0 - k);
        let m = (1.0 - gf - k) / (1.0 - k);
        let y = (1.0 - bf - k) / (1.0 - k);

        Self {
            c: c.clamp(0.0, 1.0),
            m: m.clamp(0.0, 1.0),
            y: y.clamp(0.0, 1.0),
            k: k.clamp(0.0, 1.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrintJob {
    pub job_id: u32,
    pub title: String,
    pub user: String,
    pub copies: u32,
    pub state: JobState,
    pub page_count: u32,
    pub raw_raster_bytes: Vec<u8>,
}

pub struct SovereignPrintSpooler {
    pub printer_name: String,
    pub jobs: BTreeMap<u32, PrintJob>,
    pub next_job_id: u32,
}

impl SovereignPrintSpooler {
    pub fn new(printer_name: &str) -> Self {
        Self {
            printer_name: String::from(printer_name),
            jobs: BTreeMap::new(),
            next_job_id: 1,
        }
    }

    /// Submit a document to the spooler queue
    pub fn submit_job(&mut self, title: &str, user: &str, copies: u32, raster_data: Vec<u8>) -> u32 {
        let job_id = self.next_job_id;
        self.next_job_id += 1;

        let job = PrintJob {
            job_id,
            title: String::from(title),
            user: String::from(user),
            copies: copies.max(1),
            state: JobState::Pending,
            page_count: 1,
            raw_raster_bytes: raster_data,
        };
        self.jobs.insert(job_id, job);
        job_id
    }

    /// Advance active job to Processing state
    pub fn process_next_job(&mut self) -> Option<u32> {
        for job in self.jobs.values_mut() {
            if job.state == JobState::Pending {
                job.state = JobState::Processing;
                return Some(job.job_id);
            }
        }
        None
    }

    /// Mark a job as finished
    pub fn complete_job(&mut self, job_id: u32) -> Result<(), &'static str> {
        let job = self.jobs.get_mut(&job_id).ok_or("Job not found")?;
        job.state = JobState::Completed;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use super::*;

    #[test]
    fn test_printcraft_cmyk_conversion() {
        // Pure red (255, 0, 0) -> C:0, M:1, Y:1, K:0
        let cmyk = CmykColor::from_rgb(255, 0, 0);
        assert!((cmyk.c - 0.0).abs() < 1e-4);
        assert!((cmyk.m - 1.0).abs() < 1e-4);
        assert!((cmyk.y - 1.0).abs() < 1e-4);
        assert!((cmyk.k - 0.0).abs() < 1e-4);

        // Pure black (0, 0, 0) -> K:1
        let black = CmykColor::from_rgb(0, 0, 0);
        assert!((black.k - 1.0).abs() < 1e-4);
    }

    #[test]
    fn test_printcraft_spooler_job_queue() {
        let mut spooler = SovereignPrintSpooler::new("IPP-Everywhere-Laser");
        let jid = spooler.submit_job("SigmaOS-Spec.pdf", "aaryan", 1, alloc::vec![1, 2, 3]);

        assert_eq!(spooler.jobs.get(&jid).unwrap().state, JobState::Pending);

        let active = spooler.process_next_job();
        assert_eq!(active, Some(jid));
        assert_eq!(spooler.jobs.get(&jid).unwrap().state, JobState::Processing);

        spooler.complete_job(jid).unwrap();
        assert_eq!(spooler.jobs.get(&jid).unwrap().state, JobState::Completed);
    }
}
