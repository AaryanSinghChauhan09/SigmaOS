// SigmaOS Logging and Diagnostics Subsystem Mod

pub mod rotation;
pub mod sigmalog;
pub mod unified;

pub use rotation::{
    LogCompressor, LogFacility, LogFile, LogFileID, LogRotateConfig, LogRotator, LogSeverity,
    RotationError, RotationPolicy, SimpleLogCompressor, SimpleLogFile, SimpleLogRotator,
};
pub use sigmalog::{
    SigmaLog, SigmaLogConfig, LogEntry, LogLevel, LogFilter, LogStorage,
    init_sigmalog, init_sigmalog_default, get_sigmalog, log_message,
};
pub use unified::{
    ConsoleLogTarget, FileLogTarget, LogError, LogField, LogLevel as UnifiedLogLevel, LogTarget, LoggerCapability,
    MemoryLogTarget, NetworkFramingFormat, NetworkLogTarget, NetworkProtocol, SimpleUnifiedLogger,
    SyslogFacility, TargetCapability, TargetInfo, TargetType, UnifiedLogEntry, UnifiedLogStats,
    UnifiedLogger,
};
