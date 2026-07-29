// SigmaOS Boot Module
// Firmware, PCI scanning, and early system initialization

pub mod pci;
pub mod post;
pub mod uefi;
pub mod firmware;

pub use pci::{PciBusScanner, PciClass, PciDevice, PCI_MAX_BUS, PCI_MAX_DEVICE};
pub use post::{PostDiagnostics, PostStatus, PostTest, TestType};
pub use uefi::{
    SimpleUEFIBootloader, UEFIBootloader, SecureBoot, SimpleSecureBoot, GopFramebuffer,
    AcpiParser, UsbHostController, MultiKernelBootSelector, SovereignBootWatchdog,
    GopSplashCanvas, MicrokernelProfile, BootError as UefiBootError,
};
pub use firmware::{
    BootLoader, BootParams, FirmwareInterface, Initramfs, KernelCommandLine, SetupHeader,
    BootError as FirmwareBootError,
};
