//! System Functions (Linux/BSD Command-Line Tools Inspiration)
//! Practical system administration functions and command-line tools

pub mod health;
pub mod logging;
pub mod monitoring;
pub mod network;
pub mod process;
pub mod security;
pub mod storage;
pub mod tuning;
pub mod update;
pub mod user;

pub use health::{
    DiagnosticModule, DiagnosticReport, DiagnosticsTool, HealthCheck, HealthCheckType,
    HealthChecker, HealthError as HealthCheckError, HealthStatus, RecoveryMode, RecoveryOption,
    RecoveryTool, SystemHealthStatus,
};
pub use logging::{
    Journal, JournalFile, JournalManager, LogAction, LogAlert, LogAnalyzer, LogError, LogFile,
    LogManager, LogPattern, LogRule, LogStats, LogTarget, LogTargetType, PatternSeverity,
};
pub use monitoring::{
    CpuStats, ExportFormat, HardwareInfo, IOStats, JournalEntry, JournalError, JournalViewer,
    LogFilter, LogPriority, MemoryStats, MonitorStats, ProcessInfo, SystemInfo, SystemMonitor,
};
pub use network::RuleAction;
pub use network::{
    AddressFamily, DriverInfo, Duplex, EthTool, IPAddress, InterfaceState, InterfaceStats,
    LinkSettings, NetworkConfig, NetworkDiagnostics, NetworkError, NetworkInterface, NetworkStats,
    PingResult, Route, Rule, TracerouteHop,
};
pub use process::{
    FileDescriptor, KernelParam, Process, ProcessError, ProcessManager, Service, ServiceManager,
    ServiceState, Socket, SystemControl, Thread, Timer,
};
pub use security::{
    AuthorizedKey, FirewallManager, FirewallRule, FirewallService, FirewallZone, KnownHost,
    PortRule, SELinuxBoolean, SELinuxContext, SELinuxManager, SELinuxMode, SSHKey, SSHKeyManager,
    SSHKeyType, SecurityError,
};
pub use storage::{
    BlockDevice, BlockDeviceManager, DeviceStats, Disk, Filesystem, FilesystemInfo,
    FilesystemManager, MountPoint, Partition, PartitionManager, PartitionTable, PartitionTableType,
    PartitionType, StorageError,
};
pub use tuning::{
    CPUProfile, DiskProfile, IOClass, IOScheduler, IOTuner, NetworkProfile, NetworkTuner,
    PerformanceTuner, QDisc, QDiscType, TrafficClass, TrafficFilter, TuningError, TuningProfile,
};
pub use update::{
    GPGKey, Package, PackageCache, PackageManager, Repository, RepositoryManager, SecuritySeverity,
    SecurityUpdate, Update, UpdateError, UpdateManager, UpdateSchedule,
};
pub use user::{
    AuthManager, AuthMethod, AuthModule, Group, HashAlgorithm, PasswordHash, PasswordManager,
    PasswordPolicy, User, UserError, UserGroup, UserManager,
};
