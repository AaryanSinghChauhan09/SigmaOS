#![allow(warnings)]
#![allow(clippy::all)]
extern crate alloc;
// SigmaOS Library
// Core library for SigmaOS operating system

pub mod access;
pub mod ai;
pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod crash;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod graphics;
pub mod kernel;
pub mod logging;
pub mod memory;
pub mod klib;
pub mod network;
pub mod orchestration;
pub mod distro;
pub mod package;
pub mod performance;
pub mod plugin;
pub mod power;
pub mod process;
pub mod productivity;
pub mod resilience;
pub mod runtime;
pub mod resource;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod tools;
pub mod tracing;
pub mod virtualization;

pub use access::{
    AccessError, AccessManager, AccessMode, AccessPattern, AccessResult, AccessRule,
    AccessTimeTracker, AnonymousAccessPolicy, DeviceAccessType, LdapAccessClient,
    LdapUserEntry, ProcessMigrationControl, ProtectionLevel, RemoteAccessController,
    RemoteAccessProtocol, RemoteAccessSession, RemoteFileHandle, SecurityAccessToken,
    WirelessAccessPoint, WirelessAccessPointManager,
};
pub use accessibility::{
    AccessibilityCategory, AccessibilityError, AccessibilityFeature, AccessibilityFramework,
    AccessibilityProfile, AccessibilitySetting,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use compatibility::{
    APITimelineManager, AkabeiBundle, AkabeiPackageEngine, AlternativeLink,
    AnanicyManager, AnacondaInstaller, AntixControlCenter, AntixDesktopProfiler, AntixInitManager,
    ApplicationBinary, AptRepositorySync, BinaryCompatMatrix,
    BinaryFormat, BodhiUpdateTriage, BoreSchedulerGovernor,
    BundleType, CachyInitramfs,
    CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, DebianAlternativesSystem, DebianChannel,
    DebootstrapEngine, DesktopProfile, DesktopTheme, DiscontinuedFS,
    DnfPackageResolver, DriverBridge, FSRevival,
    FedoraAlu, FedoraAluFlags, GraphicsBridge, InstallerStep,
    KapudanAssistant, KernelPersona, KernelPersonaVM, KojiBuildServer,
    LegacyBus, LegacyDriver, LegacyMemoryTrimmer, LegacyPluginManager, LibcVersion,
    MicroService, MicroServiceState, MockChrootBuilder, NetworkBridge,
    SeLinuxContext, SeLinuxEngine, SchedPolicy, SigmaChangeProposal,
    SigmaChangeProcessEngine, SigmaNextChannel, StorageBridge,
    SysVInitEngine, SysVRunlevel, SyscallAbi, SystemdPresetConfigurator,
    TargetPlatform, TranslationLayer, TribeInstaller,
    V4OptimizedPackageManager, WorkloadOptimizer, WorkloadProfile,
    GLOBAL_AKABEI, GLOBAL_ANTIX_CONTROL, GLOBAL_ANTIX_DESKTOP, GLOBAL_ANTIX_INIT, GLOBAL_KAPUDAN,
    GLOBAL_MEMORY_TRIMMER, GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER, GLOBAL_TRIBE,
    GLOBAL_WORKLOAD_OPTIMIZER,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use distro::{
    ArchDependencyResolver, PackageNode, FreeBSDJail, OpenBSDPledge, NixStyleStore,
    PinRule, AptPinStore, OpenRCService,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use drivers::{
    Bdle, Ch340Driver, DeviceGeneration, E1000Driver, GpuCommand, GpuCommandBuffer, GpuDriver,
    GpuError, GpuPipeline, GpuShader, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, IntelHdaDriver, NetworkCommand, NetworkDriver, NetworkError,
    NetworkType, NvmeCmd, NvmeCqe, NvmeDriver, PeripheralDevice, PeripheralManager, PowerState,
    RxDescriptor, ShaderStage, StorageCommand, StorageDriver, StorageError, StorageType,
    TxDescriptor, UsbHidDriver, VesaDriver, VesaError, VesaModeInfo,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, LegacyLinuxRule, LinuxPersonaRule,
    SmartSymlink, SymlinkResolverRule, VirtualFilesystem,
};
pub use graphics::{DrmAtomicPlaneState, WaylandDmaBuf, OpenBsdWsdisplayVt};
pub use logging::{
    ConsoleLogTarget, FileLogTarget, LogCompressor, LogError, LogFacility, LogField, LogFile,
    LogLevel, LogRotateConfig, LogRotator, LogSeverity, LogTarget, LoggerCapability,
    MemoryLogTarget, NetworkFramingFormat, NetworkLogTarget, NetworkProtocol, RotationPolicy,
    SimpleLogCompressor, SimpleLogFile, SimpleLogRotator, SimpleUnifiedLogger, SyslogFacility,
    TargetCapability, TargetInfo, TargetType, UnifiedLogEntry, UnifiedLogStats, UnifiedLogger,
};
pub use kernel::{
    AdvancedAlgorithmsManager, Apc, ApcMode, ApcQueue, ArchitectureEngine,
    AuditBlock, BuddyAllocator, Channel, CircularDoublyLinkedList, CpuArchitectureClass,
    CpuRegisters, EdfTask, HardwareException,
    IpcError, IpcManager, Irql,
    LcgRandom, LookasideList, LotteryTask, MemoryBlock,
    MemoryDescriptorList, Message, Pcb, PoolType, Priority, Process,
    ProcessState, ProcessorInitState, RoundRobinConfig, RoundRobinScheduler, Scheduler,
    SchedulerError, SequencedSinglyLinkedList, SinglyLinkedList, SystemThread,
    Tcb, ThreadState, WorkItem, PAGE_SIZE,
};
pub use network::{
    TcpConnection, TcpError, TcpSegment, TcpStack, TcpState,
    NpfFirewallEngine, NpfRule, NpfTable, NatRule, NatType, NpfAction, NpfDirection, FiveTuple, IpProtocol,
};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use performance::{
    CallGraph, CpuInstructionExtension, IPCError,
    ProcessProfile, Profile, ProfileType, Profiler, ProfilerError,
    SchedInstruction, SchedOpcode, SimdOptimizer, SimpleCallGraph, SimpleProfile, SimpleProfiler,
    SovereignSimdOptimizer, UdfSchedVm,
    VmPerformanceMetrics, ZeroCopyMetrics,
    ZeroCopyQueue, QUEUE_SIZE,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore, MindMapCreator, TerminalError,
};
pub use resilience::{
    BackupSnapshot, RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError,
    SelfHealingModule, SigmaTimeshift, SystemSnapshot,
};
pub use security::{
    AnonSurfShunt, AppSandboxEngine, CapabilityGate, CapabilityToken, CronDaemon, CronJob,
    DefensiveAuditSystem, DmesgLog, FirewallRule, ForensicBlock, ForensicStorageFilter,
    IptablesFirewall, KaliError, MaliciousSignature, Permission, PluggableAuthenticationModule,
    PledgeManager, PledgePromise, RoutingMode, SandboxPolicy, SudoPrivilegeEscalation,
    SwapSpaceManager, TmuxMultiplexer, TmuxPane, GLOBAL_ANONSURF, GLOBAL_FORENSIC, GLOBAL_SANDBOX,
    MAX_AUDIT_BLOCKS, MAX_SIGNATURES, SIGNATURE_LEN,
    BinaryProtectionManager, RelroMode, AslrMap, ChecksecReport,
};
pub use shell::{ShellCommand, ShellRepl};
pub use tools::{
    AlmeidaCmosRtc, AlmeidaCoreDump, ClusterNode, NodeState, SigmaAccess,
    SigmaCluster, SigmaDeploy, SigmaIdentity, SigmaMonitor, SigmaPatch, SigmaRescue, SigmaToolError,
    SovereignAptDuo, SovereignDpkgEtcher, SovereignIPCalculator, SovereignImeConvertCase,
    SovereignImageToDataUri, SovereignJsonPrettifier, SovereignKeyboardTester, SovereignIsWebsiteDown,
    SovereignPasswordGenerator, SovereignTableConverter, SovereignTextFixer, SovereignWordCounter,
    UserIdentity,
};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
pub use memory::{
    BsdZoneAllocator, LinuxKswapd, MemCgroupManager, SimpleVMM, Zone, MemCgroup, PageState,
};
