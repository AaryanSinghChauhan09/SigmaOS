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
pub mod manjaro;
pub mod nextgen;
pub mod recovery;
pub mod specialized;
pub mod tiny_core;
pub mod transformation_engine;
pub mod preseed;
pub mod linux_bsd_inspirations;
pub mod ecosystem_dimensions;

pub use ecosystem_dimensions::{
    BareMetalTelemetryRing, CommunityRemixBuilder, CommunityRemixConfig,
    CryptographicBountyLedger, CryptographicMatrixVoting, IndiaStackPublicIntegration,
    MerkleTransactionalBackupEngine, PqcVulnerabilityAdvisoryStream, SigmaAppImageFormat,
    SigmaReleaseBranch, SovereignEdition, SovereignGuardTunPqc, ZenithWiFiBroker,
};
pub use linux_bsd_inspirations::{
    ArchDependencyResolver, PackageNode, FreeBSDJail, OpenBSDPledge, NixStyleStore,
    PinRule, AptPinStore, OpenRCService, SovereignDTraceEngine, DTraceProvider, DTraceAggregation,
    SovereignRaidSelfHealer, RaidLevel, ScrubResult, SovereignDeclarativeSystemEngine,
    RollbackStatus, SovereignPrivSepSandbox, PrivSepProcessRole,
};
pub use arch_parity::{PkgBuild, AurClient, SandboxedCompiler, AlpmDatabase};
pub use preseed::{SovereignPreseedParser, PreseedVariable};
pub use chakra_parity::{AkabeiBundle, AkabeiPackageEngine, KapudanAssistant, TribeInstaller, DesktopTheme, InstallerStep};
pub use manjaro::{
    GpuType, MhwdDriverConfig, ManjaroHardwareDetection,
    ManjaroKernelRelease, MhwdDkmsRebuilder,
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
    CapabilityRight, PathAccessRule, UniversalCapabilityMatrix,
    EnclaveMeasurement, SovereignAttestationEnclave,
    KernelRelinkRecord, AutonomousKernelRelinker,
    ZfsConsensusBlock, HammerZfsConsensusStore,
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
pub use bsd_linux_innovations::{
    BsdSecureNtpConstraintSync, BsdStatefulPacketFilter, DaxMemoryRegion, DragonFlyHammerFs,
    Hammer2MultiMasterPfsReplication, Hammer2Snapshot, Hammer2TxgRecord, PfRuleAction,
    PfStateEntry, PfStateSynchronizationEngine, PfSyncMessage, PfSyncMsgType, PfsClusterNode,
    RunitService, RunitServiceState, SovereignAnonScrubber, SovereignDeltaPackageSigner,
    SovereignDeltaPatch, TlsConstraint, VirtioFsZeroCopyBridge, VoidRunitManager,
};
pub use transformation_engine::{
    AccessibilityOverlayManager, AutomationRoutineController, RoutineTrigger, SmartRoutine,
    ForensicReadinessAuditor, GlobalComplianceDashboard, DeveloperToolkitConverter,
    IotDeviceMeshOrchestrator, IotMeshDevice,
};
