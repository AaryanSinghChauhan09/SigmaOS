// SigmaOS Library
// Core library for SigmaOS operating system

pub mod accessibility;
pub mod audio;
pub mod fs;
pub mod net;
pub mod automation;
pub mod finance;
pub mod governance;
pub mod iso;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod kernel;
pub mod klib;
pub mod ml;
pub mod network;
pub mod observability;
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
    pub mod zenith;
    pub mod zenith_compositor;
    pub mod image_decoder;
    pub mod gpu_driver;

    pub use compositor::{
        BitmapSurface, Color, Compositor, CompositorResult, CompositorStrategy, FramebufferCompositor,
        LayerBlendMode, Position, Rectangle, RenderLayer, SigmaCompositor, SimpleCompositor,
        SimpleWindow, Size, Surface, Window,
    };
    pub use zenith::*;
    pub use zenith_compositor::*;
    pub use image_decoder::*;
    pub use gpu_driver::*;
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
pub mod scheduler;
pub mod crypto;

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
    DagNode, FileDescriptor, FilePermissions, FileType, FsError, HashId, Inode, SigmaFS,
    VirtualFilesystem,
};
pub use finance::{
    GoodsType, GstCalculator, GstRate, GstRegime, GstResult, GstState, TdsCalculator, TdsResult,
    TdsSection,
};
pub use fs::{
    AhciSataController, AllocationStrategy as XfsAllocationStrategy, BlockStorageDevice,
    BlockStorageError, BtrfsExtent, BtrfsFilesystem, BtrfsSnapshot, BtrfsSubvolume, CasBlock,
    ChecksumType, CompressionType as BtrfsCompressionType, JournalBlock, JournalBlockType,
    MerkleNode, NvmeStorageController, SigmaFs, SigmaFsCasEngine, TransactionalJournal,
    XfsAllocationGroup, XfsExtent, XfsFilesystem, XfsInode, XfsJournal, XfsState,
    DILITHIUM5_SIGNATURE_SIZE, SHA256_HASH_SIZE,
};
pub use governance::{
    DemocraticProposal, DemocraticVoting, FoundationMember, FoundationModel, ReleaseType,
    RoadmapMilestone, TransparentRoadmap,
};
pub use graphics::{
    Animation, AnimationCurve, ColorSpace, CompositorError, CompositorError as ZenithError,
    CompositorResult, CompositorStrategy, DecodedImage, Framebuffer as GpuFramebuffer,
    FramebufferCompositor, Geometry, GpuDevice, GpuDriver as GraphicsGpuDriver, GpuState, GpuVendor, HighContrastMode,
    ImageDecoder, ImageFormat, ImageMetadata, LayerBlendMode, LayoutStyle, Magnifier, Panel,
    PanelOrientation, PixelFormat, RenderLayer, ScreenReader, SigmaCompositor, Widget, WindowNode,
    WindowState, ZenithCompositor, WaylandZenithCompositor, SCREEN_HEIGHT,
    SCREEN_WIDTH,
};
pub use iso::builder::{
    BuildError, BuildPipeline, BuildStatus, BuildStep, GRUBConfig, ISOPackager,
    SimpleBuildPipeline, SimpleGRUBConfig, SimpleISOPackager,
};
pub use kernel::{
    AbsorbedBuddyAllocator, AbsorbedCfsScheduler, AbsorbedDriverInfo, AbsorbedExt4Driver,
    AbsorbedTcpStack, AbsorbedUsbHidDriver, AbsorptionError, AbsorptionStatus,
    AllocationPolicy as NumaAllocationPolicy, BuddyAllocator, Channel, CpuInstructionExtension,
    CpufreqManager, CpufreqPolicy, CpufreqStats, DeviceDriver, DriverError, DriverMetadata,
    DriverRegistry, DriverType as KernelDriverType, FileFlags, FileHandle, FileSystem, FilesystemMetadata, FsError as KernelFsError,
    GovernorType, HardwareMonitor, IoOperation, IoResult, IpcError, IpcError as PerfIpcError,
    IpcManager, IpcMessage, LinuxAbsorptionEngine, LinuxHeritage, MapFlags, MemoryBlock,
    MemoryError as KernelMemoryError, MemoryManager, MemoryManagerMetadata, Message, MonitorThreshold, NetworkError as KernelNetworkError,
    NetworkStack, NetworkStackMetadata, NodeState, NumaAllocator, NumaNode,
    PageDirectoryController, PageDirectoryEntry, Priority, Process, ProcessProfile, ProcessState,
    RoundRobinConfig, RoundRobinScheduler, SanitizationLevel, SchedInstruction, SchedOpcode,
    Scheduler, SchedulerError, SchedulerMetadata, SecureDriverWrapper, SecureFreeDetector,
    SecureFreeStats, SignalDispatcher, SlabAllocator as KernelSlabAllocator, SlabCache,
    SlabCacheStats, SlabState, SocketDomain, SocketHandle, SocketProtocol, SocketType,
    SovereignCompilerOptimizer, SovereignIpcBus, SovereignSignal, UdfSchedVm, WatchdogAction,
    WatchdogDevice, WatchdogManager, WatchdogState, ZeroCopyQueue, PAGE_SIZE,
};
pub use network::{
    DnsError, DnsResolver, MDnsDiscovery,
    QuicConnection, QuicError,
    TcpConnection, TcpError, TcpSegment, TcpStack, TcpState, ZeroCopyPacketRing,
};
pub use net::{
    BraveShield, BrowserCore, BrowserError,
    BrowserTab, BrowserTabState, CipherSuite, SovereignBrowser, TabCapabilities, TabContainer, TabState, TrackingProtection,
    NetworkDriverDevice, NetworkDriverManager, NetworkDriverType, NetworkError as ZenithNetworkError, NetworkPacketFrame, RouteEntry, RouteKey, RouteProtocol, RouteType, RoutingTable, Rtl8139NetworkDriver, SecurityLevel, SecurityProfile,
    TlsConfig, TlsEngine, TlsSession, TlsState, TlsVersion,
};
pub use observability::{
    ObservabilityError, ObservabilityStack, SigmaDebug, SigmaMetrics, SigmaTrace,
    SimpleObservabilityStack,
};
pub use distro::{
    AppManifest, CertificationStatus, ComponentType, HardwareCertificate,
    HardwareCertificationProgram, HardwareProfile, HardwareRegressionSuite, QAStagedRelease,
    ReleaseStage, SoftwareCertificationProgram,
    BountyStatus, BugBountyProgram, BugBountyReport, CommunityConference, ConferenceTalk,
    ForumChannel, ForumPost, HelpSystem, HowToGuide, ManPage, WikiPage,
    DllLoader, DllModule, GdiObjectType, LinuxSyscall, PosixTranslation, RegistryType,
    RegistryValue, Win32Gdi, WindowsRegistry,
    BuildJob, BuildStatus as DistroBuildStatus, CrossBuildPipeline, DevTool, DeveloperToolkit, PackageBuildService,
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
    ConflictResolution, DependencyResolver, PackageFormatAdapter, PackageError, PackageFormat,
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
    SplitDirection as TmuxSplitDirection, LayoutPreset as TmuxLayoutPreset,
    TmuxPane, TmuxWindow, TmuxSession, TmuxSessionManager,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use scheduler::{
    ComputeUnit, EevdfScheduler, Priority as ShellPriority,
    SInitSupervisor, Scheduler as ShellScheduler,
    SchedulerError as ShellSchedulerError, Service, ServiceState, SimpleThread, Task, TaskState, Thread, ThreadID, ThreadState,
};
pub use security::{
    CapabilityGate, CapabilityToken, DomainID, DomainOrchestrator, DomainType, IsolatedDomain,
    IsolationError, Permission, PledgeManager, PledgePromise, SecurityEnforcer as AndroidStyleSecurityEnforcer,
    PORT_ALLOW_SSL, PORT_ALLOW_TCP,
};
pub use shell::{
    CommandError as ShellCommandError, ShellCommand, ShellRepl, ShellSession, SimpleShellSession,
};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageDependencyResolver, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction, Version, MAX_RECIPE_DEPENDENCIES, PackageFormatAdapter as SigPkgFormatAdapter, UniversalPackageManager as SigPkgUniversalPackageManager, AdapterError,
    DebAdapter, RpmAdapter, PacmanAdapter,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
