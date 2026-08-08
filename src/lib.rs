#![no_std]
||||||| 43be3a7e8
// SigmaOS Library
// Core library for SigmaOS operating system
// SigmaOS Library
// Core library for SigmaOS operating system
#![allow(clippy::all, unused)]

pub mod kernel {
    pub mod scheduler;
||||||| 984d1301f
pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
||||||| 43be3a7e8
pub mod security;
pub mod sigpkg;
pub mod kernel;
pub mod network;
pub mod filesystem;
pub mod drivers;
pub mod accessibility;
pub mod automation;
pub mod community;
pub mod compatibility;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod kernel;
pub mod klib;
pub mod ml;
pub mod network;
pub mod observability;
||||||| 43be3a7e8
pub mod shell;
pub mod dashboard;
pub mod accessibility;
pub mod customization;
pub mod automation;
pub mod resilience;
pub mod productivity;
pub mod drivers;
pub mod ecosystem;
pub mod education;
pub mod filesystem;
pub mod governance;
pub mod init;
pub mod kernel;
pub mod legal;
pub mod net;
pub mod network;
pub mod logging;
pub mod orchestration;
pub mod distro;
pub mod package;
pub mod productivity;
pub mod remote;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
||||||| 43be3a7e8
pub mod compatibility;
pub mod productivity;
pub mod resilience;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod support;
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
pub mod ai {
    pub mod agent;
    pub mod orchestrator;
}
pub mod boot;
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
pub mod accessibility;
pub mod automation;
pub mod compatibility;
pub mod container;
pub mod customization;
pub mod dashboard;
pub mod device;
pub mod driver;
pub mod drivers;
pub mod filesystem;
pub mod kernel;
pub mod klib;
pub mod legal;
pub mod ml;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod distro;
pub mod package;
pub mod productivity;
pub mod thread;
pub mod process;
pub mod tools;
pub mod remote;
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
pub mod ai {
    pub mod agent;
    pub mod orchestrator;
}
pub mod boot;
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
||||||| 43be3a7e8
pub mod graphics {
    pub mod paint;
    pub mod video;
    pub mod compositor;
    pub mod render3d;
}
pub mod hardware {
    pub mod win32;
    pub mod compatibility;
}
pub mod power {
    pub mod governor;
}
pub mod observability {
    pub mod profiler;
}
pub mod ai {
    pub mod agent;
    pub mod orchestrator;
    pub mod runtime;
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
    pub mod energy_aware;
}
pub mod crypto {
    pub mod vectorized_pqc;
}

pub mod memory {
    pub mod buddy_allocator;
}

pub mod ipc {
    pub mod pipes;
}

pub mod security {
    pub mod pledge;
}

pub mod net {
    pub mod socket;
}
||||||| 984d1301f
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
pub use drivers::{
    GpuCommand, GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    StorageCommand, StorageDriver, StorageError, StorageType, UsbHidDriver, VesaDriver, VesaError,
    VesaModeInfo,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
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
};
pub use network::{
    compute_checksum as compute_net_checksum, IPv4Address, NetworkPacket, PacketRingBuffer,
    RingTcpState, TcpConnection, TcpError, TcpSegment, TcpSocket, TcpStack, TcpState,
    ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
};
pub use observability::{
    ObservabilityError, ObservabilityStack, SigmaDebug, SigmaMetrics, SigmaTrace,
    SimpleObservabilityStack,
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
pub use remote::{
    FileTransfer, InputAuthGate, PqcVideoCipher, RemoteDesktop, RemoteError, RemoteSession,
    RemoteShell, SessionID, SessionState, ShellError, ShellID, ShellManager, SigmaRendezvous,
    SimpleFileTransfer, SimpleRemoteDesktop, SimpleRemoteSession, SimpleScreenSharing,
    SimpleShellManager,
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
    CapabilityGate, CapabilityToken, DomainID, DomainOrchestrator, DomainType, IsolatedDomain,
    IsolationError, Permission, PledgeManager, PledgePromise, SecurityEnforcer as AndroidStyleSecurityEnforcer,
    PORT_ALLOW_SSL, PORT_ALLOW_TCP,
};
pub use shell::{
    CommandError as ShellCommandError, ShellCommand, ShellRepl, ShellSession, SimpleShellSession,
};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageDependencyResolver, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction, Version, MAX_RECIPE_DEPENDENCIES, PackageFormatAdapter, UniversalPackageManager, AdapterError,
    DebAdapter, RpmAdapter, PacmanAdapter,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
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
    PageAccessMode, MemoryArch, PageTableEntry, PageDirectory, DeferredProcedureCall,
    Kpcrb, Kpcr, Irql, IrqlController, IdtEntry, Idtr, SystemServiceTable,
    UmsThreadState, UmsContext, SovereignKernelInternals,
    LinuxEra, HistoricalCpuState, HistoricSyscallEmulator, Era0_11SyscallEmulator,
    Era1_0SyscallEmulator, Era2_4SyscallEmulator, VintageVirtualizationSandbox,
    VintageDriverTranslator, VintagePackageConverter, HistoricError, LfsToolchainBuilder,
    ProtectedModeSwitchSimulator, VgaTextModeDriverSimulator, PicKeyboardController,
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
pub use drivers::{
    GpuCommand, GpuDriver, GpuError, HidError, HidKeyboardEvent, HidReportType, InputDriver,
    InputEvent, InputType, NetworkCommand, NetworkDriver, NetworkError, NetworkType,
    StorageCommand, StorageDriver, StorageError, StorageType, UsbHidDriver, VesaDriver, VesaError,
    VesaModeInfo,
    LegacyAudioAc97, ModernAudioIntelHda, ModernNvmeDriver, ModernUsbPrinterDriver,
    ModernWifiDriver, TouchJingosDriver,
};
pub use filesystem::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem,
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
};
pub use network::{
    compute_checksum as compute_net_checksum, IPv4Address, NetworkPacket, PacketRingBuffer,
    RingTcpState, TcpConnection, TcpError, TcpSegment, TcpSocket, TcpStack, TcpState,
    ETHERNET_HEADER_LEN, IPV4_HEADER_LEN, TCP_HEADER_LEN, UDP_HEADER_LEN,
};
pub use observability::{
    ObservabilityError, ObservabilityStack, SigmaDebug, SigmaMetrics, SigmaTrace,
    SimpleObservabilityStack,
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
pub use remote::{
    FileTransfer, InputAuthGate, PqcVideoCipher, RemoteDesktop, RemoteError, RemoteSession,
    RemoteShell, SessionID, SessionState, ShellError, ShellID, ShellManager, SigmaRendezvous,
    SimpleFileTransfer, SimpleRemoteDesktop, SimpleRemoteSession, SimpleScreenSharing,
    SimpleShellManager,
};
pub use productivity::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
    SplitDirection as TmuxSplitDirection, LayoutPreset as TmuxLayoutPreset,
    TmuxPane, TmuxWindow, TmuxSession, TmuxSessionManager,
};
pub use legal::{
    ComplianceCert as LegalComplianceCert, ComponentLicense, LegalComplianceRegistry, LicenseType, PatentRecord,
    GlobalStandard, ComplianceStatus as LegalComplianceStatus, RegulatoryControl, InternationalComplianceTracker,
    LabourLawConfig, StatutoryPayrollBreakdown, LabourLawCompliance, StatutoryFiling,
    StatutoryFilingDashboard,
};
pub use resilience::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use security::{
    CapabilityGate, CapabilityToken, DomainID, DomainOrchestrator, DomainType, IsolatedDomain,
    IsolationError, Permission, PledgeManager, PledgePromise, SecurityEnforcer as AndroidStyleSecurityEnforcer,
    PORT_ALLOW_SSL, PORT_ALLOW_TCP,
};
pub use shell::{
    CommandError as ShellCommandError, ShellCommand, ShellRepl, ShellSession, SimpleShellSession,
};
pub use sigpkg::{
    BuildSystem, ContentAddressedStore, CryptoVerifier, PackageDependencyResolver, PackageRecipe, RecipeError, RecipeManager,
    SatSolver, Transaction, Version, MAX_RECIPE_DEPENDENCIES, PackageFormatAdapter, UniversalPackageManager, AdapterError,
    DebAdapter, RpmAdapter, PacmanAdapter,
};
pub use virtualization::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};

