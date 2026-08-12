// SigmaOS Boot Module
// Firmware, PCI scanning, and early system initialization

pub mod bridge_grid;
pub mod firmware;
pub mod firmware_bridge;
pub mod optimization;
pub mod pci;
pub mod post;
pub mod secure;
pub mod uefi;
pub mod verified;

pub use pci::{PciBusScanner, PciClass, PciDevice, PCI_MAX_BUS, PCI_MAX_DEVICE};
pub use post::{PostDiagnostics, PostStatus, PostTest, TestType};
pub use uefi::{
    AcpiParser, BootError, GopFramebuffer, GopSplashCanvas, MicrokernelProfile,
    MultiKernelBootSelector, SecureBoot, SimpleSecureBoot, SimpleUEFIBootloader,
    SovereignBootWatchdog, UEFIBootloader, UsbHostController,
};
