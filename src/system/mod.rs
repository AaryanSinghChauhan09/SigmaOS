#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS System Utilities Module
// System-level utilities and tools

pub mod cleanup;
pub mod config;
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
pub mod state;
pub mod user;

pub use cleanup::{
    CacheStrategy, CleanupError, CleanupStats, CleanupStrategy, LogFileStrategy,
    SystemCleanupManager, TempFileStrategy,
};
pub use config::{
    ConfigEntry, ConfigError, ConfigType, ServiceManager, ServiceUnit, SystemConfigManager,
};
pub use defrag::{
    DefragError, DefragResult, DefragStrategy, DiskDefragmenter, FileBlockInfo,
    FragmentationReport, SigmaFsDefragStrategy,
};
pub use duplicate::{
    DuplicateError, DuplicateFinder, DuplicateGroup, FileMetadata, HashAlgorithm, ScanStats,
    Sha256Algorithm,
};
pub use generation_manager::{Generation, GenerationManager};
pub use memory::{
    AllocationRecord, AllocationType, LeakDetectionStrategy, LeakLocation, LeakReport,
    MemoryLeakDetector, ReferenceCountingDetector, TimeBasedDetector,
};
pub use optimizer::{
    CpuOptimization, IoOptimization, MemoryOptimization, NetworkOptimization, OptimizationError,
    OptimizationResult, OptimizationStrategy, PerformanceEnhancer, PerformanceProfile,
};
pub use power::{
    BatterySaverManager, BatteryStatus, CpuPowerStrategy, DisplayPowerStrategy,
    NetworkPowerStrategy, PowerError, PowerMode, PowerResult, PowerStrategy,
};
pub use sandbox::{
    CapabilitySandboxEnforcer, NamespaceSandboxEnforcer, NetworkPolicy, ProcessSandboxManager,
    ResourceLimits, ResourceUsage, SandboxEnforcement, SandboxError, SandboxOperation,
    SandboxProcess, SandboxProfile, SandboxResult,
};
pub use shredder::{
    Dod5220Shredder, FileShredder, GutmannShredder, RandomPassShredder, ShredderError,
    ShreddingAlgorithm, ShreddingResult, ShreddingStrategy, ZeroPassShredder,
};
pub use snapshot::{
    FileSnapshotStorage, MerkleSnapshotStorage, RestoreResult, SnapshotConfig, SnapshotError,
    SnapshotMetadata, SnapshotResult, SnapshotStorage, SystemSnapshotManager,
};
pub use startup::{
    DependencyBasedOptimizer, ProfileBasedOptimizer, ServicePriority, StartupAnalysis,
    StartupOptimizationResult, StartupOptimizationStrategy, StartupOptimizer, StartupProfile,
    StartupService,
};
pub use state::{
    DeclarativeStateGraph, StateError, StateNode, StateValue, SystemConfiguration,
    get_system_config, get_system_config_mut, init_system_config,
};
pub use user::{
    Group, User, UserError, UserManager,
};
