#![no_std]
#![no_main]

/// OOP-based Logging System for SigmaOS
/// Implements logging using OOP principles with traits and structs
/// No dependency on external logging frameworks

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Log level
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
    Fatal = 5,
}

/// Log entry (OOP: Log entry object)
#[repr(C)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub message: [u8; 512],
    pub module: [u8; 64],
    pub line: u32,
}

impl LogEntry {
    pub fn new(level: LogLevel, message: &[u8], module: &[u8], line: u32) -> Self {
        let mut message_array = [0u8; 512];
        let mut module_array = [0u8; 64];

        let msg_len = message.len().min(511);
        let mod_len = module.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(message.as_ptr(), message_array.as_mut_ptr(), msg_len);
            core::ptr::copy_nonoverlapping(module.as_ptr(), module_array.as_mut_ptr(), mod_len);
        }

        LogEntry {
            timestamp: get_current_time(),
            level,
            message: message_array,
            module: module_array,
            line,
        }
    }
}

/// Log appender trait (OOP interface)
pub trait LogAppender {
    /// Append log entry
    fn append(&mut self, entry: &LogEntry) -> Result<(), LogError>;
    /// Flush buffered logs
    fn flush(&mut self) -> Result<(), LogError>;
    /// Get appender info
    fn info(&self) -> AppenderInfo;
}

/// Log error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LogError {
    Success = 0,
    BufferFull = 1,
    WriteFailed = 2,
    PermissionDenied = 3,
    InvalidFormat = 4,
}

/// Appender info
#[repr(C)]
pub struct AppenderInfo {
    pub appender_type: AppenderType,
    pub buffer_size: usize,
    pub entries_written: u64,
    pub capability: AppenderCapability,
}

impl AppenderInfo {
    pub fn new(appender_type: AppenderType) -> Self {
        AppenderInfo {
            appender_type,
            buffer_size: 0,
            entries_written: 0,
            capability: AppenderCapability::new(),
        }
    }
}

/// Appender type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AppenderType {
    Console = 0,
    File = 1,
    Memory = 2,
    Network = 3,
    Custom = 4,
}

/// Appender capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppenderCapability {
    pub can_append: bool,
    pub can_flush: bool,
    pub can_clear: bool,
}

impl AppenderCapability {
    pub fn new() -> Self {
        AppenderCapability {
            can_append: false,
            can_flush: false,
            can_clear: false,
        }
    }

    pub fn full() -> Self {
        AppenderCapability {
            can_append: true,
            can_flush: true,
            can_clear: true,
        }
    }
}

/// Memory appender (OOP: Concrete appender class)
#[repr(C)]
pub struct MemoryAppender {
    pub entries: Vec<Option<LogEntry>>,
    pub max_entries: usize,
    pub entries_written: AtomicUsize,
    pub capability: AppenderCapability,
}

impl MemoryAppender {
    pub fn new(max_entries: usize, capability: AppenderCapability) -> Self {
        MemoryAppender {
            entries: Vec::new(),
            max_entries,
            entries_written: AtomicUsize::new(0),
            capability,
        }
    }

    pub fn clear(&mut self) -> Result<(), LogError> {
        if !self.capability.can_clear {
            return Err(LogError::PermissionDenied);
        }

        self.entries = Vec::new();
        self.entries_written.store(0, Ordering::SeqCst);
        Ok(())
    }
}

impl LogAppender for MemoryAppender {
    fn append(&mut self, entry: &LogEntry) -> Result<(), LogError> {
        if !self.capability.can_append {
            return Err(LogError::PermissionDenied);
        }

        if self.entries.len() >= self.max_entries {
            return Err(LogError::BufferFull);
        }

        let mut entry_copy = LogEntry::new(
            entry.level,
            &entry.message,
            &entry.module,
            entry.line,
        );
        entry_copy.timestamp = entry.timestamp;

        self.entries.push(Some(entry_copy));
        self.entries_written.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), LogError> {
        if !self.capability.can_flush {
            return Err(LogError::PermissionDenied);
        }
        // Memory appender doesn't need flushing
        Ok(())
    }

    fn info(&self) -> AppenderInfo {
        AppenderInfo {
            appender_type: AppenderType::Memory,
            buffer_size: self.max_entries,
            entries_written: self.entries_written.load(Ordering::SeqCst) as u64,
            capability: self.capability,
        }
    }
}

