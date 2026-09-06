#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
pub mod gui_wizard;

pub use gui_wizard::{
    DetectedOperatingSystem, GuiInstallerWizard, InstallerStep, PartitionStrategy, PrivacySettings,
    UserAccountConfig,
};
