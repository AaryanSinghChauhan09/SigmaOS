#![no_std]
#![no_main]

/// OOP-based Unified Logging System for SigmaOS
/// Implements unified logging using OOP principles with traits and structs
/// No dependency on external logging frameworks
/// Based on Roadmap Item 13: Unified logging system

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
pub struct UnifiedLogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub component: [u8; 64],
    pub message: [u8; 512],
    pub module: [u8; 128],
    pub line: u32,
}

impl UnifiedLogEntry {
    pub fn new(level: LogLevel, component: &[u8], message: &[u8], module: &[u8], line: u32) -> Self {
        let mut component_array = [0u8; 64];
        let mut message_array = [0u8; 512];
        let mut module_array = [0u8; 128];

        let component_len = component.len().min(63);
        let message_len = message.len().min(511);
        let module_len = module.len().min(127);

        unsafe {
            core::ptr::copy_nonoverlapping(component.as_ptr(), component_array.as_mut_ptr(), component_len);
            core::ptr::copy_nonoverlapping(message.as_ptr(), message_array.as_mut_ptr(), message_len);
            core::ptr::copy_nonoverlapping(module.as_ptr(), module_array.as_mut_ptr(), module_len);
        }

        UnifiedLogEntry {
            timestamp: get_current_time(),
            level,
            component: component_array,
            message: message_array,
            module: module_array,
            line,
        }
    }
}

/// Log target trait (OOP interface)
pub trait LogTarget {
    /// Write log entry
    fn write(&mut self, entry: &UnifiedLogEntry) -> Result<(), LogError>;
    /// Flush buffered logs
    fn flush(&mut self) -> Result<(), LogError>;
    /// Get target info
    fn info(&self) -> TargetInfo;
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

/// Target info
#[repr(C)]
pub struct TargetInfo {
    target_type: TargetType,
    buffer_size: usize,
    entries_written: u64,
    capability: TargetCapability,
}

impl TargetInfo {
    pub fn new(target_type: TargetType) -> Self {
        TargetInfo {
            target_type,
            buffer_size: 0,
            entries_written: 0,
            capability: TargetCapability::new(),
        }
    }
}

/// Target type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TargetType {
    Console = 0,
    File = 1,
    Network = 2,
    Memory = 3,
    Remote = 4,
}

/// Target capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TargetCapability {
    pub can_write: bool,
    pub can_flush: bool,
    pub can_rotate: bool,
}

impl TargetCapability {
    pub fn new() -> Self {
        TargetCapability {
            can_write: false,
            can_flush: false,
            pub can_rotate: false,
        }
    }

    pub fn full() -> Self {
        TargetCapability {
            can_write: true,
            can_flush: true,
            can_rotate: true,
        }
    }
}

/// Memory target (OOP: Concrete target class)
#[repr(C)]
pub struct MemoryLogTarget {
    pub entries: Vec<Option<UnifiedLogEntry>>,
    pub max_entries: usize,
    pub entries_written: AtomicUsize,
    pub capability: TargetCapability,
}

impl MemoryLogTarget {
    pub fn new(max_entries: usize, capability: TargetCapability) -> Self {
        MemoryLogTarget {
            entries: Vec::new(),
            max_entries,
            entries_written: AtomicUsize::new(0),
            capability,
        }
    }

    pub fn clear(&mut self) -> Result<(), LogError> {
        if !self.capability.can_rotate {
            return Err(LogError::PermissionDenied);
        }

        self.entries = Vec::new();
        self.entries_written.store(0, Ordering::SeqCst);
        Ok(())
    }
}