/// Logger trait (OOP interface)
pub trait Logger {
    /// Log message at level
    fn log(&mut self, level: LogLevel, message: &[u8], module: &[u8], line: u32);
    /// Set log level
    fn set_level(&mut self, level: LogLevel);
    /// Get log level
    fn level(&self) -> LogLevel;
    /// Add appender
    fn add_appender(&mut self, appender: Box<dyn LogAppender>) -> Result<(), LogError>;
    /// Remove appender
    fn remove_appender(&mut self, index: usize) -> Result<(), LogError>;
    /// Flush all appenders
    fn flush(&mut self) -> Result<(), LogError>;
    /// Get logger statistics
    fn stats(&self) -> LoggerStats;
}

/// Logger statistics
#[repr(C)]
pub struct LoggerStats {
    pub total_entries: u64,
    pub entries_by_level: [u64; 6],
    pub appenders: usize,
}

impl LoggerStats {
    pub fn new() -> Self {
        LoggerStats {
            total_entries: 0,
            entries_by_level: [0; 6],
            appenders: 0,
        }
    }
}

/// Simple logger (OOP: Concrete logger class)
pub struct SimpleLogger {
    pub level: AtomicUsize, // LogLevel as usize
    pub appenders: Vec<Option<Box<dyn LogAppender>>>,
    pub stats: LoggerStats,
    pub capability: LoggerCapability,
}

/// Logger capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LoggerCapability {
    pub can_log: bool,
    pub can_set_level: bool,
    pub can_manage_appenders: bool,
}

impl LoggerCapability {
    pub fn new() -> Self {
        LoggerCapability {
            can_log: false,
            can_set_level: false,
            can_manage_appenders: false,
        }
    }

    pub fn full() -> Self {
        LoggerCapability {
            can_log: true,
            can_set_level: true,
            can_manage_appenders: true,
        }
    }
}

impl SimpleLogger {
    pub fn new(initial_level: LogLevel, capability: LoggerCapability) -> Self {
        SimpleLogger {
            level: AtomicUsize::new(initial_level as usize),
            appenders: Vec::new(),
            stats: LoggerStats::new(),
            capability,
        }
    }
}

impl Logger for SimpleLogger {
    fn log(&mut self, level: LogLevel, message: &[u8], module: &[u8], line: u32) {
        if !self.capability.can_log {
            return;
        }

        let current_level = unsafe { core::mem::transmute(self.level.load(Ordering::SeqCst)) };
        if level < current_level {
            return;
        }

        let entry = LogEntry::new(level, message, module, line);

        for appender_option in &mut self.appenders {
            if let Some(ref mut appender) = *appender_option {
                let _ = appender.append(&entry);
            }
        }

        self.stats.total_entries += 1;
        self.stats.entries_by_level[level as usize] += 1;
    }

    fn set_level(&mut self, level: LogLevel) {
        if !self.capability.can_set_level {
            return;
        }
        self.level.store(level as usize, Ordering::SeqCst);
    }

    fn level(&self) -> LogLevel {
        unsafe {
            core::mem::transmute(self.level.load(Ordering::SeqCst))
        }
    }

    fn add_appender(&mut self, appender: Box<dyn LogAppender>) -> Result<(), LogError> {
        if !self.capability.can_manage_appenders {
            return Err(LogError::PermissionDenied);
        }

        self.appenders.push(Some(appender));
        self.stats.appenders += 1;
        Ok(())
    }

    fn remove_appender(&mut self, index: usize) -> Result<(), LogError> {
        if !self.capability.can_manage_appenders {
            return Err(LogError::PermissionDenied);
        }

        if index < self.appenders.len() {
            self.appenders[index] = None;
            self.stats.appenders -= 1;
            Ok(())
        } else {
            Err(LogError::WriteFailed)
        }
    }

    fn flush(&mut self) -> Result<(), LogError> {
        for appender_option in &mut self.appenders {
            if let Some(ref mut appender) = *appender_option {
                let _ = appender.flush();
            }
        }
        Ok(())
    }

    fn stats(&self) -> LoggerStats {
        self.stats
    }
}

/// Get current time (nanoseconds)
fn get_current_time() -> u64 {
    static mut COUNTER: u64 = 0;
    unsafe {
        COUNTER += 1_000_000;
        COUNTER
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
