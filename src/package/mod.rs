#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Package Module
pub mod alpine_apk;
pub mod alpm_engine;
pub mod apm;
pub mod arch_aur;
pub mod aur_integration;
pub mod bsd_linux_package_innovations;
pub mod cache;
pub mod checkupdates;
pub mod cleanup;
pub mod debian;
pub mod debian_apt;
pub mod debian_translator;
pub mod dependency_graph;
pub mod dependency_resolver;
pub mod fedora_dnf;
pub mod gentoo_opt;
pub mod gentoo_portage;
pub mod hardening;
pub mod linux_translation;
pub mod manager;
pub mod mint_package;
pub mod nix_guix;
pub mod paccache;
pub mod pactree;
pub mod repository;
pub mod repository_config;
pub mod resolver;
pub mod sandbox;
pub mod sigma_pkg;
pub use sigma_pkg::{
    ForeignRepoIndexParser, PkgConfig, SandboxActionType, SandboxedScriptletResult, SigmaPkg,
    SovereignUniversalPackageVerifier, UniversalDistroPackageOrchestrator, UniversalPackageFormat,
    UniversalPackageImporter, UniversalPackageTriggerEngine, UniversalPackageTriggerType,
    UniversalPackageVerificationResult,
};
pub mod sigmarepo;
pub mod signing;
pub mod sovereign_distro_package_matrix;
pub mod sovereign_update_manager;
pub mod spac;
pub mod store;
pub mod universal;
pub mod updater;
pub mod zsync_delta;
// Note: PackageUpdate and SystemGeneration are exported without glob to avoid name conflicts
pub use sovereign_update_manager::{
    AptBackend, DeltaPatch, DeltaUpdateEngine, InstallResult, PackageBackend, PacmanBackend,
    PkgBackend, SafetyLevel, SovereignUpdateManager, UpdateCategory, UpdateCheckResult,
    UpdatePolicy, UpdateRepository,
};

