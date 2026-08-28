#![no_std]
#![allow(clippy::all, unused)]

// SigmaOS Library
// Core library for SigmaOS operating system

pub mod accessibility;
pub mod ai;
pub mod audio;
pub mod automation;
pub mod compatibility;
pub mod container;
pub mod init;
pub mod customization;
pub mod desktop;
pub mod dashboard;
pub mod desktop;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod graphics;
pub mod hardware;
pub mod init;
pub mod interrupt;
pub mod kernel;
pub mod klib;
pub mod ml;
pub mod network;
pub mod orchestration;
pub mod distro;
pub mod package;
pub mod performance;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod ui;
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
pub mod ai;
pub mod arch;
pub mod boot;
pub mod toolchain;
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
    AIAgent, AIAgentManager, AIError, AIStats, AgentCapability, AgentInfo, Intent, IntentType,
    ManagerCapability as AiManagerCapability, Pattern, SimpleAIAgent, SimpleAIAgentManager,
    SovereignWikiEngine, WikiArticle,
};
pub use audio::{
    AudioClip, AudioMasteringEffect, AudioTrack, PodcastEpisode, PodcastFeed, PodcastRecorder,
};
pub use automation::{
    AiOptimizer, AutomationError, OptimizationCategory, OptimizationError,
    OptimizationRecommendation, PerformanceProfile, PredictiveModel, SystemAction,
    SystemAutomationManager, SystemAutomationRule, SystemEventType, SystemPrediction, SystemState,
};
pub use compatibility::{
    ApplicationBinary, BIOSGatewayMesh, BinaryFormat, BuildCodexGrid, CompatibilityError,
    CompatibilityManager, CompatibilityMode, ConstellationNode, ContainerRuntime, CorebootGatewayMesh,
    DACConstellation, DotMatrixMesh, DriverArchiveGridV2, FhsConventionStatus, FileAlmanacHub,
    FirmwareGatewayMesh, FloppyMesh, GraphicsArchiveGridV2, KernelConstellationGrid,
    LegacyAsmCodexGrid, LegacyCCodexGrid, LegacyCppCodexGrid, LegacyDriverAdapter, LegacyFSAdapter,
    LegacyKernelAdapter, LegacyPackageAdapter, LegacyProtocolAdapter, LegacySecurityAdapter,
    LegacyUIAdapter, LsbProfile, NetworkAlmanacHub, NetworkArchiveGridV2, PeripheralArchiveMesh,
    PosixComplianceLevel, ProcessAlmanacHub, SELinuxConstellation, SecurityConstellation,
    StandardsComplianceManager, StorageArchiveGridV2, SyscallAlmanacHub, TapeMesh, TargetPlatform,
    TranslationLayer, UEFIGatewayMesh, ZeroTrustConstellation,
    EosMirrorReflector, EosWelcomeEngine, EosUpdateNotifier, EosLogTool, YayAurHelper,
    Mirror as EosMirror, WelcomeTab as EosWelcomeTab,
    SigmaSubiquity, SigmaNetplan, SigmaCloudInit, SigmaMultipass, SigmaCurtin,
    DnfPackageResolver, MockChrootBuilder, KojiBuildServer, BodhiUpdateTriage,
};
pub use container::{
    ContainerCapability, ContainerError, ContainerID, ContainerInfo,
    ContainerRuntime as CoreContainerRuntime, ContainerState, RuntimeCapability, RuntimeStats,
    SimpleContainer, SimpleContainerRuntime,
};
pub use customization::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme, TriggerType,
};
pub use dashboard::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use desktop::{
    ShellIntegration, SimpleShellIntegration, SimpleTerminal, SimpleTerminalManager, Terminal,
    TerminalError, TerminalID, TerminalManager,
};
pub use driver::windows_compat::{
    ImageDosHeader, ImageFileHeader, ImageOptionalHeader64, ImageSectionHeader, MajorFunction,
    PeDriverLoader, WddmMiniportDriver, WdfIoQueueDispatchType, WdfQueueContext,
    WindowsDriverAdapter, WindowsNdisAdapter, WindowsStorportAdapter, WindowsWddmAdapter,
    DEVICE_OBJECT, DRIVER_OBJECT, DXGKARG_ADDDEVICE, DXGKARG_STARTDEVICE, DXGKRNL_INTERFACE,
    DXGK_DEVICE_INFO, DXGK_DISPLAY_INFORMATION, HW_INITIALIZATION_DATA, IRP, KSPIN_LOCK,
    NDIS_HANDLE, NDIS_MINIPORT_DRIVER_CHARACTERISTICS, NDIS_OID_REQUEST, NDIS_PORT_NUMBER,
    NDIS_STATUS, NET_BUFFER_LIST, NTSTATUS, PORT_CONFIGURATION_INFORMATION, SCSI_REQUEST_BLOCK,
    STATUS_BUFFER_TOO_SMALL, STATUS_INVALID_PARAMETER, STATUS_NOT_IMPLEMENTED, STATUS_PENDING,
    STATUS_SUCCESS, STATUS_UNSUCCESSFUL, WDFDEVICE, WDFDEVICE_INIT, WDFDRIVER, WDFQUEUE,
    WDFREQUEST, WDF_DRIVER_CONFIG, WDF_IO_QUEUE_CONFIG,
};
pub use drivers::{
    GpuCommand, GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    StorageCommand, StorageDriver, StorageError, StorageType, UsbHidDriver, VesaDriver, VesaError,
    VesaModeInfo,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
    ClusterState as DefragClusterState, FragmentedFile, DefragStats, DiskDefragmenter,
};
pub use kernel::{
    ABIManager, AiNativeRuntime, BuddyAllocator, Channel, EnergyAwareScheduler, FastPathIpc,
    Generation, GenerationManager, InterruptMechanism, IpcError, IpcManager, KernelGraph, KernelPersona, KernelPlugin,
    KernelPluginManager, LegacyScheduler, MemoryBlock, Message, MetaKernel, MicroDriver, NetPod,
    PAGE_SIZE, PolicyError, PolicyManager, PrivacyFirstSandbox, Priority, Process, ProcessState,
    ProtectionDomain, PrivilegeLevel, ResourceBroker, RoundRobinConfig, RoundRobinScheduler,
    Scheduler, SchedulerError, SelfHealingKernel, SigmaFsPlusPlus, UniversalAbiTranslator,
    UserDefinedKernelFunctions, GapError, Pml4PageTableEntry, VirtualMemoryPagingManager,
    IrqRoutingTable, AcpiInterruptManager, JournalState, JournalBlock, MetadataJournal,
    PciBusScanner, PciClass, PciDevice, PCI_MAX_BUS, PCI_MAX_DEVICE,
    SignalDispatcher, SovereignSignal,
    PagingController, SimplePageTableEntry, PAGE_SIZE_BYTES, MAX_PHYSICAL_FRAMES,
    SovereignIpcBus, IpcTransactionMessage, MAX_IPC_MESSAGE_SIZE, IPC_QUEUE_CAPACITY,
};
pub use network::{
    compute_checksum as compute_net_checksum, IPv4Address, NetworkPacket, PacketRingBuffer,
    RingTcpState, TcpConnection, TcpError, TcpSegment, TcpSocket, TcpStack, TcpState,
    ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
};
pub use observability::{
    ObservabilityError, ObservabilityStack, SigmaDebug, SigmaMetrics, SigmaTrace,
    SimpleObservabilityStack,
    CognitiveOSNarrator, AdaptiveComplianceGater, SynestheticFeedbackEngine, GenerativeConfigParser, InterplanetaryDtnRoute, CollectiveSimulationNode,
};
pub use distro::{
    AppManifest, CertificationStatus, ComponentType, HardwareCertificate,
    HardwareCertificationProgram, HardwareProfile, HardwareRegressionSuite, QAStagedRelease,
    ReleaseStage, SoftwareCertificationProgram,
    BountyStatus, BugBountyProgram, BugBountyReport, CommunityConference, ConferenceTalk,
    ForumChannel, ForumPost, HelpSystem, HowToGuide, ManPage, WikiPage,
    DllLoader, DllModule, GdiObjectType, LinuxSyscall, PosixTranslation, RegistryType,
    RegistryValue, Win32Gdi, WindowsRegistry,
    BuildJob, BuildStatus, CrossBuildPipeline, DevTool, DeveloperToolkit, PackageBuildService,
    TargetArch,
    AuditResult, AuditRule, ComplianceAuditor, ConfigHook, DirectoryService, DirectoryUser,
    ImeCandidate, InputMethodEngine, LanguagePack, LocaleManager, RegionalSettings,
    AdminAction, AiSysAdmin, IntegrityState, P2pNode, PqcSelfHealing, SovereignP2PSync,
    TimeTravelCheckpoint, TimeTravelEngine, NetplanConfig, NetplanManager,
    LivepatchPatch, LivepatchManager,
    BackupSnapshot, BackupSystem, KernelTrace, LiveDebugger, RescueISO, RescueISOManager,
    CanFrame, EcuController, EduChallenge, EduPlayground, HpcClusterJob, HpcJobState,
    MpiCommunicator,
};
pub use network::{TcpConnection, TcpError, TcpSegment, TcpStack, TcpState};
pub use orchestration::{
    AutomationRule as CrossDeviceAutomationRule, AutomationTrigger, ConnectedDevice,
    ConnectionStatus, CrossDeviceAction, CrossDeviceOrchestrator, DeviceCapability,
    DeviceType as CrossDeviceType, OrchestrationError, SmartHomeDevice,
};
pub use package::{
    ConflictResolution, DependencyResolver, PackageFormatAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
pub use plugin::{
    ManagerCapability, Plugin, PluginCapability, PluginError, PluginID, PluginInfo, PluginManager,
    PluginState, PluginStats, SimplePlugin, SimplePluginManager,
};
pub use process::{
    Process as UserspaceProcess, ProcessError as UserspaceProcessError, ProcessGroup,
    ProcessID as UserspaceProcessID, ProcessSpawner, ProcessState as UserspaceProcessState,
    ProcessWaiter, SimpleProcess, SimpleProcessGroup, SimpleProcessSpawner, SimpleProcessWaiter,
    SIGINT, SIGKILL, SIGTERM, SIGUSR1,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
    SplitDirection as TmuxSplitDirection, LayoutPreset as TmuxLayoutPreset,
    TmuxPane, TmuxWindow, TmuxSession, TmuxSessionManager,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{
    CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise,
    AnonymityMode, AnonsurfEngine, RecoveredFile, ForensicsAuditTool, SniffedPacket,
    KaliSniffer, PentestAssistant, SecureWipeTool, IntrusionSeverity, IntrusionAlert, SigmaIDS,
    CapabilitySandboxEnforcer, SandboxCapabilityToken, PORT_ALLOW_TCP, PORT_ALLOW_SSL,
};
pub use init::{
    Runlevel, ServiceState as InitServiceState, InitError, Service as InitService, SimpleService as InitSimpleService,
    InitSystem, SigmaInit, DependencyResolver as InitDependencyResolver, SimpleDependencyResolver, ServiceMonitor, SimpleServiceMonitor,
    FirmwarePort, BIOSPort, UEFIPort, CorebootPort, SecurityPort, DACPort, SELinuxPort, ZeroTrustPort,
};
pub use shell::{ShellCommand, ShellRepl};
pub use desktop::{
    Notification, SimpleNotification, NotificationManager, SimpleNotificationManager, DoNotDisturb, SimpleDoNotDisturb,
};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction,
};
pub use ui::{
    LayoutCapability, LayoutStats, PlotFunction, SimpleUILayout, SimpleWidget,
    SovereignMathPlotter, UIError, UILayout, Widget, WidgetCapability, WidgetID, WidgetInfo,
    WidgetState,
};
pub use virtualization::{
    Container as VirtualContainer, KubernetesPod, ResourcePool, VirtualMachine,
    VirtualizationError, VirtualizationOrchestrator, VirtualizationTech, VmState,
};
pub use toolchain::self_host::{ToolchainError, CompilerConfig, SelfHostingManager};
pub use arch::cpu_sys::{
    SegmentType as CpuSegmentType, GdtDescriptor as CpuGdtDescriptor, IdtGate as CpuIdtGate,
    VirtualMemoryRegion as CpuVirtualMemoryRegion, ProcessorInitSuite as CpuProcessorInitSuite,
    FastSyscallDispatcher as CpuFastSyscallDispatcher,
};
