// OOP-based Unified Logging System and Diverse Targets for SigmaOS
// Inspired by Linux systemd-journald and rsyslog, providing Console, File, Network, and Memory logging targets.

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Log level
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
pub struct UnifiedLogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub component: [u8; 64],
    pub message: [u8; 512],
    pub module: [u8; 128],
    pub line: u32,
}

impl UnifiedLogEntry {
    pub fn new(
        level: LogLevel,
        component: &[u8],
        message: &[u8],
        module: &[u8],
        line: u32,
    ) -> Self {
        let mut component_array = [0u8; 64];
        let mut message_array = [0u8; 512];
        let mut module_array = [0u8; 128];

        let component_len = component.len().min(63);
        let message_len = message.len().min(511);
        let module_len = module.len().min(127);

        component_array[..component_len].copy_from_slice(&component[..component_len]);
        message_array[..message_len].copy_from_slice(&message[..message_len]);
        module_array[..module_len].copy_from_slice(&module[..module_len]);

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogError {
    Success = 0,
    BufferFull = 1,
    WriteFailed = 2,
    PermissionDenied = 3,
    InvalidFormat = 4,
}

/// Target info
pub struct TargetInfo {
    pub target_type: TargetType,
    pub buffer_size: usize,
    pub entries_written: u64,
    pub capability: TargetCapability,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetType {
    Console = 0,
    File = 1,
    Network = 2,
    Memory = 3,
    Remote = 4,
}

/// Target capability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TargetCapability {
    pub can_write: bool,
    pub can_flush: bool,
    pub can_rotate: bool,
}

impl TargetCapability {
    pub const fn new() -> Self {
        TargetCapability {
            can_write: false,
            can_flush: false,
            can_rotate: false,
        }
    }

    pub const fn full() -> Self {
        TargetCapability {
            can_write: true,
            can_flush: true,
            can_rotate: true,
        }
    }
}

impl Default for TargetCapability {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 1. MEMORY LOG TARGET
// ==========================================

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

// ==========================================
// 2. FILE LOG TARGET (/var/log/syslog)
// ==========================================

pub struct FileLogTarget {
    pub file_path: String,
    pub file_buffer: Vec<String>,
    pub entries_written: AtomicUsize,
    pub capability: TargetCapability,
}

impl FileLogTarget {
    pub fn new(path: &str, capability: TargetCapability) -> Self {
        Self {
            file_path: String::from(path),
            file_buffer: Vec::new(),
            entries_written: AtomicUsize::new(0),
            capability,
        }
    }
}

impl LogTarget for FileLogTarget {
    fn write(&mut self, entry: &UnifiedLogEntry) -> Result<(), LogError> {
        if !self.capability.can_write {
            return Err(LogError::PermissionDenied);
        }

        let msg_len = entry.message.iter().position(|&b| b == 0).unwrap_or(512);
        let msg_str = String::from_utf8_lossy(&entry.message[..msg_len]);

        let formatted = alloc::format!("[FILE][{:?}]: {}", entry.level, msg_str);
        self.file_buffer.push(formatted);
        self.entries_written.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), LogError> {
        Ok(())
    }

    fn info(&self) -> TargetInfo {
        TargetInfo {
            target_type: TargetType::File,
            buffer_size: 10000,
            entries_written: self.entries_written.load(Ordering::SeqCst) as u64,
            capability: self.capability,
        }
    }
}

// ==========================================
// 3. CONSOLE LOG TARGET (/dev/console)
// ==========================================

pub struct ConsoleLogTarget {
    pub output_history: Vec<String>,
    pub entries_written: AtomicUsize,
    pub capability: TargetCapability,
}

impl ConsoleLogTarget {
    pub fn new(capability: TargetCapability) -> Self {
        Self {
            output_history: Vec::new(),
            entries_written: AtomicUsize::new(0),
            capability,
        }
    }
}

impl LogTarget for ConsoleLogTarget {
    fn write(&mut self, entry: &UnifiedLogEntry) -> Result<(), LogError> {
        if !self.capability.can_write {
            return Err(LogError::PermissionDenied);
        }

        let msg_len = entry.message.iter().position(|&b| b == 0).unwrap_or(512);
        let msg_str = String::from_utf8_lossy(&entry.message[..msg_len]);

        let formatted = alloc::format!("[STDOUT][{:?}]: {}", entry.level, msg_str);
        self.output_history.push(formatted);
        self.entries_written.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), LogError> {
        Ok(())
    }

    fn info(&self) -> TargetInfo {
        TargetInfo {
            target_type: TargetType::Console,
            buffer_size: 5000,
            entries_written: self.entries_written.load(Ordering::SeqCst) as u64,
            capability: self.capability,
        }
    }
}

// ==========================================
// 4. NETWORK LOG TARGET (rsyslog udp:514)
// ==========================================

pub struct NetworkLogTarget {
    pub server_ip: String,
    pub server_port: u16,
    pub forwarded_packets: Vec<String>,
    pub entries_written: AtomicUsize,
    pub capability: TargetCapability,
}

impl NetworkLogTarget {
    pub fn new(server_ip: &str, server_port: u16, capability: TargetCapability) -> Self {
        Self {
            server_ip: String::from(server_ip),
            server_port,
            forwarded_packets: Vec::new(),
            entries_written: AtomicUsize::new(0),
            capability,
        }
    }
}

impl LogTarget for NetworkLogTarget {
    fn write(&mut self, entry: &UnifiedLogEntry) -> Result<(), LogError> {
        if !self.capability.can_write {
            return Err(LogError::PermissionDenied);
        }

        let msg_len = entry.message.iter().position(|&b| b == 0).unwrap_or(512);
        let msg_str = String::from_utf8_lossy(&entry.message[..msg_len]);

        let sys_packet = alloc::format!(
            "<34>[FORWARDED TO {}:{}]: {}",
            self.server_ip,
            self.server_port,
            msg_str
        );
        self.forwarded_packets.push(sys_packet);
        self.entries_written.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), LogError> {
        Ok(())
    }

    fn info(&self) -> TargetInfo {
        TargetInfo {
            target_type: TargetType::Network,
            buffer_size: 2000,
            entries_written: self.entries_written.load(Ordering::SeqCst) as u64,
            capability: self.capability,
        }
    }
}

// ==========================================
// LOGGER MANAGER & IMPLEMENTATION
// ==========================================

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnifiedLogStats {
    pub total_entries: u64,
    pub entries_by_level: [u64; 6],
    pub entries_by_component: [u64; 16],
    pub targets: usize,
}

impl UnifiedLogStats {
    pub const fn new() -> Self {
        UnifiedLogStats {
            total_entries: 0,
            entries_by_level: [0; 6],
            entries_by_component: [0; 16],
            targets: 0,
        }
    }
}

impl Default for UnifiedLogStats {
    fn default() -> Self {
        Self::new()
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoggerCapability {
    pub can_log: bool,
    pub can_set_level: bool,
    pub can_manage_targets: bool,
}

impl LoggerCapability {
    pub const fn new() -> Self {
        LoggerCapability {
            can_log: false,
            can_set_level: false,
            can_manage_targets: false,
        }
    }

    pub const fn full() -> Self {
        LoggerCapability {
            can_log: true,
            can_set_level: true,
            can_manage_targets: true,
        }
    }
}

impl Default for LoggerCapability {
    fn default() -> Self {
        Self::new()
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

    fn get_component_index(&self, component: &[u8]) -> usize {
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

        let current_level = match self.level.load(Ordering::SeqCst) {
            0 => LogLevel::Trace,
            1 => LogLevel::Debug,
            2 => LogLevel::Info,
            3 => LogLevel::Warning,
            4 => LogLevel::Error,
            _ => LogLevel::Fatal,
        };

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

        let component_index = self.get_component_index(component);
        self.stats.entries_by_component[component_index] += 1;
    }

    fn set_level(&mut self, level: LogLevel) {
        if !self.capability.can_set_level {
            return;
        }
        self.level.store(level as usize, Ordering::SeqCst);
    }

    fn level(&self) -> LogLevel {
        match self.level.load(Ordering::SeqCst) {
            0 => LogLevel::Trace,
            1 => LogLevel::Debug,
            2 => LogLevel::Info,
            3 => LogLevel::Warning,
            4 => LogLevel::Error,
            _ => LogLevel::Fatal,
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

/// Get current simulated time (nanoseconds)
fn get_current_time() -> u64 {
    static mut COUNTER: u64 = 0;
    unsafe {
        COUNTER += 1_000_000;
        COUNTER
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_routing_to_multiple_targets() {
        let mut logger = SimpleUnifiedLogger::new(LogLevel::Debug, LoggerCapability::full());

        let mem_target = MemoryLogTarget::new(5, TargetCapability::full());
        let file_target = FileLogTarget::new("/var/log/syslog", TargetCapability::full());
        let console_target = ConsoleLogTarget::new(TargetCapability::full());
        let net_target = NetworkLogTarget::new("192.168.1.50", 514, TargetCapability::full());

        logger.add_target(Box::new(mem_target)).unwrap();
        logger.add_target(Box::new(file_target)).unwrap();
        logger.add_target(Box::new(console_target)).unwrap();
        logger.add_target(Box::new(net_target)).unwrap();

        assert_eq!(logger.stats().targets, 4);

        // Dispatch log entry
        logger.log(
            LogLevel::Warning,
            b"SYS",
            b"Kernel panic averted",
            b"main.rs",
            42,
        );

        // Verify statistics
        assert_eq!(logger.stats().total_entries, 1);
        assert_eq!(
            logger.stats().entries_by_level[LogLevel::Warning as usize],
            1
        );
    }
}
