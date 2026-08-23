// SigmaOS Library
// Core library for SigmaOS operating system

pub mod access;
pub mod accessibility;
pub mod ai;
pub mod automation;
pub mod cluster;
pub mod community;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod governance;
pub mod kernel;
pub mod klib;
pub mod memory;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod process;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod storage;
pub mod thread;
pub mod tools;
pub mod unimplemented_features;
pub mod unimplemented_tools;
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
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, ScriptArgumentRouter,
    SystemAction, SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction,
    SystemState,
};
pub use compatibility::{
    AntiXControlCentre, AntiXInitSwitcher, AntiXInitSystem, AntiXPersistenceManager,
    AntiXPersistenceMode, AntiXService, AntiXServiceState, AntiXSystemRemasterEngine,
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, LegacyKernelAdapter, LegacyPackageAdapter, LegacySecurityAdapter,
    LegacyUIAdapter, TargetPlatform, TranslationLayer, ZorinChameleonColor, ZorinChameleonEngine,
    ZorinConnectManager, ZorinConnectState, ZorinLayout, ZorinLayoutMetrics, ZorinLayoutSwitcher,
    ZorinWindowsAppSupport,
};
pub use container::{
    ContainerCapability, ContainerError, ContainerID, ContainerInfo,
    ContainerRuntime as CoreContainerRuntime, ContainerState, RuntimeCapability, RuntimeStats,
    SimpleContainer, SimpleContainerRuntime,
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
pub use distro::{
    AdminAction, AiSysAdmin, AppManifest, AuditResult, AuditRule, BackupSnapshot, BackupSystem,
    BountyStatus, BsdSecureNtpConstraintSync, BsdStatefulPacketFilter, BugBountyProgram,
    BugBountyReport, BuildJob, BuildStatus, CanFrame, CertificationStatus, CommunityConference,
    ComplianceAuditor, ComponentType, ConferenceTalk, ConfigHook, CrossBuildPipeline,
    DaxMemoryRegion, DevTool, DeveloperToolkit, DirectoryService, DirectoryUser, DllLoader,
    DllModule, DragonFlyHammerFs, EcuController, EduChallenge, EduPlayground, ForumChannel,
    ForumPost, GdiObjectType, Hammer2MultiMasterPfsReplication, Hammer2Snapshot, Hammer2TxgRecord,
    HardwareCertificate, HardwareCertificationProgram, HardwareProfile, HardwareRegressionSuite,
    HelpSystem, HowToGuide, HpcClusterJob, HpcJobState, ImeCandidate, InputMethodEngine,
    IntegrityState, KernelTrace, LanguagePack, LinuxSyscall, LiveDebugger, LivepatchManager,
    LivepatchPatch, LocaleManager, ManPage, MpiCommunicator, NetplanConfig, NetplanManager,
    P2pNode, PackageBuildService, PfRuleAction, PfStateEntry, PfStateSynchronizationEngine,
    PfSyncMessage, PfSyncMsgType, PfsClusterNode, PosixTranslation, PqcSelfHealing,
    QAStagedRelease, RegionalSettings, RegistryType, RegistryValue, ReleaseStage, RescueISO,
    RescueISOManager, RunitService, RunitServiceState, SoftwareCertificationProgram,
    SovereignAnonScrubber, SovereignDeltaPackageSigner, SovereignDeltaPatch, SovereignP2PSync,
    TargetArch, TimeTravelCheckpoint, TimeTravelEngine, TlsConstraint, VirtioFsZeroCopyBridge,
    VoidRunitManager, WikiPage, Win32Gdi, WindowsRegistry,
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
    MilestoneCategory, OkrError, OkrTracker, StrategicMilestone, StrategicOkrEvaluator,
};
pub use kernel::io_uring::{
    CompletionQueueEntry, IoUringEngine, IoUringOpcode, SubmissionQueueEntry,
};
pub use kernel::{
    AdaptivePolicy, AdvancedAlgorithmsManager, AndroidBroadcastReceiverRegistry, Apc, ApcMode,
    ApcQueue, ApsrFlags, ArchitectureEngine, ArmExecutionState, AuditBlock, BottomHalfKernelThread,
    BoundedBufferProducerConsumer, BroadcastReceiver, BuddyAllocator, Channel,
    CircularDoublyLinkedList, CpuArchitectureClass, CpuRegisters, EdfTask, HardwareException,
    InstructionCyclePhase as ArchInstructionCyclePhase, InstructionCyclePhase, InterruptClass,
    IoModuleController, IoWaitProfile, IpcError, IpcManager, Irql, KernelMechanism, KernelPolicy,
    LcgRandom, LookasideList, LotteryTask, MemoryBlock, MemoryDescriptorList, Message, Pcb,
    PolicyMechanismCoordinator, PoolType, Priority, Process, ProcessState, ProcessorInitState,
    RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, SequencedSinglyLinkedList,
    SinglyLinkedList, SoftIrqType, SovereignMechanism, SovereignSystemBus, SystemThread, Tcb,
    ThreadState, WorkItem, PAGE_SIZE,
};
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
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, LayoutPreset as TmuxLayoutPreset,
    PomodoroState, PomodoroTimer, ProductivityScore, SplitDirection as TmuxSplitDirection,
    TmuxPane, TmuxSession, TmuxSessionManager, TmuxWindow,
};
pub use remote::{
    FileTransfer, RemoteDesktop, RemoteError, RemoteSession, RemoteShell, SessionID, SessionState,
    ShellError, ShellID, ShellManager, SimpleFileTransfer, SimpleRemoteDesktop,
    SimpleRemoteSession, SimpleScreenSharing, SimpleShellManager,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::hardening;
pub use security::{
    AnonSurfShunt, AppSandboxEngine, ArithmeticSubstitutionDeobfuscator, CapabilityGate,
    CapabilityToken, ForensicStorageFilter, Permission, PledgeManager, PledgePromise, RoutingMode,
    SandboxPolicy,
};
pub use shell::{ShellCommand, SimpleShellSession as ShellRepl};
pub use sigpkg::{
    AptDebManifest, BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError,
    RecipeManager, SatSolver, Transaction, UniversalPackageAdapterManager,
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
    AIModel, AdaptiveKernelPersona, AiScheduler, AiTask, DeviceTargetType, EnergyGovernorMode,
    ModelType, MultiModelOrchestrator, PredictiveSyscallTranslator, WorkloadType,
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

pub mod open_source_obsoletion;
pub use open_source_obsoletion::*;
