// SigmaOS Library
// Core library for SigmaOS operating system
#![allow(unused_imports, unused_variables, dead_code, unused_mut, clippy::all)]

pub mod accessibility;
pub mod automation;
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
pub mod governance;
pub mod kernel;
pub mod legal;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod support;
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
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, HtmlRendererCapability, MediaDecoderCapability,
    SupersetApplicationCapability, TargetPlatform, TranslationLayer,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use drivers::{
    AdLibSynthDriver, AppleSiliconUnifiedMemoryBus, Bluetooth5_4_Adapter, CgaGraphicsDriver,
    CxlMemoryDriver, create_cga_graphics, create_floppy_disk, create_parallel_printer,
    create_sound_blaster_16, FloppyDiskDriver, GameportJoystickDriver, GpuCommand, GpuDriver,
    GpuError, HidError, HidKeyboardEvent, HidReportType, IdeControllerDriver, InputDriver,
    InputEvent, InputType, IntelXeGpuDriver, KernelReleaseInfo, LinuxReleaseDriver,
    Longterm5_10_TpmDriver, Longterm5_15_SerialDriver, Longterm6_12_NetworkDriver,
    Longterm6_18_StorageDriver, Longterm6_1_InputDriver, Longterm6_6_AudioDriver,
    MainlineGpuDriver, Ne2000NetworkDriver, NetworkCommand, NetworkDriver, NetworkError,
    NetworkType, NvlinkBusDriver, ParallelPrinterDriver, PciIdeBridge, PcieGen5NvmeDriver,
    PcieGen6Bridge, Prepatch6_23_Rc1_AiDriver, Ps2MouseDriver, Sata3Controller,
    SerialMouseDriver, SoundBlaster16Driver, Stable6_22_SensorDriver, StorageCommand,
    StorageDriver, StorageError, StorageType, Thunderbolt4Controller, UdfAncientDevice,
    Ufs4StorageDriver, Usb4HostController, UsbHidDriver, VesaDriver, VesaError, VesaModeInfo,
    VgaTextModeDriver, Wifi7Adapter,
};
pub use ecosystem::{
    ArchTier, ArchitecturePort, EcosystemCertification, EcosystemManager, EcosystemPlatform,
    EnterprisePartner,
};
pub use education::{
    DocAsset, DocFormat, EducationOutreachManager, LearningPath, UniversityPartnership,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
};
pub use governance::{
    DemocraticProposal, DemocraticVoting, FoundationMember, FoundationModel, ReleaseType,
    RoadmapMilestone, TransparentRoadmap,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
};
pub use legal::{
    ComplianceCert, ComponentLicense, LegalComplianceRegistry, LicenseType, PatentRecord,
};
pub use network::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
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
pub use security::{CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise};
pub use shell::{ShellCommand, ShellRepl};
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
