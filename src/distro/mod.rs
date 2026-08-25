// SigmaOS Distro/Ecosystem Maturity Module
pub mod certification;
pub mod community;
pub mod compat_layers;
pub mod debian_parity;
pub mod developer;
pub mod ecosystem_dimensions;
pub mod endeavour_os;
pub mod enterprise;
pub mod fedora_parity;
pub mod gentoo;
pub mod i18n;
pub mod improvements;
pub mod linux_bsd_inspirations;
pub mod linux_bsd_parity;
pub mod linux_ideas;
pub mod manjaro;
pub mod nextgen;
pub mod parity;
pub mod power_network_tools;
pub mod preseed;
pub mod ready_to_use;
pub mod recovery;
pub mod specialized;
pub mod stable_components;
pub mod tiny_core;
pub mod parity;
pub mod arch;
pub mod cachy;

pub use cachy::{
    MicroArchLevel, CachyKernelVariant, CpuCapabilities, BoreSchedulerGovernor, CachyPackageRepo,
};

pub use arch::{
    ArchBuildSystem, PacmanSyncManager, PacmanSyncPackage, ArchMirror, AurPackage, AurHelper,
    ArchRepoType,
};

pub use parity::{
    InstallationTarget, InstallerStep, InstallerError, LiveInstaller, SovereignInstaller,
    UpdateChannel, SystemStateStatus, UpdateError, ChannelManager, SovereignChannelManager,
    SigmaAppBundle, BundleError, AppBundleRuntime, SovereignBundleRuntime,
    CpuArchitecture, HalError, HardwareAbstractionLayer, SovereignHal,
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

pub mod wiki_ideas_implementation;
pub use wiki_ideas_implementation::{
    CpuPStateGovernor, JournalLogEntry, NumaNodeTopology, SovereignHybridSchedulerInnovations,
    SovereignSystemdParityEngine, SystemdUnit, SystemdUnitState, SystemdUnitType,
};
