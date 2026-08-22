// SigmaOS Library
// Core library for SigmaOS operating system

extern crate alloc;

pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod container;
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
pub mod observability;
pub mod orchestration;
pub mod distro;
pub mod package;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod thread;
pub mod process;
pub mod community;
pub mod memory;
pub mod tools;
pub mod virtualization;
pub mod unimplemented_features;
pub mod unimplemented_tools;
pub mod graphics {
    pub mod compositor;
    pub mod paint;
    pub mod video;
}
pub mod hardware {
    pub mod compatibility;
    pub mod win32;
}
pub mod power {
    pub mod governor;
}
pub mod ai {
    pub mod agent;
    pub mod orchestrator;
}
pub mod boot;
pub mod toolchain {
    pub mod adapter;
    pub mod capsule;
    pub mod codex;
    pub mod bootstrap;
}
pub mod scheduler {
    pub mod numa_scheduler;
}
pub mod crypto {
    pub mod vectorized_pqc;
}

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
    AntiXInitSystem, AntiXServiceState, AntiXService, AntiXInitSwitcher,
    AntiXPersistenceMode, AntiXPersistenceManager, AntiXSystemRemasterEngine,
    AntiXControlCentre, ZorinLayout, ZorinLayoutMetrics, ZorinLayoutSwitcher,
    ZorinChameleonColor, ZorinChameleonEngine, ZorinConnectState, ZorinConnectManager,
    ZorinWindowsAppSupport, ApplicationBinary, BinaryFormat, CompatibilityError,
    CompatibilityManager, CompatibilityMode, ContainerRuntime, TargetPlatform,
    TranslationLayer,
};
pub use container::{
    ContainerCapability, ContainerError, ContainerID, ContainerInfo,
    ContainerRuntime as CoreContainerRuntime, ContainerState, RuntimeCapability,
    SimpleContainer, SimpleContainerRuntime,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use dashboard::statutory_compliance::{
    ComplianceRuleStatus, DisputeAuditRollbackEngine, PenaltyBreachNotifier,
    StatutoryBreachAlert, StatutoryFramework, StatutoryGovernanceLayer, StatutoryGovernanceRule,
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
    AiNativeRuntime, BuddyAllocator, Channel, EnergyAwareScheduler, FastPathIpc,
    InterruptMechanism, IpcError, IpcManager,
    MemoryBlock, Message, PAGE_SIZE, PolicyError, PolicyManager, PrivacyFirstSandbox, Priority, ProcessState,
    ProtectionDomain, PrivilegeLevel, ResourceBroker, RoundRobinConfig, RoundRobinScheduler,
    Scheduler, SchedulerError, SelfHealingKernel, SigmaFsPlusPlus, UniversalAbiTranslator,
    UserDefinedKernelFunctions,
};
pub use network::{
    TcpConnection, TcpError, TcpSegment, TcpStack, TcpState,
};
pub use observability::{
    ObservabilityError, ObservabilityStack, SigmaDebug, SigmaMetrics, SigmaTrace,
    SimpleObservabilityStack,
};
pub use distro::{
    AppManifest, CertificationStatus, ComponentType, HardwareCertificate,
    HardwareCertificationProgram, HardwareProfile, HardwareRegressionSuite, QAStagedRelease,
    ReleaseStage, SoftwareCertificationProgram,
    BountyStatus, BugBountyProgram, BugBountyReport, CommunityConference, ConferenceTalk,
    ForumChannel, ForumPost, HelpSystem, HowToGuide, ManPage, WikiPage,
    DllLoader, DllModule, GdiObjectType, LinuxSyscall, PosixTranslation, RegistryType,
    RegistryValue, Win32Gdi, WindowsRegistry,
    BuildJob, BuildStatus, CrossBuildPipeline, DevTool, DeveloperToolkit, PackageBuildService,
    TargetArch,
    AuditResult, AuditRule, ComplianceAuditor, ConfigHook, DirectoryService, DirectoryUser,
    ImeCandidate, InputMethodEngine, LanguagePack, LocaleManager, RegionalSettings,
    AdminAction, AiSysAdmin, IntegrityState, P2pNode, PqcSelfHealing, SovereignP2PSync,
    TimeTravelCheckpoint, TimeTravelEngine, NetplanConfig, NetplanManager,
    LivepatchPatch, LivepatchManager,
    BackupSnapshot, BackupSystem, KernelTrace, LiveDebugger, RescueISO, RescueISOManager,
    CanFrame, EcuController, EduChallenge, EduPlayground, HpcClusterJob, HpcJobState,
    MpiCommunicator,
};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use remote::{
    FileTransfer, RemoteError, RemoteSession,
    RemoteShell, SessionID, SessionState, ShellError, ShellID, ShellManager,
    SimpleFileTransfer, SimpleRemoteDesktop, SimpleRemoteSession, SimpleScreenSharing,
    SimpleShellManager,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
    SplitDirection as TmuxSplitDirection, LayoutPreset as TmuxLayoutPreset,
    TmuxPane, TmuxWindow, TmuxSession, TmuxSessionManager,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{
    CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise, SecurityEnforcer as AndroidStyleSecurityEnforcer,
    PORT_ALLOW_SSL, PORT_ALLOW_TCP,
};
pub use shell::{
    CommandError as ShellCommandError, ShellCommand, ShellRepl, ShellSession, SimpleShellSession,
};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction, Version, AdapterError,
    DebAdapter, RpmAdapter, PacmanAdapter,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};

pub use thread::{
    Thread, ThreadError, Mutex,
};

pub use process::spawn::{
    ProcessID, ProcessState as LibProcessState, ProcessError, Process, SimpleProcess, ProcessSpawner, SimpleProcessSpawner, ProcessWaiter, SimpleProcessWaiter,
    CLONE_NEWNS, CLONE_NEWNET, CLONE_NEWPID,
};
pub use process::activity_manager::{
    ActivityManager, ActivityState, ProcessActivityRecord, RegisterSnapshot, AddressSpaceBinding,
};
pub use memory::segmentation_paging::{
    AddressBindingMode, AddressType, AslrEntropyConfig, CpuRing, ExecutableAddressBinding,
    RandomizedAddressSpace, SegmentDescriptor, SegmentSelector, SegmentationPagingEngine,
    SpaceProtectionFlags, SystemControlRegisters,
};

pub use community::toolkit::{
    ArticleCategory, CommunityHandbookCatalog, HandbookArticle,
    RecipeSourceFormat, ReproduciblePackageRecipeManager, SecurityModelType,
    SecurityProfileTemplateStore, SecurityTemplate,
};

pub use tools::{
    AccessibilityFeature as LibAccessibilityFeature, ClusterNode as LibClusterNode, NodeState as LibNodeState,
    SigmaAccess as LibSigmaAccess, SigmaCluster as LibSigmaCluster, SigmaDeploy as LibSigmaDeploy,
    SigmaIdentity as LibSigmaIdentity, SigmaToolError as LibSigmaToolError, UserIdentity as LibUserIdentity,
    SovereignDpkgEtcher, SovereignAptDuo, SovereignImeConvertCase, SovereignTableConverter,
    SovereignWordCounter, SovereignTextFixer, SovereignImageToDataUri, SovereignKeyboardTester,
    SovereignIsWebsiteDown,
};

pub use unimplemented_features::{
    GenerationManager, PciBusScanner, SovereignIpcBus, SignalDispatcher as UnimplSignalDispatcher,
    PagingController, PackageDependencyResolver as UnimplPackageDependencyResolver,
    SecurityEnforcer as UnimplSecurityEnforcer, ZenithCompositor as UnimplZenithCompositor,
    MultiCallShell, GdtEntry, NimPOSTManager, SigmaTrace as UnimplSigmaTrace,
    SigmaFsCasEngine, SovereignCleanupEngine, AutoResourceOptimizer,
    FedoraSELinuxMacEngine, FedoraSystemdSupervisor, FedoraDeltaRpmEngine,
    VirtualMemoryManager, ZeroCopyNetworkStack, SovereignVmm, ContainerIsolationGuard,
    SchedMlfq, SchedCfs, VirtioGpu, NvmeController, ApicTimer, HpetController,
};

pub use unimplemented_tools::{
    AudioEditor, PodcastRecorder, GifConverter, StreamingOverlayManager, WebcamEffects, SubtitleEditor,
    SmartCleanup, PerformanceOptimizer, DiskDefragmenter, DuplicateFileFinder, BatterySaver,
    MemoryLeakDetector, ProcessSandbox, StartupOptimizer as UnimplStartupOptimizer, SecureFileShredder,
    SystemRestoreSnapshot, AccessibilitySuite, PredictiveMaintenance, ApiTestingTool, GitGuiClient,
    GamifiedTodo, MindMapCreator, KanbanBoard, GameHubLauncher, EmulatorManager, GameRecorder,
    GamePerformanceBooster, CloudGaming, VrArRuntime, ControllerMapper, GameModManager,
    AiDifficultyDirector, GamifiedDesktop, GanttChartPlanner, PdfEditor, DocumentScanner, CodeProfiler,
    StaticAnalyzer, PackagePublishingHub, AdaptiveUxAgent, AiSearchAssistant, NaturalLanguageShell,
    AiCodeAssistant, AiFileOrganizer, SmartNotificationManager, RemoteDesktop, MeshNetworking,
    IotDeviceManager, CloudBackupUtility, SecureFileSharing, AiScheduler, AiComplianceDashboard,
    GuiAppStore, MultiMonitorManager, GestureControl, VoiceControl, AiTaskbar, CrossDeviceSync,
    FlatpakSnapLayer, DeclarativeBuildSystem, AiDependencyResolver, ZeroTrustTpmBoot, ForensicSnapshot,
    AiAnomalyFirewall, SecureContainer, PrivacyDashboard, OfflinePackageInstaller, AppSandboxing,
    CrossLanguageBuildTool, PluginMarketplace, MusicLibraryManager, PacketSniffer, VpnTunnelManager,
    ZeroKnowledgeVault, MarkdownNotebook, PartitionManager, VectorDraftEngine, VmGuestSupervisor, EmailClient,
};
