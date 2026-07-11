// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/system_api/ai_integration/native_log.rs — Native Logging
//
// Simple logging implementation without external dependencies
// Provides basic logging levels and output to stderr
//
// Language: Rust (std)

use std::io::{self, Write};
use std::sync::Mutex;

/// Log level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "error" => LogLevel::Error,
            "warn" => LogLevel::Warn,
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            "trace" => LogLevel::Trace,
            _ => LogLevel::Info,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        }
    }
}

/// Simple logger
pub struct Logger {
    level: LogLevel,
    output: Mutex<Box<dyn Write + Send>>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            level: LogLevel::Info,
            output: Mutex::new(Box::new(io::stderr())),
        }
    }

    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    pub fn set_level_from_env(&mut self) {
        if let Ok(env_level) = std::env::var("SIGMA_LOG_LEVEL") {
            self.level = LogLevel::from_str(&env_level);
        } else if let Ok(env_level) = std::env::var("RUST_LOG") {
            self.level = LogLevel::from_str(&env_level);
        }
    }

    fn log(&self, level: LogLevel, target: &str, message: &str) {
        if level <= self.level {
            let timestamp = self.get_timestamp();
            let level_str = level.as_str();
            let log_line = format!("[{} {} {}] {}\n", timestamp, level_str, target, message);

            if let Ok(mut output) = self.output.lock() {
                let _ = output.write_all(log_line.as_bytes());
                let _ = output.flush();
            }
        }
    }

    pub fn error(&self, target: &str, message: &str) {
        self.log(LogLevel::Error, target, message);
    }

    pub fn warn(&self, target: &str, message: &str) {
        self.log(LogLevel::Warn, target, message);
    }

    pub fn info(&self, target: &str, message: &str) {
        self.log(LogLevel::Info, target, message);
    }

    pub fn debug(&self, target: &str, message: &str) {
        self.log(LogLevel::Debug, target, message);
    }

    pub fn trace(&self, target: &str, message: &str) {
        self.log(LogLevel::Trace, target, message);
    }

    fn get_timestamp(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        
        let secs = duration.as_secs();
        let millis = duration.subsec_millis();
        
        format!("{:}.{:03}", secs, millis)
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

/// Global logger instance
static mut GLOBAL_LOGGER: Option<Logger> = None;
static LOGGER_INIT: std::sync::Once = std::sync::Once::new();

/// Initialize global logger
pub fn init_logger() {
    LOGGER_INIT.call_once(|| {
        let mut logger = Logger::new();
        logger.set_level_from_env();
        
        unsafe {
            GLOBAL_LOGGER = Some(logger);
        }
    });
}

/// Get global logger
fn get_logger() -> &'static Logger {
    init_logger();
    
    unsafe {
        GLOBAL_LOGGER.as_ref().unwrap()
    }
}

/// Log macros
#[macro_export]
macro_rules! log_error {
    ($target:expr, $($arg:tt)*) => {
        $crate::native_log::get_logger().error($target, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warn {
    ($target:expr, $($arg:tt)*) => {
        $crate::native_log::get_logger().warn($target, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_info {
    ($target:expr, $($arg:tt)*) => {
        $crate::native_log::get_logger().info($target, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_debug {
    ($target:expr, $($arg:tt)*) => {
        $crate::native_log::get_logger().debug($target, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_trace {
    ($target:expr, $($arg:tt)*) => {
        $crate::native_log::get_logger().trace($target, &format!($($arg)*));
    };
}

/// Convenience macros without target
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        log_error!("sigma_ai", $($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        log_warn!("sigma_ai", $($arg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log_info!("sigma_ai", $($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        log_debug!("sigma_ai", $($arg)*);
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        log_trace!("sigma_ai", $($arg)*);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_from_str() {
        assert_eq!(LogLevel::from_str("error"), LogLevel::Error);
        assert_eq!(LogLevel::from_str("INFO"), LogLevel::Info);
        assert_eq!(LogLevel::from_str("invalid"), LogLevel::Info);
    }

    #[test]
    fn test_log_level_as_str() {
        assert_eq!(LogLevel::Error.as_str(), "ERROR");
        assert_eq!(LogLevel::Info.as_str(), "INFO");
    }
}
