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

pub mod atomic;
pub mod delta;
pub mod distro_update_parity;

pub use distro_update_parity::{
    ArchRollingReleaseUpdater, DebianUnattendedUpgradesEngine, FreeBsdPatchEntry,
    FreeBsdUpdateEngine, OstreeAbPartitionUpdater, OstreeDeploymentState, PacnewMergeConflict,
    PartitionSlot, PostQuantumSignedUpdateVerifier, UnattendedUpgradeRule,
};