pub use thread::management::{
    ThreadID, ThreadState as LibThreadState, ThreadAlertableState, Thread, ThreadError, SimpleThread, ThreadManager, SimpleThreadManager,
};

pub use process::spawn::{
    ProcessID, ProcessState as LibProcessState, ProcessError, Process, SimpleProcess, ProcessSpawner, SimpleProcessSpawner, ProcessWaiter, SimpleProcessWaiter, ProcessGroup, SimpleProcessGroup,
    CLONE_NEWNS, CLONE_NEWNET, CLONE_NEWPID,
};

pub use tools::{
    AccessibilityFeature as LibAccessibilityFeature, ClusterNode as LibClusterNode, NodeState as LibNodeState,
    SigmaAccess as LibSigmaAccess, SigmaCluster as LibSigmaCluster, SigmaDeploy as LibSigmaDeploy,
    SigmaIdentity as LibSigmaIdentity, SigmaToolError as LibSigmaToolError, UserIdentity as LibUserIdentity,
    SovereignDpkgEtcher, SovereignAptDuo, SovereignImeConvertCase, SovereignTableConverter,
    SovereignWordCounter, SovereignTextFixer, SovereignImageToDataUri, SovereignKeyboardTester,
    SovereignIsWebsiteDown,
};
||||||| 43be3a7e8
pub use security::{CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise};
pub use sigpkg::{SatSolver, ContentAddressedStore, CryptoVerifier, Transaction, PackageRecipe, BuildSystem, RecipeManager, RecipeError};
pub use kernel::{Scheduler, Process, Priority, ProcessState, BuddyAllocator, MemoryBlock, PAGE_SIZE, IpcManager, Channel, Message, IpcError, RoundRobinScheduler, RoundRobinConfig, SchedulerError};
pub use network::{TcpStack, TcpConnection, TcpSegment, TcpState, TcpError};
pub use filesystem::{VirtualFilesystem, Inode, FileDescriptor, FileType, FilePermissions, FsError};
pub use drivers::{GpuDriver, GpuCommand, GpuError, StorageDriver, StorageCommand, StorageType, StorageError, NetworkDriver, NetworkCommand, NetworkType, NetworkError, InputDriver, InputEvent, InputType, UsbHidDriver, HidKeyboardEvent, HidReportType, HidError, VesaDriver, VesaModeInfo, VesaError};
pub use shell::{ShellRepl, ShellCommand};
pub use dashboard::{UnifiedDashboard, DashboardWidget, MetricData, MetricType, WidgetType, SystemMonitor};
pub use accessibility::{AccessibilityFramework, AccessibilityProfile, AccessibilitySetting, AccessibilityCategory, AccessibilityFeature, AccessibilityError};
pub use customization::{CustomizationEngine, Routine, Condition, Action, Theme, TriggerType, CustomizationError};
pub use automation::{AiOptimizer, OptimizationRecommendation, SystemState, OptimizationCategory, OptimizationError, SystemAutomationManager, SystemAutomationRule, SystemAction, SystemEventType, PerformanceProfile, SystemPrediction, PredictiveModel, AutomationError};
pub use resilience::{SelfHealingModule, RecoveryRule, RecoveryAction, SystemSnapshot, RecoveryEventType, ResilienceError};
pub use productivity::{GamifiedProductivity, Achievement, Goal, PomodoroTimer, ProductivityScore, AchievementType, PomodoroState};
pub use orchestration::{CrossDeviceOrchestrator, ConnectedDevice, SmartHomeDevice, AutomationRule as CrossDeviceAutomationRule, DeviceType as CrossDeviceType, ConnectionStatus, DeviceCapability, AutomationTrigger, CrossDeviceAction, OrchestrationError};
pub use package::{UniversalPackageManager, UnifiedPackage, PackageFormat, PackageSource, PackageAdapter, DependencyResolver, ConflictResolution, PackageError};
pub use compatibility::{CompatibilityManager, ApplicationBinary, TranslationLayer, ContainerRuntime, TargetPlatform, BinaryFormat, CompatibilityMode, CompatibilityError};
pub use virtualization::{VirtualizationOrchestrator, VirtualMachine, Container, KubernetesPod, VirtualizationTech, VmState, ResourcePool, VirtualizationError};
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
    LinuxKernelVersion, LegacyKernelAdapter, LegacyPackageAdapter, LegacySecurityAdapter, LegacyUIAdapter,
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
    LegacyFsType, LegacyFSAdapter,
    FileBlock, SigmaFS,
    SigmaFhsRouter, SigmaFhsHook, SigmaFhsNamespace, SigmaFhsAuditor,
    SigmaDisasterRecoveryCleaner,
    SigmaFsJournal, SigmaFsCow, SigmaFsVolume, SigmaFsRaid, SigmaFsCrypt, SigmaFsVirtio,
    SymlinkError, SmartSymlink,
};
pub use governance::{
    DemocraticProposal, DemocraticVoting, FoundationMember, FoundationModel, ReleaseType,
    RoadmapMilestone, TransparentRoadmap,
};
pub use kernel::{
    BuddyAllocator, Channel, IpcError, IpcManager, MemoryBlock, Message, Priority, Process,
    ProcessState, RoundRobinConfig, RoundRobinScheduler, Scheduler, SchedulerError, PAGE_SIZE,
    SovereignSelfHealingKernel, UdkfHook, UserDefinedKernelFunctions,
    SovereignKernelModuleSystem, SovereignKernelModule, ModuleState, SigmaSignal, ProcessProvenanceNode, PredictiveScheduler, AdaptiveRoot, ThreatLevel,
};
pub use legal::{
    ComplianceCert, ComponentLicense, LegalComplianceRegistry, LicenseType, PatentRecord,
};
pub use network::{
    TcpConnection, TcpError, TcpSegment, TcpStack, TcpState,
    LegacyProtocol, LegacyProtocolAdapter,
};
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
    FileIndexEntry, EverythingSearchEngine, TextTab, NotepadPlusPlusBuffer,
    BrowserContainerType, BrowserTabInstance, SovereignBrowserEngine,
    CompressionMethod, ArchiveVolume, SevenZipEngine,
    AnnotationShape, ScreenshotAnnotation, FlameshotAnnotator,
    VideoSourceLayer, ObsStudioMixer,
    AudacityWaveEditor,
    VlcCodecPipeline,
    VideoTrackClip, DaVinciTimeline,
    ItemAgeColor, OneCommanderFileGrid,
    AppVolumeChannel, EarTrumpetVolumeMatrix,
    ExifMetadata, IrfanViewEngine,
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
pub use graphics::render3d::{
    Vec3, TriangleFace, MeshModel, MaterialShader, RenderCamera, BlenderRenderEngine,
};
pub use hardware::win32::{
    Win32Error, Win32Handle, PeFormat, PeLoader, RegistryManager, Win32Message, User32MessageQueue,
};
pub use hardware::compatibility::{
    HardwareCompatibilityManager, CompatibilityReport, CompatibilityResult, HardwareDevice, CompatibilityCheck,
};
pub use power::governor::{
    GovernorMode, CPUState, SigmaGovernor,
};
pub use observability::profiler::{
    TracepointType, PerformanceMetric, SigmaProfiler,
};
pub use boot::firmware_bridge::{
    FirmwareType, FirmwareBridge,
};
pub use boot::bridge_grid::{
    BIOSBridgeGrid, UEFIBridgeGrid, CorebootBridgeGrid, FirmwareBridgeGrid,
};
pub use toolchain::adapter::{
    ToolchainProfile, ToolchainAdapter,
};
pub use toolchain::capsule::{
    CapsuleProfile, BuildCapsule,
};
pub use toolchain::codex::{
    CodexCategory, CodexEntry, BuildCodex,
};
pub use toolchain::bootstrap::{
    BootstrapStage, PortPackage, LfsBootstrapEngine,
};
pub use compatibility::persona::{
    PersonaVersion, KernelPersonaContainer, SyscallCategory, SyscallNode, SyscallGraph,
};
pub use compatibility::abi_translator::{
    CpuArchitecture, ABITranslator,
};
pub use compatibility::lattice::{
    LatticeFeature, KernelLattice, SyscallLifecycle, SyscallHistory, SyscallTracker,
};
pub use compatibility::prism::{
    PrismFacet, KernelPrism, LedgerEntry, SyscallLedgerbook,
};
pub use compatibility::canonical::{
    SigmaSubiquity, SigmaNetplan, SigmaCloudInit, SigmaMultipass, SigmaCurtin,
};
pub use scheduler::numa_scheduler::{
    NumaNode, NumaScheduler, Node as LFNode, MichaelScottQueue, TreiberStack,
};
pub use scheduler::energy_aware::{
    TaskEnergyCost, EnergyAwareScheduler,
};
pub use crypto::vectorized_pqc::{
    VectorizedPqcEngine,
};
pub use network::revival::{
    RevivalProtocol, NetRevival,
};
pub use driver::simulation::{
    SimType, PeripheralSim,
};
pub use driver::mapper::{
    MapperCategory, DriverMapper,
};
pub use driver::pods::{
    PodType, PeripheralPod,
};
pub use driver::vault::{
    VaultEntry, DriverArchiveVault,
};
pub use driver::grid::{
    GridSlotType, PeripheralArchiveGrid,
};
pub use security::bridge::{
    LegacySecurityType, SecurityBridge,
};
pub use security::prism::{
    SecurityFacet, SecurityPrism,
};
pub use security::sandbox::{
    SandboxRule, PrivacyFirstSandbox,
};
pub use ai::agent::{
    IntentType, Intent, AIError, AIAgent as CoreAIAgent, SimpleAIAgent as CoreSimpleAIAgent, AIAgentManager, SimpleAIAgentManager,
};
pub use ai::orchestrator::{
    AgentID as OrchestratorAgentID, AgentState as OrchestratorAgentState, AgentError as OrchestratorAgentError,
    AIAgent as OrchestratorAIAgent, SimpleAIAgent as OrchestratorSimpleAIAgent, AgentOrchestrator, SimpleAgentOrchestrator,
    TaskQueue, SimpleTaskQueue, AgentCommunication, SimpleAgentCommunication,
};
pub use ai::runtime::{
    ModelType, ModelProcess, IModelRuntime,
};
pub use net::dns::{
    SovereignDNSResolver, DNSResolver as CoreDNSResolver, RecordType as DNSRecordType, SimpleDNSRecord, DNSError as CoreDNSError, NssHostsOrder, ResolvConf, HostsDatabase, UnboundCache,
};
pub use net::firewall::{
    UfwEngine, UfwLoggingLevel, UfwAppProfile, UfwRule, UfwCommandResponse,
};
pub use logging::rotation::{
    SimpleLogFile, LogSeverity, LogFacility, SimpleLogRotator, SimpleLogCompressor,
};
pub use init::sigma_init::{
    SigmaInit, SimpleService, Service as InitService, ServiceState as InitServiceState, InitError, InitSystem,
};
