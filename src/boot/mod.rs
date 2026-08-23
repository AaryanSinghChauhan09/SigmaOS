// SigmaOS Boot Module
// Firmware, PCI scanning, and early system initialization

pub mod pci;
pub mod post;
pub mod uefi;
pub mod firmware;
pub mod optimization;
pub mod secure;
pub mod verified;
pub mod bridge_grid;
pub mod firmware_bridge;
pub mod sigma_boot;

pub use pci::{PciBusScanner, PciClass, PciDevice, PCI_MAX_BUS, PCI_MAX_DEVICE};
pub use post::{PostDiagnostics, PostStatus, PostTest, TestType};
pub use uefi::{
    AcpiParser, BootError, GopFramebuffer, GopSplashCanvas, MicrokernelProfile,
    MultiKernelBootSelector, SecureBoot, SimpleSecureBoot, SimpleUEFIBootloader,
    SovereignBootWatchdog, UEFIBootloader, UsbHostController,
};
pub use sigma_boot::{BootEntry, BootManager, BootTheme};
