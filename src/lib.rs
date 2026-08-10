#![allow(warnings)]
#![allow(clippy::all)]

// SigmaOS Library
// Core library for SigmaOS operating system

pub mod accessibility;
pub mod ai;
pub mod automation;
pub mod compatibility;
pub mod customization;
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
pub mod virtualization;
pub mod unimplemented_features;
pub mod unimplemented_tools;
pub mod logging;
pub mod graphics;

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
    WindowCoordinates, ZenithDisplayCompositor,
    InterimLispVM, LispVal, MntReformLpcDriver, ReformPowerStats,
    ZorinAppearanceSwitcher, ZorinLayoutPreset, ZorinConnectHub, ZorinWineLayer, ZorinLiteOptimizer,
    SigmaEcosystemInit, FhsRunlevel, SigmaEcosystemProfiler, GraphicPresetMode,
    SigmaOnboardingWelcome, SigmaOnboardingLog,
    SigmaSupportSubtitleSync, SigmaSupportSubtitleEdit, SubtitleFormat,
    SigmaSupportResourceOptimizer, SigmaSupportPriorityOptimizer,
    BoreSchedulerGovernor, AnanicyManager, SchedPolicy, V4OptimizedPackageManager,
    CachyInitramfs, CachyThpTuner, ThpMode, CachyKsmDaemon, KsmPageEntry,
    CachyLatencyGovernor, GovernorPerformanceState, CachyMicroarchCompilerTuner,
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
pub use driver::{
    Irp, DriverObject, DeviceObject, IoStatus, IoStatusBlock, Apc, Dpc, Minifilter,
    IRP_MJ_CREATE, IRP_MJ_CLOSE, IRP_MJ_READ, IRP_MJ_WRITE, IRP_MJ_DEVICE_CONTROL,
    METHOD_BUFFERED, METHOD_IN_DIRECT, METHOD_OUT_DIRECT, METHOD_NEITHER,
    IoManager, DriverEntry, OpaqueDriverExtension,
    ObjectManager, ObjectType, NonPagedPool, RootkitDetector, IrpParameters,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
    SigmaFS, SigmaFhsRouter, SigmaFhsHook, SigmaFhsNamespace, SigmaFhsAuditor,
    SigmaDisasterRecoveryCleaner, SigmaFsJournal, SigmaFsCow, SigmaFsVolume,
    SigmaFsRaid, SigmaFsCrypt, SigmaFsVirtio, FileBlock, RaidLevel, JournalState,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
};
pub use klib::{
    paging::{PageTableEntry, SimplePageTableEntry, PageTable, SimplePageTable, VirtualMemoryManager, SimpleVMM, ProcessMemory, SimpleProcessMemory},
    buddy_allocator::{BlockID, Block, SimpleBuddyAllocator},
    uvm::{UvmPmap, UvmAmap, UvmPageLoan, UvmError},
};
pub use network::{
    EnterpriseNetworkError, IPv6Address, SecureVpnTunnel, TcpConnection, TcpError, TcpSegment,
    TcpStack, TcpState,
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
    TopCommand, ProcessTaskInfo, IfconfigCommand, NetworkInterface, PingCommand, PingResult,
    UnifiedSettingsManager, UserAccount, DisplayPreference, InputDeviceSettings,
    EverythingSearchEngine, NotepadPlusPlusBuffer, SovereignBrowserEngine, SevenZipEngine,
    CompressionMethod, FlameshotAnnotator, AnnotationShape, ObsStudioMixer,
    AudacityWaveEditor, VlcCodecPipeline, DaVinciTimeline, OneCommanderFileGrid,
    ItemAgeColor, EarTrumpetVolumeMatrix, IrfanViewEngine,
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
    PenetrationAssistant, ExploitPayload, Vulnerability, VulnerabilityScanner, SimpleVulnerability,
    SimpleVulnerabilityScanner, Severity, ScanSummary, ScanReport,
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
    DaemonlessContainer, K3osOrchestrator, ContainerState, RancherError,
};
pub use logging::{
    SimpleLogFile, SimpleLogRotator, SimpleLogCompressor, LogSeverity, LogFacility,
};
pub use graphics::ColorRgba;