pub use sigmarepo::{
    add_repository, get_sigmarepo, init_sigmarepo, init_sigmarepo_default, search_package,
};
pub use sigmarepo::{
    PackageDependency, PackageMetadata, RepoConfig, RepoPriority, RepoType, Repository, SigmaRepo,
};
pub use sovereign_distro_package_matrix::*;
pub use zsync_delta::{DeltaOp, RollingAdler32, ZsyncBlockSignature, ZsyncDeltaEngine};
pub mod sovereign_distro_package_advancements_v5;
pub use sovereign_distro_package_advancements_v5::*;
pub mod sovereign_distro_package_advancements_v6;
pub use sovereign_distro_package_advancements_v6::*;
pub mod sovereign_package_apc_engine;
pub use sovereign_package_apc_engine::*;
pub mod sovereign_package_access_engine;
pub use sovereign_package_access_engine::*;
pub mod universal_package_innovations_suite;
pub use universal_package_innovations_suite::*;
pub mod sovereign_distro_package_advancements_v3;
pub use sovereign_distro_package_advancements_v3::*;
pub mod sovereign_package_smp_engine;
pub use sovereign_package_smp_engine::*;
pub mod sovereign_distro_package_advancements_v4;
pub use sovereign_distro_package_advancements_v4::*;
pub mod pull_request_workflow;
pub use pull_request_workflow::*;
pub mod sovereign_pr_package_gateway;
pub use sovereign_pr_package_gateway::*;
pub mod sovereign_universal_pm_pr_bridge;
pub use sovereign_universal_pm_pr_bridge::*;
pub mod sovereign_universal_package_format_master;
pub use sovereign_universal_package_format_master::*;
pub mod sovereign_distro_package_advancements_v7;
pub use sovereign_distro_package_advancements_v7::*;
pub mod sovereign_distro_package_advancements_v8;
pub use sovereign_distro_package_advancements_v8::*;
pub mod sovereign_distro_package_advancements_v9;
pub use sovereign_distro_package_advancements_v9::*;
pub mod sovereign_distro_package_advancements_v10;
pub use sovereign_distro_package_advancements_v10::*;
pub mod sovereign_distro_package_advancements_v11;
pub use sovereign_distro_package_advancements_v11::*;
pub mod sovereign_distro_package_advancements_v12;
pub use sovereign_distro_package_advancements_v12::*;
pub mod sovereign_distro_package_advancements_v13;
pub use sovereign_distro_package_advancements_v13::*;
pub mod sovereign_distro_package_advancements_v14;
pub use sovereign_distro_package_advancements_v14::*;
pub mod sovereign_distro_package_advancements_v15;
pub use sovereign_distro_package_advancements_v15::*;
pub mod sovereign_distro_package_advancements_v16;
pub use sovereign_distro_package_advancements_v16::*;
pub mod sovereign_distro_package_advancements_v17;
pub use sovereign_distro_package_advancements_v17::*;
pub mod sovereign_distro_package_advancements_v18;
pub use sovereign_distro_package_advancements_v18::*;
pub mod sovereign_distro_package_advancements_v19;
pub use sovereign_distro_package_advancements_v19::*;
pub mod sovereign_distro_package_advancements_v37;
pub use sovereign_distro_package_advancements_v37::*;
pub mod sovereign_distro_package_advancements_v20;
pub use sovereign_distro_package_advancements_v20::*;
pub mod sovereign_distro_package_advancements_v47;
pub use sovereign_distro_package_advancements_v47::*;
pub mod sovereign_distro_package_advancements_v22;
pub use sovereign_distro_package_advancements_v22::*;
pub mod sovereign_distro_package_advancements_v25;
pub use sovereign_distro_package_advancements_v25::*;
pub mod sovereign_distro_package_advancements_v26;
pub use sovereign_distro_package_advancements_v26::*;
pub mod sovereign_distro_package_advancements_v27;
pub use sovereign_distro_package_advancements_v27::*;
pub mod sovereign_distro_package_advancements_v28;
pub use sovereign_distro_package_advancements_v28::*;
pub mod sovereign_distro_package_advancements_v29;
pub use sovereign_distro_package_advancements_v29::*;
pub mod sovereign_distro_package_advancements_v30;
pub use sovereign_distro_package_advancements_v30::*;
pub mod sovereign_distro_package_advancements_v31;
pub use sovereign_distro_package_advancements_v31::*;
pub mod sovereign_distro_package_advancements_v32;
pub use sovereign_distro_package_advancements_v32::*;
pub mod sovereign_distro_package_advancements_v33;
pub use sovereign_distro_package_advancements_v33::*;
pub mod sovereign_distro_package_advancements_v34;
pub use sovereign_distro_package_advancements_v34::*;
pub mod sovereign_distro_package_advancements_v35;
pub use sovereign_distro_package_advancements_v35::*;
pub mod sovereign_distro_package_advancements_v36;
pub use sovereign_distro_package_advancements_v36::*;
pub mod sovereign_distro_package_advancements_v40;
pub use sovereign_distro_package_advancements_v40::*;
pub mod sovereign_distro_package_advancements_v41;
pub use sovereign_distro_package_advancements_v41::*;
pub mod sovereign_distro_package_advancements_v42;
pub use sovereign_distro_package_advancements_v42::*;
pub mod sovereign_distro_package_advancements_v48;
pub use sovereign_distro_package_advancements_v48::*;
pub mod sovereign_distro_package_advancements_v50;
pub use sovereign_distro_package_advancements_v50::*;
pub mod sovereign_distro_package_advancements_v52;
pub use sovereign_distro_package_advancements_v52::*;
pub mod sovereign_distro_package_advancements_v61;
pub use sovereign_distro_package_advancements_v61::*;
pub mod sovereign_distro_package_advancements_v62;
pub use sovereign_distro_package_advancements_v62::*;
pub mod sovereign_distro_package_advancements_v63;
pub mod sovereign_distro_package_advancements_v64;
pub mod sovereign_distro_package_advancements_v65;
pub mod sovereign_distro_package_advancements_v65_pr;
pub mod sovereign_distro_package_advancements_v66;
pub mod sovereign_distro_package_advancements_v67;
pub mod sovereign_distro_package_advancements_v68;
pub mod sovereign_distro_package_advancements_v68_pr;
pub mod sovereign_distro_package_advancements_v69;
pub mod sovereign_distro_package_advancements_v69_pr;
pub mod sovereign_distro_package_advancements_v70;
pub mod sovereign_distro_package_advancements_v70_pr;
pub mod sovereign_distro_package_advancements_v73;
pub mod sovereign_distro_package_advancements_v73_pr;
pub mod sovereign_distro_package_advancements_v74;
pub mod sovereign_distro_package_advancements_v74_pr;
pub use sovereign_distro_package_advancements_v63::*;
pub use sovereign_distro_package_advancements_v64::*;

