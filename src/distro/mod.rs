// SigmaOS Distro/Ecosystem Maturity Module
pub mod certification;
pub mod community;
pub mod compat_layers;
pub mod developer;
pub mod enterprise;
pub mod i18n;
pub mod nextgen;
pub mod recovery;
pub mod specialized;

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
    TimeTravelCheckpoint, TimeTravelEngine,
};
pub use recovery::{
    BackupSnapshot, BackupSystem, KernelTrace, LiveDebugger, RescueISO, RescueISOManager,
};
pub use specialized::{
    CanFrame, DiagnosticLogTool, EcuController, EduChallenge, EduPlayground, EosUpdateNotifier,
    EosWelcomeEngine, HpcClusterJob, HpcJobState, MirrorRanker, MpiCommunicator,
    RunitServiceManager, RumpKernelShim, ServiceStatus, RunitService,
    AptCacheSimulator, DpkgMultiArch, DebianPolicyEnforcer, AptPackageManifest,
};
