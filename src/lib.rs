#![allow(clippy::all)]
#![allow(warnings)]
// SigmaOS Library
// Core library for SigmaOS operating system

pub mod klib;
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
    AIAgent, AIAgentManager, AIError, AIStats, AgentCapability, AgentInfo, Intent, IntentType,
    ManagerCapability as AiManagerCapability, Pattern, SimpleAIAgent, SimpleAIAgentManager,
    SovereignWikiEngine, WikiArticle, ApmDependency, ApmLockfile, ApmManifest, ApmPolicy,
    ApmStatus, DependencySource, McpServer, SovereignApmEngine,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use compatibility::{
    ApplicationBinary, BIOSGatewayMesh, BinaryFormat, BuildCodexGrid, CompatibilityError,
    CompatibilityManager, CompatibilityMode, ConstellationNode, ContainerRuntime,
    CorebootGatewayMesh, DACConstellation, DotMatrixMesh, DriverArchiveGridV2, EosLogTool,
    EosMirrorReflector, EosUpdateNotifier, EosWelcomeEngine, FhsConventionStatus, FileAlmanacHub,
    FirmwareGatewayMesh, FloppyMesh, GraphicsArchiveGridV2, KernelConstellationGrid,
    LegacyAsmCodexGrid, LegacyCCodexGrid, LegacyCppCodexGrid, LegacyDriverAdapter, LegacyFSAdapter,
    LegacyKernelAdapter, LegacyPackageAdapter, LegacyProtocolAdapter, LegacySecurityAdapter,
    LegacyUIAdapter, LsbProfile, Mirror as EosMirror, NetworkAlmanacHub, NetworkArchiveGridV2,
    PeripheralArchiveMesh, PosixComplianceLevel, ProcessAlmanacHub, SELinuxConstellation,
    SecurityConstellation, StandardsComplianceManager, StorageArchiveGridV2, SyscallAlmanacHub,
    TapeMesh, TargetPlatform, TranslationLayer, UEFIGatewayMesh, WelcomeTab as EosWelcomeTab,
    YayAurHelper, ZeroTrustConstellation,
    StarlingCompositor, StarlingWidgetTree, StarlingX11Server, StarlingTilingEngine,
    CosmicDesktopEngine, PopShellTiling, System76Scheduler, System76PowerSwitcher,
    BudgieAppletManager, BudgieShuffler, BudgieLayoutSwitcher,
    RhinoPkgUnified, PacstallAur, UnicornDesktopShell,
    MokshaDesktopEngine, BodhiProfileSelector, MokshaGadgetManager,
    PantheonGalaWindowManager, GraniteHigLibrary, ElementaryAppCenter,
    UbuntuDockManager, SnapcraftRuntime, UbuntuProEsm,
    MaasProvisioner, JujuOrchestrator, MultipassVmlight,
    ZorinLookChanger, ZorinConnectBridge, ZorinWinePreflight,
    DrakxtoolsSuite, HarddrakeDetector, UrpmiPackageResolver,
    LizardInstaller, CoasAdminSuite,
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
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
    DebPackageDriverTranslator, GenericLinuxTranslationUdf, LinuxDriverPackageTranslator,
    LinuxTranslationService, PackageTranslationUdf, PacmanPackageDriverTranslator,
    RpmPackageDriverTranslator, GLOBAL_TRANSLATION_SERVICE, GLOBAL_TRANSLATION_UDF,
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