impl LogTarget for MemoryLogTarget {
    fn write(&mut self, entry: &UnifiedLogEntry) -> Result<(), LogError> {
        if !self.capability.can_write {
            return Err(LogError::PermissionDenied);
        }

        if self.entries.len() >= self.max_entries {
            return Err(LogError::BufferFull);
        }

        let mut entry_copy = UnifiedLogEntry::new(
            entry.level,
            &entry.component,
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
        Ok(())
    }

    fn info(&self) -> TargetInfo {
        TargetInfo {
            target_type: TargetType::Memory,
            buffer_size: self.max_entries,
            entries_written: self.entries_written.load(Ordering::SeqCst) as u64,
            capability: self.capability,
        }
    }
}

/// Unified logger trait (OOP interface)
pub trait UnifiedLogger {
    /// Log message at level
    fn log(&mut self, level: LogLevel, component: &[u8], message: &[u8], module: &[u8], line: u32);
    /// Set global log level
    fn set_level(&mut self, level: LogLevel);
    /// Get global log level
    fn level(&self) -> LogLevel;
    /// Add target
    fn add_target(&mut self, target: Box<dyn LogTarget>) -> Result<(), LogError>;
    /// Remove target
    fn remove_target(&mut self, index: usize) -> Result<(), LogError>;
    /// Flush all targets
    fn flush(&mut self) -> Result<(), LogError>;
    /// Get logger statistics
    fn stats(&self) -> UnifiedLogStats;
}

/// Unified log statistics
#[repr(C)]
pub struct UnifiedLogStats {
    pub total_entries: u64,
    pub entries_by_level: [u64; 6],
    pub entries_by_component: [u64; 16],
    pub targets: usize,
}

impl UnifiedLogStats {
    pub fn new() -> Self {
        UnifiedLogStats {
            total_entries: 0,
            entries_by_level: [0; 6],
            entries_by_component: [0; 16],
            targets: 0,
        }
    }
}

/// Simple unified logger (OOP: Concrete logger class)
pub struct SimpleUnifiedLogger {
    pub level: AtomicUsize, // LogLevel as usize
    pub targets: Vec<Option<Box<dyn LogTarget>>>,
    pub stats: UnifiedLogStats,
    pub capability: LoggerCapability,
}

/// Logger capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LoggerCapability {
    pub can_log: bool,
    pub can_set_level: bool,
    pub can_manage_targets: bool,
}

impl LoggerCapability {
    pub fn new() -> Self {
        LoggerCapability {
            can_log: false,
            can_set_level: false,
            can_manage_targets: false,
        }
    }

    pub fn full() -> Self {
        LoggerCapability {
            can_log: true,
            can_set_level: true,
            can_manage_targets: true,
        }
    }
}

impl SimpleUnifiedLogger {
    pub fn new(initial_level: LogLevel, capability: LoggerCapability) -> Self {
        SimpleUnifiedLogger {
            level: AtomicUsize::new(initial_level as usize),
            targets: Vec::new(),
            stats: UnifiedLogStats::new(),
            capability,
        }
    }

    unsafe fn get_component_index(&self, component: &[u8]) -> usize {
        // Simple hash to map component to index
        let mut hash: usize = 0;
        for (i, &byte) in component.iter().enumerate() {
            hash = hash.wrapping_add((byte as usize) * (i + 1));
        }
        hash % 16
    }
}

impl UnifiedLogger for SimpleUnifiedLogger {
    fn log(&mut self, level: LogLevel, component: &[u8], message: &[u8], module: &[u8], line: u32) {
        if !self.capability.can_log {
            return;
        }

        let current_level = unsafe { core::mem::transmute(self.level.load(Ordering::SeqCst)) };
        if level < current_level {
            return;
        }

        let entry = UnifiedLogEntry::new(level, component, message, module, line);

        for target_option in &mut self.targets {
            if let Some(ref mut target) = *target_option {
                let _ = target.write(&entry);
            }
        }

        self.stats.total_entries += 1;
        self.stats.entries_by_level[level as usize] += 1;

        unsafe {
            let component_index = self.get_component_index(component);
            self.stats.entries_by_component[component_index] += 1;
        }
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

    fn add_target(&mut self, target: Box<dyn LogTarget>) -> Result<(), LogError> {
        if !self.capability.can_manage_targets {
            return Err(LogError::PermissionDenied);
        }

        self.targets.push(Some(target));
        self.stats.targets += 1;
        Ok(())
    }

    fn remove_target(&mut self, index: usize) -> Result<(), LogError> {
        if !self.capability.can_manage_targets {
            return Err(LogError::PermissionDenied);
        }

        if index < self.targets.len() {
            self.targets[index] = None;
            self.stats.targets -= 1;
            Ok(())
        } else {
            Err(LogError::WriteFailed)
        }
    }

    fn flush(&mut self) -> Result<(), LogError> {
        for target_option in &mut self.targets {
            if let Some(ref mut target) = *target_option {
                let _ = target.flush();
            }
        }
        Ok(())
    }

    fn stats(&self) -> UnifiedLogStats {
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
