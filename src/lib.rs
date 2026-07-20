#![allow(warnings)]
#![allow(clippy::all)]
// SigmaOS Library
// Core library for SigmaOS operating system
#![allow(clippy::all, unused)]

pub mod accessibility;
pub mod automation;
pub mod audio;
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
pub mod productivity;
pub mod resilience;
pub mod storage;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod support;
pub mod system;
pub mod tracing;
pub mod virtualization;
pub mod unimplemented_features;
pub mod phase_l_plans;


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
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, FhsConventionStatus, LsbProfile, PosixComplianceLevel,
    StandardsComplianceManager, TargetPlatform, TranslationLayer,
    IndianLanguage, LocalizationManager, LocalizationProvider,
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
};
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
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem, SigmaFS, DagNode, HashId,
};
pub use graphics::{
    ColorSpace, CompositorError, CompositorResult, CompositorStrategy, DecodedImage,
    FramebufferCompositor, ImageDecoder, ImageFormat, ImageMetadata, LayerBlendMode, RenderLayer,
    SigmaCompositor,
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
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
};
pub use legal::{
    ComplianceCert, ComponentLicense, LegalComplianceRegistry, LicenseType, PatentRecord,
};
pub use network::{
    AdblockRule, BrowserCore, BrowserTab, BrowserTabState, DnsError, DnsResolver, MDnsDiscovery,
    QuicConnection, QuicError, SecurityLevel, TabCapabilities, TcpConnection, TcpError,
    TcpSegment, TcpStack, TcpState, TrackingProtection,
};
pub use ml::{SigmaAid, LLMInterface, ModelStatus};
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
pub use security::{
    CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise, SecurityEnforcer,
    RuntimeCapabilityToken,
};
pub use shell::{ShellCommand, ShellRepl, MultiCallShell, SysCommandType};
pub use system::{Generation, GenerationManager};
pub use tracing::{SigmaTrace, TraceEvent, TraceSpan};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction,
};
pub use support::{
    LtsRelease, RecoveryConfig, SupportContract, SupportServicesManager, SupportTier,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
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
