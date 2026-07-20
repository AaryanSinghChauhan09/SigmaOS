// SigmaOS System Utilities Module
// System-level utilities and tools

pub mod cleanup;
pub mod defrag;
pub mod duplicate;
pub mod generation_manager;
pub mod memory;
pub mod optimizer;
pub mod power;
pub mod sandbox;
pub mod shredder;
pub mod snapshot;
pub mod startup;

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
pub use generation_manager::{Generation, GenerationManager};
pub use memory::{
    AllocationRecord, AllocationType, LeakLocation, LeakReport, LeakDetectionStrategy,
    MemoryLeakDetector, ReferenceCountingDetector, TimeBasedDetector,
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
pub use sandbox::{
    CapabilitySandboxEnforcer, NamespaceSandboxEnforcer, NetworkPolicy, ResourceLimits,
    ResourceUsage, SandboxEnforcement, SandboxError, SandboxOperation, SandboxProfile,
    SandboxProcess, SandboxResult, ProcessSandboxManager,
};
pub use shredder::{
    Dod5220Shredder, FileShredder, GutmannShredder, RandomPassShredder, ShredderError,
    ShreddingAlgorithm, ShreddingResult, ShreddingStrategy, ZeroPassShredder,
};
pub use snapshot::{
    FileSnapshotStorage, MerkleSnapshotStorage, RestoreResult, SnapshotConfig,
    SnapshotError, SnapshotMetadata, SnapshotResult, SnapshotStorage,
    SystemSnapshotManager,
};
pub use startup::{
    DependencyBasedOptimizer, ProfileBasedOptimizer, ServicePriority, StartupAnalysis,
    StartupOptimizationResult, StartupOptimizationStrategy, StartupOptimizer,
    StartupProfile, StartupService,
};
