#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Driver Module
pub mod device;
pub mod distro_drivers;
pub mod dkms_autoloader;
pub mod driver_test_framework;
pub mod framework;
pub mod gpu_framework;
pub mod gpu_intel_i915;
pub mod gpu_amd_rdna;
pub mod grid;
pub mod irp_system;
pub mod mapper;
pub mod network_framework;
pub mod nic_intel_e1000;
pub mod nvme_storage;
pub mod pci_bus;
pub mod pci_enumeration;
pub mod pods;
pub mod rootkit;
pub mod shims;
pub mod ubuntu_common_drivers;
pub mod universal_support;
pub mod vault;
pub mod wifi_broadcom_bcm4318;
pub mod windows_compat;
pub mod usb_xhci_host;
pub mod ahci_sata_controller;
pub mod hid_input_device;
pub mod audio_codec_hda;

pub use grid::{GridSlotType, PeripheralArchiveGrid};
pub use driver_test_framework::{
    DriverTestRunner, TestResult, TestStatus, TestSummary, GpuTestSuite, NicTestSuite,
    StorageTestSuite, WifiTestSuite, MockPciDevice, MockMmioSpace, QemuSimulator, GuestOs,
};
pub use gpu_intel_i915::{
    IntelGpuDriver, IntelGpuPciDriver, GpuMemoryManager, DisplayMode, GpuCommandBuilder,
};
pub use gpu_amd_rdna::{
    AmdGpuDriver, AmdGpuPciDriver, AmdGpuMemoryManager, DisplayConfiguration, GpxCommandQueue,
};
pub use mapper::{DriverMapper, MapperCategory};
pub use nic_intel_e1000::{
    IntelNicDriver, IntelNicPciDriver, DmaRing, RxDescriptor, TxDescriptor,
};
pub use nvme_storage::{
    NvmeController, NvmePciDriver, NvmeNamespace, QueuePair, NvmeCompletionEntry,
};
pub use wifi_broadcom_bcm4318::{
    BroadcomWifiDriver, BroadcomWifiPciDriver, WifiStandard, Band, AssociationState,
};
pub use pci_bus::{
    PciAddress, PciBarInfo, PciBarType, PciBusManager, PciDeviceNode, PciDriverMatchRule,
    PciHardwareAccess, PciHeaderType, PciInterruptMode, PcieAerLog, PcieAerSeverity, PcieAspmState,
    SimulatedPciHardwareAccess,
};
pub use pci_enumeration::{
    PciEnumerator, PciDeviceInfo, PciBar, PciDriver, PciDriverManager, PciBarType as EnumPciBarType,
    pci_read_u8, pci_read_u16, pci_read_u32, pci_write_u8, pci_write_u16, pci_write_u32,
};
pub use pods::{PeripheralPod, PodType};
pub use rootkit::{
    FileDirectoryEntry, MappedView, SectionBackingType, SectionObject, StealthFilterDriver,
    SyscallStubDisassembler,
};
pub use ubuntu_common_drivers::{
    DkmsAbiRebuildEngine, DkmsModuleSpec, DriverHardwareCategory, DriverLicense,
    UbuntuAdditionalDriversRegistry, UbuntuCommonDriverEngine, UbuntuDriverPackage,
    UbuntuLivepatchDriverHook,
};
pub use vault::{DriverArchiveVault, VaultEntry};
pub use usb_xhci_host::{
    UsbXhciHostDriver, UsbXhciPciDriver, UsbDevice, UsbEndpoint, UsbSpeed, UsbDeviceClass,
    TransferRing,
};
pub use ahci_sata_controller::{
    AhciSataController, AhciPciDriver, SataDevice, SataDeviceType, CommandListEntry,
};
pub use hid_input_device::{
    HidInputDeviceDriver, HidPciDriver, HidDevice, HidDeviceType, HidKeyboardReport,
    HidMouseReport, HidReportBuffer,
};
pub use audio_codec_hda::{
    HdaController, HdaPciDriver, HdaCodec, AudioStream, AudioFormat, SampleRate, BitDepth,
    Channels,
};
