use std::format;
use std::string::{String, ToString};
use std::vec::Vec;
// SigmaPkg - SigmaOS Package Manager
// Zero-dependency, zero-allocation-ready, safe Rust package manager

pub mod alpine_apk_engine;
pub mod arch_compat;
pub mod arch_pacman_engine;
pub mod aur;
pub mod aur_helper;
pub mod aurweb;
pub mod client;
pub mod daemon;

pub use aurweb::{
    AurComment, AurCommentThread, AurGitRepoManager, AurGitRepository, AurPackageRecord,
    AurRpcQueryType, AurRpcResponse, AurVotingSystem, SovereignAurWebEngine,
};
pub mod debian_apt_engine;
pub mod debian_crusher;
pub mod debian_defeater;
pub mod declarative_build;
pub use declarative_build::{
    ArchLinuxReproBuildInspector, BazelBuildEngine, BazelRuleType, BazelTarget,
    DebianDiffoscopeEngine, FreeBsdPortsPackageReproducer, GentooPortageReproducibleEbuildEngine,
    NetBsdPkgsrcDeterministicBulkBuilder, NixDerivation, PackageRatingsRegistry, PackageReview,
    ReproducibleBuildDiffInspector,
};
pub mod aur_rules;
pub mod fedora_rpm_engine;
pub mod gentoo_use_flags;
pub mod importer;
pub mod linux_compat;
pub mod makepkg;
pub mod multi_distro;
pub mod nix_dsl;
pub mod nix_shell;
pub mod nixos;
pub mod package_snapshot_rollback;
pub mod pacman;
pub mod pacman_contrib;
pub mod portage;
pub mod recipe;
pub mod repository_manager;
pub mod resolver;
pub mod rolling_release;
pub mod rpm_compat;
pub mod sovereign_sigpkg;
pub mod svntogit_repro;

pub mod spec;
pub mod store;
pub mod transaction;
pub mod transaction_log;
pub mod universal_adapter;
pub mod universal_engine;
pub mod universal_oop_system;
pub use universal_oop_system::*;
pub mod verifier;
pub mod zero_alloc_resolver;

#[path = "../package/bsd_linux_package_innovations.rs"]
pub mod bsd_linux_package_innovations;
pub use bsd_linux_package_innovations::{
    AlpineApkWorldAndVirtualPkgEngine, AptBugReport, AptMarkRecord, AptMarkState, AptPinRule,
    ArchCachyosMicroarchOptimizationEngine, ArchSplitPackageHookRunnerEngine, CachedPackageFile,
    CommunityPackageBuildSource, CommunityRepoBackend, CoprAurBuildRepositoryGatewayEngine,
    DebconfPreseedEntry, DebconfQuestionType, DebianAptMarkPackageStateGovernor,
    DebianDebconfStatoverrideEngine, DebianDpkgTriggersAptListbugsGuardEngine, DnfActionKind,
    DnfActionRecord, DnfTransactionItem, DpkgStatoverrideRule, DpkgTrigger, DpkgTriggerKind,
    DragonFlyDportsHammer2SnapshotEngine, EbuildSlotRecord, FedoraDnf5AdvisoryAndDeltaRpmEngine,
    FedoraDnfHistoryRollbackJournalEngine, FlakeInputLock, FreeBsdPortsFlavoursAndVuxmlEngine,
    GentooPortageEapiSlotOperatorEngine, GentooPortageSubslotAndUseExpandEngine,
    HaikuHpkgPackageFsEngine, Hammer2PfsSnapshot, MicroarchRepoRoute, MicroarchitectureLevel,
    NetBsdPkginBinaryDatabaseEngine, NetBsdPkgsrcOptionsFrameworkEngine,
    NixFlakesDevshellResolverEngine, NixGuixCasGcProfileEngine, OpenBsdPkgAddSignifyEngine,
    OpenSuseZypperVendorStickinessEngine, PkgSummaryRecord, PkgsrcOptionSpec, PortageEapiLevel,
    PpaRepository, RestrictedPackageSpec, SlackBuildInfo, SlackPackageRecord,
    SlackwarePkgtoolSlackBuildEngine, SlotOperator, UbuntuPpaAptPinningEngine,
    XbpsRestrictedNonFreeLicenseEngine, XbpsSonameAndOrphanEngine, ZypperPackageOffer,
    ZypperRepository,
};
pub use zero_alloc_resolver::{
    PackageDependencyResolver, MAX_RECIPE_DEPENDENCIES,
};
pub use universal_adapter::{
    PackageFormatAdapter, UniversalPackageAdapter, PackagePriority,
    AptDebManifest, PacmanPkgbuildV2, SnapcraftManifest, FlatpakManifest,
    FreeBsdUclManifest, OpenBsdContentsManifest, NetBsdPkgsrcManifest,
    ZypperSpecManifest, SlackwarePkgManifest,
    RpmSpecManifest, AppImageContainer, MappedScriptletHook,
    SigmaPkgHookType, UniversalDependencyMapper, UniversalDryRunResult,
    UniversalDryRunSimulator, UniversalFormatConverter, UniversalScriptletConverter,
};
pub use sovereign_sigpkg::*;

