// SigmaOS Installer Module
// Bootable ISO builder and GUI installer wizard

pub mod gui_wizard;
pub mod iso_builder;
pub mod system_installer;

pub use gui_wizard::{
    AccessibilitySettings, DiskInfo, FilesystemType, GuiInstallerWizard, InstallerError,
    InstallerScreen, InstallerSummary, InstallerTheme, PartitionEntry, PartitioningCalculator,
    PartitioningOperation, SystemConfiguration, UserAccount,
};
pub use iso_builder::{
    HybridIsoBuilder, IsoBootConfig, IsoBuildSystem, IsoBuilder, IsoFileEntry,
    IsoFilesystem, IsoMetadata, IsoValidationError, LiveSessionConfig,
};
pub use system_installer::{
    InstallConfig, DiskLayout, BootloaderType, InstallProgress, InstallStage,
    SystemInstaller, InstallError,
};
