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
pub mod graphics;
pub mod init;
pub mod ipc;
pub mod kernel;
pub mod klib;
pub mod ml;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod distro;
pub mod package;
pub mod performance;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod scheduler;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod virtualization;

pub mod hardware {
    pub mod compatibility;
    pub mod win32;
}
pub mod power {
    pub mod governor;
}
pub mod boot;
pub mod toolchain {
    pub mod adapter;
    pub mod capsule;
    pub mod codex;
    pub mod bootstrap;
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
    ApplicationBinary, BinaryFormat, CompatibilityError,
    CompatibilityManager, CompatibilityMode, ContainerRuntime, TargetPlatform, TranslationLayer,
    SigmaSubiquity, SigmaNetplan, SigmaCloudInit, SigmaMultipass, SigmaCurtin,
    DnfPackageResolver, MockChrootBuilder, KojiBuildServer, BodhiUpdateTriage,
};
pub use container::{
    Container, ContainerError, ContainerImage, ContainerNetwork, ContainerRuntime, ContainerState,
    Pod, PortMapping, RestartPolicy, RuntimeStats, Volume, VolumeMount,
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
pub use graphics::paint::ColorRgba;
pub use ipc::{
    unix_socket::{UnixSocketType, UnixSocketAddress, UnixSocketState, UnixSocket, UnixSocketManager},
    signals::{SignalType, SignalDisposition, PendingSignal, ProcessSignalState, SignalDeliverySystem},
};
pub use kernel::{
    ABIManager, AiNativeRuntime, BuddyAllocator, Channel, EnergyAwareScheduler, FastPathIpc,
    Generation, GenerationManager, InterruptMechanism, IpcError, IpcManager, KernelGraph, KernelPersona, KernelPlugin,
    KernelPluginManager, LegacyScheduler, MemoryBlock, Message, MetaKernel, MicroDriver, NetPod,
    PAGE_SIZE, PolicyError, PolicyManager, PrivacyFirstSandbox, Priority, Process, ProcessState, ProcessManager, ProcessId,
    ProtectionDomain, PrivilegeLevel, ResourceBroker, RoundRobinConfig, RoundRobinScheduler,
    Scheduler, SchedulerError, SelfHealingKernel, SigmaFsPlusPlus, UniversalAbiTranslator,
    UserDefinedKernelFunctions, GapError, Pml4PageTableEntry, VirtualMemoryPagingManager, VirtualMemoryManager,
    IrqRoutingTable, AcpiInterruptManager, JournalState, JournalBlock, MetadataJournal, Tty,
};
pub use network::{
    compute_checksum as compute_net_checksum, IPv4Address, NetworkPacket, PacketRingBuffer,
    RingTcpState, TcpConnection, TcpError, TcpSegment, TcpSocket, TcpStack, TcpState,
    ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
};
pub use observability::{
    ObservabilityError, ObservabilityStack, SigmaDebug, SigmaMetrics, SigmaTrace,
    SimpleObservabilityStack,
    CognitiveOSNarrator, AdaptiveComplianceGater, SynestheticFeedbackEngine, GenerativeConfigParser, InterplanetaryDtnRoute, CollectiveSimulationNode,
};
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

    // Newly registered modules representing missing components compared to linux & bsd distros
    PkgBuild, AurClient, SandboxedCompiler, AlpmDatabase,
    OpenBsdSecurity, ZfsManager, PortsManager, PfFirewall, BsdJail,
    AkabeiBundle, AkabeiPackageEngine, KapudanAssistant, TribeInstaller,
    BundleType, DesktopTheme, ChakraInstallerStep,
    DebianPackageManager, SnapPackageManager, DebianControl, UbuntuDesktopIntegration,
    DnfPackageManager, RpmPackage, SelinuxPolicy, SystemdService,
    UseFlag, FeatureSet, BuildSpec, CpuOptimizationDetector, SigmaBuildGraph, BuildError as GentooBuildError,
    RollingReleaseChannel, RollingPackage, RollingReleaseManager, MinimalBaseInstaller, UserPackageRepository,
    BtrfsVolumeManager, BtrfsSubvolume, BtrfsCompression, SnapshotPolicy, SystemSnapshotManager, SystemSnapshot,
    DeclarativeSystemConfig, ServiceConfig, UserConfig, BootConfig, NixStyleStorePath, AtomicUpgradeEngine,
    EphemeralSessionManager, EncryptedPersistentStorage, NetworkPrivacyMode, Amnesic, RamDisk,
    PenTestToolRegistry, PenTestTool, PenTestCategory, LiveForensicsSession, CustodyEntry,
    MinimalRuntime, LibcBackend, OpenRcStyleInit, Runlevel, InitService,
    BoreSchedulerConfig, Task, BoreScheduler, OptimizedKernelProfile, GarudaBtrfsLayout, SnapperIntegration, SnapperConfig,
    DistroReleaseError, SigmaDistroEngine, AptSource, DebControlFile, DebianAptPackageManager,
    HostMapping, HostResolver, SwapPageFrame, SwapSpaceManager, ImprovementsSnapPackageManager,
    ImprovementsSnapPackage, SnapConfinement, LtsReleaseManager, ZypperPackageManager, ZypperRepo, YastConfigManager, YastModule,
    SelinuxManager, SelinuxMode, SystemdServiceManager, ImprovementsSystemdService, ServiceStatus, SystemdTarget,
    PortagePackageManager, ImprovementsMintUpdateManager, MintUpdate, PopShop, PopApp, PantheonFileManager,
    AppCenter, AppCenterApp, PamacManager, SolusRollingManager, BudgieDesktop, BudgieSettings, ZorinWineManager, DesktopLayoutSwitcher,
    DesktopLayout, ImprovementsDdeControlCenter, DisplaySettings, SoundSettings, NetworkSettings,
    MxSnapshotTool, ImprovementsMxSnapshot, MxPackageInstaller, LinuxMintEnhancements, MintTools, CinnamonSettings,
    PanelSettings, LinuxDistroCompatibilityEngine, ArchLinuxFeatures, FedoraFeatures, UbuntuFeatures, GentooFeatures,
    OpenSuseFeatures, RhelFeatures, ManjaroFeatures, SolusFeatures, ZorinFeatures, DeepinFeatures, MxFeatures,
    UbuntuSnapManager, OpenSuseZypper, RhelSelinuxManager, GentooPortage, ManjaroPamac, ZorinWineIntegration, DeepinDdeControl,
    PopShopIntegration, ElementaryPantheon, SolusBudgie, LinuxDistroGapCloser,
    EbpfOpcode, EbpfInstruction, SovereignEbpfEngine, ArchDependencyResolver, PackageNode,
    FreeBSDJail, OpenBSDUnveil, OpenBSDPledge, NixStyleStore, PinRule, AptPinStore, DriverContext,
    RumpDriver, NetBsdRumpRouter, GentooUseFlagsManager, OpenRCService,
    InstallationTarget, ParityInstallerStep, InstallerError, LiveInstaller, SovereignInstaller,
    UpdateChannel, SystemStateStatus, UpdateError, ChannelManager, SovereignChannelManager, SigmaAppBundle,
    BundleError, AppBundleRuntime, SovereignBundleRuntime, CpuArchitecture, HalError, HardwareAbstractionLayer, SovereignHal,
    PreseedVariable, SovereignPreseedParser,
};
pub use network::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageFormatAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
    AurClient as CoreAurClient, AurPackage, PkgBuildRecipe, BuildSandboxConfig, BuiltPackage,
    PkgBuildParser, AurError, BuildError, InstallError, ParseError,
};
pub use init::{
    SigmaInit, Service, ServiceState, RestartPolicy, SystemTarget,
    Supervisor, DependencyGraph, ServiceError, DependencyError,
};
pub use scheduler::{
    SchedExtScheduler, SchedulingPolicy, BpfProgram, BpfMapDescriptor, BpfMapType,
    UserSpaceScheduler, SchedulerConfig, ScheduleDecision, SchedulerStats, SchedError,
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
pub use security::{
    CapabilityGate, CapabilityToken, DomainID, DomainOrchestrator, DomainType, IsolatedDomain,
    IsolationError, Permission, PledgeManager, PledgePromise, SecurityEnforcer as AndroidStyleSecurityEnforcer,
    PORT_ALLOW_SSL, PORT_ALLOW_TCP,
    HardenedSysctlManager, SysctlConfig, SysctlValue, SysctlError,
};
pub use shell::{
    CommandError as ShellCommandError, ShellCommand, ShellRepl, ShellSession, SimpleShellSession,
};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageDependencyResolver, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction, Version, MAX_RECIPE_DEPENDENCIES, PackageFormatAdapter, UniversalPackageManager, AdapterError,
    DebAdapter, RpmAdapter, PacmanAdapter,
};
pub use virtualization::{
    Container as VirtualizationContainer, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};

// Added networking mod
pub mod networking;
