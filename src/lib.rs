// SigmaOS Library
// Core library for SigmaOS operating system

// Core working modules
pub mod community;
pub mod governance;
pub mod accessibility;
pub mod access;
pub mod ai;
pub mod app;
pub mod auth;
pub mod automation;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod desktop;
pub mod device;
pub mod driver;
pub mod crypto;
pub mod filesystem;
pub mod futuristic_modules;
pub mod kernel;
pub mod klib;
pub use klib::ZeroDependencyMasterHub;
pub mod memory;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod process;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod runtime;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod storage;
pub mod thread;
pub use process::{
    ProcessControlError, ProcessVmReadWriteEngine, JobState, CoreDumpMetadata, ProcessJobEntry,
    JobControlLifecycleEngine, WNOHANG, WUNTRACED, WCONTINUED, BsdRusage, WaitStatus,
    ProcessWaiterAndRusageCollector, CancellationType, ProcessCancelState,
    ProcessCancellationAndTerminationManager, PosixMessage, PosixMessageQueue, EventFd,
    SigQueuePayload, AdvancedIpcHub,
};
pub mod tools;
pub mod open_source_os_gap_closure;
pub use open_source_os_gap_closure::*;
pub mod sovereign_wiki_master_engine;
pub use sovereign_wiki_master_engine::*;
pub mod open_source_obsoletion;
pub mod unimplemented_features;
pub mod unimplemented_tools;
pub mod userland;

pub mod distro;
pub use distro::*;

pub use package::bsd_linux_package_innovations::{
    AlpineApkWorldAndVirtualPkgEngine, ApkIndexMetadata, ApkSignatureKey, ApkV3SignatureEngine,
    AptBugReport, AptMarkRecord, AptMarkState, AptPinRule, ArchCachyosMicroarchOptimizationEngine,
    ArchCachyOsMicroarchBuildProfileEngine, ArchSplitPackageHookRunnerEngine, CasStorePath,
    CachedPackageFile, CommunityPackageBuildSource, CommunityRepoBackend,
    CoprAurBuildRepositoryGatewayEngine, DebconfPreseedEntry, DebconfQuestionType,
    DebianAptMarkPackageStateGovernor, DebianDebconfStatoverrideEngine,
    DebianDpkgTriggersAptListbugsGuardEngine, DeltaRpmSpec, DnfActionKind, DnfActionRecord,
    DnfTransactionItem, DpkgDivertEngine, DpkgDivertRule, DpkgStatoverrideRule, DpkgTrigger,
    DpkgTriggerKind, DragonFlyDportsHammer2SnapshotEngine, EbuildSlotRecord,
    FedoraDnf5AdvisoryAndDeltaRpmEngine, FedoraDnf5AdvisorySecurityEngine,
    FedoraDnfHistoryRollbackJournalEngine, FlakeInputLock, FreeBsdPkgAuditEngine,
    FreeBsdPortsFlavoursAndVuxmlEngine, GentooPortageEapiSlotOperatorEngine,
    GentooPortageSubslotAndUseExpandEngine, HaikuHpkgPackageFsEngine, Hammer2PfsSnapshot,
    MicroarchCompilerFlags, MicroarchRepoRoute, MicroarchitectureLevel, NetBsdPkginBinaryDatabaseEngine,
    NetBsdPkgsrcOptionsFrameworkEngine, NixCasStoreGcGovernor, NixFlakesDevshellResolverEngine,
    NixGuixCasGcProfileEngine, OpenBsdPkgAddSignifyEngine, OpenBsdSignifyBinaryIntegrityEngine,
    OpenSuseZypperVendorStickinessEngine, PacmanGpgKey, PacmanKeyTrust, PacmanKeyringEngine,
    PackageBuildAttestation, PackageBuildEnvironment, PkgAuditAdvisory, PkgSummaryRecord,
    PkgsrcOptionSpec, PortageEnvProfile, PortageEapiLevel, PortagePackageEnvEngine, PpaRepository,
    RestrictedPackageSpec, RpmDeltaReconstitutionEngine, SecurityAdvisoryDetail,
    SignifyPqcSignatureHeader, SlackBuildInfo, SlackPackageRecord, SlackwarePkgtoolSlackBuildEngine,
    SlotOperator, SovereignPackageBuildProvenanceEngine, UbuntuPpaAptPinningEngine, XbpsCachedPkg,
    XbpsDowngradeRepoEngine, XbpsRestrictedNonFreeLicenseEngine, XbpsSonameAndOrphanEngine,
    ZypperPackageOffer, ZypperRepository,
};

pub use security::*;
pub use unimplemented_features::*;
pub use unimplemented_tools::{
    ChainedAuditTrailLedger, DiskImageSignatureCarver, DistroWatchTrendAnalyzerTool,
    MetadataExifAntiForensicScrubber, NetworkPcapForensicSniffer, NixGuixStoreGarbageCollectorTool,
    OpenBsdUnveilAuditTool, PhoronixSuiteAutomatedBenchmarkRunnerTool,
    VolatileMemoryDumpForensicEngine,
};
pub mod expanded_wiki_innovations;
pub use expanded_wiki_innovations::{
    GrowthDomainItem, SigmaosGrowthArchitectureEngine, StrategicImportItem,
    StrategicImportPlanEngine,
};
pub mod virtualization;

pub mod interrupt;


