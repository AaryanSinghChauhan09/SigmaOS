#![allow(clippy::all, warnings)]

// SigmaOS Library
// Core library for SigmaOS operating system
#![allow(clippy::all, unused)]

pub mod accessibility;
pub mod automation;
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
pub mod plugin;
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
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use plugin::{
    ManagerCapability, Plugin, PluginCapability, PluginError, PluginID, PluginInfo, PluginManager,
    PluginState, PluginStats, SimplePlugin, SimplePluginManager,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise};
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
