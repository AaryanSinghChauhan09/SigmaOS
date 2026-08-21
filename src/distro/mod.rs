// SigmaOS Distro/Ecosystem Maturity Module
pub mod arch_parity;
pub mod bsd_linux_innovations;
pub mod certification;
pub mod chakra_parity;
pub mod community;
pub mod compat_layers;
pub mod developer;
pub mod enterprise;
pub mod i18n;
pub mod linux_bsd_inspirations;
pub mod manjaro;
pub mod nextgen;
pub mod preseed;
pub mod recovery;
pub mod specialized;
pub mod tiny_core;
pub mod transformation_engine;

pub use arch_parity::{AlpmDatabase, AurClient, PkgBuild, SandboxedCompiler};
pub use bsd_linux_innovations::{
    BsdStatefulPacketFilter, DragonFlyHammerFs, Hammer2Snapshot, PfRuleAction, PfStateEntry,
    PfsClusterNode, RunitService, RunitServiceState, SovereignAnonScrubber, VoidRunitManager,
};
pub use certification::{
    AppManifest, CertificationStatus, ComponentType, HardwareCertificate,
    HardwareCertificationProgram, HardwareProfile, HardwareRegressionSuite, QAStagedRelease,
    ReleaseStage, SoftwareCertificationProgram,
};
pub use chakra_parity::{
    AkabeiBundle, AkabeiPackageEngine, DesktopTheme, InstallerStep, KapudanAssistant, TribeInstaller,
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
pub use linux_bsd_inspirations::{
    AptPinStore, ArchDependencyResolver, AuditViolationEvent, AuditViolationType,
    BoreTaskProfile, CachyBoreScheduler, CoreTypePreference, DTraceAggregation, DTraceProvider,
    FreeBSDJail, FreeBsdRacctVnetGuard, JailGuardRecord, MossPackageSpec, MossTransaction,
    MossTransactionState, NixStyleStore, OpenBSDPledge, OpenBsdPledgeUnveilSentinel,
    OpenRCService, PackageNode, PinRule, PrivSepProcessRole, RacctResourceLimits, RollbackStatus,
    ScrubResult, SerpentMossEngine, SovereignDTraceEngine, SovereignDeclarativeSystemEngine,
    SovereignPrivSepSandbox, SovereignRaidSelfHealer, VnetStack,
};
pub use manjaro::{
    GpuType, ManjaroHardwareDetection, ManjaroKernelRelease, MhwdDkmsRebuilder, MhwdDriverConfig,
};
pub use nextgen::{
    AdminAction, AiSysAdmin, IntegrityState, LivepatchManager, LivepatchPatch, NetplanConfig,
    NetplanManager, P2pNode, PqcSelfHealing, SovereignP2PSync, TimeTravelCheckpoint,
    TimeTravelEngine,
};
pub use preseed::{PreseedVariable, SovereignPreseedParser};
pub use recovery::{
    BackupSnapshot, BackupSystem, KernelTrace, LiveDebugger, RescueISO, RescueISOManager,
};
pub use specialized::{
    AptCacheSimulator, CanFrame, DebianPolicyEnforcer, DebianSocialContract, DpkgMultiArch,
    EcuController, EduChallenge, EduPlayground, FreezeBasedStabilization, HpcClusterJob,
    HpcJobState, MpiCommunicator, ThreeTierReleaseModel,
};
pub use tiny_core::{AppsAuditTool, TczExtensionManager, TinyCoreMode, TinyCoreRAMEngine};
pub use transformation_engine::{
    AccessibilityOverlayManager, AutomationRoutineController, DeveloperToolkitConverter,
    ForensicReadinessAuditor, GlobalComplianceDashboard, IotDeviceMeshOrchestrator, IotMeshDevice,
    RoutineTrigger, SmartRoutine,
};
