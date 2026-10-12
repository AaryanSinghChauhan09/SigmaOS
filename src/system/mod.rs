#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
pub mod boot_manager;
pub mod monitor;
pub mod network_manager;
pub mod power_manager;
pub mod scheduler_manager;
pub mod service_manager;
pub mod settings_manager;
pub mod sigmatime;
pub mod system_manager;
pub mod system_monitor;
pub mod task_manager;
pub mod task_scheduler;
pub mod update_manager;
pub mod user_manager;
pub mod preload;
pub mod mint_tools;

// SigmaOS System Utilities Module
// System-level utilities and tools

pub mod abi;
pub mod automation;
pub mod cleanup;
pub mod config;
pub mod cron;
pub mod declarative_state;
pub mod defrag;
pub mod duplicate;
pub mod generation_manager;
pub mod memory;
pub mod optimizer;
pub mod power;
pub mod process_supervisor;
pub mod sandbox;
pub mod shredder;
pub mod snapshot;
pub mod snapshot_schedule;
pub mod sovereign_init_supervisor;
pub mod startup;

pub use sovereign_init_supervisor::*;
pub mod dmesg;
pub mod state;
pub mod syslog;
pub mod user;

pub use dmesg::{DmesgRingBuffer, KernelLogLevel, KernelLogRecord};

pub use abi::{
    AbiType, SovereignSyscallAbiCompatibilityEngine, SyscallAbiResult, SyscallRegisters64,
};

pub use automation::{AutomationTask, AutomationTaskKind, SovereignAutomationEngine, TaskStatus};

pub use boot_manager::{BootManager, StartupEntry, StartupStatistics, StartupType};
pub use cleanup::{
    CacheStrategy, CleanupError, CleanupStats, CleanupStrategy, LogFileStrategy,
    SystemCleanupManager, TempFileStrategy,
};
pub use config::{
    ConfigEntry, ConfigError, ConfigType, ServiceManager, ServiceUnit, SystemConfigManager,
};
pub use cron::{CronDaemon, CronError, CronField, CronJob, CronSchedule, SovereignCronDaemon};
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
    AllocationRecord, AllocationType, FiftyPercentRamTargetPool, LeakDetectionStrategy,
    LeakLocation, LeakReport, MemoryLeakDetector, ReferenceCountingDetector,
    SovereignFiftyPercentRamRuleEngine, TimeBasedDetector,
};
pub use optimizer::{
    CpuOptimization, IoOptimization, MemoryOptimization, NetworkOptimization, OptimizationError,
    OptimizationResult, OptimizationStrategy, PerformanceEnhancer, PerformanceProfile,
};
pub use power::{
    BatterySaverManager, BatteryStatus, CpuPowerStrategy, DisplayPowerStrategy,
    NetworkPowerStrategy, PowerError, PowerMode, PowerResult, PowerStrategy,
};
pub use sigmatime::{
    SigmaTime, TimeConfig, NtpServer, SyncStatus, TimeSource, SyncStats,
    init_sigmatime, init_sigmatime_default, get_sigmatime, add_ntp_server, sync_time, get_current_time,
};
pub use power_manager::{
    BatteryInfo, PowerBatteryStatus, PowerConfig, PowerManager, PowerProfile, PowerSource,
    PowerStatistics, SleepAction,
};
pub use process_supervisor::{
    ProcessConfig, ProcessState, ProcessStatus, ProcessSupervisor, SupervisorError,
};
pub use sandbox::{
    CapabilitySandboxEnforcer, NamespaceSandboxEnforcer, NetworkPolicy, ProcessSandboxManager,
    ResourceLimits, ResourceUsage, SandboxEnforcement, SandboxError, SandboxOperation,
    SandboxProcess, SandboxProfile, SandboxResult,
};
pub use scheduler_manager::{
    ScheduleFrequency, ScheduleStatus, ScheduledTask, SchedulerManager, SchedulerStatistics,
};
pub use service_manager::{
    SystemRestartPolicy, SystemService, SystemServiceConfig, SystemServiceManager,
    SystemServiceState, SystemServiceType,
};
pub use settings_manager::{
    SettingCategory, SettingEntry, SettingValue, SettingsManager, SettingsStatistics,
};
pub use shredder::{
    Dod5220Shredder, FileShredder, GutmannShredder, RandomPassShredder, ShredderError,
    ShreddingAlgorithm, ShreddingResult, ShreddingStrategy, ZeroPassShredder,
};
pub use snapshot::{
    FileSnapshotStorage, MerkleSnapshotStorage, RestoreResult, SnapshotConfig, SnapshotError,
    SnapshotMetadata, SnapshotResult, SnapshotStorage, SystemSnapshotManager,
};
pub use snapshot_schedule::{
    CreatedSnapshotRecord, RetentionPolicy, ScheduledSnapshotEngine, SnapshotFrequency, SnapshotJob,
};
pub use startup::{
    DependencyBasedOptimizer, ProfileBasedOptimizer, ServicePriority, StartupAnalysis,
    StartupOptimizationResult, StartupOptimizationStrategy, StartupOptimizer, StartupProfile,
    StartupService,
};
pub use state::{
    get_system_config, get_system_config_mut, init_system_config, DeclarativeStateGraph,
    StateError, StateNode, StateValue, SystemConfiguration,
};
pub use syslog::{LogAction, LogEntry, LogFacility, LogLevel, LogRule, SyslogError, SyslogManager};
pub use system_manager::{
    CpuInfo, DiskInfo, MemoryInfo, NetworkInfo, SystemHealth, SystemInfo, SystemManager,
    SystemSummary,
};
pub use system_monitor::{
    CpuCore, DiskPartition, NetworkInterface, RamMemoryInfo, SystemMonitor, SystemMonitorStatistics,
};
pub use task_manager::{
    ProcessTaskManager, ProcessTaskStatistics, TaskProcessEntry, TaskProcessState,
};
pub use update_manager::{
    UpdateCategory, UpdateConfig, UpdateLevel, UpdateManager, UpdatePackage, UpdateStatistics,
};
pub use user::{Group, ShadowEntry, SudoPolicyEngine, SudoersRule, User, UserError, UserManager};
pub use user_manager::{UserAccount, UserManager as AccountManager, UserStatistics, UserType};

