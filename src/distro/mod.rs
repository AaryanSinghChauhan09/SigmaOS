// SigmaOS Distro/Ecosystem Maturity Module
pub mod bsd_linux_innovations;
pub mod certification;
pub mod community;
pub mod compat_layers;
pub mod developer;
pub mod enterprise;
pub mod i18n;
pub mod manjaro;
pub mod nextgen;
pub mod power_network_tools;
pub mod recovery;
pub mod specialized;
pub mod stable_components;
pub mod tiny_core;
pub mod transformation_engine;
pub mod preseed;
pub mod linux_bsd_inspirations;
pub mod endeavour_os;

pub use power_network_tools::{
    TlpPowerGovernor, PowerSource, CpuGovernorPolicy, TlpConfig,
    NmtuiNetworkManager, ConnectionType, NetworkConnectionProfile,
    FreeBsdBhyveHypervisor, BhyveVmState, BhyveVirtualMachine,
    TailscaleWireguardMesh, WireguardPeer,
};
pub use bsd_linux_innovations::{
    BsdStatefulPacketFilter, PfRuleAction, PfStateEntry,
    DragonFlyHammerFs, Hammer2Snapshot,
    VoidRunitManager, RunitServiceState, RunitService,
    SovereignAnonScrubber,
};
pub use stable_components::{
    RhelSubscriptionEntitlementManager, SubscriptionPool, EntitlementCertificate,
    DebianDpkgDbSimulator, DpkgPackageStatus, DpkgPackageRecord,
    AlpineApkOverlayEngine, ApkOverlayFile,
    SystemdCgroupGovernor, CgroupV2Limits, CgroupV2Accounting,
};
pub use linux_bsd_inspirations::{
    ArchDependencyResolver, PackageNode, FreeBSDJail, OpenBSDPledge, NixStyleStore,
    PinRule, AptPinStore, OpenRCService,
};
pub use arch_parity::{PkgBuild, AurClient, SandboxedCompiler, AlpmDatabase};
pub use endeavour_os::{
    AkmKernelManager, AurPackageSpec, CalamaresConfig, CalamaresInstaller, DesktopEnvironment,
    EosKernelFlavor, EosLogTool, EosWelcomeApp, InstallMode, PacmanMirror, PartitionType,
    ReflectorMirrorManager, WelcomeButtonTask, YayParuHelper,
};
pub use preseed::{SovereignPreseedParser, PreseedVariable};
pub use manjaro::{
    GpuType, MhwdDriverConfig, ManjaroHardwareDetection,
    ManjaroKernelRelease, AurPackage, FlatpakPackage, SnapPackage, MhwdDkmsRebuilder,
};
pub use certification::{
    AppManifest, CertificationStatus, ComponentType, HardwareCertificate,
    HardwareCertificationProgram, HardwareProfile, HardwareRegressionSuite, QAStagedRelease,
    ReleaseStage, SoftwareCertificationProgram,
};
pub use community::{
    BountyStatus, BugBountyProgram, BugBountyReport, CommunityConference, ConferenceTalk,
    ForumChannel, ForumPost, HelpSystem, HowToGuide, ManPage, WikiPage,
};
pub use compat_layers::{
    DllLoader, DllModule, GdiObjectType, LinuxSyscall, PosixTranslation, RegistryType,
    RegistryValue, Win32Gdi, WindowsRegistry,
};
pub use developer::{
    BuildJob, BuildStatus, CrossBuildPipeline, DevTool, DeveloperToolkit, PackageBuildService,
    TargetArch,
};
pub use enterprise::{
    AuditResult, AuditRule, ComplianceAuditor, ConfigHook, DirectoryService, DirectoryUser,
};
pub use i18n::{ImeCandidate, InputMethodEngine, LanguagePack, LocaleManager, RegionalSettings};
pub use manjaro::{
    GpuType, ManjaroHardwareDetection, ManjaroKernelRelease, ManjaroKernelSwitcher,
    ManjaroSettingsManager, MhwdDriverConfig, PacmanMirror, PamacPackageManager,
};
pub use nextgen::{
    AdminAction, AiSysAdmin, IntegrityState, LivepatchManager, LivepatchPatch, NetplanConfig,
    NetplanManager, P2pNode, PqcSelfHealing, SovereignP2PSync, TimeTravelCheckpoint,
    TimeTravelEngine,
};
pub use recovery::{
    BackupSnapshot, BackupSystem, KernelTrace, LiveDebugger, RescueISO, RescueISOManager,
};
pub use specialized::{
    CanFrame, EcuController, EduChallenge, EduPlayground, HpcClusterJob, HpcJobState,
    MpiCommunicator, AptCacheSimulator, DpkgMultiArch, DebianPolicyEnforcer,
    ThreeTierReleaseModel, DebianSocialContract, FreezeBasedStabilization,
};
pub use tiny_core::{
    TinyCoreRAMEngine, TinyCoreMode, TczExtensionManager, AppsAuditTool,
};
pub use transformation_engine::{
    AccessibilityOverlayManager, AutomationRoutineController, RoutineTrigger, SmartRoutine,
    ForensicReadinessAuditor, GlobalComplianceDashboard, DeveloperToolkitConverter,
    IotDeviceMeshOrchestrator, IotMeshDevice,
};
