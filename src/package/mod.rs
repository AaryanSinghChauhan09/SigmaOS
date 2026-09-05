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
pub mod nix_guix;
pub mod paccache;
pub mod pactree;
pub mod repository;
pub mod resolver;
pub mod sandbox;
pub mod sigma_pkg;
pub mod signing;
pub mod spac;
pub mod store;
pub mod universal;
pub mod updater;

pub use alpine_apk::{ApkPackage, ApkPackageManager, ApkRepository, ApkWorld};
pub use arch_aur::{AURPackage, BuildError, SigmaAUR, PKGBUILD};
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
pub use checkupdates::{CheckupdatesEngine, PackageUpdate};
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
pub use store::{
    SigmaSoftwareStore, SoftwareRegistryEntry, /* StoreApp, StoreError, */ // store module not available
     GLOBAL_SOFTWARE_STORE,
};
pub use universal::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
