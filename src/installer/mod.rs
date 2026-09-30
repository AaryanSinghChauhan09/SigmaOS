pub mod gui_wizard;
pub mod installation;
pub mod iso_installer;
pub mod lightning_installer;

pub use crate::installer::gui_wizard::{
    DetectedOperatingSystem, GuiInstallerWizard, InstallerStep, PartitionStrategy, PrivacySettings,
    UserAccountConfig,
};

pub mod production_installer_engine;
pub use production_installer_engine::*;
pub mod recovery;
pub mod safe_installer;

pub use installation::{
    Architecture, InstallationConfig, InstallationManager, InstallationMethod, SystemRequirements,
    VmType,
};
