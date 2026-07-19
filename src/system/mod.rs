// SigmaOS System Utilities Module
// System-level utilities and tools

pub mod cleanup;
pub mod defrag;
pub mod duplicate;
pub mod optimizer;
pub mod power;

pub use cleanup::{
    CleanupError, CleanupStats, CleanupStrategy, CacheStrategy, LogFileStrategy,
    SystemCleanupManager, TempFileStrategy,
};
pub use defrag::{
    DefragError, DefragResult, DefragStrategy, DiskDefragmenter, FileBlockInfo,
    FragmentationReport, SigmaFsDefragStrategy,
};
pub use duplicate::{
    DuplicateError, DuplicateFinder, DuplicateGroup, FileMetadata, HashAlgorithm,
    ScanStats, Sha256Algorithm,
};
pub use optimizer::{
    CpuOptimization, IoOptimization, MemoryOptimization, NetworkOptimization,
    OptimizationError, OptimizationResult, OptimizationStrategy, PerformanceEnhancer,
    PerformanceProfile,
};
pub use power::{
    BatterySaverManager, BatteryStatus, CpuPowerStrategy, DisplayPowerStrategy,
    NetworkPowerStrategy, PowerError, PowerMode, PowerResult, PowerStrategy,
};
