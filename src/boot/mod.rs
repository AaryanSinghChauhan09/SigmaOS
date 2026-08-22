//! Boot System (GRUB2/systemd-boot/refind Inspiration)
//! Advanced boot manager with themes, secure boot, and boot environments

#![no_std]

pub mod pci;
pub mod post;
pub mod uefi;

pub use pci::{PciBusScanner, PciClass, PciDevice, PCI_MAX_BUS, PCI_MAX_DEVICE};
pub use post::{PostDiagnostics, PostStatus, PostTest, TestType};
pub use uefi::{
    SimpleSecureBoot, SimpleUEFIBootloader,
};
