#![allow(warnings)]
#![allow(clippy::all)]

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
<<<<<<< HEAD
pub mod kernel;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod productivity;
pub mod remote;
=======
pub mod graphics;
pub mod init;
pub mod kernel;
pub mod klib;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod plugin;
pub mod productivity;
>>>>>>> origin/jules-15532892492441614180-73ce6847
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod virtualization;

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
<<<<<<< HEAD
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
};
pub use container::{
    ContainerCapability, ContainerError, ContainerID, ContainerInfo,
    ContainerRuntime as CoreContainerRuntime, ContainerState, RuntimeCapability, RuntimeStats,
    SimpleContainer, SimpleContainerRuntime,
=======
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime as CompatibilityContainerRuntime, TargetPlatform, TranslationLayer,
};
pub use container::{
    Container, ContainerCapability, ContainerError, ContainerID, ContainerInfo, ContainerRuntime,
    ContainerState, Namespace, OciContainer, OciContainerError, OciContainerID,
    OciContainerRuntime, OciContainerState, RuntimeCapability, RuntimeStats, Sandbox,
    SimpleContainer, SimpleContainerRuntime, SimpleOciContainer, SimpleOciContainerRuntime,
    SimpleSandbox,
>>>>>>> origin/jules-15532892492441614180-73ce6847
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
<<<<<<< HEAD
    ABIManager, AiNativeRuntime, BuddyAllocator, Channel, EnergyAwareScheduler, FastPathIpc,
    Generation, GenerationManager, InterruptMechanism, IpcError, IpcManager, KernelGraph,
    KernelPersona, KernelPlugin, KernelPluginManager, LegacyScheduler, MemoryBlock, Message,
    MetaKernel, MicroDriver, NetPod, PolicyError, PolicyManager, Priority, PrivacyFirstSandbox,
    PrivilegeLevel, Process, ProcessState, ProtectionDomain, ResourceBroker, RoundRobinConfig,
    RoundRobinScheduler, Scheduler, SchedulerError, SelfHealingKernel, SigmaFsPlusPlus,
    UniversalAbiTranslator, UserDefinedKernelFunctions, PAGE_SIZE,
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
=======
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
};
pub use network::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
>>>>>>> origin/jules-15532892492441614180-73ce6847
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
<<<<<<< HEAD
=======
pub use plugin::{
    ManagerCapability, Plugin, PluginCapability, PluginError, PluginID, PluginInfo, PluginManager,
    PluginState, PluginStats, SimplePlugin, SimplePluginManager,
};
>>>>>>> origin/jules-15532892492441614180-73ce6847
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
<<<<<<< HEAD
pub use remote::{
    FileTransfer, InputAuthGate, PqcVideoCipher, RemoteDesktop, RemoteError, RemoteSession,
    RemoteShell, SessionID, SessionState, ShellError, ShellID, ShellManager, SigmaRendezvous,
    SimpleFileTransfer, SimpleRemoteDesktop, SimpleRemoteSession, SimpleScreenSharing,
    SimpleShellManager,
};
=======
>>>>>>> origin/jules-15532892492441614180-73ce6847
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{
<<<<<<< HEAD
    CapabilityGate, CapabilityToken, DomainID, DomainOrchestrator, DomainType, IsolatedDomain,
    IsolationError, Permission, PledgeManager, PledgePromise,
    SecurityEnforcer as AndroidStyleSecurityEnforcer, PORT_ALLOW_SSL, PORT_ALLOW_TCP,
};
pub use shell::{
    CommandError as ShellCommandError, ShellCommand, ShellRepl, ShellSession, SimpleShellSession,
};
pub use sigpkg::{
    AdapterError, BuildSystem, ContentAddressedStore, CryptoVerifier, DebAdapter,
    PackageDependencyResolver, PackageFormatAdapter, PackageRecipe, PacmanAdapter, RecipeError,
    RecipeManager, RpmAdapter, SatSolver, Transaction, UniversalPackageManager, Version,
    MAX_RECIPE_DEPENDENCIES,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
=======
    CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise, UnveilManager,
    UnveilPermission, UnveilRestriction,
};
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction,
};
pub use virtualization::{
    Container as VirtualContainer, KubernetesPod, ResourcePool, VirtualMachine,
    VirtualizationError, VirtualizationOrchestrator, VirtualizationTech, VmState,
>>>>>>> origin/jules-15532892492441614180-73ce6847
};
