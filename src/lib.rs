#![allow(clippy::all)]
#![allow(warnings)]
// SigmaOS Library
// Core library for SigmaOS operating system

pub mod audio {
    pub mod driver;
    pub mod editor;
}
pub use audio::driver::{
    AudioDevice, AudioDeviceID, AudioError, AudioManager, AudioMixer, AudioStream, AudioType,
    SimpleAudioDevice, SimpleAudioManager, SimpleAudioMixer, SimpleAudioStream,
};
pub use audio::editor::{
    AmplifyEffect, AudioEditor, AudioEffect, AudioTrack, EchoEffect, LowPassFilter,
    MultiTrackSession, NoiseGateEffect,
};

pub mod accessibility;
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
pub mod network;
pub mod orchestration;
pub mod package;
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
    AIAgent, AIAgentManager, AIError, AIStats, AgentCapability, AgentInfo, ApmDependency,
    ApmLockfile, ApmManifest, ApmPolicy, ApmStatus, DependencySource, Intent, IntentType,
    ManagerCapability as AiManagerCapability, McpServer, Pattern, SimpleAIAgent,
    SimpleAIAgentManager, SovereignApmEngine, SovereignWikiEngine, WikiArticle,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use compatibility::{
    ApplicationBinary, BIOSGatewayMesh, BinaryFormat, BodhiProfileSelector, BudgieAppletManager,
    BudgieLayoutSwitcher, BudgieShuffler, BuildCodexGrid, CoasAdminSuite, CompatibilityError,
    CompatibilityManager, CompatibilityMode, ConstellationNode, ContainerRuntime,
    CorebootGatewayMesh, CosmicDesktopEngine, DACConstellation, DotMatrixMesh, DrakxtoolsSuite,
    DriverArchiveGridV2, ElementaryAppCenter, EosLogTool, EosMirrorReflector, EosUpdateNotifier,
    EosWelcomeEngine, FhsConventionStatus, FileAlmanacHub, FirmwareGatewayMesh, FloppyMesh,
    GraniteHigLibrary, GraphicsArchiveGridV2, HarddrakeDetector, JujuOrchestrator,
    KernelConstellationGrid, LegacyAsmCodexGrid, LegacyCCodexGrid, LegacyCppCodexGrid,
    LegacyDriverAdapter, LegacyFSAdapter, LegacyKernelAdapter, LegacyPackageAdapter,
    LegacyProtocolAdapter, LegacySecurityAdapter, LegacyUIAdapter, LizardInstaller, LsbProfile,
    MaasProvisioner, Mirror as EosMirror, MokshaDesktopEngine, MokshaGadgetManager,
    MultipassVmlight, NetworkAlmanacHub, NetworkArchiveGridV2, PacstallAur,
    PantheonGalaWindowManager, PeripheralArchiveMesh, PopShellTiling, PosixComplianceLevel,
    ProcessAlmanacHub, RhinoPkgUnified, SELinuxConstellation, SecurityConstellation,
    SnapcraftRuntime, StandardsComplianceManager, StarlingCompositor, StarlingTilingEngine,
    StarlingWidgetTree, StarlingX11Server, StorageArchiveGridV2, SyscallAlmanacHub,
    System76PowerSwitcher, System76Scheduler, TapeMesh, TargetPlatform, TranslationLayer,
    UEFIGatewayMesh, UbuntuDockManager, UbuntuProEsm, UnicornDesktopShell, UrpmiPackageResolver,
    WelcomeTab as EosWelcomeTab, YayAurHelper, ZeroTrustConstellation, ZorinConnectBridge,
    ZorinLookChanger, ZorinWinePreflight,
};
pub use container::{
    ContainerCapability, ContainerError, ContainerID, ContainerInfo,
    ContainerRuntime as CoreContainerRuntime, ContainerState, RuntimeCapability, RuntimeStats,
    SimpleContainer, SimpleContainerRuntime,
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
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
};
pub use network::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DebPackageDriverTranslator, DependencyResolver, GenericLinuxTranslationUdf,
    LinuxDriverPackageTranslator, LinuxTranslationService, PackageAdapter, PackageError,
    PackageFormat, PackageSource, PackageTranslationUdf, PacmanPackageDriverTranslator,
    RpmPackageDriverTranslator, UnifiedPackage, UniversalPackageManager,
    GLOBAL_TRANSLATION_SERVICE, GLOBAL_TRANSLATION_UDF,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::hardening;
pub use security::{
    secure_zeroize, AuditLogEntry, CapabilityGate, CapabilityToken, ExploitPayload,
    HardenedAuditTrail, IntrusionMonitor, IntrusionSeverity, PenetrationAssistant, Permission,
    PledgeManager, PledgePromise, SecurityScanner, VulnerabilityClass, VulnerabilityReport,
};
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    AptDebManifest, BuildSystem, ContentAddressedStore, CryptoVerifier, FlatpakManifest,
    PackageRecipe, PacmanPkgbuild, RecipeError, RecipeManager, SatSolver, SnapcraftManifest,
    Transaction, UniversalPackageAdapter,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};

pub mod init {
    pub mod systemd_init;
}
pub use init::systemd_init::{SystemdEngine, SystemdUnit, UnitState, UnitType};

pub mod ai {
    pub mod next_gen;
    pub mod wandr;
}
pub use ai::next_gen::{
    AIModel, AdaptiveKernelPersona, AiScheduler, AiTask, DeviceTargetType, EnergyAwareScheduler,
    EnergyGovernorMode, ModelType, MultiModelOrchestrator, PredictiveSyscallTranslator,
    WorkloadType,
};
pub use ai::wandr::{
    ResearchResult, SigmaWandrAgent, WandrDocument, WandrEvaluator, WandrResearchAgent, WandrTask,
};

pub mod virt;
pub use virt::hypervisor::{
    Guest, GuestID, GuestState, Hypervisor, HypervisorError, SimpleGuest, SimpleHypervisor,
    VirtualizationGeneration,
};
pub use virt::microvm::{
    MicroVM, MicroVMState, SandboxManager, SandboxPolicy, SimpleMicroVM, SimpleSandboxManager,
};
