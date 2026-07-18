// SigmaOS Distro/Ecosystem Maturity Module
pub mod i18n;
pub mod community;
pub mod developer;
pub mod certification;
pub mod recovery;
pub mod enterprise;
pub mod specialized;

pub use i18n::{LanguagePack, InputMethodEngine, RegionalSettings, LocaleManager, ImeCandidate};
pub use community::{ManPage, HowToGuide, WikiPage, ForumChannel, ForumPost, HelpSystem, BugBountyProgram, CommunityConference, BountyStatus, BugBountyReport, ConferenceTalk};
pub use developer::{DeveloperToolkit, PackageBuildService, CrossBuildPipeline, DevTool, BuildJob, BuildStatus, TargetArch};
pub use certification::{HardwareCertificationProgram, SoftwareCertificationProgram, QAStagedRelease, HardwareCertificate, AppManifest, CertificationStatus, ReleaseStage, ComponentType, HardwareRegressionSuite, HardwareProfile};
pub use recovery::{RescueISOManager, LiveDebugger, BackupSystem, RescueISO, KernelTrace, BackupSnapshot};
pub use enterprise::{ConfigHook, DirectoryService, ComplianceAuditor, DirectoryUser, AuditRule, AuditResult};
pub use specialized::{HpcClusterJob, MpiCommunicator, EcuController, EduPlayground, CanFrame, EduChallenge, HpcJobState};