pub use sovereign_distro_package_advancements_v65::*;
pub use sovereign_distro_package_advancements_v65_pr::*;
pub use sovereign_distro_package_advancements_v66::*;
pub use sovereign_distro_package_advancements_v67::*;
pub use sovereign_distro_package_advancements_v68::*;
pub use sovereign_distro_package_advancements_v68_pr::*;
pub use sovereign_distro_package_advancements_v69::*;
pub use sovereign_distro_package_advancements_v69_pr::*;
pub use sovereign_distro_package_advancements_v70::*;
pub use sovereign_distro_package_advancements_v70_pr::*;
pub use sovereign_distro_package_advancements_v73::*;
pub use sovereign_distro_package_advancements_v73_pr::*;
pub use sovereign_distro_package_advancements_v74::*;
pub use sovereign_distro_package_advancements_v74_pr::*;
pub mod sovereign_universal_multi_distro_pm_gateway;
pub use sovereign_universal_multi_distro_pm_gateway::*;
pub mod aptkit;
pub mod captain;
pub mod declarative_app;
pub mod dpll_solver;
pub mod linux_bsd_package_advancements;
pub mod omarchy_pr_proposal_engine;
pub mod packagekit;
pub mod software_manager;
pub mod sovereign_distro_package_advancements_v2;
pub mod sovereign_distro_package_innovations;
pub mod sovereign_distro_package_master_suite;
pub mod sovereign_distro_package_matrix_expansion;
pub mod transaction_journal;
pub mod universal_pr_execution_engine;

