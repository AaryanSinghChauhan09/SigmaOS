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
pub mod observability {
    pub mod profiler;
}
pub mod ai;
pub mod arch;
pub mod boot;
pub mod toolchain;
pub mod scheduler {
    pub mod numa_scheduler;
}
pub mod crypto {
    pub mod vectorized_pqc;
}

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
    CapabilityGate, CapabilityToken, DomainID, DomainOrchestrator, DomainType, IsolatedDomain,
    IsolationError, Permission, PledgeManager, PledgePromise,
};
pub use shell::{
    CommandError as ShellCommandError, CronJob, LogEntry, LogLevel, Privilege, Service,
    ShellCommand, ShellRepl, ShellSession, SigmaCoreUtils, SigmaCron, SigmaDoc, SigmaInit,
    SigmaLog, SigmaPriv, SimpleShellSession,
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