pub mod hardware {
    pub mod compatibility;
    pub mod win32;
}
pub mod power {
    pub mod governor;
}
pub mod boot;
pub use boot::*;
pub mod toolchain {
    pub mod adapter;
    pub mod bootstrap;
    pub mod capsule;
    pub mod codex;
}
pub mod scheduler;
pub mod logging;
pub mod system;
pub use system::{AutomationTaskKind, SovereignAutomationEngine, TaskStatus};
pub mod update {
    pub mod distro_update_parity;
}

pub use accessibility::*;
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, ScriptArgumentRouter,
    SystemAction, SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction,
    SystemState,
};
pub use compatibility::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};
pub use container::{
    ContainerError, ContainerID, ContainerInfo, ContainerRuntime as CoreContainerRuntime,
    ContainerState, RuntimeCapability, RuntimeStats, SimpleContainer, SimpleContainerRuntime,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::statutory_compliance::{
    ComplianceRuleStatus, DisputeAuditRollbackEngine, PenaltyBreachNotifier, StatutoryBreachAlert,
    StatutoryFramework, StatutoryGovernanceLayer, StatutoryGovernanceRule,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use driver::pci_bus::{
    PciAddress, PciBarInfo, PciBarType, PciBusManager, PciDeviceNode, PciDriverMatchRule,
    PciHardwareAccess, PciHeaderType, PciInterruptMode, PcieAerLog, PcieAerSeverity, PcieAspmState,
    SimulatedPciHardwareAccess,
};
pub use driver::*;
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
};
pub use governance::{
    FoundationModel, FoundationMember, ReleaseType, RoadmapMilestone, TransparentRoadmap,
    DemocraticProposal, DemocraticVoting,
};
// pub use ipc::{
//     StandardStreamController, StandardStreamHandle, StreamBufferMode, StreamTeeSpliceRouter,
//     STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO,
// };
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, Message, MemoryBlock, PAGE_SIZE,
    Priority, Process, ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler,
    VirtualCpu,
    IoUringOpcode,
    BoundedBufferProducerConsumer, SoftIrqType, BottomHalfKernelThread, BroadcastReceiver,
    AndroidBroadcastReceiverRegistry,
    KernelFastPacketEngine, FastPacketFrame, XdpAction,
    KernelAccessController, LandlockPathRule, LandlockAccessRight,
    InteractiveHybridScheduler, HybridTask,
    CowStorageEngine, CowBlock,
    MemoryCompactionSuperpagesAllocator, PhysicalFrameBlock, SovereignCgroupGovernor, CgroupResourceLimits,
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
    Achievement, AchievementType, GamifiedProductivity, Goal, LayoutPreset as TmuxLayoutPreset,
    PomodoroState, PomodoroTimer, ProductivityScore, SplitDirection as TmuxSplitDirection,
    TmuxPane, TmuxSession, TmuxSessionManager, TmuxWindow,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::hardening;
pub use security::*;
pub use userland::*;
pub use shell::{
    ContextualCompleter, HistoryExpansionEngine, JobControlManager, ParameterExpansionEngine,
    PipelineExecutor, ShellCommand, ShellPledgeUnveilGuard, ShellSyntaxHighlighter,
    SimpleShellSession as ShellRepl, ZshPromptFormatter,
};
pub use sigpkg::{
    AptDebManifest, BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError,
    RecipeManager, SatSolver, Transaction,
};
pub use unimplemented_tools::{
    AdaptiveUxAgent, AiAnomalyFirewall, AiCodeAssistant, AiDependencyResolver,
    AiDifficultyDirector, AiFileOrganizer, AiScheduler, AiSearchAssistant, AiTaskbar,
    AppSandboxing, AudioEditor, CloudBackupUtility, CloudGaming, CodeProfiler, ControllerMapper,
    CrossDeviceSync, CrossLanguageBuildTool, DeclarativeBuildSystem, DocumentScanner,
    EmulatorManager, FlatpakSnapLayer, GameHubLauncher, GameModManager, GamePerformanceBooster,
    GameRecorder, GamifiedTodo, GanttChartPlanner, GestureControl, GuiAppStore, IotDeviceManager,
    MemoryLeakDetector, MeshNetworking, MindMapCreator, MultiMonitorManager, MusicLibraryManager,
    NaturalLanguageShell, OfflinePackageInstaller, PackagePublishingHub, PdfEditor,
    PluginMarketplace, PodcastRecorder, PrivacyDashboard, SecureContainer, SecureFileSharing,
    SmartNotificationManager, StaticAnalyzer, SubtitleEditor, VoiceControl, VrArRuntime,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};

pub mod init {
    pub mod systemd_init;
}
pub use init::systemd_init::{SystemdEngine, SystemdUnit, UnitState, UnitType};

pub use ai::next_gen::{
    AIModel, AdaptiveKernelPersona, AiTask, DeviceTargetType, EnergyGovernorMode, ModelType,
    MultiModelOrchestrator, PredictiveSyscallTranslator, WorkloadType,
};
pub use ai::wandr::{
    ResearchResult, SigmaWandrAgent, WandrDocument, WandrEvaluator, WandrResearchAgent, WandrTask,
};

pub use community::toolkit::{
    ArticleCategory, CommunityHandbookCatalog, HandbookArticle, HybridFirewallTemplateStore,
    PackageRecipe as CommunityPackageRecipe, RecipeSourceFormat, ReproduciblePackageRecipeManager,
    SecurityModelType, SecurityProfileTemplateStore, SecurityTemplate,
    VirtualizationBlueprintStore,
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


pub use unimplemented_features::*;
pub use tools::dependency_reduction::MasterDependencyReductionSuite;
