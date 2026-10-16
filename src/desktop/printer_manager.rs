// SPDX-License-Identifier: MIT
// SigmaOS - Printer Manager
// Inspired by system-config-printer and CUPS
// Provides printer configuration and management

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Printer status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrinterStatus {
    Idle,
    Printing,
    Busy,
    Error,
    Offline,
}

/// Printer connection type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    USB,
    Network,
    Parallel,
    Serial,
    Bluetooth,
}

/// Paper size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaperSize {
    A4,
    Letter,
    Legal,
    A3,
    A5,
    Tabloid,
    Custom,
}

/// Print quality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintQuality {
    Draft,
    Normal,
    High,
    Photo,
}

/// Print job
#[derive(Debug, Clone)]
pub struct PrintJob {
    pub id: u32,
    pub name: String,
    pub document_path: String,
    pub status: PrinterStatus,
    pub pages: u32,
    pub copies: u32,
    pub paper_size: PaperSize,
    pub quality: PrintQuality,
    pub color: bool,
    pub duplex: bool,
}

/// Printer
#[derive(Debug, Clone)]
pub struct Printer {
    pub name: String,
    pub driver: String,
    pub location: String,
    pub connection_type: ConnectionType,
    pub status: PrinterStatus,
    pub default_paper_size: PaperSize,
    pub default_quality: PrintQuality,
    pub supports_color: bool,
    pub supports_duplex: bool,
    pub queue: Vec<PrintJob>,
}

/// Printer manager
pub struct PrinterManager {
    pub printers: BTreeMap<String, Printer>,
    pub default_printer: Option<String>,
    pub next_job_id: u32,
}

impl PrinterManager {
    pub fn new() -> Self {
        Self {
            printers: BTreeMap::new(),
            default_printer: None,
            next_job_id: 1,
        }
    }

    /// Add printer
    pub fn add_printer(
        &mut self,
        name: String,
        driver: String,
        location: String,
        connection_type: ConnectionType,
    ) {
        let printer = Printer {
            name: name.clone(),
            driver,
            location,
            connection_type,
            status: PrinterStatus::Idle,
            default_paper_size: PaperSize::A4,
            default_quality: PrintQuality::Normal,
            supports_color: true,
            supports_duplex: true,
            queue: Vec::new(),
        };
        self.printers.insert(name.clone(), printer);
        if self.default_printer.is_none() {
            self.default_printer = Some(name);
        }
    }

    /// Remove printer
    pub fn remove_printer(&mut self, name: String) -> Result<(), &'static str> {
        if !self.printers.contains_key(&name) {
            return Err("Printer not found");
        }
        self.printers.remove(&name);
        if self.default_printer.as_ref() == Some(&name) {
            self.default_printer = self.printers.keys().next().cloned();
        }
        Ok(())
    }

    /// Set default printer
    pub fn set_default_printer(&mut self, name: String) -> Result<(), &'static str> {
        if !self.printers.contains_key(&name) {
            return Err("Printer not found");
        }
        self.default_printer = Some(name);
        Ok(())
    }

    /// Get printer
    pub fn get_printer(&self, name: &str) -> Option<&Printer> {
        self.printers.get(name)
    }

    /// Get printers
    pub fn get_printers(&self) -> Vec<&Printer> {
        self.printers.values().collect()
    }

    /// Submit print job
    pub fn submit_job(
        &mut self,
        printer_name: String,
        document_path: String,
        pages: u32,
        copies: u32,
    ) -> Result<u32, &'static str> {
        let printer = self
            .printers
            .get_mut(&printer_name)
            .ok_or("Printer not found")?;
        let job = PrintJob {
            id: self.next_job_id,
            name: format!("Job {}", self.next_job_id),
            document_path,
            status: PrinterStatus::Idle,
            pages,
            copies,
            paper_size: printer.default_paper_size,
            quality: printer.default_quality,
            color: printer.supports_color,
            duplex: printer.supports_duplex,
        };
        printer.queue.push(job);
        let job_id = self.next_job_id;
        self.next_job_id += 1;
        Ok(job_id)
    }

    /// Cancel job
    pub fn cancel_job(&mut self, printer_name: String, job_id: u32) -> Result<(), &'static str> {
        let printer = self
            .printers
            .get_mut(&printer_name)
            .ok_or("Printer not found")?;
        let index = printer
            .queue
            .iter()
            .position(|j| j.id == job_id)
            .ok_or("Job not found")?;
        printer.queue.remove(index);
        Ok(())
    }

    /// Get jobs for printer
    pub fn get_jobs(&self, printer_name: &str) -> Vec<&PrintJob> {
        self.printers
            .get(printer_name)
            .map(|p| p.queue.iter().collect())
            .unwrap_or_default()
    }

    /// Printer count
    pub fn printer_count(&self) -> usize {
        self.printers.len()
    }
}

/// Global printer manager
static mut PRINTER_MANAGER: Option<PrinterManager> = None;

/// Initialize printer manager
pub fn init_printer_manager() {
    unsafe {
        PRINTER_MANAGER = Some(PrinterManager::new());
    }
}

/// Get printer manager
pub fn get_printer_manager() -> &'static mut PrinterManager {
    unsafe {
        PRINTER_MANAGER
            .as_mut()
            .expect("Printer manager not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printer_manager_creation() {
        let pm = PrinterManager::new();
        assert_eq!(pm.printer_count(), 0);
    }

    #[test]
    fn test_add_printer() {
        let mut pm = PrinterManager::new();
        pm.add_printer(
            "HP LaserJet".to_string(),
            "hplip".to_string(),
            "Office".to_string(),
            ConnectionType::USB,
        );
        assert_eq!(pm.printer_count(), 1);
    }

    #[test]
    fn test_submit_job() {
        let mut pm = PrinterManager::new();
        pm.add_printer(
            "HP LaserJet".to_string(),
            "hplip".to_string(),
            "Office".to_string(),
            ConnectionType::USB,
        );
        let job_id = pm
            .submit_job(
                "HP LaserJet".to_string(),
                "/path/to/doc.pdf".to_string(),
                5,
                1,
            )
            .unwrap();
        assert_eq!(job_id, 1);
    }
}