pub use alpine_apk_engine::{AlpineCommunityRepo, ApkIndexParser, ApkPackage};
pub use arch_compat::{
    AlpmHook, AlpmHookManager, AurRecipeCompiler, MakepkgBuilder, MkinitcpioBuilder,
    PacmanDbAdapter, RollingSyncManager, SvnPackageMetadata, SvntogitMigrationEngine,
};
pub use arch_pacman_engine::{
    AURHelper, ArchBuildSystem, ArchPacmanPackage, DependencyTreeVisualizer, PacmanCacheCleaner,
    PacmanDatabase, PacnewDiffManager, PkgbuildChecksumUpdater, SafeUpdateChecker,
};
pub use aur_rules::{
    AurLintFinding, AurRuleEngine, AurSandboxPolicy, LintSeverity, MakepkgBuildResult,
    MakepkgBuildStatus, MakepkgReproduciblePipeline,
};
pub use client::{
    parse_manifest, verify_signed_metadata, Manifest, SignedMetadata, SigpkgClient, TufRole,
};
pub use daemon::{SigpkgDaemon, SyncStatus, UpdateAvailable};
pub use debian_apt_engine::{AptRepository, DebPackage};
pub use debian_defeater::{
    SovereignDeltaGenerator, SovereignMaintainerSandbox, SovereignMirrorSelector,
};
pub use fedora_rpm_engine::{
    AnityaFedoraMessagingEngine, AnityaMessageTopic, AnityaPackageMapping,
    AnityaVersionUpdateMessage, DnfRepository, FedoraAnityaReleaseMonitoringEngine,
    FedoraMirrorManager2Engine, MirrorSiteRecord, RpmPackage,
};
pub use gentoo_use_flags::{ConditionalDependency, UseFlagManager, UseProfile};
pub use importer::{
    DebPackageImporter, PackageImporter, PacmanPackageImporter, RpmPackageImporter,
};
pub use multi_distro::{
    AptPinPriority, BsdPkgDb, BsdPkgDirective, BsdPkgManifest, DnfDeltaEngine, EbuildManifestEntry,
    EbuildManifestEntryType, GentooEbuildManifestEngine, NixFlakeInput, NixFlakeLockVerifier,
    NixFlakeLockfile, PacmanAlpmHookRegistry, ParallelMirrorDownloader, PortageSlotResolver,
    SovereignMultiDistroPackageManager, StagedTransaction, TransactionRollbackHandler,
    XbpsCasExtractor,
};
pub use nix_dsl::{NixDerivationSpec, NixDslEvaluator, NixExpr};
pub use package_snapshot_rollback::{
    InstalledPackageRecord, PackageSnapshotDiff, PackageSnapshotState,
    SovereignPackageSnapshotRollbackEngine,
};
pub use pacman_contrib::{
    CheckUpdatesEngine, InstalledPackage, PacCacheResult, PacCacheTrimmer, PacDiffAction,
    PacDiffCandidate, PacDiffConfigResolver, PacListRepoFilter, PacLogAction, PacLogAuditor,
    PacLogEntry, PackageCacheEntry, PendingUpdate, SyncPackage, UpdPkgSumsGenerator,
};
pub use portage::{EbuildSpec, PortageResolver, Slot, UseFlag};
pub use recipe::{BuildSystem, PackageRecipe, RecipeError, RecipeManager};
pub use resolver::SatSolver;
pub use rpm_compat::{PackageSourceFormat, RpmPackageTranslator, SpecMetadata};
pub use spec::{
    CachyCpuDetector, CachyosPackageAdapter, CpuArchLevel, ManagerCapability, PackageCapability,
    PackageDependency, PackageError as SpecPackageError, PackageInfo,
    PackageManager as SpecPackageManager, PackageStats, PackageVersion, SimplePackage,
    SimplePackageManager, UniversalPackage, UniversalPackageType, UserDefinedPackageHook,
};
pub use store::{
    BsdPkgRepositoryMirror, ContentAddressedStore, GentooPortageUseFlagMask, NixOsHermeticCasStore,
};
pub use svntogit_repro::{
    BuildArtifact, ConvertedGitCommit, ReproducibilityAttestationReport,
    ReproducibleBuildEnvironment, ReproduciblePackageBuilder, SovereignSvnToGitMigrator,
    SvnBranchType, SvnRevisionLog,
};
pub use transaction::Transaction;
pub use verifier::CryptoVerifier;

