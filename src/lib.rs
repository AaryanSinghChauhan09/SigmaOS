#![allow(warnings)]
#![allow(clippy::all)]

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
pub use driver::{
    Apc, DeviceObject, Dpc, DriverObject, IoStatus, IoStatusBlock, Irp, IrpManager, Minifilter,
    IRP_MJ_CLOSE, IRP_MJ_CREATE, IRP_MJ_DEVICE_CONTROL, IRP_MJ_READ, IRP_MJ_WRITE, METHOD_BUFFERED,
    METHOD_IN_DIRECT, METHOD_NEITHER, METHOD_OUT_DIRECT,
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
pub use klib::{
    buddy_allocator::{Block, BlockID, SimpleBuddyAllocator},
    paging::{
        PageTable, PageTableEntry, ProcessMemory, SimplePageTable, SimplePageTableEntry,
        SimpleProcessMemory, SimpleVMM, VirtualMemoryManager,
    },
    uvm::{UvmAmap, UvmError, UvmPageLoan, UvmPmap},
};
pub use network::{
    AlertSeverity, AlertType, AlpineZeroAllocCaptureBuffer, AnalysisStrategy, BandwidthAnalysis,
    ClearLinuxFlowLoadBalancer, ConnectionInfo, ConnectionState, EnterpriseNetworkError,
    GentooUseFlagsDissector, IPv6Address, KaliPacketFingerprinter, KaliSnoopAnalysis,
    NetworkTrafficAnalyzer, NixDeclarativeFilter, Protocol, SecureVpnTunnel, SecurityAnalysis,
    TcpConnection, TcpError, TcpSegment, TcpStack, TcpState, TrafficAlert, TrafficPacket,
    TrafficStatistics,
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
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{
    secure_zeroize, AppArmorManager, AppArmorProfile, AuditLogEntry, BiometricAuth,
    BiometricResult, BiometricType, CapabilityGate, CapabilityToken, DefensiveAuditSystem,
    DomainID, DomainOrchestrator, DomainType, FaceIdAuth, FingerprintAuth, ForensicBlock,
    HardenedAuditTrail, IntrusionMonitor, IntrusionSeverity, IsolatedDomain, IsolationError,
    MaliciousSignature, ObjectType, PasswordCategory, PasswordEntry, PasswordError,
    PasswordManager, PasswordManagerResult, Permission, PledgeManager, PledgePromise,
    SecurityContext, SecurityLabel, SecurityPolicy, SecurityRule, SelinuxPermission,
};
pub use shell::{
    CommandError as ShellCommandError, ShellCommand, ShellRepl, ShellSession, SimpleShellSession,
};
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
};
pub use distro::{
    AppManifest as DistroAppManifest, BuildError as DistroBuildError, BuildSpec as DistroBuildSpec, CertificationStatus,
    CpuOptimizationDetector, FeatureSet as DistroFeatureSet, SigmaBuildGraph, UseFlag,
};
