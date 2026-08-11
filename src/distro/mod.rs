// SigmaOS Distro/Ecosystem Maturity Module
pub mod certification;
pub mod community;
pub mod compat_layers;
pub mod developer;
pub mod enterprise;
pub mod i18n;
pub mod manjaro;
pub mod nextgen;
pub mod recovery;
pub mod specialized;
pub mod tiny_core;
pub mod transformation_engine;

// Added modules representing missing components compared to linux & bsd distros
pub mod arch_parity;
pub mod bsd_parity;
pub mod chakra_parity;
pub mod debian_parity;
pub mod fedora_parity;
pub mod gentoo;
pub mod improvements;
pub mod linux_bsd_inspirations;
pub mod linux_ideas;
pub mod parity;
pub mod preseed;

pub use manjaro::{
    GpuType, MhwdDriverConfig, ManjaroHardwareDetection,
    ManjaroKernelRelease, ManjaroKernelSwitcher,
    PacmanMirror, PamacPackageManager, ManjaroSettingsManager,
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
pub use nextgen::{
    AdminAction, AiSysAdmin, IntegrityState, P2pNode, PqcSelfHealing, SovereignP2PSync,
    TimeTravelCheckpoint, TimeTravelEngine, NetplanConfig, NetplanManager,
    LivepatchPatch, LivepatchManager,
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

// Re-export added components to integrate them cleanly into the distro ecosystem
pub use arch_parity::{PkgBuild, AurClient, SandboxedCompiler, AlpmDatabase};
pub use bsd_parity::{OpenBsdSecurity, ZfsManager, PortsManager, PfFirewall, BsdJail};
pub use chakra_parity::{
    AkabeiBundle, AkabeiPackageEngine, KapudanAssistant, TribeInstaller,
    BundleType, DesktopTheme, InstallerStep as ChakraInstallerStep,
};
pub use debian_parity::{
    DebianPackageManager, SnapPackageManager, DebianControl, UbuntuDesktopIntegration,
};
pub use fedora_parity::{
    DnfPackageManager, RpmPackage, SelinuxPolicy, SystemdService,
};
pub use gentoo::{
    UseFlag, FeatureSet, BuildSpec, CpuOptimizationDetector, SigmaBuildGraph, BuildError,
};
pub use improvements::{
    RollingReleaseChannel, RollingPackage, RollingReleaseManager, MinimalBaseInstaller, UserPackageRepository,
    BtrfsVolumeManager, BtrfsSubvolume, BtrfsCompression, SnapshotPolicy, SystemSnapshotManager, SystemSnapshot,
    DeclarativeSystemConfig, ServiceConfig, UserConfig, BootConfig, NixStyleStorePath, AtomicUpgradeEngine,
    EphemeralSessionManager, EncryptedPersistentStorage, NetworkPrivacyMode, Amnesic, RamDisk,
    PenTestToolRegistry, PenTestTool, PenTestCategory, LiveForensicsSession, CustodyEntry,
    MinimalRuntime, LibcBackend, OpenRcStyleInit, Runlevel, InitService,
    BoreSchedulerConfig, Task, BoreScheduler, OptimizedKernelProfile, GarudaBtrfsLayout, SnapperIntegration, SnapperConfig,
    ReleaseError as DistroReleaseError, SigmaDistroEngine, AptSource, DebControlFile, DebianAptPackageManager,
    HostMapping, HostResolver, SwapPageFrame, SwapSpaceManager, SnapPackageManager as ImprovementsSnapPackageManager,
    SnapPackage as ImprovementsSnapPackage, SnapConfinement, LtsReleaseManager, ZypperPackageManager, ZypperRepo, YastConfigManager, YastModule,
    SelinuxManager, SelinuxMode, SystemdServiceManager, SystemdService as ImprovementsSystemdService, ServiceStatus, SystemdTarget,
    PortagePackageManager, MintUpdateManager as ImprovementsMintUpdateManager, MintUpdate, PopShop, PopApp, PantheonFileManager,
    AppCenter, AppCenterApp, PamacManager, SolusRollingManager, BudgieDesktop, BudgieSettings, ZorinWineManager, DesktopLayoutSwitcher,
    DesktopLayout, DdeControlCenter as ImprovementsDdeControlCenter, DisplaySettings, SoundSettings, NetworkSettings,
    MxSnapshotTool, MxSnapshotToolState as ImprovementsMxSnapshot, MxPackageInstaller, LinuxMintEnhancements, MintTools, CinnamonSettings,
    PanelSettings, LinuxDistroCompatibilityEngine, ArchLinuxFeatures, FedoraFeatures, UbuntuFeatures, GentooFeatures,
    OpenSuseFeatures, RhelFeatures, ManjaroFeatures, SolusFeatures, ZorinFeatures, DeepinFeatures, MxFeatures,
    UbuntuSnapManager, OpenSuseZypper, RhelSelinuxManager, GentooPortage, ManjaroPamac, ZorinWineIntegration, DeepinDdeControl,
    PopShopIntegration, ElementaryPantheon, SolusBudgie, LinuxDistroGapCloser,
};
pub use linux_bsd_inspirations::{
    EbpfOpcode, EbpfInstruction, SovereignEbpfEngine, ArchDependencyResolver, PackageNode,
    FreeBSDJail, OpenBSDUnveil, OpenBSDPledge, NixStyleStore, PinRule, AptPinStore, DriverContext,
    RumpDriver, NetBsdRumpRouter, GentooUseFlagsManager, OpenRCService,
};
pub use parity::{
    InstallationTarget, InstallerStep as ParityInstallerStep, InstallerError, LiveInstaller, SovereignInstaller,
    UpdateChannel, SystemStateStatus, UpdateError, ChannelManager, SovereignChannelManager, SigmaAppBundle,
    BundleError, AppBundleRuntime, SovereignBundleRuntime, CpuArchitecture, HalError, HardwareAbstractionLayer, SovereignHal,
};
pub use preseed::{PreseedVariable, SovereignPreseedParser};
