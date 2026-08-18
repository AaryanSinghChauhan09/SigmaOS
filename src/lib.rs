// SigmaOS Library
// Core library for SigmaOS operating system

pub mod accessibility;
pub mod automation;
pub mod boot;
pub mod container;
pub mod compatibility;
pub mod container;
pub mod unimplemented_features;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod kernel;
pub mod klib;
pub mod ml;
pub mod network;
pub mod observability {
    pub mod profiler;
}
pub mod orchestration;
pub mod distro;
pub mod package;
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
pub mod ai {
    pub mod agent;
    pub mod orchestrator;
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
    AccessibilityProfile, AccessibilitySetting,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use compatibility::{
    APITimelineManager, BinaryAbiFormat, LinuxBsdAbiBridge, ServiceInitType, ServiceUnitTranslator, TranslatedService, AiResourceScheduler,
    AntixDesktopProfiler, AntixInitManager, AppSuiteBundle, AppSuiteType, ApplicationBinary,
    BinaryCompatMatrix, BinaryFormat, BrailleMatrix, BsdJailSandbox, BundleType, CloudOrchestrator,
    CloudProvider, CompatBinary, CompatBinaryFormat, CompatibilityError, CompatibilityLayer,
    CompatibilityManager, CompatibilityMode, ContainerRuntime, ContinuityCoordinator, DesktopMode,
    DesktopProfile, DesktopTheme, DiscontinuedFS, DistroReleaseChannel, DriverBridge,
    EcosystemSnapshot, FSRevival, FlatpakApp, GraphicsBridge, HandoffTask, InstallerStep,
    KapudanAssistant, KernelPersona, KernelPersonaVM, LanguageTranslationCatalog, LegacyBus,
    LegacyDriver, LegacyMemoryTrimmer, LegacyPluginManager, LibcVersion, LocaleManager,
    MicroService, MicroServiceState, NetworkBridge, ReleaseGovernanceCouncil,
    ReproducibleBuildVerifier, SigmaContainer, SnapshotManager, StorageBridge, SuiteRegistry,
    SyscallAbi, TargetPlatform, TranslationLayer, TribeInstaller, TtsSynthesizer, UnifiedAppStore,
    WorkloadOptimizer, WorkloadProfile, ZorinAppearanceSwitcher, GLOBAL_AKABEI,
    GLOBAL_ANTIX_CONTROL, GLOBAL_ANTIX_DESKTOP, GLOBAL_ANTIX_INIT, GLOBAL_KAPUDAN,
    GLOBAL_MEMORY_TRIMMER, GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER, GLOBAL_TRIBE,
    GLOBAL_WORKLOAD_OPTIMIZER,
};
pub use container::{
    ContainerCapability, ContainerError, ContainerID, ContainerInfo,
    ContainerRuntime as CoreContainerRuntime, ContainerState, RuntimeCapability, RuntimeStats,
    SimpleContainer, SimpleContainerRuntime,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use drivers::{
    DeviceGeneration, GpuCommand, GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType,
    InputDriver, InputEvent, InputType, NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    PeripheralDevice, PeripheralManager, PowerState, StorageCommand, StorageDriver, StorageError,
    StorageType, UsbHidDriver, VesaDriver, VesaError, VesaModeInfo, E1000RxDescriptor,
    E1000TxDescriptor, IntelE1000Driver, LegacyAudioAc97, ModernAudioIntelHda, ModernNvmeDriver,
    ModernUsbPrinterDriver, ModernWifiDriver, TouchJingosDriver, VirtioBlkDriver, VirtioDeviceType,
    VirtioMmioHeader, VirtioNetDriver, VirtioRngDriver, UnifiedDmaBroker, SelfHealingDriverManager,
    DmaDescriptor, DeviceCommandType, DeviceTransactionLog, GLOBAL_DMA_BROKER, GLOBAL_HEALING_MANAGER,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
};
pub use kernel::{
    AiNativeRuntime, BuddyAllocator, Channel, EnergyAwareScheduler,
    IpcError, IpcManager, MemoryBlock, Message,
    PAGE_SIZE, PrivacyFirstSandbox, Priority, Process, ProcessState,
    RoundRobinConfig, RoundRobinScheduler,
    Scheduler, SchedulerError, SelfHealingKernel, SigmaFsPlusPlus, UniversalAbiTranslator,
    UserDefinedKernelFunctions,
};
pub use network::{
    compute_checksum as compute_net_checksum, IPv4Address, NetworkPacket, PacketRingBuffer,
    RingTcpState, TcpConnection, TcpError, TcpSegment, TcpSocket, TcpStack, TcpState,
    ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
};
// pub use observability::{
//     Metric, MetricCapability, MetricID, MetricInfo, MetricType, ObservabilityError,
//     ObservabilityStack, ObservabilityStats, SigmaDebug, SigmaMetrics, SigmaTrace, SimpleMetric,
//     SimpleObservabilityStack, SimpleSigmaDebug, SimpleSigmaMetrics, SimpleSigmaTrace, SimpleSpan,
//     Span, SpanCapability, SpanInfo, StackCapability, TraceID,
// };
pub use distro::{
    AppManifest, CertificationStatus, ComponentType, HardwareCertificate,
    HardwareCertificationProgram, HardwareProfile, HardwareRegressionSuite, QAStagedRelease,
    ReleaseStage, SoftwareCertificationProgram,
    BountyStatus, BugBountyProgram, BugBountyReport, CommunityConference, ConferenceTalk,
    ForumChannel, ForumPost, HelpSystem, HowToGuide, ManPage, WikiPage,
    DllLoader, DllModule, GdiObjectType, LinuxSyscall, PosixTranslation, RegistryType,
    RegistryValue, Win32Gdi, WindowsRegistry,
    BuildJob, BuildStatus, CrossBuildPipeline, DevTool, DeveloperToolkit, PackageBuildService,
    TargetArch,
    AuditResult, AuditRule, ComplianceAuditor, ConfigHook, DirectoryService, DirectoryUser,
    ImeCandidate, InputMethodEngine, LanguagePack, LocaleManager, RegionalSettings,
    AdminAction, AiSysAdmin, IntegrityState, P2pNode, PqcSelfHealing, SovereignP2PSync,
    TimeTravelCheckpoint, TimeTravelEngine, NetplanConfig, NetplanManager,
    LivepatchPatch, LivepatchManager,
    BackupSnapshot, BackupSystem, KernelTrace, LiveDebugger, RescueISO, RescueISOManager,
    CanFrame, EcuController, EduChallenge, EduPlayground, HpcClusterJob, HpcJobState,
    MpiCommunicator,
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
    FileTransfer, RemoteError, RemoteSession,
    RemoteShell, SessionID, SessionState, ShellError, ShellID, ShellManager,
    SimpleFileTransfer, SimpleRemoteDesktop, SimpleRemoteSession, SimpleScreenSharing,
    SimpleShellManager,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use kernel::vmm_paging::{PageTableFlags as VmmPageFlags, PageTableManager as VmmPageTableManager, VirtualMemoryManager as VmmManager, VmArea, VmProtection};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{
    CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise, SecurityEnforcer as AndroidStyleSecurityEnforcer,
    PORT_ALLOW_SSL, PORT_ALLOW_TCP,
};
pub use shell::{
    CommandError as ShellCommandError, ShellCommand, ShellRepl, ShellSession, SimpleShellSession,
};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageImporter, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction, Version,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
