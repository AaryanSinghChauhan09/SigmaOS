// SigmaOS Library
// Core library for SigmaOS operating system

pub mod accessibility;
pub mod ai;
pub mod automation;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod distro;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod governance;
pub mod kernel;
pub mod klib;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod storage;
pub mod thread;
pub mod process;
pub mod community;
pub mod memory;
pub mod access;
pub mod tools;
pub mod unimplemented_features;
pub mod unimplemented_tools;
pub mod virtualization;
pub mod cluster;

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
    ScriptArgumentRouter,
};
pub use compatibility::{
    ApplicationBinary, BinaryFormat, CompatibilityError,
    CompatibilityManager, CompatibilityMode, ContainerRuntime,
    TargetPlatform, TranslationLayer,
};
pub use container::{
    ContainerError, ContainerID, ContainerInfo,
    ContainerRuntime as CoreContainerRuntime, ContainerState, RuntimeCapability, RuntimeStats,
    SimpleContainer, SimpleContainerRuntime,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use dashboard::statutory_compliance::{
    ComplianceRuleStatus, DisputeAuditRollbackEngine, PenaltyBreachNotifier, StatutoryBreachAlert,
    StatutoryFramework, StatutoryGovernanceLayer, StatutoryGovernanceRule,
};
pub use driver::pci_bus::{
    PciAddress, PciBarInfo, PciBarType, PciBusManager, PciDeviceNode, PciDriverMatchRule,
    PciHardwareAccess, PciHeaderType, PciInterruptMode, PcieAerLog, PcieAerSeverity,
    PcieAspmState, SimulatedPciHardwareAccess,
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
pub use governance::{
    FoundationModel, FoundationMember, ReleaseType, RoadmapMilestone, TransparentRoadmap,
    DemocraticProposal, DemocraticVoting,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, Message, MemoryBlock, PAGE_SIZE,
    Priority, Process, ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler,
    VirtualCpu,
    IoUringEngine, IoUringOpcode, SubmissionQueueEntry, CompletionQueueEntry,
};
pub use kernel::roundrobin::SchedulerError as RoundRobinSchedulerError;
pub use network::{
    compute_checksum as compute_net_checksum, IPv4Address, NetworkPacket, PacketRingBuffer,
    RingTcpState, TcpConnection, TcpError, TcpSegment, TcpSocket, TcpStack, TcpState,
    ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
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
    NixOSFlakeEngine, FlakeInput, SystemClosure,
    ArchPacmanHooksManager, PacmanHook, HookWhen, HookAction,
    VoidRunitSupervisor, RunitService, ServiceState,
    GentooPortageUseFlagsEngine, PortagePackage,
    MicroArchLevel, CachyKernelVariant, CpuCapabilities, BoreSchedulerGovernor, CachyPackageRepo,
    ArchBuildSystem, PacmanSyncManager, PacmanSyncPackage, ArchMirror, AurPackage, AurHelper,
    ArchRepoType,
    InstallationTarget, InstallerStep, InstallerError, LiveInstaller, SovereignInstaller,
    UpdateChannel, SystemStateStatus, UpdateError, ChannelManager, SovereignChannelManager,
    SigmaAppBundle, BundleError, AppBundleRuntime, SovereignBundleRuntime,
    CpuArchitecture, HalError, HardwareAbstractionLayer, SovereignHal,
    DllLoader, DllModule, GdiObjectType, LinuxSyscall, PosixTranslation, RegistryType,
    RegistryValue, Win32Gdi, WindowsRegistry,
    BuildJob, BuildStatus, CrossBuildPipeline, DevTool, DeveloperToolkit, PackageBuildService,
    TargetArch,
    AuditResult, AuditRule, ComplianceAuditor, ConfigHook, DirectoryService, DirectoryUser,
    ImeCandidate, InputMethodEngine, LanguagePack, LocaleManager, RegionalSettings,
    AdminAction, AiSysAdmin, IntegrityState, LivepatchManager, LivepatchPatch, NetplanConfig,
    NetplanManager, P2pNode, PqcSelfHealing, SovereignP2PSync, TimeTravelCheckpoint,
    TimeTravelEngine,
    BackupSnapshot, BackupSystem, KernelTrace, LiveDebugger, RescueISO, RescueISOManager,
    CanFrame, EcuController, EduChallenge, EduPlayground, HpcClusterJob, HpcJobState,
    MpiCommunicator, AptCacheSimulator, DpkgMultiArch, DebianPolicyEnforcer,
    ThreeTierReleaseModel, DebianSocialContract, FreezeBasedStabilization,
    TinyCoreRAMEngine, TinyCoreMode, TczExtensionManager, AppsAuditTool,
    BsdSecureNtpConstraintSync, BsdStatefulPacketFilter, DaxMemoryRegion, DragonFlyHammerFs,
    Hammer2MultiMasterPfsReplication, Hammer2Snapshot, Hammer2TxgRecord, PfRuleAction,
    PfStateEntry, PfStateSynchronizationEngine, PfSyncMessage, PfSyncMsgType, PfsClusterNode,
    RunitServiceState, SovereignAnonScrubber, SovereignDeltaPackageSigner,
    SovereignDeltaPatch, TlsConstraint, VirtioFsZeroCopyBridge, VoidRunitManager,
    SlackPackage, SlackwarePkgTools, SlackBuildCompiler, GuixDerivation, GuixFunctionalStore,
    ShepherdServiceState, ShepherdService, GNUGuixShepherdSupervisor, OstreeDeployment,
    OstreeDeploymentEngine, CrossbowVnic, SolarisCrossbowVnicEngine, RumpKernelServer,
    NetBsdRumpKernel, NetplanInterface, NetplanYamlRenderer, CloudInitBootstrapEngine,
    YastSetting, Yast2ControlCenter, SnapperType, SnapperSnapshot, SnapperBtrfsEngine,
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
pub use remote::{
    FileTransfer, RemoteDesktop, RemoteError, RemoteSession,
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
pub use security::hardening;
pub use security::{
    AnonSurfShunt, AppSandboxEngine, CapabilityGate, CapabilityToken,
    ArithmeticSubstitutionDeobfuscator,
    ForensicStorageFilter, Permission, PledgeManager,
    PledgePromise, RoutingMode, SandboxPolicy,
};
pub use shell::{ShellCommand, SimpleShellSession as ShellRepl};
pub use sigpkg::{
    AptDebManifest, BuildSystem, ContentAddressedStore, CryptoVerifier,
    PackageRecipe, RecipeError, RecipeManager, SatSolver,
    Transaction,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
pub use unimplemented_tools::{
    AudioEditor, PodcastRecorder, SubtitleEditor, MemoryLeakDetector, GamifiedTodo, MindMapCreator,
    GameHubLauncher, EmulatorManager, GameRecorder, GamePerformanceBooster, CloudGaming, VrArRuntime,
    ControllerMapper, GameModManager, AiDifficultyDirector, GanttChartPlanner, PdfEditor,
    DocumentScanner, CodeProfiler, StaticAnalyzer, PackagePublishingHub, AdaptiveUxAgent,
    AiSearchAssistant, NaturalLanguageShell, AiCodeAssistant, AiFileOrganizer, SmartNotificationManager,
    MeshNetworking, IotDeviceManager, CloudBackupUtility, SecureFileSharing,
    AiScheduler, GuiAppStore, MultiMonitorManager, GestureControl, VoiceControl, AiTaskbar,
    CrossDeviceSync, FlatpakSnapLayer, DeclarativeBuildSystem, AiDependencyResolver, AiAnomalyFirewall,
    SecureContainer, PrivacyDashboard, OfflinePackageInstaller, AppSandboxing, CrossLanguageBuildTool,
    PluginMarketplace, MusicLibraryManager, FedoraToolboxContainerEngine, NixHomeManagerEnvironment,
    MiseUniversalVersionManager, DevenvReproducibleEnvironment, AircrackWirelessAuditor,
    UbuntuProLivepatchEngine, FlatpakSdkContainerBuilder, ClearLinuxStatelessEngine,
};

pub mod init {
    pub mod systemd_init;
}
pub use init::systemd_init::{SystemdEngine, SystemdUnit, UnitState, UnitType};

pub use ai::next_gen::{
    AIModel, AdaptiveKernelPersona, AiTask, DeviceTargetType,
    EnergyGovernorMode, ModelType, MultiModelOrchestrator, PredictiveSyscallTranslator,
    WorkloadType,
};
pub use ai::wandr::{
    ResearchResult, SigmaWandrAgent, WandrDocument, WandrEvaluator, WandrResearchAgent, WandrTask,
};

pub use community::toolkit::{
    ArticleCategory, CommunityHandbookCatalog, HandbookArticle, PackageRecipe as CommunityPackageRecipe,
    RecipeSourceFormat, ReproduciblePackageRecipeManager, SecurityModelType,
    SecurityProfileTemplateStore, SecurityTemplate, HybridFirewallTemplateStore, VirtualizationBlueprintStore,
};

pub use tools::{
    AccessibilityFeature as LibAccessibilityFeature, ClusterNode as LibClusterNode, NodeState as LibNodeState,
    SigmaAccess as LibSigmaAccess, SigmaCluster as LibSigmaCluster, SigmaDeploy as LibSigmaDeploy,
    SigmaIdentity as LibSigmaIdentity, SigmaToolError as LibSigmaToolError, UserIdentity as LibUserIdentity,
    SovereignDpkgEtcher, SovereignAptDuo, SovereignImeConvertCase, SovereignTableConverter,
    SovereignWordCounter, SovereignTextFixer, SovereignImageToDataUri, SovereignKeyboardTester,
    SovereignIsWebsiteDown,
};

pub mod open_source_obsoletion;
pub use open_source_obsoletion::*;
