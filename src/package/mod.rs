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
pub mod signing;
pub mod sovereign_distro_package_matrix;
pub mod spac;
pub mod store;
pub mod universal;
pub mod updater;
pub use sovereign_distro_package_matrix::*;
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
pub mod sovereign_distro_package_advancements_v20;
pub use sovereign_distro_package_advancements_v20::*;
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
pub mod sovereign_universal_multi_distro_pm_gateway;
pub use sovereign_universal_multi_distro_pm_gateway::*;
pub mod omarchy_pr_proposal_engine;
pub use omarchy_pr_proposal_engine::*;

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
    DnfError, DnfPackage, Repository, SigmaDNF, Transaction, TransactionOperation,
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
pub use repository_config::{RepoConfig, RepositoryConfigManager};
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
