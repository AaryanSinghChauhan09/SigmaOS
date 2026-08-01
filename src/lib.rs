// SigmaOS Library
// Core library for SigmaOS operating system

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
pub mod gpu;
pub mod graphics;
pub mod kernel;
pub mod klib;
pub mod media;
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
pub mod virtualization;

pub use media::{
    CaptureSource, GpuEncoderType, RecorderState, SovereignScreenRecorder, RecordingStats,
};

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
    PAGE_SIZE, PolicyError, PolicyManager, PrivacyFirstSandbox, Priority, Process, ProcessState,
    ProtectionDomain, PrivilegeLevel, ResourceBroker, RoundRobinConfig, RoundRobinScheduler,
    Scheduler, SchedulerError, SelfHealingKernel, SigmaFsPlusPlus, UniversalAbiTranslator,
    UserDefinedKernelFunctions,
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
pub use remote::{
    FileTransfer, InputAuthGate, PqcVideoCipher, RemoteDesktop, RemoteError, RemoteSession,
    RemoteShell, SessionID, SessionState, ShellError, ShellID, ShellManager, SigmaRendezvous,
    SimpleFileTransfer, SimpleRemoteDesktop, SimpleRemoteSession, SimpleScreenSharing,
    SimpleShellManager,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
    SubtitleFormat, SubtitleEntry, AegisubEngine, SubtitleEditEngine,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{
    CapabilityGate, CapabilityToken, DomainID, DomainOrchestrator, DomainType, IsolatedDomain,
    IsolationError, Permission, PledgeManager, PledgePromise,
    SecurityPolicy, SecurityRule, SecurityLabel, SelinuxPermission, ObjectType, SecurityContext,
    AppArmorProfile, AppArmorManager,
    DefensiveAuditSystem, ForensicBlock, MaliciousSignature,
    IntrusionMonitor, IntrusionSeverity, AuditLogEntry, HardenedAuditTrail, secure_zeroize,
};
pub use shell::{
    CommandError as ShellCommandError, ShellCommand, ShellRepl, ShellSession, SimpleShellSession,
};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction, DebPackageImporter, RpmPackageImporter, PacmanPackageImporter, PackageImporter,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
