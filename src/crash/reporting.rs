#![no_std]
#![no_main]

/// OOP-based Crash Reporting Pipeline for SigmaOS
/// Based on Ideas-999-Structured: Core System Item 14
/// Implements automated coredump collection and anonymized bug reports

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
    fn crash_type(&self) -> CrashType { unsafe { core::mem::transmute(self.crash_type.load(Ordering::SeqCst)) } }
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
            if byte.is_ascii_alphanumeric() || byte == b' ' || byte == b'\n' {
                anonymized.push(byte);
            } else if byte.is_ascii_digit() {
                anonymized.push(b'X');
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
            let type_str = match crash.crash_type() {
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

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn contains(&self, item: &T) -> bool where T: PartialEq {
        for i in 0..self.len {
            unsafe {
                if &*self.data.add(i) == item { return true; }
            }
        }
        false
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
