// SigmaOS Library
// Core library for SigmaOS operating system

pub mod klib;
pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod customization;
pub mod distro;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod graphics;
pub mod kernel;
pub mod klib;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod performance;
pub mod productivity;
pub mod resilience;
pub mod resource;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod tools;
pub mod virtualization;
pub mod tracing;
pub mod crash;
pub mod media;
pub mod gpu;

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
pub use compatibility::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ComputeNode, ContainerRuntime, DistributedComputeHandoff, GstCalculator, IndiaStackError,
    InterimLispVM, JehanneError, JehanneNamespace, LispVal, MintBackupTool, MintSoftwareManager,
    MintUpdateItem, MintUpdateLevel, MintUpdateManager, MntReformLpcDriver, MockUPIService,
    MultilingualSupport, NamespaceBindEntry, NtHandle, NtObjectManager, NtObjectType, NtStatus,
    Plan9pMessage, Plan9pMsgType, PortableExecutableLoader, ReformPowerStats, RegistryHive,
    SoftwareMeta, TargetPlatform, TranslationLayer, WindowCoordinates, ZenithDisplayCompositor,
    JudicialTimelinePlanner, MsmeComplianceEngine, AyushFormularyHelper,
    PMWaniHotspotController, DigiYatraPassScanner, IrctcPnrTracker,
    Literal, SpacSatResolver,
    ApkInstalledPackage, ApkDatabaseIndex, SyslogSeverity, SyslogMessage,
    AlpineSyslogManager, BusyBoxMulticall,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use distro::{
    AkmKernelManager, CalamaresConfig, CalamaresInstaller, EosLogTool, EosWelcomeApp,
    ReflectorMirrorManager, YayParuHelper,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use drivers::{
    Bdle, Ch340Driver, DeviceGeneration, E1000Driver, GpuCommand, GpuCommandBuffer, GpuDriver,
    GpuError, GpuPipeline, GpuShader, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, IntelHdaDriver, NetworkCommand, NetworkDriver, NetworkError,
    NetworkType, NvmeCmd, NvmeCqe, NvmeDriver, PeripheralDevice, PeripheralManager, PowerState,
    RxDescriptor, ShaderStage, StorageCommand, StorageDriver, StorageError, StorageType,
    TxDescriptor, UsbHidDriver, VesaDriver, VesaError, VesaModeInfo,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, LegacyLinuxRule, LinuxPersonaRule,
    SmartSymlink, SymlinkResolverRule, VirtualFilesystem,
};
pub use graphics::paint::ColorRgba;
pub use kernel::{
    AdaptivePolicy, AdvancedAlgorithmsManager, Apc, ApcMode, ApcQueue, ArchitectureEngine,
    AuditBlock, BsdPfStateTable, BuddyAllocator, CgroupResourceLimits, Channel,
    CircularDoublyLinkedList, CpuArchitectureClass, CpuRegisters, FreeBsdVfsNullfs, FutexOp,
    FutexWaiter, HardwareException, InstructionCyclePhase as ArchInstructionCyclePhase,
    InstructionCyclePhase, InterruptClass, IoWaitProfile, IpcError, IpcManager, Irql,
    KernelMechanism, KernelPolicy, LcgRandom, LinuxFutexEngine, LookasideList, LotteryTask,
    MemoryBlock, MemoryDescriptorList, Message, Pcb, PfFiveTuple, PfStateEntry,
    PolicyMechanismCoordinator, PoolType, Priority, Process, ProcessState, ProcessorInitState,
    RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, SequencedSinglyLinkedList,
    SinglyLinkedList, SovereignCgroupGovernor, SovereignMechanism, SystemThread, Tcb, ThreadState,
    WorkItem, PAGE_SIZE,
};
pub use network::{
    FirewallAction, FirewallCommand, FirewallFilterRule, IpRoute2Command, LinkState, PingCommand,
    SocketStatsCommand, SocketStatsEntry, TcpConnection, TcpError, TcpSegment, TcpStack, TcpState,
    UfwDefaultRule, GLOBAL_FIREWALL, GLOBAL_IP_COMMAND, GLOBAL_UFW_RULE,
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
pub use performance::{
    AnanicyCppDaemon, AnanicyRule, BoreScheduler, CachyKernelManager, CallGraph,
    CpuPriorityOptimizer, GlarySmartRule, IoPriorityOptimizer, IoSchedClass, IoTaskPriority,
    PerformanceProfileRule, PhysicalPageFrame, Profile, ProfileType, Profiler, ProfilerError,
    RamDefragmenter, SimpleCallGraph, SimpleProfile, SimpleProfiler, SmartPerformanceProfile,
    SmartResourceOptimizer, UltraKernelSamepageMerger, X86v3v4OptimizationDetector,
    GLOBAL_GLARY_RULE, GLOBAL_SMART_OPTIMIZER,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore, EverythingSearchEngine, NotepadPlusPlusBuffer, SovereignBrowserEngine, SevenZipEngine,
    CompressionMethod, FlameshotAnnotator, AnnotationShape, ObsStudioMixer,
    AudacityWaveEditor, VlcCodecPipeline, DaVinciTimeline, OneCommanderFileGrid,
    ItemAgeColor, EarTrumpetVolumeMatrix, IrfanViewEngine,
};
pub use resilience::{
    FsSnapshot, RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError,
    SelfHealingModule, SigmaTimeshift, SystemSnapshot, GLOBAL_TIMESHIFT,
};
pub use security::hardening;
pub use security::{
    AnonSurfShunt, AppSandboxEngine, CapabilityGate, CapabilityToken, DefensiveAuditSystem,
    ForensicBlock, ForensicStorageFilter, MaliciousSignature, Permission, PledgeManager,
    PledgePromise, RoutingMode, SandboxPolicy, GLOBAL_ANONSURF, GLOBAL_FORENSIC, GLOBAL_SANDBOX,
    MAX_AUDIT_BLOCKS, MAX_SIGNATURES, SIGNATURE_LEN, Securelevel, LinuxCapability,
    SovereignSecurelevelManager, PamError, PamUser, PamGroup, SovereignPamManager,
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
pub use gpu::driver::{GPUDeviceID, GPUVendor};
pub use media::{
    SovereignScreenRecorder, CaptureSource, GpuEncoderType, RecorderState, RecordingStats,
    AdBlockFilter, BrowserProcess, BrowserProcessType, SearchEngineType, SearchSwitcher,
    SecureStorageContainer, SovereignBrowserEngine,
    CGroup, CGroupController, CodecType, DnsResolver, InitService, NtpClient, PageTable,
    PlayerState, SecureBootKeyring, SigmaSystemd, SovereignVideoPlayer, SovereignVmm,
    SovereignVideoEditor, VideoTrack, TimelineClip, AscCdl, EditorError,
};

// Temporarily disabled problematic modules
// pub mod accessibility;
// pub mod automation;
// pub mod container;

// #[cfg(test)]
// #[path = "compatibility/fedora.rs"]
// pub mod fedora_compat_test;
// pub mod customization;
// pub mod dashboard;
// pub mod desktop;
// pub mod device;
// pub mod driver;
// pub mod filesystem;
// pub mod ml;
// pub mod network;
// pub mod observability;
// pub mod orchestration;
// pub mod distro;
// pub mod package;
// pub mod performance;
// pub mod productivity;
// pub mod remote;
// pub mod resilience;
// pub mod shell;
// pub mod sigpkg;
// pub mod virtualization;
// pub mod graphics {
//     pub mod compositor;
//     pub mod paint;
//     pub mod video;
// }
// pub mod hardware {
//     pub mod compatibility;
//     pub mod win32;
// }
// pub mod power {
//     pub mod governor;
// }
// pub mod ai {
//     pub mod agent;
//     pub mod orchestrator;
// }
// pub mod boot;
// pub mod system;
// pub mod installer;
