//! Boot System (GRUB2/systemd-boot/refind Inspiration)
//! Advanced boot manager with themes, secure boot, and boot environments

#![no_std]

pub use pci::{PciBusScanner, PciClass, PciDevice, PCI_MAX_BUS, PCI_MAX_DEVICE};
pub use post::{PostDiagnostics, PostStatus, PostTest, TestType};
pub use uefi::{
    AcpiParser, BootError, GopFramebuffer, GopSplashCanvas, MicrokernelProfile,
    MultiKernelBootSelector, SecureBoot, SimpleSecureBoot, SimpleUEFIBootloader,
    SovereignBootWatchdog, UEFIBootloader, UsbHostController,
};
