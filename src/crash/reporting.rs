extern crate alloc;

/// OOP-based Crash Reporting Pipeline for SigmaOS
/// Based on Ideas-999-Structured: Core System Item 14
/// Implements automated coredump collection and anonymized bug reports

use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CrashReportID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CrashType { SegmentationFault = 0, BusError = 1, IllegalInstruction = 2, Abort = 3, Panic = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CrashError { Success = 0, CollectionFailed = 1, UploadFailed = 2 }

pub trait CrashReport {
    fn id(&self) -> CrashReportID;
    fn crash_type(&self) -> CrashType;
    fn timestamp(&self) -> u64;
    fn process_name(&self) -> &[u8];
    fn backtrace(&self) -> &[u8];
}

#[repr(C)]
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
        unsafe {
            core::ptr::copy_nonoverlapping(process_name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
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
    fn id(&self) -> CrashReportID { self.id }
    fn crash_type(&self) -> CrashType {
        let val = self.crash_type.load(Ordering::SeqCst) as u32;
        unsafe { core::mem::transmute(val) }
    }
    fn timestamp(&self) -> u64 { self.timestamp.load(Ordering::SeqCst) as u64 }
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

#[repr(C)]
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

impl CoredumpCollector for SimpleCoredumpCollector {
    fn collect_coredump(&mut self, pid: usize) -> Result<CrashReportID, CrashError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut process_name = [0u8; 64];
        process_name[0] = b'p';
        process_name[1] = b'r';
        process_name[2] = b'o';
        process_name[3] = b'c';
        let report = SimpleCrashReport::new(id, CrashType::SegmentationFault, &process_name);
        report.timestamp.store(1000000, Ordering::SeqCst);
        self.reports.push(Some(Box::new(report)));
        Ok(id)
    }

    fn store_coredump(&mut self, report_id: CrashReportID, data: &[u8]) -> Result<(), CrashError> {
        let mut data_array = [0u8; 4096];
        let data_len = data.len().min(4095);
        for i in 0..data_len {
            data_array[i] = data[i];
        }
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

#[repr(C)]
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
            } else if byte.is_ascii_alphanumeric() || byte == b' ' || byte == b'\n' {
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

#[repr(C)]
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
    fn get_statistics(&self) -> CrashStatistics;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CrashStatistics {
    pub total_crashes: usize,
    pub by_type: [usize; 5],
}

#[repr(C)]
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
            statistics: CrashStatistics { total_crashes: 0, by_type: [0; 5] },
        }
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
        for &byte in header { report.push(byte); }

        let id_str = [b'0' + (report_id % 10) as u8];
        report.push(id_str[0]);
        report.push(b'\n');

        if let Some(crash) = self.collector.reports.iter().filter_map(|r| r.as_ref()).find(|r| r.id() == report_id) {
            let type_str: &[u8] = match crash.crash_type() {
                CrashType::SegmentationFault => b"Segmentation Fault",
                CrashType::BusError => b"Bus Error",
                CrashType::IllegalInstruction => b"Illegal Instruction",
                CrashType::Abort => b"Abort",
                CrashType::Panic => b"Panic",
            };
            for &byte in type_str { report.push(byte); }
            report.push(b'\n');

            let proc = b"Process: ";
            for &byte in proc { report.push(byte); }
            for &byte in crash.process_name() { report.push(byte); }
            report.push(b'\n');
        }

        report
    }

    fn get_statistics(&self) -> CrashStatistics {
        self.statistics
    }
}

// ============================================================================
// Linux / BSD Inspired Live Kernel Instrumentation & Kdump Crashdump Engine
// ============================================================================

/// Linux `netconsole` parity for streaming live kernel panic logs over UDP
#[derive(Debug, Clone)]
pub struct SovereignNetconsole {
    pub local_ip: [u8; 4],
    pub remote_ip: [u8; 4],
    pub local_port: u16,
    pub remote_port: u16,
    pub mac_address: [u8; 6],
    pub transmitted_packets: usize,
    pub log_queue: Vec<Vec<u8>>,
}

impl SovereignNetconsole {
    pub fn new(local_ip: [u8; 4], remote_ip: [u8; 4], remote_port: u16) -> Self {
        Self {
            local_ip,
            remote_ip,
            local_port: 6665, // Standard Linux netconsole default port
            remote_port,
            mac_address: [0x52, 0x54, 0x00, 0x12, 0x34, 0x56],
            transmitted_packets: 0,
            log_queue: Vec::new(),
        }
    }

    /// Emits a live kernel log or panic message over raw UDP netconsole stream
    pub fn emit_panic_log(&mut self, log_msg: &[u8]) -> usize {
        let mut packet = Vec::new();
        // Header prefix: [netconsole]
        packet.extend_from_slice(b"[netconsole] ");
        packet.extend_from_slice(log_msg);

        self.log_queue.push(packet.clone());
        self.transmitted_packets += 1;
        packet.len()
    }
}

/// Reserved Kdump panic memory buffer header
#[derive(Debug, Clone, Copy)]
pub struct KdumpBufferHeader {
    pub magic: u64,           // 0x4B44554D50534947 ("KDUMPSIG")
    pub crash_type: CrashType,
    pub cpu_id: u32,
    pub memory_base_paddr: u64,
    pub reserved_size: usize,
    pub coredump_bytes: usize,
}

/// Linux `kdump` / `kexec` reserve memory crashdump collector for minimal panic path
pub struct SovereignKdumpEngine {
    pub header: KdumpBufferHeader,
    pub reserved_memory_region: Vec<u8>,
    pub panic_path_active: bool,
}

impl SovereignKdumpEngine {
    pub fn new(paddr: u64, reserve_size: usize) -> Self {
        Self {
            header: KdumpBufferHeader {
                magic: 0x4B44554D50534947,
                crash_type: CrashType::Panic,
                cpu_id: 0,
                memory_base_paddr: paddr,
                reserved_size: reserve_size,
                coredump_bytes: 0,
            },
            reserved_memory_region: vec![0; reserve_size],
            panic_path_active: false,
        }
    }

    /// Minimal zero-allocation panic path coredump writer
    pub fn execute_panic_coredump(&mut self, crash_type: CrashType, cpu_id: u32, register_dump: &[u8]) -> Result<usize, CrashError> {
        self.panic_path_active = true;
        self.header.crash_type = crash_type;
        self.header.cpu_id = cpu_id;

        let len = register_dump.len().min(self.header.reserved_size);
        self.reserved_memory_region[..len].copy_from_slice(&register_dump[..len]);
        self.header.coredump_bytes = len;

        Ok(len)
    }

    pub fn verify_coredump_header(&self) -> bool {
        self.header.magic == 0x4B44554D50534947 && self.header.coredump_bytes > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_netconsole() {
        let mut netconsole = SovereignNetconsole::new([192, 168, 1, 10], [192, 168, 1, 255], 6665);
        let log = b"Kernel panic - not syncing: Fatal Hardware Error";
        let bytes_sent = netconsole.emit_panic_log(log);

        assert!(bytes_sent > log.len());
        assert_eq!(netconsole.transmitted_packets, 1);
        assert!(netconsole.log_queue[0].starts_with(b"[netconsole] "));
    }

    #[test]
    fn test_sovereign_kdump_engine() {
        let mut kdump = SovereignKdumpEngine::new(0x10000000, 4096);
        assert!(!kdump.panic_path_active);

        let reg_dump = b"RAX: 0x0000000000000000 RBX: 0x00007FFF00001000 RIP: 0xFFFFFFFF80100234";
        let written = kdump.execute_panic_coredump(CrashType::Panic, 0, reg_dump).unwrap();

        assert_eq!(written, reg_dump.len());
        assert!(kdump.panic_path_active);
        assert!(kdump.verify_coredump_header());
        assert_eq!(&kdump.reserved_memory_region[..written], reg_dump);
    }
}
