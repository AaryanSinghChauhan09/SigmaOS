#![allow(clippy::all, warnings)]

// SigmaOS Library
// Core library for SigmaOS operating system
#![allow(clippy::all, unused)]

pub mod accessibility;
pub mod audio;
pub mod automation;
pub mod boot;
pub mod community;
pub mod compatibility;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod ecosystem;
pub mod education;
pub mod filesystem;
pub mod finance;
pub mod fs;
pub mod init;
pub mod kernel;
pub mod klib;
pub mod net;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod productivity;
pub mod resilience;
pub mod scheduler;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod storage;
pub mod support;
pub mod system;
pub mod tools;
pub mod tracing;
pub mod unimplemented_features;
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
pub mod ai {
    pub mod agent;
    pub mod orchestrator;
}
pub mod boot {
    pub mod firmware_bridge;
    pub mod bridge_grid;
}
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
pub use ai::{
    Agent, Agent as SaiAgent, AgentOrchestrator, AgentOrchestrator as SaiOrchestrator, AgentRole,
    AgentState, AgentTask, AgentTask as SaiTask, AiError, ComputeBackend, LocalModel, ModelSize,
    SaiEngine, Task, TaskStatus, TaskType, Tensor, TensorCore,
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
pub use boot::{
    PciBusScanner, PciClass, PciDevice, PostDiagnostics, PostStatus, PostTest, TestType,
    PCI_MAX_BUS, PCI_MAX_DEVICE,
};
pub use community::{
    BugSeverity, BugTracker, CommunityIssue, ContributorProfile, FundingSustainability,
    IssueStatus, MentorshipProgram, OnboardingStage, Sponsor,
};
pub use compatibility::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
    KernelPersonality, KernelProxy, SyscallLedgerEntry, LedgerManager, LegacyDriver,
    StorageProxy, NetworkProxy, GraphicsProxy, DriverProxy, FirmwareInterface,
    BIOSProxy, UEFIProxy, CorebootProxy, FirmwareProxy, CompilerBackend, LegacyCProxy,
    LegacyCppProxy, LegacyAsmProxy, BuildProxy, SecurityModel, DACProxy, SELinuxProxy,
    ZeroTrustProxy, SecurityProxy, ObsoleteDevice, FloppyProxy, TapeProxy, CRTProxy,
    DotMatrixProxy, PeripheralProxy,
    OldLinuxRelease, OldLinuxCompatManager,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use device::{
    DeviceClass, DeviceDriver as PublicDeviceDriver, DeviceError, DeviceHotplug,
    DeviceManager as PublicDeviceManager, SimpleDevice, SimpleDeviceDriver, SimpleDeviceHotplug,
    SimpleDeviceManager,
};
pub use drivers::{
    create_cga_graphics, create_floppy_disk, create_parallel_printer, create_sound_blaster_16,
    AcpiTableParser, AdLibSynthDriver, AppleSiliconUnifiedMemoryBus, Bluetooth5_4_Adapter, BusType,
    CgaGraphicsDriver, ClockController, ClockError, CxlMemoryDriver, DeviceError as DdeDeviceError,
    DeviceId, DriverType, FloppyDiskDriver, GameportJoystickDriver, GenericClock, GenericDriver,
    GenericPin, GpuCommand, GpuDriver, GpuError, HardwareBroker, HidError, HidKeyboardEvent,
    HidReportType, IdeControllerDriver, InputDriver, InputEvent, InputType, IntelXeGpuDriver,
    KernelReleaseInfo, LinuxDdeShim, LinuxReleaseDriver, Longterm5_10_TpmDriver,
    Longterm5_15_SerialDriver, Longterm6_12_NetworkDriver, Longterm6_18_StorageDriver,
    Longterm6_1_InputDriver, Longterm6_6_AudioDriver, MainlineGpuDriver, Ne2000NetworkDriver,
    NetworkCommand, NetworkDriver, NetworkError, NetworkType, NvlinkBusDriver,
    ParallelPrinterDriver, PciIdeBridge, PcieGen5NvmeDriver, PcieGen6Bridge, PinController,
    PinDirection, PinError, PinPull, Prepatch6_23_Rc1_AiDriver, Ps2MouseDriver, Sata3Controller,
    SerialMouseDriver, SocClockController, SocPinController, SoundBlaster16Driver,
    Stable6_22_SensorDriver, StorageCommand, StorageDriver, StorageError, StorageType,
    Thunderbolt4Controller, UdfAncientDevice, UdfInterpreter, UefiGopDriver, Ufs4StorageDriver,
    UnifiedPeripheral as DdeUnifiedPeripheral, UnifiedSocController, Usb4HostController,
    UsbHidDriver, VesaDriver, VesaError, VesaModeInfo, VgaTextModeDriver, WasmDriverVm,
    Wifi7Adapter, WindowsNdisWrapper, XhciHostController,
};
pub use ecosystem::{
    ArchTier, ArchitecturePort, EcosystemCertification, EcosystemManager, EcosystemPlatform,
    EnterprisePartner,
};
pub use education::{
    DocAsset, DocFormat, EducationOutreachManager, LearningPath, UniversityPartnership,
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
    FramebufferCompositor, Geometry, GpuDevice, HighContrastMode, ImageDecoder, ImageFormat,
    ImageMetadata, LayerBlendMode, LayoutStyle, Magnifier, Panel, PanelOrientation, PixelFormat,
    RenderLayer, ScreenReader, SigmaCompositor, Widget, WindowNode, WindowState, ZenithCompositor,
    ZenithCompositor as WaylandZenithCompositor, SCREEN_HEIGHT, SCREEN_WIDTH,
};
pub use init::{
    DependencyResolver as InitDependencyResolver, InitSystem, Service as InitService, ServiceID,
    ServiceMonitor as InitServiceMonitor, ServiceState as InitServiceState, SigmaInit,
    SimpleDependencyResolver, SimpleService, SimpleServiceMonitor,
};
pub use iso::builder::{
    BuildError, BuildPipeline, BuildStatus, BuildStep, GRUBConfig, ISOPackager,
    SimpleBuildPipeline, SimpleGRUBConfig, SimpleISOPackager,
};
pub use finance::{
    GoodsType, GstCalculator, GstRate, GstRegime, GstResult, GstState, TdsCalculator, TdsResult,
    TdsSection,
};
pub use kernel::{
    AbsorbedBuddyAllocator, AbsorbedCfsScheduler, AbsorbedDriverInfo, AbsorbedExt4Driver,
    AbsorbedTcpStack, AbsorbedUsbHidDriver, AbsorptionError, AbsorptionStatus,
    AllocationPolicy as NumaAllocationPolicy, BuddyAllocator, Channel, CpuInstructionExtension,
    CpufreqManager, CpufreqPolicy, CpufreqStats, DeviceDriver, DriverError, DriverMetadata,
    DriverRegistry, DriverType as KernelDriverType, FileFlags, FileHandle, FileSystem,
    FsError as KernelFsError, GovernorType, HardwareMonitor, IoOperation, IoResult, IpcError,
    IpcError as PerfIpcError, IpcManager, IpcMessage, LinuxAbsorptionEngine, LinuxHeritage,
    MapFlags, MemoryBlock, MemoryError as KernelMemoryError, MemoryManager, Message,
    MonitorThreshold, NetworkError as KernelNetworkError, NetworkStack, NodeState, NumaAllocator,
    NumaNode, PageDirectoryController, PageDirectoryEntry, Priority, Process, ProcessProfile,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, SanitizationLevel, SchedInstruction,
    SchedOpcode, Scheduler, SchedulerError, SecureDriverWrapper, SecureFreeDetector,
    SecureFreeStats, SignalDispatcher, SlabAllocator as KernelSlabAllocator, SlabCache,
    SlabCacheStats, SlabState, SocketDomain, SocketHandle, SocketProtocol, SocketType,
    SovereignCompilerOptimizer, SovereignIpcBus, SovereignSignal,
    TraitsFilesystemMetadata as FilesystemMetadata,
    TraitsMemoryManagerMetadata as MemoryManagerMetadata,
    TraitsNetworkStackMetadata as NetworkStackMetadata,
    TraitsSchedulerMetadata as SchedulerMetadata, UdfSchedVm, WatchdogAction, WatchdogDevice,
    WatchdogManager, WatchdogState, ZeroCopyQueue, PAGE_SIZE,
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
    AdblockRule, BraveShield, BrowserCore, BrowserError, BrowserTab,
    BrowserTab as SovereignBrowserTab, BrowserTabState, CipherSuite, E1000NetworkDriver,
    Ipv6Address, Ipv6AddressType, Ipv6ExtensionHeader, Ipv6Header, Ipv6Interface, Ipv6Route,
    Ipv6Stack, NetworkDriverDevice, NetworkDriverManager, NetworkDriverType,
    NetworkError as ZenithNetworkError, NetworkPacketFrame, RouteEntry, RouteKey, RouteProtocol,
    RouteType, RoutingTable, Rtl8139NetworkDriver, SecurityLevel, SecurityProfile,
    SovereignAdBlockRule, SovereignBrowser, TabCapabilities, TabContainer, TabState, TlsConfig,
    TlsEngine, TlsSession, TlsState, TlsVersion, TrackingProtection, ZeroCopyPacketRing,
};
pub use network::{
    DnsError, DnsResolver, MDnsDiscovery, QuicConnection, QuicError, TcpConnection, TcpError,
    TcpSegment, TcpStack, TcpState,
};
pub use network::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
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
    ConflictResolution, DependencyResolver, PackageAdapter, PackageDependencyResolver,
    PackageError, PackageFormat, PackageSource, PackageState, SovereignPackage, SpacPackageManager,
    UnifiedPackage, UniversalPackageManager, Version,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use scheduler::{
    ComputeUnit, EevdfScheduler, Priority as ShellPriority, ProcessLifecycleManager,
    ResourceLimits, SInitSupervisor, Scheduler as ShellScheduler,
    SchedulerError as ShellSchedulerError, Service, ServiceState, Signal, SignalHandler,
    SignalManager, SimpleThread, Task, TaskState, Thread, ThreadID, ThreadState,
};
pub use security::{
    AppArmorManager, AppArmorProfile, CapabilityGate, CapabilityToken,
    ObjectType as SelinuxObjectType, Permission, Permission as SelinuxPermission, PledgeManager,
    PledgePromise, RuntimeCapabilityToken, SecurityContext as SelinuxContext, SecurityEnforcer,
    SecurityLabel, SecurityPolicy, SecurityRule,
};
pub use shell::{MultiCallShell, ShellCommand, ShellRepl, SysCommandType};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction as SigpkgTransaction,
};
pub use storage::{
    Column, QueryResult, SqlEngine, SqlType, SqlValue, Table, Transaction, TransactionState,
};
pub use support::{
    LtsRelease, RecoveryConfig, SupportContract, SupportServicesManager, SupportTier,
};
pub use system::{Generation, GenerationManager};
pub use tools::{
    AccessibilityFeature as SigmaAccessibilityFeature, ClusterNode, NodeState as ToolNodeState,
    SigmaAccess, SigmaCluster, SigmaDeploy, SigmaIdentity, SigmaToolError, UserIdentity,
};
pub use tracing::{SigmaTrace as TraceSigmaTrace, TraceEvent, TraceSpan};
pub use virtualization::{
    Cgroup, CgroupController, CgroupManager, CgroupState, CgroupSubsystem, Container,
    KubernetesPod, Namespace as VirtNamespace, NamespaceData,
    NamespaceManager as VirtNamespaceManager, NamespaceType as VirtNamespaceType, ResourcePool,
    VirtualMachine, VirtualizationError, VirtualizationOrchestrator, VirtualizationTech, VmState,
};

#[cfg(test)]
#[no_mangle]
pub unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout =
        Layout::from_size_align(size, 8).unwrap_or_else(|_| Layout::from_size_align(8, 8).unwrap());
    std_alloc(layout)
}

#[cfg(test)]
#[no_mangle]
pub unsafe extern "C" fn free(_ptr: *mut u8) {
    // No-op deallocation in host test environment to avoid layout-tracking complexity.
}
