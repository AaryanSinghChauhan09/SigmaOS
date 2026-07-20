#![allow(warnings)]
#![allow(clippy::all)]
// SigmaOS Library
// Core library for SigmaOS operating system
#![allow(clippy::all, unused)]

pub mod accessibility;
pub mod automation;
pub mod audio;
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
pub mod graphics;
pub mod governance;
pub mod iso;
pub mod kernel;
pub mod legal;
pub mod ml;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod phase_l_plans;
pub mod productivity;
pub mod resilience;
pub mod scheduler;
pub mod storage;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod support;
pub mod system;
pub mod tools;
pub mod tracing;
pub mod unimplemented_features;
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
pub use community::{
    BugSeverity, BugTracker, CommunityIssue, ContributorProfile, FundingSustainability,
    IssueStatus, MentorshipProgram, OnboardingStage, Sponsor,
};
pub use compatibility::{
    ApplicationBinary, BinaryFormat as CrossPlatformBinaryFormat, CompatibilityError as CrossPlatformError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, FhsConventionStatus, IndianLanguage, LocalizationManager,
    LocalizationProvider, LsbProfile, PosixComplianceLevel, StandardsComplianceManager,
    TargetPlatform, TranslationLayer,
    ApkLoader, BinderCallType, BinaryFormat as ScosmosBinaryFormat, CompatibilityError as ScosmosError, MachoLoader, PeBinaryLoader, ScosmosManager,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use drivers::{
    create_cga_graphics, create_floppy_disk, create_parallel_printer, create_sound_blaster_16,
    AcpiTableParser, AdLibSynthDriver, AppleSiliconUnifiedMemoryBus, Bluetooth5_4_Adapter,
    CgaGraphicsDriver, CxlMemoryDriver, FloppyDiskDriver, GameportJoystickDriver, GpuCommand,
    GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType, IdeControllerDriver,
    InputDriver, InputEvent, InputType, IntelXeGpuDriver, KernelReleaseInfo, LinuxReleaseDriver,
    Longterm5_10_TpmDriver, Longterm5_15_SerialDriver, Longterm6_12_NetworkDriver,
    Longterm6_18_StorageDriver, Longterm6_1_InputDriver, Longterm6_6_AudioDriver,
    MainlineGpuDriver, Ne2000NetworkDriver, NetworkCommand, NetworkDriver, NetworkError,
    NetworkType, NvlinkBusDriver, ParallelPrinterDriver, PciIdeBridge, PcieGen5NvmeDriver,
    PcieGen6Bridge, Prepatch6_23_Rc1_AiDriver, Ps2MouseDriver, Sata3Controller, SerialMouseDriver,
    SoundBlaster16Driver, Stable6_22_SensorDriver, StorageCommand, StorageDriver, StorageError,
    StorageType, Thunderbolt4Controller, UdfAncientDevice, UefiGopDriver, Ufs4StorageDriver,
    Usb4HostController, UsbHidDriver, VesaDriver, VesaError, VesaModeInfo, VgaTextModeDriver,
    Wifi7Adapter, XhciHostController,
    ClockController, ClockError, GenericClock, GenericPin, PinController, PinDirection, PinError,
    PinPull, SocClockController, SocPinController, UnifiedSocController,
    DeviceError as DdeDeviceError, DeviceId, DriverType, GenericDriver, HardwareBroker, LinuxDdeShim,
    UnifiedPeripheral as DdeUnifiedPeripheral, UdfInterpreter, WasmDriverVm, WindowsNdisWrapper, BusType,
};
pub use boot::{PciBusScanner, PciClass, PciDevice, PCI_MAX_BUS, PCI_MAX_DEVICE};
pub use ecosystem::{
    ArchTier, ArchitecturePort, EcosystemCertification, EcosystemManager, EcosystemPlatform,
    EnterprisePartner,
};
pub use education::{
    DocAsset, DocFormat, EducationOutreachManager, LearningPath, UniversityPartnership,
};
pub use audio::{
    AudioChannels, AudioCodec, AudioDriver, AudioDriverError, AudioDriverResult, AudioFormat,
    AudioMetadata, AudioSampleRate, DecodedAudio,
    AlsaAudioStack, AudioDirection as AlsaDirection, AudioFormat as AlsaFormat, ChannelConfig, MixerControl, PcmStream, SampleRate as AlsaSampleRate,
};
pub use filesystem::{
    DagNode, FileDescriptor, FilePermissions, FileType, FsError, HashId, Inode, SigmaFS,
    VirtualFilesystem,
};
pub use graphics::{
    ColorSpace, CompositorError, CompositorResult, CompositorStrategy, DecodedImage,
    FramebufferCompositor, ImageDecoder, ImageFormat, ImageMetadata, LayerBlendMode, RenderLayer,
    SigmaCompositor,
    Framebuffer as GpuFramebuffer, GpuDevice, GpuDriver, GpuState, GpuVendor, PixelFormat,
    Animation, AnimationCurve, CompositorError as ZenithError, HighContrastMode, LayoutStyle,
    Magnifier, Panel, PanelOrientation, ScreenReader, Widget, ZenithCompositor,
};
pub use governance::{
    DemocraticProposal, DemocraticVoting, FoundationMember, FoundationModel, ReleaseType,
    RoadmapMilestone, TransparentRoadmap,
};
pub use iso::builder::{
    BuildError, BuildPipeline, BuildStatus, BuildStep, GRUBConfig, ISOPackager,
    SimpleBuildPipeline, SimpleGRUBConfig, SimpleISOPackager,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, Message, MemoryBlock, PAGE_SIZE,
    AllocationPolicy as NumaAllocationPolicy, NumaAllocator, NumaNode, NodeState,
    Priority, Process, ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError,
    SanitizationLevel, SecureFreeDetector, SecureFreeStats,
    SlabAllocator as KernelSlabAllocator, SlabCache, SlabCacheStats, SlabState,
    CpufreqManager, CpufreqPolicy, CpufreqStats, GovernorType,
    HardwareMonitor, MonitorThreshold, WatchdogAction, WatchdogDevice, WatchdogManager, WatchdogState,
    IpcError as PerfIpcError, ProcessProfile, SchedInstruction, SchedOpcode, UdfSchedVm, ZeroCopyQueue,
    CpuInstructionExtension, SovereignCompilerOptimizer,
    IpcMessage, PageDirectoryController, PageDirectoryEntry, SignalDispatcher, SovereignIpcBus,
    SovereignSignal,
};
pub use legal::{
    ComplianceCert, ComponentLicense, LegalComplianceRegistry, LicenseType, PatentRecord,
};
pub use ml::{LLMInterface, ModelStatus, SigmaAid};
pub use ai::{
    Agent, AgentRole, AgentState,
    AgentOrchestrator, Task, TaskStatus, TaskType,
    Agent as SaiAgent, AgentOrchestrator as SaiOrchestrator, AgentTask, AgentTask as SaiTask,
    AiError, ComputeBackend, LocalModel, ModelSize, SaiEngine, Tensor, TensorCore,
};
pub use network::{
    AdblockRule, BrowserCore, BrowserTab, BrowserTabState, DnsError, DnsResolver, MDnsDiscovery,
    QuicConnection, QuicError, SecurityLevel, TabCapabilities, TcpConnection, TcpError,
    TcpSegment, TcpStack, TcpState, TrackingProtection,
    Ipv6Address, Ipv6AddressType, Ipv6ExtensionHeader, Ipv6Header, Ipv6Interface, Ipv6Route,
    Ipv6Stack,
    RouteEntry, RouteKey, RouteProtocol, RouteType, RoutingTable,
    CipherSuite, TlsConfig, TlsEngine, TlsSession, TlsState, TlsVersion,
    E1000NetworkDriver, NetworkDriverDevice, NetworkDriverManager, NetworkDriverType,
    NetworkError as ZenithNetworkError, NetworkPacketFrame, Rtl8139NetworkDriver, ZeroCopyPacketRing,
    AdBlockRule as SovereignAdBlockRule, BraveShield, BrowserTab as SovereignBrowserTab, BrowserError,
    SecurityProfile, SovereignBrowser, TabContainer, TabState,
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
    PackageSource, UnifiedPackage, UniversalPackageManager, PackageDependencyResolver, Version,
    PackageState, SpacPackageManager, SovereignPackage,
};
pub use productivity::{
    Achievement, AchievementType, Document as ProductivityDocument, DocumentEngine, DocumentFormat,
    DocumentMetadata, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer, ProductivityScore,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use storage::{Column, QueryResult, SqlEngine, SqlType, SqlValue, Table, Transaction, TransactionState};
pub use fs::{
    BtrfsExtent, BtrfsFilesystem, BtrfsSnapshot, BtrfsSubvolume, CompressionType as BtrfsCompressionType, ChecksumType,
    AhciSataController, BlockStorageDevice, BlockStorageError, JournalBlock, JournalBlockType,
    MerkleNode, NvmeStorageController, SigmaFs, TransactionalJournal,
    CasBlock, SigmaFsCasEngine, SHA256_HASH_SIZE, DILITHIUM5_SIGNATURE_SIZE,
    AllocationStrategy as XfsAllocationStrategy, XfsAllocationGroup, XfsExtent, XfsFilesystem, XfsInode, XfsJournal, XfsState,
};
pub use security::{
    CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise, SecurityEnforcer,
    RuntimeCapabilityToken,
    AppArmorManager, AppArmorProfile, ObjectType as SelinuxObjectType, Permission as SelinuxPermission,
    SecurityContext as SelinuxContext, SecurityLabel, SecurityPolicy, SecurityRule,
};
pub use shell::{ShellCommand, ShellRepl, MultiCallShell, SysCommandType};
pub use scheduler::{
    ComputeUnit, EevdfScheduler, Service, ServiceState, SInitSupervisor, Task, TaskState,
    ProcessLifecycleManager, ResourceLimits, Signal, SignalHandler, SignalManager,
    Scheduler, SchedulerError,
    Priority, SimpleThread, Thread, ThreadID, ThreadState,
};
pub use system::{Generation, GenerationManager};
pub use tracing::{SigmaTrace, TraceEvent, TraceSpan};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction,
};
pub use support::{
    LtsRelease, RecoveryConfig, SupportContract, SupportServicesManager, SupportTier,
};
pub use tools::{
    SigmaToolError, SigmaDeploy, SigmaCluster, ClusterNode, NodeState,
    SigmaIdentity, UserIdentity, SigmaAccess, AccessibilityFeature as SigmaAccessibilityFeature,
};
pub use virtualization::{
    Cgroup, CgroupController, CgroupManager, CgroupState, CgroupSubsystem,
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
    Namespace, NamespaceData, NamespaceManager, NamespaceType,
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
pub unsafe extern "C" fn free(ptr: *mut u8) {
    // No-op deallocation in host test environment to avoid layout-tracking complexity.
}