pub use crate::sigpkg::{
    SovereignUniversalPackageManagerInteropEngine, SovereignUniversalPackageTranslationBridge,
};
pub use alpine_apk::{ApkPackage, ApkPackageManager, ApkRepository, ApkWorld};
pub use arch_aur::{AURPackage, BuildError, SigmaAUR, PKGBUILD};
pub use bsd_linux_package_innovations::{
    AlpineApkWorldAndVirtualPkgEngine, ApkIndexMetadata, ApkSignatureKey, ApkV3SignatureEngine,
    AptBugReport, AptMarkRecord, AptMarkState, AptPinRule, ArchCachyOsMicroarchBuildProfileEngine,
    ArchCachyosMicroarchOptimizationEngine, ArchSplitPackageHookRunnerEngine, CachedPackageFile,
    CasStorePath, CommunityPackageBuildSource, CommunityRepoBackend,
    CoprAurBuildRepositoryGatewayEngine, DebconfPreseedEntry, DebconfQuestionType,
    DebianAptMarkPackageStateGovernor, DebianDebconfStatoverrideEngine,
    DebianDpkgTriggersAptListbugsGuardEngine, DeltaRpmSpec, DnfActionKind, DnfActionRecord,
    DnfTransactionItem, DpkgDivertEngine, DpkgDivertRule, DpkgStatoverrideRule, DpkgTrigger,
    DpkgTriggerKind, DragonFlyDportsHammer2SnapshotEngine, EbuildSlotRecord,
    FedoraDnf5AdvisoryAndDeltaRpmEngine, FedoraDnf5AdvisorySecurityEngine,
    FedoraDnfHistoryRollbackJournalEngine, FlakeInputLock, FreeBsdPkgAuditEngine,
    FreeBsdPortsFlavoursAndVuxmlEngine, GentooPortageEapiSlotOperatorEngine,
    GentooPortageSubslotAndUseExpandEngine, HaikuHpkgPackageFsEngine, Hammer2PfsSnapshot,
    MicroarchCompilerFlags, MicroarchRepoRoute, MicroarchitectureLevel,
    NetBsdPkginBinaryDatabaseEngine, NetBsdPkgsrcOptionsFrameworkEngine, NixCasStoreGcGovernor,
    NixFlakesDevshellResolverEngine, NixGuixCasGcProfileEngine, OpenBsdPkgAddSignifyEngine,
    OpenBsdSignifyBinaryIntegrityEngine, OpenSuseZypperVendorStickinessEngine,
    PackageBuildAttestation, PackageBuildEnvironment, PacmanGpgKey, PacmanKeyTrust,
    PacmanKeyringEngine, PkgAuditAdvisory, PkgSummaryRecord, PkgsrcOptionSpec, PortageEapiLevel,
    PortageEnvProfile, PortagePackageEnvEngine, PpaRepository, RestrictedPackageSpec,
    RpmDeltaReconstitutionEngine, SecurityAdvisoryDetail, SignifyPqcSignatureHeader,
    SlackBuildInfo, SlackPackageRecord, SlackwarePkgtoolSlackBuildEngine, SlotOperator,
    SovereignPackageBuildProvenanceEngine, UbuntuPpaAptPinningEngine, XbpsCachedPkg,
    XbpsDowngradeRepoEngine, XbpsRestrictedNonFreeLicenseEngine, XbpsSonameAndOrphanEngine,
    ZypperPackageOffer, ZypperRepository,
};
pub use checkupdates::{CheckupdatesEngine, PackageUpdate};
pub use cleanup::{
    CachedPackage, CleanupOperation, CleanupResult, OldPackageVersion, OrphanPackage,
    PackageCleanupManager,
};
pub use debian::{
    parse_dpkg_status, parse_sources_list, AptSource, DebControl, DebPackage, DpkgStatusEntry,
};
pub use debian_apt::{AptDatabase, AptError, AptPackage, SigmaAPT, SourcesEntry};
pub use dependency_graph::{
    DependencyConstraint, DependencyGraph, PackageNode, PackageVersion, VersionConstraint,
};
pub use fedora_dnf::{
    DnfError, DnfPackage, Repository as DnfRepository, SigmaDNF, Transaction, TransactionOperation,
};
pub use gentoo_portage::{
    Ebuild, PackageDatabase, PortageError, PortageTree, ProfileManager, SigmaPortage, UseFlag,
    UseFlagManager, UseFlagType,
};
pub use hardening::{
    PackageSecurityMetadata, PackageSignature, PackageSignatureType, PackageSigningEngine,
    PackageVerificationResult,
};
pub use linux_translation::{
    DebPackageDriverTranslator, GenericLinuxTranslationUdf, LinuxDriverPackageTranslator,
    LinuxTranslationService, PackageTranslationUdf, PacmanPackageDriverTranslator,
    RpmPackageDriverTranslator, GLOBAL_TRANSLATION_SERVICE, GLOBAL_TRANSLATION_UDF,
};
pub use mint_package::{
    MintInstallManager, MintMirrorManager, MintPackageMetadata, MintPackageSource,
    MintRepositoryMirror, MintSnapshotConfig, MintUpdateLevel, MintUpdateManager,
};
pub use nix_guix::{
    Derivation, EnvironmentScrubber, NixPackageManager, StorePath, SystemGeneration,
};
pub use paccache::{PaccacheConfig, PaccacheEngine, PackageCacheEntry};
pub use pactree::{DependencyNode, PactreeEngine};
pub use repository::{
    MirrorEntry, MirrorSyncEngine, PackagePinEngine, PackagePinRule, PackageRepository,
    PackageTransactionJournal, PinPriority, RepoError, RepositoryManager, RepositoryMetadata,
    TransactionJournalEntry,
};
pub use repository_config::{RepoConfig as RepoManagerConfig, RepositoryConfigManager};
pub use store::{
    SigmaSoftwareStore,
    SoftwareRegistryEntry, /* StoreApp, StoreError, */
    // store module not available
    GLOBAL_SOFTWARE_STORE,
};
pub use universal::{
    AptDebManifest, ConflictResolution, DependencyResolver, FreeBsdVuXmlPoudriereAuditAdapter,
    HomebrewBottleMacPortsAdapter, PackageAdapter, PackageError, PackageFormat, PackagePriority,
    PackageSource, SovereignUniversalDistroPackageMasterGateway, UnifiedPackage,
    UniversalPackageManager, ZypperYastRpmDeltaPackageAdapter,
};
