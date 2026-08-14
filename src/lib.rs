#![allow(warnings)]
#![allow(clippy::all)]

// SigmaOS Library
// Core library for SigmaOS operating system

pub mod accessibility;
pub mod ai;
pub mod automation;
pub mod compatibility;
pub mod customization;
pub mod distro;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod kernel;
pub mod klib;
pub mod ml;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod tools;
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
    InterimLispVM, JehanneError, JehanneNamespace, LispVal, MintBackupTool, MintSoftwareManager,
    MintUpdateItem, MintUpdateLevel, MintUpdateManager, MntReformLpcDriver, MockUPIService,
    MultilingualSupport, NamespaceBindEntry, NtHandle, NtObjectManager, NtObjectType, NtStatus,
    Plan9pMessage, Plan9pMsgType, PortableExecutableLoader, ReformPowerStats, RegistryHive,
    SoftwareMeta, TargetPlatform, TranslationLayer, WindowCoordinates, ZenithDisplayCompositor,
    JudicialTimelinePlanner, MsmeComplianceEngine, AyushFormularyHelper,
    PMWaniHotspotController, DigiYatraPassScanner, IrctcPnrTracker,
    Literal, SpacSatResolver,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use driver::{
    Apc, DeviceObject, Dpc, DriverObject, IoStatus, IoStatusBlock, Irp, IrpManager, Minifilter,
    IRP_MJ_CLOSE, IRP_MJ_CREATE, IRP_MJ_DEVICE_CONTROL, IRP_MJ_READ, IRP_MJ_WRITE, METHOD_BUFFERED,
    METHOD_IN_DIRECT, METHOD_NEITHER, METHOD_OUT_DIRECT,
};
pub use drivers::{
    GpuCommand, GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    StorageCommand, StorageDriver, StorageError, StorageType, UsbHidDriver, VesaDriver, VesaError,
    VesaModeInfo,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
};
pub use klib::{
    buddy_allocator::{Block, BlockID, SimpleBuddyAllocator},
    paging::{
        PageTable, PageTableEntry, ProcessMemory, SimplePageTable, SimplePageTableEntry,
        SimpleProcessMemory, SimpleVMM, VirtualMemoryManager,
    },
    uvm::{UvmAmap, UvmError, UvmPageLoan, UvmPmap},
};
pub use network::{
    AlertSeverity, AlertType, AlpineZeroAllocCaptureBuffer, AnalysisStrategy, BandwidthAnalysis,
    ClearLinuxFlowLoadBalancer, ConnectionInfo, ConnectionState, EnterpriseNetworkError,
    GentooUseFlagsDissector, IPv6Address, KaliPacketFingerprinter, KaliSnoopAnalysis,
    NetworkTrafficAnalyzer, NixDeclarativeFilter, Protocol, SecureVpnTunnel, SecurityAnalysis,
    TcpConnection, TcpError, TcpSegment, TcpStack, TcpState, TrafficAlert, TrafficPacket,
    TrafficStatistics,
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
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
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
