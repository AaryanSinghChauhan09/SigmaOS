#![no_std]
#![no_main]

/// OOP-based Print Driver for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 301
/// Implements printer management and job queue

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PrinterID = usize;
pub type JobID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PrinterState { Idle = 0, Printing = 1, Error = 2, Offline = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PrintError { Success = 0, NotFound = 1, JobFailed = 2 }

pub trait Printer {
    fn id(&self) -> PrinterID;
    fn name(&self) -> &[u8];
    fn state(&self) -> PrinterState;
    fn set_state(&mut self, state: PrinterState);
}

#[repr(C)]
pub struct SimplePrinter {
    pub id: PrinterID,
    pub name: [u8; 64],
    pub state: AtomicUsize,
}

impl SimplePrinter {
    pub fn new(id: PrinterID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimplePrinter {
            id,
            name: name_array,
            state: AtomicUsize::new(PrinterState::Idle as usize),
        }
    }
}

impl Printer for SimplePrinter {
    fn id(&self) -> PrinterID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn state(&self) -> PrinterState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }

    fn set_state(&mut self, state: PrinterState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

pub trait PrintJob {
    fn id(&self) -> JobID;
    fn printer_id(&self) -> PrinterID;
    fn document(&self) -> &[u8];
    fn pages(&self) -> u32;
    fn is_complete(&self) -> bool;
}

#[repr(C)]
pub struct SimplePrintJob {
    pub id: JobID,
    pub printer_id: PrinterID,
    pub document: [u8; 256],
    pub pages: AtomicUsize,
    pub complete: AtomicUsize,
}

impl SimplePrintJob {
    pub fn new(id: JobID, printer_id: PrinterID, document: &[u8], pages: u32) -> Self {
        let mut doc_array = [0u8; 256];
        let doc_len = document.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(document.as_ptr(), doc_array.as_mut_ptr(), doc_len);
        }
        SimplePrintJob {
            id,
            printer_id,
            document: doc_array,
            pages: AtomicUsize::new(pages as usize),
            complete: AtomicUsize::new(0),
        }
    }
}

impl PrintJob for SimplePrintJob {
    fn id(&self) -> JobID { self.id }
    fn printer_id(&self) -> PrinterID { self.printer_id }
    fn document(&self) -> &[u8] {
        let len = self.document.iter().position(|&b| b == 0).unwrap_or(256);
        &self.document[..len]
    }
    fn pages(&self) -> u32 { self.pages.load(Ordering::SeqCst) as u32 }
    fn is_complete(&self) -> bool { self.complete.load(Ordering::SeqCst) == 1 }
}

pub trait PrintManager {
    fn add_printer(&mut self, printer: Box<dyn Printer>) -> Result<PrinterID, PrintError>;
    fn remove_printer(&mut self, id: PrinterID) -> Result<(), PrintError>;
    fn submit_job(&mut self, printer_id: PrinterID, document: &[u8], pages: u32) -> Result<JobID, PrintError>;
    fn cancel_job(&mut self, job_id: JobID) -> Result<(), PrintError>;
    fn get_job_status(&self, job_id: JobID) -> Option<&dyn PrintJob>;
}

#[repr(C)]
pub struct SimplePrintManager {
    pub printers: Vec<Option<Box<dyn Printer>>>,
    pub jobs: Vec<Option<Box<dyn PrintJob>>>,
    pub next_printer_id: AtomicUsize,
    pub next_job_id: AtomicUsize,
}

impl SimplePrintManager {
    pub fn new() -> Self {
        SimplePrintManager {
            printers: Vec::new(),
            jobs: Vec::new(),
            next_printer_id: AtomicUsize::new(1),
            next_job_id: AtomicUsize::new(1),
        }
    }
}

impl PrintManager for SimplePrintManager {
    fn add_printer(&mut self, printer: Box<dyn Printer>) -> Result<PrinterID, PrintError> {
        let id = printer.id();
        self.printers.push(Some(printer));
        Ok(id)
    }

    fn remove_printer(&mut self, id: PrinterID) -> Result<(), PrintError> {
        for printer_option in &mut self.printers {
            if let Some(ref printer) = *printer_option {
                if printer.id() == id {
                    return Ok(());
                }
            }
        }
        Err(PrintError::NotFound)
    }

    fn submit_job(&mut self, printer_id: PrinterID, document: &[u8], pages: u32) -> Result<JobID, PrintError> {
        let id = self.next_job_id.fetch_add(1, Ordering::SeqCst);
        let job = SimplePrintJob::new(id, printer_id, document, pages);
        self.jobs.push(Some(Box::new(job)));
        Ok(id)
    }

    fn cancel_job(&mut self, job_id: JobID) -> Result<(), PrintError> {
        for job_option in &mut self.jobs {
            if let Some(ref job) = *job_option {
                if job.id() == job_id {
                    return Ok(());
                }
            }
        }
        Err(PrintError::NotFound)
    }

    fn get_job_status(&self, job_id: JobID) -> Option<&dyn PrintJob> {
        for job_option in &self.jobs {
            if let Some(ref job) = *job_option {
                if job.id() == job_id { return Some(job.as_ref()); }
            }
        }
        None
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
