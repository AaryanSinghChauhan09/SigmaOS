#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Package Module
pub mod apm;
pub mod aur_integration;
pub mod bsd_linux_package_innovations;
pub mod cache;
pub mod debian;
pub mod debian_translator;
pub mod dependency_resolver;
pub mod gentoo_opt;
pub mod linux_translation;
pub mod manager;
pub mod repository;
pub mod repository;
pub mod resolver;
pub mod sandbox;
pub mod sigma_pkg;
pub mod signing;
pub mod spac;
pub mod store;
pub mod store;
pub mod universal;
pub mod universal;
pub mod updater;

pub use bsd_linux_package_innovations::{
    AlpineApkWorldAndVirtualPkgEngine, AptPinRule, ArchSplitPackageHookRunnerEngine,
    DebconfPreseedEntry, DebconfQuestionType, DebianDebconfStatoverrideEngine,
    DpkgStatoverrideRule, FedoraDnf5AdvisoryAndDeltaRpmEngine, FlakeInputLock,
    FreeBsdPortsFlavoursAndVuxmlEngine, GentooPortageSubslotAndUseExpandEngine,
    HaikuHpkgPackageFsEngine, NixFlakesDevshellResolverEngine, NixGuixCasGcProfileEngine,
    OpenBsdPkgAddSignifyEngine, OpenSuseZypperVendorStickinessEngine, PpaRepository,
    SlackBuildInfo, SlackPackageRecord, SlackwarePkgtoolSlackBuildEngine,
    UbuntuPpaAptPinningEngine, XbpsSonameAndOrphanEngine, ZypperPackageOffer, ZypperRepository,
};
pub use debian::{
    parse_dpkg_status, parse_sources_list, AptSource, DebControl, DebPackage, DpkgStatusEntry,
};
pub use linux_translation::{
    DebPackageDriverTranslator, GenericLinuxTranslationUdf, LinuxDriverPackageTranslator,
    LinuxTranslationService, PackageTranslationUdf, PacmanPackageDriverTranslator,
    RpmPackageDriverTranslator, GLOBAL_TRANSLATION_SERVICE, GLOBAL_TRANSLATION_UDF,
};
pub use repository::{
    MirrorEntry, MirrorSyncEngine, PackagePinEngine, PackagePinRule, PackageRepository,
    PackageTransactionJournal, PinPriority, RepoError, RepositoryManager, RepositoryMetadata,
    TransactionJournalEntry,
};
pub use store::{
    SigmaSoftwareStore, SoftwareRegistryEntry, StoreApp, StoreError, GLOBAL_SOFTWARE_STORE,
};
pub use universal::{
    ConflictResolution, DependencyResolver, PackageAdapter, PackageError, PackageFormat,
    PackageSource, UnifiedPackage, UniversalPackageManager,
};
