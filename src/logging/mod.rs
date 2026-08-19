// SigmaOS Logging and Diagnostics Subsystem Mod

pub mod unified;

pub use unified::{
    ConsoleLogTarget, FileLogTarget, LogError, LogLevel, LogTarget, LoggerCapability,
    MemoryLogTarget, NetworkLogTarget, SimpleUnifiedLogger, TargetCapability, TargetInfo,
    TargetType, UnifiedLogEntry, UnifiedLogStats, UnifiedLogger,
};
pub mod rotation;

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