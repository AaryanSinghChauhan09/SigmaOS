#![allow(warnings)]
#![allow(clippy::all)]
// SigmaOS Library
// Core library for SigmaOS operating system
#![allow(clippy::all, unused)]

pub mod accessibility;
pub mod ai;
pub mod audio;
pub mod automation;
pub mod compatibility;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod distro;
pub mod driver;
pub mod drivers;
pub mod ecosystem;
pub mod education;
pub mod fs;
pub mod init;
pub mod net;
pub mod filesystem;
pub mod finance;
pub mod kernel;
pub mod klib;
pub mod media;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod productivity;
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
pub mod boot {
    pub mod firmware_bridge;
    pub mod bridge_grid;
}
pub mod toolchain {
    pub mod adapter;
    pub mod capsule;
    pub mod codex;
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
pub use ai::{
    Agent, Agent as SaiAgent, AgentOrchestrator, AgentOrchestrator as SaiOrchestrator, AgentRole,
    AgentState, AgentTask, AgentTask as SaiTask, AiError, ComputeBackend, LocalModel, ModelSize,
    SaiEngine, Task as AiTask, TaskStatus, Tensor, TensorCore,
};
pub use audio::{
    AlsaAudioStack, AudioChannels, AudioCodec, AudioDirection as AlsaDirection, AudioDriver,
    AudioDriverError, AudioDriverResult, AudioFormat, AudioFormat as AlsaFormat, AudioMetadata,
    AudioSampleRate, ChannelConfig, DecodedAudio, MixerControl, PcmStream,
    SampleRate as AlsaSampleRate,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use compatibility::{
    AiTaskOrchestrator, ApplicationBinary, ArchiveProfile, BinaryFormat, BootInterface,
    BuildArchive, BuildCapsule, BuildLedgerSystem, BuildProfile, CapsuleVersion, ChronicleType,
    CompatibilityError, CompatibilityManager, CompatibilityMode, ConstellationNode,
    ConstellationSecurityModel, ContainerRuntime, D3dToVulkanTranslator, DriverClass,
    DriverEmulator, DriverMuseum, DriverRepositoryManager, EmulatedPeripheral, EmulatorProfile,
    ExhibitType, FirmwareBridgeManager, FirmwarePavilion, FirmwarePersona, FirmwareType,
    GapSandboxPolicy, HardwareDriver, HidGraphicsDriver, JobClass, KernelConstellation,
    KernelModule, KernelModuleManager, KernelShard, LedgerSnapshot, MemoryProtection, ModuleState,
    NetworkStackGateway, ObsoleteDevice, ObsoletePeripheral, PavilionType, PeFormat, PeLoader,
    PeripheralEmulationLibrary, PeripheralMuseum, PeripheralPod, RegistryManager, SecurityGrid,
    SecurityModel, SecurityPavilion, SecurityPolicyManager, ShardType, SyscallCapsule,
    SyscallChronicle, SyscallCompatibilityRegistry, TargetPlatform, TranslationLayer,
    User32MessageQueue, VirtualMemoryManager, Win32Error, Win32Message, WinSockAdapter,
};
pub use customization::{
    Action, AutoThemeScheduler, Condition, CustomizationEngine, CustomizationError, Routine,
    SituationalPersonalizer, Theme, TriggerType, WindowGridLayout, WorkspaceLayoutCustomizer,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use distro::{
    CanFrame, DiagnosticLogTool, EcuController, EduChallenge, EduPlayground, EosUpdateNotifier,
    EosWelcomeEngine, HpcClusterJob, HpcJobState, MirrorRanker, MpiCommunicator,
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
    FramebufferCompositor, Geometry, GpuDevice, HighContrastMode,
    ImageDecoder, ImageFormat, ImageMetadata, LayerBlendMode, LayoutStyle, Magnifier, Panel,
    PanelOrientation, PixelFormat, RenderLayer, ScreenReader, SigmaCompositor, Widget, WindowNode,
    WindowState, ZenithCompositor, ZenithCompositor as WaylandZenithCompositor, SCREEN_HEIGHT,
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
    DriverRegistry, DriverType as KernelDriverType, FileFlags, FileHandle, FileSystem, FsError as KernelFsError,
    GovernorType, HardwareMonitor, IoOperation, IoResult, IpcError, IpcError as PerfIpcError,
    IpcManager, IpcMessage, LinuxAbsorptionEngine, LinuxHeritage, MapFlags, MemoryBlock,
    MemoryError as KernelMemoryError, MemoryManager, TraitsMemoryManagerMetadata as MemoryManagerMetadata, Message, MonitorThreshold, NetworkError as KernelNetworkError,
    NetworkStack, TraitsNetworkStackMetadata as NetworkStackMetadata, NodeState, NumaAllocator, NumaNode,
    PageDirectoryController, PageDirectoryEntry, Priority, Process, ProcessProfile, ProcessState,
    RoundRobinConfig, RoundRobinScheduler, SanitizationLevel, SchedInstruction, SchedOpcode,
    Scheduler, SchedulerError, TraitsSchedulerMetadata as SchedulerMetadata, TraitsFilesystemMetadata as FilesystemMetadata, SecureDriverWrapper, SecureFreeDetector,
    SecureFreeStats, SignalDispatcher, SlabAllocator as KernelSlabAllocator, SlabCache,
    SlabCacheStats, SlabState, SocketDomain, SocketHandle, SocketProtocol, SocketType,
    SovereignCompilerOptimizer, SovereignIpcBus, SovereignSignal, UdfSchedVm, WatchdogAction,
    WatchdogDevice, WatchdogManager, WatchdogState, ZeroCopyQueue, PAGE_SIZE,
};
pub use legal::{
    ComplianceCert, ComponentLicense, LegalComplianceRegistry, LicenseType, PatentRecord,
};
pub use memory::{
    MemoryError, PageDirectory, PageDirectoryPointerTable, PageTable, PageTableEntry,
    PhysicalAddress, SimpleVMM, VirtualAddress, PAGE_SIZE_BYTES, PAGE_TABLE_ENTRIES,
};
pub use ml::{LLMInterface, ModelStatus, SigmaAid};
pub use net::{
    SovereignAdBlockRule, AdblockRule, BraveShield, BrowserCore, BrowserError,
    BrowserTab, BrowserTab as SovereignBrowserTab, BrowserTabState, CipherSuite,
    E1000NetworkDriver, Ipv6Address, Ipv6AddressType, Ipv6ExtensionHeader, Ipv6Header,
    Ipv6Interface, Ipv6Route, Ipv6Stack, NetworkDriverDevice, NetworkDriverManager,
    NetworkDriverType, NetworkError as ZenithNetworkError, NetworkPacketFrame,
    RouteEntry, RouteKey, RouteProtocol, RouteType, RoutingTable, Rtl8139NetworkDriver,
    SecurityLevel, SecurityProfile, SovereignBrowser, TabCapabilities, TabContainer, TabState,
    TlsConfig, TlsEngine, TlsSession,
    TlsState, TlsVersion, TrackingProtection, ZeroCopyPacketRing,
};
pub use network::{
    DnsError, DnsResolver, MDnsDiscovery, QuicConnection, QuicError,
    TcpConnection, TcpError, TcpSegment, TcpStack, TcpState,
};
pub use init::{
    InitSystem, Service as InitService, ServiceID, ServiceState as InitServiceState, SigmaInit, SimpleService,
    DependencyResolver as InitDependencyResolver, SimpleDependencyResolver, ServiceMonitor as InitServiceMonitor, SimpleServiceMonitor,
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
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{
    CapabilityGate, CapabilityToken, DecoyHoneyPot, ForensicAnalyzer, KAslrHardener,
    KaliSnifferAudit, PassComplexityAuditor, Permission, PledgeManager, PledgePromise,
    RecoveredFile, SigmaPortScanner, StackCanaryGuard, VulnerabilitySeverity, WxorEPageGuard,
    ZeroizeSec,
};
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    AurRecipeCompiler, ContentAddressedStore, CryptoVerifier, PackageRecipe, PacmanDbAdapter,
    RollingSyncManager, SatSolver, Transaction,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};

pub mod init {
    pub mod systemd_init;
}
pub use init::systemd_init::{
    SystemdEngine, SystemdUnit, UnitState, UnitType,
};

pub mod virt;
pub use virt::hypervisor::{
    Guest, GuestID, GuestState, Hypervisor, HypervisorError, SimpleGuest, SimpleHypervisor,
    VirtualizationGeneration,
};
pub use virt::microvm::{
    MicroVM, MicroVMState, SandboxManager, SandboxPolicy, SimpleMicroVM, SimpleSandboxManager,
};
pub use boot::firmware_bridge::{
    FirmwareType, FirmwareBridge,
};
pub use boot::bridge_grid::{
    BIOSBridgeGrid, UEFIBridgeGrid, CorebootBridgeGrid, FirmwareBridgeGrid,
};
pub use toolchain::adapter::{
    ToolchainProfile, ToolchainAdapter,
};
pub use toolchain::capsule::{
    CapsuleProfile, BuildCapsule,
};
pub use toolchain::codex::{
    CodexCategory, CodexEntry, BuildCodex,
};
pub use compatibility::persona::{
    PersonaVersion, KernelPersonaContainer, SyscallCategory, SyscallNode, SyscallGraph,
};
pub use compatibility::abi_translator::{
    CpuArchitecture, ABITranslator,
};
pub use compatibility::lattice::{
    LatticeFeature, KernelLattice, SyscallLifecycle, SyscallHistory, SyscallTracker,
};
pub use compatibility::prism::{
    PrismFacet, KernelPrism, LedgerEntry, SyscallLedgerbook,
};
pub use compatibility::canonical::{
    SigmaSubiquity, SigmaNetplan, SigmaCloudInit, SigmaMultipass, SigmaCurtin,
};
pub use scheduler::numa_scheduler::{
    NumaNode, NumaScheduler, Node as LFNode, MichaelScottQueue, TreiberStack,
};
pub use crypto::vectorized_pqc::{
    VectorizedPqcEngine,
};
pub use network::revival::{
    RevivalProtocol, NetRevival,
};
pub use driver::simulation::{
    SimType, PeripheralSim,
};
pub use driver::mapper::{
    MapperCategory, DriverMapper,
};
pub use driver::pods::{
    PodType, PeripheralPod,
};
pub use driver::vault::{
    VaultEntry, DriverArchiveVault,
};
pub use driver::grid::{
    GridSlotType, PeripheralArchiveGrid,
};
pub use security::bridge::{
    LegacySecurityType, SecurityBridge,
};
pub use security::prism::{
    SecurityFacet, SecurityPrism,
};
