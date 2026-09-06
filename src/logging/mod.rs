#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Logging and Diagnostics Subsystem Mod

pub mod rotation;
pub mod unified;

pub use rotation::{
    LogCompressor, LogFacility, LogFile, LogFileID, LogRotateConfig, LogRotator, LogSeverity,
    RotationError, RotationPolicy, SimpleLogCompressor, SimpleLogFile, SimpleLogRotator,
};
pub use unified::{
    ConsoleLogTarget, FileLogTarget, LogError, LogField, LogLevel, LogTarget, LoggerCapability,
    MemoryLogTarget, NetworkFramingFormat, NetworkLogTarget, NetworkProtocol, SimpleUnifiedLogger,
    SyslogFacility, TargetCapability, TargetInfo, TargetType, UnifiedLogEntry, UnifiedLogStats,
    UnifiedLogger,
};
