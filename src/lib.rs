// SigmaOS Library
// Core library for SigmaOS operating system

pub mod audio {
    pub mod driver;
    pub mod editor;
    pub mod pipewire;
}
pub use audio::driver::{
    AudioDeviceID, AudioType, AudioError, AudioDevice, SimpleAudioDevice, AudioManager,
    SimpleAudioManager, AudioMixer, SimpleAudioMixer, AudioStream, SimpleAudioStream,
};
pub use audio::editor::{
    AudioTrack, MultiTrackSession, AudioEffect, AmplifyEffect, EchoEffect, LowPassFilter, NoiseGateEffect, AudioEditor,
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
pub mod klib;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod hal;
pub mod kernel;
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
    AccessibilityProfile, AccessibilitySetting,
    VoiceID, VoiceGender, SimpleVoice, Voice, ScreenReader, SimpleScreenReader, BrailleDisplay, SimpleBrailleDisplay,
    MagnifierID, Magnifier, SimpleMagnifier, MagnifierManager, SimpleMagnifierManager, ColorFilter, SimpleColorFilter,
    KeyID, KeyType, VirtualKey, SimpleVirtualKey, OnScreenKeyboard, SimpleOnScreenKeyboard, StickyKeys, SimpleStickyKeys,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use backup::recovery::{BackupChunk, RecoveryManager, SystemSnapshot as BackupSystemSnapshot};
pub use boot::sigma_boot::{BootEntry as SigmaBootEntry, BootManager as SigmaBootManager, BootTheme};
pub use compatibility::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer, WasmState, WasmModule, WasmSandboxEngine,
    PledgePermission, PledgeUnveilSandbox, PqcSecureChannel, Literal, Clause,
    DpllSatSolver, CasObject, ContentAddressedStorage,
    TinyCoreBootConfig, TczExtension, TceLoader, FiletoolOverlay, FrugalLoader,
    MetricAggregation, OssieMetric, OssieDimension, OssieRelationship, OssieCatalog,
    SemanticRow, OssieInterpreter, OssieOntology,
    EverySearch, SysDiag, ProcessExplorerState, ProcMonitor, CreativeMatrix, ImageLayer,
    FancyZonesManager, LayoutZone, JoplinE2ee, SpreadsheetCore,
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
pub use security::{
    CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise,
    VulnerabilityClass, VulnerabilityReport, SecurityScanner, ExploitPayload,
    PenetrationAssistant, secure_zeroize, IntrusionSeverity, IntrusionMonitor, AuditLogEntry, HardenedAuditTrail,
};
pub use security::hardening;
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction, AptDebManifest, PacmanPkgbuild, SnapcraftManifest, FlatpakManifest, UniversalPackageAdapter,
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
pub use init::systemd_init::{
    SystemdEngine, SystemdUnit, UnitState, UnitType,
};

pub mod ai {
    pub mod next_gen;
    pub mod wandr;
    pub mod developer_platform;
}
pub use ai::next_gen::{
    WorkloadType, AdaptiveKernelPersona, PredictiveSyscallTranslator, DeviceTargetType,
    AiTask, AiScheduler, EnergyGovernorMode, EnergyAwareScheduler, ModelType,
    AIModel, MultiModelOrchestrator,
};
pub use ai::wandr::{
    WandrTask, WandrDocument, ResearchResult, WandrEvaluator, SigmaWandrAgent, WandrResearchAgent,
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
