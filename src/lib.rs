#![allow(warnings)]
#![allow(clippy::all)]
extern crate alloc;
// SigmaOS Library
// Core library for SigmaOS operating system

pub mod audio {
    pub mod driver;
    pub mod editor;
}
pub use audio::driver::{
    AudioDevice, AudioDeviceID, AudioError, AudioManager, AudioMixer, AudioStream, AudioType,
    SimpleAudioDevice, SimpleAudioManager, SimpleAudioMixer, SimpleAudioStream,
};
pub use audio::editor::{
    AmplifyEffect, AudioEditor, AudioEffect, AudioTrack, EchoEffect, LowPassFilter,
    MultiTrackSession, NoiseGateEffect,
};

pub mod accessibility;
pub mod ai;
pub mod automation;
pub mod boot;
pub mod compatibility;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod distro;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod graphics;
pub mod kernel;
pub mod memory;
pub mod klib;
pub mod network;
pub mod orchestration;
pub mod package;
pub mod performance;
pub mod process;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod virtualization;
pub mod tracing;
pub mod crash;
pub mod media;
pub mod gpu;

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
pub mod toolchain {
    pub mod adapter;
    pub mod bootstrap;
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
    APITimelineManager, AkabeiBundle, AkabeiPackageEngine, AntixControlCenter,
    AntixDesktopProfiler, AntixInitManager, ApplicationBinary,
    BinaryCompatMatrix, BinaryFormat, CompatibilityError, 
    CompatibilityManager, CompatibilityMode, ContainerRuntime, 
    DesktopProfile, DesktopTheme, DiscontinuedFS, DriverBridge,
    FSRevival, GraphicsBridge, InstallerStep,
    KapudanAssistant, KernelPersona, KernelPersonaVM, LegacyBus,
    LegacyMemoryTrimmer, LegacyPluginManager, LibcVersion, 
    NetworkBridge, StorageBridge, SyscallAbi, TargetPlatform, TranslationLayer, TribeInstaller,
    WorkloadOptimizer, WorkloadProfile, GLOBAL_AKABEI,
    GLOBAL_ANTIX_CONTROL, GLOBAL_ANTIX_DESKTOP, GLOBAL_ANTIX_INIT, GLOBAL_KAPUDAN,
    GLOBAL_MEMORY_TRIMMER, GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER, GLOBAL_TRIBE,
    GLOBAL_WORKLOAD_OPTIMIZER,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use distro::{
    ArchDependencyResolver, PackageNode, FreeBSDJail, OpenBSDPledge, NixStyleStore,
    PinRule, AptPinStore, OpenRCService, ArchPacmanHooksManager, FlakeInput, GentooPortageUseFlagsEngine,
    NixOSFlakeEngine, PacmanHook, PortagePackage, RunitService, ServiceState, SystemClosure, VoidRunitSupervisor,
    AdminAction, AiSysAdmin, AppManifest, AuditResult, AuditRule, BackupSnapshot, BackupSystem,
    BountyStatus, BugBountyProgram, BugBountyReport, BuildJob, BuildStatus, CanFrame,
    CertificationStatus, CommunityConference, ConferenceTalk, ConfigHook, CrossBuildPipeline, DevTool, DeveloperToolkit, DirectoryService, DirectoryUser,
    DllLoader, DllModule, EcuController, EduChallenge, EduPlayground, ForumChannel, ForumPost,
    GdiObjectType, HardwareCertificate, HardwareCertificationProgram, HardwareProfile,
    HardwareRegressionSuite, HelpSystem, HowToGuide, HpcClusterJob, HpcJobState, ImeCandidate,
    InputMethodEngine, IntegrityState, KernelTrace, LanguagePack, LinuxSyscall, LiveDebugger,
    LivepatchManager, LivepatchPatch, LocaleManager as DistroLocaleManager, ManPage, MpiCommunicator, NetplanConfig,
    NetplanManager, P2pNode, PackageBuildService, PosixTranslation, PqcSelfHealing,
    QAStagedRelease, RegionalSettings, RegistryType, RegistryValue, ReleaseStage, RescueISO,
    RescueISOManager, SoftwareCertificationProgram, SovereignP2PSync, TargetArch,
    TimeTravelCheckpoint, TimeTravelEngine, WikiPage, Win32Gdi, WindowsRegistry,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use drivers::{
    DeviceGeneration, GpuCommand, GpuDriver, GpuError,
    InputDriver, InputEvent, InputType,
    E1000RxDesc, E1000TxDesc, IntelE1000Driver,
    LegacyAudioAc97, ModernAudioIntelHda, ModernNvmeDriver, ModernUsbPrinterDriver,
    ModernWifiDriver, TouchJingosDriver, VirtioBlkDriver, VirtioNetDriver,
    VirtioRngDriver, UnifiedDmaBroker, SelfHealingDriverManager, DmaDescriptor, DeviceCommandType,
    DeviceTransactionLog, GLOBAL_DMA_BROKER, GLOBAL_HEALING_MANAGER,
    NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    PeripheralDevice, PeripheralManager, PowerState,
    StorageCommand, StorageDriver, StorageError, StorageType,
    HidError, HidKeyboardEvent, HidReportType, UsbHidDriver,
    VesaDriver, VesaError, VesaModeInfo,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
};
pub use graphics::paint::ColorRgba;
pub use kernel::{
    ArchitectureEngine, BuddyAllocator, Channel, CpuRegisters, HardwareException,
    Irql, IpcError, IpcManager,
    LookasideList, MemoryBlock, MemoryDescriptorList, Message, Pcb, PoolType, Priority, Process,
    ProcessState, ProcessorInitState, RoundRobinConfig, RoundRobinScheduler, Scheduler,
    Tcb, ThreadState, PAGE_SIZE,
    ABIManager, BpfLsmPolicyGovernor, CompletionQueueEntry, Generation, GenerationManager,
    InterruptClass, InstructionCyclePhase, IoWaitProfile, KernelGraph, KernelIoUringEngine, KernelPlugin, KernelPluginManager,
    KernelMechanism, KernelPolicy,
    LegacyScheduler, LsmHookType, MemfdSecretGuard, MetaKernel, MicroDriver, NetPod,
    PageFolio, PageFolioCacheManager, SubmissionQueueEntry,
    Bus, PciBus, UsableBus, KRef, KernelObject, KObject,
    BsdPfStateTable, FreeBsdVfsNullfs, FutexOp, FutexWaiter, LinuxFutexEngine,
    PfFiveTuple, PfStateEntry, SovereignCgroupGovernor, CgroupResourceLimits,
    SovereignMechanism, AdaptivePolicy, PolicyMechanismCoordinator,
    PageTable, PageTableEntry, PageTableFlags, VirtualMemoryManagerV2,
    Component, ComponentTree, ComponentId, ComponentState, CapabilityHandle, CapabilityRights, ComponentError, ResourceType, ResourceAllocation,
};
pub use network::{
    TcpConnection, TcpError, TcpSegment, TcpStack, TcpState,
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
pub use performance::{
    Profiler, SimpleProfiler, Profile, SimpleProfile, ProfileType, ProfilerError, CallGraph, SimpleCallGraph,
    MultiGenLRU, PageInfo, MAX_GENERATIONS, MAX_PAGES_TRACKED,
    IoUring, IoOpcode, SQ_RING_SIZE, CQ_RING_SIZE,
    AdaptiveIOScheduler, DeviceType, IOSchedulerPolicy, IORequest,
    BbrEngine, BbrState,
    EevdfScheduler, EevdfTask, MAX_SCHED_TASKS,
    ZeroCopyQueue, IPCError, QUEUE_SIZE,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore, EverythingSearchEngine, AudacityEditor, BraveBrowserEngine, EarTrumpetAudioRouter,
    NotepadPlusWorkspace, ObsStudioMixer, OneCommanderDualPane, PotPlayerVlcEngine,
    SevenZipCompressor, ShareXFlameshotEngine,
    ContentType, Folder, InMemoryNoteStorage, Note, NoteError, NoteSearchResult, NoteStorage,
    NoteTakingApp, Notebook,
    AudioQuality, FfmpegBackend, GStreamerBackend, RecorderError, RecordingBackend,
    RecordingConfig, RecordingFormat, RecordingProgress, RecordingRegion, RecordingState,
    ScreenRecorder, VideoQuality,
    CaptureRegion, ImageFormat, MacOsBackend, ScreenshotBackend, ScreenshotConfig, ScreenshotError,
    ScreenshotMode, ScreenshotResult, ScreenshotTool, WaylandBackend, WindowsBackend, X11Backend,
    CellValue, ChartType, sigma_office::DocumentMetadata as SigmaOfficeDocumentMetadata, DocumentNode,
    DocumentType, PresentationProcessor, ShapeType, SigmaDocument, SigmaOffice, SlideElementType,
    SpreadsheetProcessor, TextProcessor, TypographyRenderer,
    InMemoryStorage, KanbanBoard, KanbanColumn, Project, Reminder, ReminderType, Subtask, Task,
    TaskError, TaskManager, TaskPriority, TaskStatus, TaskStorage,
    BashShell, ColorScheme, CommandResult, CursorStyle, IntegratedTerminal, ShellImpl, ShellType,
    SigmaShell, TerminalConfig, TerminalError, TerminalSession, ZshShell,
    LayoutPreset, SplitDirection, TmuxPane, TmuxSession, TmuxSessionManager, TmuxWindow,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError,
    SelfHealingModule, SigmaTimeshift, SystemSnapshot,
};
pub use security::hardening;
pub use security::{
    CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise,
    secure_zeroize, AuditLogEntry, HardenedAuditTrail, IntrusionMonitor, IntrusionSeverity,
    ExploitPayload, PenetrationAssistant, VulnerabilityScanner,
    PledgeNamespace, PledgePromise as SigmaPledgePromise, SyscallFilter,
    UnveilEntry, UnveilManager, UnveilPermissions, UnveilState,
    CapabilityToken as RuntimeCapabilityToken, SecurityEnforcer,
    CapabilityToken as AndroidStyleCapabilityToken,
    SecurityEnforcer as AndroidStyleSecurityEnforcer, PORT_ALLOW_SSL, PORT_ALLOW_TCP,
    ClipboardEntry, ClipboardError, ClipboardSecurity, ClipboardType, NoEncryption,
    SecureClipboardManager, clipboard::SecurityLevel as ClipboardSecurityLevel, XorEncryption,
    AnomalyDetection, DetectionResult, DetectionRule, DetectionStrategy, EventType, IdsError,
    IntrusionDetectionSystem, RuleAction, SecurityEvent, Severity, SignatureDetection,
    BiometricAuth, BiometricResult, BiometricType, FaceIdAuth, FingerprintAuth, PasswordCategory,
    PasswordEntry, PasswordError, PasswordManager, PasswordManagerResult,
    LegacySecurityType, SecurityBridge,
    SecurityFacet, SecurityPrism,
    SandboxRule, PrivacyFirstSandbox,
};
pub use shell::{ShellCommand, ShellRepl};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier,
    PackageRecipe, RecipeError, RecipeManager, SatSolver,
    Transaction, PackageFormatAdapter,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
pub use memory::{
    BsdZoneAllocator, LinuxKswapd, MemCgroupManager, SimpleVMM, Zone, MemCgroup, PageState,
};
