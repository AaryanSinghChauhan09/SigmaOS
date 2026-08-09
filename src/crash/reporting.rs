// OOP-based Crash Reporting and Kernel Oops/Panic Dump Pipeline for SigmaOS
// Inspired by Linux kernel oops/panic dumps and BSD core dumps, implementing system stability logs.

extern crate alloc;

use crate::kernel::virtual_cpu::RegisterSet;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type CrashReportID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrashType {
    SegmentationFault = 0,
    BusError = 1,
    IllegalInstruction = 2,
    Abort = 3,
    Panic = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrashError {
    Success = 0,
    CollectionFailed = 1,
    UploadFailed = 2,
}

pub trait CrashReport {
    fn id(&self) -> CrashReportID;
    fn crash_type(&self) -> CrashType;
    fn timestamp(&self) -> u64;
    fn process_name(&self) -> &[u8];
    fn backtrace(&self) -> &[u8];
}

pub struct SimpleCrashReport {
    pub id: CrashReportID,
    pub crash_type: AtomicUsize,
    pub timestamp: AtomicUsize,
    pub process_name: [u8; 64],
    pub backtrace: [u8; 512],
}

impl SimpleCrashReport {
    pub fn new(id: CrashReportID, crash_type: CrashType, process_name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = process_name.len().min(63);
        name_array[..name_len].copy_from_slice(&process_name[..name_len]);

        SimpleCrashReport {
            id,
            crash_type: AtomicUsize::new(crash_type as usize),
            timestamp: AtomicUsize::new(0),
            process_name: name_array,
            backtrace: [0u8; 512],
        }
    }
}

impl CrashReport for SimpleCrashReport {
    fn id(&self) -> CrashReportID {
        self.id
    }
    fn crash_type(&self) -> CrashType {
        match self.crash_type.load(Ordering::SeqCst) {
            0 => CrashType::SegmentationFault,
            1 => CrashType::BusError,
            2 => CrashType::IllegalInstruction,
            3 => CrashType::Abort,
            _ => CrashType::Panic,
        }
    }
    fn timestamp(&self) -> u64 {
        self.timestamp.load(Ordering::SeqCst) as u64
    }
    fn process_name(&self) -> &[u8] {
        let len = self.process_name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.process_name[..len]
    }
    fn backtrace(&self) -> &[u8] {
        let len = self.backtrace.iter().position(|&b| b == 0).unwrap_or(512);
        &self.backtrace[..len]
    }
}

pub trait CoredumpCollector {
    fn collect_coredump(&mut self, pid: usize) -> Result<CrashReportID, CrashError>;
    fn store_coredump(&mut self, report_id: CrashReportID, data: &[u8]) -> Result<(), CrashError>;
    fn get_coredump(&self, report_id: CrashReportID) -> Option<&[u8]>;
}

pub struct SimpleCoredumpCollector {
    pub reports: Vec<Option<Box<dyn CrashReport>>>,
    pub coredumps: Vec<(CrashReportID, [u8; 4096])>,
    pub next_id: AtomicUsize,
}

impl SimpleCoredumpCollector {
    pub fn new() -> Self {
        SimpleCoredumpCollector {
            reports: Vec::new(),
            coredumps: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Default for SimpleCoredumpCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl CoredumpCollector for SimpleCoredumpCollector {
    fn collect_coredump(&mut self, _pid: usize) -> Result<CrashReportID, CrashError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut process_name = [0u8; 64];
        process_name[..4].copy_from_slice(b"proc");
        let report = SimpleCrashReport::new(id, CrashType::SegmentationFault, &process_name);
        report.timestamp.store(1000000, Ordering::SeqCst);
        self.reports.push(Some(Box::new(report)));
        Ok(id)
    }

    fn store_coredump(&mut self, report_id: CrashReportID, data: &[u8]) -> Result<(), CrashError> {
        let mut data_array = [0u8; 4096];
        let data_len = data.len().min(4095);
        data_array[..data_len].copy_from_slice(&data[..data_len]);
        self.coredumps.push((report_id, data_array));
        Ok(())
    }

    fn get_coredump(&self, report_id: CrashReportID) -> Option<&[u8]> {
        for &(id, ref data) in &self.coredumps {
            if id == report_id {
                let len = data.iter().position(|&b| b == 0).unwrap_or(4096);
                return Some(&data[..len]);
            }
        }
        None
    }
}

pub trait Anonymizer {
    fn anonymize_report(&mut self, report_id: CrashReportID) -> Result<(), CrashError>;
    fn strip_pii(&self, data: &[u8]) -> Vec<u8>;
}

pub struct SimpleAnonymizer {
    pub collector: SimpleCoredumpCollector,
}

impl SimpleAnonymizer {
    pub fn new(collector: SimpleCoredumpCollector) -> Self {
        SimpleAnonymizer { collector }
    }
}

impl Anonymizer for SimpleAnonymizer {
    fn anonymize_report(&mut self, _report_id: CrashReportID) -> Result<(), CrashError> {
        Ok(())
    }

    fn strip_pii(&self, data: &[u8]) -> Vec<u8> {
        let mut anonymized = Vec::new();
        for &byte in data {
            if byte.is_ascii_digit() {
                anonymized.push(b'X');
            } else {
                anonymized.push(byte);
            }
        }
        anonymized
    }
}

pub trait CrashUploader {
    fn upload_report(&mut self, report_id: CrashReportID) -> Result<(), CrashError>;
    fn get_upload_status(&self, report_id: CrashReportID) -> bool;
}

pub struct SimpleCrashUploader {
    pub uploaded_reports: Vec<CrashReportID>,
}

impl SimpleCrashUploader {
    pub fn new() -> Self {
        SimpleCrashUploader {
            uploaded_reports: Vec::new(),
        }
    }
}

impl Default for SimpleCrashUploader {
    fn default() -> Self {
        Self::new()
    }
}

impl CrashUploader for SimpleCrashUploader {
    fn upload_report(&mut self, report_id: CrashReportID) -> Result<(), CrashError> {
        self.uploaded_reports.push(report_id);
        Ok(())
    }

    fn get_upload_status(&self, report_id: CrashReportID) -> bool {
        self.uploaded_reports.contains(&report_id)
    }
}

pub trait CrashPipeline {
    fn process_crash(&mut self, pid: usize) -> Result<CrashReportID, CrashError>;
    fn generate_report(&self, report_id: CrashReportID) -> Vec<u8>;
    fn generate_linux_grade_panic_dump(
        &self,
        report_id: CrashReportID,
        regs: &RegisterSet,
    ) -> String;
    fn get_statistics(&self) -> CrashStatistics;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrashStatistics {
    pub total_crashes: usize,
    pub by_type: [usize; 5],
}

pub struct SimpleCrashPipeline {
    pub collector: SimpleCoredumpCollector,
    pub anonymizer: SimpleAnonymizer,
    pub uploader: SimpleCrashUploader,
    pub statistics: CrashStatistics,
}

impl SimpleCrashPipeline {
    pub fn new() -> Self {
        let collector = SimpleCoredumpCollector::new();
        let anonymizer = SimpleAnonymizer::new(SimpleCoredumpCollector::new());
        let uploader = SimpleCrashUploader::new();
        SimpleCrashPipeline {
            collector,
            anonymizer,
            uploader,
            statistics: CrashStatistics {
                total_crashes: 0,
                by_type: [0; 5],
            },
        }
    }
}

impl Default for SimpleCrashPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl CrashPipeline for SimpleCrashPipeline {
    fn process_crash(&mut self, pid: usize) -> Result<CrashReportID, CrashError> {
        let report_id = self.collector.collect_coredump(pid)?;
        self.anonymizer.anonymize_report(report_id)?;
        self.uploader.upload_report(report_id)?;

        self.statistics.total_crashes += 1;

        Ok(report_id)
    }

    fn generate_report(&self, report_id: CrashReportID) -> Vec<u8> {
        let mut report = Vec::new();
        let header = b"Crash Report #";
        report.extend_from_slice(header);

        let id_str = [b'0' + (report_id % 10) as u8];
        report.push(id_str[0]);
        report.push(b'\n');

        if let Some(crash) = self
            .collector
            .reports
            .iter()
            .filter_map(|r| r.as_ref())
            .find(|r| r.id() == report_id)
        {
            let type_str: &[u8] = match crash.crash_type() {
                CrashType::SegmentationFault => b"Segmentation Fault",
                CrashType::BusError => b"Bus Error",
                CrashType::IllegalInstruction => b"Illegal Instruction",
                CrashType::Abort => b"Abort",
                CrashType::Panic => b"Panic",
            };
            report.extend_from_slice(type_str);
            report.push(b'\n');

            let proc = b"Process: ";
            report.extend_from_slice(proc);
            report.extend_from_slice(crash.process_name());
            report.push(b'\n');
        }

        report
    }

    /// Automatically generates a professional Linux-grade kernel panic Oops register state dump
    fn generate_linux_grade_panic_dump(
        &self,
        report_id: CrashReportID,
        regs: &RegisterSet,
    ) -> String {
        let mut process_name = "unknown";
        if let Some(crash) = self
            .collector
            .reports
            .iter()
            .filter_map(|r| r.as_ref())
            .find(|r| r.id() == report_id)
        {
            if let Ok(name) = core::str::from_utf8(crash.process_name()) {
                process_name = name;
            }
        }

        format!(
            "----------------[ CUT HERE ]----------------\n\
             KERNEL PANIC: Oops: 0000 [#1] SMP NOPTI\n\
             CPU: 0 PID: {} Comm: {} Not tainted\n\
             RIP: 0010:0x{:016X}\n\
             RSP: 0018:0x{:016X}\n\
             RAX: {:016X} RBX: {:016X} RCX: {:016X}\n\
             RDX: {:016X} CR0: {:016X} CR3: {:016X}\n\
             ---[ END TRACE ]---\n",
            report_id,
            process_name,
            regs.rip,
            regs.rsp,
            regs.rax,
            regs.rbx,
            regs.rcx,
            regs.rdx,
            regs.cr0,
            regs.cr3
        )
    }

    fn get_statistics(&self) -> CrashStatistics {
        self.statistics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crash_reporting_and_pii_anonymization() {
        let mut pipeline = SimpleCrashPipeline::new();
        let report_id = pipeline.process_crash(155).unwrap();

        assert_eq!(pipeline.get_statistics().total_crashes, 1);

        let report_bytes = pipeline.generate_report(report_id);
        assert!(report_bytes.len() > 0);

        // Test stripping PII (digits to 'X')
        let raw_ip = b"IP: 192.168.1.100";
        let clean_ip = pipeline.anonymizer.strip_pii(raw_ip);
        assert_eq!(clean_ip, b"IP: XXX.XXX.X.XXX".to_vec());
    }

    #[test]
    fn test_linux_grade_panic_register_dump() {
        let mut pipeline = SimpleCrashPipeline::new();
        let report_id = pipeline.process_crash(42).unwrap();

        let regs = RegisterSet {
            rax: 0xDEADBEEF,
            rbx: 0xCAFEBABE,
            rcx: 0x1234,
            rdx: 0x5678,
            rdi: 0,
            rsi: 0,
            cr0: 0x80000011,
            cr3: 0x100000,
            cr4: 0,
            rip: 0xFFFFFFFF80001000,
            rsp: 0xFFFF880000001000,
        };

        let dump = pipeline.generate_linux_grade_panic_dump(report_id, &regs);
        assert!(dump.contains("KERNEL PANIC: Oops"));
        assert!(dump.contains("RIP: 0010:0xFFFFFFFF80001000"));
        assert!(dump.contains("RAX: 00000000DEADBEEF"));
        assert!(dump.contains("CR0: 0000000080000011"));
        assert!(dump.contains("---[ END TRACE ]---"));
    }
}
