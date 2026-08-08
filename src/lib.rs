#![allow(warnings)]
#![allow(clippy::all)]

// SigmaOS Library
// Core library for SigmaOS operating system

pub mod accessibility;
pub mod ai;
pub mod automation;
pub mod compatibility;
pub mod crash;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod gpu;
pub mod kernel;
pub mod klib;
pub mod ml;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod process;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod virtualization;

pub use accessibility::{
    AccessibilityCategory, AccessibilityError, AccessibilityFeature, AccessibilityFramework,
    AccessibilityProfile, AccessibilitySetting,
};
pub use ai::{
    agent::{AIAgent, AIAgentManager, AIError, AIStats, SimpleAIAgent, SimpleAIAgentManager},
    orchestrator::{ContextWindowPruner, DeviceTarget, LocalLlmOrchestrator, OrchestratorError},
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use compatibility::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ComputeNode, ContainerRuntime, DistributedComputeHandoff, GstCalculator, IndiaStackError,
    JehanneError, JehanneNamespace, MintBackupTool, MintSoftwareManager, MintUpdateItem,
    MintUpdateLevel, MintUpdateManager, MockUPIService, MultilingualSupport, NamespaceBindEntry,
    NtHandle, NtObjectManager, NtObjectType, NtStatus, Plan9pMessage, Plan9pMsgType,
    PortableExecutableLoader, RegistryHive, SoftwareMeta, TargetPlatform, TranslationLayer,
    WindowCoordinates, ZenithDisplayCompositor, SovereignNamespaceType, SovereignNamespaceIsolation,
    SovereignSeccompFilter, FreeBsdJail, SovereignSandboxCoordinator,
};
pub use crash::{
    CpuRegisterDump, OopsReport, PiiAnonymizer, CrashReporter,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use drivers::{
    GpuCommand, GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    StorageCommand, StorageDriver, StorageError, StorageType, UsbHidDriver, VesaDriver, VesaError,
    VesaModeInfo,
};
pub use gpu::{
    GpuFrameFormat, GpuRecordedFrame, GpuRecorderStats, GpuScreenRecorder,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
    NumaTask, LockFreeTaskQueue, NumaNode, NumaScheduler,
};
pub use klib::{
    paging::{PageTableEntry, SimplePageTableEntry, PageTable, SimplePageTable, VirtualMemoryManager, SimpleVMM, ProcessMemory, SimpleProcessMemory},
    buddy_allocator::{BlockID, Block, SimpleBuddyAllocator},
    uvm::{UvmPmap, UvmAmap, UvmPageLoan, UvmError},
};
pub use network::{
    EnterpriseNetworkError, IPv6Address, SecureVpnTunnel, TcpConnection, TcpError, TcpSegment,
    TcpStack, TcpState, UnixSocketAddress, UnixSocketState, UnixSocketConn, UnixSocketRegistry,
};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager, SovereignApp, SovereignApm,
    SovereignIsolationLevel,
};
pub use process::{
    NiceValue as LinuxNiceValue, CGroup as LinuxCGroup, PidNamespace as LinuxPidNamespace,
    LinuxProcessEntry, LinuxProcessState, LinuxSignal, ProcFileSystem, SysfsAttribute,
    LoopDevice, SysfsRegistry,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot, SovereignProblemType, SovereignRemediationAction, SovereignFixerStats,
    AutomatedFixerDaemon,
};
pub use security::{
    CapabilityGate, CapabilityToken, CronDaemon, CronJob, DefaultDenyNetworkPolicy, DmesgLog,
    FirewallRule, IptablesFirewall, KaliError, NemoClawError, OpenShellAgentSandbox, Permission,
    PledgeManager, PledgePromise, PluggableAuthenticationModule, PrivacyRouter,
    SudoPrivilegeEscalation, SwapSpaceManager, TmuxMultiplexer, TmuxPane,
};
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, DebianPackageTranslator,
    LinuxPackageCompatManager, LinuxPackageType, MakePkgEngine, PackageRecipe, PacmanError,
    PacmanManager, PkgBuildScript, RecipeError, RecipeManager, RpmPackageTranslator, SatSolver,
    Transaction, TranslatedMetadata, TranslatorError,
};
pub use virtualization::{
    Container, DeterministicError, DeterministicHypervisor, DeterministicVirtualMachine,
    KubernetesPod, ResourcePool, VirtualCpuContext, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmExecutionSnapshot, VmState,
};
