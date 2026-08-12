// SigmaOS Logging and Diagnostics Subsystem Mod

pub mod unified;
pub mod rotation;

pub use unified::{
    ConsoleLogTarget, FileLogTarget, LogError, LogLevel, LogTarget, LoggerCapability,
    MemoryLogTarget, NetworkLogTarget, SimpleUnifiedLogger, TargetCapability, TargetInfo,
    TargetType, UnifiedLogEntry, UnifiedLogStats, UnifiedLogger,
};
pub use rotation::{
    LogSeverity, LogFacility, SimpleLogFile, SimpleLogRotator, SimpleLogCompressor,
};
