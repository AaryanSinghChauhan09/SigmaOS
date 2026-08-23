// SigmaOS Library
// Core library for SigmaOS operating system

pub mod audio {
    pub mod driver;
    pub mod editor;
    pub mod pipewire;
}
pub use audio::driver::{
    AudioDevice, AudioDeviceID, AudioError, AudioManager, AudioMixer, AudioStream, AudioType,
    SimpleAudioDevice, SimpleAudioManager, SimpleAudioMixer, SimpleAudioStream,
};
pub use audio::editor::{
    AmplifyEffect, AudioEditor, AudioEffect, AudioTrack, EchoEffect, LowPassFilter,
    MultiTrackSession, NoiseGateEffect,
};
pub use audio::pipewire::{AudioGraph, AudioLink, AudioNode, GraphState as AudioGraphState, NodeType as AudioNodeType};

pub mod accessibility;
pub mod automation;
pub mod backup;
pub mod boot;
pub mod compatibility;
pub mod config;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod hal;
pub mod kernel;
pub mod klib;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod performance;
pub mod power;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod virtualization;
pub mod tracing;
pub mod crash;
pub mod media;
pub mod graphics;
pub mod gpu;
pub mod installer;

pub use accessibility::{
    AccessibilityCategory, AccessibilityError, AccessibilityFeature, AccessibilityFramework,
    AccessibilityProfile, AccessibilitySetting, BrailleDisplay, ColorFilter, KeyID, KeyType,
    Magnifier, MagnifierID, MagnifierManager, OnScreenKeyboard, ScreenReader, SimpleBrailleDisplay,
    SimpleColorFilter, SimpleMagnifier, SimpleMagnifierManager, SimpleOnScreenKeyboard,
    SimpleScreenReader, SimpleStickyKeys, SimpleVirtualKey, SimpleVoice, StickyKeys, VirtualKey,
    Voice, VoiceGender, VoiceID,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use backup::recovery::{BackupChunk, RecoveryManager, SystemSnapshot as BackupSystemSnapshot};
pub use boot::sigma_boot::{BootEntry as SigmaBootEntry, BootManager as SigmaBootManager, BootTheme};
pub use compatibility::{
    ApplicationBinary, BinaryFormat, CasObject, Clause, CompatibilityError, CompatibilityManager,
    CompatibilityMode, ContainerRuntime, ContentAddressedStorage, CreativeMatrix, DpllSatSolver,
    EverySearch, FancyZonesManager, FiletoolOverlay, FrugalLoader, ImageLayer, JoplinE2ee,
    LayoutZone, Literal, MetricAggregation, OssieCatalog, OssieDimension, OssieInterpreter,
    OssieMetric, OssieOntology, OssieRelationship, PledgePermission, PledgeUnveilSandbox,
    PqcSecureChannel, ProcMonitor, ProcessExplorerState, SemanticRow, SpreadsheetCore, SysDiag,
    TargetPlatform, TceLoader, TczExtension, TinyCoreBootConfig, TranslationLayer, WasmModule,
    WasmSandboxEngine, WasmState,
};
pub use config::declarative::{ConfigManager, ConfigModule, ConfigState, SystemGeneration};
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
pub use hal::advanced_hal::{DeviceCategory, HardwareDevice, SigmaDeviceManager, UdevAction, UdevCondition, UdevRule};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
};
pub use network::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
pub use network::wireless_manager::{BluetoothDevice, WifiProfile, WifiSecurity, WirelessManager};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use performance::tuned::{BootStageMetrics, PerformanceTuner, TuningProfileKind};
pub use power::advanced::{Battery, PowerManager, PowerProfileMode, ThermalZone as PowerThermalZone};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::hardening;
pub use security::{
    secure_zeroize, AuditLogEntry, CapabilityGate, CapabilityToken, ExploitPayload,
    HardenedAuditTrail, IntrusionMonitor, IntrusionSeverity, PenetrationAssistant, Permission,
    PledgeManager, PledgePromise, SecurityScanner, VulnerabilityClass, VulnerabilityReport,
};
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    AptDebManifest, BuildSystem, ContentAddressedStore, CryptoVerifier, FlatpakManifest,
    PackageRecipe, PacmanPkgbuild, RecipeError, RecipeManager, SatSolver, SnapcraftManifest,
    Transaction, UniversalPackageAdapter,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
pub use graphics::{
    PixelRgba, VideoFrame, BitmapSurface, Color, Compositor, Position, Rectangle, SimpleCompositor, SimpleWindow, Size,
    Surface, Window, VideoClip, VideoEffect, VideoTimeline, VideoTrack as GraphicsVideoTrack,
};
pub use graphics::advanced_accel::{GpuDevice as AccelGpuDevice, GraphicsBackendApi, GraphicsManager as AccelGraphicsManager, RenderPipeline};
pub use gpu::driver::{GPUDeviceID, GPUVendor};
pub use media::{
    SovereignScreenRecorder, CaptureSource, GpuEncoderType, RecorderState, RecordingStats,
    AdBlockFilter, BrowserProcess, BrowserProcessType, SearchEngineType, SearchSwitcher,
    SecureStorageContainer, SovereignBrowserEngine,
    CGroup, CGroupController, CodecType, DnsResolver, InitService, NtpClient, PageTable,
    PlayerState, SecureBootKeyring, SigmaSystemd, SovereignVideoPlayer, SovereignVmm,
    SovereignVideoEditor, VideoTrack, TimelineClip, AscCdl, EditorError,
};
pub use installer::{
    GuiInstallerWizard, InstallerStep, PartitionStrategy, UserAccountConfig, PrivacySettings,
    DetectedOperatingSystem,
};

pub mod init {
    pub mod systemd_init;
}
pub use init::systemd_init::{SystemdEngine, SystemdUnit, UnitState, UnitType};

pub mod ai {
    pub mod next_gen;
    pub mod wandr;
    pub mod developer_platform;
}
pub use ai::next_gen::{
    AIModel, AdaptiveKernelPersona, AiScheduler, AiTask, DeviceTargetType, EnergyAwareScheduler,
    EnergyGovernorMode, ModelType, MultiModelOrchestrator, PredictiveSyscallTranslator,
    WorkloadType,
};
pub use ai::wandr::{
    ResearchResult, SigmaWandrAgent, WandrDocument, WandrEvaluator, WandrResearchAgent, WandrTask,
};
pub use ai::developer_platform::{
    DeviceTarget, LocalLlmOrchestrator, ModelAllocation, PrivacyRouter,
    DefaultDenyNetworkPolicy, OpenShellAgentSandbox, ExperimentRun, MlExperimentTracker,
    AiSafetyPolicyEngine, MarketplaceModel, SignedModelMarketplace, compute_blake3_simulated,
};

pub mod virt;
pub use virt::hypervisor::{
    Guest, GuestID, GuestState, Hypervisor, HypervisorError, SimpleGuest, SimpleHypervisor,
    VirtualizationGeneration,
};
pub use virt::microvm::{
    MicroVM, MicroVMState, SandboxManager, SandboxPolicy, SimpleMicroVM, SimpleSandboxManager,
};
pub use kernel::sched::aperiodic::{
    AperiodicPriority, AperiodicScheduler, AperiodicServerKind, AperiodicTask, SchedulerMetrics,
};