/// Package version using SemVer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn parse(version_str: &str) -> Result<Self, ParseError> {
        let clean = version_str.split('-').next().unwrap_or(version_str);
        if !clean.chars().any(|c| c.is_ascii_digit()) {
            return Err(ParseError::InvalidFormat);
        }
        let mut parts = clean.split('.');

        let major_str = parts.next().unwrap_or("0");
        let minor_str = parts.next().unwrap_or("0");
        let patch_str = parts.next().unwrap_or("0");

        let major_clean: String = major_str.chars().filter(|c| c.is_ascii_digit()).collect();
        let minor_clean: String = minor_str.chars().filter(|c| c.is_ascii_digit()).collect();
        let patch_clean: String = patch_str.chars().filter(|c| c.is_ascii_digit()).collect();

        if major_clean.is_empty() {
            return Err(ParseError::InvalidNumber);
        }

        let major = major_clean
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber)?;
        let minor = if minor_clean.is_empty() {
            0
        } else {
            minor_clean
                .parse::<u64>()
                .map_err(|_| ParseError::InvalidNumber)?
        };
        let patch = if patch_clean.is_empty() {
            0
        } else {
            patch_clean
                .parse::<u64>()
                .map_err(|_| ParseError::InvalidNumber)?
        };

        Ok(Version::new(major, minor, patch))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidFormat,
    InvalidNumber,
}

/// Package metadata
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub dependencies: Vec<Dependency>,
    pub checksum: String,
    pub mirrors: Vec<String>,
    pub signing_keys: Vec<String>,
    pub licenses: Vec<String>,
    pub maintainers: Vec<String>,
    pub changelogs: Vec<String>,
}

impl Package {
    pub fn new(
        name: String,
        version: Version,
        description: String,
        dependencies: Vec<Dependency>,
        checksum: String,
    ) -> Self {
        Self {
            name,
            version,
            description,
            dependencies,
            checksum,
            mirrors: Vec::new(),
            signing_keys: Vec::new(),
            licenses: Vec::new(),
            maintainers: Vec::new(),
            changelogs: Vec::new(),
        }
    }
}

/// Package dependency
#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version_constraint: VersionConstraint,
}

/// Version constraint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionConstraint {
    Exact(Version),
    GreaterThan(Version),
    LessThan(Version),
    GreaterOrEqual(Version),
    LessOrEqual(Version),
    Any,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_creation() {
        let version = Version::new(1, 2, 3);
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn test_version_parsing() {
        let version = Version::parse("1.2.3").unwrap();
        assert_eq!(version, Version::new(1, 2, 3));
    }

    #[test]
    fn test_version_comparison() {
        let v1 = Version::new(1, 2, 3);
        let v2 = Version::new(1, 2, 4);
        assert!(v1 < v2);
    }
}
