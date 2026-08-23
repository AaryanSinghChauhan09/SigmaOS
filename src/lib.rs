// SigmaOS Library
// Core library for SigmaOS operating system

pub mod ai;
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
pub mod ai {
    pub mod agent;
    pub mod next_gen;
    pub mod orchestrator;
    pub mod wandr;
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
    ApplicationBinary, BinaryFormat, BodhiUpdateTriage, CompatibilityError,
    CompatibilityManager, CompatibilityMode, ContainerRuntime, DnfPackageResolver,
    FedoraAlu, FedoraAluFlags, KojiBuildServer, MockChrootBuilder, SigmaChangeProposal,
    SigmaChangeProcessEngine, SigmaNextChannel, TargetPlatform, TranslationLayer,
    ZorinAppearanceSwitcher, ZorinLayoutPreset, ZorinConnectHub, ZorinWineLayer,
    ZorinLiteOptimizer, SigmaEcosystemInit, FhsRunlevel, SigmaEcosystemProfiler,
    GraphicPresetMode, SigmaOnboardingWelcome, SigmaOnboardingLog,
    SigmaSupportSubtitleSync, SigmaSupportSubtitleEdit, SubtitleFormat,
    SigmaSupportResourceOptimizer, SigmaSupportPriorityOptimizer,
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
    AdvancedAlgorithmsManager, Apc, ApcMode, ApcQueue, ArchitectureEngine,
    AuditBlock, BuddyAllocator, Channel, CircularDoublyLinkedList,
    CpuRegisters, EdfTask, HardwareException,
    IpcError, IpcManager, Irql,
    LcgRandom, LookasideList, LotteryTask, MemoryBlock,
    MemoryDescriptorList, Message, Pcb, PoolType, Priority, Process,
    ProcessState, ProcessorInitState, RoundRobinConfig, RoundRobinScheduler,
    SchedulerError, SequencedSinglyLinkedList, SinglyLinkedList, SystemThread,
    Tcb, ThreadState, WorkItem, PAGE_SIZE,
};
pub use logging::*;
pub use network::{
    TcpConnection, TcpError, TcpSegment, TcpStack, TcpState,
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
    FileTransfer, InputAuthGate, PqcVideoCipher, RemoteDesktop, RemoteError, RemoteSession,
    RemoteShell, SessionID, SessionState, ShellError, ShellID, ShellManager, SigmaRendezvous,
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
    CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise,
    PrivacyFirstSandbox, SandboxRule, secure_zeroize, AuditLogEntry, HardenedAuditTrail,
    IntrusionMonitor, IntrusionSeverity,
};
pub use plugin::{
    ExtensionType, ManagerCapability, MarketplaceItem, Plugin, PluginCapability, PluginError,
    PluginID, PluginInfo, PluginManager, PluginMarketplace, PluginState, PluginStats, SimplePlugin,
    SimplePluginManager,
};
pub use shell::{ShellCommand, SimpleShellSession as ShellRepl};
pub use sigpkg::{
    AptDebManifest, BuildSystem, ContentAddressedStore, CryptoVerifier, FlatpakManifest,
    PackageRecipe, PacmanPkgbuild, RecipeError, RecipeManager, SatSolver, SnapcraftManifest,
    Transaction, UniversalPackageAdapter,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
pub use governance::strategic_vision::{
    MilestoneCategory, OkrError, OkrTracker, StrategicMilestone, StrategicOkrEvaluator,
};
pub use unimplemented_tools::{
    AudioEditor, PodcastRecorder, SubtitleEditor, MemoryLeakDetector, GamifiedTodo, MindMapCreator,
    GameHubLauncher, EmulatorManager, GameRecorder, GamePerformanceBooster, CloudGaming, VrArRuntime,
    ControllerMapper, GameModManager, AiDifficultyDirector, GanttChartPlanner, PdfEditor,
    DocumentScanner, CodeProfiler, StaticAnalyzer, PackagePublishingHub, AdaptiveUxAgent,
    AiSearchAssistant, NaturalLanguageShell, AiCodeAssistant, AiFileOrganizer, SmartNotificationManager,
    RemoteDesktop, MeshNetworking, IotDeviceManager, CloudBackupUtility, SecureFileSharing,
    AiScheduler, GuiAppStore, MultiMonitorManager, GestureControl, VoiceControl, AiTaskbar,
    CrossDeviceSync, FlatpakSnapLayer, DeclarativeBuildSystem, AiDependencyResolver, AiAnomalyFirewall,
    SecureContainer, PrivacyDashboard, OfflinePackageInstaller, AppSandboxing, CrossLanguageBuildTool,
    PluginMarketplace, MusicLibraryManager, TimeMachineBackup, SysinternalsProcMon, SystemdCgTop,
    TrussSyscallTracer, NetworkQualityProbe, WindowsPowercfg,
};

pub mod init {
    pub mod systemd_init;
}
pub use init::systemd_init::{SystemdEngine, SystemdUnit, UnitState, UnitType};

pub use ai::next_gen::{
    AIModel, AdaptiveKernelPersona, AiScheduler, AiTask, DeviceTargetType,
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
