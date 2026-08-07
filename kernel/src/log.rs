/// SigmaOS: Kernel Logging System
/// Provides structured logging with different log levels

#[allow(dead_code)]

use core::fmt::Write;

// ─── Log Levels ───────────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

// ─── Logger ─────────────────────────────────────────────────────────────

pub struct Logger {
    min_level: LogLevel,
    log_count: usize,
}

impl Logger {
    pub const fn new() -> Self {
        Self {
            min_level: LogLevel::Info,
            log_count: 0,
        }
    }

    /// Set minimum log level
    pub fn set_min_level(&mut self, level: LogLevel) {
        self.min_level = level;
    }

    /// Get minimum log level
    pub fn get_min_level(&self) -> LogLevel {
        self.min_level
    }

    /// Get log count
    pub fn get_log_count(&self) -> usize {
        self.log_count
    }

    /// Log a message at the specified level
    pub fn log(&mut self, level: LogLevel, module: &str, message: &str) {
        if (level as u8) < self.min_level as u8 {
            return;
        }

        self.log_count += 1;

        let mut writer = LogWriter::new();
        
        // Write log level
        match level {
            LogLevel::Trace => writer.write_str("[TRACE] "),
            LogLevel::Debug => writer.write_str("[DEBUG] "),
            LogLevel::Info => writer.write_str("[INFO] "),
            LogLevel::Warn => writer.write_str("[WARN] "),
            LogLevel::Error => writer.write_str("[ERROR] "),
        };

        // Write module name
        if !module.is_empty() {
            writer.write_str(module);
            writer.write_str(": ");
        }

        // Write message
        writer.write_str(message);
        writer.write_str("\n");
    }

    /// Convenience methods for different log levels
    pub fn trace(&mut self, module: &str, message: &str) {
        self.log(LogLevel::Trace, module, message);
    }

    pub fn debug(&mut self, module: &str, message: &str) {
        self.log(LogLevel::Debug, module, message);
    }

    pub fn info(&mut self, module: &str, message: &str) {
        self.log(LogLevel::Info, module, message);
    }

    pub fn warn(&mut self, module: &str, message: &str) {
        self.log(LogLevel::Warn, module, message);
    }

    pub fn error(&mut self, module: &str, message: &str) {
        self.log(LogLevel::Error, module, message);
    }
}

// ─── Log Writer ─────────────────────────────────────────────────────────

struct LogWriter;

impl LogWriter {
    const fn new() -> Self {
        Self
    }
}

impl Write for LogWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // TODO: Implement actual output to VGA/serial
        // For now, this is a stub
        let _ = s;
        Ok(())
    }
    
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        // TODO: Implement actual output
        let _ = c;
        Ok(())
    }
}

// ─── Global Logger Instance ─────────────────────────────────────────────

static mut LOGGER: Logger = Logger::new();

/// Initialize the global logger
pub fn init() {
    unsafe {
        LOGGER = Logger::new();
    }
}

/// Set minimum log level
pub fn set_min_level(level: LogLevel) {
    unsafe {
        LOGGER.set_min_level(level);
    }
}

/// Get minimum log level
pub fn get_min_level() -> LogLevel {
    unsafe {
        LOGGER.get_min_level()
    }
}

/// Get log count
pub fn get_log_count() -> usize {
    unsafe {
        LOGGER.get_log_count()
    }
}

/// Log a message at the specified level
pub fn log(level: LogLevel, module: &str, message: &str) {
    unsafe {
        LOGGER.log(level, module, message);
    }
}

/// Convenience functions for different log levels
pub fn trace(module: &str, message: &str) {
    unsafe {
        LOGGER.trace(module, message);
    }
}

pub fn debug(module: &str, message: &str) {
    unsafe {
        LOGGER.debug(module, message);
    }
}

pub fn info(module: &str, message: &str) {
    unsafe {
        LOGGER.info(module, message);
    }
}

pub fn warn(module: &str, message: &str) {
    unsafe {
        LOGGER.warn(module, message);
    }
}

pub fn error(module: &str, message: &str) {
    unsafe {
        LOGGER.error(module, message);
    }
}

// ─── Log Macros ───────────────────────────────────────────────────────────

/// Macro for trace logging
#[macro_export]
macro_rules! ktrace {
    ($($arg:tt)*) => {
        $crate::log::trace(module_path!(), &format_args!($($arg)*).to_string());
    };
}

/// Macro for debug logging
#[macro_export]
macro_rules! kdebug {
    ($($arg:tt)*) => {
        $crate::log::debug(module_path!(), &format_args!($($arg)*).to_string());
    };
}

/// Macro for info logging
#[macro_export]
macro_rules! kinfo {
    ($($arg:tt)*) => {
        $crate::log::info(module_path!(), &format_args!($($arg)*).to_string());
    };
}

/// Macro for warn logging
#[macro_export]
macro_rules! kwarn {
    ($($arg:tt)*) => {
        $crate::log::warn(module_path!(), &format_args!($($arg)*).to_string());
    };
}

/// Macro for error logging
#[macro_export]
macro_rules! kerror {
    ($($arg:tt)*) => {
        $crate::log::error(module_path!(), &format_args!($($arg)*).to_string());
    };
}