pub mod atomic_upgrade;

pub mod runit_supervisor;

pub mod omarchy_atreyu_system_plugin;
pub use omarchy_atreyu_system_plugin::*;

pub mod omarchy_docker_rootless_engine;
pub use omarchy_docker_rootless_engine::*;
pub mod gamemode_daemon;
pub mod boot_analyzer;
pub use preload::{
    PreloadEntry, PreloadManager,
    init_preload_manager, get_preload_manager, add_common_preloads,
};
pub use boot_analyzer::{
    BootAnalyzer, ServiceBootTime, ServiceStatus, BootAnalysis,
    init_boot_analyzer, get_boot_analyzer,
    analyze_boot_time, analyze_blame, analyze_critical_chain,
};
pub use task_scheduler::{
    TaskScheduler,
    ScheduledTask as SchedTask,
    ScheduleFrequency as SchedFrequency,
    TaskStatus as SchedTaskStatus,
    init_task_scheduler, get_task_scheduler,
};
pub use monitor::{
    SystemMonitor as SystemMonitorNew, CpuStats, MemoryStats, ProcessStats,
    ProcessState as MonitorProcessState,
    NetworkStats, DiskStats,
    init_system_monitor, get_system_monitor,
};

pub mod sovereign_init_and_supervisor_inspirations_pr;
pub use sovereign_init_and_supervisor_inspirations_pr::*;

pub use mint_tools::{
    SystemToolsEngine, BackupConfig, BackupJob, BackupType, BackupStatus,
    MintUpdateManager, PackageUpdate, UpdateType, UpdateSeverity,
    SoftwareManager, SoftwarePackage, SoftwareCategory,
    DriverManager, Driver, DriverType, DriverStatus,
    SystemReport, DiagnosticEntry, DiagnosticTest, TestResult,
    HardwareInfo, SoftwareInfo,
    init_system_tools, get_system_tools,
};
pub use network_manager::{
    NetworkManager, Connection, NetworkDevice,
    ConnectionType, ConnectionState, IPMethod, WiFiSecurity,
    IPAddress, DNSServer,
    init_network_manager, get_network_manager,
};
pub use system_monitor::{
    SystemMonitor as NewSystemMonitor, Process, ProcessState,
    CPUCore, MemoryInfo, DiskInfo, NetworkInterface,
    init_system_monitor as init_new_system_monitor, get_system_monitor as get_new_system_monitor,
};
