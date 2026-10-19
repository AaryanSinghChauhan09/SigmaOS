// SigmaOS Logging and Diagnostics Subsystem Mod

pub mod rotation;
pub mod sigmalog;
pub mod unified;

pub use rotation::{
    LogCompressor, LogFacility as RotationLogFacility, LogFile, LogFileID, LogRotateConfig,
    LogRotator, LogSeverity, RotationError, RotationPolicy, SimpleLogCompressor, SimpleLogFile,
    SimpleLogRotator,
};
pub use sigmalog::{
    get_sigmalog, init_sigmalog, init_sigmalog_default, log_message, LogEntry,
    LogFacility as SigmaLogFacility, LogFilter, LogLevel, LogStorage, SigmaLog, SigmaLogConfig,
};
pub use unified::{
    ConsoleLogTarget, FileLogTarget, LogError, LogField, LogLevel as UnifiedLogLevel, LogTarget,
    LoggerCapability, MemoryLogTarget, NetworkFramingFormat, NetworkLogTarget, NetworkProtocol,
    SimpleUnifiedLogger, SyslogFacility, TargetCapability, TargetInfo, TargetType, UnifiedLogEntry,
    UnifiedLogStats, UnifiedLogger,
};
