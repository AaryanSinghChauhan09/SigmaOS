// SigmaOS Library
// Core library for SigmaOS operating system

pub mod ai;
pub mod klib;
pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod customization;
pub mod distro;
pub mod dashboard;
pub mod desktop;
pub mod device;
pub mod distro;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod graphics;
pub mod ipc;
pub mod kernel;
pub mod klib;
pub mod legal;
pub mod ml;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod performance;

pub mod process;
pub mod productivity;
pub mod resilience;
pub mod resource;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod tools;
pub mod virtualization;

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
pub mod observability {
    pub mod profiler;
}
pub mod ai {
    pub mod agent;
    pub mod orchestrator;
}
pub mod boot;
pub mod toolchain {
    pub mod adapter;
    pub mod bootstrap;
    pub mod capsule;
    pub mod codex;
}
pub mod scheduler {
    pub mod numa_scheduler;
}
pub mod crypto {
    pub mod vectorized_pqc;
}

pub use accessibility::{
    AccessibilityCategory, AccessibilityError, AccessibilityFeature, AccessibilityFramework,
    AccessibilityProfile, AccessibilitySetting, BrailleDisplay, ColorFilter, KeyID, KeyType,
    Magnifier, MagnifierID, MagnifierManager, OnScreenKeyboard, ScreenReader, SimpleBrailleDisplay,
    SimpleColorFilter, SimpleMagnifier, SimpleMagnifierManager, SimpleOnScreenKeyboard,
    SimpleScreenReader, SimpleStickyKeys, SimpleVirtualKey, SimpleVoice, StickyKeys, VirtualKey,
    Voice, VoiceGender, VoiceID,
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
    ZorinWindowsAppSupport,
    ApplicationBinary, BIOSGatewayMesh, BinaryFormat, BuildCodexGrid, CompatibilityError,
    CompatibilityManager, CompatibilityMode, ConstellationNode, ContainerRuntime, CorebootGatewayMesh,
    DACConstellation, DotMatrixMesh, DriverArchiveGridV2, FhsConventionStatus, FileAlmanacHub,
    FirmwareGatewayMesh, FloppyMesh, GraphicsArchiveGridV2, KernelConstellationGrid,
    LegacyAsmCodexGrid, LegacyCCodexGrid, LegacyCppCodexGrid, LegacyDriverAdapter, LegacyFSAdapter,
    LegacyKernelAdapter, LegacyPackageAdapter, LegacyProtocolAdapter, LegacySecurityAdapter,
    LegacyUIAdapter, LsbProfile, NetworkAlmanacHub, NetworkArchiveGridV2, PeripheralArchiveMesh,
    PosixComplianceLevel, ProcessAlmanacHub, SELinuxConstellation, SecurityConstellation,
    StandardsComplianceManager, StorageArchiveGridV2, SyscallAlmanacHub, TapeMesh, TargetPlatform,
    TranslationLayer, UEFIGatewayMesh, ZeroTrustConstellation,
    EosMirrorReflector, EosWelcomeEngine, EosUpdateNotifier, EosLogTool, YayAurHelper,
    Mirror as EosMirror, WelcomeTab as EosWelcomeTab,
    SigmaSubiquity, SigmaNetplan, SigmaCloudInit, SigmaMultipass, SigmaCurtin,
    DnfPackageResolver, MockChrootBuilder, KojiBuildServer, BodhiUpdateTriage,
};
pub use container::{
    ContainerCapability, ContainerError, ContainerID, ContainerInfo,
    ContainerRuntime as CoreContainerRuntime, ContainerState, RuntimeCapability, RuntimeStats,
    SimpleContainer, SimpleContainerRuntime,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use distro::{
    AkmKernelManager, CalamaresConfig, CalamaresInstaller, EosLogTool, EosWelcomeApp,
    ReflectorMirrorManager, YayParuHelper,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use distro::{
    AdminAction, AiSysAdmin, AppManifest, AuditResult, AuditRule, BackupSnapshot, BackupSystem,
    BountyStatus, BugBountyProgram, BugBountyReport, BuildJob, BuildStatus, CanFrame,
    CertificationStatus, CommunityConference, ComplianceAuditor, ComponentType, ConferenceTalk,
    ConfigHook, CrossBuildPipeline, DevTool, DeveloperToolkit, DirectoryService, DirectoryUser,
    DllLoader, DllModule, EcuController, EduChallenge, EduPlayground, ForumChannel, ForumPost,
    GdiObjectType, HardwareCertificate, HardwareCertificationProgram, HardwareProfile,
    HardwareRegressionSuite, HelpSystem, HowToGuide, HpcClusterJob, HpcJobState, ImeCandidate,
    InputMethodEngine, IntegrityState, KernelTrace, LanguagePack, LinuxSyscall, LiveDebugger,
    LivepatchManager, LivepatchPatch, LocaleManager, ManPage, MpiCommunicator, NetplanConfig,
    NetplanManager, P2pNode, PackageBuildService, PosixTranslation, PqcSelfHealing,
    QAStagedRelease, RegionalSettings, RegistryType, RegistryValue, ReleaseStage, RescueISO,
    RescueISOManager, SoftwareCertificationProgram, SovereignP2PSync, TargetArch,
    TimeTravelCheckpoint, TimeTravelEngine, WikiPage, Win32Gdi, WindowsRegistry,
};
pub use drivers::{
    E1000RxDescriptor, E1000TxDescriptor, GpuCommand, GpuDriver, GpuError, HidError,
    HidKeyboardEvent, HidReportType, InputDriver, InputEvent, InputType, IntelE1000Driver,
    LegacyAudioAc97, ModernAudioIntelHda, ModernNvmeDriver, ModernUsbPrinterDriver,
    ModernWifiDriver, NetworkCommand, NetworkDriver, NetworkError, NetworkType, StorageCommand,
    StorageDriver, StorageError, StorageType, TouchJingosDriver, UsbHidDriver, VesaDriver,
    VesaError, VesaModeInfo, VirtioBlkDriver, VirtioDeviceType, VirtioMmioHeader, VirtioNetDriver,
    VirtioRngDriver,
};

pub use desktop::{
    Notification, SimpleNotification, NotificationManager, SimpleNotificationManager,
    NotificationUrgency, NotificationError,
};
pub use drivers::{
    GpuCommand, GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    StorageCommand, StorageDriver, StorageError, StorageType, UsbHidDriver, VesaDriver, VesaError,
    VesaModeInfo,
    UnifiedDmaBroker, SelfHealingDriverManager, DmaDescriptor, DeviceCommandType,
    DeviceTransactionLog, GLOBAL_DMA_BROKER, GLOBAL_HEALING_MANAGER,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, LegacyLinuxRule, LinuxPersonaRule,
    SmartSymlink, SymlinkResolverRule, VirtualFilesystem,
};
pub use graphics::paint::ColorRgba;
pub use ipc::{
    unix_socket::{UnixSocketType, UnixSocketAddress, UnixSocketState, UnixSocket, UnixSocketManager},
    signals::{SignalType, SignalDisposition, PendingSignal, ProcessSignalState, SignalDeliverySystem},
};
pub use kernel::{
    sched::{DragonFlySmpQueueManager, LwktMessage, PowerGovernor, SovereignGameMode, UksmPageDeduplicator},
    ABIManager, AcpiInterruptManager, AiNativeRuntime, BuddyAllocator, Channel,
    EnergyAwareScheduler, FastPathIpc, GapError, Generation, GenerationManager, InterruptMechanism,
    IpcError, IpcManager, IrqRoutingTable, JournalBlock, JournalState, KernelGraph, KernelPersona,
    KernelPlugin, KernelPluginManager, LegacyScheduler, MemoryBlock, Message, MetaKernel,
    MetadataJournal, MicroDriver, NetPod, Pml4PageTableEntry, PolicyError, PolicyManager, Priority,
    PrivacyFirstSandbox, PrivilegeLevel, Process, ProcessState, ProtectionDomain, ResourceBroker,
    RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, SelfHealingKernel,
    SigmaFsPlusPlus, UniversalAbiTranslator, UserDefinedKernelFunctions,
    VirtualMemoryPagingManager, PAGE_SIZE,
};
pub use legal::{
    ComplianceCert as LegalComplianceCert, ComplianceStatus as LegalComplianceStatus,
    ComponentLicense, GlobalStandard, InternationalComplianceTracker, LabourLawCompliance,
    LabourLawConfig, LegalComplianceRegistry, LicenseType, PatentRecord, RegulatoryControl,
    StatutoryFiling, StatutoryFilingDashboard, StatutoryPayrollBreakdown,
};
pub use network::{
    FirewallAction, FirewallCommand, FirewallFilterRule, IpRoute2Command, LinkState, PingCommand,
    SocketStatsCommand, SocketStatsEntry, TcpConnection, TcpError, TcpSegment, TcpStack, TcpState,
    UfwDefaultRule, GLOBAL_FIREWALL, GLOBAL_IP_COMMAND, GLOBAL_UFW_RULE,
};
pub use observability::{
    ObservabilityError, ObservabilityStack, SigmaDebug, SigmaMetrics, SigmaTrace,
    SimpleObservabilityStack,
    CognitiveOSNarrator, AdaptiveComplianceGater, SynestheticFeedbackEngine, GenerativeConfigParser, InterplanetaryDtnRoute, CollectiveSimulationNode,
};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageError, PackageFormat, PackageFormatAdapter,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, LayoutPreset as TmuxLayoutPreset,
    PomodoroState, PomodoroTimer, ProductivityScore, SplitDirection as TmuxSplitDirection,
    TmuxPane, TmuxSession, TmuxSessionManager, TmuxWindow,
};
pub use remote::{
    FileTransfer, InputAuthGate, PqcVideoCipher, RemoteDesktop, RemoteError, RemoteSession,
    RemoteShell, SessionID, SessionState, ShellError, ShellID, ShellManager, SigmaRendezvous,
    SimpleFileTransfer, SimpleRemoteDesktop, SimpleRemoteSession, SimpleScreenSharing,
    SimpleShellManager,
};
pub use resilience::{
    FsSnapshot, RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError,
    SelfHealingModule, SigmaTimeshift, SystemSnapshot, GLOBAL_TIMESHIFT,
};
pub use security::hardening;
pub use security::{
    CapabilityGate, CapabilityToken, DomainID, DomainOrchestrator, DomainType, IsolatedDomain,
    IsolationError, Permission, PledgeManager, PledgePromise, SecurityEnforcer as AndroidStyleSecurityEnforcer,
    PORT_ALLOW_SSL, PORT_ALLOW_TCP,
    AnonSurfShunt, AppSandboxEngine, ForensicStorageFilter, MacChanger, KaliPacketSniffer,
    HashAuditor, RoutingMode, PacketAnomaly, GLOBAL_ANONSURF, GLOBAL_SANDBOX, GLOBAL_FORENSIC,
    GLOBAL_MACCHANGER, GLOBAL_SNIFFER, GLOBAL_AUDITOR,

    AnonSurfShunt, AppSandboxEngine, CapabilityGate, CapabilityToken, CronDaemon, CronJob,
    DefensiveAuditSystem, DmesgLog, FirewallRule, ForensicBlock, ForensicStorageFilter,
    IptablesFirewall, KaliError, MaliciousSignature, Permission, PluggableAuthenticationModule,
    PledgeManager, PledgePromise, RoutingMode, SandboxPolicy, SudoPrivilegeEscalation,
    SwapSpaceManager, TmuxMultiplexer, TmuxPane, GLOBAL_ANONSURF, GLOBAL_FORENSIC, GLOBAL_SANDBOX,
    MAX_AUDIT_BLOCKS, MAX_SIGNATURES, SIGNATURE_LEN,
};
pub use shell::{ShellCommand, ShellRepl};
pub use tools::{
    AccessibilityFeature, AlmeidaCmosRtc, AlmeidaCoreDump, ClusterNode, NodeState, SigmaAccess,
    SigmaCluster, SigmaDeploy, SigmaIdentity, SigmaMonitor, SigmaPatch, SigmaRescue, SigmaToolError,
    SovereignAptDuo, SovereignDpkgEtcher, SovereignIPCalculator, SovereignImeConvertCase,
    SovereignImageToDataUri, SovereignJsonPrettifier, SovereignKeyboardTester, SovereignIsWebsiteDown,
    SovereignPasswordGenerator, SovereignTableConverter, SovereignTextFixer, SovereignWordCounter,
    UserIdentity,
};
pub use sigpkg::{
    AdapterError, BuildSystem, ContentAddressedStore, CryptoVerifier, DebAdapter, FileState,
    FileTransactionEntry, MirrorNode, PackageDependencyResolver,
    PackageFormatAdapter as SpecPackageFormatAdapter, PackageRecipe, PacmanAdapter, RecipeError,
    RecipeManager, RpmAdapter, SandboxRule, SatSolver, SovereignDeltaGenerator,
    SovereignMirrorSelector, SovereignSandboxEnforcer, SovereignTransactionManager, Transaction,
    UniversalPackageManager as SpecUniversalPackageManager, Version, MAX_RECIPE_DEPENDENCIES,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};

pub use thread::management::{
    SimpleThread, SimpleThreadManager, Thread, ThreadAlertableState, ThreadError, ThreadID,
    ThreadManager, ThreadState as LibThreadState,
};

// #[cfg(test)]
// #[path = "compatibility/fedora.rs"]
// pub mod fedora_compat_test;
// pub mod customization;
// pub mod dashboard;
// pub mod desktop;
// pub mod device;
// pub mod driver;
// pub mod filesystem;
// pub mod ml;
// pub mod network;
// pub mod observability;
// pub mod orchestration;
// pub mod distro;
// pub mod package;
// pub mod performance;
// pub mod productivity;
// pub mod remote;
// pub mod resilience;
// pub mod shell;
// pub mod sigpkg;
// pub mod virtualization;
// pub mod graphics {
//     pub mod compositor;
//     pub mod paint;
//     pub mod video;
// }
// pub mod hardware {
//     pub mod compatibility;
//     pub mod win32;
// }
// pub mod power {
//     pub mod governor;
// }
// pub mod ai {
//     pub mod agent;
//     pub mod orchestrator;
// }
// pub mod boot;
// pub mod system;
// pub mod installer;
pub use process::spawn::{
    Process, ProcessError, ProcessGroup, ProcessID, ProcessSpawner,
    ProcessState as LibProcessState, ProcessWaiter, SimpleProcess, SimpleProcessGroup,
    SimpleProcessSpawner, SimpleProcessWaiter, CLONE_NEWNET, CLONE_NEWNS, CLONE_NEWPID,
};

pub use tools::{
    AccessibilityFeature as LibAccessibilityFeature, ClusterNode as LibClusterNode,
    NodeState as LibNodeState, SigmaAccess as LibSigmaAccess, SigmaCluster as LibSigmaCluster,
    SigmaDeploy as LibSigmaDeploy, SigmaIdentity as LibSigmaIdentity,
    SigmaToolError as LibSigmaToolError, SovereignAptDuo, SovereignDpkgEtcher,
    SovereignImageToDataUri, SovereignImeConvertCase, SovereignIsWebsiteDown,
    SovereignKeyboardTester, SovereignTableConverter, SovereignTextFixer, SovereignWordCounter,
    UserIdentity as LibUserIdentity,
};

// Re-export strategic unimplemented tools and features
pub use unimplemented_features::{
    AiNativeRuntime as FeatureAiNativeRuntime, ContinuationTask, CrossDeviceContinuity,
    EnergyAwareScheduler as FeatureEnergyAwareScheduler, FsPluginType, GuestOsType, ModelProcess,
    ModelType as FeatureModelType, PrivacyFirstSandbox as FeaturePrivacyFirstSandbox, SigmaFsPlus,
    UniversalAbiTranslator, UserDefinedKernelFunctions as FeatureUserDefinedKernelFunctions,
};

pub use unimplemented_tools::{
    AccessibilitySuite, AdaptiveUxAgent, AiAnomalyFirewall, AiCodeAssistant, AiComplianceDashboard,
    AiDependencyResolver, AiDifficultyDirector, AiFileOrganizer, AiScheduler, AiSearchAssistant,
    AiTaskbar, ApiTestingTool, AppSandboxing, AppStoreItem, AudioEditor, AudioTrack,
    AutomationRoutine, BatterySaver, ButtonToKeyMapping, CameraFilter, CloudBackupUtility,
    CloudGaming, CodeProfiler, ControllerMapper, CrossDeviceSync as ToolCrossDeviceSync,
    CrossLanguageBuildTool, DeclarativeBuildSystem, DiagnosticMetric, DiskDefragmenter,
    DisplayScreen, DocumentScanner, DuplicateFileFinder, EmulatorCore, EmulatorManager,
    FlatpakSnapLayer, ForensicSnapshot, GameDetails, GameHubLauncher, GameModManager,
    GamePerformanceBooster, GameRecorder, GamifiedDesktop, GamifiedTodo, GamifiedTodoTask,
    GanttChartPlanner, GanttTask, GestureControl, GifConverter, GitCommitNode, GitGuiClient,
    GuiAppStore, IotDevice, IotDeviceManager, KanbanBoard, KanbanColumn, KanbanTask,
    MemoryLeakDetector, MeshNetworking as ToolMeshNetworking, MeshPeer, MindMapCreator,
    MindMapNode, MockHttpRequest, ModDetails, MultiMonitorManager, MusicLibraryManager, MusicTrack,
    NaturalLanguageShell, Notification, OfflinePackageInstaller, OverlayWidget,
    PackagePublishingHub, PdfEditor, PerformanceOptimizer, PluginDetails, PluginMarketplace,
    PodcastRecorder, PredictiveMaintenance, PrivacyDashboard, ProcessSandbox, ProfileSample,
    RemoteDesktop as ToolRemoteDesktop, SecureContainer as ToolSecureContainer, SecureFileSharing,
    SecureFileShredder, SmartCleanup, SmartNotificationManager, StartupOptimizer,
    StaticAnalysisWarning, StaticAnalyzer, StreamingOverlayManager, SubtitleEditor, SubtitleLine,
    SystemRestoreSnapshot, VoiceControl, VrArRuntime, VrPose, WebcamEffects, ZeroTrustTpmBoot,
};
