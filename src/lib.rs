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
pub mod kernel;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod power;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod virtualization;
pub mod unimplemented_features;
pub mod unimplemented_tools;

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
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ComputeNode, ContainerRuntime, DistributedComputeHandoff, GstCalculator, IndiaStackError,
    JehanneError, JehanneNamespace, MintBackupTool, MintSoftwareManager, MintUpdateItem,
    MintUpdateLevel, MintUpdateManager, MockUPIService, MultilingualSupport, NamespaceBindEntry,
    NtHandle, NtObjectManager, NtObjectType, NtStatus, Plan9pMessage, Plan9pMsgType,
    PortableExecutableLoader, RegistryHive, SoftwareMeta, TargetPlatform, TranslationLayer,
    WindowCoordinates, ZenithDisplayCompositor,
    InterimLispVM, LispVal, MntReformLpcDriver, ReformPowerStats,
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
pub use driver::{
    Irp, DriverObject, DeviceObject, IoStatus, IoStatusBlock, Apc, Dpc, Minifilter,
    IRP_MJ_CREATE, IRP_MJ_CLOSE, IRP_MJ_READ, IRP_MJ_WRITE, IRP_MJ_DEVICE_CONTROL,
    METHOD_BUFFERED, METHOD_IN_DIRECT, METHOD_OUT_DIRECT, METHOD_NEITHER,
    IoManager, DriverEntry, OpaqueDriverExtension,
    ObjectManager, ObjectType, NonPagedPool, RootkitDetector, IrpParameters,
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
    KernelSymbol, KernelSymbolType, KldModule, SymbolRegistry, SysInitItem, SysInitOrchestrator,
    SysInitPriority,
};
pub use network::{
    compute_checksum as compute_net_checksum, IPv4Address, NetworkPacket, PacketRingBuffer,
    RingTcpState, TcpConnection, TcpError, TcpSegment, TcpSocket, TcpStack, TcpState,
    ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
    Protocol, UdpTcpState, UdpNetworkError, Socket as UdpSocket, SimpleSocket as UdpSimpleSocket,
    UdpTCPConnection, UDPSocket as CoreUdpSocket, RenoCongestionControl, BBRCongestionControl,
    FirewallTarget, FirewallChain, ConntrackState, FirewallRule, IptablesFirewall,
    SimpleFirewall, ZeroCopy, ZeroCopyNetwork, CoreNetworkStack, SimpleNetworkStack,
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
pub use power::{
    AspmLevel, BatteryDevice, BatteryError, BatteryInfo, BatteryStatus, CpuFreqCore, CpuGovernor,
    EnergyAwareThreadBalancer, PowerCapability, PowerError, PowerEvent, PowerManagement,
    PowerStack, PowerState, SimpleBatteryDevice, SimplePowerManager, SimplePowerStack,
    TlpPowerManager,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
    TopCommand, ProcessTaskInfo, IfconfigCommand, NetworkInterface, PingCommand, PingResult,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{
    CapabilityGate, CapabilityToken, CronDaemon, CronJob, DefaultDenyNetworkPolicy, DmesgLog,
    FirewallRule, IptablesFirewall, KaliError, NemoClawError, OpenShellAgentSandbox, Permission,
    PledgeManager, PledgePromise, PluggableAuthenticationModule, PrivacyRouter,
    SudoPrivilegeEscalation, SwapSpaceManager, TmuxMultiplexer, TmuxPane,
    PenetrationAssistant, ExploitPayload, Vulnerability, VulnerabilityScanner, SimpleVulnerability,
    SimpleVulnerabilityScanner, Severity, ScanSummary, ScanReport,
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
