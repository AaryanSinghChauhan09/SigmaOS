//! System Functions (Linux/BSD Command-Line Tools Inspiration)
//! Practical system administration functions and command-line tools

pub mod monitoring;
pub mod network;
pub mod security;
pub mod storage;
pub mod process;
pub mod user;
pub mod update;
pub mod logging;
pub mod tuning;
pub mod health;

pub use monitoring::{
    JournalEntry, LogPriority, LogFilter, JournalViewer, ExportFormat, JournalError,
    CpuStats, MemoryStats, ProcessInfo, IOStats, SystemMonitor, MonitorStats,
    SystemInfo, HardwareInfo,
};
pub use network::{
    NetworkInterface, InterfaceState, IPAddress, AddressFamily, Route, Rule, RuleAction,
    NetworkConfig, PingResult, TracerouteHop, NetworkDiagnostics, NetworkStats,
    InterfaceStats, DriverInfo, LinkSettings, Duplex, EthTool, NetworkError,
};
pub use security::{
    FirewallZone, PortRule, FirewallService, FirewallRule, RuleAction, FirewallManager,
    SELinuxMode, SELinuxBoolean, SELinuxContext, SELinuxManager,
    SSHKey, SSHKeyType, SSHKeyManager, AuthorizedKey, KnownHost, SecurityError,
};
pub use storage::{
    BlockDevice, Partition, PartitionType, FilesystemInfo, BlockDeviceManager, DeviceStats,
    Disk, PartitionTableType, PartitionTable, PartitionManager,
    Filesystem, MountPoint, FilesystemManager, StorageError,
};
pub use process::{
    Service, ServiceState, Socket, Timer, ServiceManager,
    Process, Thread, FileDescriptor, ProcessManager,
    KernelParam, SystemControl, ProcessError,
};
pub use user::{
    User, Group, UserGroup, UserManager,
    PasswordPolicy, PasswordHash, HashAlgorithm, PasswordManager,
    AuthModule, AuthMethod, AuthManager, UserError,
};
pub use update::{
    Package, Repository, PackageCache, PackageManager,
    Update, SecurityUpdate, SecuritySeverity, UpdateSchedule, UpdateManager,
    GPGKey, RepositoryManager, UpdateError,
};
pub use logging::{
    LogFile, LogRule, LogAction, LogTarget, LogTargetType, LogManager, LogStats,
    Journal, JournalFile, JournalManager,
    LogPattern, PatternSeverity, LogAlert, LogAnalyzer, LogError,
};
pub use tuning::{
    TuningProfile, CPUProfile, DiskProfile, NetworkProfile, PerformanceTuner,
    IOClass, IOScheduler, IOTuner,
    QDisc, QDiscType, TrafficClass, TrafficFilter, NetworkTuner, TuningError,
};
pub use health::{
    HealthCheck, HealthCheckType, HealthStatus, SystemHealthStatus, HealthChecker,
    DiagnosticModule, DiagnosticReport, DiagnosticsTool,
    RecoveryMode, RecoveryOption, RecoveryTool, HealthError as HealthCheckError,
};