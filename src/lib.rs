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
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod gpu;
pub mod graphics;
pub mod kernel;
pub mod klib;
pub mod media;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod performance;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod virtualization;

pub use media::{
    CaptureSource, GpuEncoderType, RecorderState, RecordingStats, SovereignScreenRecorder,
};

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
    ABIManager, AiNativeRuntime, BuddyAllocator, Channel, EnergyAwareScheduler, FastPathIpc,
    InterruptMechanism, IpcError, IpcManager, KernelGraph, KernelPersona, KernelPlugin,
    KernelPluginManager, LegacyScheduler, MemoryBlock, Message, MetaKernel, MicroDriver, NetPod,
    PolicyError, PolicyManager, Priority, PrivacyFirstSandbox, PrivilegeLevel, Process,
    ProcessState, ProtectionDomain, ResourceBroker, RoundRobinConfig, RoundRobinScheduler,
    Scheduler, SchedulerError, SelfHealingKernel, SigmaFsPlusPlus, UniversalAbiTranslator,
    UserDefinedKernelFunctions, PAGE_SIZE,
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
    ConflictResolution, DebPackageDriverTranslator, DependencyResolver, GenericLinuxTranslationUdf,
    LinuxDriverPackageTranslator, LinuxTranslationService, PackageAdapter, PackageError,
    PackageFormat, PackageSource, PackageTranslationUdf, PacmanPackageDriverTranslator,
    RpmPackageDriverTranslator, UnifiedPackage, UniversalPackageManager,
    GLOBAL_TRANSLATION_SERVICE, GLOBAL_TRANSLATION_UDF,
};
pub use productivity::{
    Achievement, AchievementType, AegisubEngine, GamifiedProductivity, Goal, PomodoroState,
    PomodoroTimer, ProductivityScore, SubtitleEditEngine, SubtitleEntry, SubtitleFormat,
};
pub use remote::{
    FileTransfer, InputAuthGate, PqcVideoCipher, RemoteDesktop, RemoteError, RemoteSession,
    RemoteShell, SessionID, SessionState, ShellError, ShellID, ShellManager, SigmaRendezvous,
    SimpleFileTransfer, SimpleRemoteDesktop, SimpleRemoteSession, SimpleScreenSharing,
    SimpleShellManager,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{
    secure_zeroize, AppArmorManager, AppArmorProfile, AuditLogEntry, CapabilityGate,
    CapabilityToken, DefensiveAuditSystem, DomainID, DomainOrchestrator, DomainType, ForensicBlock,
    HardenedAuditTrail, IntrusionMonitor, IntrusionSeverity, IsolatedDomain, IsolationError,
    MaliciousSignature, ObjectType, Permission, PledgeManager, PledgePromise, SecurityContext,
    SecurityLabel, SecurityPolicy, SecurityRule, SelinuxPermission,
};
pub use shell::{
    CommandError as ShellCommandError, ShellCommand, ShellRepl, ShellSession, SimpleShellSession,
};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, DebPackageImporter, PackageImporter,
    PackageRecipe, PacmanPackageImporter, RecipeError, RecipeManager, RpmPackageImporter,
    SatSolver, Transaction,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
