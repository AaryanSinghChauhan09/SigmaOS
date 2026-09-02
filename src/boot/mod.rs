// SigmaOS Boot Module
// Firmware, PCI scanning, and early system initialization

pub mod bridge_grid;
pub mod firmware;
pub mod firmware_bridge;
pub mod optimization;
pub mod pci;
pub mod plymouth;
pub mod post;
pub mod secure;
pub mod sigma_boot;
pub mod uefi;
pub mod verified;

pub use firmware::{
    efi_attr, CpuMicrocodePatchEngine, EfiVariable, EfiVariableStore, EsrtEntry, EsrtFirmwareType,
    FirmwareCapsuleUpdateManager, IommuArchitecture, IommuFirmwareEngine, MicrocodeHeader,
    MicrocodeVendor, SmbiosFirmwareParser, SmbiosType0BiosInfo, SmbiosType1SystemInfo,
    SmbiosType2BaseboardInfo, SmbiosType3ChassisInfo, EFI_GLOBAL_VARIABLE_GUID,
    SECURITY_DATABASE_GUID,
};
pub use pci::{PciBusScanner, PciClass, PciDevice, PCI_MAX_BUS, PCI_MAX_DEVICE};
pub use plymouth::{GtkPlymouthBootsplashEngine, PlymouthMode, PlymouthTheme};
pub use post::{PostDiagnostics, PostStatus, PostTest, TestType};
pub use sigma_boot::{
    BootEntry, BootManager, BootStageDescriptor, BootTheme, HandoffProtocol, OpenBsdBootDirective,
    SovereignDistroBootStageHandoff, SovereignFastBootServicePipeline,
};
pub use uefi::{
    AcpiParser, BootError, GopFramebuffer, GopSplashCanvas, MicrokernelProfile,
    MultiKernelBootSelector, SecureBoot, SimpleSecureBoot, SimpleUEFIBootloader,
    SovereignBootWatchdog, UEFIBootloader, UsbHostController,
};
