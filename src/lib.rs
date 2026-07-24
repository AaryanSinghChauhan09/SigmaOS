#![allow(clippy::all, warnings)]

// SigmaOS Library
// Core library for SigmaOS operating system
#![allow(clippy::all, unused)]

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
pub mod graphics {
    pub mod paint;
    pub mod video;
    pub mod compositor;
}
pub mod hardware {
    pub mod win32;
    pub mod compatibility;
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
pub use community::{
    BugSeverity, BugTracker, CommunityIssue, ContributorProfile, FundingSustainability,
    IssueStatus, MentorshipProgram, OnboardingStage, Sponsor,
};
pub use compatibility::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
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
    StoreError, StoreApp, SigmaSoftwareStore,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
    MediaFormat, PlaybackState, AudioTrack, SigmaMediaEngine,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
    BackupError, BackupSnapshot, SigmaTimeshift,
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
pub use graphics::paint::{
    ColorRgba, BlendMode, PhotoError, ImageFilter, CanvasLayer, RasterLayer, GaussianBlurFilter, GrayscaleConversionFilter,
};
pub use graphics::video::{
    VideoError, PixelRgba, VideoFrame, VideoEffect, TimelineClip, YuvToRgbEffect, SubtitleOverlayEffect, VideoClip,
};
pub use graphics::compositor::{
    Position, Size, Rectangle, Color, Surface, SurfaceInfo, PixelFormat, SurfaceCapability, BitmapSurface, Window, WindowInfo, WindowCapability, SimpleWindow, Compositor, GraphicsError, CompositorStats, CompositorCapability, SimpleCompositor,
};
pub use hardware::win32::{
    Win32Error, Win32Handle, PeFormat, PeLoader, RegistryManager, Win32Message, User32MessageQueue,
};
pub use hardware::compatibility::{
    HardwareCompatibilityManager, CompatibilityReport, CompatibilityResult, HardwareDevice, CompatibilityCheck,
};
